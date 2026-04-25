use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;
use crate::config::loader::load_config;
use log::info;
use reqwest::Client;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MINION_DEFINITIONS_CACHE: Lazy<Mutex<Option<Value>>> =
    Lazy::new(|| Mutex::new(None));

/// Fetch ALL minions from /skyblock/items
async fn fetch_all_minions() -> Result<Value, String> {
    info!("Fetching ALL minions from Hypixel items API");

    // Step 1: check cache
    {
        let cache = MINION_DEFINITIONS_CACHE.lock().unwrap();
        if let Some(cached) = cache.clone() {
            return Ok(cached);
        }
    }

    let config = load_config()?;
    let client = Client::new();
    let url = &config.hypixel_items_url;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let full: Value = response.json().await.map_err(|e| e.to_string())?;

    let mut minions: Vec<String> = vec![];

    if let Some(items) = full["items"].as_array() {
        for item in items {
            if let Some(id) = item["id"].as_str() {
                // Match CARROT_GENERATOR_1, FLOWER_GENERATOR_1, etc.
                if id.contains("_GENERATOR_") {
                    let base = id.split("_GENERATOR_").next().unwrap_or(id);

                    // Convert to readable name
                    let name = base
                        .to_lowercase()
                        .split('_')
                        .map(|w| {
                            let mut chars = w.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    minions.push(name);
                }
            }
        }
    }

    // Deduplicate
    minions.sort();
    minions.dedup();

    let result = json!({ "minions": minions });

    // Cache it
    {
        let mut cache = MINION_DEFINITIONS_CACHE.lock().unwrap();
        *cache = Some(result.clone());
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_minions(cute_name: String) -> Result<Value, String> {
    info!("Fetching minions for cute_name={}", cute_name);

    // Load cached profiles
    let data = get_cached_profiles()?;

    let uuid = data["uuid"]
        .as_str()
        .ok_or("Cached data missing UUID")?
        .to_string();

    let profiles = data["profiles"]
        .as_array()
        .ok_or("Invalid profile format")?;

    // Find profile by cute_name
    let profile = profiles
        .iter()
        .find(|p| p["cute_name"].as_str() == Some(cute_name.as_str()))
        .ok_or("Profile not found for this cute_name")?;

    // Ensure player is in this profile
    let members = profile["members"]
        .as_object()
        .ok_or("Invalid members format")?;

    let member = members
        .get(&uuid)
        .ok_or("This player is not a member of this profile")?;

    // Crafted minions list
    let crafted = member["player_data"]["crafted_generators"]
        .as_array()
        .unwrap_or(&vec![])
        .clone();

    // Fetch ALL minion names
    let defs = fetch_all_minions().await?;
    let names = defs["minions"].as_array().unwrap();

    let mut result_list = vec![];

    for name_val in names {
        let name = name_val.as_str().unwrap_or("Unknown");
        let id = name.to_uppercase().replace(" ", "_");

        let mut tiers = vec![];

        for tier_num in 1..=12 {
            let key = format!("{}_{}", id, tier_num);

            let owned = crafted
                .iter()
                .any(|v| v.as_str() == Some(&key));

            tiers.push(json!({
                "tier": tier_num,
                "owned": owned
            }));
        }

        result_list.push(json!({
            "id": id,
            "name": name,
            "tiers": tiers
        }));
    }

    Ok(json!({
        "uuid": uuid,
        "cute_name": cute_name,
        "minions": result_list
    }))
}
