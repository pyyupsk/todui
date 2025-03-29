use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};
use todui::{core::input, ui, App, Error, InputMode};

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Error> {
    loop {
        terminal.draw(|f| ui::render::<B>(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') && matches!(app.input_mode, InputMode::Normal) {
                    return Ok(());
                }
                input::handle_input(&mut app, Event::Key(key));
            }
        }

        app.update();
    }
}

fn setup_terminal() -> Result<(Terminal<CrosstermBackend<io::Stdout>>, App), Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let app = App::new();

    Ok((terminal, app))
}

fn cleanup_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--version".to_string()) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let (mut terminal, app) = setup_terminal()?;
    let result = run_app(&mut terminal, app);
    cleanup_terminal(terminal)?;
    result
}
