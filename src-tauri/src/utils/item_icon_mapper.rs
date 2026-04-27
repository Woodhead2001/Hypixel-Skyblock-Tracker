use std::collections::HashMap;
use std::path::PathBuf;

use log::{info, debug, error};

/// Represents a SkyBlock item from your backend.
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

/// Base directory where icons live (after CATS extraction).
fn base_icons_dir() -> PathBuf {
    // Current working dir / icons / skyblock
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("icons")
        .join("skyblock")
}

/// Main mapping function:
/// Given a SkyblockItem, return the icon path (if any).
pub fn map_item_icon(item: &SkyblockItem) -> Option<PathBuf> {
    let id = normalize_id(&item.id);
    let material = normalize_material(&item.material);

    let cache_key = format!("{}:{}", id, material);

    // 1. Check cache
    if let Some(path) = cache().get(&cache_key) {
        //debug!("Icon cache hit for {}", cache_key);
        return Some(path.clone());
    }

    //info!("Mapping icon for item_id={} material={}", id, material);

    // Very simple strategy for now:
    //   icons/skyblock/<id>.png  (id already uppercased; we can lower it for filenames)
    let candidate = base_icons_dir().join(format!("{}.png", id.to_lowercase()));

    if candidate.exists() {
        //debug!("Resolved icon path: {:?}", candidate);
        cache().insert(cache_key, candidate.clone());
        Some(candidate)
    } else {
        //error!("Icon file not found for item_id={} at {:?}", id, candidate);
        None
    }
}

/// Optional: export icon into a final directory for frontend consumption.
/// For now, we just return the resolved path (no extra copying).
pub fn export_item_icon(item: &SkyblockItem) -> Option<PathBuf> {
    let id = normalize_id(&item.id);
    let material = normalize_material(&item.material);

    info!("Exporting icon for item_id={} material={}", id, material);

    let path = map_item_icon(&SkyblockItem { id, material })?;

    // If you later want to copy into a web‑served dir, do it here.
    Some(path)
}
