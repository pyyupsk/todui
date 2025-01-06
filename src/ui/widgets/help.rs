use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_help_popup(f: &mut Frame, area: Rect) {
    let help_text = vec![
        "Keyboard Shortcuts:",
        "─────────────────",
        "q      - Quit application",
        "a      - Add todo",
        "j/↓    - Move selection down",
        "k/↑    - Move selection up",
        "Space  - Toggle completion",
        "d      - Delete selected todo",
        "p      - Cycle priority",
        "t      - Add/edit tags",
        "n      - Add/edit note",
        "Tab    - Cycle through filters",
        "?      - Toggle this help",
        "",
        "Press Esc to close help",
    ];

    f.render_widget(
        Paragraph::new(Text::from(help_text.join("\n")))
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White)),
        area,
    );
}
