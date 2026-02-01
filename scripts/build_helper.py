#!/usr/bin/env python3
"""
Cross-platform build helper for AstroFS Python bindings.

This script provides a unified interface for building AstroFS wheels
across Windows, Linux, and macOS.
"""

import argparse
import os
import platform
import subprocess
import sys
from pathlib import Path


def get_platform_name():
    """Get human-readable platform name."""
    system = platform.system()
    if system == "Darwin":
        return "macOS"
    elif system == "Windows":
        return "Windows"
    elif system == "Linux":
        return "Linux"
    else:
        return system


def check_dependencies():
    """Check if required tools are installed."""
    print("🔍 Checking dependencies...")
    
    # Check Python
    try:
        result = subprocess.run(
            [sys.executable, "--version"],
            capture_output=True,
            text=True
        )
        print(f"✓ Python: {result.stdout.strip()}")
    except Exception as e:
        print(f"✗ Python not found: {e}")
        return False
    
    # Check maturin
    try:
        result = subprocess.run(
            [sys.executable, "-m", "maturin", "--version"],
            capture_output=True,
            text=True
        )
        print(f"✓ Maturin: {result.stdout.strip()}")
    except Exception as e:
        print(f"✗ Maturin not found. Install with: pip install maturin")
        return False
    
    # Check Rust/Cargo
    try:
        result = subprocess.run(
            ["cargo", "--version"],
            capture_output=True,
            text=True
        )
        print(f"✓ Cargo: {result.stdout.strip()}")
    except Exception as e:
        print(f"✗ Cargo not found. Install Rust from https://rustup.rs/")
        return False
    
    return True


def run_build(output_dir, release, generate_stubs, debug):
    """Run the build process."""
    print("")
    print("🔨 Building AstroFS Python bindings...")
    print(f"Platform: {get_platform_name()}")
    print(f"Output directory: {output_dir}")
    
    # Set environment variables
    os.environ["PYO3_USE_ABI3_FORWARD_COMPATIBILITY"] = "1"
    
    # Prepare output directory
    Path(output_dir).mkdir(exist_ok=True)
    
    # Build command
    build_cmd = [sys.executable, "-m", "maturin", "build"]
    
    if release and not debug:
        build_cmd.append("--release")
        print("📦 Building in RELEASE mode...")
    else:
        print("📦 Building in DEBUG mode...")
    
    # Run build
    try:
        result = subprocess.run(build_cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"✗ Build failed: {e}")
        return False
    
    # Copy wheels to output directory
    import shutil
    
    wheels_src = Path("target/wheels")
    if wheels_src.exists():
        print(f"\n📋 Copying wheels to {output_dir}/...")
        for wheel in wheels_src.glob("*.whl"):
            shutil.copy2(wheel, output_dir)
            size_mb = wheel.stat().st_size / (1024 * 1024)
            print(f"  ✓ {wheel.name} ({size_mb:.2f} MB)")
    
    # Generate type stubs
    if generate_stubs:
        print("\n📝 Generating type stubs...")
        generate_type_stubs(output_dir)
    
    return True


def generate_type_stubs(output_dir):
    """Generate pyastrofs.pyi type stub file."""
    pyi_content = '''"""
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
'''
    
    stub_path = Path(output_dir) / "pyastrofs.pyi"
    stub_path.write_text(pyi_content)
    print(f"  ✓ Type stubs generated: {stub_path}")


def list_output_files(output_dir):
    """List files in output directory."""
    output_path = Path(output_dir)
    if not output_path.exists():
        return
    
    print(f"\n📁 Output directory contents ({output_dir}):")
    files = sorted(output_path.iterdir(), key=lambda x: x.stat().st_mtime, reverse=True)
    
    if not files:
        print("  (empty)")
        return
    
    for file in files:
        size_mb = file.stat().st_size / (1024 * 1024)
        print(f"  • {file.name:<50} ({size_mb:>7.2f} MB)")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Build AstroFS Python bindings for all platforms",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/build_helper.py                    # Build to whl/ (release)
  python scripts/build_helper.py --debug            # Build in debug mode
  python scripts/build_helper.py -o build --release # Build to build/ (release)
  python scripts/build_helper.py --skip-stubs       # Skip type stub generation
        """
    )
    
    parser.add_argument(
        "-o", "--output",
        default="whl",
        help="Output directory for wheels (default: whl)"
    )
    parser.add_argument(
        "-r", "--release",
        action="store_true",
        default=True,
        help="Build in release mode (default)"
    )
    parser.add_argument(
        "-d", "--debug",
        action="store_true",
        help="Build in debug mode"
    )
    parser.add_argument(
        "--skip-deps-check",
        action="store_true",
        help="Skip dependency checks"
    )
    parser.add_argument(
        "--skip-stubs",
        action="store_true",
        help="Skip type stub generation"
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Skip build (only generate stubs/check deps)"
    )
    
    args = parser.parse_args()
    
    print(f"╔{'=' * 60}╗")
    print(f"║ AstroFS Python Bindings Build Helper".ljust(62) + "║")
    print(f"║ Platform: {get_platform_name()}".ljust(62) + "║")
    print(f"╚{'=' * 60}╝")
    print()
    
    # Check dependencies
    if not args.skip_deps_check:
        if not check_dependencies():
            sys.exit(1)
        print()
    
    # Build
    if not args.skip_build:
        success = run_build(
            args.output,
            args.release and not args.debug,
            not args.skip_stubs,
            args.debug
        )
        
        if not success:
            sys.exit(1)
    
    # List output files
    list_output_files(args.output)
    
    print()
    print("✅ Build complete!")
    print()
    print("Next steps:")
    print(f"  1. Install: pip install {args.output}/*.whl")
    print(f"  2. Release: python -m twine upload {args.output}/*.whl")
    print(f"  3. Examples: python examples_python_bindings.py")
    print()


if __name__ == "__main__":
    main()
