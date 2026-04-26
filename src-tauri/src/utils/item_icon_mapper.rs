use std::collections::HashMap;
use std::path::PathBuf;

use log::{info, debug, error};

use crate::icons::icons::{resolve_icon, export_icon};

/// Represents a SkyBlock item from your backend.
/// Adjust this struct to match your actual item model.
#[derive(Debug, Clone)]
pub struct SkyblockItem {
    pub id: String,       // e.g. "DIVAN_ALLOY"
    pub material: String, // e.g. "SKULL_ITEM" or "DIAMOND"
}

/// Cache so we don't resolve icons repeatedly.
static mut ICON_CACHE: Option<HashMap<String, PathBuf>> = None;

/// Initialize the cache if needed.
fn cache() -> &'static mut HashMap<String, PathBuf> {
    unsafe {
        if ICON_CACHE.is_none() {
            ICON_CACHE = Some(HashMap::new());
        }
        ICON_CACHE.as_mut().unwrap()
    }
}

/// Normalize item ID (Hypixel sometimes sends weird casing).
fn normalize_id(id: &str) -> String {
    id.trim().to_uppercase()
}

/// Normalize material (vanilla fallback).
fn normalize_material(mat: &str) -> String {
    mat.trim().to_uppercase()
}

/// Main mapping function:
/// Given a SkyblockItem, return the icon path (if any).
pub fn map_item_icon(item: &SkyblockItem) -> Option<PathBuf> {
    let id = normalize_id(&item.id);
    let material = normalize_material(&item.material);

    let cache_key = format!("{}:{}", id, material);

    // 1. Check cache
    if let Some(path) = cache().get(&cache_key) {
        debug!("Cache hit for {}", cache_key);
        return Some(path.clone());
    }

    info!("Mapping icon for item_id={} material={}", id, material);

    // 2. Resolve icon
    let resolved = resolve_icon(&id, &material);

    if let Some(path) = resolved.clone() {
        cache().insert(cache_key, path.clone());
        return Some(path);
    }

    error!("Failed to map icon for item_id={} material={}", id, material);
    None
}

/// Optional: export icon into a final directory for frontend consumption.
pub fn export_item_icon(item: &SkyblockItem) -> Option<PathBuf> {
    let id = normalize_id(&item.id);
    let material = normalize_material(&item.material);

    info!("Exporting icon for item_id={} material={}", id, material);

    let exported = export_icon(&id, &material);

    if exported.is_none() {
        error!("Failed to export icon for {}", id);
    }

    exported
}
