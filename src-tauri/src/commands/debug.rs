use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;
use log::info;
use reqwest::Client;
use crate::config::loader::load_config;

#[tauri::command]
pub async fn debug_player_data(profile_id: String) -> Result<Value, String> {
    info!("Debug: Fetching player data structure for profile_id={}", profile_id);

    let data = get_cached_profiles()?;

    let uuid = data["uuid"]
        .as_str()
        .ok_or("Cached data missing UUID")?
        .to_string();

    let profiles = data["profiles"]
        .as_array()
        .ok_or("Invalid profile format")?;

    let profile = profiles
        .iter()
        .find(|p| p["profile_id"] == profile_id)
        .ok_or("Profile not found")?;

    let members = profile["members"]
        .as_object()
        .ok_or("Invalid members format")?;

    let member = members
        .get(&uuid)
        .ok_or("This player is not a member of this profile")?;

    // Get all top-level keys
    let top_keys: Vec<String> = member
        .as_object()
        .map(|m| m.keys().map(|k| k.to_string()).collect())
        .unwrap_or_default();

    // Check for various potential locations of collections data
    let has_unlocked_coll_tiers_top = member["unlocked_coll_tiers"].is_object();
    let has_unlocked_coll_tiers_nested = member["player_data"]["unlocked_coll_tiers"].is_object();
    let has_collections = member["collections"].is_object();

    // Try to get sample collection tiers
    let mut sample_collections = Vec::new();
    if has_unlocked_coll_tiers_top {
        if let Some(obj) = member["unlocked_coll_tiers"].as_object() {
            sample_collections = obj.keys().take(5).map(|k| k.to_string()).collect();
        }
    } else if has_unlocked_coll_tiers_nested {
        if let Some(obj) = member["player_data"]["unlocked_coll_tiers"].as_object() {
            sample_collections = obj.keys().take(5).map(|k| k.to_string()).collect();
        }
    } else if has_collections {
        if let Some(obj) = member["collections"].as_object() {
            sample_collections = obj.keys().take(5).map(|k| k.to_string()).collect();
        }
    }

    // Also fetch and show collection tier API structure
    let config = load_config()?;
    let client = Client::new();
    let url = format!("{}/v2/resources/skyblock/collections", config.hypixel_api_url);
    
    let mut collection_tier_keys = Vec::new();
    if let Ok(response) = client.get(&url).send().await {
        if let Ok(tier_data) = response.json::<Value>().await {
            if let Some(collections) = tier_data["collections"].as_object() {
                collection_tier_keys = collections
                    .keys()
                    .take(10)
                    .map(|k| k.to_string())
                    .collect();
            }
        }
    }

    Ok(json!({
        "top_level_keys": top_keys,
        "has_unlocked_coll_tiers_top_level": has_unlocked_coll_tiers_top,
        "has_unlocked_coll_tiers_in_player_data": has_unlocked_coll_tiers_nested,
        "has_collections_field": has_collections,
        "sample_collection_names_from_player": sample_collections,
        "sample_collection_keys_from_api": collection_tier_keys,
    }))
}
