import { invoke } from '@tauri-apps/api/core';

export const api = {
  async addRecipe(recipe) {
    return invoke('add_recipe', { recipe });
  },

  async getRecipes() {
    return invoke('get_recipes');
  },

  async addGoal(goal) {
    return invoke('add_goal', { goal });
  },

  async getGoals() {
    return invoke('get_goals');
  },

  async getPlayerData(username) {
    return invoke('get_player_data', { username });
  },
};
