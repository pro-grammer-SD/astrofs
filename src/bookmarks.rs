use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use dirs::data_dir;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
    pub icon: String,
}

impl Bookmark {
    pub fn new(name: String, path: PathBuf, icon: String) -> Self {
        Self { name, path, icon }
    }
}

/// Manages bookmarks for quick access to directories
pub struct BookmarkManager {
    bookmarks: HashMap<String, Bookmark>,
    order: Vec<String>,
    file_path: PathBuf,
}

impl BookmarkManager {
    pub fn new() -> Result<Self> {
        let data_dir = data_dir().ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?;
        let astrofs_dir = data_dir.join("astrofs");
        fs::create_dir_all(&astrofs_dir)?;
        
        let file_path = astrofs_dir.join("bookmarks.json");
        
        let mut manager = Self {
            bookmarks: HashMap::new(),
            order: Vec::new(),
            file_path,
        };

        manager.load()?;
        Ok(manager)
    }

    /// Add a bookmark
    pub fn add(&mut self, name: String, path: PathBuf, icon: String) -> Result<()> {
        if self.bookmarks.contains_key(&name) {
            return Err(anyhow::anyhow!("Bookmark '{}' already exists", name));
        }

        let bookmark = Bookmark::new(name.clone(), path, icon);
        self.bookmarks.insert(name.clone(), bookmark);
        self.order.push(name);
        self.save()?;
        Ok(())
    }

    /// Remove a bookmark
    pub fn remove(&mut self, name: &str) -> Result<()> {
        if self.bookmarks.remove(name).is_some() {
            self.order.retain(|n| n != name);
            self.save()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Bookmark '{}' not found", name))
        }
    }

    /// Get a bookmark by name
    pub fn get(&self, name: &str) -> Option<&Bookmark> {
        self.bookmarks.get(name)
    }

    /// List all bookmarks in order
    pub fn list(&self) -> Vec<&Bookmark> {
        self.order
            .iter()
            .filter_map(|name| self.bookmarks.get(name))
            .collect()
    }

    /// Update a bookmark
    pub fn update(&mut self, name: String, new_path: PathBuf, new_icon: String) -> Result<()> {
        if let Some(bookmark) = self.bookmarks.get_mut(&name) {
            bookmark.path = new_path;
            bookmark.icon = new_icon;
            self.save()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Bookmark '{}' not found", name))
        }
    }

    /// Quick bookmark access by index
    pub fn get_by_index(&self, index: usize) -> Option<&Bookmark> {
        self.order.get(index).and_then(|name| self.bookmarks.get(name))
    }

    /// Get number of bookmarks
    pub fn count(&self) -> usize {
        self.bookmarks.len()
    }

    /// Check if a path is bookmarked
    pub fn is_bookmarked(&self, path: &Path) -> bool {
        self.bookmarks.values().any(|b| b.path == path)
    }

    /// Load bookmarks from file
    fn load(&mut self) -> Result<()> {
        if !self.file_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.file_path)?;
        let bookmarks: HashMap<String, Bookmark> = serde_json::from_str(&content)?;
        let order: Vec<String> = bookmarks.keys().cloned().collect();

        self.bookmarks = bookmarks;
        self.order = order;
        Ok(())
    }

    /// Save bookmarks to file
    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.bookmarks)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            bookmarks: HashMap::new(),
            order: Vec::new(),
            file_path: PathBuf::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_operations() -> Result<()> {
        let mut manager = BookmarkManager::default();
        
        manager.add("home".to_string(), PathBuf::from("/home"), "🏠".to_string())?;
        assert_eq!(manager.count(), 1);
        
        let bookmark = manager.get("home").unwrap();
        assert_eq!(bookmark.name, "home");
        
        manager.remove("home")?;
        assert_eq!(manager.count(), 0);
        
        Ok(())
    }
}
