use anyhow::Result;
use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchHistory {
    queries: VecDeque<String>,
    max_size: usize,
}

impl SearchHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            queries: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    /// Add a search query to history
    pub fn add(&mut self, query: String) {
        if query.is_empty() {
            return;
        }

        // Remove if already exists (to avoid duplicates at different positions)
        self.queries.retain(|q| q != &query);

        // Add to front
        self.queries.push_front(query);

        // Maintain max size
        if self.queries.len() > self.max_size {
            self.queries.pop_back();
        }
    }

    /// Get all queries in order
    pub fn all(&self) -> Vec<&str> {
        self.queries.iter().map(|q| q.as_str()).collect()
    }

    /// Get query by index (0 is most recent)
    pub fn get(&self, index: usize) -> Option<&str> {
        self.queries.get(index).map(|q| q.as_str())
    }

    /// Get next query from current index
    pub fn next(&self, current_index: usize) -> Option<&str> {
        if current_index == 0 {
            None
        } else {
            self.queries.get(current_index - 1).map(|q| q.as_str())
        }
    }

    /// Get previous query from current index
    pub fn prev(&self, current_index: usize) -> Option<&str> {
        self.queries.get(current_index + 1).map(|q| q.as_str())
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.queries.clear();
    }

    /// Get count of queries
    pub fn len(&self) -> usize {
        self.queries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.queries.is_empty()
    }

    /// Save to file
    pub fn save(&self) -> Result<()> {
        let data_dir = data_dir().ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?;
        let astrofs_dir = data_dir.join("astrofs");
        fs::create_dir_all(&astrofs_dir)?;

        let file_path = astrofs_dir.join("search_history.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(file_path, content)?;

        Ok(())
    }

    /// Load from file
    pub fn load() -> Result<Self> {
        let data_dir = data_dir().ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?;
        let file_path = data_dir.join("astrofs/search_history.json");

        if !file_path.exists() {
            return Ok(Self::new(50));
        }

        let content = fs::read_to_string(file_path)?;
        let history = serde_json::from_str(&content)?;

        Ok(history)
    }
}

impl Default for SearchHistory {
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_history() {
        let mut history = SearchHistory::new(3);

        history.add("rust".to_string());
        history.add("python".to_string());
        history.add("go".to_string());

        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some("go"));
        assert_eq!(history.get(2), Some("rust"));

        // Adding duplicate should move it to front
        history.add("rust".to_string());
        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some("rust"));

        // Adding new one when full should remove oldest
        history.add("zig".to_string());
        assert_eq!(history.len(), 3);
        assert_eq!(history.get(0), Some("zig"));
    }
}
