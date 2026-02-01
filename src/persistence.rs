// Comprehensive persistence layer for all user settings and state
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Custom serialization for DateTime<Utc>
mod datetime_format {
    use chrono::{DateTime, Utc};
    use serde::{Serializer, Deserializer, Deserialize};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Central persistence store for all application state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub version: String,
    #[serde(with = "datetime_format")]
    pub last_updated: DateTime<Utc>,
    pub settings_id: String,

    // Theme
    pub current_theme: String,
    pub custom_theme_paths: Vec<PathBuf>,
    pub theme_history: Vec<String>,

    // UI State
    pub last_opened_directory: PathBuf,
    pub opened_tabs: Vec<TabState>,
    pub active_tab_index: usize,
    pub preview_width_ratio: f32,

    // Bookmarks
    pub bookmarks: Vec<BookmarkState>,

    // Search
    pub search_history: Vec<SearchQueryState>,
    pub max_search_history: usize,

    // Plugins
    pub enabled_plugins: Vec<String>,
    pub plugin_settings: HashMap<String, serde_json::Value>,
    pub plugin_directory: PathBuf,

    // General
    pub show_hidden_files: bool,
    pub vim_mode: bool,
    pub mouse_enabled: bool,
    pub auto_preview: bool,
    pub preserve_case_on_rename: bool,

    // Keybindings (user custom)
    pub custom_keybindings: HashMap<String, String>,

    // Performance
    pub max_file_preview_size: u64,
    pub parallel_search_threads: usize,

