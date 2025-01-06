use crate::{App, InputMode};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_help(f: &mut Frame, app: &App, layout: Rect) {
    // Help text
    let help_text = match app.input_mode {
        InputMode::Normal => "[q]Quit [a]Add [↑↓/jk]Navigate [Space]Toggle [d]Delete [p]Priority [t]Tags [n]Note [Tab]Filter [?]Help",
        InputMode::AddingTodo => "[Enter]Save [Esc]Cancel",
        InputMode::AddingTags => "[Enter]Save tags [Esc]Cancel",
        InputMode::AddingNote => "[Enter]Save note [Esc]Cancel",
        InputMode::Help => "[Esc]Back to list",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(help, layout);
}

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
