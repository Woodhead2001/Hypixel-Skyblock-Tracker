use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;
use crate::config::loader::load_config;
use crate::icons::get_icon;
use log::info;
use reqwest::Client;
use std::collections::{BTreeSet, HashSet};

/// Convert crafted format (ZOMBIE_8) → generator format (ZOMBIE_GENERATOR_8)
fn crafted_to_generator(id: &str) -> Option<String> {
    if let Some((base, tier)) = id.rsplit_once('_') {
        if tier.parse::<u32>().is_ok() {
            return Some(format!("{}_GENERATOR_{}", base, tier));
        }
    }
    None
}

/// Extract NAME_GENERATOR from NAME_GENERATOR_1
fn extract_base(id_str: &str) -> String {
    if let Some((base, tier)) = id_str.rsplit_once('_') {
        if tier.parse::<u32>().is_ok() {
            return base.to_string();
        }
    }
    id_str.to_string()
}

/// Fetch ALL minion bases from Hypixel items API.
async fn fetch_all_minion_bases() -> Result<BTreeSet<String>, String> {
    let config = load_config()?;
    let client = Client::new();

    let response = client
        .get(&config.hypixel_items_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let full: Value = response.json().await.map_err(|e| e.to_string())?;

    let mut bases = BTreeSet::new();

    if let Some(items) = full["items"].as_array() {
        for item in items {
            if let Some(id) = item["id"].as_str() {
                // Only accept NAME_GENERATOR_<tier>
                if let Some((base, tier)) = id.rsplit_once('_') {
                    if tier.parse::<u32>().is_ok() && base.ends_with("_GENERATOR") {
                        bases.insert(base.to_string());
                    }
                }
            }
        }
    }

    Ok(bases)
}

#[tauri::command]
pub async fn get_minions(profileId: String) -> Result<Value, String> {
    info!("Fetching minions for profileId={}", profileId);

    // Load cached profiles
    let data = get_cached_profiles()?;

    let uuid = data["uuid"]
        .as_str()
        .ok_or("Cached data missing UUID")?
        .to_string();

    let profiles = data["profiles"]
        .as_array()
        .ok_or("Invalid profile format")?;

    // Find profile
    let profile = profiles
        .iter()
        .find(|p| p["profile_id"].as_str() == Some(profileId.as_str()))
        .ok_or("Profile not found")?;

    // Get member
    let members = profile["members"]
        .as_object()
        .ok_or("Invalid members format")?;

    let member = members
        .get(&uuid)
        .ok_or("Player not in this profile")?;

    // Crafted minions list
    let crafted_raw = member["player_data"]["crafted_generators"]
        .as_array()
        .unwrap_or(&vec![])
        .clone();

    // Convert crafted entries to generator format
    let crafted_set: HashSet<String> = crafted_raw
        .iter()
        .filter_map(|v| v.as_str())
        .filter_map(|s| crafted_to_generator(s))
        .collect();

    info!("Crafted entries converted: {}", crafted_set.len());

    // Fetch ALL minion bases from /skyblock/items
    let all_bases = fetch_all_minion_bases().await?;

    let mut result_list = vec![];

    for base in all_bases {
        let pretty = base
            .replace("_GENERATOR", "")
            .to_lowercase()
            .replace('_', " ");

        let mut tiers = vec![];

        for tier_num in 1..=12 {
            let key = format!("{}_{}", base, tier_num);
            let owned = crafted_set.contains(&key);
            tiers.push(owned);
        }

        result_list.push(json!({
            "id": base,
            "name": pretty,
            "icon": get_icon(&base),
            "tiers": tiers
        }));
    }

    Ok(json!({
        "profileId": profileId,
        "minions": result_list
    }))
}
