use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
  pub uuid: String,
  pub username: String,
  pub skyblock_level: Option<f32>,
  pub skills: Option<std::collections::HashMap<String, i32>>,
  pub last_updated: String,
}
