use crate::App;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub fn render_todo_list(f: &mut Frame, app: &App, layout: Rect) {
    let todos: Vec<ListItem> = app
        .filtered_todos()
        .iter()
        .map(|todo| {
            let mut spans = vec![
                if todo.completed {
                    Span::styled("✓ ", Style::default().fg(Color::Green))
                } else {
                    Span::styled("□ ", Style::default().fg(Color::Gray))
                },
                match todo.priority {
                    crate::todo::Priority::High => {
                        Span::styled("⚡", Style::default().fg(Color::Red))
                    }
                    crate::todo::Priority::Medium => {
                        Span::styled("●", Style::default().fg(Color::Yellow))
                    }
                    crate::todo::Priority::Low => {
                        Span::styled("○", Style::default().fg(Color::Green))
                    }
                },
                Span::raw(" "),
                Span::styled(
                    &todo.description,
                    Style::default()
                        .fg(if todo.completed {
                            Color::Gray
                        } else {
                            Color::White
                        })
                        .add_modifier(if todo.completed {
                            Modifier::CROSSED_OUT
                        } else {
                            Modifier::empty()
                        }),
                ),
            ];

            if !todo.tags.is_empty() {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    format!("[{}]", todo.tags.join(", ")),
                    Style::default().fg(Color::Cyan),
                ));
            }

            if !todo.notes.is_empty() {
                spans.push(Span::styled(" 📝", Style::default().fg(Color::Yellow)));
            }

            spans.push(Span::styled(
                format!(" ({})", todo.created_at.format("%Y-%m-%d")),
                Style::default().fg(Color::Gray),
            ));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let todos_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .title(Span::styled(
            " Tasks ",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ));

    let todos = List::new(todos)
        .block(todos_block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("➤ ");

    f.render_stateful_widget(
        todos,
        layout,
        &mut ListState::default().with_selected(app.selected_index),
    );
}
