use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use anyhow::{anyhow, Result};
use image::{ImageBuffer, RgbaImage};
use log::{debug, error, info};

#[derive(Debug)]
struct Sprite {
    name: String,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

fn read_u8(cur: &mut Cursor<Vec<u8>>) -> Result<u8> {
    let mut b = [0u8; 1];
    cur.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_u16_le(cur: &mut Cursor<Vec<u8>>) -> Result<u16> {
    let mut b = [0u8; 2];
    cur.read_exact(&mut b)?;
    Ok(u16::from_le_bytes(b))
}

pub fn extract_cats<P: AsRef<Path>>(cats_path: P, out_dir: P) -> Result<()> {
    info!("Starting CATS extraction from {:?}", cats_path.as_ref());

    let mut data = Vec::new();
    File::open(&cats_path)
        .map_err(|e| anyhow!("Failed to open CATS file: {}", e))?
        .read_to_end(&mut data)?;

    let mut cur = Cursor::new(data);

    // Read magic
    let mut magic = [0u8; 4];
    cur.read_exact(&mut magic)?;
    if &magic != b"CATS" {
        error!("Invalid CATS magic header");
        return Err(anyhow!("Not a CATS file"));
    }
    debug!("Magic OK: CATS");

    // Version
    let version = read_u8(&mut cur)?;
    if version != 1 {
        error!("Unsupported CATS version: {}", version);
        return Err(anyhow!("Unsupported CATS version {}", version));
    }
    debug!("CATS version: {}", version);

    // Flags (unused)
    let _flags = read_u8(&mut cur)?;

    // Sprite count
    let sprite_count = read_u16_le(&mut cur)?;
    info!("Sprite count: {}", sprite_count);

    let mut sprites = Vec::with_capacity(sprite_count as usize);

    for i in 0..sprite_count {
        let name_len = read_u8(&mut cur)? as usize;
        let mut name_buf = vec![0u8; name_len];
        cur.read_exact(&mut name_buf)?;
        let name = String::from_utf8(name_buf)?;

        let x = read_u16_le(&mut cur)?;
        let y = read_u16_le(&mut cur)?;
        let w = read_u16_le(&mut cur)?;
        let h = read_u16_le(&mut cur)?;

        debug!(
            "Sprite {}: {} @ ({}, {}) size {}x{}",
            i, name, x, y, w, h
        );

        sprites.push(Sprite { name, x, y, w, h });
    }

    // Remaining bytes = RGBA atlas
    let pos = cur.position() as usize;
    let buf = cur.into_inner();
    let pixels = &buf[pos..];

    // Your atlas is 918x918 (confirmed from file size)
    let atlas_w: u32 = 918;
    let atlas_h: u32 = 918;

    let expected_bytes = (atlas_w * atlas_h * 4) as usize;
    if pixels.len() < expected_bytes {
        error!(
            "Atlas pixel data too small: {} bytes, expected {}",
            pixels.len(),
            expected_bytes
        );
        return Err(anyhow!("Atlas pixel data incomplete"));
    }

    let atlas_img = RgbaImage::from_raw(atlas_w, atlas_h, pixels[..expected_bytes].to_vec())
        .ok_or_else(|| anyhow!("Failed to decode atlas image"))?;

    // Ensure output directory exists
    std::fs::create_dir_all(&out_dir)?;

    info!("Extracting {} sprites…", sprites.len());

    for s in sprites {
        let Sprite { name, x, y, w, h } = s;
        let (x, y, w, h) = (x as u32, y as u32, w as u32, h as u32);

        let mut sub = ImageBuffer::new(w, h);

        for yy in 0..h {
            for xx in 0..w {
                let px = atlas_img.get_pixel(x + xx, y + yy);
                sub.put_pixel(xx, yy, *px);
            }
        }

        let out_path = out_dir.as_ref().join(&name);

        if let Err(e) = sub.save(&out_path) {
            error!("Failed to save sprite {}: {}", name, e);
        } else {
            debug!("Saved {}", name);
        }
    }

    info!("CATS extraction complete.");
    Ok(())
}
