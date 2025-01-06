use crate::{App, InputMode};
use ratatui::{backend::Backend, widgets::Clear, Frame};

use super::{
    layouts::{centered_rect, create_main_layout},
    widgets::{
        render_filter_tabs, render_help_popup, render_input, render_message, render_status,
        render_title, render_todo_list,
    },
};

pub fn render<B: Backend>(f: &mut Frame, app: &App) {
    let layout = create_main_layout(f);

    render_title(f, layout.title);
    render_filter_tabs(f, &app.filter, layout.tabs);
    render_todo_list(f, app, layout.content);

    // Show detailed help if in help mode
    if matches!(app.input_mode, InputMode::Help) {
        let area = centered_rect(20, 35, f.area());
        f.render_widget(Clear, area);
        render_help_popup(f, area);
    }

    render_status(f, app, layout.status);
    render_input(f, &app.input, &app.input_mode, layout.input);
    render_message(f, &app.message, &app.message_timeout, layout.message);
}
