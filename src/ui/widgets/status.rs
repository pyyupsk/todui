use crate::App;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_status(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let status = Paragraph::new(app.get_status_line())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .alignment(Alignment::Center);
    f.render_widget(status, area);
}
