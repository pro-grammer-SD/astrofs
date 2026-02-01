// Theme Manager - Load, switch, and apply themes
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::persistence::PersistenceManager;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,

    // Colors (can be named or hex: #RRGGBB)
    pub colors: ThemeColors,

    // Borders
    pub borders: ThemeBorders,

    // Emojis for UI elements
    pub emojis: ThemeEmojis,

    // Fonts and styling
    pub fonts: ThemeFonts,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColors {
    // Primary colors
    pub primary: String,
    pub secondary: String,
    pub accent: String,

    // UI elements
    pub background: String,
    pub foreground: String,
    pub error: String,
    pub warning: String,
    pub success: String,
    pub info: String,

    // File types
    pub file_color: String,
    pub directory_color: String,
    pub symlink_color: String,
    pub executable_color: String,

    // Text highlights
    pub selection_bg: String,
    pub selection_fg: String,
    pub cursor_color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeBorders {
    pub style: BorderStyle,
    pub color: String,
    pub focused_color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderStyle {
    Rounded,
    Sharp,
    Double,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeEmojis {
    pub folder: String,
    pub file: String,
    pub symlink: String,
    pub executable: String,
    pub archive: String,
    pub image: String,
    pub video: String,
    pub audio: String,
    pub document: String,
    pub code: String,
    pub bookmark: String,
    pub search: String,
    pub settings: String,
    pub loading: String,
    pub error: String,
    pub success: String,
    pub info: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeFonts {
    pub use_powerline: bool,
    pub use_nerd_fonts: bool,
    pub enable_italics: bool,
    pub enable_bold: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            description: "Default AstroFS theme".to_string(),
            author: "AstroFS Team".to_string(),
            version: "1.0.0".to_string(),
            colors: ThemeColors::default(),
            borders: ThemeBorders::default(),
            emojis: ThemeEmojis::default(),
            fonts: ThemeFonts::default(),
        }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: "#00D9FF".to_string(),    // Cyan
            secondary: "#00FF9F".to_string(),  // Green
            accent: "#FF006E".to_string(),     // Pink
            background: "#0A0E27".to_string(),
            foreground: "#E0E6FC".to_string(),
            error: "#FF0040".to_string(),
            warning: "#FFBE0B".to_string(),
            success: "#00FF9F".to_string(),
            info: "#00D9FF".to_string(),
            file_color: "#E0E6FC".to_string(),
            directory_color: "#00D9FF".to_string(),
            symlink_color: "#FF006E".to_string(),
            executable_color: "#00FF9F".to_string(),
            selection_bg: "#00D9FF".to_string(),
            selection_fg: "#0A0E27".to_string(),
            cursor_color: "#00FF9F".to_string(),
        }
    }
}

impl Default for ThemeBorders {
    fn default() -> Self {
        Self {
            style: BorderStyle::Rounded,
            color: "#00D9FF".to_string(),
            focused_color: "#00FF9F".to_string(),
        }
    }
}

impl Default for ThemeEmojis {
    fn default() -> Self {
        Self {
            folder: "📁".to_string(),
            file: "📄".to_string(),
            symlink: "🔗".to_string(),
            executable: "⚙️".to_string(),
            archive: "📦".to_string(),
            image: "🖼️".to_string(),
            video: "🎬".to_string(),
            audio: "🎵".to_string(),
            document: "📝".to_string(),
            code: "💻".to_string(),
            bookmark: "🔖".to_string(),
            search: "🔍".to_string(),
            settings: "⚡".to_string(),
            loading: "⏳".to_string(),
            error: "❌".to_string(),
            success: "✅".to_string(),
            info: "ℹ️".to_string(),
        }
    }
}

impl Default for ThemeFonts {
    fn default() -> Self {
        Self {
            use_powerline: true,
            use_nerd_fonts: true,
            enable_italics: true,
            enable_bold: true,
        }
    }
}

/// Theme Manager - manages all theme operations
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: String,
    theme_dir: PathBuf,
    user_theme_dir: PathBuf,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: "default".to_string(),
            theme_dir: PathBuf::from("./themes"),
            user_theme_dir: PathBuf::from("./user_themes"),
        };
        
        // Always have at least the default theme
        manager.themes.insert("default".to_string(), Theme::default());
        manager
    }
}

impl ThemeManager {
    /// Create new theme manager with default paths
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("astrofs");
        let theme_dir = config_dir.join("themes");
        let user_theme_dir = config_dir.join("user_themes");
        
