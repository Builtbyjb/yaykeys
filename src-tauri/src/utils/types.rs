use std::path::{Path, PathBuf};

#[derive(Debug)]
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
            app_type: "application".to_string(),
            exe_path,
            mode: "default".to_string(),
            enabled: true,
        }
    }
}

pub struct App {
    name: String,
    folder_path: PathBuf,
}

impl App {
    pub fn new(name: String, folder_path: PathBuf) -> Self {
        Self { name, folder_path }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn folder_path(&self) -> &Path {
        &self.folder_path
    }
}
