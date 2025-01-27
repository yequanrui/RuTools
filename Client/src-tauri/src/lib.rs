use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .setup(|app| {
            // Get the autostart manager
            let autostart_manager = app.autolaunch();
            // 启用 autostart
            let _ = autostart_manager.enable();
            // 检查 enable 状态
            println!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );
            // 禁用 autostart
            let _ = autostart_manager.disable();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
