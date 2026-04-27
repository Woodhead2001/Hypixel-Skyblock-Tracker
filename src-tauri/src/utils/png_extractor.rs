use std::{
    fs::{self, File},
    io::{Read},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use log::{info, error};
use zip::ZipArchive;

/// Extract all PNGs from the SkyBlock resource pack ZIP in AppData.
pub fn extract_pngs_from_pack() -> Result<()> {
    // Path to the ZIP in AppData
    let appdata = std::env::var("APPDATA")
        .map_err(|e| anyhow!("Failed to read APPDATA: {}", e))?;

    let zip_path = Path::new(&appdata)
        .join("com.hypixel.tracker")
        .join("skyblock-pack.zip");

    if !zip_path.exists() {
        return Err(anyhow!("Resource pack ZIP not found at {:?}", zip_path));
    }

    info!("Opening resource pack ZIP at {:?}", zip_path);

    let file = File::open(&zip_path)?;
    let mut zip = ZipArchive::new(file)?;

    // Output directory
    let out_dir = Path::new("icons").join("skyblock");
    fs::create_dir_all(&out_dir)?;

    let mut extracted_count = 0;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let name = entry.name().to_string();

        // Only extract PNGs
        if !name.to_lowercase().ends_with(".png") {
            continue;
        }

        // Normalize filename (remove directories)
        let filename = Path::new(&name)
            .file_name()
            .ok_or_else(|| anyhow!("Invalid PNG path: {}", name))?;

        let out_path = out_dir.join(filename);

        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;

        if let Err(e) = fs::write(&out_path, &buf) {
            error!("Failed to write {:?}: {}", out_path, e);
        } else {
            extracted_count += 1;
        }
    }

    info!("Extracted {} PNG icons.", extracted_count);

    if extracted_count == 0 {
        return Err(anyhow!("No PNGs were extracted — pack may be incomplete."));
    }

    Ok(())
}
