//! 打印队列实时监测（事件驱动，无作业时阻塞挂起，零 CPU 轮询）
//!
//! 使用 winapi::um::winspool（windows 0.58 的 Devices_Printing 未稳定导出，与锁屏检测同坑）。
//! 流程：OpenPrinterW(NULL) → FindFirstPrinterChangeNotification → WaitForMultipleObjects
//! 唤醒后 EnumJobsW 拉全量快照，经节流后 emit `print-queue-tick`。

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

// ──────────────────────────────────────────────
// 契约结构（camelCase，与前端一致）
// ──────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrintJob {
    pub job_id: u32,
    pub document: String,
    pub printer: String,
    pub pages_printed: u32,
    pub total_pages: u32,
    pub position: u32,
    pub status: String,
    pub submitted: u64,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrintQueueState {
    pub has_jobs: bool,
    pub default_printer: String,
    pub jobs: Vec<PrintJob>,
}

impl Default for PrintQueueState {
    fn default() -> Self {
        Self {
            has_jobs: false,
            default_printer: String::new(),
            jobs: Vec::new(),
        }
    }
}

// ──────────────────────────────────────────────
// 全局状态
// ──────────────────────────────────────────────

static PRINTER_MONITOR_ENABLED: AtomicBool = AtomicBool::new(true);
static MONITOR_THREAD_STARTED: AtomicBool = AtomicBool::new(false);
static STOP_EVENT: OnceLock<isize> = OnceLock::new(); // HANDLE as isize
static LAST_STATE: Mutex<PrintQueueState> = Mutex::new(PrintQueueState {
    has_jobs: false,
    default_printer: String::new(),
    jobs: Vec::new(),
});

const THROTTLE_MS: u128 = 1500;
const WAIT_TIMEOUT_MS: u32 = 5000;
const RECONNECT_WAIT_MS: u32 = 5000;

// PRINTER_CHANGE_* (winspool.h)
const PRINTER_CHANGE_ADD_JOB: u32 = 0x0000_0100;
const PRINTER_CHANGE_SET_JOB: u32 = 0x0000_0200;
const PRINTER_CHANGE_DELETE_JOB: u32 = 0x0000_0400;
const PRINTER_CHANGE_WRITE_JOB: u32 = 0x0000_0800;
const PRINTER_CHANGE_JOB: u32 =
    PRINTER_CHANGE_ADD_JOB | PRINTER_CHANGE_SET_JOB | PRINTER_CHANGE_DELETE_JOB | PRINTER_CHANGE_WRITE_JOB;

// JOB_STATUS_*
const JOB_STATUS_PAUSED: u32 = 0x0000_0001;
const JOB_STATUS_ERROR: u32 = 0x0000_0002;
const JOB_STATUS_DELETING: u32 = 0x0000_0004;
const JOB_STATUS_SPOOLING: u32 = 0x0000_0008;
const JOB_STATUS_PRINTING: u32 = 0x0000_0010;
const JOB_STATUS_OFFLINE: u32 = 0x0000_0020;
const JOB_STATUS_PAPEROUT: u32 = 0x0000_0040;
const JOB_STATUS_PRINTED: u32 = 0x0000_0080;
const JOB_STATUS_DELETED: u32 = 0x0000_0100;
const JOB_STATUS_BLOCKED_DEVQ: u32 = 0x0000_0200;
const JOB_STATUS_USER_INTERVENTION: u32 = 0x0000_0400;
const JOB_STATUS_RESTART: u32 = 0x0000_0800;

// WaitForMultipleObjects 返回值
const WAIT_OBJECT_0: u32 = 0x0000_0000;
const WAIT_TIMEOUT: u32 = 0x0000_0102;
const WAIT_FAILED: u32 = 0xFFFF_FFFF;

// ──────────────────────────────────────────────
// 公共 API
// ──────────────────────────────────────────────

