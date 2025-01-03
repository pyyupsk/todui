use crate::todo::Todo;
use directories::ProjectDirs;
use std::{
    fs,
    io::{self, Result},
    path::PathBuf,
};

pub fn load_todos() -> Result<Vec<Todo>> {
    let config_dir = get_config_dir()?;
    let todo_file = config_dir.join("todos.json");

    match fs::read_to_string(todo_file) {
        Ok(contents) => {
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub fn save_todos(todos: &[Todo]) {
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
