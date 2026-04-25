// src-tauri/src/commands/skills.rs

use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;
use log::{info, debug};

/// Hypixel skill XP per level (1–60)
const SKILL_XP_PER_LEVEL: [f64; 60] = [
    50.0, 125.0, 200.0, 300.0, 500.0, 750.0, 1000.0, 1500.0, 2000.0, 3500.0,
    5000.0, 7500.0, 10000.0, 15000.0, 20000.0, 30000.0, 50000.0, 75000.0,
    100000.0, 200000.0, 300000.0, 400000.0, 500000.0, 600000.0, 700000.0,
    800000.0, 900000.0, 1000000.0, 1100000.0, 1200000.0, 1300000.0,
    1400000.0, 1500000.0, 1600000.0, 1700000.0, 1800000.0, 1900000.0,
    2000000.0, 2100000.0, 2200000.0, 2300000.0, 2400000.0, 2500000.0,
    2600000.0, 2750000.0, 2900000.0, 3100000.0, 3400000.0, 3700000.0,
    4000000.0, 4300000.0, 4600000.0, 4900000.0, 5200000.0, 5500000.0,
    5800000.0, 6100000.0, 6400000.0, 6700000.0, 7000000.0
];

fn xp_to_level_and_progress(xp: f64) -> (u32, f64) {
    let mut accumulated = 0.0;

    for (i, req) in SKILL_XP_PER_LEVEL.iter().enumerate() {
        let next_total = accumulated + req;

        if xp < next_total {
            let level = i as u32;
            let progress = (xp - accumulated) / req;

            debug!(
                "XP {} → level {} (accumulated={}, req={}, progress={})",
                xp, level, accumulated, req, progress
            );

            return (level, progress.clamp(0.0, 1.0));
        }

        accumulated = next_total;
    }

    (60, 1.0)
}

fn map_skill_key(key: &str) -> Option<&'static str> {
    match key {
        "SKILL_FARMING" => Some("farming"),
        "SKILL_MINING" => Some("mining"),
        "SKILL_COMBAT" => Some("combat"),
        "SKILL_FORAGING" => Some("foraging"),
        "SKILL_FISHING" => Some("fishing"),
        "SKILL_ENCHANTING" => Some("enchanting"),
        "SKILL_ALCHEMY" => Some("alchemy"),
        "SKILL_TAMING" => Some("taming"),
        "SKILL_RUNECRAFTING" => Some("runecrafting"),
        "SKILL_CARPENTRY" => Some("carpentry"),
        "SKILL_HUNTING" => Some("hunting"),
        _ => None,
    }
}

#[tauri::command]
pub async fn get_player_skills(profile_id: String) -> Result<Value, String> {
    info!("Fetching skills for profile_id={}", profile_id);

    let data = get_cached_profiles()?;

    // Extract the UUID we originally fetched from Mojang
    let uuid = data["uuid"]
        .as_str()
        .ok_or("Cached data missing UUID")?
        .to_string();

    info!("Resolved active player UUID from cache: {}", uuid);

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

    // FIX: Select the correct member using the cached UUID
    let member = members
        .get(&uuid)
        .ok_or("This player is not a member of this profile")?;

    info!("Using correct member UUID: {}", uuid);

    let experience = member["player_data"]["experience"]
        .as_object()
        .ok_or("No experience data found")?;

    let mut skills = serde_json::Map::new();

    for (key, value) in experience.iter() {
        if let Some(name) = map_skill_key(key) {
            if let Some(xp) = value.as_f64() {
                let (level, progress) = xp_to_level_and_progress(xp);

                info!(
                    "Skill {} → XP={}, Level={}, Progress={:.2}%",
                    name, xp, level, progress * 100.0
                );

                skills.insert(
                    name.to_string(),
                    json!({
                        "level": level,
                        "progress": progress,
                        "xp": xp
                    }),
                );
            }
        }
    }

    Ok(json!({
        "profile_id": profile_id,
        "uuid": uuid,
        "skills": skills
    }))
}