/// 启动打印队列监控线程（幂等：重复调用不会生成多个线程）
pub fn start_print_queue_monitor(app: AppHandle) {
    if MONITOR_THREAD_STARTED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    // 预创建 stop 事件，供 set_printer_monitor_enabled(false) 立即唤醒
    ensure_stop_event();

    thread::spawn(move || {
        #[cfg(target_os = "windows")]
        {
            monitor_loop(app);
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = app;
            // 非 Windows：线程立即结束（保持编译通过）
        }
    });
}

#[tauri::command]
pub fn set_printer_monitor_enabled(enabled: bool) {
    PRINTER_MONITOR_ENABLED.store(enabled, Ordering::SeqCst);
    // 无论启停都脉冲唤醒监控线程（手动复位事件：Set 后由循环侧 Reset）
    signal_stop_event();
    if !enabled {
        // 清空快照；空队列 emit 由监控线程在退出等待后负责
        let empty = PrintQueueState::default();
        if let Ok(mut guard) = LAST_STATE.lock() {
            *guard = empty;
        }
    }
}

#[tauri::command]
pub fn get_printer_state() -> PrintQueueState {
    LAST_STATE
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}

// ──────────────────────────────────────────────
// stop 事件工具
// ──────────────────────────────────────────────

fn ensure_stop_event() -> isize {
    *STOP_EVENT.get_or_init(|| {
        #[cfg(target_os = "windows")]
        unsafe {
            // 手动复位事件，初始未信号
            let h = winapi::um::synchapi::CreateEventW(
                std::ptr::null_mut(),
                1, // bManualReset = TRUE
                0, // bInitialState = FALSE
                std::ptr::null(),
            );
            h as isize
        }
        #[cfg(not(target_os = "windows"))]
        {
            0
        }
    })
}

fn signal_stop_event() {
    #[cfg(target_os = "windows")]
    unsafe {
        let h = ensure_stop_event() as winapi::shared::ntdef::HANDLE;
        if !h.is_null() {
            winapi::um::synchapi::SetEvent(h);
        }
    }
}

fn reset_stop_event() {
    #[cfg(target_os = "windows")]
    unsafe {
        let h = ensure_stop_event() as winapi::shared::ntdef::HANDLE;
        if !h.is_null() {
            winapi::um::synchapi::ResetEvent(h);
        }
    }
}

