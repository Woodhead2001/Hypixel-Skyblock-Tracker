use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;
use log::info;
use reqwest::Client;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MINION_DEFINITIONS_CACHE: Lazy<Mutex<Option<Value>>> =
    Lazy::new(|| Mutex::new(None));

async fn fetch_minion_definitions() -> Result<Value, String> {
    info!("Fetching collection definitions from Hypixel API");

    // Step 1: check cache
    {
        let cache = MINION_DEFINITIONS_CACHE.lock().unwrap();
        if let Some(cached) = cache.clone() {
            return Ok(cached);
        }
    }

    // Step 2: fetch full data
    let url = "https://api.hypixel.net/v2/resources/skyblock/collections";
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let full: Value = response.json().await.map_err(|e| e.to_string())?;

    // Step 3: build grouped structure: skill → [item names]
    let mut grouped = json!({ "collections": {} });

    if let Some(collections) = full["collections"].as_object() {
        for (skill_id, skill_data) in collections {
            // Convert skill name to Title Case
            let skill_name = skill_id
                .to_lowercase()
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>();

            let mut item_list = vec![];

            // Correct path: items are inside skill_data["items"]
            if let Some(item_map) = skill_data["items"].as_object() {
                for (item_id, item_data) in item_map {
                    let name = item_data["name"].as_str().unwrap_or(item_id);
                    item_list.push(json!(name));
                }
            }

            grouped["collections"][skill_name] = json!(item_list);
        }
    }

    // Step 4: cache result
    {
        let mut cache = MINION_DEFINITIONS_CACHE.lock().unwrap();
        *cache = Some(grouped.clone());
    }

    Ok(grouped)
}

#[tauri::command]
pub async fn get_minions(cute_name: String) -> Result<Value, String> {
    info!("Fetching grouped minions for cute_name={}", cute_name);

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

    // Fetch grouped collection names
    let defs = fetch_minion_definitions().await?;

    let mut result = json!({});

    // Build 12-tier minions grouped by skill
    if let Some(groups) = defs["collections"].as_object() {
        for (skill, items) in groups {
            let mut skill_list = vec![];

            for item_name in items.as_array().unwrap() {
                let name = item_name.as_str().unwrap_or("Unknown");
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

                skill_list.push(json!({
                    "id": id,
                    "name": name,
                    "tiers": tiers
                }));
            }

            result[skill] = json!(skill_list);
        }
    }

    Ok(json!({
        "uuid": uuid,
        "cute_name": cute_name,
        "collections": result
    }))
}
