# Hypixel API Integration

## Setup

1. Get API key from: https://developer.hypixel.net/
2. Add to `src/constants.js`:
   ```javascript
   export const HYPIXEL_API_KEY = 'your_api_key_here';
   export const HYPIXEL_API_URL = 'https://api.hypixel.net';
   ```

## Endpoints Used

- `/player` - Get player by UUID
- `/skyblock/profiles` - Get player's skyblock profiles
- `/skyblock/profile` - Get specific profile data

## Implementation

See `src-tauri/src/api/hypixel.rs` for client implementation.
