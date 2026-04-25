// src-tauri/src/api/hypixel_api.rs

use reqwest::Client;
use serde_json::Value;

use crate::config::loader::{AppConfig, load_config};

/// Convenience wrapper so we don't call load_config() everywhere
fn get_config() -> Result<AppConfig, String> {
    load_config()
}

/// Fetch Mojang UUID using the configured Mojang URL
pub async fn get_player_uuid(username: &str) -> Result<String, String> {
    let config = get_config()?;
    let client = Client::new();

    let url = format!("{}{}", config.minecraft_uuid_lookup_url, username);

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Player not found".into());
    }

    let data: Value = response.json().await.map_err(|e| e.to_string())?;

    data.get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or("Invalid UUID response".into())
}

/// Fetch raw SkyBlock profiles JSON from Hypixel
pub async fn fetch_raw_profiles(uuid: &str) -> Result<Value, String> {
    let config = get_config()?;
    let client = Client::new();

    // Example config value:
    // "hypixel_profiles_url": "https://api.hypixel.net/v2/skyblock/profiles?uuid="
    let url = format!(
        "{}{}&key={}",
        config.hypixel_profiles_url,
        uuid,
        config.hypixel_api_key
    );

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let data: Value = response.json().await.map_err(|e| e.to_string())?;

    Ok(data) // return raw JSON exactly as Hypixel gives it
}
