use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};
use serde::Serialize;
use serde::Deserialize;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
// 锁屏检测改用 winapi 0.3：其 OpenInputDesktop/CloseDesktop/GENERIC_READ API 在
// Win32_UI_WindowsAndMessaging / Win32_Foundation 下稳定且无需特殊处理（windows 0.58 的
// 桌面函数在该特性组合下未导出，会导致编译失败）。
use winapi::um::winuser::{CloseDesktop, OpenInputDesktop};
use winapi::um::winnt::GENERIC_READ;

// 结构化系统事件载荷（取代原纯文本 system-event / 结构化 battery-event）
// kind ∈ {volume, power, battery, network, lock, unlock}
// level ∈ {info, success, warn}
#[derive(Clone, Serialize)]
struct SysEventPayload {
    kind: String,
    level: String,
    text: String,
    ts: u64,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct SystemEventFilter {
    pub enabled: bool,
    pub volume: bool,
    pub power: bool,
    pub battery: bool,
    pub unlock: bool,
}

// 分类门控原子：总闸 + 四个子类别（音量 / 电源 / 电量 / 解锁提示）
static SYS_EVT_ENABLED: AtomicBool = AtomicBool::new(true);
static SYS_EVT_VOLUME: AtomicBool = AtomicBool::new(true);
static SYS_EVT_POWER: AtomicBool = AtomicBool::new(true);
static SYS_EVT_BATTERY: AtomicBool = AtomicBool::new(true);
static SYS_EVT_UNLOCK: AtomicBool = AtomicBool::new(true);

#[tauri::command]
pub fn set_system_event_filter(filter: SystemEventFilter) {
    SYS_EVT_ENABLED.store(filter.enabled, Ordering::Relaxed);
    SYS_EVT_VOLUME.store(filter.volume, Ordering::Relaxed);
    SYS_EVT_POWER.store(filter.power, Ordering::Relaxed);
    SYS_EVT_BATTERY.store(filter.battery, Ordering::Relaxed);
    SYS_EVT_UNLOCK.store(filter.unlock, Ordering::Relaxed);
}

// 统一事件发射：仅在总闸开启时发送结构化 sysmsg-event
fn emit_sys_event(app: &AppHandle, kind: &str, level: &str, text: &str) {
    if !SYS_EVT_ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let payload = SysEventPayload {
        kind: kind.to_string(),
        level: level.to_string(),
        text: text.to_string(),
        ts: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0),
    };
    let _ = app.emit("sysmsg-event", payload);
}

