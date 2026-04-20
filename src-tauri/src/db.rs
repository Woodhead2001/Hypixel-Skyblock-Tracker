use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct DbState {
  pub db: Mutex<Connection>,
}

impl DbState {
  pub fn new(db_path: PathBuf) -> Self {
    let conn = Connection::open(&db_path).expect("Failed to open database");
    DbState {
      db: Mutex::new(conn),
    }
  }
}

pub fn init_db(db_path: &PathBuf) -> Result<()> {
  let conn = Connection::open(db_path)?;

  // Create tables
  conn.execute_batch(
    "
    CREATE TABLE IF NOT EXISTS recipes (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL UNIQUE,
      description TEXT,
      output_item TEXT NOT NULL,
      output_quantity INTEGER DEFAULT 1,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS recipe_ingredients (
      id INTEGER PRIMARY KEY,
      recipe_id INTEGER NOT NULL,
      ingredient_name TEXT NOT NULL,
      quantity_needed INTEGER NOT NULL,
      FOREIGN KEY(recipe_id) REFERENCES recipes(id)
    );

    CREATE TABLE IF NOT EXISTS player_data (
      id INTEGER PRIMARY KEY,
      username TEXT NOT NULL UNIQUE,
      uuid TEXT,
      last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      data TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS goals (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      item_name TEXT,
      quantity_target INTEGER,
      is_completed BOOLEAN DEFAULT 0,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      completed_at TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS inventory (
      id INTEGER PRIMARY KEY,
      username TEXT NOT NULL,
      item_name TEXT NOT NULL,
      quantity INTEGER NOT NULL,
      last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY(username) REFERENCES player_data(username)
    );
    "
  )?;

  Ok(())
}
