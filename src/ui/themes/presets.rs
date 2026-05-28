use crate::config::defaults::{default_phrases, default_segments};
use crate::config::{Config, StyleConfig, StyleMode};
use std::fs;

pub struct ThemePresets;

impl ThemePresets {
    pub fn get_theme(theme_name: &str) -> Config {
        if let Ok(config) = Self::load_theme_from_file(theme_name) {
            return config;
        }

        match theme_name {
            "default" => Self::get_default(),
            "minimal" => Self::get_minimal(),
            "nord" => Self::get_nord(),
            "gruvbox" => Self::get_gruvbox(),
            _ => Self::get_default(),
        }
    }

    fn load_theme_from_file(theme_name: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        if !theme_path.exists() {
            return Err(format!("Theme file not found: {}", theme_path.display()).into());
        }

        let content = fs::read_to_string(&theme_path)?;
        let mut config: Config = toml::from_str(&content)?;
        config.theme = theme_name.to_string();

        Ok(config)
    }

    fn get_themes_path() -> std::path::PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".qwen").join("qcstatusbar").join("themes")
        } else {
            std::path::PathBuf::from(".qwen/qcstatusbar/themes")
        }
    }

    pub fn get_default() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " | ".to_string(),
            },
            segments: default_segments(),
            theme: "default".to_string(),
            phrases: default_phrases(),
        }
    }

    pub fn get_minimal() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " │ ".to_string(),
            },
            segments: default_segments(),
            theme: "minimal".to_string(),
            phrases: default_phrases(),
        }
    }

    pub fn get_nord() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: default_segments(),
            theme: "nord".to_string(),
            phrases: default_phrases(),
        }
    }

    pub fn get_gruvbox() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: " | ".to_string(),
            },
            segments: default_segments(),
            theme: "gruvbox".to_string(),
            phrases: default_phrases(),
        }
    }
}
