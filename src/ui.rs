use crate::{App, widgets::create_todo};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

// TODO
// Use app if its needed inside of render function
pub fn render(app: &mut App, frame: &mut Frame) {
    let shared_block = Block::new()
        .border_type(BorderType::Rounded)
        .border_style(Color::Cyan)
        .borders(Borders::ALL)
        .title_position(ratatui::widgets::block::Position::Top)
        .title_alignment(ratatui::layout::Alignment::Center);
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.area());
    let paragraph = Paragraph::new("Hello world!")
        .alignment(Alignment::Center)
        .centered()
        .block(shared_block.title("Preview"));

    frame.render_widget(paragraph, layout[0]);
    if (app.create_todo_widget.is_active) {
        app.create_todo_widget.render_create_todo(frame);
    }
}
