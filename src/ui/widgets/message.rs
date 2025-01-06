use chrono::{DateTime, Local};
use ratatui::{
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render_message(
    f: &mut Frame,
    message: &Option<(String, Color)>,
    timeout: &Option<DateTime<Local>>,
    area: ratatui::layout::Rect,
) {
    if let Some((message, color)) = message {
        if let Some(timeout) = timeout {
            if timeout > &Local::now() {
                let message = Paragraph::new(message.clone())
                    .style(Style::default().fg(*color))
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(message, area);
            }
        }
    }
}
