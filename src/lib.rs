use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyModule;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::app::App;

pub mod app;
pub mod bookmarks;
pub mod config;
pub mod fileops;
pub mod files;
pub mod git;
pub mod input;
pub mod palette;
pub mod plugin;
pub mod preview;
pub mod search;
pub mod search_history;
pub mod theme;
pub mod ui;
pub mod workspace;
pub mod persistence;
pub mod plugin_api;
pub mod theme_manager;
pub mod media_preview;
pub mod media_player;
pub mod integration_helpers;

/// Python module initialization
#[pymodule]
fn pyastrofs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Core Classes
    m.add_class::<PyAstroFS>()?;
    
    // Data Model Classes
    m.add_class::<PyFileEntry>()?;
    m.add_class::<PyBookmark>()?;
    
    // Manager Classes
    m.add_class::<PyWorkspace>()?;
    m.add_class::<PyBookmarkManager>()?;
    m.add_class::<PySearchEngine>()?;
    m.add_class::<PyThemeManager>()?;
    m.add_class::<PyPluginManager>()?;
    m.add_class::<PyPlugin>()?;
    m.add_class::<PyMediaPlayer>()?;
    m.add_class::<PyMediaPreview>()?;
    
    // Enums
    m.add_class::<PyAppMode>()?;
    m.add_class::<PyInputMode>()?;
    
    // Sub-modules
    let config_module = PyModule::new(_py, "config")?;
    config_module.add_class::<PyAppConfig>()?;
    m.add_submodule(&config_module)?;
    
    let theme_module = PyModule::new(_py, "theme")?;
    theme_module.add_class::<PyTheme>()?;
    theme_module.add_class::<PyColorConfig>()?;
    theme_module.add_class::<PyStyleConfig>()?;
    theme_module.add_class::<PyThemeConfig>()?;
    m.add_submodule(&theme_module)?;
    
    let search_module = PyModule::new(_py, "search")?;
    search_module.add_class::<PySearchResult>()?;
    m.add_submodule(&search_module)?;
    
    let fileops_module = PyModule::new(_py, "fileops")?;
    fileops_module.add_class::<PyFileOperation>()?;
    m.add_submodule(&fileops_module)?;
    
    Ok(())
}

#[pyclass]
pub struct PyAstroFS {
    app: App,
}

