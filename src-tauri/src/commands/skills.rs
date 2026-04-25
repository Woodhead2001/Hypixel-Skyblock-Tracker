// src-tauri/src/commands/skills.rs

use serde_json::Value;
use std::ptr::null;

#[tauri::command]
pub async fn get_player_skills(username: String) -> Result<Value, String> {
    return Ok(Value::Null);
}
