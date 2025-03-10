use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{self, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Widget},
};
use tui_textarea::TextArea;

use super::utils::calculate_popup_area;

#[derive(Debug, PartialEq)]
pub enum CreateTodoInputField {
    TitleInput,
    BodyInput,
}

impl CreateTodoInputField {
    pub fn index(&self) -> usize {
        match self {
            CreateTodoInputField::TitleInput => 0,
            CreateTodoInputField::BodyInput => 1,
        }
    }
}

#[derive(Debug)]
pub struct CreateTodoWidget<'a> {
    pub input_fields: Vec<TextArea<'a>>,
    pub active_field: CreateTodoInputField,
    pub is_active: bool,
    outer_block: Block<'a>,
}

impl Default for CreateTodoWidget<'_> {
    fn default() -> Self {
        CreateTodoWidget {
            input_fields: { vec![TextArea::default(), TextArea::default()] },
            active_field: CreateTodoInputField::TitleInput,
            is_active: false,
            outer_block: {
                Block::default()
                    .borders(Borders::ALL)
                    .style(Color::Cyan)
                    .border_type(BorderType::Double)
                    .title("Create new TODO")
                    .title_bottom("Save TODO <CTRL> + s, Exit <ESC>")
            },
        }
    }
}

impl CreateTodoWidget<'_> {
    pub fn toggle_is_active(&mut self) {
        if !self.is_active {
            self.active_field = CreateTodoInputField::TitleInput;
        }
        self.is_active = !self.is_active
    }

    pub fn toggle_active_field(&mut self) {
        match self.active_field {
            CreateTodoInputField::TitleInput => self.active_field = CreateTodoInputField::BodyInput,
            _ => self.active_field = CreateTodoInputField::TitleInput,
        }
    }

    pub fn clear_fields(&mut self) {
        self.input_fields = vec![TextArea::default(), TextArea::default()];
    }

    pub fn key_event_handler(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_is_active();
            }
            KeyCode::Esc => {
                self.toggle_is_active();
                self.clear_fields();
            }
            KeyCode::Tab => {
                self.toggle_active_field();
            }
            KeyCode::Enter if self.active_field == CreateTodoInputField::TitleInput => {}
            _ => {
                self.input_fields[self.active_field.index()].input(key_event);
            }
        }
    }

    pub fn render_create_todo(&mut self, frame: &mut Frame) {
        let popup_area = calculate_popup_area(frame.area(), 30, 40);
        Clear.render(popup_area, frame.buffer_mut());
        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(0)])
            .margin(1)
            .split(popup_area);
        self.input_fields[CreateTodoInputField::TitleInput.index()]
            .set_placeholder_text("Insert title");
        self.input_fields[CreateTodoInputField::BodyInput.index()]
            .set_placeholder_text("Insert body");
        inactivate(&mut self.input_fields[self.active_field.index() ^ 1]);
        activate(&mut self.input_fields[self.active_field.index()]);
        frame.render_widget(&self.outer_block, popup_area);
        frame.render_widget(
            &self.input_fields[CreateTodoInputField::TitleInput.index()],
            layout[0],
        );
        frame.render_widget(
            &self.input_fields[CreateTodoInputField::BodyInput.index()],
            layout[1],
        );
    }
}

fn inactivate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_type(BorderType::Plain)
            .style(Style::default().fg(Color::DarkGray))
            .title_bottom(Line::from("Swap active field <Tab>").right_aligned()),
    );
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(Block::default().borders(Borders::ALL).style(Color::Cyan));
}
