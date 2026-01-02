//! Configuration loading and types

use serde::{Deserialize, Serialize};

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub airplay: AirPlayConfig,
    pub spotify: SpotifyConfig,
    pub views: ViewsConfig,
    pub logging: LoggingConfig,
    pub assets: AssetsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub fps: u32,
    pub fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirPlayConfig {
    pub poll_interval_ms: u64,
    pub state_file: String,
    pub background_fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyConfig {
    pub poll_interval_ms: u64,
    pub background_poll_interval_ms: u64,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub token_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewsConfig {
    pub default: String,
    pub transition_duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsConfig {
    pub cache_dir: String,
    pub max_cache_size_mb: u64,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