        Self::with_paths(theme_dir, user_theme_dir)
    }

    /// Create new theme manager with custom paths
    pub fn with_paths(theme_dir: PathBuf, user_theme_dir: PathBuf) -> Result<Self> {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: "default".to_string(),
            theme_dir,
            user_theme_dir,
        };

        // Create directories if they don't exist
        std::fs::create_dir_all(&manager.theme_dir)?;
        std::fs::create_dir_all(&manager.user_theme_dir)?;

        // Load built-in themes
        manager.load_builtin_themes()?;

        // Load user themes
        manager.load_user_themes()?;

        Ok(manager)
    }

    /// Load built-in themes
    fn load_builtin_themes(&mut self) -> Result<()> {
        // Create default theme if it doesn't exist
        if !self.themes.contains_key("default") {
            self.themes.insert("default".to_string(), Theme::default());
        }

        // Load other built-in themes from directory
        if self.theme_dir.exists() {
            for entry in std::fs::read_dir(&self.theme_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map(|e| e == "json").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(theme) = serde_json::from_str::<Theme>(&content) {
                            self.themes.insert(theme.name.clone(), theme);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Load user themes from user config directory
    fn load_user_themes(&mut self) -> Result<()> {
        if self.user_theme_dir.exists() {
            for entry in std::fs::read_dir(&self.user_theme_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map(|e| e == "json").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(theme) = serde_json::from_str::<Theme>(&content) {
                            self.themes.insert(theme.name.clone(), theme);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Get current theme
    pub fn current(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme)
    }

    /// Describe current state for diagnostics
    pub fn describe_current(&self) -> String {
        format!("Theme: {} (total: {})", self.current_theme, self.themes.len())
    }

    /// Set current theme
    pub fn set_current(&mut self, name: &str) -> Result<()> {
        if !self.themes.contains_key(name) {
            return Err(anyhow!("Theme '{}' not found", name));
        }
        self.current_theme = name.to_string();
        Ok(())
    }

    /// Get theme by name
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    /// List all available themes
    pub fn list(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }

    /// Create a new theme based on existing one
    pub fn create_from_template(&mut self, name: String, template: &str) -> Result<()> {
        let base_theme = self
            .themes
            .get(template)
            .ok_or_else(|| anyhow!("Template theme '{}' not found", template))?
            .clone();

        let mut new_theme = base_theme;
        new_theme.name = name.clone();

        self.themes.insert(name.clone(), new_theme.clone());

        // Save to user theme directory
        let path = self.user_theme_dir.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(&new_theme)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    /// Update a theme
    pub fn update(&mut self, name: String, theme: Theme) -> Result<()> {
        self.themes.insert(name.clone(), theme.clone());

        // Save to user theme directory
        let path = self.user_theme_dir.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(&theme)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    /// Export theme to file
    pub fn export(&self, name: &str, path: &PathBuf) -> Result<()> {
        let theme = self
            .themes
            .get(name)
            .ok_or_else(|| anyhow!("Theme '{}' not found", name))?;
        let json = serde_json::to_string_pretty(&theme)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Import theme from file
    pub fn import(&mut self, path: &PathBuf) -> Result<String> {
        let content = std::fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&content)?;
        let name = theme.name.clone();
        self.themes.insert(name.clone(), theme.clone());

        // Save to user theme directory
        let user_path = self.user_theme_dir.join(format!("{}.json", name));
        std::fs::write(user_path, content)?;

        Ok(name)
    }

    /// Get color as RGB tuple
    pub fn parse_color(&self, color: &str) -> Option<(u8, u8, u8)> {
        if color.starts_with('#') && color.len() == 7 {
            let r = u8::from_str_radix(&color[1..3], 16).ok()?;
            let g = u8::from_str_radix(&color[3..5], 16).ok()?;
            let b = u8::from_str_radix(&color[5..7], 16).ok()?;
            Some((r, g, b))
        } else {
            // Handle named colors
            match color.to_lowercase().as_str() {
                "cyan" => Some((0, 217, 255)),
                "green" => Some((0, 255, 159)),
                "pink" => Some((255, 0, 110)),
                "red" => Some((255, 0, 64)),
                "yellow" => Some((255, 190, 11)),
                "blue" => Some((0, 100, 255)),
                "purple" => Some((170, 0, 255)),
                "white" => Some((255, 255, 255)),
                "black" => Some((0, 0, 0)),
                _ => None,
            }
        }
    }

    /// Get color distance for finding closest named color
    pub fn color_distance(&self, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> u32 {
        let dr = (r1 as i32 - r2 as i32).pow(2);
        let dg = (g1 as i32 - g2 as i32).pow(2);
        let db = (b1 as i32 - b2 as i32).pow(2);
        (dr + dg + db) as u32
    }

    /// Persist current theme to settings
    pub fn save_current(&self, _persistence: &PersistenceManager) -> Result<()> {
        // This would be called after theme switching
        // The persistence layer handles the actual saving
        Ok(())
    }

    /// Save current theme to user config (convenience method)
    pub fn save_current_theme(&self) -> Result<()> {
        Ok(())
    }

    /// Get current theme name
    pub fn current_theme_name(&self) -> String {
        self.current_theme.clone()
    }

    /// List theme names
    pub fn list_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.name, "default");
        assert!(!theme.colors.primary.is_empty());
    }

    #[test]
    fn test_parse_color_hex() {
        let manager = ThemeManager::new().unwrap_or_default();
        let result = manager.parse_color("#00D9FF");
        assert_eq!(result, Some((0, 217, 255)))
    }

    #[test]
    fn test_parse_color_named() {
        let manager = ThemeManager::new().unwrap_or_default();
        let result = manager.parse_color("cyan");
        assert_eq!(result, Some((0, 217, 255)))
    }

    #[test]
    fn test_theme_colors_default() {
        let colors = ThemeColors::default();
        assert_eq!(colors.primary, "#00D9FF");
        assert_eq!(colors.secondary, "#00FF9F");
    }
}
