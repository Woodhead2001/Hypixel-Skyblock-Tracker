// Constants and configuration
import { invoke } from '@tauri-apps/api/core';

// Config will be loaded at runtime
let CONFIG = null;

// Initialize config on app startup
export async function initializeConfig() {
  if (!CONFIG) {
    try {
      CONFIG = await invoke('get_app_config');
    } catch (err) {
      console.error('Failed to load config:', err);
      // Fallback defaults if config load fails
      CONFIG = {
        hypixel_api_url: 'https://api.hypixel.net',
        craft_recursion_depth: 10,
        cache_duration: 3600000,
        skyblock_skills: [
          'farming',
          'mining',
          'combat',
          'foraging',
          'fishing',
          'enchanting',
          'alchemy',
          'carpentry',
          'runecrafting',
          'social',
        ]
      };
    }
  }
  return CONFIG;
}

// Getter functions for config values
export async function getConfig() {
  return initializeConfig();
}

export async function getHypixelApiUrl() {
  const config = await initializeConfig();
  return config.hypixel_api_url;
}

export async function getCraftRecursionDepth() {
  const config = await initializeConfig();
  return config.craft_recursion_depth;
}

export async function getCacheDuration() {
  const config = await initializeConfig();
  return config.cache_duration;
}

export async function getSkyblockSkills() {
  const config = await initializeConfig();
  return config.skyblock_skills;
}

// Legacy exports for backwards compatibility (cached values)
export const HYPIXEL_API_URL = 'https://api.hypixel.net';
export const CRAFT_RECURSION_DEPTH = 10;
export const CACHE_DURATION = 3600000; // 1 hour
export const SKYBLOCK_SKILLS = [
  'farming',
  'mining',
  'combat',
  'foraging',
  'fishing',
  'enchanting',
  'alchemy',
  'carpentry',
  'runecrafting',
  'social',
];
