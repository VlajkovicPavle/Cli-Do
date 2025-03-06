use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::app::AppResult;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    // Event sender channel
    sender: mpsc::UnboundedSender<Event>,
    // Event receiver channel
    receiver: mpsc::UnboundedReceiver<Event>,
    // Event handler thread
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _sender.closed() => {break;}
                    _ = tick_delay => {
                            _sender.send(Event::Tick).unwrap();
                        }
                    Some(Ok(event)) = crossterm_event => {
                        match event {
                            CrosstermEvent::Key(key) => {
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    _sender.send(Event::Key(key)).unwrap();
                                }
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                _sender.send(Event::Mouse(mouse)).unwrap();
                            }
                            CrosstermEvent::Resize(x,y) => {
                                _sender.send(Event::Resize(x, y)).unwrap();
                            }
                            CrosstermEvent::FocusLost => {
                                todo!("Implement focusLost");
                            }
                            CrosstermEvent::FocusGained => {
                                todo!("Implement focus gained");
                            }
                            CrosstermEvent::Paste(_) => {
                                todo!("Implement paste");
                            }
                        }
                    }
                }
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is IO error!",
            )))
    }
}
