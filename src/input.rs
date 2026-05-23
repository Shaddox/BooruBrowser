use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Focus};

pub async fn handle_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return;
    }

    if app.show_help {
        match key.code {
            KeyCode::Char('?') | KeyCode::Esc => app.show_help = false,
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        }
        return;
    }

    match app.focus {
        Focus::Search => handle_search_key(app, key).await,
        Focus::Results => handle_results_key(app, key).await,
    }
}

async fn handle_search_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.focus = Focus::Results,
        KeyCode::Enter => {
            app.page = 1;
            app.search().await;
        }
        KeyCode::Backspace => {
            app.query.pop();
        }
        KeyCode::Char('?') => app.show_help = true,
        KeyCode::Char('q') if app.query.is_empty() => app.should_quit = true,
        KeyCode::Char(char) => app.query.push(char),
        _ => {}
    }
}

async fn handle_results_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('?') => app.show_help = true,
        KeyCode::Char('/') => app.focus = Focus::Search,
        KeyCode::Char('j') | KeyCode::Down => app.next(),
        KeyCode::Char('k') | KeyCode::Up => app.previous(),
        KeyCode::Char('n') | KeyCode::Right => app.next_page().await,
        KeyCode::Char('p') | KeyCode::Left => app.previous_page().await,
        KeyCode::Enter => app.open_selected_post(),
        KeyCode::Char('o') => app.open_selected_post(),
        KeyCode::Char('i') => app.open_selected_image(),
        KeyCode::Char('r') => app.search().await,
        _ => {}
    }
}
