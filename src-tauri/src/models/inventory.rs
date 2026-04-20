use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inventory {
  pub username: String,
  pub items: Vec<InventoryItem>,
  pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
  pub name: String,
  pub quantity: i32,
  pub rarity: Option<String>,
}
