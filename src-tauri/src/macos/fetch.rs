use crate::utils;

use std::fs;
use std::path::Path;
use utils::db_state;
use utils::state::AppState;
use utils::storage;
use utils::types::{App, Setting};

pub fn fetch(state: tauri::State<'_, AppState>) -> Vec<Setting> {
    let dirs = [
        "/Applications",
        "/Applications/Utilities",
        // "/Applications/Python 3.14",
        "/System/Applications",
        "/System/Applications/Utilities",
        // "/System/Library/CoreServices",
        "/System/Library/CoreServices/Applications",
        "/System/Library/CoreServices/Finder.app/Contents/Applications",
        "/System/Volumes/Preboot/Cryptexes/App/System/Applications",
    ];

    let apps = get_apps(&dirs);
    let (new_apps, deleted_apps) = db_state::get_db_state(state, &apps).unwrap();

    if new_apps.len() > 0 {
        // pub struct Setting {
        //     name: String,
        //     hotkey: String,
        //     app_type: String,
        //     path: String,
        //     option: String,
        //     enabled: bool,
        // }
        for new_app in new_apps {
            let name = new_app;
            storage::set()
        }
    }

    if deleted_apps.len() > 0 {
        // Remove deleted apps from database
    }

    // Get all settings

    // Return all settings

    return apps;
}

fn get_apps(dirs: &[&str]) -> Vec<App> {
    let mut apps: Vec<App> = Vec::new();

    for dir in dirs {
        let path = Path::new(dir);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".app") {
                        let app_name = name.strip_suffix(".app").unwrap();
                        apps.push(App::new(app_name.to_owned(), path.to_path_buf()))
                    }
                }
            }
        }
    }

    apps
}
