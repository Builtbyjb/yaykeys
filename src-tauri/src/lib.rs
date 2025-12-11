// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod macos;
mod utils;

use std::sync::Mutex;
use tauri::Manager;
use utils::state::AppState;
use utils::storage;
use utils::types::App;

#[tauri::command]
fn get() -> Vec<String> {
    #[cfg(target_os = "macos")]
    let apps: Vec<String> = macos::fetch::fetch();
    //
    return apps;
}

#[tauri::command]
fn search(name: &str) -> Vec<App> {
    let apps: Vec<App> = vec![];
    return apps;
}

#[tauri::command]
fn update() -> bool {
    //
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = storage::init_db(app);
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
