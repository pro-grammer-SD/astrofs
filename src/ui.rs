use crate::app::{App, AppMode};
use crate::input::get_help_text;
use crate::theme::{get_file_emoji, get_file_style, Theme};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let theme = Theme::default();
    
    // Update viewport height
    let size = f.size();
    app.set_viewport_height((size.height.saturating_sub(4)) as usize);

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),      // Main content
            Constraint::Length(3),   // Status bar
        ])
        .split(size);

    // Horizontal split for file list and preview
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // File list
            Constraint::Percentage(70), // Preview
        ])
        .split(chunks[0]);

    // Draw based on mode
    match app.mode {
        AppMode::Help => draw_help(f, app, size, &theme),
        AppMode::Search => {
            draw_file_list(f, app, main_chunks[0], &theme);
            draw_search_results(f, app, main_chunks[1], &theme);
            draw_status_bar(f, app, chunks[1], &theme);
        }
        _ => {
            draw_file_list(f, app, main_chunks[0], &theme);
            draw_preview(f, app, main_chunks[1], &theme);
            draw_status_bar(f, app, chunks[1], &theme);
        }
    }
}

fn draw_file_list(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let items: Vec<ListItem> = app
        .entries
        .iter()
        .enumerate()
        .skip(app.scroll_offset)
        .take(area.height.saturating_sub(2) as usize)
        .map(|(i, entry)| {
            let emoji = get_file_emoji(&entry.path, entry.is_dir);
            let style = if i == app.selected_index {
                theme.selected
            } else {
                get_file_style(&entry.path, entry.is_dir, theme)
            };

            let size = entry.size_formatted();
            let content = format!("{} {} ({})", emoji, entry.name, size);
            
            ListItem::new(Line::from(vec![Span::styled(content, style)]))
        })
        .collect();

    let title = format!(
        "📁 {} ({} items)",
        app.current_dir.display(),
        app.entries.len()
    );

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border)
                .title(Span::styled(title, theme.folder)),
        );

    f.render_widget(list, area);
}

fn draw_preview(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let title = if let Some(entry) = app.selected_entry() {
        format!("📄 Preview: {}", entry.name)
    } else {
        "📄 Preview".to_string()
    };

    // Preview lines are now Line<'static> with syntax highlighting
    let preview_lines: Vec<Line> = app
        .preview
        .lines
        .iter()
        .take(area.height.saturating_sub(2) as usize)
        .cloned()
        .collect();

    let paragraph = Paragraph::new(preview_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border)
                .title(Span::styled(title, theme.normal)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_search_results(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let title = format!("🔍 Search: {} ({} results)", app.search_query, app.search_engine.results.len());

    let items: Vec<ListItem> = app
        .search_engine
        .results
        .iter()
        .take(area.height.saturating_sub(2) as usize)
        .map(|result| {
            let emoji = get_file_emoji(&result.path, result.is_dir);
            let style = if result.is_dir {
                theme.folder
            } else {
                theme.normal
            };

            let path_str = result.path.display().to_string();
            let content = format!("{} {} (score: {})", emoji, path_str, result.relevance);
            
            ListItem::new(Line::from(vec![Span::styled(content, style)]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border)
                .title(Span::styled(title, theme.help)),
        );

    f.render_widget(list, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let mut status_spans = Vec::new();

    // Git info
    if !app.git_info.status_string().is_empty() {
        status_spans.push(Span::styled(
            format!("{} {}", app.git_info.icon(), app.git_info.status_string()),
            Style::default().fg(ratatui::style::Color::Green),
        ));
        status_spans.push(Span::raw(" | "));
    }

    // Current directory
    status_spans.push(Span::styled(
        format!("📁 {}", app.current_dir.display()),
        theme.folder,
    ));

    // Selected file info
    if let Some(entry) = app.selected_entry() {
        status_spans.push(Span::raw(" | "));
        status_spans.push(Span::styled(
            format!("📄 {} ({})", entry.name, entry.size_formatted()),
            theme.normal,
        ));
    }

    // Mode indicator
    let mode_text = match app.mode {
        AppMode::Search => format!(" | 🔍 Search: {}_", app.search_query),
        AppMode::Help => " | ❓ Help".to_string(),
        AppMode::Normal => String::new(),
    };
    if !mode_text.is_empty() {
        status_spans.push(Span::styled(mode_text, theme.help));
    }

    let status_line = Line::from(status_spans);

    // Help hints
    let help_text = match app.mode {
        AppMode::Search => "ESC: Cancel | Enter: Open | Type to search",
        AppMode::Help => "Press any key to close help",
        _ => "↑/↓: Navigate | Enter: Open | /: Search | ?: Help | q: Quit",
    };

    let help_line = Line::from(vec![Span::styled(help_text, theme.help)]);

    // Message or error
    let message_line = if let Some(error) = &app.error {
        Line::from(vec![Span::styled(format!("❌ {}", error), theme.error)])
    } else if let Some(message) = &app.message {
        Line::from(vec![Span::styled(format!("ℹ️  {}", message), theme.normal)])
    } else {
        Line::from("")
    };

    let status_content = vec![status_line, help_line, message_line];

    let paragraph = Paragraph::new(status_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border)
                .style(theme.status_bar),
        );

    f.render_widget(paragraph, area);
}

fn draw_help(f: &mut Frame, _app: &App, area: Rect, theme: &Theme) {
    let help_items = get_help_text();
    
    let mut lines = vec![
        Line::from(Span::styled(
            "🌟 AstroFS - Help 🌟",
            Style::default()
                .fg(ratatui::style::Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Keyboard Shortcuts:", theme.help)),
        Line::from(""),
    ];

    for (key, description) in help_items {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:12}", key),
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" - {}", description), theme.normal),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press any key to close this help screen",
        theme.help,
    )));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border)
                .title(Span::styled("Help", theme.help)),
        )
        .wrap(Wrap { trim: false });

    // Center the help dialog
    let help_area = centered_rect(60, 80, area);
    f.render_widget(ratatui::widgets::Clear, help_area);
    f.render_widget(paragraph, help_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
