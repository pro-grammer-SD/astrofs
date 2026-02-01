"""
AstroFS Python bindings - Type stubs

Complete type hints for the AstroFS Rust library exposed to Python.
Provides full IDE support and type checking for all AstroFS functionality.
"""

from typing import Dict, List, Optional, Tuple
from pathlib import Path

class PyFileEntry:
    """Represents a file or directory entry"""
    name: str
    """File or directory name"""
    
    path: str
    """Full path to the file or directory"""
    
    is_dir: bool
    """True if this is a directory"""
    
    size: int
    """Size in bytes"""

class PyBookmark:
    """Represents a saved bookmark to a directory"""
    name: str
    """Bookmark name"""
    
    path: str
    """Path to the bookmarked directory"""
    
    icon: str
    """Emoji or icon for the bookmark"""

class PyWorkspace:
    """Represents a workspace with directory navigation state"""
    current_dir: str
    """Current working directory"""
    
    selected_index: int
    """Index of currently selected entry (0-based)"""
    
    show_hidden: bool
    """Whether hidden files are visible"""
    
    entries: List[PyFileEntry]
    """Entries in the current directory"""

class PySearchEngine:
    """Provides search functionality"""
    query: str
    """Current search query"""
    
    results: List[PyFileEntry]
    """Search results"""

class PyThemeManager:
    """Manages application themes"""
    current_theme: str
    """Name of currently active theme"""
    
    available_themes: List[str]
    """List of available theme names"""

class PyPlugin:
    """Represents a loaded plugin"""
    id: str
    """Unique plugin identifier"""
    
    name: str
    """Human-readable plugin name"""
    
    description: str
    """Plugin description"""
    
    enabled: bool
    """Whether plugin is enabled"""

class PyPluginManager:
    """Manages plugins"""
    plugins: List[PyPlugin]
    """List of loaded plugins"""

class PyMediaPlayer:
    """Audio/video playback control"""
    state: str
    """Current playback state (Playing, Paused, Stopped)"""
    
    position: float
    """Current playback position in seconds"""
    
    volume: float
    """Volume level (0.0 to 1.0)"""
    
    speed: float
    """Playback speed (0.5 to 2.0)"""
    
    repeat_mode: str
    """Repeat mode (None, One, All)"""
    
    current_index: int
    """Current index in playlist"""
    
    playlist: List[str]
    """Current playlist"""

class PyMediaPreview:
    """Media preview functionality"""
    last_path: Optional[str]
    """Path to last previewed media file"""

class PyBookmarkManager:
    """Manages saved bookmarks"""
    bookmarks: Dict[str, PyBookmark]
    """Mapping of bookmark names to bookmark objects"""

