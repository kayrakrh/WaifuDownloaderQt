use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SaveMode {
    #[default]
    DefaultFolder,
    Ask,
}

impl SaveMode {
    pub const ALL: [SaveMode; 2] = [SaveMode::DefaultFolder, SaveMode::Ask];

    pub fn label(self) -> &'static str {
        match self {
            SaveMode::DefaultFolder => "Default folder",
            SaveMode::Ask => "Ask every time",
        }
    }

    pub fn from_index(i: i32) -> Self {
        Self::ALL.get(i as usize).copied().unwrap_or_default()
    }

    pub fn index(self) -> i32 {
        Self::ALL.iter().position(|m| *m == self).unwrap_or(0) as i32
    }

    fn key(self) -> &'static str {
        match self {
            SaveMode::DefaultFolder => "default",
            SaveMode::Ask => "ask",
        }
    }

    fn from_key(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|m| m.key() == s)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    pub save_mode: SaveMode,
}

fn home() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

pub fn images_dir() -> PathBuf {
    home().join(".local/share/waifudownloader/images")
}

pub fn config_path() -> PathBuf {
    home().join(".config/waifudownloader/configs.conf")
}

pub fn load() -> Config {
    let mut cfg = Config::default();
    let Ok(text) = fs::read_to_string(config_path()) else {
        return cfg;
    };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == "save_mode" {
                if let Some(m) = SaveMode::from_key(v.trim()) {
                    cfg.save_mode = m;
                }
            }
        }
    }
    cfg
}

pub fn save(cfg: &Config) -> Result<()> {
    let path = config_path();
    let dir = path.parent().context("config yolu geçersiz")?;
    fs::create_dir_all(dir).with_context(|| format!("{} oluşturulamadı", dir.display()))?;

    let content = format!("# WaifuDownloader\nsave_mode={}\n", cfg.save_mode.key());
    let tmp = path.with_extension("conf.tmp");
    fs::write(&tmp, content)?;
    fs::rename(&tmp, &path)?; 
    Ok(())
}
