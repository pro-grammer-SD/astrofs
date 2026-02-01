# AstroFS - Terminal File Explorer Usage Guide

## Installation

### From Source (Recommended)

```bash
cd astrofs
cargo build --release
# Binary located at: target/release/astrofs.exe (Windows) or target/release/astrofs (Linux/macOS)
```

### Running

```bash
# From project directory
./target/release/astrofs
# or add to PATH and run from anywhere
astrofs
```

## Key Bindings

### Navigation

| Key | Action |
| `j` / `↓` | Move down one entry |
| `k` / `↑` | Move up one entry |
| `h` / `←` | Go to parent directory |
| `l` / `→` | Enter directory or open file |
| `Home` | Go to home directory |
| `End` | Go to root directory |
| `Page Up` | Scroll up 10 entries |
| `Page Down` | Scroll down 10 entries |
| `g` | Go to beginning of list |
| `G` | Go to end of list |

### File Operations

| Key | Action |
| `c` | Copy selected file/directory |
| `m` | Move selected file/directory |
| `d` | Delete selected file/directory |
| `r` | Rename selected file/directory |
| `n` | Create new file |
| `N` | Create new directory |
| `o` | Open file with default application |

### Workspaces (Tabs)

| Key | Action |
| `t` | Create new workspace (tab) |
| `w` | Close current workspace |
| `]` | Switch to next workspace |
| `[` | Switch to previous workspace |
| `1-9` | Jump to workspace number |

### Search & Filter

| Key | Action |
| `/` | Start search mode |
| `Esc` | Cancel search |
| `Enter` | Navigate to first search result |
| `.` | Toggle hidden files visibility |

### Bookmarks

| Key | Action |
| `b` | Add current directory as bookmark |
| `B` | Go to bookmarked directory (choose from list) |

### Other

| Key | Action |
| `p` | Open command palette |
| `?` | Show help screen |
| `q` | Quit application |
| `Ctrl+C` | Force quit |

## Features

### 🎯 Multi-Pane Workspaces

Open multiple directories in separate tabs/workspaces. Switch between them instantly with `]`/`[` or number keys.

### 📦 Archive Preview

- **ZIP files**: View contents with file sizes
- **TAR/TAR.GZ**: List all entries with compression support
- Up to 20 files shown with "... and more files" indicator

### 🖼️ File Previews

- **Images**: Display dimensions and file size
- **Text Files**: Show first 50 lines with syntax highlighting
- **Code Files**: Syntax highlighting for 100+ languages
- **Directories**: List first 50 entries with emoji indicators

### 🎨 Configurable Themes

Themes are JSON-based and loaded from `~/.config/astrofs/theme/`. Available themes:

- `default` - Standard cyan/green theme
- `monokai` - Monokai color scheme
- `dracula` - Dracula purple theme

Customize colors in theme JSON files.

### 🔖 Bookmarks

Save frequently-visited directories as bookmarks. They're stored persistently in `~/.astrofs/bookmarks.json`.

Bookmarks are auto-saved when you add them.

### 🔍 Search History

Recent searches are automatically saved. Access with `/` key and scroll through history with Up/Down arrows.

Stored in `~/.astrofs/search_history.json` (max 50 by default).

### ⌨️ Command Palette

Press `p` to open the command palette with fuzzy filtering. Commands include:

- File operations (copy, move, delete, rename)
- Navigation commands
- Workspace management
- View toggles
- And more...

Filter by typing, select with Up/Down, execute with Enter.

### 📋 Copy/Move Operations

1. Select file: Position cursor on target
2. Press `c` (copy) or `m` (move)
3. Navigate to destination
4. Press again to complete or `Esc` to cancel

### 🎁 File Operations

- **Create File**: Press `n`, enter name, press Enter
- **Create Directory**: Press `N`, enter name, press Enter
- **Rename**: Press `r`, edit name, press Enter
- **Delete**: Press `d` (with confirmation)
- **Open With Default App**: Press `o`

### 🔌 Plugin System (Framework Ready)

The plugin system is ready for extensions. Plugins can:

- Hook into file operations
- Add custom commands
- Extend UI rendering

Plugin directory: `./plugins` (configurable)

## Configuration

### Config File Location

- **Linux/macOS**: `~/.config/astrofs/config.json`
- **Windows**: `%APPDATA%\astrofs\config.json`

### Default Configuration

```json
{
  "theme": "default",
  "show_hidden": false,
  "default_directory": ".",
  "preview_width_ratio": 0.7,
  "max_search_results": 100,
  "search_history_size": 50,
  "enable_git_integration": true,
  "enable_plugins": true,
  "plugin_directory": "./plugins"
}
```

