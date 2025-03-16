use commands::{backup, list, restore, verify_connection};
use log::LevelFilter;
mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list,
            backup,
            restore,
            verify_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
