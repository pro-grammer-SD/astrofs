# 🎨 AstroFS Themes & Customization Guide

## Quick Start

AstroFS comes with **5 built-in themes** ready to use. Switch between them instantly without restarting:

```bash
# Press Ctrl+T to open the theme switcher
# Then select a theme with arrow keys and press Enter
```

## Built-in Themes

### 1. **Default** (Default on startup)
```json
Primary: Cyan (#00D9FF)
Secondary: Green (#00FF9F)  
Accent: Pink (#FF006E)
Best for: Developers who want vibrant, modern colors
```

### 2. **Dracula**
```json
Primary: Cyan (#8BE9FD)
Secondary: Green (#50FA7B)
Accent: Pink (#FF79C6)
Background: #282A36
Best for: Low-light environments, high contrast
```

### 3. **Nord**
```json
Primary: Blue (#88C0D0)
Secondary: Green (#A3BE8C)
Accent: Purple (#B48EAD)
Background: #2E3440
Best for: Professional look, arctic colors
```

### 4. **Monokai**
```json
Primary: Cyan (#66D9EF)
Secondary: Green (#A6E22E)
Accent: Pink (#F92672)
Background: #272822
Best for: Classic programmer aesthetic
```

### 5. **Solarized Dark**
```json
Primary: Blue (#268BD2)
Secondary: Green (#859900)
Accent: Red (#D33682)
Background: #002B36
Best for: Long coding sessions, eye-friendly
```

## 🎯 Switching Themes

### Interactive Switcher (Recommended)
```
1. Press Ctrl+T
2. Use arrow keys to navigate themes
3. Press Enter to apply
4. Theme preference is saved automatically
```

### Via Command Palette
```
1. Press Ctrl+/ to open Command Palette
2. Type: "switch-theme"
3. Select desired theme
4. Press Enter
```

### Via Settings (Advanced)
Edit `~/.config/astrofs/settings.json`:
```json
{
  "current_theme": "dracula",
  "theme_history": ["default", "dracula", "nord"]
}
```

## 🛠️ Creating Custom Themes

### Step 1: Create Theme File
Save a new JSON file in `~/.config/astrofs/themes/` (Linux/macOS) or `%APPDATA%/astrofs/themes/` (Windows):

```bash
# Linux/macOS
mkdir -p ~/.config/astrofs/themes
cat > ~/.config/astrofs/themes/my-awesome-theme.json << 'EOF'
{
  "name": "my-awesome-theme",
  "description": "My custom dark theme",
  "author": "Your Name",
  "version": "1.0.0",
  "colors": {
    "primary": "#00D9FF",
    "secondary": "#00FF9F",
    "accent": "#FF006E",
    "background": "#0A0E27",
    "foreground": "#E0E6FC",
    "error": "#FF0040",
    "warning": "#FFBE0B",
    "success": "#00FF9F",
    "info": "#00D9FF",
    "file_color": "#E0E6FC",
    "directory_color": "#00D9FF",
    "symlink_color": "#FF006E",
    "executable_color": "#00FF9F",
    "selection_bg": "#00D9FF",
    "selection_fg": "#0A0E27",
    "cursor_color": "#00FF9F"
  },
  "borders": {
    "style": "rounded",
    "color": "#00D9FF",
    "focused_color": "#00FF9F"
  },
  "emojis": {
    "folder": "📁",
    "file": "📄",
    "symlink": "🔗",
    "executable": "⚙️",
    "archive": "📦",
    "image": "🖼️",
    "video": "🎬",
    "audio": "🎵",
    "document": "📝",
    "code": "💻",
    "bookmark": "🔖",
    "search": "🔍",
    "settings": "⚡",
    "loading": "⏳",
    "error": "❌",
    "success": "✅",
    "info": "ℹ️"
  },
  "fonts": {
    "use_powerline": true,
    "use_nerd_fonts": true,
    "enable_italics": true,
    "enable_bold": true
  }
}
EOF
```

### Step 2: Use Your Theme
After creating the file, restart AstroFS and use Ctrl+T to switch to your theme.

## 🎨 Theme Structure Explained

