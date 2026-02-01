# PyAstroFS Python Bindings

Comprehensive Python bindings for **AstroFS**, the advanced file system navigator written in Rust.

## Overview

PyAstroFS provides a production-ready interface to access all AstroFS functionality from Python, including:

- 🗂️ **File System Navigation** - Browse, search, and manage files
- 🔍 **Advanced Search** - Pattern-based file searching
- 🔖 **Bookmarks** - Save and quickly navigate to favorite directories
- 🎨 **Theme System** - Customize the interface appearance
- 🔌 **Plugin Support** - Extend functionality with plugins
- 🎵 **Media Playback** - Control media playback
- ⚙️ **Settings Management** - Save and restore user preferences

Built with [maturin](https://github.com/PyO3/maturin) and [PyO3](https://github.com/PyO3/pyo3) for seamless Rust-Python interoperability.

## Installation

### From PyPI (Recommended)

```bash
pip install pyastrofs
```

### From Source

```bash
# Clone the repository
git clone https://github.com/pro-grammer-SD/astrofs.git
cd astrofs

# Build and install
pip install -e .

# Or use the build scripts:
# On Linux/macOS:
./scripts/build_bindings.sh --release

# On Windows:
.\scripts\build_bindings.ps1 -Release
```

## Supported Python Versions

- Python 3.10
- Python 3.11
- Python 3.12
- Python 3.13
- Python 3.14

## Supported Platforms

- ✅ Linux (x86_64)
- ✅ macOS (Intel & Apple Silicon)
- ✅ Windows (x86_64)

## Quick Start

### Basic File Navigation

```python
from pyastrofs import PyAstroFS

# Create an instance
fs = PyAstroFS()

# Navigate to a directory
fs.navigate("/home/user/documents")
print(f"Current directory: {fs.current_dir()}")

# List files
files = fs.list_files()
for file in files:
    if file.is_dir:
        print(f"📁 {file.name}/")
    else:
        print(f"📄 {file.name} ({file.size} bytes)")
```

### Search Files

```python
# Search for Python files
fs.search("*.py")

results = fs.search_results()
print(f"Found {len(results)} Python files")

for result in results[:5]:
    print(f"  • {result.path}")
```

### Manage Bookmarks

```python
# Add a bookmark
fs.add_bookmark("projects")

# List bookmarks
bm_manager = fs.get_bookmark_manager()
for name, bookmark in bm_manager.bookmarks.items():
    print(f"{bookmark.icon} {name} → {bookmark.path}")

# Navigate to a bookmark
fs.goto_bookmark("projects")
```

### Switch Themes

```python
# List available themes
themes = fs.list_themes()
print(f"Available themes: {', '.join(themes)}")

# Switch theme
fs.switch_theme("dark")

# Get theme manager
tm = fs.get_theme_manager()
print(f"Current theme: {tm.current_theme}")
```

### Manage Plugins

```python
# Load plugins
fs.load_plugins()

# Get plugin manager
pm = fs.get_plugin_manager()
print(f"Loaded {len(pm.plugins)} plugins")

for plugin in pm.plugins:
    status = "✓" if plugin.enabled else "✗"
    print(f"{status} {plugin.name} - {plugin.description}")

# Enable/disable plugins
fs.enable_plugin(plugin_id)
fs.disable_plugin(plugin_id)
```

### Media Playback

```python
# Get media player state
mp = fs.get_media_player()
print(f"State: {mp.state}")
print(f"Volume: {mp.volume * 100:.0f}%")

# Play media
fs.play_media("/path/to/audio.mp3")

# Control playback
fs.pause_media()
fs.toggle_media_playback()
fs.media_seek(30.5)  # Seek to 30.5 seconds
fs.media_adjust_volume(0.1)  # Increase volume by 10%
```

## Complete API Reference

### Main Class: `PyAstroFS`

#### Navigation Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `current_dir()` | Get current directory path | None | `str` |
| `navigate(path)` | Navigate to directory | `path: str` | None |
| `move_up()` | Move to parent directory | None | None |
| `move_down()` | Move to selected entry | None | None |
| `go_to_path(path)` | Go to specific path | `path: str` | None |
| `refresh()` | Refresh current view | None | None |

#### File Operations

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `list_files(show_hidden=None)` | List files in current directory | `show_hidden: Optional[bool]` | `List[FileEntry]` |
| `get_selected_entry()` | Get currently selected entry | None | `Optional[FileEntry]` |
| `create_file(name)` | Create new file | `name: str` | None |
| `create_directory(name)` | Create new directory | `name: str` | None |
| `delete_selected()` | Delete selected entry | None | None |
| `delete_file(path)` | Delete file at path | `path: str` | None |
| `rename_selected(new_name)` | Rename selected entry | `new_name: str` | None |
| `duplicate_selected()` | Duplicate selected entry | None | None |
| `toggle_hidden()` | Toggle hidden file visibility | None | None |

#### Search Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `search(pattern)` | Search for files matching pattern | `pattern: str` | None |
| `search_results()` | Get search results | None | `List[FileEntry]` |
| `navigate_to_search_result(index)` | Navigate to search result | `index: int` | None |
| `clear_search()` | Clear search results | None | None |

#### Bookmark Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `add_bookmark(name)` | Add bookmark | `name: str` | None |
| `remove_bookmark(name)` | Remove bookmark | `name: str` | None |
| `goto_bookmark(name)` | Navigate to bookmark | `name: str` | None |
| `get_bookmark_manager()` | Get bookmark manager | None | `BookmarkManager` |

#### Theme Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `list_themes()` | List available themes | None | `List[str]` |
| `switch_theme(theme)` | Switch to theme | `theme: str` | None |
| `get_theme_manager()` | Get theme manager | None | `ThemeManager` |

#### Plugin Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `load_plugins()` | Load all plugins | None | None |
| `enable_plugin(id)` | Enable plugin | `id: str` | None |
| `disable_plugin(id)` | Disable plugin | `id: str` | None |
| `get_plugin_manager()` | Get plugin manager | None | `PluginManager` |

#### Media Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `play_media(path)` | Play media file | `path: str` | None |
| `pause_media()` | Pause playback | None | None |
| `toggle_media_playback()` | Toggle play/pause | None | None |
| `media_seek(seconds)` | Seek to position | `seconds: float` | None |
| `media_adjust_volume(delta)` | Adjust volume | `delta: float` | None |
| `media_adjust_speed(delta)` | Adjust playback speed | `delta: float` | None |
| `get_media_player()` | Get media player state | None | `MediaPlayer` |
| `get_media_status()` | Get media status | None | `str` |

#### State Methods

| Method | Description | Parameters | Returns |
|--------|-------------|-----------|---------|
| `get_current_workspace()` | Get workspace state | None | `Workspace` |
| `save_settings()` | Save user settings | None | None |
| `load_user_preferences()` | Load user preferences | None | None |
| `export_settings(path)` | Export settings to file | `path: str` | None |
| `import_settings(path)` | Import settings from file | `path: str` | None |

### Data Types

#### `FileEntry`

Represents a file system entry.

```python
@dataclass
class FileEntry:
    path: str          # Full path to the file
    name: str          # File or directory name
    is_dir: bool       # True if directory
    size: int          # File size in bytes
    is_hidden: bool    # True if hidden
```

#### `Workspace`

Represents the current workspace state.

```python
@dataclass
class Workspace:
    current_dir: str      # Current directory path
    selected_index: int   # Index of selected entry
    show_hidden: bool     # Whether hidden files are visible
    entries: List[FileEntry]  # Entries in current directory
```

#### `Bookmark`

Represents a bookmarked directory.

```python
@dataclass
class Bookmark:
    path: str        # Directory path
    name: str        # Bookmark name
    icon: str        # Display icon
```

#### `Plugin`

Represents a loaded plugin.

```python
@dataclass
class Plugin:
    id: str           # Unique plugin ID
    name: str         # Plugin name
    description: str  # Plugin description
    enabled: bool     # Whether plugin is enabled
```

#### `MediaPlayer`

Represents media player state.

```python
@dataclass
class MediaPlayer:
    state: str           # Player state (playing, paused, stopped)
    position: float      # Current position in seconds
    volume: float        # Volume level (0.0 to 1.0)
    speed: float         # Playback speed (1.0 = normal)
    repeat_mode: str     # Repeat mode (off, one, all)
    current_index: int   # Current playlist index
    playlist: List[str]  # List of queued files
```

## Advanced Examples

### Batch File Operations

```python
from pyastrofs import PyAstroFS
from pathlib import Path

fs = PyAstroFS()
fs.navigate("/home/user/downloads")

# Create batch of files
for i in range(5):
    fs.create_file(f"document_{i}.txt")

# List and print all files
files = fs.list_files()
for file in files:
    if not file.is_dir:
        print(f"{file.name}: {file.size} bytes")
```

### Search and Process Results

```python
fs = PyAstroFS()
fs.navigate("/home/user")

# Search for image files
fs.search("*.jpg")
fs.search("*.png")  # Note: second search replaces first

results = fs.search_results()
print(f"Found {len(results)} PNG files")

for idx, result in enumerate(results):
    if idx >= 5:
        break
    print(f"  {result.name} - {result.size} bytes at {result.path}")
```

### Export and Restore State

```python
# Export current settings
fs.export_settings("/home/user/.config/astrofs_backup.json")

# Later, restore from backup
fs.import_settings("/home/user/.config/astrofs_backup.json")
```

### Multi-Directory Workflow

```python
# Work with multiple directories using bookmarks
fs.navigate("/home/user/projects")
fs.add_bookmark("projects")

fs.navigate("/home/user/documents")
fs.add_bookmark("documents")

# Quick switching between bookmarks
fs.goto_bookmark("projects")
print(f"Working in: {fs.current_dir()}")

fs.goto_bookmark("documents")
print(f"Working in: {fs.current_dir()}")
```

## Error Handling

All methods raise `ValueError` for errors. Use try/except blocks:

```python
from pyastrofs import PyAstroFS

fs = PyAstroFS()

try:
    fs.navigate("/nonexistent/path")
except ValueError as e:
    print(f"Navigation failed: {e}")

try:
    fs.add_bookmark("projects")
except ValueError as e:
    print(f"Bookmark already exists: {e}")
```

## Building from Source

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- Python 3.10+ ([install](https://www.python.org/downloads/))
- maturin (`pip install maturin`)

### Build Steps

#### On Linux/macOS

```bash
git clone https://github.com/pro-grammer-SD/astrofs.git
cd astrofs
chmod +x build_bindings.sh
./build_bindings.sh --release

# Install the built wheel
pip install target/wheels/astrofs-*.whl
```

#### On Windows

```powershell
git clone https://github.com/pro-grammer-SD/astrofs.git
cd astrofs
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process
.\build_bindings.ps1 -Release

# Install the built wheel
pip install target/wheels/astrofs-*.whl
```

## Testing

Run the examples:

```bash
python examples_python_bindings.py
```

This will demonstrate all major AstroFS features with 10 comprehensive examples.

## Type Hints

Full type stubs are included (`pyastrofs.pyi`) for excellent IDE support:

```python
from pyastrofs import PyAstroFS

fs: PyAstroFS = PyAstroFS()
files: list = fs.list_files()  # Full type hints for autocomplete
```

Works perfectly with:
- PyCharm / IntelliJ IDEA
- VS Code with Pylance
- mypy type checker
- pydantic

## Performance

AstroFS Python bindings are built on high-performance Rust code:

- ⚡ Fast file system operations
- 💾 Low memory footprint
- 🔄 Efficient recursive operations
- 📊 Optimized search algorithms

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md).

## License

Licensed under the same license as the main AstroFS project. See [LICENSE](../../LICENSE).

## Support

- 📚 [Documentation](https://github.com/pro-grammer-SD/astrofs/wiki)
- 🐛 [Issue Tracker](https://github.com/pro-grammer-SD/astrofs/issues)
- 💬 [Discussions](https://github.com/pro-grammer-SD/astrofs/discussions)

## Changelog

### v0.1.1

- ✨ Initial Python bindings release
- 🐍 Complete API coverage (9 classes, 50+ methods)
- 📦 Wheels for Python 3.10-3.14
- 🌐 Cross-platform support (Linux, macOS, Windows)
- 📝 Comprehensive type stubs
- 📚 Full documentation and examples

---

**Built with ❤️ using Rust and Python**
