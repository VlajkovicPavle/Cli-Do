use app::{App, AppResult};
use event::{Event, EventHandler};
use handler::handle_key_events;
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;
use tui::Tui;

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;
pub mod widgets;

struct Config {
    tick_rate: u64,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = Config { tick_rate: 250 };
    let mut app = App::new();
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(config.tick_rate);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    tui.exit()?;
    Ok(())
}
