use std::path::PathBuf;
use crate::files::FileEntry;
use crate::preview::PreviewContent;
use ratatui::text::Line;

/// Represents a single tab/pane in the file explorer
#[derive(Clone, Debug)]
pub struct Workspace {
    pub id: usize,
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub preview: PreviewContent,
    pub show_hidden: bool,
    pub title: String,
}

impl Workspace {
    pub fn new(id: usize, path: PathBuf) -> Self {
        Self {
            id,
            current_dir: path.clone(),
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            preview: PreviewContent {
                lines: vec![Line::from("No file selected")],
                is_binary: false,
                preview_type: crate::preview::PreviewType::Text,
            },
            show_hidden: false,
            title: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Workspace")
                .to_string(),
        }
    }

    pub fn rename(&mut self, name: String) {
        self.title = name;
    }

    pub fn get_selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.adjust_scroll();
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
            self.adjust_scroll();
        }
    }

    pub fn page_up(&mut self, viewport_height: usize) {
        self.selected_index = self.selected_index.saturating_sub(viewport_height);
        self.adjust_scroll();
    }

    pub fn page_down(&mut self, viewport_height: usize) {
        self.selected_index = (self.selected_index + viewport_height)
            .min(self.entries.len().saturating_sub(1));
        self.adjust_scroll();
    }

    pub fn go_home(&mut self) {
        self.selected_index = 0;
        self.adjust_scroll();
    }

    pub fn go_end(&mut self) {
        self.selected_index = self.entries.len().saturating_sub(1);
        self.adjust_scroll();
    }

    fn adjust_scroll(&mut self) {
        let viewport_height = 20; // Default, should be passed or stored
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + viewport_height {
            self.scroll_offset = self.selected_index - viewport_height + 1;
        }
    }
}

/// Manages multiple workspaces/tabs
pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
    active_workspace_id: usize,
    next_id: usize,
}

impl WorkspaceManager {
    pub fn new(initial_path: PathBuf) -> Self {
        let mut manager = Self {
            workspaces: Vec::new(),
            active_workspace_id: 0,
            next_id: 1,
        };

        let workspace = Workspace::new(0, initial_path);
        manager.workspaces.push(workspace);
        manager
    }

    /// Create a new workspace
    pub fn create_workspace(&mut self, path: PathBuf) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let workspace = Workspace::new(id, path);
        self.workspaces.push(workspace);
        self.active_workspace_id = id;
        
        id
    }

    /// Close a workspace by ID
    pub fn close_workspace(&mut self, id: usize) -> bool {
        if self.workspaces.len() == 1 {
            return false; // Don't close the last workspace
        }

        if let Some(pos) = self.workspaces.iter().position(|w| w.id == id) {
            self.workspaces.remove(pos);
            
            // If we closed the active workspace, switch to another
            if self.active_workspace_id == id {
                self.active_workspace_id = self.workspaces[0].id;
            }
            return true;
        }
        false
    }

    /// Switch to a workspace by ID
    pub fn switch_workspace(&mut self, id: usize) -> bool {
        if self.workspaces.iter().any(|w| w.id == id) {
            self.active_workspace_id = id;
            true
        } else {
            false
        }
    }

    /// Get the active workspace mutably
    pub fn active_workspace_mut(&mut self) -> &mut Workspace {
        self.workspaces
            .iter_mut()
            .find(|w| w.id == self.active_workspace_id)
            .expect("Active workspace should always exist")
    }

    /// Get the active workspace immutably
    pub fn active_workspace(&self) -> &Workspace {
        self.workspaces
            .iter()
            .find(|w| w.id == self.active_workspace_id)
            .expect("Active workspace should always exist")
    }

    /// Get all workspaces
    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }

    /// Get the ID of the active workspace
    pub fn active_id(&self) -> usize {
        self.active_workspace_id
    }

    /// Get total number of workspaces
    pub fn count(&self) -> usize {
        self.workspaces.len()
    }

    /// Get the index of the active workspace in the list
    pub fn active_index(&self) -> usize {
        self.workspaces
            .iter()
            .position(|w| w.id == self.active_workspace_id)
            .unwrap_or(0)
    }

    /// Switch to next workspace
    pub fn next_workspace(&mut self) {
        let current_idx = self.active_index();
        let next_idx = (current_idx + 1) % self.workspaces.len();
        self.active_workspace_id = self.workspaces[next_idx].id;
    }

    /// Switch to previous workspace
    pub fn prev_workspace(&mut self) {
        let current_idx = self.active_index();
        let prev_idx = if current_idx == 0 {
            self.workspaces.len() - 1
        } else {
            current_idx - 1
        };
        self.active_workspace_id = self.workspaces[prev_idx].id;
    }

    /// Rename active workspace
    pub fn rename_active_workspace(&mut self, name: String) {
        self.active_workspace_mut().rename(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_manager() {
        let mut manager = WorkspaceManager::new(PathBuf::from("/tmp"));
        assert_eq!(manager.count(), 1);

        let id = manager.create_workspace(PathBuf::from("/home"));
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.active_id(), id);

        manager.prev_workspace();
        assert_eq!(manager.active_id(), 0);

        manager.next_workspace();
        assert_eq!(manager.active_id(), id);
    }
}
