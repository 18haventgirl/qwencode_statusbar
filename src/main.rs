use qcstatusbar::cli::Cli;
use qcstatusbar::config::{Config, QwenInput};
use qcstatusbar::core::{collect_all_segments, StatusLineGenerator};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_args();

    // Load configuration
    let mut config = Config::load().unwrap_or_else(|_| Config::default());

    // Apply theme override if provided
    if let Some(theme) = cli.theme {
        config = qcstatusbar::ui::themes::ThemePresets::get_theme(&theme);
    }

    // Handle --sync-phrases
    if cli.sync_phrases {
        return sync_phrases(&config);
    }

    // Read Qwen Code JSON from stdin
    let mut stdin_content = String::new();
    io::stdin().read_to_string(&mut stdin_content)?;

    if stdin_content.trim().is_empty() {
        eprintln!("qcstatusbar: no input data. This tool is designed to be called by Qwen Code.");
        eprintln!("Configure in ~/.qwen/settings.json:");
        eprintln!("  {{\"ui\": {{\"statusLine\": {{\"type\": \"command\", \"command\": \"qcstatusbar\"}}}}}}");
        std::process::exit(1);
    }

    let input: QwenInput = serde_json::from_str(&stdin_content)?;

    // Collect segment data
    let segments_data = collect_all_segments(&config, &input);

    // Render statusline
    let generator = StatusLineGenerator::new(config);
    let statusline = generator.generate(segments_data);

    println!("{}", statusline);

    Ok(())
}

fn sync_phrases(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let settings_path = if let Some(home) = dirs::home_dir() {
        home.join(".qwen").join("settings.json")
    } else {
        return Err("Cannot determine home directory".into());
    };

    if !settings_path.exists() {
        return Err(format!("Settings file not found: {}", settings_path.display()).into());
    }

    let content = std::fs::read_to_string(&settings_path)?;
    let mut settings: serde_json::Value = serde_json::from_str(&content)?;

    let phrases_json = serde_json::Value::Array(
        config
            .phrases
            .iter()
            .map(|s| serde_json::Value::String(s.clone()))
            .collect(),
    );

    if let Some(ui) = settings.get_mut("ui") {
        ui["customWittyPhrases"] = phrases_json;
    } else {
        settings["ui"] = serde_json::json!({ "customWittyPhrases": phrases_json });
    }

    let output = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&settings_path, output)?;

    eprintln!(
        "✓ Synced {} phrases to {}",
        config.phrases.len(),
        settings_path.display()
    );

    Ok(())
}
