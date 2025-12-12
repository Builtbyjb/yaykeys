use crate::utils;

use rusqlite::Result;
use utils::state::AppState;
use utils::types::App;

pub fn get_db_state(
    state: tauri::State<'_, AppState>,
    apps: &Vec<App>,
) -> Result<(Vec<String>, Vec<String>), rusqlite::Error> {
    // Returns a list of newly added apps and delete apps
    let db = state.db.lock().unwrap();

    // Create temporary table
    db.execute("CREATE TEMP TABLE name_list", []).unwrap();

    let mut stmt = db.prepare("INSERT INTO name_list VALUES(?)").unwrap();
    for app in apps {
        stmt.execute(&[app.name()]).unwrap();
    }
    drop(stmt);

    // Find name in list but not in table
    let query_in_list_only =
        "SELECT name FROM name_list WHERE name NOT IN (SELECT name FROM settings)";

    let mut stmt = db.prepare(&query_in_list_only).unwrap();
    let new_apps = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<String>>>()
        .unwrap();

    // Find names in table but not in list
    let query_in_table_only =
        "SELECT name FROM settings WHERE name NOT IN (SELECT name FROM name_list)";

    let mut stmt = db.prepare(&query_in_table_only).unwrap();
    let deleted_apps = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<String>>>()
        .unwrap();

    db.execute("DROP TABLE name_list", []).unwrap();

    Ok((new_apps, deleted_apps))
}
