use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Plugin trait for dynamic plugin loading
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, args: Vec<String>) -> Result<String>;
}

/// Plugin metadata
#[derive(Clone, Debug)]
pub struct PluginMetadata {
    pub name: String,
    pub path: PathBuf,
    pub version: String,
    pub description: String,
    pub enabled: bool,
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: HashMap<String, PluginMetadata>,
    plugin_dir: PathBuf,
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_dir,
        }
    }

    /// Load plugins from plugin directory
    pub fn load_plugins(&mut self) -> Result<()> {
        if !self.plugin_dir.exists() {
            std::fs::create_dir_all(&self.plugin_dir)?;
            return Ok(());
        }

        for entry in std::fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            #[cfg(target_os = "windows")]
            let is_plugin = path.extension().map(|e| e == "dll").unwrap_or(false);

            #[cfg(not(target_os = "windows"))]
            let is_plugin = path.extension().map(|e| e == "so" || e == "dylib").unwrap_or(false);

            if is_plugin {
                if let Ok(metadata) = self.load_plugin_metadata(&path) {
                    self.plugins.insert(metadata.name.clone(), metadata);
                }
            }
        }

        Ok(())
    }

    /// Load plugin metadata from path
    fn load_plugin_metadata(&self, path: &Path) -> Result<PluginMetadata> {
        let name = path
            .file_stem()
            .ok_or_else(|| anyhow!("Invalid plugin path"))?
            .to_string_lossy()
            .to_string();

        Ok(PluginMetadata {
            name,
            path: path.to_path_buf(),
            version: "0.1.0".to_string(),
            description: "Dynamically loaded plugin".to_string(),
            enabled: true,
        })
    }

    /// Register a plugin manually
    pub fn register(&mut self, metadata: PluginMetadata) -> Result<()> {
        if self.plugins.contains_key(&metadata.name) {
            return Err(anyhow!("Plugin '{}' already registered", metadata.name));
        }
        self.plugins.insert(metadata.name.clone(), metadata);
        Ok(())
    }

    /// Get plugin metadata
    pub fn get(&self, name: &str) -> Option<&PluginMetadata> {
        self.plugins.get(name)
    }

    /// List all plugins
    pub fn list(&self) -> Vec<&PluginMetadata> {
        self.plugins.values().collect()
    }

    /// Enable plugin
    pub fn enable(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(name) {
            plugin.enabled = true;
            Ok(())
        } else {
            Err(anyhow!("Plugin '{}' not found", name))
        }
    }

    /// Disable plugin
    pub fn disable(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(name) {
            plugin.enabled = false;
            Ok(())
        } else {
            Err(anyhow!("Plugin '{}' not found", name))
        }
    }

    /// Get plugin directory
    pub fn plugin_dir(&self) -> &Path {
        &self.plugin_dir
    }

    /// Get count of plugins
    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        let plugin_dir = dirs::config_dir()
            .map(|d| d.join("astrofs/plugins"))
            .unwrap_or_else(|| PathBuf::from("./plugins"));

        Self::new(plugin_dir)
    }
}

/// Built-in plugin examples
pub mod builtin {
    use super::*;

    /// File info plugin
    pub struct FileInfoPlugin;

    impl Plugin for FileInfoPlugin {
        fn name(&self) -> &str {
            "file-info"
        }

        fn version(&self) -> &str {
            "0.1.0"
        }

        fn description(&self) -> &str {
            "Display detailed file information"
        }

        fn execute(&self, args: Vec<String>) -> Result<String> {
            if args.is_empty() {
                return Err(anyhow!("No file path provided"));
            }

            let path = Path::new(&args[0]);
            let metadata = std::fs::metadata(path)?;

            let info = format!(
                "File: {}\nSize: {} bytes\nModified: {:?}",
                path.display(),
                metadata.len(),
                metadata.modified()?
            );

            Ok(info)
        }
    }

    /// Directory statistics plugin
    pub struct DirStatsPlugin;

    impl Plugin for DirStatsPlugin {
        fn name(&self) -> &str {
            "dir-stats"
        }

        fn version(&self) -> &str {
            "0.1.0"
        }

        fn description(&self) -> &str {
            "Show directory statistics"
        }

        fn execute(&self, args: Vec<String>) -> Result<String> {
            if args.is_empty() {
                return Err(anyhow!("No directory path provided"));
            }

            let path = Path::new(&args[0]);
            if !path.is_dir() {
                return Err(anyhow!("Not a directory"));
            }

            let mut file_count = 0;
            let mut dir_count = 0;
            let mut total_size = 0u64;

            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_dir() {
                    dir_count += 1;
                } else {
                    file_count += 1;
                    if let Ok(metadata) = std::fs::metadata(entry.path()) {
                        total_size += metadata.len();
                    }
                }
            }

            let stats = format!(
                "Directories: {}\nFiles: {}\nTotal Size: {} bytes",
                dir_count, file_count, total_size
            );

            Ok(stats)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager() -> Result<()> {
        let manager = PluginManager::new(PathBuf::from("./plugins"));
        assert_eq!(manager.count(), 0);
        Ok(())
    }
}
