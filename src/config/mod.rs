// Configuration management

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(toml::de::Error),
    SerializeError(toml::ser::Error),
    InvalidPath,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::ParseError(e) => write!(f, "Failed to parse config: {}", e),
            ConfigError::SerializeError(e) => write!(f, "Failed to serialize config: {}", e),
            ConfigError::InvalidPath => write!(f, "Invalid config path"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::IoError(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::ParseError(e)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(e: toml::ser::Error) -> Self {
        ConfigError::SerializeError(e)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub updates: UpdateConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub color_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_theme")]
    pub theme: ThemeMode,
    #[serde(default = "default_history_size")]
    pub history_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Dark,
    Light,
    System,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(default = "default_channel")]
    pub channel: String,
    #[serde(default = "default_true")]
    pub check_on_startup: bool,
    #[serde(default = "default_true")]
    pub check_in_background: bool,
    #[serde(default = "default_true")]
    pub prompt_before_update: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_true")]
    pub show_system_tray: bool,
    #[serde(default = "default_true")]
    pub remember_window_position: bool,
    #[serde(default = "default_format")]
    pub default_color_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            updates: UpdateConfig::default(),
            ui: UiConfig::default(),
            color_history: Vec::new(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            history_size: default_history_size(),
        }
    }
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            channel: default_channel(),
            check_on_startup: true,
            check_in_background: true,
            prompt_before_update: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_system_tray: true,
            remember_window_position: true,
            default_color_format: default_format(),
        }
    }
}

fn default_theme() -> ThemeMode {
    ThemeMode::Dark
}

fn default_history_size() -> usize {
    20
}

fn default_channel() -> String {
    "stable".to_string()
}

fn default_true() -> bool {
    true
}

fn default_format() -> String {
    "hex".to_string()
}

impl Config {
    pub fn load() -> Self {
        match Self::try_load() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: Failed to load config ({}), using defaults", e);
                Self::default()
            }
        }
    }

    fn try_load() -> Result<Self, ConfigError> {
        let path = Self::config_path()?;
        
        if path.exists() {
            let contents = fs::read_to_string(&path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::config_path()?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, contents)?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf, ConfigError> {
        let mut path = dirs::config_dir().ok_or(ConfigError::InvalidPath)?;
        path.push("casapps");
        path.push("cascolor");
        path.push("config.toml");
        Ok(path)
    }
    
    pub fn config_path_string() -> String {
        Self::config_path()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "Unknown".to_string())
    }
}
