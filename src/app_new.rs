use crate::bookmarks::BookmarkManager;
use crate::config::AppConfig;
use crate::fileops::FileOperation;
use crate::files::list_directory;
use crate::palette::{Command, CommandPalette};
use crate::plugin::PluginManager;
use crate::preview::{generate_preview, PreviewContent};
use crate::search::SearchEngine;
use crate::search_history::SearchHistory;
use crate::theme::Theme;
use crate::workspace::{Workspace, WorkspaceManager};
use anyhow::Result;
use open::that;
use ratatui::text::Line;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub enum AppMode {
    Normal,
    Search,
    CommandPalette,
    Help,
    Input(InputMode),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputMode {
    CreateFile,
    CreateDirectory,
    Rename,
    GoToPath,
    AddBookmark,
}

pub struct App {
    // Core
    pub workspace_manager: WorkspaceManager,
    pub config: AppConfig,
    pub theme: Theme,

    // State
    pub mode: AppMode,
    pub running: bool,
    pub viewport_height: usize,
    pub viewport_width: usize,

    // UI State
    pub message: Option<String>,
    pub error: Option<String>,
    pub input_buffer: String,
    pub input_mode: Option<InputMode>,

    // Search
    pub search_engine: SearchEngine,
    pub search_history: SearchHistory,
    pub search_query: String,

    // Command palette
    pub command_palette: CommandPalette,
    pub command_search_index: usize,

    // Managers
    pub bookmark_manager: BookmarkManager,
    pub plugin_manager: PluginManager,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut config = AppConfig::load().unwrap_or_default();
        config.validate();

        let start_dir = PathBuf::from(&config.default_directory);
        let workspace_manager = WorkspaceManager::new(start_dir);

        let bookmark_manager = BookmarkManager::new().unwrap_or_default();
        let mut plugin_manager = PluginManager::default();

        // Load plugins silently, don't fail if plugins directory doesn't exist
        let _ = plugin_manager.load_plugins();

        let search_history = SearchHistory::load().unwrap_or_default();

        let app = Self {
            workspace_manager,
            config,
            theme: Theme::default(),
            mode: AppMode::Normal,
            running: true,
            viewport_height: 20,
            viewport_width: 80,
            message: None,
            error: None,
            input_buffer: String::new(),
            input_mode: None,
            search_engine: SearchEngine::new(),
            search_history,
            search_query: String::new(),
            command_palette: CommandPalette::new(),
            command_search_index: 0,
            bookmark_manager,
            plugin_manager,
        };

        Ok(app)
    }

    // ========== Navigation ==========
    pub fn move_up(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        workspace.move_up();
        self.update_preview();
    }

    pub fn move_down(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        workspace.move_down();
        self.update_preview();
    }

    pub fn page_up(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        workspace.page_up(self.viewport_height);
        self.update_preview();
    }

    pub fn page_down(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        workspace.page_down(self.viewport_height);
        self.update_preview();
    }

    pub fn go_home(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        workspace.go_home();
        self.update_preview();
    }

    pub fn go_end(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        workspace.go_end();
        self.update_preview();
    }

    pub fn enter_selected(&mut self) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace_mut();
        
        if let Some(entry) = workspace.get_selected_entry().cloned() {
            if entry.is_dir {
                workspace.current_dir = entry.path.clone();
                workspace.selected_index = 0;
                workspace.scroll_offset = 0;
                self.refresh_workspace()?;
            } else {
                // Open with default application
                let _ = that(&entry.path);
                self.message = Some(format!("Opened: {}", entry.name));
            }
        }
        Ok(())
    }

    pub fn go_back(&mut self) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace_mut();
        
        if let Some(parent) = workspace.current_dir.parent() {
            workspace.current_dir = parent.to_path_buf();
            workspace.selected_index = 0;
            workspace.scroll_offset = 0;
            self.refresh_workspace()?;
        }
        Ok(())
    }

    pub fn go_to_path(&mut self, path: &str) -> Result<()> {
        let path = PathBuf::from(path);
        if path.exists() {
            let workspace = self.workspace_manager.active_workspace_mut();
            workspace.current_dir = path;
            workspace.selected_index = 0;
            workspace.scroll_offset = 0;
            self.refresh_workspace()?;
            self.message = Some("Navigated to path".to_string());
        } else {
            self.error = Some("Path does not exist".to_string());
        }
        Ok(())
    }

    // ========== File Operations ==========
    pub fn copy_selected(&mut self) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace_mut();
        
        if let Some(entry) = workspace.get_selected_entry().cloned() {
            // For now, set a message. Full clipboard support would need a clipboard library
            self.message = Some(format!("Copied: {} (paste with Ctrl+V)", entry.name));
        }
        Ok(())
    }

    pub fn delete_selected(&mut self) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace_mut();
        
        if let Some(entry) = workspace.get_selected_entry().cloned() {
            match FileOperation::delete(&entry.path) {
                Ok(_) => {
                    self.message = Some(format!("Deleted: {}", entry.name));
                    self.refresh_workspace()?;
                }
                Err(e) => {
                    self.error = Some(format!("Delete failed: {}", e));
                }
            }
        }
        Ok(())
    }

    pub fn rename_selected(&mut self, new_name: &str) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace_mut();
        
        if let Some(entry) = workspace.get_selected_entry().cloned() {
            match FileOperation::rename(&entry.path, new_name) {
                Ok(_) => {
                    self.message = Some(format!("Renamed to: {}", new_name));
                    self.refresh_workspace()?;
                }
                Err(e) => {
                    self.error = Some(format!("Rename failed: {}", e));
                }
            }
        }
        Ok(())
    }

    pub fn create_file(&mut self, name: &str) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace();
        let file_path = workspace.current_dir.join(name);

        match FileOperation::create_file(&file_path) {
            Ok(_) => {
                self.message = Some(format!("Created file: {}", name));
                self.refresh_workspace()?;
            }
            Err(e) => {
                self.error = Some(format!("Create file failed: {}", e));
            }
        }
        Ok(())
    }

    pub fn create_directory(&mut self, name: &str) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace();
        let dir_path = workspace.current_dir.join(name);

        match FileOperation::create_directory(&dir_path) {
            Ok(_) => {
                self.message = Some(format!("Created directory: {}", name));
                self.refresh_workspace()?;
            }
            Err(e) => {
                self.error = Some(format!("Create directory failed: {}", e));
            }
        }
        Ok(())
    }

    // ========== Preview ==========
    pub fn update_preview(&mut self) {
        let workspace = self.workspace_manager.active_workspace_mut();
        
        if let Some(entry) = workspace.get_selected_entry().cloned() {
            workspace.preview = generate_preview(&entry.path, 200);
        } else {
            workspace.preview = PreviewContent {
                lines: vec![Line::from("No file selected")],
                is_binary: false,
                preview_type: crate::preview::PreviewType::Text,
            };
        }
    }

    pub fn refresh_workspace(&mut self) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace_mut();
        let current_dir = workspace.current_dir.clone();
        let show_hidden = workspace.show_hidden;

        workspace.entries = list_directory(&current_dir, show_hidden)?;

        if workspace.selected_index >= workspace.entries.len() && !workspace.entries.is_empty() {
            workspace.selected_index = workspace.entries.len() - 1;
        }

        self.update_preview();
        Ok(())
    }

    pub fn toggle_hidden(&mut self) -> Result<()> {
        let show_hidden = {
            let workspace = self.workspace_manager.active_workspace_mut();
            workspace.show_hidden = !workspace.show_hidden;
            workspace.show_hidden
        };
        self.refresh_workspace()?;
        
        self.message = Some(format!(
            "Hidden files: {}",
            if show_hidden { "shown" } else { "hidden" }
        ));
        Ok(())
    }

    // ========== Search ==========
    pub fn start_search(&mut self) {
        self.mode = AppMode::Search;
        self.search_query.clear();
        self.message = Some("Search mode: Type to search (ESC to cancel, Enter to navigate)".to_string());
    }

    pub fn cancel_search(&mut self) {
        self.mode = AppMode::Normal;
        self.search_query.clear();
        self.search_engine.clear();
        self.message = None;
    }

    pub fn perform_search(&mut self) {
        if !self.search_query.is_empty() {
            let workspace = self.workspace_manager.active_workspace();
            self.search_engine.search_current_dir(
                &workspace.current_dir,
                &self.search_query,
                self.config.max_search_results,
            );
            self.message = Some(format!("Found {} results", self.search_engine.results.len()));
        } else {
            self.search_engine.clear();
        }
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.perform_search();
    }

    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
        self.perform_search();
    }

    pub fn navigate_to_search_result(&mut self, index: usize) -> Result<()> {
        if let Some(result) = self.search_engine.results.get(index) {
            let workspace = self.workspace_manager.active_workspace_mut();
            
            if result.is_dir {
                workspace.current_dir = result.path.clone();
            } else if let Some(parent) = result.path.parent() {
                workspace.current_dir = parent.to_path_buf();
            }

            workspace.selected_index = 0;
            workspace.scroll_offset = 0;
            self.refresh_workspace()?;
            self.search_history.add(self.search_query.clone());
            self.cancel_search();
        }
        Ok(())
    }

    // ========== Workspaces/Tabs ==========
    pub fn new_workspace(&mut self) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace();
        let new_path = workspace.current_dir.clone();
        
        self.workspace_manager.create_workspace(new_path);
        self.message = Some("Created new workspace".to_string());
        Ok(())
    }

    pub fn close_workspace(&mut self) -> Result<()> {
        let id = self.workspace_manager.active_id();
        if self.workspace_manager.close_workspace(id) {
            self.message = Some("Closed workspace".to_string());
        } else {
            self.error = Some("Cannot close last workspace".to_string());
        }
        Ok(())
    }

    pub fn next_workspace(&mut self) {
        self.workspace_manager.next_workspace();
        self.message = Some("Switched to next workspace".to_string());
    }

    pub fn prev_workspace(&mut self) {
        self.workspace_manager.prev_workspace();
        self.message = Some("Switched to previous workspace".to_string());
    }

    pub fn rename_workspace(&mut self, name: String) {
        self.workspace_manager.rename_active_workspace(name.clone());
        self.message = Some(format!("Renamed workspace to: {}", name));
    }

    // ========== Bookmarks ==========
    pub fn add_bookmark(&mut self, name: String) -> Result<()> {
        let workspace = self.workspace_manager.active_workspace();
        let path = workspace.current_dir.clone();

        self.bookmark_manager.add(name.clone(), path, "📌".to_string())?;
        self.message = Some(format!("Added bookmark: {}", name));
        Ok(())
    }

    pub fn goto_bookmark(&mut self, name: &str) -> Result<()> {
        if let Some(bookmark) = self.bookmark_manager.get(name) {
            let workspace = self.workspace_manager.active_workspace_mut();
            workspace.current_dir = bookmark.path.clone();
            workspace.selected_index = 0;
            workspace.scroll_offset = 0;
            self.refresh_workspace()?;
            self.message = Some(format!("Navigated to bookmark: {}", name));
        } else {
            self.error = Some("Bookmark not found".to_string());
        }
        Ok(())
    }

    // ========== Command Palette ==========
    pub fn start_command_palette(&mut self) {
        self.mode = AppMode::CommandPalette;
        self.command_palette.clear_filter();
        self.command_search_index = 0;
        self.message = Some("Command palette (type to filter, ESC to cancel)".to_string());
    }

    pub fn execute_command(&mut self, cmd: &Command) -> Result<()> {
        match cmd {
            Command::Copy => self.copy_selected()?,
            Command::Delete => self.delete_selected()?,
            Command::CreateFile => {
                self.mode = AppMode::Input(InputMode::CreateFile);
                self.input_buffer.clear();
            }
            Command::CreateDirectory => {
                self.mode = AppMode::Input(InputMode::CreateDirectory);
                self.input_buffer.clear();
            }
            Command::ParentDirectory => self.go_back()?,
            Command::Home => {
                if let Ok(home_dir) = std::env::var("HOME") {
                    self.go_to_path(&home_dir)?;
                }
            }
            Command::Root => self.go_to_path("/")?,
            Command::GoToPath => {
                self.mode = AppMode::Input(InputMode::GoToPath);
                self.input_buffer.clear();
            }
            Command::Search => self.start_search(),
            Command::ToggleHidden => self.toggle_hidden()?,
            Command::NewWorkspace => self.new_workspace()?,
            Command::NextWorkspace => self.next_workspace(),
            Command::PrevWorkspace => self.prev_workspace(),
            Command::AddBookmark => {
                self.mode = AppMode::Input(InputMode::AddBookmark);
                self.input_buffer.clear();
            }
            Command::OpenWithDefault => {
                let workspace = self.workspace_manager.active_workspace();
                if let Some(entry) = workspace.get_selected_entry() {
                    let _ = that(&entry.path);
                }
            }
            Command::ShowHelp => {
                self.mode = AppMode::Help;
            }
            Command::Quit => self.running = false,
            _ => {}
        }

        if !matches!(self.mode, AppMode::Input(_)) {
            self.mode = AppMode::Normal;
        }
        
        Ok(())
    }

    // ========== Utilities ==========
    pub fn quit(&mut self) {
        let _ = self.bookmark_manager.save();
        let _ = self.search_history.save();
        let _ = self.config.save();
        self.running = false;
    }

    pub fn set_viewport(&mut self, width: usize, height: usize) {
        self.viewport_width = width;
        self.viewport_height = height;
    }

    pub fn get_current_workspace(&self) -> &Workspace {
        self.workspace_manager.active_workspace()
    }
}
