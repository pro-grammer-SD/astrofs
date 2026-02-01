// Enhanced Plugin API - fully-featured plugin system
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

/// Core plugin trait with complete API access
pub trait Plugin: Send + Sync {
    /// Plugin metadata
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn author(&self) -> &str;

    /// Lifecycle hooks
    fn on_load(&mut self) -> Result<()> {
        Ok(())
    }
    fn on_unload(&mut self) -> Result<()> {
        Ok(())
    }
    fn on_enable(&mut self) -> Result<()> {
        Ok(())
    }
    fn on_disable(&mut self) -> Result<()> {
        Ok(())
    }

    /// File operations hooks
    fn on_file_created(&self, _path: &PathBuf) -> Result<()> {
        Ok(())
    }
    fn on_file_deleted(&self, _path: &PathBuf) -> Result<()> {
        Ok(())
    }
    fn on_file_renamed(&self, _old_path: &PathBuf, _new_path: &PathBuf) -> Result<()> {
        Ok(())
    }
    fn on_file_copied(&self, _src: &PathBuf, _dest: &PathBuf) -> Result<()> {
        Ok(())
    }
    fn on_file_moved(&self, _src: &PathBuf, _dest: &PathBuf) -> Result<()> {
        Ok(())
    }

    /// Command execution
    fn execute_command(&self, _command: &str, _args: Vec<String>) -> Result<String> {
        Ok(String::new())
    }

    /// Get plugin commands for palette
    fn get_commands(&self) -> Vec<PluginCommand> {
        Vec::new()
    }

    /// Theme manipulation
    fn on_theme_changed(&self, _theme_name: &str) -> Result<()> {
        Ok(())
    }
    fn customize_theme(&self, _theme: &mut PluginTheme) -> Result<()> {
        Ok(())
    }

    /// Keybinding extensions
    fn get_keybindings(&self) -> HashMap<String, PluginAction> {
        HashMap::new()
    }

    /// UI rendering hooks
    fn render_custom_ui(&self, _ctx: &mut RenderContext) -> Result<()> {
        Ok(())
    }

    /// Search enhancement
    fn filter_search_results(&self, _query: &str, _results: &mut Vec<String>) -> Result<()> {
        Ok(())
    }

    /// Settings persistence for plugin
    fn save_data(&self, _key: &str, _value: serde_json::Value) -> Result<()> {
        Ok(())
    }
    fn load_data(&self, _key: &str) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    /// Performance metrics
    fn on_idle(&self) -> Result<()> {
        Ok(())
    }
    fn get_stats(&self) -> PluginStats {
        PluginStats::default()
    }
}

/// A single command exposed by a plugin
#[derive(Clone, Debug)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub shortcuts: Vec<String>,
    pub category: String,
    pub args: Vec<CommandArg>,
}

#[derive(Clone, Debug)]
pub struct CommandArg {
    pub name: String,
    pub arg_type: String,
    pub required: bool,
    pub description: String,
}

/// Plugin action bound to a key
#[derive(Clone, Debug)]
pub enum PluginAction {
    Command(String),
    Callback(String),
    Custom(String),
}

/// Plugin theme customization
#[derive(Clone, Debug)]
pub struct PluginTheme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub borders: HashMap<String, String>,
    pub emojis: HashMap<String, String>,
}

/// Rendering context for custom UI
pub struct RenderContext {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub buffer: Vec<u8>,
}

/// Plugin statistics
#[derive(Clone, Debug, Default)]
pub struct PluginStats {
    pub load_time_ms: u64,
    pub memory_usage_bytes: u64,
    pub function_calls: u64,
    pub errors: u64,
}

/// Plugin metadata for registry
#[derive(Clone, Debug)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub path: PathBuf,
    pub enabled: bool,
    pub permissions: Vec<PluginPermission>,
}

/// Granular permissions for plugins
#[derive(Clone, Debug, PartialEq)]
pub enum PluginPermission {
    /// File system access
    ReadFiles,
    WriteFiles,
    DeleteFiles,
    ExecuteFiles,

    /// UI access
    RenderUI,
    InterceptInput,
    ModifyTheme,

    /// System access
    ExecuteCommands,
    NetworkAccess,
    AccessSettings,

    /// Data access
    AccessBookmarks,
    AccessHistory,
    AccessClipboard,

    /// Plugin management
    LoadPlugins,
    UnloadPlugins,
    ModifyOtherPlugins,
}

