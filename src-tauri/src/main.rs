#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;
mod api;
mod config;

use tauri::Manager;
use commands::*;
use api::hypixel_api::{get_player_uuid, fetch_and_cache_profiles};

fn main() {
    env_logger::init();

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
            app.manage(db::DbState::new(db_path));

            // -------------------------------
            // ⭐ Fetch SkyBlock data on startup
            // -------------------------------
            tauri::async_runtime::spawn(async move {
                let username = "roninsghost68"; // later: make dynamic
                if let Ok(uuid) = get_player_uuid(username).await {
                    let _ = fetch_and_cache_profiles(&uuid).await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_hypixel_player,
            get_player_skills,
            get_player_profiles,
            get_minions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}