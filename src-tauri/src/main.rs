#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;
mod api;
mod config;

use tauri::Manager;
use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).ok();

            let db_path = app_data_dir.join("skyblock_tracker.db");

            db::init_db(&db_path).expect("Failed to initialize database");

            // Store database path in app state
            app.manage(db::DbState::new(db_path));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_hypixel_player,
            get_player_skills,
            get_player_profiles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