### Colors Section
```json
"colors": {
  "primary": "#00D9FF",      // Main UI color
  "secondary": "#00FF9F",    // Secondary highlights
  "accent": "#FF006E",       // Accent color for focus
  "background": "#0A0E27",   // Background color
  "foreground": "#E0E6FC",   // Text color
  "error": "#FF0040",        // Error messages
  "warning": "#FFBE0B",      // Warnings
  "success": "#00FF9F",      // Success messages
  "info": "#00D9FF",         // Info messages
  
  // File type colors
  "file_color": "#E0E6FC",
  "directory_color": "#00D9FF",
  "symlink_color": "#FF006E",
  "executable_color": "#00FF9F",
  
  // UI element colors
  "selection_bg": "#00D9FF",     // Selected item background
  "selection_fg": "#0A0E27",     // Selected item text
  "cursor_color": "#00FF9F"      // Cursor color
}
```

### Borders Section
```json
"borders": {
  "style": "rounded",           // Options: rounded, sharp, double, none
  "color": "#00D9FF",          // Border color
  "focused_color": "#00FF9F"   // Color when focused
}
```

### Emojis Section
Customize the emojis for each file type and UI element:
```json
"emojis": {
  "folder": "📁",      // Folder icon
  "file": "📄",        // Regular file icon
  "symlink": "🔗",     // Symlink icon
  "executable": "⚙️",  // Executable icon
  // ... more emojis
}
```

### Fonts Section
Control typography:
```json
"fonts": {
  "use_powerline": true,   // Use Powerline symbols
  "use_nerd_fonts": true,  // Use Nerd font icons
  "enable_italics": true,  // Italic text support
  "enable_bold": true      // Bold text support
}
```

## 🌈 Color Formats

### Hex Format (Recommended)
```json
"primary": "#00D9FF"
```

### Named Colors
Supported colors:
- `cyan`, `green`, `pink`, `red`, `yellow`, `blue`, `purple`, `white`, `black`

```json
"primary": "cyan"
```

## 📝 Theme Creation Tips

### 1. **Choose a Color Palette**
Start with complementary colors:
- **Primary**: Main UI color
- **Secondary**: Accent for highlights
- **Accent**: Focus/selection color

### 2. **Test Readability**
Ensure sufficient contrast between text and background:
```
Good: Dark text (#E0E6FC) on dark background (#0A0E27)
Bad: Similar brightness colors
```

### 3. **Use Consistent Emojis**
Pick emojis that work together visually:
- File operations: 📄 📁 🗑️ 📋 ✂️ 📋
- Status: ✅ ❌ ⚠️ ℹ️ ⏳
- Media: 🖼️ 🎬 🎵 📻 📸

### 4. **Consider Accessibility**
- High contrast for better visibility
- Avoid red-green color combinations
- Support colorblind-friendly palettes

## 🔧 Advanced: Theme Inheritance

Create a theme based on an existing one:

```bash
# In AstroFS command palette (Ctrl+/)
create-theme-from: nord my-custom-nord

# Then edit the new theme file in ~/.config/astrofs/themes/
```

## 💾 Exporting and Sharing Themes

### Export Your Theme
```
1. Press Ctrl+/
2. Type: "export-theme"
3. Select your theme
4. Choose export path
5. Share the .json file!
```

### Import a Theme
```
1. Place the .json file in ~/.config/astrofs/themes/
2. Restart AstroFS
3. Use Ctrl+T to switch to it
```

## 🎬 Theme Switching Shortcuts

Add these to your custom keybindings:

```json
{
  "keybindings": {
    "Ctrl+T": "switch-theme",
    "Ctrl+Shift+T": "theme-history",
    "Ctrl+Alt+T": "random-theme"
  }
}
```

## 🆘 Troubleshooting

### Theme Not Appearing
```bash
# Ensure JSON is valid
cat ~/.config/astrofs/themes/my-theme.json | python -m json.tool

# Check file permissions
chmod 644 ~/.config/astrofs/themes/my-theme.json

# Restart AstroFS
```

### Colors Not Applying
- Restart AstroFS after changing theme
- Verify terminal supports true color (24-bit)
- Check for conflicting keybindings

### Emojis Not Showing
- Install Nerd Font or Unicode font
- Check terminal Unicode support
- Enable `use_nerd_fonts` in theme

## 📚 Resources

- [Built-in Theme Gallery](themes/)
- [Color Palette Generator](https://coolors.co/)
- [Nerd Fonts](https://www.nerdfonts.com/)
- [Create Your Own Theme](THEME_TEMPLATE.md)

---

**Next Steps:**
- [Plugin Development Guide](PLUGINS.md)
- [Complete Usage Guide](USAGE.md)
- [Configuration](config.example.json)
