use serde_json::{json, Value};

use crate::api::hypixel_api::{
    get_player_uuid,
    fetch_and_cache_profiles,
    get_cached_profiles,
};

#[tauri::command]
pub async fn fetch_hypixel_player(username: String) -> Result<Value, String> {
    // 1. Resolve UUID from Mojang
    let uuid = get_player_uuid(&username).await?;

    // 2. Fetch profiles from Hypixel and cache them
    fetch_and_cache_profiles(&uuid).await?;

    // 3. Read back from cache (so other commands can reuse it)
    let cached = get_cached_profiles()?;

    // 4. Return a simple payload to the frontend
    Ok(json!({
        "username": username,
        "uuid": uuid,
        "profiles": cached["profiles"],
    }))
}
