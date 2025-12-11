use crate::utils::state::AppState;
use rusqlite::Connection;
use std::path::PathBuf;
use tauri::{App, Manager};

fn get_db_path(app: &App) -> PathBuf {
    let mut path = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&path).expect("Failed to create app data directory");
    path.push("yaykeys.db");

    path
}

pub fn init_db(app: &App) -> Connection {
    let db_path = get_db_path(app);
    let conn = Connection::open(db_path).expect("Failed to open database");

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

pub fn set() {}

pub fn update() {}

pub fn delete() {}

pub fn sync_all() {}

pub fn sync() {}