#[pymethods]
impl PyAstroFS {
    #[new]
    fn new() -> PyResult<Self> {
        App::new()
            .map(|app| PyAstroFS { app })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn navigate(&mut self, path: String) -> PyResult<()> {
        self.app.go_to_path(&path).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn current_dir(&self) -> String {
        self.app.workspace_manager.active_workspace().current_dir.to_string_lossy().to_string()
    }

    fn list_files(&self) -> Vec<PyFileEntry> {
        self.app.workspace_manager.active_workspace().entries.iter().map(Into::into).collect()
    }

    fn move_up(&mut self) { self.app.move_up(); }
    fn move_down(&mut self) { self.app.move_down(); }

    fn enter_selected(&mut self) -> PyResult<()> {
        self.app.enter_selected().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn go_back(&mut self) -> PyResult<()> {
        self.app.go_back().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn create_file(&mut self, name: String) -> PyResult<()> {
        self.app.create_file(&name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn create_directory(&mut self, name: String) -> PyResult<()> {
        self.app.create_directory(&name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn delete_selected(&mut self) -> PyResult<()> {
        self.app.delete_selected().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn rename_selected(&mut self, new_name: String) -> PyResult<()> {
        self.app.rename_selected(&new_name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn copy_selected(&mut self) -> PyResult<()> {
        self.app.copy_selected().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn toggle_hidden(&mut self) -> PyResult<()> {
        self.app.toggle_hidden().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn start_search(&mut self) { self.app.start_search(); }

    fn search(&mut self, query: String) {
        self.app.search_query = query;
        self.app.perform_search();
    }

    fn search_results(&self) -> Vec<PyFileEntry> {
        self.app.search_engine.results.iter().map(|s| PyFileEntry::from(s)).collect()
    }

    fn navigate_to_search_result(&mut self, index: usize) -> PyResult<()> {
        self.app.navigate_to_search_result(index).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn add_bookmark(&mut self, name: String) -> PyResult<()> {
        self.app.add_bookmark(name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn goto_bookmark(&mut self, name: String) -> PyResult<()> {
        self.app.goto_bookmark(&name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn switch_theme(&mut self, theme_name: String) -> PyResult<()> {
        self.app.switch_theme(&theme_name).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn list_themes(&self) -> Vec<String> { self.app.list_available_themes() }

    fn load_plugins(&mut self) -> PyResult<()> {
        self.app.load_plugins().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn enable_plugin(&mut self, id: String) -> PyResult<()> {
        self.app.enable_plugin(&id).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn disable_plugin(&mut self, id: String) -> PyResult<()> {
        self.app.disable_plugin(&id).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn preview_media(&mut self, path: String) -> PyResult<Option<String>> {
        self.app.preview_media(&PathBuf::from(path)).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn play_media(&mut self, path: String) -> PyResult<()> {
        self.app.play_media(&PathBuf::from(path)).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn pause_media(&mut self) { self.app.pause_media(); }
    fn toggle_media_playback(&mut self) { self.app.toggle_media_playback(); }
    fn media_seek(&mut self, seconds: f32) { self.app.media_seek(seconds); }
    fn media_adjust_volume(&mut self, delta: f32) { self.app.media_adjust_volume(delta); }
    fn media_adjust_speed(&mut self, delta: f32) { self.app.media_adjust_speed(delta); }
    fn get_media_status(&self) -> String { self.app.get_media_status() }

    fn save_settings(&mut self) -> PyResult<()> {
        self.app.save_settings().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn load_user_preferences(&mut self) -> PyResult<()> {
        self.app.load_user_preferences().map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn export_settings(&mut self, path: String) -> PyResult<()> {
        self.app.export_settings(&path).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn import_settings(&mut self, path: String) -> PyResult<()> {
        self.app.import_settings(&path).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn get_current_workspace(&self) -> PyWorkspace {
        let ws = self.app.get_current_workspace();
        PyWorkspace {
            current_dir: ws.current_dir.to_string_lossy().to_string(),
            selected_index: ws.selected_index,
            show_hidden: ws.show_hidden,
            entries: ws.entries.iter().map(Into::into).collect(),
        }
    }

    fn get_bookmark_manager(&self) -> PyBookmarkManager {
        PyBookmarkManager {
            bookmarks: self.app.bookmark_manager.list().into_iter().map(|b| {
                (b.name.clone(), PyBookmark {
                    name: b.name.clone(),
                    path: b.path.to_string_lossy().to_string(),
                    icon: b.icon.clone(),
                })
            }).collect::<HashMap<_, _>>(),
        }
    }

    fn get_search_engine(&self) -> PySearchEngine {
        PySearchEngine {
            query: self.app.search_query.clone(),
            results: self.app.search_engine.results.iter().map(|s| PyFileEntry::from(s)).collect(),
        }
    }

    fn get_theme_manager(&self) -> PyThemeManager {
        PyThemeManager {
            current_theme: self.app.theme_manager.current_theme_name(),
            available_themes: self.app.list_available_themes(),
        }
    }

    fn get_plugin_manager(&self) -> PyPluginManager {
        PyPluginManager {
            plugins: self.app.api_plugin_manager.list().into_iter().map(|p| PyPlugin {
                id: p.id.clone(),
                name: p.name.clone(),
                description: p.description.clone(),
                enabled: p.enabled,
            }).collect(),
        }
    }

    fn get_media_player(&self) -> PyMediaPlayer {
        PyMediaPlayer {
            state: format!("{:?}", self.app.media_player.state),
            position: self.app.media_player.position.as_secs_f32(),
            volume: self.app.media_player.volume,
            speed: self.app.media_player.speed,
            repeat_mode: format!("{:?}", self.app.media_player.repeat_mode),
            current_index: self.app.media_player.current_index,
            playlist: self.app.media_player.playlist.clone(),
        }
    }

    fn get_media_preview(&self) -> PyMediaPreview {
        PyMediaPreview {
            last_path: self.app.media_preview.last_path().map(|p| p.to_string_lossy().to_string()),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyFileEntry {
    #[pyo3(get)] pub name: String,
    #[pyo3(get)] pub path: String,
    #[pyo3(get)] pub is_dir: bool,
    #[pyo3(get)] pub size: u64,
}

impl From<&crate::files::FileEntry> for PyFileEntry {
    fn from(e: &crate::files::FileEntry) -> Self {
        Self {
            name: e.name.clone(),
            path: e.path.to_string_lossy().to_string(),
            is_dir: e.is_dir,
            size: e.size,
        }
    }
}

impl From<&crate::search::SearchResult> for PyFileEntry {
    fn from(s: &crate::search::SearchResult) -> Self {
        Self {
            name: s.name.clone(),
            path: s.path.to_string_lossy().to_string(),
            is_dir: s.is_dir,
            size: 0,
        }
    }
}

#[pyclass]
pub struct PyWorkspace {
    #[pyo3(get)] pub current_dir: String,
    #[pyo3(get)] pub selected_index: usize,
    #[pyo3(get)] pub show_hidden: bool,
    #[pyo3(get)] pub entries: Vec<PyFileEntry>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyBookmark {
    #[pyo3(get)] pub name: String,
    #[pyo3(get)] pub path: String,
    #[pyo3(get)] pub icon: String,
}

#[pyclass]
pub struct PyBookmarkManager {
    #[pyo3(get)] pub bookmarks: HashMap<String, PyBookmark>,
}

#[pyclass]
pub struct PySearchEngine {
    #[pyo3(get)] pub query: String,
    #[pyo3(get)] pub results: Vec<PyFileEntry>,
}

#[pyclass]
pub struct PyThemeManager {
    #[pyo3(get)] pub current_theme: String,
    #[pyo3(get)] pub available_themes: Vec<String>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyPlugin {
    #[pyo3(get)] pub id: String,
    #[pyo3(get)] pub name: String,
    #[pyo3(get)] pub description: String,
    #[pyo3(get)] pub enabled: bool,
}

#[pyclass]
pub struct PyPluginManager {
    #[pyo3(get)] pub plugins: Vec<PyPlugin>,
}

#[pyclass]
pub struct PyMediaPlayer {
    #[pyo3(get)] pub state: String,
    #[pyo3(get)] pub position: f32,
    #[pyo3(get)] pub volume: f32,
    #[pyo3(get)] pub speed: f32,
    #[pyo3(get)] pub repeat_mode: String,
    #[pyo3(get)] pub current_index: usize,
    #[pyo3(get)] pub playlist: Vec<String>,
}

#[pyclass]
pub struct PyMediaPreview {
    #[pyo3(get)] pub last_path: Option<String>,
}

// ============================================================================
// ENUM WRAPPERS - AppMode and InputMode
// ============================================================================

#[pyclass]
#[derive(Clone, PartialEq)]
pub enum PyAppMode {
    Normal,
    Search,
    CommandPalette,
    Help,
    Input,
}

#[pymethods]
impl PyAppMode {
    fn __repr__(&self) -> String {
        match self {
            PyAppMode::Normal => "AppMode.Normal".to_string(),
            PyAppMode::Search => "AppMode.Search".to_string(),
            PyAppMode::CommandPalette => "AppMode.CommandPalette".to_string(),
            PyAppMode::Help => "AppMode.Help".to_string(),
            PyAppMode::Input => "AppMode.Input".to_string(),
        }
    }
}

#[pyclass]
#[derive(Clone, PartialEq)]
pub enum PyInputMode {
    CreateFile,
    CreateDirectory,
    Rename,
    GoToPath,
    AddBookmark,
}

#[pymethods]
impl PyInputMode {
    fn __repr__(&self) -> String {
        match self {
            PyInputMode::CreateFile => "InputMode.CreateFile".to_string(),
            PyInputMode::CreateDirectory => "InputMode.CreateDirectory".to_string(),
            PyInputMode::Rename => "InputMode.Rename".to_string(),
            PyInputMode::GoToPath => "InputMode.GoToPath".to_string(),
            PyInputMode::AddBookmark => "InputMode.AddBookmark".to_string(),
        }
    }
}

// ============================================================================
// CONFIG MODULE
// ============================================================================

#[pyclass]
#[derive(Clone)]
pub struct PyAppConfig {
    #[pyo3(get, set)] pub theme_name: String,
    #[pyo3(get, set)] pub show_hidden_files: bool,
    #[pyo3(get, set)] pub auto_preview: bool,
    #[pyo3(get, set)] pub max_history: usize,
}

#[pymethods]
impl PyAppConfig {
    #[new]
    fn new() -> Self {
        PyAppConfig {
            theme_name: "dracula".to_string(),
            show_hidden_files: false,
            auto_preview: true,
            max_history: 100,
        }
    }
}

// ============================================================================
// THEME MODULE
// ============================================================================

#[pyclass]
#[derive(Clone)]
pub struct PyColorConfig {
    #[pyo3(get, set)] pub r: u8,
    #[pyo3(get, set)] pub g: u8,
    #[pyo3(get, set)] pub b: u8,
}

#[pymethods]
impl PyColorConfig {
    #[new]
    fn new(r: u8, g: u8, b: u8) -> Self {
        PyColorConfig { r, g, b }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyStyleConfig {
    #[pyo3(get, set)] pub fg: Option<String>,
    #[pyo3(get, set)] pub bg: Option<String>,
    #[pyo3(get, set)] pub bold: bool,
    #[pyo3(get, set)] pub italic: bool,
    #[pyo3(get, set)] pub underline: bool,
}

#[pymethods]
impl PyStyleConfig {
    #[new]
    fn new() -> Self {
        PyStyleConfig {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyThemeConfig {
    #[pyo3(get, set)] pub name: String,
    #[pyo3(get, set)] pub background: (u8, u8, u8),
    #[pyo3(get, set)] pub foreground: (u8, u8, u8),
}

#[pymethods]
impl PyThemeConfig {
    #[new]
    fn new(name: String) -> Self {
        PyThemeConfig {
            name,
            background: (30, 30, 30),
            foreground: (200, 200, 200),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyTheme {
    #[pyo3(get)] pub name: String,
    #[pyo3(get)] pub config: PyThemeConfig,
}

#[pymethods]
impl PyTheme {
    #[new]
    fn new(name: String) -> Self {
        PyTheme {
            name: name.clone(),
            config: PyThemeConfig::new(name),
        }
    }
}

// ============================================================================
// SEARCH MODULE
// ============================================================================

#[pyclass]
#[derive(Clone)]
pub struct PySearchResult {
    #[pyo3(get)] pub name: String,
    #[pyo3(get)] pub path: String,
    #[pyo3(get)] pub is_dir: bool,
    #[pyo3(get)] pub relevance: f32,
}

#[pymethods]
impl PySearchResult {
    #[new]
    fn new(name: String, path: String, is_dir: bool, relevance: f32) -> Self {
        PySearchResult {
            name,
            path,
            is_dir,
            relevance,
        }
    }
}

// ============================================================================
// FILE OPERATIONS MODULE
// ============================================================================

#[pyclass]
#[derive(Clone)]
pub struct PyFileOperation {
    #[pyo3(get)] pub operation_type: String,
    #[pyo3(get)] pub source: String,
    #[pyo3(get)] pub destination: Option<String>,
    #[pyo3(get)] pub status: String,
}

#[pymethods]
impl PyFileOperation {
    #[new]
    fn new(operation_type: String, source: String) -> Self {
        PyFileOperation {
            operation_type,
            source,
            destination: None,
            status: "pending".to_string(),
        }
    }

    fn set_destination(&mut self, dest: String) {
        self.destination = Some(dest);
    }

    fn set_status(&mut self, status: String) {
        self.status = status;
    }
}
