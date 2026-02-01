"""
AstroFS Python Bindings - Complete Usage Examples

This module demonstrates all capabilities of the AstroFS Python bindings,
including file system navigation, search, bookmarks, themes, plugins, and media playback.
"""

from pyastrofs import PyAstroFS # pyright: ignore[reportMissingModuleSource]
from pathlib import Path
import sys


def example_1_basic_navigation():
    """Example 1: Basic file system navigation"""
    print("=" * 60)
    print("EXAMPLE 1: Basic File System Navigation")
    print("=" * 60)
    
    # Create AstroFS instance
    fs = PyAstroFS()
    print(f"Initial directory: {fs.current_dir()}")
    
    # Navigate to home directory
    home = str(Path.home())
    fs.navigate(home)
    print(f"Navigated to: {fs.current_dir()}")
    
    # List files in current directory
    files = fs.list_files()
    print(f"\nFiles in {fs.current_dir()}:")
    for i, file in enumerate(files[:10]):  # Show first 10
        file_type = "📁" if file.is_dir else "📄"
        size_kb = file.size / 1024
        print(f"  {i+1}. {file_type} {file.name:<30} ({size_kb:.1f} KB)")
    
    if len(files) > 10:
        print(f"  ... and {len(files) - 10} more files")
    
    print()


def example_2_search():
    """Example 2: Search functionality"""
    print("=" * 60)
    print("EXAMPLE 2: Search Functionality")
    print("=" * 60)
    
    fs = PyAstroFS()
    home = str(Path.home())
    fs.navigate(home)
    
    # Search for Python files
    print(f"Searching for '*.py' in {home}...\n")
    fs.search("*.py")
    
    results = fs.search_results()
    print(f"Found {len(results)} Python files:\n")
    for i, result in enumerate(results[:5]):  # Show first 5
        print(f"  {i+1}. {result.name:<40} - {result.path}")
    
    if len(results) > 5:
        print(f"  ... and {len(results) - 5} more")
    
    print()


def example_3_bookmarks():
    """Example 3: Bookmark management"""
    print("=" * 60)
    print("EXAMPLE 3: Bookmark Management")
    print("=" * 60)
    
    fs = PyAstroFS()
    home = str(Path.home())
    fs.navigate(home)
    
    # Add bookmarks
    print(f"Adding bookmarks...\n")
    try:
        fs.add_bookmark("home")
        print("✓ Added bookmark 'home'")
    except ValueError as e:
        print(f"ℹ Bookmark already exists: {e}")
    
    # List bookmarks
    bm_manager = fs.get_bookmark_manager()
    print(f"\nBookmarks ({len(bm_manager.bookmarks)}):")
    for name, bookmark in bm_manager.bookmarks.items():
        print(f"  {bookmark.icon} {name:<20} → {bookmark.path}")
    
    # Navigate to bookmark
    if "home" in bm_manager.bookmarks:
        fs.goto_bookmark("home")
        print(f"\n✓ Navigated to 'home' bookmark: {fs.current_dir()}")
    
    print()


def example_4_themes():
    """Example 4: Theme management"""
    print("=" * 60)
    print("EXAMPLE 4: Theme Management")
    print("=" * 60)
    
    fs = PyAstroFS()
    
    # List themes
    themes = fs.list_themes()
    print(f"Available themes ({len(themes)}):")
    for theme in themes:
        print(f"  • {theme}")
    
    # Get current theme
    tm = fs.get_theme_manager()
    print(f"\nCurrent theme: {tm.current_theme}")
    
    # Switch theme
    if len(themes) > 1:
        new_theme = themes[1]
        try:
            fs.switch_theme(new_theme)
            tm = fs.get_theme_manager()
            print(f"✓ Switched to theme: {tm.current_theme}")
        except ValueError as e:
            print(f"✗ Failed to switch theme: {e}")
    
    print()


