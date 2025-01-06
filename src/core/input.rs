use crate::{App, InputMode};
use crossterm::event::{Event, KeyCode};
use std::mem;

pub fn handle_input(app: &mut App, key: Event) {
    if let Event::Key(key) = key {
        match app.input_mode {
            InputMode::Normal => handle_normal_mode(app, key.code),
            InputMode::AddingTodo => handle_editing_mode(app, key.code),
            InputMode::AddingTags => handle_adding_tags_mode(app, key.code),
            InputMode::AddingNote => handle_adding_note_mode(app, key.code),
            InputMode::Help => handle_help_mode(app, key.code),
        }
    }
}

fn handle_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('a') => {
            app.input_mode = InputMode::AddingTodo;
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
    }
}

fn handle_editing_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            let input = mem::take(&mut app.input);
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
    }
}

fn handle_adding_tags_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            let tags = mem::take(&mut app.input);
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
    }
}

fn handle_adding_note_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            let note = mem::take(&mut app.input);
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
    }
}

fn handle_help_mode(app: &mut App, key: KeyCode) {
    if key == KeyCode::Esc {
        app.input_mode = InputMode::Normal;
    }
}
