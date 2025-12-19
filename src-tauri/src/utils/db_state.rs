use std::path::{Path, PathBuf};

use crate::types;
use crate::utils;

use rusqlite::Result;
use types::app::App;
use types::setting::Setting;
use utils::state::AppState;
use utils::storage;

pub fn get_db_state(
    state: tauri::State<'_, AppState>,
    apps: &Vec<App>,
) -> Result<Vec<Setting>, rusqlite::Error> {
    let conn = state.conn.lock().unwrap();

    // Create temporary table
    conn.execute("CREATE TEMP TABLE name_list (name TEXT NOT NULL)", [])
        .expect("Failed to create temp table");

    let mut stmt = conn.prepare("INSERT INTO name_list VALUES(?)").unwrap();
    for app in apps {
        stmt.execute([app.name()]).unwrap();
    }
    drop(stmt);

    // Delete the names in table but not in list
    let mut stmt = conn
        .prepare("DELETE FROM settings WHERE name NOT IN (SELECT name FROM name_list)")
        .unwrap();
    stmt.raw_execute().unwrap();

    // Find name in list but not in table
    for app in apps {
        let mut stmt = conn
            .prepare("SELECT name FROM settings WHERE name= ?")
            .unwrap();

        let mut rows = stmt.query([app.name()]).unwrap();

        match rows.next() {
            Ok(Some(_)) => {
                // println!("App Exists")
            }
            Ok(None) => {
                let exe_path = get_exe_path(app.folder_path(), app.name());
                let setting = Setting::new(app.name().to_string(), PathBuf::from(exe_path));
                setting.insert(&conn).unwrap()
            }
            Err(_) => {
                println!("Here Err");
            }
        }
    }

    conn.execute("DROP TABLE name_list", []).unwrap();

    let settings = storage::get_all_settings(&conn).unwrap();

    Ok(settings)
}

fn get_exe_path(folder_path: &Path, name: &str) -> String {
    format!("{}/Contents/MacOs/{}", folder_path.to_str().unwrap(), name).to_string()
}
