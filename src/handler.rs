use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        _ => {}
    }
    Ok(())
}
