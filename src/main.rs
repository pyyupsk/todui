use chrono::{DateTime, Local};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use directories::ProjectDirs;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Tabs},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    io::{self, Result},
    path::PathBuf,
    time::Duration,
};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    description: String,
    completed: bool,
    priority: Priority,
    created_at: DateTime<Local>,
    completed_at: Option<DateTime<Local>>,
    tags: Vec<String>,
    notes: String,
}

#[derive(Clone)]
enum InputMode {
    Normal,
    Editing,
    AddingNote,
    AddingTags,
    Help,
}

#[derive(Clone)]
enum Filter {
    All,
    Active,
    Completed,
    HighPriority,
}

struct App {
    todos: Vec<Todo>,
    input: String,
    input_mode: InputMode,
    selected_index: Option<usize>,
    filter: Filter,
    message: Option<(String, Color)>,
    message_timeout: Option<DateTime<Local>>,
}

impl App {
    fn new() -> App {
        App {
            todos: load_todos().unwrap_or_default(),
            input: String::new(),
            input_mode: InputMode::Normal,
            selected_index: None,
            filter: Filter::All,
            message: None,
            message_timeout: None,
        }
    }

    fn filtered_todos(&self) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| match self.filter {
                Filter::All => true,
                Filter::Active => !todo.completed,
                Filter::Completed => todo.completed,
                Filter::HighPriority => todo.priority == Priority::High,
            })
            .collect()
    }

    fn add_todo(&mut self, description: String) {
        let todo = Todo {
            id: self.todos.len() + 1,
            description,
            completed: false,
            priority: Priority::Medium,
            created_at: Local::now(),
            completed_at: None,
            tags: Vec::new(),
            notes: String::new(),
        };
        self.todos.push(todo);
        save_todos(&self.todos);
        self.show_message("Todo added successfully!", Color::Green);
    }

    fn toggle_priority(&mut self) {
        if let Some(index) = self.selected_index {
            if let Some(todo) = self.todos.get_mut(index) {
                todo.priority = match todo.priority {
                    Priority::Low => Priority::Medium,
                    Priority::Medium => Priority::High,
                    Priority::High => Priority::Low,
                };
                save_todos(&self.todos);
                self.show_message("Priority updated!", Color::Yellow);
            }
        }
    }

    fn add_tags(&mut self, tags: String) {
        if let Some(index) = self.selected_index {
            if let Some(todo) = self.todos.get_mut(index) {
                todo.tags = tags
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                save_todos(&self.todos);
                self.show_message("Tags updated!", Color::Green);
            }
        }
    }

    fn add_note(&mut self, note: String) {
        if let Some(index) = self.selected_index {
            if let Some(todo) = self.todos.get_mut(index) {
                todo.notes = note;
                save_todos(&self.todos);
                self.show_message("Note added!", Color::Green);
            }
        }
    }

    fn show_message(&mut self, message: &str, color: Color) {
        self.message = Some((message.to_string(), color));
        self.message_timeout = Some(Local::now() + chrono::Duration::seconds(3));
    }

    fn toggle_todo(&mut self) {
        if let Some(index) = self.selected_index {
            if let Some(todo) = self.todos.get_mut(index) {
                todo.completed = !todo.completed;
                if todo.completed {
                    todo.completed_at = Some(Local::now());
                } else {
                    todo.completed_at = None;
                }
                save_todos(&self.todos);
            }
        }
    }

    fn delete_todo(&mut self) {
        if let Some(index) = self.selected_index {
            self.todos.remove(index);
            save_todos(&self.todos);
            if self.selected_index.unwrap() >= self.todos.len() {
                self.selected_index = if self.todos.is_empty() {
                    None
                } else {
                    Some(self.todos.len() - 1)
                };
            }
        }
    }

    fn move_selection(&mut self, delta: i32) {
        let len = self.todos.len();
        if len == 0 {
            self.selected_index = None;
            return;
        }

        let new_index = match self.selected_index {
            Some(index) => {
                if delta > 0 {
                    (index + 1).min(len - 1)
                } else {
                    index.saturating_sub(1)
                }
            }
            _none => 0,
        };
        self.selected_index = Some(new_index);
    }
}

fn load_todos() -> Result<Vec<Todo>> {
    let config_dir = get_config_dir()?;
    let todo_file = config_dir.join("todos.json");

    match fs::read_to_string(todo_file) {
        Ok(contents) => {
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }
        Err(_) => Ok(Vec::new()),
    }
}

