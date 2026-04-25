use crate::api::hypixel_api;

#[tauri::command]
pub async fn fetch_hypixel_player(username: String) -> Result<serde_json::Value, String> {
    Ok(serde_json::Value::Null)
}
