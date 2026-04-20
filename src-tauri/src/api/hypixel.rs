// Hypixel API client
// https://api.hypixel.net/

pub async fn get_player_by_username(username: &str, api_key: &str) -> Result<String, String> {
  // TODO: Fetch player UUID from Hypixel API
  // TODO: Fetch player skyblock data using UUID
  Err("Not implemented".to_string())
}

pub async fn get_player_by_uuid(uuid: &str, api_key: &str) -> Result<String, String> {
  // TODO: Fetch player skyblock data from Hypixel API
  Err("Not implemented".to_string())
}

pub async fn get_player_skyblock_profiles(uuid: &str, api_key: &str) -> Result<String, String> {
  // TODO: Get all skyblock profiles for player
  Err("Not implemented".to_string())
}
