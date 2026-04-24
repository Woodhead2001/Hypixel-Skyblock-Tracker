// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;
mod api;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).ok();

            let db_path = app_data_dir.join("skyblock_tracker.db");

            db::init_db(&db_path).expect("Failed to initialize database");

            // Store database path in app state for commands
            app.manage(db::DbState::new(db_path));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_player_data,
            commands::save_player_data,
            commands::fetch_hypixel_player,
            commands::get_player_skills,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
