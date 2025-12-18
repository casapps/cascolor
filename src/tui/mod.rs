mod app;
mod ui;

pub use app::TuiApp;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::fmt;

use crate::config::Config;

#[derive(Debug)]
pub enum TuiError {
    IoError(io::Error),
    TerminalError(String),
    AppError(String),
}

impl fmt::Display for TuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TuiError::IoError(e) => write!(f, "IO error: {}", e),
            TuiError::TerminalError(msg) => write!(f, "Terminal error: {}", msg),
            TuiError::AppError(msg) => write!(f, "Application error: {}", msg),
        }
    }
}

impl std::error::Error for TuiError {}

impl From<io::Error> for TuiError {
    fn from(e: io::Error) -> Self {
        TuiError::IoError(e)
    }
}

pub fn run(config: Config) -> Result<(), TuiError> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = TuiApp::new(config);
    let res = app.run(&mut terminal);

    // Restore terminal (always restore, even on error)
    let cleanup_result = cleanup_terminal(&mut terminal);

    // Return app result first, then cleanup errors
    res?;
    cleanup_result?;

    Ok(())
}

fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), TuiError> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
