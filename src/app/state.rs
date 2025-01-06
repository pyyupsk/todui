use crate::core::{load_todos, save_todos};
use crate::todo::Todo;
use crate::Priority;
use chrono::{DateTime, Local};
use ratatui::style::Color;

#[derive(Clone)]
pub enum InputMode {
    Normal,
    AddingTodo,
    AddingNote,
    AddingTags,
    Help,
}

#[derive(Clone)]
pub enum Filter {
    All,
    Active,
    Completed,
    HighPriority,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub input: String,
    pub input_mode: InputMode,
    pub selected_index: Option<usize>,
    pub filter: Filter,
    pub message: Option<(String, Color)>,
    pub message_timeout: Option<DateTime<Local>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
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

    pub fn filtered_todos(&self) -> Vec<&Todo> {
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

    pub fn add_todo(&mut self, description: String) {
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

    pub fn toggle_priority(&mut self) {
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

    pub fn add_tags(&mut self, tags: String) {
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

    pub fn add_note(&mut self, note: String) {
        if let Some(index) = self.selected_index {
            if let Some(todo) = self.todos.get_mut(index) {
                todo.notes = note;
                save_todos(&self.todos);
                self.show_message("Note added!", Color::Green);
            }
        }
    }

    pub fn show_message(&mut self, message: &str, color: Color) {
        self.message = Some((message.to_string(), color));
        self.message_timeout = Some(Local::now() + chrono::Duration::seconds(3));
    }

    pub fn toggle_todo(&mut self) {
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

    pub fn delete_todo(&mut self) {
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

    pub fn move_selection(&mut self, delta: i32) {
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
            None => 0,
        };
        self.selected_index = Some(new_index);
    }

    pub fn get_selected_todo(&self) -> Option<&Todo> {
        self.selected_index.and_then(|i| self.todos.get(i))
    }

    pub fn get_status_line(&self) -> String {
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

    pub fn cycle_filter(&mut self) {
        self.filter = match self.filter {
            Filter::All => Filter::Active,
            Filter::Active => Filter::Completed,
            Filter::Completed => Filter::HighPriority,
            Filter::HighPriority => Filter::All,
        };
        self.selected_index = None;
    }

    pub fn update(&mut self) {
        if let Some(timeout) = self.message_timeout {
            if timeout <= chrono::Local::now() {
                self.message = None;
                self.message_timeout = None;
            }
        }
    }
}
