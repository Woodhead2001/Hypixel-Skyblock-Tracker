# Hypixel Skyblock Tracker - File Structure

## Overview
```
hypixel-skyblock-tracker/
├── src/                          # React frontend
│   ├── components/
│   │   ├── Dashboard.jsx
│   │   ├── RecipeTracker.jsx
│   │   ├── GoalTracker.jsx
│   │   ├── InventoryView.jsx
│   │   ├── CraftPlanner.jsx
│   │   └── PlayerSearch.jsx
│   ├── pages/
│   │   ├── HomePage.jsx
│   │   ├── SettingsPage.jsx
│   │   └── CraftPlannerPage.jsx
│   ├── hooks/
│   │   ├── useRecipes.js
│   │   ├── useGoals.js
│   │   └── usePlayer.js
│   ├── styles/
│   │   ├── App.css
│   │   ├── components.css
│   │   └── pages.css
│   ├── api.js                    # Tauri command wrappers
│   ├── App.js
│   ├── index.js
│   └── constants.js              # API keys, URLs, etc.
│
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── db.rs                # Database initialization & state
│   │   ├── commands.rs          # Tauri command handlers
│   │   ├── api/
│   │   │   └── hypixel.rs       # Hypixel API client
│   │   ├── models/
│   │   │   ├── recipe.rs
│   │   │   ├── player.rs
│   │   │   ├── goal.rs
│   │   │   └── inventory.rs
│   │   └── utils/
│   │       ├── craft_calculator.rs  # Recursive craft logic
│   │       └── validators.rs
│   ├── tauri.conf.json
│   └── Cargo.toml
│
├── public/
│   ├── index.html
│   ├── main.js                  # (Old Electron file - can remove)
│   └── preload.js               # (Old Electron file - can remove)
│
├── package.json
├── .gitignore
├── README.md
├── ARCHITECTURE.md
└── docs/
    ├── API_INTEGRATION.md       # Hypixel API setup
    ├── DATABASE_SCHEMA.md       # DB tables & relationships
    └── CRAFT_ALGORITHM.md       # Recursive craft logic
```

## Module Descriptions

### Frontend (`src/`)
- **components/** - Reusable UI components
  - `Dashboard.jsx` - Main overview screen
  - `RecipeTracker.jsx` - View/manage recipes
  - `GoalTracker.jsx` - Manage goals
  - `InventoryView.jsx` - Display inventory
  - `CraftPlanner.jsx` - Craft calculator interface
  - `PlayerSearch.jsx` - Search for player by username

- **pages/** - Full page views (routes)
- **hooks/** - Custom React hooks for state & data fetching
- **styles/** - CSS organized by scope
- **api.js** - Wrapper functions calling Rust commands
- **constants.js** - Config values, API keys, etc.

### Backend (`src-tauri/src/`)
- **main.rs** - Tauri initialization, DB setup
- **db.rs** - SQLite schema & connection pool
- **commands.rs** - All Tauri command handlers (exported to frontend)
- **api/hypixel.rs** - HTTP client for Hypixel API
- **models/** - Data structures (Recipe, Player, Goal, etc.)
- **utils/craft_calculator.rs** - Recursive craft logic

## Next: Create Structure?
Should I create these directories and placeholder files?
