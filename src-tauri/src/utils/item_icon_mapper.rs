use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SkyblockItem {
    pub id: String,
    pub material: String,
}

static mut ICON_CACHE: Option<HashMap<String, PathBuf>> = None;

fn cache() -> &'static mut HashMap<String, PathBuf> {
    unsafe {
        if ICON_CACHE.is_none() {
            ICON_CACHE = Some(HashMap::new());
        }
        ICON_CACHE.as_mut().unwrap()
    }
}

fn normalize_id(id: &str) -> String {
    id.trim().to_uppercase()
}

fn normalize_material(mat: &str) -> String {
    mat.trim().to_uppercase()
}

fn skyblock_dir() -> PathBuf {
    Path::new("..")
        .join("public")
        .join("icons")
        .join("skyblock")
}

fn vanilla_dir() -> PathBuf {
    Path::new("..")
        .join("public")
        .join("icons")
        .join("vanilla")
}

fn model_json_dir() -> PathBuf {
    Path::new("..")
        .join("public")
        .join("skyblock-pack")
        .join("assets")
        .join("firmskyblock")
        .join("models")
        .join("item")
}

#[derive(Deserialize)]
struct ModelJson {
    textures: Option<Textures>,
}

#[derive(Deserialize)]
struct Textures {
    layer0: Option<String>,
}

/// Vanilla alias table for items whose Hypixel material name
/// does NOT match the vanilla PNG filename.
fn vanilla_aliases() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // Crops / food
        ("CARROT_ITEM", "carrot.png"),
        ("CACTUS", "cactus_side.png"),
        ("MELON", "melon_slice.png"),
        ("POTATO_ITEM", "potato.png"),
        ("PUMPKIN", "pumpkin_side.png"),
        ("SEEDS", "wheat_seeds.png"),
        ("RAW_CHICKEN", "chicken.png"),
        ("PORK", "porkchop.png"),

        // Fish variants
        ("RAW_FISH:0", "cod.png"),
        ("RAW_FISH:1", "salmon.png"),
        ("RAW_FISH:2", "tropical_fish.png"),
        ("RAW_FISH:3", "pufferfish.png"),
        ("INK_SACK:3", "cocoa_beans.png"), // cocoa beans, idk why
        ("INK_SACK", "ink_sac.png"),
        ("RAW_FISH", "cod.png"),
        ("WATER_LILY", "lily_pad.png"),

        // Plants / flowers
        ("MOONFLOWER", "torchflower.png"),
        ("DOUBLE_PLANT", "sunflower_front.png"),
        ("MUSHROOM_COLLECTION", "red_mushroom.png"),
        ("WILD_ROSE", "poppy.png"),
        ("LILY_PAD", "lily_pad.png"),
        ("NETHER_STALK", "nether_wart.png"),

        // Logs / wood
        ("LOG", "oak_log.png"),
        ("LOG:1", "spruce_log.png"),
        ("LOG:2", "birch_log.png"),
        ("LOG:3", "jungle_log.png"),
        ("LOG_2", "acacia_log.png"),
        ("LOG_2:1", "dark_oak_log.png"),
        ("MANGROVE_LOG", "mangrove_log.png"),
        ("SEA_LUMIES", "sea_pickle.png"),
        ("TENDER_WOOD", "oak_log.png"),
        ("FIG_LOG", "oak_log.png"),

        // Blocks
        ("COBBLESTONE", "cobblestone.png"),
        ("ENDER_STONE", "end_stone.png"),
        ("GRAVEL", "gravel.png"),
        ("ICE", "ice.png"),
        ("INK_SACK:4", "lapis_lazuli.png"),
        ("MYCEL", "mycelium_side.png"),
        ("NETHERRACK", "netherrack.png"),
        ("OBSIDIAN", "obsidian.png"),
        ("SAND", "sand.png"),
        ("SAND:1", "red_sand.png"),
        ("SPONGE", "sponge.png"),
        ("WILTED_BERBERIS", "dead_bush.png")
    ])
}

/// Resolve texture filename via firmskyblock model JSON.
/// Example:
///   id = "SULPHUR_ORE"
///   model: assets/firmskyblock/models/item/sulphur_ore.json
///   layer0: "cittofirmgenerated:item/sulphur"
///   → "sulphur.png"
fn resolve_model_texture(id: &str) -> Option<String> {
    let model_path = model_json_dir().join(format!("{}.json", id.to_lowercase()));

    if !model_path.exists() {
        return None;
    }

    let data = fs::read_to_string(&model_path).ok()?;
    let model: ModelJson = serde_json::from_str(&data).ok()?;

    let layer0 = model.textures?.layer0?;

    let parts: Vec<&str> = layer0.split('/').collect();
    let tex_name = parts.last()?.to_string();

    Some(format!("{}.png", tex_name))
}

pub fn map_item_icon(item: &SkyblockItem) -> Option<PathBuf> {
    let id = normalize_id(&item.id);
    let material = normalize_material(&item.material);

    let cache_key = format!("{}:{}", id, material);

    if let Some(path) = cache().get(&cache_key) {
        return Some(path.clone());
    }

    // 1) Try model JSON → texture mapping
    if let Some(tex_file) = resolve_model_texture(&id) {
        let candidate = skyblock_dir().join(&tex_file);
        if candidate.exists() {
            cache().insert(cache_key.clone(), candidate.clone());
            return Some(candidate);
        }
    }

    // 2) Fallback: direct SkyBlock <id>.png
    let sb_file = skyblock_dir().join(format!("{}.png", id.to_lowercase()));
    if sb_file.exists() {
        cache().insert(cache_key.clone(), sb_file.clone());
        return Some(sb_file);
    }

    // 3) Fallback: vanilla alias table
    let aliases = vanilla_aliases();
    if let Some(&alias) = aliases.get(material.as_str()) {
        let aliased = vanilla_dir().join(alias);
        if aliased.exists() {
            cache().insert(cache_key.clone(), aliased.clone());
            return Some(aliased);
        }
    }

    // 4) Fallback: vanilla material.png
    let vanilla_file = vanilla_dir().join(format!("{}.png", material.to_lowercase()));
    if vanilla_file.exists() {
        cache().insert(cache_key.clone(), vanilla_file.clone());
        return Some(vanilla_file);
    }

    // 5) Nothing found
    None
}

pub fn export_item_icon(item: &SkyblockItem) -> Option<PathBuf> {
    map_item_icon(item)
}
