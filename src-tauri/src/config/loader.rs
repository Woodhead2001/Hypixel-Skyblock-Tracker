use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub hypixel_api_key: String,
    pub hypixel_api_url: String,
    pub minecraft_uuid_lookup_url: String,
    pub hypixel_skills_url: String,
    pub hypixel_profiles_url: String,
    pub hypixel_items_url: String,
    pub default_username: String,
    pub craft_recursion_depth: u32,
    pub cache_duration: u64,
    pub skyblock_skills: Vec<String>,
}

pub fn load_config() -> Result<AppConfig, String> {
    let contents = include_str!("../../config.json");
    serde_json::from_str::<AppConfig>(contents).map_err(|e| e.to_string())
}
