#!/bin/bash

##
# Build AstroFS Python bindings using Maturin
#
# Description:
#   Builds the complete AstroFS Python wheel file including type stubs (.pyi).
#   Uses PYO3 with ABI3 for forward compatibility with newer Python versions.
#   Outputs all wheels to the whl/ directory.
#
# Usage:
#   ./build_bindings.sh [OPTIONS]
#
# Options:
#   -o, --output DIR      Output directory for wheel files (default: whl)
#   -r, --release         Build in release mode (default: true)
#   -d, --debug           Build in debug mode
#   -s, --skip-stubs      Skip type stub generation
#   -h, --help            Show this help message
#
# Examples:
#   ./build_bindings.sh                    # Build to whl/ directory
#   ./build_bindings.sh --debug            # Build in debug mode
#   ./build_bindings.sh -o build -r       # Build to build/ directory in release mode
##

set -e

# Default values
OUTPUT_DIR="whl"
RELEASE_MODE=true
GENERATE_STUBS=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -r|--release)
            RELEASE_MODE=true
            shift
            ;;
        -d|--debug)
            RELEASE_MODE=false
            shift
            ;;
        -s|--skip-stubs)
            GENERATE_STUBS=false
            shift
            ;;
        -h|--help)
            grep '^#' "$0" | tail -n +4 | sed 's/^##//' | sed 's/^# //'
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Enable ABI3 forward compatibility for Python 3.10+
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

echo "🔨 Building AstroFS Python bindings..."
echo "Output directory: $OUTPUT_DIR"
echo ""

# Check if maturin is installed
if ! python -m maturin --version > /dev/null 2>&1; then
    echo "✗ maturin not found. Install it with: pip install maturin"
    exit 1
fi

MATURIN_VERSION=$(python -m maturin --version)
echo "✓ Using maturin: $MATURIN_VERSION"

# Create output directory
mkdir -p "$OUTPUT_DIR"
echo "✓ Output directory: $OUTPUT_DIR"
echo ""

# Build
if [ "$RELEASE_MODE" = true ]; then
    echo "📦 Building in RELEASE mode..."
    python -m maturin build --release --out "$OUTPUT_DIR"
else
    echo "📦 Building in DEBUG mode..."
    python -m maturin build --out "$OUTPUT_DIR"
fi

if [ $? -ne 0 ]; then
    echo "✗ Build failed!"
    exit 1
fi

echo "✓ Build completed successfully!"
echo ""

# Wheels are already in $OUTPUT_DIR due to --out flag
if [ -f "$OUTPUT_DIR"/*.whl ]; then
    echo "✓ Wheels available in: $OUTPUT_DIR/"
else
    echo "ℹ No wheels found in $OUTPUT_DIR"
fi

# Generate type stubs if requested
if [ "$GENERATE_STUBS" = true ]; then
    echo ""
    echo "📝 Generating type stubs (pyastrofs.pyi)..."
    
    cat > "$OUTPUT_DIR/pyastrofs.pyi" << 'EOF'
"""
AstroFS - Advanced File System Navigator
Python bindings for the Rust-based file system navigation tool.

This module provides comprehensive file system navigation, search,
bookmarking, theming, plugin support, and media playback capabilities.
"""

from typing import List, Dict, Optional
from enum import Enum

__version__: str

class FileEntry:
    """Represents a file system entry"""
    path: str
    name: str
    is_dir: bool
    size: int
    is_hidden: bool

class Workspace:
    """Represents a file system workspace state"""
    current_dir: str
    selected_index: int
    show_hidden: bool
    entries: List[FileEntry]

class Bookmark:
    """Represents a bookmarked directory"""
    path: str
    name: str
    icon: str

class BookmarkManager:
    """Manages bookmarks"""
    bookmarks: Dict[str, Bookmark]

class SearchEngine:
    """Provides search functionality"""
    pass

class ThemeManager:
    """Manages themes"""
    current_theme: str
    available_themes: List[str]

class Plugin:
    """Represents a loaded plugin"""
    id: str
    name: str
    description: str
    enabled: bool

class PluginManager:
    """Manages plugins"""
    plugins: List[Plugin]

class MediaPlayer:
    """Represents media player state"""
    state: str
    position: float
    volume: float
    speed: float
    repeat_mode: str
    current_index: int
    playlist: List[str]

class MediaPreview:
    """Provides media preview functionality"""
    pass

class PyAstroFS:
    """Main AstroFS interface for Python"""
    
    # Navigation
    def current_dir(self) -> str: ...
    def navigate(self, path: str) -> None: ...
    def move_up(self) -> None: ...
    def move_down(self) -> None: ...
    def go_to_path(self, path: str) -> None: ...
    def refresh(self) -> None: ...
    
    # File operations
    def list_files(self, show_hidden: Optional[bool] = None) -> List[FileEntry]: ...
    def get_selected_entry(self) -> Optional[FileEntry]: ...
    def create_file(self, name: str) -> None: ...
    def create_directory(self, name: str) -> None: ...
    def delete_selected(self) -> None: ...
    def delete_file(self, path: str) -> None: ...
    def rename_selected(self, new_name: str) -> None: ...
    def duplicate_selected(self) -> None: ...
    def toggle_hidden(self) -> None: ...
    
    # Search
    def search(self, pattern: str) -> None: ...
    def search_results(self) -> List[FileEntry]: ...
    def navigate_to_search_result(self, index: int) -> None: ...
    def clear_search(self) -> None: ...
    
    # Bookmarks
    def add_bookmark(self, name: str) -> None: ...
    def remove_bookmark(self, name: str) -> None: ...
    def goto_bookmark(self, name: str) -> None: ...
    def get_bookmark_manager(self) -> BookmarkManager: ...
    
    # Themes
    def list_themes(self) -> List[str]: ...
    def switch_theme(self, theme: str) -> None: ...
    def get_theme_manager(self) -> ThemeManager: ...
    
    # Plugins
    def load_plugins(self) -> None: ...
    def enable_plugin(self, plugin_id: str) -> None: ...
    def disable_plugin(self, plugin_id: str) -> None: ...
    def get_plugin_manager(self) -> PluginManager: ...
    
    # Media
    def play_media(self, path: str) -> None: ...
    def pause_media(self) -> None: ...
    def toggle_media_playback(self) -> None: ...
    def media_seek(self, seconds: float) -> None: ...
    def media_adjust_volume(self, delta: float) -> None: ...
    def media_adjust_speed(self, delta: float) -> None: ...
    def get_media_player(self) -> MediaPlayer: ...
    def get_media_status(self) -> str: ...
    
    # State
    def get_current_workspace(self) -> Workspace: ...
    def save_settings(self) -> None: ...
    def load_user_preferences(self) -> None: ...
    def export_settings(self, path: str) -> None: ...
    def import_settings(self, path: str) -> None: ...
EOF
    
    echo "✓ Type stubs generated: $OUTPUT_DIR/pyastrofs.pyi"
fi

# List output files
echo ""
echo "📁 Wheel directory contents:"
if ls "$OUTPUT_DIR"/* 1> /dev/null 2>&1; then
    ls -lh "$OUTPUT_DIR" | awk 'NR>1 {printf "  • %-45s (%s)\n", $9, $5}'
else
    echo "  (empty)"
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "Next steps:"
echo "  1. Install locally: pip install $OUTPUT_DIR/*.whl"
echo "  2. Release to PyPI: python -m twine upload $OUTPUT_DIR/*.whl"
echo ""
