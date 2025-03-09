use std::error;

use tui_textarea::{Input, TextArea};

use crate::widgets::create_todo::{self, CreateTodoWidget};

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App<'a> {
    pub running: bool,
    pub create_todo_widget: CreateTodoWidget<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        App {
            running: true,
            create_todo_widget: CreateTodoWidget::default(),
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false
    }
}
