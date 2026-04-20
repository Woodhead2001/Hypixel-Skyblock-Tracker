// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.app_handle();
      
      // Initialize database
      let db_path = app.path_resolver()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("skyblock_tracker.db");
      
      db::init_db(&db_path).expect("Failed to initialize database");
      
      // Store database path in app state for commands
      app.manage(db::DbState::new(db_path));
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_player_data,
      commands::add_recipe,
      commands::get_recipes,
      commands::add_goal,
      commands::get_goals,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

