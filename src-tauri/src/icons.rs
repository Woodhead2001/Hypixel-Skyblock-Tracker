use log::{info, error};
use serde::Serialize;

use crate::utils::item_icon_mapper::{map_item_icon, export_item_icon as mapper_export_icon, SkyblockItem};

/// Response sent back to the frontend.
#[derive(Serialize)]
pub struct IconResponse {
    pub path: Option<String>,
    pub error: Option<String>,
}

/// Simple helper used by collections/minions etc.
/// Returns a relative path like `icons/skyblock/<id>.png`.
pub fn get_icon(id: &str) -> Option<String> {
    // You can make this smarter later; for now we assume PNG in icons/skyblock
    Some(format!("icons/skyblock/{}.png", id.to_lowercase()))
}

/// Tauri command: resolve icon path for an item.
/// Does NOT copy the icon, just returns the resolved path.
#[tauri::command]
pub fn get_item_icon(id: String, material: String) -> IconResponse {
    //info!("Tauri: get_item_icon id={} material={}", id, material);

    let item = SkyblockItem { id, material };

    match map_item_icon(&item) {
        Some(path) => IconResponse {
            path: Some(path.to_string_lossy().to_string()),
            error: None,
        },
        None => {
            //error!("Tauri: failed to resolve icon");
            IconResponse {
                path: None,
                error: Some("Icon not found".into()),
            }
        }
    }
}

/// Tauri command: export icon into final directory and return the exported path.
#[tauri::command]
pub fn export_item_icon(id: String, material: String) -> IconResponse {
    //info!("Tauri: export_item_icon id={} material={}", id, material);

    let item = SkyblockItem { id, material };

    match mapper_export_icon(&item) {
        Some(path) => IconResponse {
            path: Some(path.to_string_lossy().to_string()),
            error: None,
        },
        None => {
            //error!("Tauri: failed to export icon");
            IconResponse {
                path: None,
                error: Some("Export failed".into()),
            }
        }
    }
}
