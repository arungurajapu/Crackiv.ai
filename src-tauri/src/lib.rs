pub mod app_state;
pub mod capture;

use std::sync::Arc;
use tauri::Manager;
use app_state::AppState;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, SetWindowDisplayAffinity,
    GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT,
    WDA_EXCLUDEFROMCAPTURE,
};

#[tauri::command]
async fn trigger_ai_call(prompt_type: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let buf_lock = state.screenshots.lock().await;
    
    // Simulate getting latest screenshot
    let latest_screenshot = buf_lock.back().cloned().unwrap_or_default();
    
    // Simulate API logic
    Ok(format!("AI Call with prompt type '{}' executed processing latest screenshot: bytes length {}", prompt_type, latest_screenshot.len()))
}

#[tauri::command]
async fn get_latest_screenshots(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let buf_lock = state.screenshots.lock().await;
    Ok(buf_lock.iter().cloned().collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = AppState::new();
            
            // Start capture thread with reference to screenshots buffer
            let shots_buffer = Arc::clone(&state.screenshots);
            tauri::async_runtime::spawn(async move {
                capture::start_capture_thread(shots_buffer).await;
            });
            
            // Manage application state
            app.manage(state);

            // Apply Win32 overrides to the main window
            let window = app.get_webview_window("main").unwrap();
            let hwnd_addr = window.hwnd().unwrap().0 as *mut std::ffi::c_void;
            let hwnd = HWND(hwnd_addr);

            unsafe {
                // Prevent screen capture
                let _ = SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE);

                // Add WS_EX_TOOLWINDOW (hide from alt-tab) and WS_EX_TRANSPARENT (click passthrough)
                let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                SetWindowLongW(
                    hwnd,
                    GWL_EXSTYLE,
                    ex_style | WS_EX_TOOLWINDOW.0 as i32 | WS_EX_TRANSPARENT.0 as i32,
                );
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![trigger_ai_call, get_latest_screenshots])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
