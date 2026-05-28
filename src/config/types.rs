use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Main config structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub style: StyleConfig,
    pub segments: Vec<SegmentConfig>,
    pub theme: String,
    #[serde(default)]
    pub phrases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub mode: StyleMode,
    pub separator: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleMode {
    Plain,
    NerdFont,
    Powerline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentConfig {
    pub id: SegmentId,
    pub enabled: bool,
    pub icon: IconConfig,
    pub colors: ColorConfig,
    pub styles: TextStyleConfig,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconConfig {
    pub plain: String,
    pub nerd_font: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub icon: Option<AnsiColor>,
    pub text: Option<AnsiColor>,
    pub background: Option<AnsiColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyleConfig {
    pub text_bold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnsiColor {
    Color16 { c16: u8 },
    Color256 { c256: u8 },
    Rgb { r: u8, g: u8, b: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentId {
    Model,
    Directory,
    Git,
    ContextWindow,
    Session,
    Metrics,
}

// ============================================================================
// Qwen Code stdin JSON input structures
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct QwenInput {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub model: QwenModel,
    #[serde(default)]
    pub context_window: QwenContextWindow,
    #[serde(default)]
    pub workspace: QwenWorkspace,
    #[serde(default)]
    pub git: Option<QwenGit>,
    #[serde(default)]
    pub metrics: QwenMetrics,
    #[serde(default)]
    pub vim: Option<QwenVim>,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenModel {
    #[serde(default)]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenContextWindow {
    #[serde(default)]
    pub context_window_size: u32,
    #[serde(default)]
    pub used_percentage: f64,
    #[serde(default)]
    pub remaining_percentage: f64,
    #[serde(default)]
    pub current_usage: u32,
    #[serde(default)]
    pub total_input_tokens: u32,
    #[serde(default)]
    pub total_output_tokens: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenWorkspace {
    #[serde(default)]
    pub current_dir: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenGit {
    #[serde(default)]
    pub branch: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenMetrics {
    #[serde(default)]
    pub models: HashMap<String, QwenModelMetrics>,
    #[serde(default)]
    pub files: QwenFileMetrics,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenModelMetrics {
    #[serde(default)]
    pub api: QwenApiMetrics,
    #[serde(default)]
    pub tokens: QwenTokenMetrics,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenApiMetrics {
    #[serde(default)]
    pub total_requests: u32,
    #[serde(default)]
    pub total_errors: u32,
    #[serde(default)]
    pub total_latency_ms: u64,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenTokenMetrics {
    #[serde(default)]
    pub prompt: u32,
    #[serde(default)]
    pub completion: u32,
    #[serde(default)]
    pub total: u32,
    #[serde(default)]
    pub cached: u32,
    #[serde(default)]
    pub thoughts: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenFileMetrics {
    #[serde(default)]
    pub total_lines_added: u32,
    #[serde(default)]
    pub total_lines_removed: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct QwenVim {
    #[serde(default)]
    pub mode: String,
}

impl PartialEq for AnsiColor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AnsiColor::Color16 { c16: a }, AnsiColor::Color16 { c16: b }) => a == b,
            (AnsiColor::Color256 { c256: a }, AnsiColor::Color256 { c256: b }) => a == b,
            (
                AnsiColor::Rgb {
                    r: r1,
                    g: g1,
                    b: b1,
                },
                AnsiColor::Rgb {
                    r: r2,
                    g: g2,
                    b: b2,
                },
            ) => r1 == r2 && g1 == g2 && b1 == b2,
            _ => false,
        }
    }
}

impl Config {
    /// Check if current config matches the specified theme preset
    pub fn matches_theme(&self, theme_name: &str) -> bool {
        let theme_preset = crate::ui::themes::ThemePresets::get_theme(theme_name);

        if self.style.mode != theme_preset.style.mode
            || self.style.separator != theme_preset.style.separator
        {
            return false;
        }

        if self.segments.len() != theme_preset.segments.len() {
            return false;
        }

        for (current, preset) in self.segments.iter().zip(theme_preset.segments.iter()) {
            if !self.segment_matches(current, preset) {
                return false;
            }
        }

        true
    }

    pub fn is_modified_from_theme(&self) -> bool {
        !self.matches_theme(&self.theme)
    }

    fn segment_matches(&self, current: &SegmentConfig, preset: &SegmentConfig) -> bool {
        current.id == preset.id
            && current.enabled == preset.enabled
            && current.icon.plain == preset.icon.plain
            && current.icon.nerd_font == preset.icon.nerd_font
            && self.color_matches(&current.colors.icon, &preset.colors.icon)
            && self.color_matches(&current.colors.text, &preset.colors.text)
            && self.color_matches(&current.colors.background, &preset.colors.background)
            && current.styles.text_bold == preset.styles.text_bold
            && current.options == preset.options
    }

    fn color_matches(&self, current: &Option<AnsiColor>, preset: &Option<AnsiColor>) -> bool {
        match (current, preset) {
            (None, None) => true,
            (Some(c1), Some(c2)) => c1 == c2,
            _ => false,
        }
    }
}
