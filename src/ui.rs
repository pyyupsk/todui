use crate::app::{App, Filter, InputMode};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Tabs},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(2), // Tabs
            Constraint::Length(2), // Help
            Constraint::Min(3),    // Content
            Constraint::Length(3), // Status
            Constraint::Length(3), // Input
            Constraint::Length(1), // Message
        ])
        .split(f.area());

    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled("📝 ", Style::default().fg(Color::Yellow)),
        Span::styled(
            "Todo Master",
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
    .alignment(Alignment::Center);
    f.render_widget(title, main_layout[0]);

    // Filter tabs
    let filters = vec!["All", "Active", "Completed", "High Priority"];
    let filter_index = match app.filter {
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
    f.render_widget(tabs, main_layout[1]);

    // Help text
    let help_text = match app.input_mode {
        InputMode::Normal => "[q]Quit [e]Edit [↑↓/jk]Navigate [Space]Toggle [d]Delete [p]Priority [t]Tags [n]Note [Tab]Filter [?]Help",
        InputMode::Editing => "[Enter]Save [Esc]Cancel",
        InputMode::AddingTags => "[Enter]Save tags [Esc]Cancel",
        InputMode::AddingNote => "[Enter]Save note [Esc]Cancel",
        InputMode::Help => "[Esc]Back to list",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, main_layout[2]);

    // Todo list
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
        main_layout[3],
        &mut ListState::default().with_selected(app.selected_index),
    );

    // Show detailed view if in help mode
    if matches!(app.input_mode, InputMode::Help) {
        let help_text = vec![
            "Keyboard Shortcuts:",
            "─────────────────",
            "q      - Quit application",
            "e      - Edit selected todo",
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

        let area = centered_rect(60, 70, f.area());
        f.render_widget(Clear, area);
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

    // Status bar
    let status = Paragraph::new(app.get_status_line())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .alignment(Alignment::Center);
    f.render_widget(status, main_layout[4]);

    // Input box
    let input_style = match app.input_mode {
        InputMode::Normal => Style::default().fg(Color::DarkGray),
        _ => Style::default().fg(Color::Yellow),
    };

    let input_title = match app.input_mode {
        InputMode::Normal => " Input (Press 'e' to edit) ",
        InputMode::Editing => " Adding Todo... ",
        InputMode::AddingTags => " Adding Tags... ",
        InputMode::AddingNote => " Adding Note... ",
        InputMode::Help => " Help Mode ",
    };

    let input = Paragraph::new(app.input.clone()).style(input_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(input_style)
            .title(input_title),
    );
    f.render_widget(input, main_layout[5]);

    // Message display
    if let Some((message, color)) = &app.message {
        if let Some(timeout) = app.message_timeout {
            if timeout > chrono::Local::now() {
                let message = Paragraph::new(message.clone())
                    .style(Style::default().fg(*color))
                    .alignment(Alignment::Center);
                f.render_widget(message, main_layout[6]);
            }
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