pub fn start_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        // 初始化 COM 接口以获取音频
        unsafe { let _ = CoInitializeEx(None, COINIT_MULTITHREADED); }

        let mut last_volume = get_system_volume().unwrap_or(-1.0);
        let mut last_power_state = 255;    // 255 = 未知状态
        let mut last_battery_percent = 255; // 255 = 未知电量
        // 锁屏检测：true = 已解锁（可操作桌面），false = 已锁定
        let mut last_unlocked = is_desktop_unlocked().unwrap_or(true);
        // 空闲优化：连续多次状态无变化时延长休眠，减少 COM API 调用频率
        let mut stability_counter: u32 = 0;

        // 启动时先拉取一次真实状态打底
        if let Some((ac_status, battery_percent)) = get_power_status() {
            last_power_state = ac_status;
            last_battery_percent = battery_percent;
        }

        loop {
            // 空闲优化：状态越稳定，检查间隔越长（800ms ~ 2000ms）
            let sleep_ms = if stability_counter >= 3 { 2000 } else { 800 };
            std::thread::sleep(Duration::from_millis(sleep_ms));

            let mut changed = false;

            // 1. 检查音量变化
            if let Some(current_volume) = get_system_volume() {
                if (current_volume - last_volume).abs() > 0.01 && last_volume != -1.0 {
                    let vol_percent = (current_volume * 100.0).round() as i32;
                    if SYS_EVT_VOLUME.load(Ordering::Relaxed) {
                        emit_sys_event(&app, "volume", "info", &format!("当前系统音量 {}%", vol_percent));
                    }
                    changed = true;
                }
                last_volume = current_volume;
            }

            // 2. 检查电源状态与电量变化
            if let Some((current_power, current_percent)) = get_power_status() {

                // 【情况 A】电源插入/拔出状态发生了改变
                if current_power != last_power_state && last_power_state != 255 {
                    if current_power == 1 {
                        // 插入电源
                        if SYS_EVT_POWER.load(Ordering::Relaxed) {
                            emit_sys_event(&app, "power", "success", &format!("已接入电源，当前电量 {}%", current_percent));
                        }
                    } else if current_power == 0 {
                        // 拔出电源
                        if SYS_EVT_POWER.load(Ordering::Relaxed) {
                            emit_sys_event(&app, "power", "info", "正在使用电池供电");
                        }
                    }
                    changed = true;
                }

                // 【情况 B】正在使用电池（未插电），且电量正在下降
                if current_power == 0 && current_percent < last_battery_percent {
                    // 低电量防抖机制：仅在跌破这几个关键节点时，触发红色警告
                    if current_percent <= 20 && [20, 15, 10, 5].contains(&current_percent) {
                        if SYS_EVT_BATTERY.load(Ordering::Relaxed) {
                            emit_sys_event(&app, "battery", "warn", &format!("电池电量低，剩余 {}%", current_percent));
                        }
                        changed = true;
                    }
                }

                if current_power != last_power_state || current_percent != last_battery_percent {
                    changed = true;
                }
                last_power_state = current_power;
                last_battery_percent = current_percent;
            }

            // 3. 锁屏/解锁检测（解锁提示）：桌面不可用时视为已锁定
            if let Some(unlocked) = is_desktop_unlocked() {
                if unlocked != last_unlocked {
                    if SYS_EVT_UNLOCK.load(Ordering::Relaxed) {
                        if unlocked {
                            emit_sys_event(&app, "unlock", "info", "已解锁，欢迎回来");
                        } else {
                            emit_sys_event(&app, "lock", "info", "系统已锁定");
                        }
                    }
                    changed = true;
                }
                last_unlocked = unlocked;
            }

            // 空闲优化：状态变化时重置计数器，无变化时递增
            if changed {
                stability_counter = 0;
            } else {
                stability_counter = stability_counter.saturating_add(1);
            }
        }
    });
}

// 辅助函数：获取 Windows 系统音量 (0.0 到 1.0)
fn get_system_volume() -> Option<f32> {
    unsafe {
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).ok()?;
        let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None).ok()?;

        let volume = endpoint_volume.GetMasterVolumeLevelScalar().ok()?;
        Some(volume)
    }
}

// 辅助函数：同时获取电源插入状态和剩余电量
fn get_power_status() -> Option<(u8, u8)> {
    unsafe {
        let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut status).is_ok() {
            // 返回元组: (ACLineStatus, BatteryLifePercent)
            // ACLineStatus: 0 = 使用电池, 1 = 插入电源
            // BatteryLifePercent: 0 ~ 100
            Some((status.ACLineStatus, status.BatteryLifePercent))
        } else {
            None
        }
    }
}

// 辅助函数：通过 OpenInputDesktop 判断当前会话是否处于锁定状态。
// 锁屏时输入桌面不可打开，返回 false；可操作时返回 true。非 Windows 平台直接返回 None（跳过检测）。
fn is_desktop_unlocked() -> Option<bool> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            // OpenInputDesktop：锁屏时输入桌面不可打开，返回 NULL；可操作时返回有效句柄。
            // dwFlags=0，fInherit=FALSE(0)，dwDesiredAccess=GENERIC_READ。
            let h = OpenInputDesktop(0, 0, GENERIC_READ);
            if h.is_null() {
                // 打不开输入桌面（返回 NULL）→ 已锁定
                return Some(false);
            }
            let _ = CloseDesktop(h);
            Some(true)
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        // 非 Windows 平台无锁屏概念，跳过检测
        None
    }
}
