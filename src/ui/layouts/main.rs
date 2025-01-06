use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct AppLayout {
    pub title: Rect,
    pub tabs: Rect,
    pub help: Rect,
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
            Constraint::Length(2), // Help
            Constraint::Min(3),    // Content
            Constraint::Length(3), // Status
            Constraint::Length(3), // Input
            Constraint::Length(1), // Message
        ])
        .split(f.area());

    AppLayout {
        title: areas[0],
        tabs: areas[1],
        help: areas[2],
        content: areas[3],
        status: areas[4],
        input: areas[5],
        message: areas[6],
    }
}
