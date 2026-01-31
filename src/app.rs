use crate::files::{FileEntry, list_directory};
use crate::git::GitInfo;
use crate::preview::{generate_preview, PreviewContent};
use crate::search::SearchEngine;
use std::path::PathBuf;
use std::env;
use ratatui::text::Line;

pub enum AppMode {
    Normal,
    Search,
    Help,
}

pub struct App {
    pub current_dir: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub preview: PreviewContent,
    pub show_hidden: bool,
    pub mode: AppMode,
    pub search_query: String,
    pub search_engine: SearchEngine,
    pub git_info: GitInfo,
    pub message: Option<String>,
    pub error: Option<String>,
    pub running: bool,
    pub viewport_height: usize,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let current_dir = env::current_dir()?;
        let mut app = Self {
            current_dir: current_dir.clone(),
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            preview: PreviewContent {
                lines: vec![Line::from("No file selected")],
                is_binary: false,
                preview_type: crate::preview::PreviewType::Text,
            },
            show_hidden: false,
            mode: AppMode::Normal,
            search_query: String::new(),
            search_engine: SearchEngine::new(),
            git_info: GitInfo::from_path(&current_dir),
            message: None,
            error: None,
            running: true,
            viewport_height: 20,
        };
        app.refresh_directory()?;
        Ok(app)
    }

    pub fn refresh_directory(&mut self) -> anyhow::Result<()> {
        self.entries = list_directory(&self.current_dir, self.show_hidden)?;
        self.git_info = GitInfo::from_path(&self.current_dir);
        
        if self.selected_index >= self.entries.len() && !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
        }
        
        self.update_preview();
        Ok(())
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.adjust_scroll();
            self.update_preview();
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
            self.adjust_scroll();
            self.update_preview();
        }
    }

    pub fn page_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(self.viewport_height);
        self.adjust_scroll();
        self.update_preview();
    }

    pub fn page_down(&mut self) {
        self.selected_index = (self.selected_index + self.viewport_height)
            .min(self.entries.len().saturating_sub(1));
        self.adjust_scroll();
        self.update_preview();
    }

    pub fn go_home(&mut self) {
        self.selected_index = 0;
        self.adjust_scroll();
        self.update_preview();
    }

    pub fn go_end(&mut self) {
        self.selected_index = self.entries.len().saturating_sub(1);
        self.adjust_scroll();
        self.update_preview();
    }

    fn adjust_scroll(&mut self) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + self.viewport_height {
            self.scroll_offset = self.selected_index - self.viewport_height + 1;
        }
    }

    pub fn enter_selected(&mut self) -> anyhow::Result<()> {
        if let Some(entry) = self.entries.get(self.selected_index) {
            if entry.is_dir {
                self.current_dir = entry.path.clone();
                self.selected_index = 0;
                self.scroll_offset = 0;
                self.refresh_directory()?;
            } else {
                self.message = Some(format!("Selected: {}", entry.name));
            }
        }
        Ok(())
    }

    pub fn go_back(&mut self) -> anyhow::Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected_index = 0;
            self.scroll_offset = 0;
            self.refresh_directory()?;
        }
        Ok(())
    }

    pub fn toggle_hidden(&mut self) -> anyhow::Result<()> {
        self.show_hidden = !self.show_hidden;
        self.refresh_directory()?;
        self.message = Some(format!(
            "Hidden files: {}",
            if self.show_hidden { "shown" } else { "hidden" }
        ));
        Ok(())
    }

    pub fn update_preview(&mut self) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            self.preview = generate_preview(&entry.path, 200);
        } else {
            self.preview = PreviewContent {
                lines: vec![Line::from("No file selected")],
                is_binary: false,
                preview_type: crate::preview::PreviewType::Text,
            };
        }
    }

    pub fn start_search(&mut self) {
        self.mode = AppMode::Search;
        self.search_query.clear();
        self.message = Some("Search mode: Type to search (ESC to cancel)".to_string());
    }

    pub fn cancel_search(&mut self) {
        self.mode = AppMode::Normal;
        self.search_query.clear();
        self.search_engine.clear();
        self.message = None;
    }

    pub fn perform_search(&mut self) {
        if !self.search_query.is_empty() {
            self.search_engine.search_current_dir(&self.current_dir, &self.search_query, 100);
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

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn set_viewport_height(&mut self, height: usize) {
        self.viewport_height = height;
    }
}
