use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};
use todui::{ui, App, InputMode};

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui::<B>(f, &app))?;

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
                        KeyCode::Tab => app.cycle_filter(),
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

        if let Some(timeout) = app.message_timeout {
            if timeout <= chrono::Local::now() {
                app.message = None;
                app.message_timeout = None;
            }
        }
    }
}

fn main() -> io::Result<()> {
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
