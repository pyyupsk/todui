use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
    pub tags: Vec<String>,
    pub notes: String,
}