class PyAstroFS:
    """Main AstroFS file manager class
    
    This is the primary interface for interacting with AstroFS from Python.
    It provides file system navigation, search, theme management, plugin control,
    and media playback functionality.
    
    Example:
        >>> fs = PyAstroFS()
        >>> fs.navigate("/home/user")
        >>> files = fs.list_files()
        >>> for file in files:
        ...     print(f"{file.name} ({file.size} bytes)")
    """
    
    def __init__(self) -> None:
        """Initialize AstroFS application"""
        ...
    
    # Navigation
    def navigate(self, path: str) -> None:
        """Navigate to a directory
        
        Args:
            path: Directory path to navigate to
            
        Raises:
            ValueError: If path doesn't exist or is not accessible
        """
        ...
    
    def current_dir(self) -> str:
        """Get current working directory
        
        Returns:
            Current directory path
        """
        ...
    
    def list_files(self) -> List[PyFileEntry]:
        """List files in current directory
        
        Returns:
            List of file entries in current directory
        """
        ...
    
    def move_up(self) -> None:
        """Move selection up in file list"""
        ...
    
    def move_down(self) -> None:
        """Move selection down in file list"""
        ...
    
    def enter_selected(self) -> None:
        """Enter selected directory or open file
        
        Raises:
            ValueError: If selection is invalid
        """
        ...
    
    def go_back(self) -> None:
        """Go back to parent directory
        
        Raises:
            ValueError: If already at root or navigation fails
        """
        ...
    
    # File Operations
    def create_file(self, name: str) -> None:
        """Create a new file in current directory
        
        Args:
            name: Name of file to create
            
        Raises:
            ValueError: If file already exists or creation fails
        """
        ...
    
    def create_directory(self, name: str) -> None:
        """Create a new directory in current directory
        
        Args:
            name: Name of directory to create
            
        Raises:
            ValueError: If directory already exists or creation fails
        """
        ...
    
    def delete_selected(self) -> None:
        """Delete selected file or directory
        
        Raises:
            ValueError: If deletion fails
        """
        ...
    
    def rename_selected(self, new_name: str) -> None:
        """Rename selected file or directory
        
        Args:
            new_name: New name for file/directory
            
        Raises:
            ValueError: If rename fails
        """
        ...
    
    def copy_selected(self) -> None:
        """Copy selected file or directory
        
        Raises:
            ValueError: If copy fails
        """
        ...
    
    def toggle_hidden(self) -> None:
        """Toggle visibility of hidden files"""
        ...
    
    # Search
    def start_search(self) -> None:
        """Start search mode"""
        ...
    
    def search(self, query: str) -> None:
        """Perform search with given query
        
        Args:
            query: Search query string
        """
        ...
    
    def search_results(self) -> List[PyFileEntry]:
        """Get current search results
        
        Returns:
            List of search result entries
        """
        ...
    
    def navigate_to_search_result(self, index: int) -> None:
        """Navigate to a search result
        
        Args:
            index: Index of search result
            
        Raises:
            ValueError: If index is invalid
        """
        ...
    
    # Bookmarks
    def add_bookmark(self, name: str) -> None:
        """Add bookmark to current directory
        
        Args:
            name: Name for the bookmark
            
        Raises:
            ValueError: If bookmark already exists
        """
        ...
    
    def goto_bookmark(self, name: str) -> None:
        """Navigate to a saved bookmark
        
        Args:
            name: Name of bookmark
            
        Raises:
            ValueError: If bookmark doesn't exist
        """
        ...
    
    # Themes
    def switch_theme(self, theme_name: str) -> None:
        """Switch to a different theme
        
        Args:
            theme_name: Name of theme to switch to
            
        Raises:
            ValueError: If theme doesn't exist
        """
        ...
    
    def list_themes(self) -> List[str]:
        """List all available themes
        
        Returns:
            List of theme names
        """
        ...
    
    # Plugins
    def load_plugins(self) -> None:
        """Load all plugins from plugin directory
        
        Raises:
            ValueError: If plugin loading fails
        """
        ...
    
    def enable_plugin(self, id: str) -> None:
        """Enable a plugin
        
        Args:
            id: Plugin ID to enable
            
        Raises:
            ValueError: If plugin doesn't exist
        """
        ...
    
    def disable_plugin(self, id: str) -> None:
        """Disable a plugin
        
        Args:
            id: Plugin ID to disable
            
        Raises:
            ValueError: If plugin doesn't exist
        """
        ...
    
    # Media
    def preview_media(self, path: str) -> Optional[str]:
        """Preview media file (image, video, audio)
        
        Args:
            path: Path to media file
            
        Returns:
            Preview content or None if not available
            
        Raises:
            ValueError: If file doesn't exist or isn't media
        """
        ...
    
    def play_media(self, path: str) -> None:
        """Start playing media file
        
        Args:
            path: Path to media file
            
        Raises:
            ValueError: If file doesn't exist or isn't playable
        """
        ...
    
    def pause_media(self) -> None:
        """Pause media playback"""
        ...
    
    def toggle_media_playback(self) -> None:
        """Toggle between play and pause"""
        ...
    
    def media_seek(self, seconds: float) -> None:
        """Seek to position in media
        
        Args:
            seconds: Position in seconds
        """
        ...
    
    def media_adjust_volume(self, delta: float) -> None:
        """Adjust media volume
        
        Args:
            delta: Volume change (-1.0 to 1.0)
        """
        ...
    
    def media_adjust_speed(self, delta: float) -> None:
        """Adjust playback speed
        
        Args:
            delta: Speed adjustment (-1.0 to 1.0)
        """
        ...
    
    def get_media_status(self) -> str:
        """Get current media playback status
        
        Returns:
            Status string
        """
        ...
    
    # Settings
    def save_settings(self) -> None:
        """Save application settings
        
        Raises:
            ValueError: If saving fails
        """
        ...
    
    def load_user_preferences(self) -> None:
        """Load user preferences
        
        Raises:
            ValueError: If loading fails
        """
        ...
    
    def export_settings(self, path: str) -> None:
        """Export settings to file
        
        Args:
            path: Path to export file
            
        Raises:
            ValueError: If export fails
        """
        ...
    
    def import_settings(self, path: str) -> None:
        """Import settings from file
        
        Args:
            path: Path to settings file
            
        Raises:
            ValueError: If import fails
        """
        ...
    
    # State Getters
    def get_current_workspace(self) -> PyWorkspace:
        """Get current workspace state
        
        Returns:
            Current workspace information
        """
        ...
    
    def get_bookmark_manager(self) -> PyBookmarkManager:
        """Get bookmark manager state
        
        Returns:
            Bookmark manager with all bookmarks
        """
        ...
    
    def get_search_engine(self) -> PySearchEngine:
        """Get search engine state
        
        Returns:
            Search engine with current results
        """
        ...
    
    def get_theme_manager(self) -> PyThemeManager:
        """Get theme manager state
        
        Returns:
            Theme manager with available themes
        """
        ...
    
    def get_plugin_manager(self) -> PyPluginManager:
        """Get plugin manager state
        
        Returns:
            Plugin manager with loaded plugins
        """
        ...
    
    def get_media_player(self) -> PyMediaPlayer:
        """Get media player state
        
        Returns:
            Media player with current state
        """
        ...
    
    def get_media_preview(self) -> PyMediaPreview:
        """Get media preview state
        
        Returns:
            Media preview with last path
        """
        ...
