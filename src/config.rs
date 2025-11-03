//! Configuration handling

use directories::ProjectDirs;
use serde::Deserialize;
use std::path::PathBuf;

/// Configuration file structure
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// Path to theme file (relative to config directory)
    pub theme_file: Option<String>,
}

impl ConfigFile {
    /// Load configuration from XDG config directories
    pub fn load() -> Option<Self> {
        let project_dirs = ProjectDirs::from("com", "livemd", "livemd")?;
        let config_path = project_dirs.config_dir().join("config.json");

        if config_path.exists() {
            match std::fs::read_to_string(&config_path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(config) => Some(config),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse config file {:?}: {}", config_path, e);
                        None
                    }
                },
                Err(e) => {
                    eprintln!("Warning: Failed to read config file {:?}: {}", config_path, e);
                    None
                }
            }
        } else {
            None
        }
    }
}

/// Configuration for the Markdown streamer
#[derive(Debug)]
pub struct StreamerConfig {
    /// Maximum chunk size before forcing a flush
    pub chunk_size: usize,
    /// Delay between chunks in seconds for streaming effect
    pub speed: f64,
    /// Whether to convert ASCII box drawings to Markdown headers
    pub strip_boxes: bool,
    /// Command to invoke for LLM functionality
    pub llm_cmd: String,
    /// Whether to inject Markdown instruction for LLM queries
    pub inject_md_instruction: bool,
    /// Theme name for color selection
    pub theme_name: String,
    /// Path to custom theme JSON file
    pub theme_file: Option<PathBuf>,
}