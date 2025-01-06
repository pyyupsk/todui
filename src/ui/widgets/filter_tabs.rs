use crate::app::Filter;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub fn render_filter_tabs(f: &mut Frame, filter: &Filter, area: ratatui::layout::Rect) {
    let filters = vec!["All", "Active", "Completed", "High Priority"];
    let filter_index = match filter {
        Filter::All => 0,
        Filter::Active => 1,
        Filter::Completed => 2,
        Filter::HighPriority => 3,
    };

    let tabs = Tabs::new(filters)
        .select(filter_index)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider("|");

    f.render_widget(tabs, area);
}
