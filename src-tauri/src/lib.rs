// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod macos;
mod utils;

use utils::types::App;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get() -> Vec<String> {
    #[cfg(target_os = "macos")]
    let apps: Vec<String> = macos::fetch::fetch();
    //
    return apps;
}

#[tauri::command]
fn find(name: &str) -> Vec<App> {
    let apps: Vec<App> = vec![];
    return apps;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
