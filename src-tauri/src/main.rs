#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::{info, error};

mod api;
mod config;
mod utils;
mod icons;
mod commands;

use crate::utils::cats_extractor::extract_cats;
use crate::utils::pack_loader::extract_pack_cats;
use crate::utils::png_extractor::extract_pngs_from_pack;

use crate::icons::{get_item_icon, export_item_icon};
use crate::commands::{
    fetch_hypixel_player,
    get_player_profiles,
    get_player_skills,
    get_minions,
    get_app_config,
    get_player_collections,
    debug_player_data,
};

fn init_logging() {
    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .try_init();
    info!("Logging initialized");
}

fn run_icon_extractor() {
    info!("Extracting PNG icons from resource pack…");

    if let Err(e) = extract_pngs_from_pack() {
        error!("PNG extraction failed: {}", e);
    } else {
        info!("PNG extraction complete.");
    }
}


fn main() {
    init_logging();
    info!("Starting backend…");

    run_icon_extractor();

    info!("Launching Tauri…");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_hypixel_player,
            get_player_profiles,
            get_player_skills,
            get_minions,
            get_app_config,
            get_player_collections,
            debug_player_data,
            get_item_icon,
            export_item_icon,
        ])
        .setup(|_app| {
            info!("Tauri setup complete. App ready.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
