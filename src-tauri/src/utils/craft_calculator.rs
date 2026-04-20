// Craft calculator with recursive logic
// Determines if a recipe can be crafted given inventory

pub fn can_craft_item(item_name: &str, inventory: &Vec<(String, i32)>, recipes: &Vec<Recipe>) -> bool {
  // TODO: Check if item exists in inventory with enough quantity
  // TODO: If not, recursively check if we can craft it
  // TODO: Handle circular dependencies
  false
}

pub fn get_craft_path(item_name: &str, inventory: &Vec<(String, i32)>, recipes: &Vec<Recipe>) -> Vec<String> {
  // TODO: Return the sequence of crafts needed to make the item
  vec![]
}

pub struct Recipe {
  pub name: String,
  pub ingredients: Vec<(String, i32)>, // (item, quantity)
  pub output: (String, i32), // (item, quantity)
}
