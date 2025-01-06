use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn render_title(f: &mut Frame, area: ratatui::layout::Rect) {
    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled("📝 ", Style::default().fg(Color::Yellow)),
        Span::styled(
            "Todui",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        ),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::horizontal(1)),
    )
    .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(title, area);
}
