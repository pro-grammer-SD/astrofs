use std::fs;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

#[allow(dead_code)]
pub struct PreviewContent {
    pub lines: Vec<Line<'static>>,
    pub is_binary: bool,
    pub preview_type: PreviewType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewType {
    Text,
    Code,
    Binary,
    Image,
    Archive,
    Error(String),
}

pub fn generate_preview(path: &Path, max_lines: usize) -> PreviewContent {
    if !path.exists() {
        return PreviewContent {
            lines: vec![Line::from("File not found")],
            is_binary: false,
            preview_type: PreviewType::Error("File not found".to_string()),
        };
    }

    if path.is_dir() {
        return preview_directory(path);
    }

    // Check if binary
    if let Ok(kind) = infer::get_from_path(path) {
        if let Some(kind) = kind {
            match kind.mime_type() {
                mime if mime.starts_with("image/") => {
                    return preview_image(path);
                }
                mime if mime.starts_with("application/") && mime.contains("zip") => {
                    return preview_archive(path);
                }
                _ => {}
            }
        }
    }

    // Try to read as text
    match fs::read_to_string(path) {
        Ok(content) => {
            if is_code_file(path) {
                // Use syntax highlighting for code files
                preview_code_with_highlighting(path, &content, max_lines)
            } else {
                // Plain text
                let lines: Vec<Line<'static>> = content
                    .lines()
                    .take(max_lines)
                    .map(|s| Line::from(s.to_string()))
                    .collect();

                PreviewContent {
                    lines,
                    is_binary: false,
                    preview_type: PreviewType::Text,
                }
            }
        }
        Err(_) => PreviewContent {
            lines: vec![
                Line::from("⚠️  Binary file"),
                Line::from(format!("Size: {}", humansize::format_size(
                    fs::metadata(path).map(|m| m.len()).unwrap_or(0),
                    humansize::BINARY
                ))),
            ],
            is_binary: true,
            preview_type: PreviewType::Binary,
        },
    }
}

fn preview_directory(path: &Path) -> PreviewContent {
    let mut lines = vec![Line::from("📁 Directory Contents:"), Line::from("")];

    if let Ok(entries) = fs::read_dir(path) {
        let mut count = 0;
        for entry in entries.take(50) {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.path().is_dir();
                let emoji = if is_dir { "📁" } else { "📄" };
                lines.push(Line::from(format!("{} {}", emoji, name)));
                count += 1;
            }
        }
        if count == 50 {
            lines.push(Line::from("... (more items)"));
        }
    }

    PreviewContent {
        lines,
        is_binary: false,
        preview_type: PreviewType::Text,
    }
}

fn preview_image(path: &Path) -> PreviewContent {
    let mut lines = vec![Line::from("🖼️  Image File"), Line::from("")];

    if let Ok(metadata) = fs::metadata(path) {
        lines.push(Line::from(format!("Size: {}", humansize::format_size(metadata.len(), humansize::BINARY))));
    }

    if let Some(ext) = path.extension() {
        lines.push(Line::from(format!("Format: {}", ext.to_string_lossy().to_uppercase())));
    }

    lines.push(Line::from(""));
    lines.push(Line::from("(Image preview not available in terminal)"));

    PreviewContent {
        lines,
        is_binary: true,
        preview_type: PreviewType::Image,
    }
}

fn preview_archive(path: &Path) -> PreviewContent {
    let mut lines = vec![Line::from("🗜️  Archive File"), Line::from("")];

    if let Ok(metadata) = fs::metadata(path) {
        lines.push(Line::from(format!("Size: {}", humansize::format_size(metadata.len(), humansize::BINARY))));
    }

    lines.push(Line::from(""));
    lines.push(Line::from("(Archive listing not yet implemented)"));

    PreviewContent {
        lines,
        is_binary: true,
        preview_type: PreviewType::Archive,
    }
}

fn is_code_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "hpp" | "java" | "go" 
            | "rb" | "php" | "swift" | "kt" | "cs" | "html" | "css" | "json" 
            | "yaml" | "toml" | "xml" | "sh" | "bash" | "md" | "txt"
        )
    } else {
        false
    }
}

fn preview_code_with_highlighting(path: &Path, content: &str, max_lines: usize) -> PreviewContent {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    let syntax = ps.find_syntax_for_file(path)
        .ok()
        .flatten()
        .or_else(|| Some(ps.find_syntax_plain_text()));

    if let Some(syntax) = syntax {
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        let mut highlighted_lines = Vec::new();

        for (idx, line) in LinesWithEndings::from(content).enumerate() {
            if idx >= max_lines {
                break;
            }

            if let Ok(ranges) = highlighter.highlight_line(line, &ps) {
                let mut spans = Vec::new();
                
                for (style, text) in ranges {
                    let fg_color = Color::Rgb(
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b,
                    );
                    
                    let ratatui_style = Style::default().fg(fg_color);
                    spans.push(Span::styled(text.to_string(), ratatui_style));
                }
                
                highlighted_lines.push(Line::from(spans));
            } else {
                highlighted_lines.push(Line::from(line.to_string()));
            }
        }

        PreviewContent {
            lines: highlighted_lines,
            is_binary: false,
            preview_type: PreviewType::Code,
        }
    } else {
        // Fallback to plain text
        let lines: Vec<Line<'static>> = content
            .lines()
            .take(max_lines)
            .map(|s| Line::from(s.to_string()))
            .collect();

        PreviewContent {
            lines,
            is_binary: false,
            preview_type: PreviewType::Text,
        }
    }
}
