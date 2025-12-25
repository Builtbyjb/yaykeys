use std::path::{Path, PathBuf};

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
