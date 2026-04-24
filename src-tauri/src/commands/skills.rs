// src-tauri/src/commands/skills.rs

use serde_json::json;

#[tauri::command]
pub async fn get_player_skills(username: String) -> Result<serde_json::Value, String> {
    // 1. Get UUID from Mojang
    let uuid_url = format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        username
    );
    let uuid_res = reqwest::get(&uuid_url).await.map_err(|e| e.to_string())?;
    if !uuid_res.status().is_success() {
        return Err("Failed to fetch UUID".into());
    }
    let uuid_json: serde_json::Value = uuid_res.json().await.map_err(|e| e.to_string())?;
    let uuid = uuid_json["id"]
        .as_str()
        .ok_or("Invalid UUID response")?
        .to_string();

    // 2. Get Hypixel API key
    let api_key = std::env::var("HYPIXEL_API_KEY")
        .map_err(|_| "Missing HYPIXEL_API_KEY env var".to_string())?;

    // 3. Fetch SkyBlock profiles
    let profile_url = format!(
        "https://api.hypixel.net/skyblock/profiles?key={}&uuid={}",
        api_key, uuid
    );
    let profile_res = reqwest::get(&profile_url).await.map_err(|e| e.to_string())?;
    if !profile_res.status().is_success() {
        return Err("Failed to fetch SkyBlock profiles".into());
    }
    let profile_json: serde_json::Value = profile_res.json().await.map_err(|e| e.to_string())?;

    let profiles = profile_json["profiles"]
        .as_array()
        .ok_or("No profiles found")?;
    if profiles.is_empty() {
        return Err("Player has no SkyBlock profiles".into());
    }

    let profile = &profiles[0];
    let members = profile["members"]
        .as_object()
        .ok_or("Invalid members structure")?;
    let member = members
        .get(&uuid)
        .ok_or("Player not found in profile members")?;

    // 4. Extract skills
    let skills = vec![
        ("Farming", "experience_skill_farming"),
        ("Mining", "experience_skill_mining"),
        ("Combat", "experience_skill_combat"),
        ("Foraging", "experience_skill_foraging"),
        ("Fishing", "experience_skill_fishing"),
        ("Enchanting", "experience_skill_enchanting"),
        ("Alchemy", "experience_skill_alchemy"),
        ("Taming", "experience_skill_taming"),
    ];

    let mut skill_list = vec![];

    for (name, key) in skills {
        let xp = member[key].as_f64().unwrap_or(0.0);
        let level = xp_to_level(xp);
        skill_list.push(json!({
            "name": name,
            "xp": xp,
            "level": level,
            "maxLevel": 60
        }));
    }

    Ok(json!({ "skills": skill_list }))
}

fn xp_to_level(xp: f64) -> u32 {
    (xp / 1000.0).floor() as u32
}
