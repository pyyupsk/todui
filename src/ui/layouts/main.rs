use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct AppLayout {
    pub title: Rect,
    pub tabs: Rect,
    pub content: Rect,
    pub status: Rect,
    pub input: Rect,
    pub message: Rect,
}

pub fn create_main_layout(f: &Frame) -> AppLayout {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(2), // Tabs
            Constraint::Min(3),    // Content
            Constraint::Length(3), // Status
            Constraint::Length(3), // Input
            Constraint::Length(1), // Message
        ])
        .split(f.area());

    AppLayout {
        title: areas[0],
        tabs: areas[1],
        content: areas[2],
        status: areas[3],
        input: areas[4],
        message: areas[5],
    }
}
