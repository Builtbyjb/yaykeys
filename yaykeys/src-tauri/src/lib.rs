// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod macos;
mod types;
mod utils;

use std::sync::Mutex;
use tauri::Manager;
use types::setting::Setting;
use utils::state::AppState;
use utils::storage;

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> Vec<Setting> {
    #[cfg(target_os = "macos")]
    let apps = macos::fetch::fetch(state);
    return apps;
}

#[tauri::command]
fn search(name: &str) -> Vec<Setting> {
    let apps: Vec<Setting> = vec![];
    return apps;
}

#[tauri::command]
fn update_enabled(id: u16, value: bool) -> bool {
    println!("id: {}; value: {}", id, value);
    true
}

#[tauri::command]
fn update_mode(id: u16, value: &str) -> bool {
    println!("id: {}; value: {}", id, value);
    true
}

#[tauri::command]
fn update_hotkey(id: u16, value: &str) -> bool {
    println!("id: {}; value: {}", id, value);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = storage::init_db(app);
            app.manage(AppState {
                conn: Mutex::new(conn),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            search,
            update_enabled,
            update_mode,
            update_hotkey
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
