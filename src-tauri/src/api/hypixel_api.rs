use reqwest::Client;
use serde_json::{json, Value};
use log::{info, error, debug};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::config::loader::{AppConfig, load_config};

/// Global cache for SkyBlock profiles
pub static PROFILE_CACHE: Lazy<Mutex<Option<Value>>> = Lazy::new(|| Mutex::new(None));

fn get_config() -> Result<AppConfig, String> {
    load_config()
}

/// Fetch Mojang UUID
pub async fn get_player_uuid(username: &str) -> Result<String, String> {
    info!("Resolving UUID for username: {}", username);
    let config = get_config()?;
    let client = Client::new();

    let url = format!("{}{}", config.minecraft_uuid_lookup_url, username);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Failed to resolve UUID".into());
    }

    let data: Value = response.json().await.map_err(|e| e.to_string())?;
    data.get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or("Invalid UUID response".into())
}

/// Fetch raw profiles from Hypixel
pub async fn fetch_raw_profiles(uuid: &str) -> Result<Value, String> {
    info!("Fetching raw profiles for UUID: {}", uuid);
    let config = get_config()?;
    let client = Client::new();

    let url = format!("{}{}", config.hypixel_profiles_url, uuid);
    debug!("Constructed URL: {}", url);

    let response = client
        .get(&url)
        .header("API-Key", &config.hypixel_api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(data)
}

/// Fetch and store profiles in cache
pub async fn fetch_and_cache_profiles(uuid: &str) -> Result<(), String> {
    let data = fetch_raw_profiles(uuid).await?;

    // Wrap the Hypixel response with the UUID so skills.rs can use it
    let wrapped = json!({
        "uuid": uuid,
        "profiles": data["profiles"]
    });

    let mut cache = PROFILE_CACHE.lock().unwrap();
    *cache = Some(wrapped);

    info!("SkyBlock profiles cached successfully");
    Ok(())
}

/// Retrieve cached profiles
pub fn get_cached_profiles() -> Result<Value, String> {
    let cache = PROFILE_CACHE.lock().unwrap();
    cache.clone().ok_or("No cached profiles available".into())
}
