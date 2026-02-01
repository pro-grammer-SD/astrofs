use crate::app::{App, AppMode, InputMode};
use crate::theme::{get_file_emoji, get_file_style, Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let theme = Theme::default();
    let size = f.size();
    app.set_viewport(size.width as usize, size.height as usize);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(size);

    match app.mode {
        AppMode::Help => draw_help(f, app, chunks[0], &theme),
        AppMode::CommandPalette => draw_command_palette(f, app, size, &theme),
        AppMode::Input(_) => {
            draw_file_browser(f, app, chunks[0], &theme);
            draw_workspace_tabs(f, app, chunks[1], &theme);
            draw_input_dialog(f, app, chunks[2], &theme);
        }
        AppMode::Search => {
            draw_file_browser(f, app, chunks[0], &theme);
            draw_workspace_tabs(f, app, chunks[1], &theme);
            draw_search_status(f, app, chunks[2], &theme);
        }
        _ => {
            draw_file_browser(f, app, chunks[0], &theme);
            draw_workspace_tabs(f, app, chunks[1], &theme);
            draw_status_bar(f, app, chunks[2], &theme);
        }
    }
}

fn draw_file_browser(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let workspace = app.get_current_workspace();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_file_list(f, app, workspace, chunks[0], theme);
    draw_preview_pane(f, app, workspace, chunks[1], theme);
}

fn draw_file_list(f: &mut Frame, _app: &App, workspace: &crate::workspace::Workspace, area: Rect, theme: &Theme) {
    let current_dir_name = workspace
        .current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Root");

    let block = Block::default()
        .title(format!(" {} ", current_dir_name))
        .borders(Borders::ALL)
        .style(theme.border);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut items = Vec::new();
    for (idx, entry) in workspace.entries.iter().enumerate() {
        let style = if idx == workspace.selected_index {
            theme.selected
        } else if entry.is_hidden {
            theme.hidden
        } else {
            get_file_style(&entry.path, entry.is_dir, theme)
        };

        let emoji = get_file_emoji(&entry.path, entry.is_dir);
        let name = if entry.is_hidden {
            format!("·{}", entry.name)
        } else {
            entry.name.clone()
        };

        let display = format!("{} {}", emoji, name);
        items.push(ListItem::new(display).style(style));
    }

    let list = List::new(items);
    f.render_widget(list, inner);
}

fn draw_preview_pane(
    f: &mut Frame,
    _app: &App,
    workspace: &crate::workspace::Workspace,
    area: Rect,
    theme: &Theme,
) {
    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .style(theme.border);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = workspace.preview.lines.clone();
    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(theme.normal);

    f.render_widget(paragraph, inner);
}

fn draw_workspace_tabs(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let mut tabs = Vec::new();

    for (idx, workspace) in app.workspace_manager.workspaces().iter().enumerate() {
        let is_active = app.workspace_manager.active_id() == workspace.id;
        let style = if is_active {
            theme.selected
        } else {
            theme.normal
        };

        let marker = if is_active { "▶ " } else { "  " };
        let text = format!("{}{}", marker, workspace.title);
        tabs.push(Span::styled(text, style));
        if idx < app.workspace_manager.workspaces().len() - 1 {
            tabs.push(Span::raw(" │ "));
        }
    }

    let line = Line::from(tabs);
    let paragraph = Paragraph::new(line)
        .alignment(Alignment::Left)
        .style(theme.normal);

    f.render_widget(paragraph, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let workspace = app.get_current_workspace();
    let path_display = format!(" {} ", workspace.current_dir.display());

    let status = if let Some(err) = &app.error {
        Paragraph::new(err.clone())
            .style(theme.error)
            .alignment(Alignment::Left)
    } else if let Some(msg) = &app.message {
        Paragraph::new(msg.clone())
            .style(theme.status_bar)
            .alignment(Alignment::Left)
    } else {
        Paragraph::new(path_display)
            .style(theme.status_bar)
            .alignment(Alignment::Left)
    };

    f.render_widget(status, area);
}

fn draw_search_status(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let search_info = format!(
        " 🔍 Search: {} {} ",
        app.search_query,
        if app.search_engine.is_searching { "..." } else { "" }
    );

    let paragraph = Paragraph::new(search_info)
        .style(theme.status_bar)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_input_dialog(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let input_label = match &app.mode {
        AppMode::Input(InputMode::CreateFile) => "📄 Create File",
        AppMode::Input(InputMode::CreateDirectory) => "📁 Create Directory",
        AppMode::Input(InputMode::Rename) => "✏️  Rename",
        AppMode::Input(InputMode::GoToPath) => "🌐 Go to Path",
        AppMode::Input(InputMode::AddBookmark) => "🔖 Add Bookmark",
        _ => "",
    };

    let text = format!("{}: {}_", input_label, app.input_buffer);
    let paragraph = Paragraph::new(text)
        .style(theme.normal)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_command_palette(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let palette_height = (area.height as usize).min(15);
    let palette_width = (area.width as usize).min(60);

    let x = (area.width as usize - palette_width) / 2;
    let y = (area.height as usize - palette_height) / 2;

    let palette_area = Rect {
        x: x as u16,
        y: y as u16,
        width: palette_width as u16,
        height: palette_height as u16,
    };

    let block = Block::default()
        .title(" Command Palette ")
        .borders(Borders::ALL)
        .style(theme.border)
        .bg(Color::Black);

    let inner = block.inner(palette_area);
    f.render_widget(block, palette_area);

    let filter_line = Line::from(format!("> {}_", app.command_palette.filter()));
    let filter_para = Paragraph::new(filter_line).style(theme.normal);

    let input_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: 1,
    };
    f.render_widget(filter_para, input_area);

    let visible_commands = app.command_palette.visible();
    let mut items = Vec::new();

    for (idx, (_, cmd)) in visible_commands.iter().enumerate() {
        let style = if idx == app.command_search_index {
            theme.selected
        } else {
            theme.normal
        };
        items.push(ListItem::new(cmd.to_string()).style(style));
    }

    let list = List::new(items);
    let list_area = Rect {
        x: inner.x,
        y: inner.y + 2,
        width: inner.width,
        height: inner.height.saturating_sub(2),
    };

    f.render_widget(list, list_area);
}

fn draw_help(f: &mut Frame, _app: &App, area: Rect, theme: &Theme) {
    let help_text = vec![
        Line::from("🚀 AstroFS Help - Terminal File Explorer"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  j/↓ - Move down     k/↑ - Move up     h/← - Go back     l/→ - Open"),
        Line::from("  PgDn - Page down   PgUp - Page up    Home - Start      End - End"),
        Line::from(""),
        Line::from(vec![
            Span::styled("File Operations:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  n - Create file    N - Create directory    r - Rename"),
        Line::from("  d - Delete        c - Copy               . - Toggle hidden"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Workspaces:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  t - New tab       w - Close tab         [ - Prev tab      ] - Next tab"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Search & Commands:", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  / - Search        b - Add bookmark      p - Command palette"),
        Line::from("  ? - Help          q - Quit              ESC - Cancel"),
        Line::from(""),
        Line::from("Press any key to return..."),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .style(theme.border);

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(theme.normal);

    f.render_widget(paragraph, area);
}
