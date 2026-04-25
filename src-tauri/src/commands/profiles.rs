// src-tauri/src/commands/profiles.rs

use serde_json::{json, Value};
use crate::api::hypixel_api::get_cached_profiles;

#[tauri::command]
pub async fn get_player_profiles() -> Result<Value, String> {
    let data = get_cached_profiles()?;

    let profiles = data["profiles"]
        .as_array()
        .ok_or("Invalid profile format")?;

    let simplified: Vec<Value> = profiles
        .iter()
        .map(|p| {
            json!({
                "profile_id": p["profile_id"],
                "cute_name": p["cute_name"],
                "selected": p["selected"],
                "members": p["members"],
            })
        })
        .collect();

    Ok(json!({ "profiles": simplified }))
}
