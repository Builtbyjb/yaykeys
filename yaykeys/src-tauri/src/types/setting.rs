use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct Setting {
    id: u16,
    name: String,
    hotkey: Option<String>,
    exe_path: PathBuf,
    mode: String,
    enabled: bool,
}

impl Setting {
    pub fn from_db(
        id: u16,
        name: String,
        hotkey: Option<String>,
        exe_path: PathBuf,
        mode: String,
        enabled: bool,
    ) -> Self {
        Self {
            id,
            name,
            hotkey,
            exe_path,
            mode,
            enabled,
        }
    }

    // pub fn get() {}

    pub fn insert(conn: &Connection, name: String, exe_path: PathBuf) -> Result<()> {
        conn.execute(
            "INSERT INTO settings (name, exe_path) VALUES ( ?, ?)",
            params![name, exe_path.to_str()],
        )
        .unwrap();
        Ok(())
    }

    // pub fn update() {}

    // pub fn delete() {}

    // pub fn search() {}

    // pub fn sync() {}
}
