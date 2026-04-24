// src-tauri/src/commands/player.rs

use crate::db::DbState;

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
pub fn save_player_data(
    state: tauri::State<DbState>,
    username: String,
    data: String,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();

    conn.execute(
        "INSERT OR REPLACE INTO player_data (username, data) VALUES (?1, ?2)",
        (&username, &data),
    )
    .map_err(|e| format!("Failed to save player data: {}", e))?;

    Ok(())
}