/// Plugin registry and manager
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    metadata: HashMap<String, PluginMetadata>,
    plugin_dir: PathBuf,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self {
            plugins: HashMap::new(),
            metadata: HashMap::new(),
            plugin_dir: PathBuf::from("./plugins"),
        }
    }
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: HashMap::new(),
            metadata: HashMap::new(),
            plugin_dir,
        }
    }

    /// Register a plugin
    pub fn register(&mut self, id: String, plugin: Box<dyn Plugin>, meta: PluginMetadata) {
        self.metadata.insert(id.clone(), meta);
        self.plugins.insert(id, plugin);
    }

    /// Get plugin by ID
    pub fn get(&self, id: &str) -> Option<&(dyn Plugin + 'static)> {
        self.plugins.get(id).map(|p| p.as_ref())
    }

    /// Get mutable plugin by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut (dyn Plugin + 'static)> {
        self.plugins.get_mut(id).map(|p| p.as_mut())
    }

    /// List all plugins
    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.metadata.values().collect()
    }

    /// Enable plugin
    pub fn enable(&mut self, id: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.on_enable()?;
        }
        if let Some(meta) = self.metadata.get_mut(id) {
            meta.enabled = true;
        }
        Ok(())
    }

    /// Disable plugin
    pub fn disable(&mut self, id: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.on_disable()?;
        }
        if let Some(meta) = self.metadata.get_mut(id) {
            meta.enabled = false;
        }
        Ok(())
    }

    /// Load all plugins from directory
    pub fn load_all(&mut self) -> Result<()> {
        if !self.plugin_dir.exists() {
            std::fs::create_dir_all(&self.plugin_dir)?;
        }
        Ok(())
    }

    /// Unload all plugins
    pub fn unload_all(&mut self) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.on_unload()?;
        }
        self.plugins.clear();
        Ok(())
    }

    /// Get plugin count
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Check if plugin has permission
    pub fn check_permission(&self, id: &str, permission: &PluginPermission) -> bool {
        if let Some(meta) = self.metadata.get(id) {
            meta.permissions.contains(permission)
        } else {
            false
        }
    }

    /// Call hooks on all plugins
    pub fn call_file_created(&mut self, path: &PathBuf) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.on_file_created(path)?;
        }
        Ok(())
    }

    pub fn call_file_deleted(&mut self, path: &PathBuf) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.on_file_deleted(path)?;
        }
        Ok(())
    }

    pub fn call_file_renamed(&mut self, old: &PathBuf, new: &PathBuf) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.on_file_renamed(old, new)?;
        }
        Ok(())
    }

    pub fn call_theme_changed(&mut self, theme_name: &str) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.on_theme_changed(theme_name)?;
        }
        Ok(())
    }

    pub fn call_idle(&mut self) -> Result<()> {
        for plugin in self.plugins.values_mut() {
            plugin.on_idle()?;
        }
        Ok(())
    }

    /// Get all commands from all plugins
    pub fn get_all_commands(&self) -> Vec<(String, PluginCommand)> {
        let mut commands = Vec::new();
        for (id, plugin) in &self.plugins {
            for cmd in plugin.get_commands() {
                commands.push((id.clone(), cmd));
            }
        }
        commands
    }

    /// Get all keybindings from all plugins
    pub fn get_all_keybindings(&self) -> HashMap<String, (String, PluginAction)> {
        let mut bindings = HashMap::new();
        for (id, plugin) in &self.plugins {
            for (key, action) in plugin.get_keybindings() {
                bindings.insert(key, (id.clone(), action));
            }
        }
        bindings
    }
}

/// Built-in example plugins

/// Example: File statistics plugin
pub struct FileStatsPlugin {
    total_files_processed: u64,
}

impl FileStatsPlugin {
    pub fn new() -> Self {
        Self {
            total_files_processed: 0,
        }
    }
    
    pub fn total_files_processed(&self) -> u64 {
        self.total_files_processed
    }
}

impl Plugin for FileStatsPlugin {
    fn name(&self) -> &str {
        "FileStats"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn description(&self) -> &str {
        "Tracks file operation statistics"
    }
    fn author(&self) -> &str {
        "AstroFS Team"
    }

    fn on_file_created(&self, _path: &PathBuf) -> Result<()> {
        Ok(())
    }

    fn get_commands(&self) -> Vec<PluginCommand> {
        vec![PluginCommand {
            name: "show-stats".to_string(),
            description: "Show file operation statistics".to_string(),
            shortcuts: vec!["Ctrl+S".to_string()],
            category: "Statistics".to_string(),
            args: Vec::new(),
        }]
    }
}

/// Example: Theme customizer plugin
pub struct ThemeCustomizer;

impl Plugin for ThemeCustomizer {
    fn name(&self) -> &str {
        "ThemeCustomizer"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn description(&self) -> &str {
        "Interactive theme customization"
    }
    fn author(&self) -> &str {
        "AstroFS Team"
    }

    fn get_commands(&self) -> Vec<PluginCommand> {
        vec![PluginCommand {
            name: "customize-theme".to_string(),
            description: "Interactively customize current theme".to_string(),
            shortcuts: vec!["Ctrl+T".to_string()],
            category: "Theme".to_string(),
            args: Vec::new(),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_stats() {
        let stats = PluginStats {
            load_time_ms: 100,
            memory_usage_bytes: 1024,
            ..Default::default()
        };
        assert_eq!(stats.load_time_ms, 100);
    }
}
