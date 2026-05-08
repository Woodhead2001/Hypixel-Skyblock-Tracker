use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use anyhow::{anyhow, Result};
use log::{info, error};
use zip::ZipArchive;

pub fn extract_pngs_from_pack() -> Result<()> {
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

    let icons_dir = Path::new("..")
        .join("public")
        .join("icons")
        .join("skyblock");

    let pack_root = Path::new("..")
        .join("public")
        .join("skyblock-pack");

    fs::create_dir_all(&icons_dir)?;
    fs::create_dir_all(&pack_root)?;

    let mut png_count = 0;
    let mut json_count = 0;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let name = entry.name().to_string();

        if name.to_lowercase().ends_with(".png") {
            if name.contains("cittofirmgenerated/textures/item/") {
                let filename = match Path::new(&name).file_name() {
                    Some(f) => f,
                    None => continue,
                };

                let out_path = icons_dir.join(filename);

                let mut buf = Vec::new();
                entry.read_to_end(&mut buf)?;

                if fs::write(&out_path, &buf).is_ok() {
                    png_count += 1;
                }
            }

            continue;
        }

        if name.to_lowercase().ends_with(".json") && name.contains("firmskyblock/models/item/") {
            let out_path = pack_root.join(&name);

            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;

            if fs::write(&out_path, &buf).is_ok() {
                json_count += 1;
            }

            continue;
        }
    }

    info!("Extracted {} PNG item textures.", png_count);
    info!("Extracted {} model JSONs.", json_count);

    if png_count == 0 {
        return Err(anyhow!("No PNGs were extracted — pack may be incomplete."));
    }

    if json_count == 0 {
        return Err(anyhow!("No model JSONs were extracted — cannot map SkyBlock items."));
    }

    Ok(())
}
