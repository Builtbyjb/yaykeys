use crate::utils;

use std::fs;
use std::path::Path;
use utils::types::App;

pub fn fetch() -> Vec<String> {
    // Fresh the system

    // Check for new apps
    // Check for deleted apps

    // update app struct
    let mut apps: Vec<String> = Vec::new();
    let app_dirs = [
        "/Applications",
        "/Applications/Utilities",
        "/System/Applications",
        "/System/Applications/Utilities",
    ];

    // System applications
    let system_apps = Path::new("/System/Applications");
    if let Ok(entries) = fs::read_dir(system_apps) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".app") {
                    apps.push(name.to_string())
                }
            }
        }
    }

    // User applications
    // if let Some(home) = dirs::home_dir() {
    //     let user_apps = home.join("Applications");
    //     if let Ok(entries) = fs::read_dir(user_apps) {
    //         for entry in entries.flatten() {
    //             if let Some(name) = entry.file_name().to_str() {
    //                 if name.ends_with(".app") {
    //                     apps.push(name.to_string())
    //                 }
    //             }
    //         }
    //     }
    // }

    return apps;
}
