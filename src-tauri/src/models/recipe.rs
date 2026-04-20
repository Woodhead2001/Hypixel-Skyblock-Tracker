use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
  pub id: Option<i32>,
  pub name: String,
  pub description: Option<String>,
  pub output_item: String,
  pub output_quantity: i32,
}
