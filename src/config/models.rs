use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    #[serde(rename = "models", default)]
    pub model_entries: Vec<ModelEntry>,
    #[serde(default)]
    pub context_modifiers: Vec<ContextModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    pub pattern: String,
    pub display_name: String,
    pub context_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextModifier {
    pub pattern: String,
    pub display_suffix: String,
    pub context_limit: u32,
}

impl ModelConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: ModelConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load() -> Self {
        let mut model_config = Self::default();

        if let Some(home_dir) = dirs::home_dir() {
            let user_models_path = home_dir
                .join(".qwen")
                .join("qcstatusbar")
                .join("models.toml");
            if !user_models_path.exists() {
                let _ = Self::create_default_file(&user_models_path);
            }
        }

        let config_paths = [
            dirs::home_dir().map(|d| d.join(".qwen").join("qcstatusbar").join("models.toml")),
            Some(Path::new("models.toml").to_path_buf()),
        ];

        for path in config_paths.iter().flatten() {
            if path.exists() {
                if let Ok(config) = Self::load_from_file(path) {
                    let mut merged_entries = config.model_entries;
                    merged_entries.extend(model_config.model_entries);
                    model_config.model_entries = merged_entries;

                    let mut merged_modifiers = config.context_modifiers;
                    merged_modifiers.extend(model_config.context_modifiers);
                    model_config.context_modifiers = merged_modifiers;

                    return model_config;
                }
            }
        }

        model_config
    }

    pub fn get_context_limit(&self, model_name: &str) -> u32 {
        let model_lower = model_name.to_lowercase();

        // Check context modifiers first
        for modifier in &self.context_modifiers {
            if model_lower.contains(&modifier.pattern.to_lowercase()) {
                return modifier.context_limit;
            }
        }

        // Check model entries
        for entry in &self.model_entries {
            if model_lower.contains(&entry.pattern.to_lowercase()) {
                return entry.context_limit;
            }
        }

        // Default
        131_072
    }

    pub fn get_display_name(&self, model_name: &str) -> Option<String> {
        let model_lower = model_name.to_lowercase();

        for entry in &self.model_entries {
            if model_lower.contains(&entry.pattern.to_lowercase()) {
                let mut name = entry.display_name.clone();

                // Apply context modifier suffix
                for modifier in &self.context_modifiers {
                    if model_lower.contains(&modifier.pattern.to_lowercase()) {
                        name.push_str(&modifier.display_suffix);
                        break;
                    }
                }

                return Some(name);
            }
        }

        None
    }

    pub fn create_default_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let template_content = "# QCStatusBar Model Configuration\n\
             # File location: ~/.qwen/qcstatusbar/models.toml\n\
             \n\
             # Model configurations (simple substring matching)\n\
             # [[models]]\n\
             # pattern = \"my-model\"\n\
             # display_name = \"My Model\"\n\
             # context_limit = 128000\n\
             \n\
             # Context modifiers override context limits and append suffix to display names\n\
             # [[context_modifiers]]\n\
             # pattern = \"[1m]\"\n\
             # display_suffix = \" 1M\"\n\
             # context_limit = 1000000\n"
            .to_string();

        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, template_content)?;
        Ok(())
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_entries: vec![
                ModelEntry {
                    pattern: "qwen3-coder".to_string(),
                    display_name: "Qwen Coder".to_string(),
                    context_limit: 256_000,
                },
                ModelEntry {
                    pattern: "qwen3".to_string(),
                    display_name: "Qwen 3".to_string(),
                    context_limit: 131_072,
                },
                ModelEntry {
                    pattern: "mimo".to_string(),
                    display_name: "MiMo".to_string(),
                    context_limit: 131_072,
                },
                ModelEntry {
                    pattern: "glm-4".to_string(),
                    display_name: "GLM-4".to_string(),
                    context_limit: 128_000,
                },
                ModelEntry {
                    pattern: "kimi-k2".to_string(),
                    display_name: "Kimi K2".to_string(),
                    context_limit: 128_000,
                },
            ],
            context_modifiers: vec![ContextModifier {
                pattern: "[1m]".to_string(),
                display_suffix: " 1M".to_string(),
                context_limit: 1_000_000,
            }],
        }
    }
}
