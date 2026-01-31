mod app;
mod files;
mod git;
mod input;
mod preview;
mod search;
mod theme;
mod ui;

use app::{App, AppMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use input::{handle_key_event, Action};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if !app.running {
            break;
        }

        // Handle events with timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only process key press events, not release
                if key.kind == KeyEventKind::Press {
                    handle_input(app, key)?;
                }
            }
        }
    }

    Ok(())
}

fn handle_input(app: &mut App, key: event::KeyEvent) -> anyhow::Result<()> {
    // Clear messages on new input
    app.message = None;
    app.error = None;

    // Handle search mode input
    if matches!(app.mode, AppMode::Search) {
        match key.code {
            KeyCode::Char(c) => {
                app.add_search_char(c);
                return Ok(());
            }
            KeyCode::Backspace => {
                app.remove_search_char();
                return Ok(());
            }
            KeyCode::Esc => {
                app.cancel_search();
                return Ok(());
            }
            KeyCode::Enter => {
                if !app.search_engine.results.is_empty() {
                    // Navigate to first result - clone the path to avoid borrow issues
                    let result_path = app.search_engine.results.first().map(|r| r.path.clone());
                    let result_is_dir = app.search_engine.results.first().map(|r| r.is_dir);
                    
                    if let (Some(path), Some(is_dir)) = (result_path, result_is_dir) {
                        if is_dir {
                            app.current_dir = path;
                            app.selected_index = 0;
                            app.scroll_offset = 0;
                            app.refresh_directory()?;
                        } else if let Some(parent) = path.parent() {
                            app.current_dir = parent.to_path_buf();
                            app.refresh_directory()?;
                            // Try to select the file
                            if let Some(pos) = app.entries.iter().position(|e| e.path == path) {
                                app.selected_index = pos;
                                app.update_preview();
                            }
                        }
                    }
                    app.cancel_search();
                }
                return Ok(());
            }
            _ => {}
        }
    }

    // Handle help mode
    if matches!(app.mode, AppMode::Help) {
        app.mode = AppMode::Normal;
        return Ok(());
    }

    // Handle normal mode actions
    let action = handle_key_event(key, matches!(app.mode, AppMode::Search));

    match action {
        Action::MoveUp => app.move_up(),
        Action::MoveDown => app.move_down(),
        Action::PageUp => app.page_up(),
        Action::PageDown => app.page_down(),
        Action::Home => app.go_home(),
        Action::End => app.go_end(),
        Action::Enter => {
            if let Err(e) = app.enter_selected() {
                app.error = Some(format!("Error: {}", e));
            }
        }
        Action::GoBack => {
            if let Err(e) = app.go_back() {
                app.error = Some(format!("Error: {}", e));
            }
        }
        Action::ToggleHidden => {
            if let Err(e) = app.toggle_hidden() {
                app.error = Some(format!("Error: {}", e));
            }
        }
        Action::Search => app.start_search(),
        Action::Refresh => {
            if let Err(e) = app.refresh_directory() {
                app.error = Some(format!("Error refreshing: {}", e));
            } else {
                app.message = Some("Directory refreshed".to_string());
            }
        }
        Action::Help => {
            app.mode = AppMode::Help;
        }
        Action::Quit => app.quit(),
        _ => {}
    }

    Ok(())
}
