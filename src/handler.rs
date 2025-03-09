use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.create_todo_widget.is_active {
        app.create_todo_widget.key_event_handler(key_event);
    }
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('n') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.create_todo_widget.toggle_is_active();
        }
        KeyCode::Char('n') => if key_event.modifiers == KeyModifiers::CONTROL {},
        _ => {}
    }
    Ok(())
}
