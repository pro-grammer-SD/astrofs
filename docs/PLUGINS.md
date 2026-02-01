# 🧩 AstroFS Plugin Development Guide

## Table of Contents
1. [Quick Start](#quick-start)
2. [Plugin Architecture](#plugin-architecture)
3. [API Reference](#api-reference)
4. [Examples](#examples)
5. [Best Practices](#best-practices)
6. [Distribution](#distribution)

## Quick Start

### 1. Create Plugin File
Create a new Rust file in the `plugins/` folder:

```bash
mkdir -p plugins
cat > plugins/my_first_plugin.rs << 'EOF'
use astrofs::plugin_api::*;
use anyhow::Result;

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "My first AstroFS plugin"
    }

    fn author(&self) -> &str {
        "Your Name"
    }

    fn on_load(&mut self) -> Result<()> {
        println!("✅ Plugin loaded!");
        Ok(())
    }
}
EOF
```

### 2. Register Plugin
In your app startup code:

```rust
let mut plugin_mgr = PluginManager::new(plugin_dir);

let plugin = Box::new(MyPlugin);
let meta = PluginMetadata {
    id: "my-plugin".to_string(),
    name: "MyPlugin".to_string(),
    version: "1.0.0".to_string(),
    description: "My first plugin".to_string(),
    author: "Your Name".to_string(),
    path: plugin_path,
    enabled: true,
    permissions: vec![
        PluginPermission::ReadFiles,
        PluginPermission::AccessSettings,
    ],
};

plugin_mgr.register("my-plugin".to_string(), plugin, meta);
```

### 3. Use Plugin
```rust
// Call lifecycle hooks
plugin_mgr.enable("my-plugin")?;
plugin_mgr.disable("my-plugin")?;

// Get plugin commands
let commands = plugin_mgr.get_all_commands();

// Call file operation hooks
plugin_mgr.call_file_created(&file_path)?;
```

## Plugin Architecture

### Plugin Trait
Every plugin implements the `Plugin` trait which provides:

```rust
pub trait Plugin: Send + Sync {
    // Metadata (required)
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn author(&self) -> &str;

    // Lifecycle hooks
    fn on_load(&mut self) -> Result<()>;
    fn on_unload(&mut self) -> Result<()>;
    fn on_enable(&mut self) -> Result<()>;
    fn on_disable(&mut self) -> Result<()>;

    // File operation hooks
    fn on_file_created(&self, path: &PathBuf) -> Result<()>;
    fn on_file_deleted(&self, path: &PathBuf) -> Result<()>;
    fn on_file_renamed(&self, old: &PathBuf, new: &PathBuf) -> Result<()>;
    fn on_file_copied(&self, src: &PathBuf, dest: &PathBuf) -> Result<()>;
    fn on_file_moved(&self, src: &PathBuf, dest: &PathBuf) -> Result<()>;

    // Command execution
    fn execute_command(&self, cmd: &str, args: Vec<String>) -> Result<String>;
    fn get_commands(&self) -> Vec<PluginCommand>;

    // Theme manipulation
    fn on_theme_changed(&self, theme: &str) -> Result<()>;
    fn customize_theme(&self, theme: &mut PluginTheme) -> Result<()>;

    // Keybinding extensions
    fn get_keybindings(&self) -> HashMap<String, PluginAction>;

    // UI rendering
    fn render_custom_ui(&self, ctx: &mut RenderContext) -> Result<()>;

    // Search enhancement
    fn filter_search_results(&self, query: &str, results: &mut Vec<String>) -> Result<()>;

    // Data persistence
    fn save_data(&self, key: &str, value: serde_json::Value) -> Result<()>;
    fn load_data(&self, key: &str) -> Result<Option<serde_json::Value>>;

    // Performance
    fn on_idle(&self) -> Result<()>;
    fn get_stats(&self) -> PluginStats;
}
```

### Plugin Manager
The `PluginManager` handles:
- Plugin registration and lifecycle
- Hook invocation
- Command palette integration
- Permission checking

```rust
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    metadata: HashMap<String, PluginMetadata>,
    plugin_dir: PathBuf,
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf) -> Self;
    pub fn register(&mut self, id: String, plugin: Box<dyn Plugin>, meta: PluginMetadata);
    pub fn get(&self, id: &str) -> Option<&(dyn Plugin + 'static)>;
    pub fn enable(&mut self, id: &str) -> Result<()>;
    pub fn disable(&mut self, id: &str) -> Result<()>;
    pub fn load_all(&mut self) -> Result<()>;
    pub fn unload_all(&mut self) -> Result<()>;
    pub fn check_permission(&self, id: &str, permission: &PluginPermission) -> bool;
    // ... more methods
}
```

## API Reference

### PluginCommand
Register commands callable from the command palette:

```rust
pub struct PluginCommand {
    pub name: String,           // Command name (e.g., "show-stats")
    pub description: String,    // Help text
    pub shortcuts: Vec<String>, // Keyboard shortcuts (e.g., ["Ctrl+S"])
    pub category: String,       // Category (e.g., "Statistics")
    pub args: Vec<CommandArg>,  // Arguments
}
```

### CommandArg
Define command arguments:

```rust
pub struct CommandArg {
    pub name: String,           // Argument name
    pub arg_type: String,       // Type: "string", "integer", "boolean"
    pub required: bool,
    pub description: String,
}
```

### PluginAction
Define keybinding actions:

```rust
pub enum PluginAction {
    Command(String),        // Call a command
    Callback(String),       // Invoke callback
    Custom(String),         // Custom action
}
```

### PluginPermission
Request fine-grained permissions:

```rust
pub enum PluginPermission {
    // File system
    ReadFiles,
    WriteFiles,
    DeleteFiles,
    ExecuteFiles,

    // UI
    RenderUI,
    InterceptInput,
    ModifyTheme,

    // System
    ExecuteCommands,
    NetworkAccess,
    AccessSettings,

    // Data
    AccessBookmarks,
    AccessHistory,
    AccessClipboard,

    // Plugin management
    LoadPlugins,
    UnloadPlugins,
    ModifyOtherPlugins,
}
```

### PluginStats
Track plugin performance:

```rust
pub struct PluginStats {
    pub load_time_ms: u64,       // Load time
    pub memory_usage_bytes: u64, // Memory usage
    pub function_calls: u64,     // Function call count
    pub errors: u64,             // Error count
}
```

## Examples

### Example 1: File Stats Plugin

```rust
use astrofs::plugin_api::*;
use std::path::PathBuf;
use anyhow::Result;

pub struct FileStatsPlugin {
    total_files: u64,
    total_size: u64,
}

impl Plugin for FileStatsPlugin {
    fn name(&self) -> &str { "FileStats" }
    fn version(&self) -> &str { "1.0.0" }
    fn description(&self) -> &str { "Tracks file statistics" }
    fn author(&self) -> &str { "You" }

    fn on_file_created(&self, path: &PathBuf) -> Result<()> {
        if let Ok(meta) = std::fs::metadata(path) {
            println!("📊 New file: {} ({} bytes)", 
                path.display(), 
                meta.len());
        }
        Ok(())
    }

    fn get_commands(&self) -> Vec<PluginCommand> {
        vec![PluginCommand {
            name: "show-stats".to_string(),
            description: "Show file statistics".to_string(),
            shortcuts: vec!["Ctrl+S".to_string()],
            category: "Statistics".to_string(),
            args: Vec::new(),
        }]
    }

    fn execute_command(&self, cmd: &str, _args: Vec<String>) -> Result<String> {
        match cmd {
            "show-stats" => Ok(format!(
                "📊 Files: {}, Total Size: {} MB",
                self.total_files,
                self.total_size / 1024 / 1024
            )),
            _ => Ok("Unknown command".to_string()),
        }
    }
}
```

### Example 2: Search Enhancement Plugin

```rust
use astrofs::plugin_api::*;
use anyhow::Result;

pub struct SmartSearchPlugin;

impl Plugin for SmartSearchPlugin {
    fn name(&self) -> &str { "SmartSearch" }
    fn version(&self) -> &str { "1.0.0" }
    fn description(&self) -> &str { "Advanced search filtering" }
    fn author(&self) -> &str { "You" }

    fn filter_search_results(&self, query: &str, results: &mut Vec<String>) -> Result<()> {
        // Filter by extension if query starts with @
        if query.starts_with("@") {
            let ext = &query[1..];
            results.retain(|r| r.ends_with(ext));
        }
        // Filter by size if query starts with #
        else if query.starts_with("#") {
            // Custom size filtering logic
        }
        Ok(())
    }

    fn get_commands(&self) -> Vec<PluginCommand> {
        vec![PluginCommand {
            name: "search-help".to_string(),
            description: "Show search syntax help".to_string(),
            shortcuts: vec!["Ctrl+Shift+?".to_string()],
            category: "Help".to_string(),
            args: Vec::new(),
        }]
    }
}
```

### Example 3: Theme Modifier Plugin

```rust
use astrofs::plugin_api::*;
use anyhow::Result;

pub struct ThemeModifierPlugin;

impl Plugin for ThemeModifierPlugin {
    fn name(&self) -> &str { "ThemeModifier" }
    fn version(&self) -> &str { "1.0.0" }
    fn description(&self) -> &str { "Modify theme on the fly" }
    fn author(&self) -> &str { "You" }

    fn on_theme_changed(&self, theme_name: &str) -> Result<()> {
        println!("🎨 Theme changed to: {}", theme_name);
        Ok(())
    }

    fn customize_theme(&self, theme: &mut PluginTheme) -> Result<()> {
        // Add custom colors
        theme.colors.insert("custom_ui".to_string(), "#FF00FF".to_string());
        Ok(())
    }

    fn get_keybindings(&self) -> HashMap<String, PluginAction> {
        let mut map = HashMap::new();
        map.insert(
            "Ctrl+Alt+C".to_string(),
            PluginAction::Command("customize-theme".to_string()),
        );
        map
    }
}
```

## Best Practices

### 1. **Error Handling**
Always use `Result<T>` and provide descriptive errors:

```rust
fn on_file_created(&self, path: &PathBuf) -> Result<()> {
    let meta = std::fs::metadata(path)
        .map_err(|e| anyhow::anyhow!("Failed to read metadata: {}", e))?;
    // ... use metadata
    Ok(())
}
```

### 2. **Performance**
- Don't block in hooks
- Use async operations when possible
- Cache expensive computations
- Report memory usage via `get_stats()`

### 3. **Security**
- Check permissions before accessing resources
- Validate command arguments
- Escape output for CLI safety
- Don't execute untrusted code

### 4. **Documentation**
```rust
/// Load the plugin and initialize resources
/// 
/// This hook is called when the plugin is first loaded.
/// Initialize any expensive resources here.
/// 
/// # Errors
/// Returns an error if initialization fails
fn on_load(&mut self) -> Result<()> {
    // Implementation
}
```

### 5. **Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = MyPlugin;
        assert_eq!(plugin.name(), "MyPlugin");
        assert_eq!(plugin.version(), "1.0.0");
    }

    #[test]
    fn test_command_execution() {
        let plugin = MyPlugin;
        let result = plugin.execute_command("test", vec![]);
        assert!(result.is_ok());
    }
}
```

## Distribution

### 1. Package Your Plugin
```bash
# Create a plugin package
mkdir my-plugin
cp plugins/my_plugin.rs my-plugin/
cat > my-plugin/Cargo.toml << 'EOF'
[package]
name = "my-plugin"
version = "1.0.0"
edition = "2021"

[dependencies]
astrofs = "0.1"
anyhow = "1.0"
EOF
```

### 2. Share on Plugin Registry
```bash
# Publish to AstroFS plugin repository
astrofs plugin publish my-plugin
```

### 3. Installation
Users can install your plugin:
```bash
astrofs plugin install my-plugin
# OR
astrofs plugin install https://github.com/user/my-plugin
```

## Troubleshooting

### Plugin Not Loading
```bash
# Check plugin file location
ls ~/.config/astrofs/plugins/

# Check permissions
chmod 644 ~/.config/astrofs/plugins/my-plugin.rs

# Check for compilation errors
cargo build
```

### Hooks Not Firing
- Verify plugin is enabled: `astrofs plugin list`
- Check permissions in metadata
- Verify hook implementation exists

### Commands Not Appearing
```bash
# List registered commands
astrofs command list

# Verify get_commands() returns commands
# Check for command name conflicts
```

---

**Next Steps:**
- [Theme Development](THEMES.md)
- [API Documentation](docs/api.md)
- [Plugin Examples](plugins/)
- [Community Plugins](COMMUNITY_PLUGINS.md)
