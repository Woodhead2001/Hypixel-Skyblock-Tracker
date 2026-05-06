use std::fs;
use crate::config::loader::{AppConfig, load_config};

#[tauri::command]
pub async fn fetch_username() -> Result<String, String> {
    let config = load_config()?;
    Ok(config.default_username)
}

