use crate::types;

use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::Manager;
use types::setting::Setting;

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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            hotkey TEXT DEFAULT NULL,
            exe_path TEXT NOT NULL,
            mode TEXT NOT NULL DEFAULT 'Default',
            enabled BOOLEAN NOT NULL DEFAULT TRUE
        )",
        [],
    )
    .expect("Failed to create table");

    conn
}

pub fn get_all_settings(conn: &Connection) -> Result<Vec<Setting>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM settings").unwrap();

    let settings = stmt
        .query_map([], |row| {
            let exe_path: String = row.get(3).unwrap();
            Ok(Setting::from_db(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                PathBuf::from(exe_path),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
            ))
        })
        .unwrap()
        .collect::<Result<Vec<Setting>>>()
        .unwrap();

    Ok(settings)
}

// Cloud sync
// pub fn sync_all() {}