fn save_todos(todos: &[Todo]) {
    if let Ok(config_dir) = get_config_dir() {
        let todo_file = config_dir.join("todos.json");
        if let Ok(json) = serde_json::to_string_pretty(todos) {
            let _ = fs::create_dir_all(&config_dir);
            let _ = fs::write(todo_file, json);
        }
    }
}

fn get_config_dir() -> Result<PathBuf> {
    ProjectDirs::from("com", "pyyupsk", "todui")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine config directory",
            )
        })
}

fn ui<B: Backend>(f: &mut Frame, app: &App) {
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
    .alignment(ratatui::layout::Alignment::Center);
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
        .alignment(ratatui::layout::Alignment::Center);
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
                    Priority::High => Span::styled("⚡", Style::default().fg(Color::Red)),
                    Priority::Medium => Span::styled("●", Style::default().fg(Color::Yellow)),
                    Priority::Low => Span::styled("○", Style::default().fg(Color::Green)),
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

            // Add tags if present
            if !todo.tags.is_empty() {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    format!("[{}]", todo.tags.join(", ")),
                    Style::default().fg(Color::Cyan),
                ));
            }

            // Add note indicator if present
            if !todo.notes.is_empty() {
                spans.push(Span::styled(" 📝", Style::default().fg(Color::Yellow)));
            }

            // Add date
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
        .alignment(ratatui::layout::Alignment::Center);
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
            if timeout > Local::now() {
                let message = Paragraph::new(message.clone())
                    .style(Style::default().fg(*color))
                    .alignment(ratatui::layout::Alignment::Center);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::<B>(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                            app.input.clear();
                        }
                        KeyCode::Char('j') | KeyCode::Down => app.move_selection(1),
                        KeyCode::Char('k') | KeyCode::Up => app.move_selection(-1),
                        KeyCode::Char(' ') => app.toggle_todo(),
                        KeyCode::Char('d') => app.delete_todo(),
                        KeyCode::Char('p') => app.toggle_priority(),
                        KeyCode::Char('t') => {
                            if app.selected_index.is_some() {
                                app.input_mode = InputMode::AddingTags;
                                if let Some(todo) = app.get_selected_todo() {
                                    app.input = todo.tags.join(", ");
                                }
                            }
                        }
                        KeyCode::Char('n') => {
                            if app.selected_index.is_some() {
                                app.input_mode = InputMode::AddingNote;
                                if let Some(todo) = app.get_selected_todo() {
                                    app.input = todo.notes.clone();
                                }
                            }
                        }
                        KeyCode::Tab => {
                            app.filter = match app.filter {
                                Filter::All => Filter::Active,
                                Filter::Active => Filter::Completed,
                                Filter::Completed => Filter::HighPriority,
                                Filter::HighPriority => Filter::All,
                            };
                            app.selected_index = None;
                        }
                        KeyCode::Char('?') => {
                            app.input_mode = if matches!(app.input_mode, InputMode::Help) {
                                InputMode::Normal
                            } else {
                                InputMode::Help
                            };
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let input = std::mem::take(&mut app.input);
                            if !input.is_empty() {
                                app.add_todo(input);
                                app.input_mode = InputMode::Normal;
                            }
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input.clear();
                        }
                        _ => {}
                    },
                    InputMode::AddingTags => match key.code {
                        KeyCode::Enter => {
                            let tags = std::mem::take(&mut app.input);
                            app.add_tags(tags);
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input.clear();
                        }
                        _ => {}
                    },
                    InputMode::AddingNote => match key.code {
                        KeyCode::Enter => {
                            let note = std::mem::take(&mut app.input);
                            app.add_note(note);
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input.clear();
                        }
                        _ => {}
                    },
                    InputMode::Help => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }

        // Clear message if timeout has passed
        if let Some(timeout) = app.message_timeout {
            if timeout <= Local::now() {
                app.message = None;
                app.message_timeout = None;
            }
        }
    }
}

impl App {
    fn get_selected_todo(&self) -> Option<&Todo> {
        self.selected_index.and_then(|i| self.todos.get(i))
    }

    fn get_status_line(&self) -> String {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let high_priority = self
            .todos
            .iter()
            .filter(|t| matches!(t.priority, Priority::High))
            .count();

        format!(
            "Total: {} | Completed: {} | Pending: {} | High Priority: {} | Filter: {}",
            total,
            completed,
            total - completed,
            high_priority,
            match self.filter {
                Filter::All => "All",
                Filter::Active => "Active",
                Filter::Completed => "Completed",
                Filter::HighPriority => "High Priority",
            }
        )
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}
