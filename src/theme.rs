use ratatui::style::{Color, Modifier, Style};

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
            // Changed from purple/magenta to blue background with white text
            status_bar: Style::default().bg(Color::Blue).fg(Color::White),
            error: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            normal: Style::default().fg(Color::White),
            border: Style::default().fg(Color::Cyan),
            help: Style::default().fg(Color::Yellow),
        }
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
