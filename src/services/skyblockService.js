// src/services/skyblockService.js
import { invoke } from "@tauri-apps/api/core";

export async function getPlayerSkills(username) {
  return invoke("get_player_skills", { username });
}