### Theme Configuration

Theme files are stored in `~/.config/astrofs/theme/` as JSON:

```json
{
  "folder": { "fg": "cyan", "bg": "black" },
  "file": { "fg": "white", "bg": "black" },
  "selected": { "fg": "black", "bg": "yellow" },
  "error": { "fg": "red", "bg": "black" }
}
```

Supported colors: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `gray`

Or use RGB: `"rgb(255, 128, 64)"`

## Tips & Tricks

### Fast Navigation

- Use workspace tabs to keep multiple locations open
- Bookmarks for frequent directories
- Search with `/` to jump directly to files

### Efficient Workflows

- Use command palette (`p`) to discover shortcuts
- Help (`?`) always available for keybinding reference
- Hidden files toggle (`.`) for showing system files

### File Operations

- Selections persist across operations
- Copy/move can be canceled with `Esc`
- Deletions require confirmation

### Performance

- Search is parallelized for large directories
- Preview caching for frequently viewed files
- Lazy loading of directory contents

## Troubleshooting

### Application Won't Start

1. Verify Rust and dependencies are installed
2. Check `cargo build --release` completes without errors
3. Ensure terminal supports 256+ colors

### Preview Not Showing

- Not all file types are previewed (system files, etc.)
- Archive preview limited to 20 entries
- Very large files may timeout

### Permissions Issues

- Ensure read permissions on directories
- Operations on protected files will fail with error message
- Use system shell for privileged operations if needed

### Bookmarks/History Not Saving

- Check directory exists: `~/.astrofs/` and `~/.config/astrofs/`
- Verify write permissions on config directory
- Application should auto-create directories on first run

## Advanced Usage

### Custom Themes

1. Copy `config.default.json` to `~/.config/astrofs/theme/mytheme.json`
2. Edit colors in the new file
3. Set `"theme": "mytheme"` in config.json
4. Restart application

### Plugin Development

Plugins are Rust libraries loaded via `libloading`. See [plugin documentation](./src/plugin.rs) for trait definitions.

### Environment Variables

- `ASTROFS_CONFIG`: Override config file location
- `ASTROFS_THEME`: Override theme selection

## File Type Support

### Direct Preview

- Text files: `.txt`, `.md`, `.json`, `.toml`, `.yaml`, `.sh`, `.rs`, etc.
- Code files: `.rs`, `.py`, `.js`, `.go`, `.c`, `.cpp`, `.java`, etc.
- Archives: `.zip`, `.tar`, `.tar.gz`, `.gz`
- Images: `.png`, `.jpg`, `.jpeg`, `.gif`, `.bmp` (dimensions only)

### Handled by System

- Media files: `.mp3`, `.mp4`, `.mkv` (open with default app)
- Documents: `.pdf`, `.docx` (open with default app)
- Other: Any unrecognized format opens with system default

## Performance Metrics

Tested on typical systems:

- Directory listing: < 100ms for 10,000 files
- Search: < 500ms for 10,000 files (parallel)
- Preview generation: < 50ms for text files
- Archive listing: < 100ms for moderate archives

## Cross-Platform Notes

### Windows

- Uses `open` crate to launch default apps
- System32 directory protection prevents accidental deletion
- Path separator automatically handled

### Linux

- Respects XDG Base Directory specification
- System directory protection for `/sys`, `/proc`
- Requires `xdg-open` for default app handling

### macOS

- Uses `open` command for default apps
- /system directory protection enabled
- Full support for both Intel and Apple Silicon

## Known Limitations

1. **Large Archives**: Preview limited to first 20 files
2. **EXIF Data**: Not currently read from images
3. **Clipboard**: Copy/move don't interact with system clipboard (yet)
4. **Network Paths**: No built-in SMB/NFS support
5. **Plugins**: Dynamic loading framework ready, examples included

## Future Enhancements

- [ ] System clipboard integration
- [ ] Trash/Recycle bin instead of permanent delete
- [ ] Image thumbnail generation
- [ ] FTP/SFTP support
- [ ] Advanced plugin marketplace
- [ ] Mouse support
- [ ] Custom key binding configuration

## Getting Help

1. Press `?` in-app for keybinding help
2. Press `p` for command palette with descriptions
3. Check [OVERVIEW.md](OVERVIEW.md) for architecture details
4. Review [config.example.json](config.example.json) for configuration options

---

**AstroFS** - A blazing-fast, emoji-powered, oh-my-zsh inspired terminal file explorer in Rust 🚀
