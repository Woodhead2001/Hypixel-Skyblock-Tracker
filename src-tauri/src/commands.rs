use serde::{Deserialize, Serialize};
use crate::db::DbState;
use crate::api::hypixel;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
  pub id: Option<i32>,
  pub name: String,
  pub description: Option<String>,
  pub output_item: String,
  pub output_quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Goal {
  pub id: Option<i32>,
  pub name: String,
  pub description: Option<String>,
  pub item_name: Option<String>,
  pub quantity_target: Option<i32>,
  pub is_completed: bool,
}

#[tauri::command]
pub fn get_player_data(state: tauri::State<DbState>, username: String) -> Result<String, String> {
  let conn = state.db.lock().unwrap();
  
  let result: String = conn
    .query_row(
      "SELECT data FROM player_data WHERE username = ?1",
      [&username],
      |row| row.get(0),
    )
    .map_err(|e| format!("Failed to get player data: {}", e))?;
  
  Ok(result)
}

#[tauri::command]
pub fn add_recipe(state: tauri::State<DbState>, recipe: Recipe) -> Result<(), String> {
  let conn = state.db.lock().unwrap();
  
  conn
    .execute(
      "INSERT INTO recipes (name, description, output_item, output_quantity) VALUES (?1, ?2, ?3, ?4)",
      (&recipe.name, &recipe.description, &recipe.output_item, &recipe.output_quantity),
    )
    .map_err(|e| format!("Failed to add recipe: {}", e))?;
  
  Ok(())
}

#[tauri::command]
pub fn get_recipes(state: tauri::State<DbState>) -> Result<Vec<Recipe>, String> {
  let conn = state.db.lock().unwrap();
  
  let mut stmt = conn
    .prepare("SELECT id, name, description, output_item, output_quantity FROM recipes")
    .map_err(|e| format!("Failed to prepare statement: {}", e))?;
  
  let recipes = stmt
    .query_map([], |row| {
      Ok(Recipe {
        id: Some(row.get(0)?),
        name: row.get(1)?,
        description: row.get(2)?,
        output_item: row.get(3)?,
        output_quantity: row.get(4)?,
      })
    })
    .map_err(|e| format!("Failed to query recipes: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect recipes: {}", e))?;
  
  Ok(recipes)
}

#[tauri::command]
pub fn add_goal(state: tauri::State<DbState>, goal: Goal) -> Result<(), String> {
  let conn = state.db.lock().unwrap();
  
  conn
    .execute(
      "INSERT INTO goals (name, description, item_name, quantity_target, is_completed) VALUES (?1, ?2, ?3, ?4, ?5)",
      (
        &goal.name,
        &goal.description,
        &goal.item_name,
        &goal.quantity_target,
        &goal.is_completed,
      ),
    )
    .map_err(|e| format!("Failed to add goal: {}", e))?;
  
  Ok(())
}

#[tauri::command]
pub fn get_goals(state: tauri::State<DbState>) -> Result<Vec<Goal>, String> {
  let conn = state.db.lock().unwrap();
  
  let mut stmt = conn
    .prepare("SELECT id, name, description, item_name, quantity_target, is_completed FROM goals")
    .map_err(|e| format!("Failed to prepare statement: {}", e))?;
  
  let goals = stmt
    .query_map([], |row| {
      Ok(Goal {
        id: Some(row.get(0)?),
        name: row.get(1)?,
        description: row.get(2)?,
        item_name: row.get(3)?,
        quantity_target: row.get(4)?,
        is_completed: row.get(5)?,
      })
    })
    .map_err(|e| format!("Failed to query goals: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect goals: {}", e))?;
  
  Ok(goals)
}

#[tauri::command]
pub async fn fetch_hypixel_player(username: String) -> Result<serde_json::Value, String> {
  hypixel::get_full_player_data(&username).await
}

#[tauri::command]
pub fn save_player_data(state: tauri::State<DbState>, username: String, data: String) -> Result<(), String> {
  let conn = state.db.lock().unwrap();
  
  conn
    .execute(
      "INSERT OR REPLACE INTO player_data (username, data) VALUES (?1, ?2)",
      (&username, &data),
    )
    .map_err(|e| format!("Failed to save player data: {}", e))?;
  
  Ok(())
}
