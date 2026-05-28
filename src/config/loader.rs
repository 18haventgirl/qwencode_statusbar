use super::types::Config;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum InitResult {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Config {
        Config::load().unwrap_or_else(|_| Config::default())
    }

    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_themes_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".qwen").join("qcstatusbar").join("themes")
        } else {
            PathBuf::from(".qwen/qcstatusbar/themes")
        }
    }

    pub fn ensure_themes_exist() {
        let _ = Self::init_themes_silent();
    }

    fn init_themes_silent() -> Result<(), Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        fs::create_dir_all(&themes_dir)?;

        let builtin_themes = ["default", "minimal", "nord", "gruvbox"];

        for theme_name in &builtin_themes {
            let theme_path = themes_dir.join(format!("{}.toml", theme_name));
            if !theme_path.exists() {
                let theme_config = crate::ui::themes::ThemePresets::get_theme(theme_name);
                let content = toml::to_string_pretty(&theme_config)?;
                fs::write(&theme_path, content)?;
            }
        }

        Ok(())
    }
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        ConfigLoader::ensure_themes_exist();

        let config_path = Self::get_config_path();

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    fn get_config_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".qwen").join("qcstatusbar").join("config.toml")
        } else {
            PathBuf::from(".qwen/qcstatusbar/config.toml")
        }
    }

    pub fn init() -> Result<InitResult, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            Ok(InitResult::Created(config_path))
        } else {
            Ok(InitResult::AlreadyExists(config_path))
        }
    }

    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.segments.is_empty() {
            return Err("No segments configured".into());
        }

        let mut seen_ids = std::collections::HashSet::new();
        for segment in &self.segments {
            if !seen_ids.insert(segment.id) {
                return Err(format!("Duplicate segment ID: {:?}", segment.id).into());
            }
        }

        Ok(())
    }
}
