//! Configuration handling
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// LLM command configuration - either a single command or multiple named commands
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LlmCmdConfig {
    /// Single LLM command string
    Single(String),
    /// Multiple named LLM commands
    Multiple(HashMap<String, String>),
}

/// Configuration file structure
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// Path to theme file (relative to config directory)
    #[serde(rename = "theme-file")]
    pub theme_file: Option<String>,
    /// Default LLM command
    #[serde(rename = "llm-cmd")]
    pub llm_cmd: Option<LlmCmdConfig>,
    /// Default streaming speed
    #[serde(rename = "speed")]
    pub speed: Option<f64>,
    /// Default chunk size
    #[serde(rename = "chunk-size")]
    pub chunk_size: Option<usize>,
    /// Default theme name
    #[serde(rename = "theme")]
    pub theme: Option<String>,
    /// Whether to strip boxes by default
    #[serde(rename = "strip-boxes")]
    pub strip_boxes: Option<bool>,
    /// Whether to inject markdown instruction by default
    #[serde(rename = "inject-md-instruction")]
    pub inject_md_instruction: Option<bool>,
}

impl ConfigFile {
    /// Load configuration from ~/.config/livemd/config.json
    pub fn load() -> Option<Self> {
        let config_path = dirs::home_dir()
            .map(|h| h.join(".config").join("livemd").join("config.json"))
            .unwrap_or_default();

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

    /// Resolve LLM command from config and CLI arg
    pub fn resolve_llm_cmd(&self, cli_llm_cmd: Option<&str>) -> Option<String> {
        match (&self.llm_cmd, cli_llm_cmd) {
            // If config has multiple commands and CLI specifies a preset
            (Some(LlmCmdConfig::Multiple(map)), Some(preset)) => {
                map.get(preset).cloned()
            }
            // If config has multiple commands and no CLI preset, use "default" if it exists
            (Some(LlmCmdConfig::Multiple(map)), None) => {
                map.get("default").or_else(|| map.values().next()).cloned()
            }
            // If config has single command
            (Some(LlmCmdConfig::Single(cmd)), _) => Some(cmd.clone()),
            // No config, use CLI
            (None, Some(cmd)) => Some(cmd.to_string()),
            (None, None) => None,
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
    pub llm_cmd: Option<String>,
    /// Whether to inject Markdown instruction for LLM queries
    pub inject_md_instruction: bool,
    /// Theme name for color selection
    pub theme_name: String,
    /// Path to custom theme JSON file
    pub theme_file: Option<PathBuf>,
}