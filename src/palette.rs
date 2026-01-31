use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Command {
    // File operations
    Copy,
    Move,
    Delete,
    Rename,
    CreateFile,
    CreateDirectory,
    
    // Navigation
    ParentDirectory,
    Home,
    Root,
    GoToPath,
    
    // Search & Filter
    Search,
    ClearSearch,
    ToggleHidden,
    
    // Workspaces
    NewWorkspace,
    CloseWorkspace,
    NextWorkspace,
    PrevWorkspace,
    
    // Bookmarks
    AddBookmark,
    GoToBookmark,
    ShowBookmarks,
    
    // Preview
    TogglePreview,
    RefreshPreview,
    
    // System
    OpenWithDefault,
    ShowHelp,
    ShowSettings,
    Quit,
    
    // Custom
    Custom(String),
}

impl Command {
    pub fn to_string(&self) -> String {
        match self {
            Command::Copy => "Copy".to_string(),
            Command::Move => "Move".to_string(),
            Command::Delete => "Delete".to_string(),
            Command::Rename => "Rename".to_string(),
            Command::CreateFile => "Create File".to_string(),
            Command::CreateDirectory => "Create Directory".to_string(),
            Command::ParentDirectory => "Go to Parent".to_string(),
            Command::Home => "Go Home".to_string(),
            Command::Root => "Go to Root".to_string(),
            Command::GoToPath => "Go to Path".to_string(),
            Command::Search => "Search".to_string(),
            Command::ClearSearch => "Clear Search".to_string(),
            Command::ToggleHidden => "Toggle Hidden".to_string(),
            Command::NewWorkspace => "New Workspace".to_string(),
            Command::CloseWorkspace => "Close Workspace".to_string(),
            Command::NextWorkspace => "Next Workspace".to_string(),
            Command::PrevWorkspace => "Previous Workspace".to_string(),
            Command::AddBookmark => "Add Bookmark".to_string(),
            Command::GoToBookmark => "Go to Bookmark".to_string(),
            Command::ShowBookmarks => "Show Bookmarks".to_string(),
            Command::TogglePreview => "Toggle Preview".to_string(),
            Command::RefreshPreview => "Refresh Preview".to_string(),
            Command::OpenWithDefault => "Open with Default App".to_string(),
            Command::ShowHelp => "Help".to_string(),
            Command::ShowSettings => "Settings".to_string(),
            Command::Quit => "Quit".to_string(),
            Command::Custom(s) => s.clone(),
        }
    }
}

pub struct CommandPalette {
    commands: HashMap<String, Command>,
    visible_commands: Vec<(String, Command)>,
    filter: String,
}

impl CommandPalette {
    pub fn new() -> Self {
        let mut commands = HashMap::new();

        // Register all built-in commands
        let cmd_list = vec![
            ("copy", Command::Copy),
            ("move", Command::Move),
            ("delete", Command::Delete),
            ("rename", Command::Rename),
            ("create-file", Command::CreateFile),
            ("create-dir", Command::CreateDirectory),
            ("parent", Command::ParentDirectory),
            ("home", Command::Home),
            ("root", Command::Root),
            ("goto", Command::GoToPath),
            ("search", Command::Search),
            ("clear-search", Command::ClearSearch),
            ("hidden", Command::ToggleHidden),
            ("new-workspace", Command::NewWorkspace),
            ("close-workspace", Command::CloseWorkspace),
            ("next-ws", Command::NextWorkspace),
            ("prev-ws", Command::PrevWorkspace),
            ("bookmark-add", Command::AddBookmark),
            ("bookmark-go", Command::GoToBookmark),
            ("bookmarks", Command::ShowBookmarks),
            ("toggle-preview", Command::TogglePreview),
            ("refresh-preview", Command::RefreshPreview),
            ("open", Command::OpenWithDefault),
            ("help", Command::ShowHelp),
            ("settings", Command::ShowSettings),
            ("quit", Command::Quit),
        ];

        for (key, cmd) in cmd_list {
            commands.insert(key.to_string(), cmd);
        }

        Self {
            commands,
            visible_commands: Vec::new(),
            filter: String::new(),
        }
    }

    /// Update filter and rebuild visible commands
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter.to_lowercase();
        self.rebuild_visible();
    }

    /// Add character to filter
    pub fn add_char(&mut self, c: char) {
        self.filter.push(c);
        self.rebuild_visible();
    }

    /// Remove last character from filter
    pub fn remove_char(&mut self) {
        self.filter.pop();
        self.rebuild_visible();
    }

    /// Clear filter
    pub fn clear_filter(&mut self) {
        self.filter.clear();
        self.rebuild_visible();
    }

    /// Rebuild visible commands based on current filter
    fn rebuild_visible(&mut self) {
        self.visible_commands.clear();

        for (key, cmd) in &self.commands {
            let cmd_str = cmd.to_string().to_lowercase();
            
            if self.filter.is_empty() || 
               key.contains(&self.filter) || 
               cmd_str.contains(&self.filter) {
                self.visible_commands.push((key.clone(), cmd.clone()));
            }
        }

        // Sort by relevance: exact match first, then starts with, then contains
        self.visible_commands.sort_by(|a, b| {
            let a_key = &a.0.to_lowercase();
            let b_key = &b.0.to_lowercase();
            
            let a_exact = a_key == &self.filter;
            let b_exact = b_key == &self.filter;
            
            if a_exact != b_exact {
                return if a_exact { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
            }
            
            let a_starts = a_key.starts_with(&self.filter);
            let b_starts = b_key.starts_with(&self.filter);
            
            if a_starts != b_starts {
                return if a_starts { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
            }
            
            a_key.cmp(b_key)
        });
    }

    /// Get list of visible commands
    pub fn visible(&self) -> &[(String, Command)] {
        &self.visible_commands
    }

    /// Get command by index
    pub fn get_by_index(&self, index: usize) -> Option<&Command> {
        self.visible_commands.get(index).map(|(_, cmd)| cmd)
    }

    /// Get command by name
    pub fn get(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }

    /// Register a custom command
    pub fn register(&mut self, key: String, cmd: Command) {
        self.commands.insert(key, cmd);
        self.rebuild_visible();
    }

    /// Get current filter text
    pub fn filter(&self) -> &str {
        &self.filter
    }

    /// Get number of visible commands
    pub fn visible_count(&self) -> usize {
        self.visible_commands.len()
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_palette() {
        let mut palette = CommandPalette::new();
        
        palette.set_filter("copy".to_string());
        assert_eq!(palette.visible_count(), 1);
        
        palette.clear_filter();
        assert!(palette.visible_count() > 1);
        
        palette.add_char('d');
        palette.add_char('e');
        palette.add_char('l');
        let cmd = palette.get_by_index(0);
        assert!(cmd.is_some());
    }
}
