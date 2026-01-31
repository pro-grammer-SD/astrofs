use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    Enter,
    GoBack,
    Quit,
    ToggleHidden,
    Search,
    CancelSearch,
    Refresh,
    PageUp,
    PageDown,
    Home,
    End,
    Help,
    None,
}

pub fn handle_key_event(key: KeyEvent, search_mode: bool) -> Action {
    if search_mode {
        return handle_search_mode_key(key);
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
        KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
        KeyCode::Enter => Action::Enter,
        KeyCode::Backspace | KeyCode::Char('h') => Action::GoBack,
        KeyCode::Char('.') => Action::ToggleHidden,
        KeyCode::Char('/') => Action::Search,
        KeyCode::Char('?') => Action::Help,
        KeyCode::F(5) => Action::Refresh,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::PageDown => Action::PageDown,
        KeyCode::Home => Action::Home,
        KeyCode::End => Action::End,
        _ => Action::None,
    }
}

fn handle_search_mode_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::CancelSearch,
        KeyCode::Enter => Action::Enter,
        _ => Action::None,
    }
}

pub fn get_help_text() -> Vec<(&'static str, &'static str)> {
    vec![
        ("↑/k", "Move up"),
        ("↓/j", "Move down"),
        ("Enter", "Open folder"),
        ("Backspace/h", "Go back"),
        ("/", "Search files"),
        (".", "Toggle hidden files"),
        ("F5", "Refresh"),
        ("PgUp/PgDn", "Page up/down"),
        ("Home/End", "First/last item"),
        ("?", "Show this help"),
        ("q / Ctrl+C", "Quit"),
    ]
}
