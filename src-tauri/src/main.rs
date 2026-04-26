#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::{info, error};
use tauri::Manager;

mod icons;
mod commands;

use icons::cats_extractor::extract_cats;
use commands::icons::{get_item_icon, export_item_icon};

fn init_logging() {
    // Simple logger for development; replace with env_logger if you prefer.
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to init logger");
}

fn run_cats_extractor() {
    info!("Running CATS extractor at startup…");

    if let Err(e) = extract_cats("pack.cats", "icons/skyblock") {
        error!("CATS extraction failed: {}", e);
    } else {
        info!("CATS extraction completed successfully.");
    }
}

fn main() {
    init_logging();
    info!("Starting backend…");

    run_cats_extractor();

    info!("Launching Tauri…");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_item_icon,
            export_item_icon
        ])
        .setup(|app| {
            info!("Tauri setup complete. App ready.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
