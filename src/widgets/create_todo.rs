use std::{default, ops::Index, rc};

use crate::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{self, Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};
use tui_textarea::TextArea;

use super::utils::calculate_popup_area;

#[derive(Debug)]
pub enum CreateTodoInputFields {
    TitleInput,
    BodyInput,
}

impl CreateTodoInputFields {
    pub fn index(&self) -> usize {
        match self {
            CreateTodoInputFields::TitleInput => 0,
            CreateTodoInputFields::BodyInput => 1,
        }
    }
}

#[derive(Debug)]
pub struct CreateTodoWidget<'a> {
    pub input_fields: Vec<TextArea<'a>>,
    pub active_field: CreateTodoInputFields,
    pub is_active: bool,
}

impl<'a> Default for CreateTodoWidget<'a> {
    fn default() -> Self {
        CreateTodoWidget {
            input_fields: { vec![TextArea::default(), TextArea::default()] },
            active_field: CreateTodoInputFields::TitleInput,
            is_active: false,
        }
    }
}

impl<'a> CreateTodoWidget<'a> {
    pub fn toggle_is_active(&mut self) {
        if (!self.is_active) {
            self.active_field = CreateTodoInputFields::TitleInput;
        }
        self.is_active = !self.is_active
    }

    pub fn toggle_active_field(&mut self) {
        match self.active_field {
            CreateTodoInputFields::TitleInput => {
                self.active_field = CreateTodoInputFields::BodyInput
            }
            _ => self.active_field = CreateTodoInputFields::TitleInput,
        }
    }

    pub fn render_create_todo(&mut self, frame: &mut Frame) {
        let popup_area = calculate_popup_area(frame.area(), 50, 50);
        Clear.render(popup_area, frame.buffer_mut());
        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(popup_area);
        inactivate(&mut self.input_fields[self.active_field.index() ^ 1]);
        activate(&mut self.input_fields[self.active_field.index()]);
        frame.render_widget(&self.input_fields[0], layout[0]);
        frame.render_widget(&self.input_fields[1], layout[1]);
    }

    pub fn key_event_handler(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_is_active();
            }
            KeyCode::Tab => {
                self.toggle_active_field();
            }
            _ => {
                self.input_fields[self.active_field.index()].input(key_event);
            }
        }
    }
}

fn inactivate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .title_bottom("Swap active field <Tab>"),
    );
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(Block::default().borders(Borders::ALL).style(Color::Cyan));
}
