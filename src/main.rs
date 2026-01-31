mod app_new;
mod bookmarks;
mod config;
mod fileops;
mod files;
mod git;
mod input;
mod palette;
mod plugin;
mod preview;
mod search;
mod search_history;
mod theme;
mod ui_new;
mod workspace;

use app_new::{App, AppMode, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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
        let size = terminal.get_frame().size();
        app.set_viewport(size.width as usize, size.height as usize);

        terminal.draw(|f| ui_new::draw(f, app))?;

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
    // Don't clear messages for input mode
    if !matches!(app.mode, AppMode::Input(_)) {
        app.message = None;
        app.error = None;
    }

    // Handle input mode (text input for file creation, path entry, etc.)
    if let AppMode::Input(input_mode) = app.mode.clone() {
        match key.code {
            KeyCode::Char(c) => {
                app.input_buffer.push(c);
                return Ok(());
            }
            KeyCode::Backspace => {
                app.input_buffer.pop();
                return Ok(());
            }
            KeyCode::Enter => {
                let input = app.input_buffer.clone();
                app.input_buffer.clear();
                app.mode = AppMode::Normal;

                match input_mode {
                    InputMode::CreateFile => {
                        app.create_file(&input)?;
                    }
                    InputMode::CreateDirectory => {
                        app.create_directory(&input)?;
                    }
                    InputMode::Rename => {
                        app.rename_selected(&input)?;
                    }
                    InputMode::GoToPath => {
                        app.go_to_path(&input)?;
                    }
                    InputMode::AddBookmark => {
                        app.add_bookmark(input)?;
                    }
                }
                return Ok(());
            }
            KeyCode::Esc => {
                app.input_buffer.clear();
                app.mode = AppMode::Normal;
                return Ok(());
            }
            _ => return Ok(()),
        }
    }

    // Handle search mode
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
                    app.navigate_to_search_result(0)?;
                }
                return Ok(());
            }
            _ => return Ok(()),
        }
    }

    // Handle command palette
    if matches!(app.mode, AppMode::CommandPalette) {
        match key.code {
            KeyCode::Char(c) => {
                app.command_palette.add_char(c);
                app.command_search_index = 0;
                return Ok(());
            }
            KeyCode::Backspace => {
                app.command_palette.remove_char();
                return Ok(());
            }
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
                app.message = None;
                return Ok(());
            }
            KeyCode::Up => {
                if app.command_search_index > 0 {
                    app.command_search_index -= 1;
                }
                return Ok(());
            }
            KeyCode::Down => {
                if app.command_search_index < app.command_palette.visible_count().saturating_sub(1) {
                    app.command_search_index += 1;
                }
                return Ok(());
            }
            KeyCode::Enter => {
                if let Some(cmd) = app.command_palette.get_by_index(app.command_search_index) {
                    let cmd = cmd.clone();
                    app.execute_command(&cmd)?;
                }
                return Ok(());
            }
            _ => return Ok(()),
        }
    }

    // Handle help mode
    if matches!(app.mode, AppMode::Help) {
        app.mode = AppMode::Normal;
        return Ok(());
    }

    // Handle normal mode navigation and actions
    match key.code {
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::PageUp => app.page_up(),
        KeyCode::PageDown => app.page_down(),
        KeyCode::Home => app.go_home(),
        KeyCode::End => app.go_end(),
        
        // Enter/Open
        KeyCode::Enter | KeyCode::Right => {
            app.enter_selected()?;
        }
        
        // Go back/Up
        KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => {
            app.go_back()?;
        }
        
        // File operations
        KeyCode::Char('n') => app.mode = AppMode::Input(InputMode::CreateFile),
        KeyCode::Char('N') => app.mode = AppMode::Input(InputMode::CreateDirectory),
        KeyCode::Char('d') => app.delete_selected()?,
        KeyCode::Char('r') => app.mode = AppMode::Input(InputMode::Rename),
        KeyCode::Char('c') => app.copy_selected()?,
        
        // Search
        KeyCode::Char('/') => app.start_search(),
        KeyCode::Char('.') => app.toggle_hidden()?,
        
        // Workspaces
        KeyCode::Char('t') => app.new_workspace()?,
        KeyCode::Char('w') => app.close_workspace()?,
        KeyCode::Char(']') => app.next_workspace(),
        KeyCode::Char('[') => app.prev_workspace(),
        
        // Bookmarks
        KeyCode::Char('b') => app.mode = AppMode::Input(InputMode::AddBookmark),
        
        // Command Palette
        KeyCode::Char('p') => app.start_command_palette(),
        
        // System
        KeyCode::Char('?') => app.mode = AppMode::Help,
        KeyCode::Char('q') => app.quit(),
        KeyCode::Esc => {} // Just cancel any selection
        
        _ => {}
    }

    Ok(())
}

