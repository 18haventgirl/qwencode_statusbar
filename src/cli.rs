use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "qcstatusbar")]
#[command(version, about = "High-performance Qwen Code StatusLine tool")]
pub struct Cli {
    /// Set theme
    #[arg(short = 't', long = "theme")]
    pub theme: Option<String>,

    /// Sync witty phrases from config.toml to ~/.qwen/settings.json
    #[arg(long = "sync-phrases")]
    pub sync_phrases: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
