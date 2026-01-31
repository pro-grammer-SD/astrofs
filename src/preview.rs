use std::fs;
use std::io::Read;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

#[allow(dead_code)]
#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
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

    // Try to extract image dimensions
    if let Ok(img_data) = image::image_dimensions(path) {
        lines.push(Line::from(format!("Dimensions: {}x{} px", img_data.0, img_data.1)));
    }

    if lines.len() < 5 {
        lines.push(Line::from(""));
        lines.push(Line::from("(Image preview not available in terminal)"));
    }

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

    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        lines.push(Line::from(format!("Type: {}", ext_str.to_uppercase())));
    }

    lines.push(Line::from(""));

    // Try to list ZIP contents
    if path.extension().map(|e| e.to_string_lossy().to_lowercase() == "zip").unwrap_or(false) {
        if let Ok(file) = fs::File::open(path) {
            if let Ok(mut zip) = zip::ZipArchive::new(file) {
                lines.push(Line::from(format!("📦 Contents ({} files):", zip.len())));
                for i in 0..zip.len().min(20) {
                    if let Ok(file) = zip.by_index(i) {
                        let size_str = if file.is_dir() { "<DIR>".to_string() } else { 
                            humansize::format_size(file.size(), humansize::BINARY) 
                        };
                        lines.push(Line::from(format!("  {} ({})", file.name(), size_str)));
                    }
                }
                if zip.len() > 20 {
                    lines.push(Line::from(format!("  ... and {} more files", zip.len() - 20)));
                }
            } else {
                lines.push(Line::from("⚠️  Could not read ZIP archive"));
            }
        }
    }
    // Try to list TAR contents
    else if path.extension().map(|e| {
        let s = e.to_string_lossy().to_lowercase();
        s == "tar" || s == "gz" || s == "bz2" || s == "xz"
    }).unwrap_or(false) {
        if let Ok(file) = fs::File::open(path) {
            let reader: Box<dyn Read> = if path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default() == "gz" {
                Box::new(flate2::read::GzDecoder::new(file))
            } else {
                Box::new(file)
            };

            let mut archive = tar::Archive::new(reader);
            lines.push(Line::from("📦 Contents (TAR archive):"));
            if let Ok(entries) = archive.entries() {
                let mut count = 0;
                for entry_result in entries.take(20) {
                    if let Ok(entry) = entry_result {
                        if let Ok(size) = entry.header().size() {
                            let size_str = humansize::format_size(size, humansize::BINARY);
                            if let Ok(path) = entry.path() {
                                lines.push(Line::from(format!("  {} ({})", path.display(), size_str)));
                                count += 1;
                            }
                        }
                    }
                }
                if count == 20 {
                    lines.push(Line::from("  ... and more files"));
                }
            }
        }
    } else {
        lines.push(Line::from("Archive format not directly supported for preview"));
        lines.push(Line::from("Supported: .zip, .tar, .tar.gz"));
    }

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
