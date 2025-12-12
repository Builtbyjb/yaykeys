use crate::utils;

use rusqlite::Connection;
use std::path::PathBuf;
use tauri::Manager;
use utils::state::AppState;
use utils::types;

fn get_db_path(app: &tauri::App) -> PathBuf {
    let mut path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&path).expect("Failed to create app data directory");
    path.push("yaykeys.db");

    path
}

pub fn init_db(app: &tauri::App) -> Connection {
    let db_path = get_db_path(app);
    let conn = Connection::open(db_path).expect("Failed to open database");

    // Schema:
    // name
    // app_type
    // path
    // hotkey
    // option
    // enabled
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");

    conn
}

pub fn get_all(state: tauri::State<'_, AppState>) {
    let db = state.db.lock().unwrap();
}

pub fn get() {}

pub fn set(setting: types::Setting) {
    // Adds a new app setting
}

pub fn update() {}

pub fn search() {}

pub fn delete() {}

// Cloud sync
pub fn sync_all() {}

pub fn sync() {}
