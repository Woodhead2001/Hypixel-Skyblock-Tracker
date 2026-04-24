// src-tauri/src/commands/hypixel.rs

use crate::api::hypixel;

#[tauri::command]
pub async fn fetch_hypixel_player(username: String) -> Result<serde_json::Value, String> {
    hypixel::get_full_player_data(&username).await
}
