//! Theme handling for markdown rendering

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use termimad::crossterm::style::Color;

/// Color theme for markdown rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Heading colors (hex or named) - can be single color or array for H1-H6
    #[serde(default = "default_heading")]
    pub heading: HeadingColors,
    /// Code block background/border color
    pub code: String,
    /// Bold text color
    pub bold: String,
    /// Italic text color
    pub italic: String,
    /// Link color
    pub link: String,
    /// List bullet color
    pub list: String,
}

fn default_heading() -> HeadingColors {
    HeadingColors::Single("#ffffff".to_string())
}

/// Heading color configuration - either single color for all headers or individual colors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeadingColors {
    /// Single color for all heading levels
    Single(String),
    /// Individual colors for each heading level (H1-H6)
    Multiple(Vec<String>),
}

impl Theme {
    /// Convert string color name to crossterm Color
    pub fn parse_color(color_str: &str) -> Color {
        // Check if it's a hex color
        if color_str.starts_with('#') && color_str.len() == 7 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&color_str[1..3], 16),
                u8::from_str_radix(&color_str[3..5], 16),
                u8::from_str_radix(&color_str[5..7], 16)) {
                return Color::Rgb { r, g, b };
            }
        }

        // Fall back to named colors
        match color_str.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "dark_grey" | "dark_gray" => Color::DarkGrey,
            "grey" | "gray" => Color::Grey,
            _ => Color::White, // Default fallback
        }
    }

    /// Get color for a theme field
    pub fn get_color(&self, field: &str) -> Color {
        let color_str = match field {
            "code" => &self.code,
            "bold" => &self.bold,
            "italic" => &self.italic,
            "link" => &self.link,
            "list" => &self.list,
            _ => "white",
        };
        Self::parse_color(color_str)
    }

    /// Get heading color for a specific level (1-6)
    pub fn get_heading_color(&self, level: usize) -> Color {
        match &self.heading {
            HeadingColors::Single(color) => Self::parse_color(color.as_str()),
            HeadingColors::Multiple(colors) => {
                if let Some(c) = colors.get(level.saturating_sub(1)) {
                    Self::parse_color(c.as_str())
                } else if let Some(last) = colors.last() {
                    Self::parse_color(last.as_str())
                } else {
                    Self::parse_color("#ffffff")
                }
            }
        }
    }

    /// Default dark theme
    pub fn dark() -> Self {
        Self {
            heading: HeadingColors::Single("#89b4fa".to_string()), // Catppuccin blue
            code: "#1e1e2e".to_string(), // Catppuccin base
            bold: "#cdd6f4".to_string(), // Catppuccin text
            italic: "#f5c2e7".to_string(), // Catppuccin pink
            link: "#a6e3a1".to_string(), // Catppuccin green
            list: "#f9e2af".to_string(), // Catppuccin yellow
        }
    }

    /// Light theme for light backgrounds
    pub fn light() -> Self {
        Self {
            heading: HeadingColors::Single("#1e66f5".to_string()), // Catppuccin blue
            code: "#eff1f5".to_string(), // Catppuccin base
            bold: "#4c4f69".to_string(), // Catppuccin text
            italic: "#ea76cb".to_string(), // Catppuccin pink
            link: "#40a02b".to_string(), // Catppuccin green
            list: "#df8e1d".to_string(), // Catppuccin yellow
        }
    }

    /// Monochrome theme
    pub fn mono() -> Self {
        Self {
            heading: HeadingColors::Single("#4c4f69".to_string()), // Catppuccin blue
            code: "#dce0e8".to_string(), // Catppuccin surface0
            bold: "#4c4f69".to_string(), // Catppuccin text
            italic: "#4c4f69".to_string(), // Catppuccin text
            link: "#40a02b".to_string(), // Catppuccin green
            list: "#df8e1d".to_string(), // Catppuccin yellow
        }
    }

    /// Load theme from JSON file
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&content)?;
        Ok(theme)
    }
}