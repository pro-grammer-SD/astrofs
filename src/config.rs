use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub show_hidden: bool,
    pub default_directory: String,
    pub preview_width_ratio: f32,
    pub max_search_results: usize,
    pub search_history_size: usize,
    pub enable_git_integration: bool,
    pub enable_plugins: bool,
    pub plugin_directory: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = config_dir().ok_or_else(|| anyhow::anyhow!("Cannot find config directory"))?;
        Ok(config_dir.join("astrofs").join("config.json"))
    }

    pub fn plugin_dir(&self) -> PathBuf {
        PathBuf::from(&self.plugin_directory)
    }

    pub fn validate(&mut self) {
        if self.preview_width_ratio < 0.1 || self.preview_width_ratio > 0.9 {
            self.preview_width_ratio = 0.7;
        }
        if self.search_history_size == 0 {
            self.search_history_size = 50;
        }
        if self.max_search_results == 0 {
            self.max_search_results = 100;
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "/".to_string());

        let plugin_dir = config_dir()
            .map(|d| d.join("astrofs").join("plugins"))
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "./plugins".to_string());

        Self {
            theme: "default".to_string(),
            show_hidden: false,
            default_directory: home_dir,
            preview_width_ratio: 0.7,
            max_search_results: 100,
            search_history_size: 50,
            enable_git_integration: true,
            enable_plugins: true,
            plugin_directory: plugin_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.theme, "default");
        assert!(!config.show_hidden);
        assert!(config.preview_width_ratio > 0.0 && config.preview_width_ratio < 1.0);
    }

    #[test]
    fn test_config_validation() {
        let mut config = AppConfig::default();
        config.preview_width_ratio = 1.5;
        config.search_history_size = 0;
        
        config.validate();
        
        assert!(config.preview_width_ratio <= 0.9);
        assert!(config.search_history_size > 0);
    }
}
