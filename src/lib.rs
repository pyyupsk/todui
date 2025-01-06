pub mod app;
pub mod core;
pub mod todo;
pub mod ui;

// Re-export commonly used types
pub use app::{App, Filter, InputMode};
pub use core::error::Error;
pub use todo::{Priority, Todo};
