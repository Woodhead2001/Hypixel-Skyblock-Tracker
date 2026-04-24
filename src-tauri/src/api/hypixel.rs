// Hypixel API client
// https://api.hypixel.net/

use reqwest::Client;
use serde_json::{json, Value};

const HYPIXEL_API_URL: &str = "https://api.hypixel.net";
const HYPIXEL_API_KEY: &str = "dd532b8d-84c1-4f4e-b2cc-6cdc4751987a";

pub async fn get_player_uuid(username: &str) -> Result<String, String> {
  let client = Client::new();
  let url = format!("{}/player?name={}&key={}", HYPIXEL_API_URL, username, HYPIXEL_API_KEY);
  
  match client.get(&url).send().await {
    Ok(response) => {
      match response.json::<Value>().await {
        Ok(data) => {
          if let Some(success) = data.get("success").and_then(|s| s.as_bool()) {
            if !success {
              return Err("Player not found".to_string());
            }
          }
          
          if let Some(uuid) = data.get("player").and_then(|p| p.get("uuid")).and_then(|u| u.as_str()) {
            Ok(uuid.to_string())
          } else {
            Err("No UUID in response".to_string())
          }
        }
        Err(e) => Err(format!("Failed to parse response: {}", e)),
      }
    }
    Err(e) => Err(format!("Request failed: {}", e)),
  }
}

pub async fn get_player_skyblock_data(uuid: &str) -> Result<Value, String> {
  let client = Client::new();
  let url = format!("{}/skyblock/profiles?uuid={}&key={}", HYPIXEL_API_URL, uuid, HYPIXEL_API_KEY);
  
  match client.get(&url).send().await {
    Ok(response) => {
      match response.json::<Value>().await {
        Ok(data) => {
          if let Some(success) = data.get("success").and_then(|s| s.as_bool()) {
            if !success {
              return Err("Failed to fetch skyblock data".to_string());
            }
          }
          
          if let Some(profiles) = data.get("profiles") {
            Ok(profiles.clone())
          } else {
            Ok(Value::Array(vec![]))
          }
        }
        Err(e) => Err(format!("Failed to parse response: {}", e)),
      }
    }
    Err(e) => Err(format!("Request failed: {}", e)),
  }
}

pub async fn extract_player_info(username: &str, uuid: &str, profiles: &Value) -> Result<Value, String> {
  let mut skills = serde_json::Map::new();
  
  // Extract skills from profiles
  if let Some(profile_list) = profiles.as_array() {
    if let Some(first_profile) = profile_list.first() {
      if let Some(members) = first_profile.get("members").and_then(|m| m.as_object()) {
        if let Some(member) = members.get(uuid) {
          if let Some(experience) = member.get("experience").and_then(|e| e.as_object()) {
            // Calculate skill levels from experience
            for (skill_key, exp_value) in experience {
              if let Some(exp) = exp_value.as_f64() {
                let level = calculate_level(exp);
                skills.insert(skill_key.clone(), json!(level));
              }
            }
          }
        }
      }
    }
  }
  
  Ok(json!({
    "username": username,
    "uuid": uuid,
    "skills": skills
  }))
}

fn calculate_level(experience: f64) -> i32 {
  // Skyblock skill levels
  let thresholds = [
    0.0, 50.0, 150.0, 275.0, 435.0, 635.0, 885.0, 1185.0, 1535.0, 1935.0,
    2385.0, 2885.0, 3435.0, 4035.0, 4685.0, 5385.0, 6135.0, 6935.0, 7785.0, 8685.0,
    9635.0, 10635.0, 11735.0, 12835.0, 14035.0
  ];
  
  for (level, &threshold) in thresholds.iter().enumerate() {
    if experience < threshold * 10_000.0 {
      return level as i32;
    }
  }
  
  60 // Max level
}

pub async fn get_full_player_data(username: &str) -> Result<Value, String> {
  // First get UUID
  let uuid = get_player_uuid(username).await?;
  
  // Then get skyblock data
  let skyblock_data = get_player_skyblock_data(&uuid).await?;
  
  // Extract and format player info
  extract_player_info(username, &uuid, &skyblock_data).await
}
