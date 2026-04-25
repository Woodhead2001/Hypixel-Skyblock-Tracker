#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod api;
mod config;

use tauri::Manager;
use commands::*;
use api::hypixel_api::{get_player_uuid, fetch_and_cache_profiles};
use config::loader::load_config;

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
            // -------------------------------
            // ⭐ Fetch SkyBlock data on startup
            // -------------------------------
            tauri::async_runtime::spawn(async move {
                let config = match load_config() {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        eprintln!("Failed to load config: {}", e);
                        return;
                    }
                };
                
                let username = &config.default_username;
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
            get_minions,
            get_app_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}