    // UI Preferences
    pub emoji_style: EmojiStyle,
    pub border_style: BorderStyle,
    pub status_bar_position: StatusBarPosition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TabState {
    pub id: String,
    pub path: PathBuf,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub title: Option<String>,    #[serde(with = "datetime_format")]    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookmarkState {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub emoji: String,
    pub tags: Vec<String>,
    #[serde(with = "datetime_format")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "datetime_format")]
    pub last_accessed: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueryState {
    pub query: String,
    #[serde(with = "datetime_format")]
    pub timestamp: DateTime<Utc>,
    pub result_count: usize,
    pub last_used_directory: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EmojiStyle {
    Full,      // Complete emoji set
    Minimal,   // Single character indicators
    Disabled,  // No emojis
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BorderStyle {
    Rounded,
    Sharp,
    Double,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StatusBarPosition {
    Bottom,
    Top,
    Hidden,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            last_updated: Utc::now(),
            settings_id: Uuid::new_v4().to_string(),
            current_theme: "default".to_string(),
            custom_theme_paths: Vec::new(),
            theme_history: vec!["default".to_string()],
            last_opened_directory: dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            opened_tabs: vec![],
            active_tab_index: 0,
            preview_width_ratio: 0.7,
            bookmarks: Vec::new(),
            search_history: Vec::new(),
            max_search_history: 100,
            enabled_plugins: Vec::new(),
            plugin_settings: HashMap::new(),
            plugin_directory: PathBuf::from("./plugins"),
            show_hidden_files: false,
            vim_mode: true,
            mouse_enabled: false,
            auto_preview: true,
            preserve_case_on_rename: false,
            custom_keybindings: HashMap::new(),
            max_file_preview_size: 10 * 1024 * 1024, // 10MB
            parallel_search_threads: num_cpus::get(),
            emoji_style: EmojiStyle::Full,
            border_style: BorderStyle::Rounded,
            status_bar_position: StatusBarPosition::Bottom,
        }
    }
}

pub struct PersistenceManager {
    config_dir: PathBuf,
    settings_file: PathBuf,
}

impl Default for PersistenceManager {
    fn default() -> Self {
        Self {
            config_dir: PathBuf::from("./config"),
            settings_file: PathBuf::from("./config/settings.json"),
        }
    }
}

impl PersistenceManager {
    pub fn new() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir)?;

        let settings_file = config_dir.join("settings.json");

        Ok(Self {
            config_dir,
            settings_file,
        })
    }

    fn get_config_dir() -> Result<PathBuf> {
        let config_dir = if let Some(config_home) = dirs::config_dir() {
            config_home.join("astrofs")
        } else {
            dirs::home_dir()
                .ok_or_else(|| anyhow!("Could not determine home directory"))?
                .join(".config")
                .join("astrofs")
        };

        fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }

    pub fn load_settings(&self) -> Result<UserSettings> {
        if self.settings_file.exists() {
            let content = fs::read_to_string(&self.settings_file)?;
            let settings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(UserSettings::default())
        }
    }

    pub fn save_settings(&self, settings: &UserSettings) -> Result<()> {
        let mut updated = settings.clone();
        updated.last_updated = Utc::now();

        let json = serde_json::to_string_pretty(&updated)?;
        fs::write(&self.settings_file, json)?;
        Ok(())
    }

    pub fn get_config_dir_path(&self) -> &Path {
        &self.config_dir
    }

    /// Describe settings state for diagnostics
    pub fn describe_settings(settings: &UserSettings) -> String {
        format!("Settings: theme={}, bookmarks={}, queries={}",
            settings.current_theme, settings.bookmarks.len(), settings.search_history.len())
    }

    /// Add a theme to history
    pub fn add_theme_to_history(settings: &mut UserSettings, theme_name: String) {
        if !settings.theme_history.contains(&theme_name) {
            settings.theme_history.push(theme_name);
        }
    }

    /// Add bookmark
    pub fn add_bookmark(
        settings: &mut UserSettings,
        name: String,
        path: PathBuf,
        emoji: String,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let bookmark = BookmarkState {
            id: id.clone(),
            name,
            path,
            emoji,
            tags: Vec::new(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };
        settings.bookmarks.push(bookmark);
        id
    }

    /// Add search query to history
    pub fn add_search_query(
        settings: &mut UserSettings,
        query: String,
        result_count: usize,
        last_used_directory: PathBuf,
    ) {
        let search = SearchQueryState {
            query,
            timestamp: Utc::now(),
            result_count,
            last_used_directory,
        };

        settings.search_history.push(search);

        // Trim to max size
        if settings.search_history.len() > settings.max_search_history {
            settings.search_history.remove(0);
        }
    }

    /// Add opened tab
    pub fn add_tab(settings: &mut UserSettings, path: PathBuf, title: Option<String>) -> String {
        let id = Uuid::new_v4().to_string();
        let tab = TabState {
            id: id.clone(),
            path,
            selected_index: 0,
            scroll_offset: 0,
            title,
            created_at: Utc::now(),
        };
        settings.opened_tabs.push(tab);
        id
    }

    /// Save custom keybinding
    pub fn set_keybinding(settings: &mut UserSettings, key: String, action: String) {
        settings.custom_keybindings.insert(key, action);
    }

    /// Get plugin data
    pub fn get_plugin_data(&self, plugin_name: &str) -> Result<Option<serde_json::Value>> {
        let settings = self.load_settings()?;
        Ok(settings.plugin_settings.get(plugin_name).cloned())
    }

    /// Save plugin data
    pub fn save_plugin_data(
        &self,
        plugin_name: &str,
        data: serde_json::Value,
    ) -> Result<()> {
        let mut settings = self.load_settings()?;
        settings.plugin_settings.insert(plugin_name.to_string(), data);
        self.save_settings(&settings)?;
        Ok(())
    }

    /// Export settings to backup
    pub fn export_settings(&self, backup_path: &Path) -> Result<()> {
        let settings = self.load_settings()?;
        let json = serde_json::to_string_pretty(&settings)?;
        fs::write(backup_path, json)?;
        Ok(())
    }

    /// Import settings from backup
    pub fn import_settings(&self, backup_path: &Path) -> Result<()> {
        let content = fs::read_to_string(backup_path)?;
        let settings: UserSettings = serde_json::from_str(&content)?;
        self.save_settings(&settings)?;
        Ok(())
    }

    /// Static method to load settings from default location
    pub fn load_default() -> Result<UserSettings> {
        let manager = PersistenceManager::new()?;
        manager.load_settings()
    }

    /// Static method to save settings to default location
    pub fn save_default(settings: &UserSettings) -> Result<()> {
        let manager = PersistenceManager::new()?;
        manager.save_settings(settings)
    }
}

impl Default for EmojiStyle {
    fn default() -> Self {
        EmojiStyle::Full
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::Rounded
    }
}

impl Default for StatusBarPosition {
    fn default() -> Self {
        StatusBarPosition::Bottom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = UserSettings::default();
        assert_eq!(settings.current_theme, "default");
        assert_eq!(settings.max_search_history, 100);
    }

    #[test]
    fn test_add_bookmark() {
        let mut settings = UserSettings::default();
        let id = PersistenceManager::add_bookmark(
            &mut settings,
            "test".to_string(),
            PathBuf::from("/tmp"),
            "🔖".to_string(),
        );
        assert_eq!(settings.bookmarks.len(), 1);
        assert_eq!(settings.bookmarks[0].id, id);
    }
}