def example_5_plugins():
    """Example 5: Plugin management"""
    print("=" * 60)
    print("EXAMPLE 5: Plugin Management")
    print("=" * 60)
    
    fs = PyAstroFS()
    
    # Load plugins
    print("Loading plugins...")
    try:
        fs.load_plugins()
        print("✓ Plugins loaded")
    except ValueError as e:
        print(f"ℹ No plugins to load: {e}")
    
    # Get plugin manager
    pm = fs.get_plugin_manager()
    print(f"\nLoaded plugins ({len(pm.plugins)}):")
    for plugin in pm.plugins:
        status = "✓" if plugin.enabled else "✗"
        print(f"  {status} {plugin.name:<20} (ID: {plugin.id})")
        print(f"     Description: {plugin.description}")
    
    # Enable/disable plugins
    if pm.plugins:
        plugin = pm.plugins[0]
        try:
            if plugin.enabled:
                fs.disable_plugin(plugin.id)
                print(f"\n✓ Disabled plugin: {plugin.name}")
            else:
                fs.enable_plugin(plugin.id)
                print(f"\n✓ Enabled plugin: {plugin.name}")
        except ValueError as e:
            print(f"✗ Failed to toggle plugin: {e}")
    
    print()


def example_6_media_playback():
    """Example 6: Media playback control"""
    print("=" * 60)
    print("EXAMPLE 6: Media Playback Control")
    print("=" * 60)
    
    fs = PyAstroFS()
    
    # Get media player state
    mp = fs.get_media_player()
    print("Media Player Status:")
    print(f"  State:          {mp.state}")
    print(f"  Position:       {mp.position:.1f}s")
    print(f"  Volume:         {mp.volume * 100:.0f}%")
    print(f"  Speed:          {mp.speed:.1f}x")
    print(f"  Repeat Mode:    {mp.repeat_mode}")
    print(f"  Current Index:  {mp.current_index}")
    print(f"  Playlist Size:  {len(mp.playlist)}")
    
    # Get media status
    status = fs.get_media_status()
    print(f"\n  Status: {status}")
    
    # Media controls example (don't actually do these in example)
    print("\nAvailable media controls:")
    print("  • play_media(path)      - Start playing a file")
    print("  • pause_media()         - Pause playback")
    print("  • toggle_media_playback() - Toggle play/pause")
    print("  • media_seek(seconds)   - Seek to position")
    print("  • media_adjust_volume(delta) - Change volume")
    print("  • media_adjust_speed(delta)  - Change speed")
    
    print()


def example_7_file_operations():
    """Example 7: File operations"""
    print("=" * 60)
    print("EXAMPLE 7: File Operations")
    print("=" * 60)
    
    fs = PyAstroFS()
    
    # Navigate to a temp directory for demo
    import tempfile
    temp_dir = tempfile.gettempdir()
    fs.navigate(temp_dir)
    print(f"Working in: {fs.current_dir()}\n")
    
    # Create a test file
    test_file = "astrofs_test_file.txt"
    try:
        fs.create_file(test_file)
        print(f"✓ Created file: {test_file}")
    except ValueError as e:
        print(f"ℹ File already exists: {e}")
    
    # Create a test directory
    test_dir = "astrofs_test_dir"
    try:
        fs.create_directory(test_dir)
        print(f"✓ Created directory: {test_dir}")
    except ValueError as e:
        print(f"ℹ Directory already exists: {e}")
    
    # List files to confirm
    files = fs.list_files()
    print(f"\nFiles in {fs.current_dir()}:")
    test_entries = [f for f in files if f.name.startswith("astrofs_test")]
    for entry in test_entries:
        file_type = "📁" if entry.is_dir else "📄"
        print(f"  {file_type} {entry.name}")
    
    print()


def example_8_settings():
    """Example 8: Settings and preferences"""
    print("=" * 60)
    print("EXAMPLE 8: Settings and Preferences")
    print("=" * 60)
    
    fs = PyAstroFS()
    
    # Load user preferences
    print("Loading user preferences...")
    try:
        fs.load_user_preferences()
        print("✓ User preferences loaded")
    except ValueError as e:
        print(f"ℹ No preferences to load: {e}")
    
    # Save settings
    print("\nSaving current settings...")
    try:
        fs.save_settings()
        print("✓ Settings saved")
    except ValueError as e:
        print(f"✗ Failed to save settings: {e}")
    
    # Export settings to file
    export_path = str(Path.home() / "astrofs_settings_export.json")
    print(f"\nExporting settings to: {export_path}")
    try:
        fs.export_settings(export_path)
        print("✓ Settings exported successfully")
    except ValueError as e:
        print(f"✗ Failed to export settings: {e}")
    
    print()


