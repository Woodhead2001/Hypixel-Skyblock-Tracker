import { invoke } from "@tauri-apps/api/core";

export const api = {

  async getPlayerData(username) {
    return invoke('get_player_data', { username });
  },

  async fetchHypixelPlayer(username) {
    return invoke('fetch_hypixel_player', { username });
  },

  async savePlayerData(username, data) {
    return invoke('save_player_data', { username, data });
  },

  async readFile(filepath) {
    return invoke('read_file', { filepath: 'Cargo.toml' });
  },
};

