use std::fs::{self, File};
use std::path::PathBuf;
use zip::ZipArchive;
use anyhow::{Result, anyhow};
use log::{info, error};

/// Extracts `pack.cats` from the user's resource pack ZIP.
/// Returns the path to the extracted file.
pub fn extract_pack_cats() -> Result<PathBuf> {
    // Location of the resource pack ZIP
    let appdata = dirs::data_dir()
        .ok_or_else(|| anyhow!("Failed to locate AppData directory"))?;

    let zip_path = appdata
        .join("com.hypixel.tracker")
        .join("skyblock-pack.zip");

    if !zip_path.exists() {
        return Err(anyhow!("Resource pack ZIP not found at {:?}", zip_path));
    }

    info!("Opening resource pack ZIP at {:?}", zip_path);

    let file = File::open(&zip_path)?;
    let mut zip = ZipArchive::new(file)?;

    // Extract pack.cats into a temp folder
    let out_dir = std::env::current_dir()?.join("extracted_pack");
    fs::create_dir_all(&out_dir)?;

    let mut cats_file = zip
        .by_name("pack.cats")
        .map_err(|_| anyhow!("pack.cats not found inside ZIP"))?;

    let out_path = out_dir.join("pack.cats");
    let mut out = File::create(&out_path)?;
    std::io::copy(&mut cats_file, &mut out)?;

    info!("Extracted pack.cats to {:?}", out_path);

    Ok(out_path)
}