// ──────────────────────────────────────────────
// Windows 监控主循环
// ──────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn monitor_loop(app: AppHandle) {
    use winapi::shared::minwindef::FALSE;
    use winapi::shared::ntdef::HANDLE;
    use winapi::um::synchapi::WaitForMultipleObjects;
    use winapi::um::winspool::{
        ClosePrinter, FindClosePrinterChangeNotification, FindFirstPrinterChangeNotification,
        FindNextPrinterChangeNotification, OpenPrinterW,
    };

    let stop_handle = ensure_stop_event() as HANDLE;
    let mut last_emit = Instant::now() - Duration::from_secs(10);
    let mut last_jobs_sig: Vec<(u32, u32, u32, String)> = Vec::new(); // (id, pages, total, status)
    let mut last_set_sig: Vec<u32> = Vec::new(); // job ids 集合

    // 启动后先拉一次快照，保证前端晚订阅也能 get_printer_state 恢复
    if PRINTER_MONITOR_ENABLED.load(Ordering::SeqCst) {
        let state = snapshot_queue();
        store_and_emit(&app, &state, true);
        last_jobs_sig = jobs_progress_sig(&state.jobs);
        last_set_sig = jobs_set_sig(&state.jobs);
        last_emit = Instant::now();
    }

    loop {
        // 禁用时：清空状态，阻塞等待 re-enable（可被 stop 事件立即唤醒，无忙循环）
        if !PRINTER_MONITOR_ENABLED.load(Ordering::SeqCst) {
            let empty = PrintQueueState::default();
            store_and_emit(&app, &empty, true);
            last_jobs_sig.clear();
            last_set_sig.clear();
            // 先 Reset 掉本次唤醒信号，再阻塞等待下次 SetEvent（启用）或超时复查
            reset_stop_event();
            unsafe {
                let _ = WaitForMultipleObjects(1, &stop_handle, FALSE, RECONNECT_WAIT_MS);
            }
            // 醒来后清信号，避免下一轮瞬时返回
            reset_stop_event();
            continue;
        }

        // OpenPrinterW(NULL) —— 打开本地打印服务器
        let mut h_printer: HANDLE = std::ptr::null_mut();
        let open_ok = unsafe { OpenPrinterW(std::ptr::null_mut(), &mut h_printer, std::ptr::null_mut()) };
        if open_ok == 0 || h_printer.is_null() {
            // spooler 不可用：可被 stop 打断的阻塞等待后重试，不忙循环
            unsafe {
                let _ = WaitForMultipleObjects(1, &stop_handle, FALSE, RECONNECT_WAIT_MS);
            }
            if !PRINTER_MONITOR_ENABLED.load(Ordering::SeqCst) {
                // 禁用路径走上方分支
                continue;
            }
            continue;
        }

        // FindFirstPrinterChangeNotification
        let h_change = unsafe {
            FindFirstPrinterChangeNotification(
                h_printer,
                PRINTER_CHANGE_JOB,
                0,
                std::ptr::null_mut(),
            )
        };
        if h_change.is_null() || h_change == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            unsafe {
                ClosePrinter(h_printer);
            }
            unsafe {
                let _ = WaitForMultipleObjects(1, &stop_handle, FALSE, RECONNECT_WAIT_MS);
            }
            continue;
        }

        // 进入事件等待循环
        loop {
            if !PRINTER_MONITOR_ENABLED.load(Ordering::SeqCst) {
                break;
            }

            let handles: [HANDLE; 2] = [h_change, stop_handle];
            let wait_rc = unsafe {
                WaitForMultipleObjects(2, handles.as_ptr(), FALSE, WAIT_TIMEOUT_MS)
            };

            // stop 事件（index 1）或已禁用
            if wait_rc == WAIT_OBJECT_0 + 1 || !PRINTER_MONITOR_ENABLED.load(Ordering::SeqCst) {
                reset_stop_event();
                break;
            }

            // WAIT_FAILED：句柄失效，重建
            if wait_rc == WAIT_FAILED {
                break;
            }

            let from_notification = wait_rc == WAIT_OBJECT_0;
            let from_timeout = wait_rc == WAIT_TIMEOUT;

            if !from_notification && !from_timeout {
                // 其他返回值也重建
                break;
            }

            if from_notification {
                // 必须确认通知，否则不会再触发
                unsafe {
                    let mut change: u32 = 0;
                    let _ = FindNextPrinterChangeNotification(
                        h_change,
                        &mut change,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );
                }
            }

            // 拉全量快照
            let state = snapshot_with_printer(h_printer);
            let set_sig = jobs_set_sig(&state.jobs);
            let prog_sig = jobs_progress_sig(&state.jobs);

            // 作业集合（id）或状态变化 → 立即；仅页数进度变化 → 1.5s 节流；
            // 5s 超时兜底：有变化则发，无变化只更新共享快照。
            let ids_changed = set_sig != last_set_sig;
            let status_changed = {
                let old_map: std::collections::BTreeMap<u32, &str> =
                    last_jobs_sig.iter().map(|s| (s.0, s.3.as_str())).collect();
                state.jobs.iter().any(|j| {
                    old_map.get(&j.job_id).copied().unwrap_or("") != j.status.as_str()
                })
            };
            let progress_changed = prog_sig != last_jobs_sig;
            let structural = ids_changed || status_changed;
            let only_progress = !structural && progress_changed;

            let should_emit = if structural {
                true
            } else if only_progress {
                last_emit.elapsed().as_millis() >= THROTTLE_MS
            } else if from_timeout && progress_changed {
                // 兜底超时且有差异（含上次节流跳过的进度）
                true
            } else {
                false
            };

            if let Ok(mut guard) = LAST_STATE.lock() {
                *guard = state.clone();
            }

            if should_emit {
                let _ = app.emit("print-queue-tick", &state);
                last_emit = Instant::now();
                last_jobs_sig = prog_sig;
                last_set_sig = set_sig;
            }
        }

        // 清理句柄
        unsafe {
            FindClosePrinterChangeNotification(h_change);
            ClosePrinter(h_printer);
        }

        // 禁用时 emit 空队列
        if !PRINTER_MONITOR_ENABLED.load(Ordering::SeqCst) {
            let empty = PrintQueueState::default();
            store_and_emit(&app, &empty, true);
            last_jobs_sig.clear();
            last_set_sig.clear();
        }
        // 否则是 spooler 重启/句柄失效，短暂等待后重建（可被 stop 打断）
        else {
            unsafe {
                let _ = WaitForMultipleObjects(1, &stop_handle, FALSE, 500);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn store_and_emit(app: &AppHandle, state: &PrintQueueState, emit: bool) {
    if let Ok(mut guard) = LAST_STATE.lock() {
        *guard = state.clone();
    }
    if emit {
        let _ = app.emit("print-queue-tick", state);
    }
}

#[cfg(target_os = "windows")]
fn jobs_set_sig(jobs: &[PrintJob]) -> Vec<u32> {
    let mut ids: Vec<u32> = jobs.iter().map(|j| j.job_id).collect();
    ids.sort_unstable();
    ids
}

#[cfg(target_os = "windows")]
fn jobs_progress_sig(jobs: &[PrintJob]) -> Vec<(u32, u32, u32, String)> {
    jobs.iter()
        .map(|j| (j.job_id, j.pages_printed, j.total_pages, j.status.clone()))
        .collect()
}

#[cfg(target_os = "windows")]
fn snapshot_queue() -> PrintQueueState {
    use winapi::shared::ntdef::HANDLE;
    use winapi::um::winspool::{ClosePrinter, OpenPrinterW};

    let mut h_printer: HANDLE = std::ptr::null_mut();
    let open_ok = unsafe { OpenPrinterW(std::ptr::null_mut(), &mut h_printer, std::ptr::null_mut()) };
    if open_ok == 0 || h_printer.is_null() {
        return PrintQueueState {
            has_jobs: false,
            default_printer: get_default_printer_name(),
            jobs: Vec::new(),
        };
    }
    let state = snapshot_with_printer(h_printer);
    unsafe {
        ClosePrinter(h_printer);
    }
    state
}

#[cfg(target_os = "windows")]
fn snapshot_with_printer(h_printer: winapi::shared::ntdef::HANDLE) -> PrintQueueState {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::winspool::{EnumJobsW, JOB_INFO_1W};

    let default_printer = get_default_printer_name();
    let mut jobs: Vec<PrintJob> = Vec::new();

    // 两阶段 EnumJobsW：先取所需缓冲区大小
    let mut needed: DWORD = 0;
    let mut returned: DWORD = 0;
    unsafe {
        // First call: 期望返回 ERROR_INSUFFICIENT_BUFFER
        let _ = EnumJobsW(
            h_printer,
            0,
            0xFFFF_FFFF, // 全部作业
            1,           // JOB_INFO_1
            std::ptr::null_mut(),
            0,
            &mut needed,
            &mut returned,
        );
    }

    if needed > 0 {
        let mut buf = vec![0u8; needed as usize];
        let ok = unsafe {
            EnumJobsW(
                h_printer,
                0,
                0xFFFF_FFFF,
                1,
                buf.as_mut_ptr(),
                needed,
                &mut needed,
                &mut returned,
            )
        };
        if ok != 0 && returned > 0 {
            let base = buf.as_ptr() as *const JOB_INFO_1W;
            for i in 0..returned as isize {
                let info = unsafe { &*base.offset(i) };
                // 过滤已删除/已完成作业（部分 spooler 仍短暂保留）
                if info.Status & (JOB_STATUS_DELETED | JOB_STATUS_PRINTED) != 0
                    && info.Status & JOB_STATUS_PRINTING == 0
                    && info.Status & JOB_STATUS_SPOOLING == 0
                {
                    // 已打印且不再活动 → 跳过
                    if info.Status & JOB_STATUS_DELETED != 0 {
                        continue;
                    }
                }
                let document = wide_ptr_to_string(info.pDocument);
                let printer = wide_ptr_to_string(info.pPrinterName);
                let status = map_job_status(info.Status, info.pStatus);
                let submitted = systemtime_to_unix_ms(&info.Submitted);
                jobs.push(PrintJob {
                    job_id: info.JobId,
                    document,
                    printer,
                    pages_printed: info.PagesPrinted,
                    total_pages: info.TotalPages,
                    position: info.Position,
                    status,
                    submitted,
                });
            }
        }
    }

    // 按队列位置排序
    jobs.sort_by_key(|j| j.position);

    PrintQueueState {
        has_jobs: !jobs.is_empty(),
        default_printer,
        jobs,
    }
}

#[cfg(target_os = "windows")]
fn get_default_printer_name() -> String {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::winspool::GetDefaultPrinterW;

    let mut needed: DWORD = 0;
    unsafe {
        // 取长度
        let _ = GetDefaultPrinterW(std::ptr::null_mut(), &mut needed);
    }
    if needed == 0 {
        return String::new();
    }
    let mut buf = vec![0u16; needed as usize];
    let ok = unsafe { GetDefaultPrinterW(buf.as_mut_ptr(), &mut needed) };
    if ok == 0 {
        return String::new();
    }
    // 去掉末尾 NUL
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

#[cfg(target_os = "windows")]
fn wide_ptr_to_string(ptr: *mut u16) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        let mut len = 0usize;
        // 防御性上限，避免异常指针死循环
        while len < 4096 && *ptr.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len))
    }
}

