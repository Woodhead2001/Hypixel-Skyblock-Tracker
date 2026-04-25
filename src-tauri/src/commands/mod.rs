// src-tauri/src/commands/mod.rs

pub mod hypixel;
pub mod skills;
pub mod profiles;

pub use hypixel::fetch_hypixel_player;
pub use skills::get_player_skills;
pub use profiles::get_player_profiles;
