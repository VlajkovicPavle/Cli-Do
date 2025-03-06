use crate::App;
use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};

// TODO
// Use app if its needed inside of render function
pub fn render(_: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("HellowWorld")
            .block(
                Block::bordered()
                    .title("HelloWorldTitle")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        frame.area(),
    );
}
