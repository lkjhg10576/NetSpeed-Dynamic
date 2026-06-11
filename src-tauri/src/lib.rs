use std::sync::Mutex;
use tauri::State;
use tauri::Manager;
use sysinfo::Networks;

struct AppState {
    networks: Mutex<Networks>,
}

#[tauri::command]
fn get_network_stats(state: State<'_, AppState>) -> (u64, u64) {
    let mut networks = state.networks.lock().unwrap();
    networks.refresh_list();

    let mut total_rx = 0;
    let mut total_tx = 0;

    for (_interface_name, data) in networks.iter() {
        total_rx += data.total_received();
        total_tx += data.total_transmitted();
    }

    (total_rx, total_tx)
}

#[tauri::command]
fn is_widget_visible(app: tauri::AppHandle) -> bool {
    match app.get_webview_window("widget") {
        Some(win) => win.is_visible().unwrap_or(false),
        None => false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let networks = Networks::new_with_refreshed_list();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            networks: Mutex::new(networks),
        })
        .invoke_handler(tauri::generate_handler![get_network_stats, is_widget_visible])
        .setup(|app| {
            if let Some(widget_window) = app.get_webview_window("widget") {
                #[cfg(target_os = "windows")]
                {
                    use windows_sys::Win32::Graphics::Dwm::{
                        DwmSetWindowAttribute,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        DWMWA_BORDER_COLOR,
                        DWMWCP_DONOTROUND,
                    };
                    use windows_sys::Win32::Graphics::Gdi::{
                        CreateRoundRectRgn,
                        SetWindowRgn,
                    };
                    use windows_sys::Win32::UI::WindowsAndMessaging::{
                        GetWindowRect,  // ← 关键：获取完整窗口尺寸
                        SetWindowLongPtrW,
                        GWL_STYLE,
                        WS_CAPTION,     // ← 关键：禁用系统标题栏
                    };
                    use windows_sys::Win32::Foundation::{HWND, RECT};

                    if let Ok(hwnd) = widget_window.hwnd() {
                        let hwnd_raw = hwnd.0 as HWND;
                        unsafe {
                            // ① 【彻底干掉系统标题栏】
                            use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;

                            let style = GetWindowLongPtrW(hwnd_raw, GWL_STYLE);
                            SetWindowLongPtrW(hwnd_raw, GWL_STYLE, style & !(WS_CAPTION as isize));

                            // ② 【干掉边框】
                            let border_color: u32 = 0xFFFFFFFE;
                            let _ = DwmSetWindowAttribute(
                                hwnd_raw,
                                DWMWA_BORDER_COLOR as u32,
                                &border_color as *const _ as *const _,
                                std::mem::size_of::<u32>() as u32,
                            );

                            // ③ 【干掉DWM圆角和阴影】
                            let corner_preference = DWMWCP_DONOTROUND;
                            let _ = DwmSetWindowAttribute(
                                hwnd_raw,
                                DWMWA_WINDOW_CORNER_PREFERENCE as u32,
                                &corner_preference as *const _ as *const _,
                                std::mem::size_of::<i32>() as u32,
                            );

                            // ④ 【正确获取物理尺寸】
                            //    GetWindowRect → 获取整个窗口物理尺寸（包含非客户区）
                            //    GetClientRect → 仅客户区尺寸（会导致偏移）        
                            let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                            GetWindowRect(hwnd_raw, &mut rect);
                            let w = rect.right - rect.left;
                            let h = rect.bottom - rect.top;

                            // ⑤ 【正确裁剪】
                            //    圆角 = 高度（保证胶囊形）
                            let corner = h;
                            let region = CreateRoundRectRgn(0, 0, w, h, corner, corner);
                            if !region.is_null() {
                                SetWindowRgn(hwnd_raw, region, 1);
                            }
                        }
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}