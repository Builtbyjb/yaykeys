use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct Setting {
    name: String,
    hotkey: String,
    app_type: String,
    exe_path: PathBuf,
    mode: String,
    enabled: bool,
}

impl Setting {
    pub fn new(name: String, exe_path: PathBuf) -> Self {
        Self {
            name,
            hotkey: "".to_string(),
            app_type: "Application".to_string(),
            exe_path,
            mode: "default".to_string(),
            enabled: true,
        }
    }

    pub fn from_db(
        name: String,
        hotkey: String,
        app_type: String,
        exe_path: PathBuf,
        mode: String,
        enabled: bool,
    ) -> Self {
        Self {
            name,
            hotkey,
            app_type,
            exe_path,
            mode,
            enabled,
        }
    }

    pub fn get() {}

    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO settings (name, hotkey, app_type, exe_path, mode, enabled) VALUES ( ?, ?, ?, ?, ?, ?)",
            params![
                self.name,
                self.hotkey,
                self.app_type,
                self.exe_path.to_str(),
                self.mode,
                self.enabled
            ],
        )
        .unwrap();
        Ok(())
    }

    pub fn update() {}

    pub fn delete() {}

    pub fn search() {}

    pub fn sync() {}
}
