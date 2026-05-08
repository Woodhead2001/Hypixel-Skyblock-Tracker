use serde_json::{json, Value};
use log::{info, debug, error};
use reqwest::Client;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::config::loader::load_config;
use crate::api::hypixel_api::get_cached_profiles;
use crate::icons::get_icon;

static COLLECTION_TIERS_CACHE: Lazy<Mutex<Option<Value>>> =
    Lazy::new(|| Mutex::new(None));

async fn fetch_collection_tiers() -> Result<Value, String> {
    {
        let cache = COLLECTION_TIERS_CACHE.lock().unwrap();
        if let Some(cached) = cache.clone() {
            debug!("Using cached collection tiers");
            return Ok(cached);
        }
    }

    let config = load_config()?;
    let url = format!("{}/v2/resources/skyblock/collections", config.hypixel_api_url);

    info!("Fetching collection tiers from {}", url);

    let client = Client::new();
    let response = client.get(&url).send().await;

    if let Err(e) = response {
        error!("Failed to fetch collection tiers: {}", e);
        return Err(format!("Failed to fetch collection tiers: {}", e));
    }

    let bytes = response.unwrap().bytes().await;
    if let Err(e) = bytes {
        error!("Failed to read collection tiers body: {}", e);
        return Err(format!("Failed to read collection tiers body: {}", e));
    }

    let data: Value = serde_json::from_slice(&bytes.unwrap())
        .map_err(|e| {
            error!("Failed to parse collection tiers JSON: {}", e);
            e.to_string()
        })?;

    {
        let mut cache = COLLECTION_TIERS_CACHE.lock().unwrap();
        *cache = Some(data.clone());
    }

    Ok(data)
}

fn extract_player_collections(member: &Value) -> std::collections::HashMap<String, u64> {
    let mut map = std::collections::HashMap::new();

    let paths = [
        &member["collection"],
        &member["player_data"]["collection"],
    ];

    for p in paths {
        if let Some(obj) = p.as_object() {
            for (k, v) in obj {
                if let Some(num) = v.as_u64() {
                    map.insert(k.clone(), num);
                }
            }
        }
    }

    map
}

#[tauri::command]
pub async fn get_player_collections(profile_id: String) -> Result<Value, String> {
    info!("Building collections for profile {}", profile_id);

    let data = get_cached_profiles()?;
    let uuid = data["uuid"].as_str().ok_or("Missing UUID")?;
    let profiles = data["profiles"].as_array().ok_or("Invalid profiles")?;

    let profile = profiles
        .iter()
        .find(|p| p["profile_id"] == profile_id)
        .ok_or("Profile not found")?;

    let members = profile["members"].as_object().ok_or("Invalid members")?;
    let member = members.get(uuid).ok_or("Player not in profile")?;

    let player_collections = extract_player_collections(member);

    let tiers_data = fetch_collection_tiers().await?;

    let api_collections = tiers_data["collections"]
        .as_object()
        .ok_or("Invalid API collection structure")?;

    let mut grouped = serde_json::Map::new();

    for (skill_name, skill_data) in api_collections {
        let mut items_out = vec![];
        let mut total_tiers = 0u32;
        let mut completed_tiers = 0u32;

        if let Some(items) = skill_data["items"].as_object() {
            for (collection_id, item_info) in items {
                let count = player_collections.get(collection_id).cloned().unwrap_or(0);
                let max_tier = item_info["maxTiers"].as_u64().unwrap_or(0) as u32;

                let mut current_tier = 0u32;
                let mut next_required = 0u64;

                if let Some(tiers) = item_info["tiers"].as_array() {
                    total_tiers += tiers.len() as u32;

                    for t in tiers {
                        let tier_num = t["tier"].as_u64().unwrap_or(0) as u32;
                        let required = t["amountRequired"].as_u64().unwrap_or(0);

                        if count >= required {
                            current_tier = current_tier.max(tier_num);
                        }
                    }

                    if let Some(next) = tiers.iter()
                        .find(|t| t["tier"].as_u64().unwrap_or(0) == (current_tier as u64 + 1))
                    {
                        next_required = next["amountRequired"].as_u64().unwrap_or(0);
                    }

                    completed_tiers += current_tier;
                }

                let progress = if next_required > 0 {
                    (count as f64 / next_required as f64).min(1.0)
                } else { 1.0 };

                let maxed = current_tier >= max_tier && max_tier > 0;

                let icon = get_icon(collection_id);

                items_out.push(json!({
                    "id": collection_id,
                    "name": item_info["name"],
                    "tier": current_tier,
                    "max_tier": max_tier,
                    "count": count,
                    "next_required": next_required,
                    "progress": progress,
                    "maxed": maxed,
                    "icon": icon
                }));
            }
        } else {
            error!("Skill {} has no items object", skill_name);
        }

        grouped.insert(skill_name.clone(), json!({
            "items": items_out,
            "total_tiers": total_tiers,
            "completed_tiers": completed_tiers
        }));
    }

    Ok(json!({
        "collections": grouped
    }))
}
