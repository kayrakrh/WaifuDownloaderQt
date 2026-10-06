use crate::config;
use anyhow::{anyhow, bail, Context, Result};
use image::ImageFormat;
use std::{
    fs,
    io::{ErrorKind, Write},
    path::PathBuf,
};

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const NAME_LEN: usize = 12;

/// İşletim sistemi RNG'sinden, yalnızca [A-Za-z0-9] içeren rastgele isim.
pub fn random_name(len: usize) -> Result<String> {
    let mut out = String::with_capacity(len);
    let mut buf = [0u8; 32];
    while out.len() < len {
        getrandom::fill(&mut buf).map_err(|e| anyhow!("rastgele veri alınamadı: {e}"))?;
        for &b in &buf {
            // 248 = 62 * 4: modulo bias olmaması için fazlasını at
            if b < 248 {
                out.push(ALPHABET[(b % 62) as usize] as char);
                if out.len() == len {
                    break;
                }
            }
        }
    }
    Ok(out)
}

pub fn extension_for(bytes: &[u8]) -> &'static str {
    match image::guess_format(bytes) {
        Ok(ImageFormat::Png) => "png",
        Ok(ImageFormat::Jpeg) => "jpg",
        Ok(ImageFormat::WebP) => "webp",
        Ok(ImageFormat::Gif) => "gif",
        _ => "png",
    }
}

/// Varsayılan klasöre, rastgele adla, orijinal baytları yazar.
pub fn save_default(bytes: &[u8]) -> Result<PathBuf> {
    let dir = config::images_dir();
    fs::create_dir_all(&dir).with_context(|| format!("{} oluşturulamadı", dir.display()))?;
    let ext = extension_for(bytes);

    for _ in 0..8 {
        let path = dir.join(format!("{}.{ext}", random_name(NAME_LEN)?));
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                f.write_all(bytes)?;
                return Ok(path);
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e).with_context(|| format!("{} yazılamadı", path.display())),
        }
    }
    bail!("benzersiz dosya adı üretilemedi")
}

/// Diyalog açar. Kullanıcı iptal ederse None döner.
pub fn save_ask(bytes: &[u8]) -> Option<Result<PathBuf>> {
    let detected = image::guess_format(bytes).ok();
    let name = match random_name(NAME_LEN) {
        Ok(n) => format!("{n}.{}", extension_for(bytes)),
        Err(e) => return Some(Err(e)),
    };

    let mut dialog = rfd::FileDialog::new()
        .set_file_name(name)
        .add_filter("PNG", &["png"])
        .add_filter("JPEG", &["jpg", "jpeg"])
        .add_filter("WebP", &["webp"]);
    let dir = config::images_dir();
    if dir.is_dir() {
        dialog = dialog.set_directory(dir);
    }
    let path = dialog.save_file()?;

    let res = (|| -> Result<()> {
        let target = ImageFormat::from_path(&path).ok();
        if detected.is_some() && detected == target {
            fs::write(&path, bytes)?;
        } else {
            let img = image::load_from_memory(bytes)?;
            if target == Some(ImageFormat::Jpeg) {
                img.to_rgb8().save(&path)?; // JPEG alfa desteklemez
            } else {
                img.save(&path)?;
            }
        }
        Ok(())
    })();
    Some(res.map(|_| path))
}
