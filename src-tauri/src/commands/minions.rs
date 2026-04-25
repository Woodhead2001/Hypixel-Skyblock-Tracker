use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;
use log::{info, debug};
use reqwest::Client;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MINION_DEFINITIONS_CACHE: Lazy<Mutex<Option<Value>>> = Lazy::new(|| Mutex::new(None));

async fn fetch_minion_definitions() -> Result<Value, String> {
    // Step 1: check cache WITHOUT awaiting
    {
        let cache = MINION_DEFINITIONS_CACHE.lock().unwrap();
        if let Some(cached) = cache.clone() {
            return Ok(cached);
        }
    } // lock dropped here

    // Step 2: perform async request safely
    let url = "https://api.hypixel.net/v2/resources/skyblock/collections";
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: Value = response.json().await.map_err(|e| e.to_string())?;

    // Step 3: store in cache AFTER await
    {
        let mut cache = MINION_DEFINITIONS_CACHE.lock().unwrap();
        *cache = Some(data.clone());
    }

    Ok(data)
}

#[tauri::command]
pub async fn get_minions() -> Result<Value, String> {
    info!("Fetching minions…");

    let data = get_cached_profiles()?;

    // UUID *is* the profile_id
    let uuid = data["uuid"]
        .as_str()
        .ok_or("Cached data missing UUID")?
        .to_string();

    info!("Using profile_id = uuid = {}", uuid);

    let profiles = data["profiles"]
        .as_array()
        .ok_or("Invalid profile format")?;

    // Find the profile where profile_id == uuid
    let profile = profiles
        .iter()
        .find(|p| p["profile_id"] == uuid)
        .ok_or("Profile not found for this UUID")?;

    let members = profile["members"]
        .as_object()
        .ok_or("Invalid members format")?;

    let member = members
        .get(&uuid)
        .ok_or("This player is not a member of this profile")?;

    let crafted = member["player_data"]["crafted_generators"]
        .as_array()
        .unwrap_or(&vec![])
        .clone();

    let defs = fetch_minion_definitions().await?;

    let mut result = vec![];

    if let Some(collections) = defs["collections"].as_object() {
        for (collection_id, collection) in collections {
            if let Some(minions) = collection["minion_tiers"].as_array() {
                let mut tiers = vec![];

                for tier in minions {
                    let tier_num = tier["tier"].as_u64().unwrap_or(0) as u32;

                    let key = format!("{}_{}", collection_id.to_uppercase(), tier_num);

                    let owned = crafted
                        .iter()
                        .any(|v| v.as_str() == Some(&key));

                    tiers.push(json!({
                        "tier": tier_num,
                        "owned": owned
                    }));
                }

                result.push(json!({
                    "id": collection_id,
                    "name": collection["name"].as_str().unwrap_or(collection_id),
                    "tiers": tiers
                }));
            }
        }
    }

    Ok(json!({
        "uuid": uuid,
        "minions": result
    }))
}
