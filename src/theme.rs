use anyhow::Result;
use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use dirs::config_dir;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorConfig {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorConfig {
    pub fn to_color(&self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
}

impl StyleConfig {
    pub fn to_style(&self) -> Style {
        let mut style = Style::default();

        if let Some(ref fg) = self.fg {
            style = style.fg(parse_color(fg));
        }

        if let Some(ref bg) = self.bg {
            style = style.bg(parse_color(bg));
        }

        if self.bold.unwrap_or(false) {
            style = style.add_modifier(Modifier::BOLD);
        }

        if self.italic.unwrap_or(false) {
            style = style.add_modifier(Modifier::ITALIC);
        }

        if self.underline.unwrap_or(false) {
            style = style.add_modifier(Modifier::UNDERLINED);
        }

        style
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub folder: StyleConfig,
    pub executable: StyleConfig,
    pub image: StyleConfig,
    pub archive: StyleConfig,
    pub text_file: StyleConfig,
    pub selected: StyleConfig,
    pub hidden: StyleConfig,
    pub status_bar: StyleConfig,
    pub error: StyleConfig,
    pub normal: StyleConfig,
    pub border: StyleConfig,
    pub help: StyleConfig,
}

impl ThemeConfig {
    pub fn to_theme(&self) -> Theme {
        Theme {
            folder: self.folder.to_style(),
            executable: self.executable.to_style(),
            image: self.image.to_style(),
            archive: self.archive.to_style(),
            text_file: self.text_file.to_style(),
            selected: self.selected.to_style(),
            hidden: self.hidden.to_style(),
            status_bar: self.status_bar.to_style(),
            error: self.error.to_style(),
            normal: self.normal.to_style(),
            border: self.border.to_style(),
            help: self.help.to_style(),
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: ThemeConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn load_or_default(name: &str) -> Result<Self> {
        if let Some(config_dir) = config_dir() {
            let theme_path = config_dir.join("astrofs").join(format!("{}.json", name));
            if theme_path.exists() {
                return Self::load_from_file(&theme_path);
            }
        }
        Ok(ThemeConfig::default_theme())
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn default_theme() -> Self {
        Self {
            name: "default".to_string(),
            folder: StyleConfig {
                fg: Some("cyan".to_string()),
                bg: None,
                bold: Some(true),
                italic: None,
                underline: None,
            },
            executable: StyleConfig {
                fg: Some("green".to_string()),
                bg: None,
                bold: Some(true),
                italic: None,
                underline: None,
            },
            image: StyleConfig {
                fg: Some("magenta".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
            archive: StyleConfig {
                fg: Some("yellow".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
            text_file: StyleConfig {
                fg: Some("white".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
            selected: StyleConfig {
                fg: Some("black".to_string()),
                bg: Some("cyan".to_string()),
                bold: Some(true),
                italic: None,
                underline: None,
            },
            hidden: StyleConfig {
                fg: Some("gray".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
            status_bar: StyleConfig {
                fg: Some("white".to_string()),
                bg: Some("blue".to_string()),
                bold: None,
                italic: None,
                underline: None,
            },
            error: StyleConfig {
                fg: Some("red".to_string()),
                bg: None,
                bold: Some(true),
                italic: None,
                underline: None,
            },
            normal: StyleConfig {
                fg: Some("white".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
            border: StyleConfig {
                fg: Some("cyan".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
            help: StyleConfig {
                fg: Some("yellow".to_string()),
                bg: None,
                bold: None,
                italic: None,
                underline: None,
            },
        }
    }
}

#[allow(dead_code)]
pub struct Theme {
    pub folder: Style,
    pub executable: Style,
    pub image: Style,
    pub archive: Style,
    pub text_file: Style,
    pub selected: Style,
    pub hidden: Style,
    pub status_bar: Style,
    pub error: Style,
    pub normal: Style,
    pub border: Style,
    pub help: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            folder: Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            executable: Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            image: Style::default().fg(Color::Magenta),
            archive: Style::default().fg(Color::Yellow),
            text_file: Style::default().fg(Color::White),
            selected: Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
            hidden: Style::default().fg(Color::DarkGray),
            status_bar: Style::default().bg(Color::Blue).fg(Color::White),
            error: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::White),
            border: Style::default().fg(Color::Cyan),
            help: Style::default().fg(Color::Yellow),
        }
    }
}

fn parse_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "gray" | "darkgray" | "dark_gray" => Color::DarkGray,
        "lightgray" | "light_gray" => Color::Gray,
        s if s.starts_with("rgb(") => {
            let parts: Vec<&str> = s.trim_start_matches("rgb(").trim_end_matches(")").split(',').collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].trim().parse::<u8>(),
                    parts[1].trim().parse::<u8>(),
                    parts[2].trim().parse::<u8>(),
                ) {
                    return Color::Rgb(r, g, b);
                }
            }
            Color::White
        }
        _ => Color::White,
    }
}

pub fn get_file_emoji(path: &std::path::Path, is_dir: bool) -> &'static str {
    if is_dir {
        return "📁";
    }

    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap_or("").to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => "🖼️",
            "zip" | "tar" | "gz" | "rar" | "7z" | "bz2" | "xz" => "🗜️",
            "exe" | "sh" | "bat" | "cmd" => "⚡",
            "rs" | "py" | "js" | "ts" | "c" | "cpp" | "java" | "go" => "📝",
            "txt" | "md" | "json" | "yaml" | "toml" | "xml" => "📄",
            "mp3" | "wav" | "flac" | "ogg" | "m4a" => "🎵",
            "mp4" | "avi" | "mkv" | "mov" | "webm" => "🎬",
            "pdf" => "📕",
            "lock" | "key" => "🔒",
            _ => "📄",
        }
    } else {
        "📄"
    }
}

pub fn get_file_style(path: &std::path::Path, is_dir: bool, theme: &Theme) -> Style {
    if is_dir {
        return theme.folder;
    }

    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap_or("").to_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => theme.image,
            "zip" | "tar" | "gz" | "rar" | "7z" | "bz2" | "xz" => theme.archive,
            "exe" | "sh" | "bat" | "cmd" => theme.executable,
            _ => theme.text_file,
        }
    } else {
        theme.text_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_parsing() {
        assert_eq!(parse_color("cyan"), Color::Cyan);
        assert_eq!(parse_color("red"), Color::Red);
        assert_eq!(parse_color("RGB(255,128,64)"), Color::Rgb(255, 128, 64));
    }

    #[test]
    fn test_theme_conversion() {
        let config = ThemeConfig::default_theme();
        let theme = config.to_theme();
        assert_eq!(theme.folder, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    }
}
