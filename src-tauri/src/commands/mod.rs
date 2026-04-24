// src-tauri/src/commands/mod.rs

pub mod player;
pub mod recipes;
pub mod goals;
pub mod hypixel;
pub mod skills;

pub use player::{get_player_data, save_player_data};
pub use recipes::{add_recipe, get_recipes};
pub use goals::{add_goal, get_goals};
pub use hypixel::fetch_hypixel_player;
pub use skills::get_player_skills;