#[cfg(target_os = "windows")]
fn systemtime_to_unix_ms(st: &winapi::um::minwinbase::SYSTEMTIME) -> u64 {
    // SYSTEMTIME 为本地/UTC 混合场景；用 chrono-free 近似：
    // 通过 SystemTime::now 不做复杂转换时，回退 0；优先用 Windows API 转 FILETIME
    use winapi::shared::minwindef::FILETIME;
    use winapi::um::minwinbase::SYSTEMTIME;
    use winapi::um::timezoneapi::SystemTimeToFileTime;

    unsafe {
        let mut ft: FILETIME = std::mem::zeroed();
        if SystemTimeToFileTime(st as *const SYSTEMTIME, &mut ft) == 0 {
            return 0;
        }
        // FILETIME: 100-ns intervals since 1601-01-01 UTC
        let ticks = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
        // 1601 → 1970 偏移：11644473600 秒
        const EPOCH_DIFF_100NS: u64 = 11644473600u64 * 10_000_000;
        if ticks < EPOCH_DIFF_100NS {
            return 0;
        }
        let unix_100ns = ticks - EPOCH_DIFF_100NS;
        unix_100ns / 10_000 // → 毫秒
    }
}

#[cfg(target_os = "windows")]
fn map_job_status(status: u32, p_status: *mut u16) -> String {
    // 优先可读 pStatus 字符串
    let custom = wide_ptr_to_string(p_status);
    if !custom.is_empty() {
        return custom;
    }
    if status & JOB_STATUS_ERROR != 0 {
        return "错误".into();
    }
    if status & JOB_STATUS_PAPEROUT != 0 {
        return "缺纸".into();
    }
    if status & JOB_STATUS_OFFLINE != 0 {
        return "脱机".into();
    }
    if status & JOB_STATUS_USER_INTERVENTION != 0 {
        return "需要干预".into();
    }
    if status & JOB_STATUS_BLOCKED_DEVQ != 0 {
        return "已阻塞".into();
    }
    if status & JOB_STATUS_PAUSED != 0 {
        return "已暂停".into();
    }
    if status & JOB_STATUS_DELETING != 0 {
        return "删除中".into();
    }
    if status & JOB_STATUS_DELETED != 0 {
        return "已删除".into();
    }
    if status & JOB_STATUS_RESTART != 0 {
        return "重启中".into();
    }
    if status & JOB_STATUS_PRINTING != 0 {
        return "打印中".into();
    }
    if status & JOB_STATUS_SPOOLING != 0 {
        return "后台处理".into();
    }
    if status & JOB_STATUS_PRINTED != 0 {
        return "已完成".into();
    }
    if status == 0 {
        return "排队中".into();
    }
    "处理中".into()
}


