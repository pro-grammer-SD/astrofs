#!/usr/bin/env pwsh
<#
.SYNOPSIS
Build AstroFS Python bindings using Maturin

.DESCRIPTION
Builds the complete AstroFS Python wheel file including type stubs (.pyi).
Uses PYO3 with ABI3 for forward compatibility with newer Python versions.
Outputs all wheels to the whl/ directory.

.PARAMETER OutputDir
Output directory for wheel files (default: whl)

.PARAMETER Release
Build in release mode (default: true)

.PARAMETER GenerateStubs
Generate type stub files (default: true)

.EXAMPLE
.\build_bindings.ps1
# Builds with default options to whl/ directory

.EXAMPLE
.\build_bindings.ps1 -Release $true
# Builds in release mode to whl/ directory
#>

param(
    [string]$OutputDir = "whl",
    [bool]$Release = $true,
    [bool]$GenerateStubs = $true
)

# Enable ABI3 forward compatibility for Python 3.10+
$env:PYO3_USE_ABI3_FORWARD_COMPATIBILITY = "1"

Write-Host "🔨 Building AstroFS Python bindings..." -ForegroundColor Cyan
Write-Host "Output directory: $OutputDir" -ForegroundColor Yellow

# Check if maturin is installed
try {
    $maturin = python -m maturin --version
    Write-Host "✓ Using maturin: $maturin" -ForegroundColor Green
} catch {
    Write-Host "✗ maturin not found. Install it with: pip install maturin" -ForegroundColor Red
    exit 1
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
    Write-Host "✓ Created $OutputDir directory" -ForegroundColor Green
}

# Build command
$buildArgs = @("python", "-m", "maturin", "build", "--out", $OutputDir)

if ($Release) {
    $buildArgs += "--release"
    Write-Host "📦 Building in RELEASE mode..." -ForegroundColor Yellow
} else {
    Write-Host "📦 Building in DEBUG mode..." -ForegroundColor Yellow
}

# Run the build
Write-Host ""
Write-Host "Running: $($buildArgs -join ' ')" -ForegroundColor Gray
& ([string]$buildArgs[0]) $buildArgs[1..($buildArgs.Length-1)]

if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Build completed successfully!" -ForegroundColor Green

# Wheels are already in $OutputDir due to --out flag
if ((Test-Path $OutputDir) -and (Test-Path "$OutputDir/*.whl")) {
    Write-Host ""
    Write-Host "✓ Wheels available in: $OutputDir/" -ForegroundColor Green
} else {
    Write-Host "⚠ No wheels found in $OutputDir" -ForegroundColor Yellow
}

# Generate type stubs if requested
if ($GenerateStubs) {
    Write-Host ""
    Write-Host "📝 Generating type stubs (pyastrofs.pyi)..." -ForegroundColor Cyan
    
    $pyi_content = @"
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
"@

    $stubPath = Join-Path $OutputDir "pyastrofs.pyi"
    Set-Content -Path $stubPath -Value $pyi_content -Encoding UTF8
    Write-Host "✓ Type stubs generated: $stubPath" -ForegroundColor Green
}

# List output files
if (Test-Path $OutputDir) {
    Write-Host ""
    Write-Host "📁 Wheel directory contents:" -ForegroundColor Cyan
    $files = Get-ChildItem $OutputDir | Sort-Object LastWriteTime -Descending
    foreach ($file in $files) {
        $size = "{0:N2} MB" -f ($file.Length / 1MB)
        Write-Host "  • $($file.Name) ($size)" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "✅ Build complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Install locally: pip install $OutputDir/*.whl" -ForegroundColor Gray
Write-Host "  2. Release to PyPI: python -m twine upload $OutputDir/*.whl" -ForegroundColor Gray
Write-Host ""
