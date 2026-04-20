use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
  pub id: Option<i32>,
  pub name: String,
  pub description: Option<String>,
  pub item_name: Option<String>,
  pub quantity_target: Option<i32>,
  pub is_completed: bool,
}