def example_9_workspace():
    """Example 9: Workspace management"""
    print("=" * 60)
    print("EXAMPLE 9: Workspace Management")
    print("=" * 60)
    
    fs = PyAstroFS()
    home = str(Path.home())
    fs.navigate(home)
    
    # Get current workspace
    ws = fs.get_current_workspace()
    print("Current Workspace:")
    print(f"  Current Dir:    {ws.current_dir}")
    print(f"  Selected Index: {ws.selected_index}")
    print(f"  Show Hidden:    {ws.show_hidden}")
    print(f"  Entries:        {len(ws.entries)}")
    
    # Navigate up
    print("\nNavigating...")
    fs.move_down()
    ws = fs.get_current_workspace()
    print(f"✓ Selected index: {ws.selected_index}")
    
    fs.move_up()
    ws = fs.get_current_workspace()
    print(f"✓ Selected index: {ws.selected_index}")
    
    # Toggle hidden files
    print(f"\nHidden files visible: {ws.show_hidden}")
    fs.toggle_hidden()
    ws = fs.get_current_workspace()
    print(f"✓ Hidden files visible: {ws.show_hidden}")
    
    print()


def example_10_complete_workflow():
    """Example 10: Complete workflow combining multiple features"""
    print("=" * 60)
    print("EXAMPLE 10: Complete Workflow")
    print("=" * 60)
    
    fs = PyAstroFS()
    home = str(Path.home())
    
    print(f"Starting in: {home}\n")
    
    # 1. Navigate to home
    fs.navigate(home)
    print(f"1. ✓ Navigated to {fs.current_dir()}")
    
    # 2. Add bookmark
    try:
        fs.add_bookmark("home")
        print("2. ✓ Bookmarked current directory")
    except:
        pass
    
    # 3. Search for files
    fs.search("*")
    results = fs.search_results()
    print(f"3. ✓ Found {len(results)} items")
    
    # 4. Get current state
    ws = fs.get_current_workspace()
    bm = fs.get_bookmark_manager()
    tm = fs.get_theme_manager()
    
    print(f"4. ✓ Current state:")
    print(f"   - Workspace entries: {len(ws.entries)}")
    print(f"   - Bookmarks: {len(bm.bookmarks)}")
    print(f"   - Theme: {tm.current_theme}")
    
    # 5. Save settings
    try:
        fs.save_settings()
        print("5. ✓ Settings saved")
    except:
        print("5. ℹ Settings already saved")
    
    print("\n✅ Workflow complete!")
    print()


def main():
    """Run all examples"""
    print("\n")
    print("╔" + "=" * 58 + "╗")
    print("║" + " " * 58 + "║")
    print("║" + "  AstroFS Python Bindings - Complete Examples  ".center(58) + "║")
    print("║" + " " * 58 + "║")
    print("╚" + "=" * 58 + "╝")
    print()
    
    try:
        example_1_basic_navigation()
        example_2_search()
        example_3_bookmarks()
        example_4_themes()
        example_5_plugins()
        example_6_media_playback()
        example_7_file_operations()
        example_8_settings()
        example_9_workspace()
        example_10_complete_workflow()
        
        print("=" * 60)
        print("✅ ALL EXAMPLES COMPLETED SUCCESSFULLY!")
        print("=" * 60)
        print()
        print("For more information, see:")
        print("  • Documentation: https://github.com/pro-grammer-SD/astrofs/tree/main/docs")
        print("  • API Reference: Check https://github.com/pro-grammer-SD/astrofs/blob/main/pyastrofs.pyi for type stubs")
        print("  • Issues: https://github.com/pro-grammer-SD/astrofs/issues")
        print()
        
    except Exception as e:
        print(f"\n✗ Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
    