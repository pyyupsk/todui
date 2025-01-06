use crate::app::InputMode;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_input(
    f: &mut Frame,
    input: &str,
    input_mode: &InputMode,
    area: ratatui::layout::Rect,
) {
    let input_style = match input_mode {
        InputMode::Normal => Style::default().fg(Color::DarkGray),
        _ => Style::default().fg(Color::Yellow),
    };

    let input_title = match input_mode {
        InputMode::Normal => " Press 'a' to add a new todo ",
        InputMode::AddingTodo => " Enter a todo description... ",
        InputMode::AddingTags => " Enter a comma-separated list of tags... (Ex: work, personal) ",
        InputMode::AddingNote => " Enter a note... ",
        InputMode::Help => " Help Mode ",
    };

    let input = Paragraph::new(input.to_string()).style(input_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(input_style)
            .title(input_title),
    );

    f.render_widget(input, area);
}
