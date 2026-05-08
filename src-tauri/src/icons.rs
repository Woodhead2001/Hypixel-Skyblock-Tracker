use serde::Serialize;

use crate::utils::item_icon_mapper::{
    map_item_icon,
    export_item_icon as mapper_export_icon,
    SkyblockItem,
};

#[derive(Serialize)]
pub struct IconResponse {
    pub path: Option<String>,
    pub error: Option<String>,
}

pub fn get_icon(id: &str) -> String {
    format!("icons/skyblock/{}.png", id.to_lowercase())
}

#[tauri::command]
pub fn get_item_icon(id: String, material: String) -> IconResponse {
    let item = SkyblockItem { id, material };

    match map_item_icon(&item) {
        Some(fs_path) => {
            let file_name = fs_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let is_skyblock = fs_path.to_string_lossy().contains("skyblock");

            let web_path = if is_skyblock {
                format!("icons/skyblock/{}", file_name)
            } else {
                format!("icons/vanilla/{}", file_name)
            };

            IconResponse {
                path: Some(web_path),
                error: None,
            }
        }
        None => IconResponse {
            path: Some("icons/missing.png".into()),
            error: None,
        },
    }
}

#[tauri::command]
pub fn export_item_icon(id: String, material: String) -> IconResponse {
    let item = SkyblockItem { id, material };

    match mapper_export_icon(&item) {
        Some(fs_path) => {
            let file_name = fs_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let is_skyblock = fs_path.to_string_lossy().contains("skyblock");

            let web_path = if is_skyblock {
                format!("icons/skyblock/{}", file_name)
            } else {
                format!("icons/vanilla/{}", file_name)
            };

            IconResponse {
                path: Some(web_path),
                error: None,
            }
        }
        None => IconResponse {
            path: Some("icons/missing.png".into()),
            error: Some("Export failed".into()),
        },
    }
}
