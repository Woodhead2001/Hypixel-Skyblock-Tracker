// src-tauri/src/commands/profiles.rs

use serde_json::Value;
use crate::api::hypixel_api::{get_player_uuid, fetch_raw_profiles};

#[tauri::command]
pub async fn get_player_profiles(username: String) -> Result<Value, String> {
    // Step 1: Resolve UUID
    let uuid = get_player_uuid(&username).await?;

    // Step 2: Fetch raw Hypixel profiles JSON
    let profiles = fetch_raw_profiles(&uuid).await?;

    // Step 3: Return raw JSON directly to the UI
    Ok(profiles)
}
