mod bridge;
mod commands;
mod save;
pub mod dto;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::open_save::open_save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
