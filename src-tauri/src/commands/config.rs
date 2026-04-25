use serde::Serialize;
use crate::config::loader::{AppConfig, load_config};

#[derive(Debug, Serialize, Clone)]
pub struct ConfigResponse {
    pub hypixel_api_url: String,
    pub craft_recursion_depth: u32,
    pub cache_duration: u64,
    pub skyblock_skills: Vec<String>,
}

/// Expose non-sensitive config to the frontend
#[tauri::command]
pub fn get_app_config() -> Result<ConfigResponse, String> {
    let config = load_config()?;
    Ok(ConfigResponse {
        hypixel_api_url: config.hypixel_api_url,
        craft_recursion_depth: config.craft_recursion_depth,
        cache_duration: config.cache_duration,
        skyblock_skills: config.skyblock_skills,
    })
}
