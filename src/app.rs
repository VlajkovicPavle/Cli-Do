use std::error;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        App { running: true }
    }
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false
    }
}
