mod commands;
mod keyboard;
mod search;
mod ui;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use ui::App;

fn main() -> Result<()> {
    // Load commands
    let commands = commands::load_commands()?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(commands);

    // Main loop
    while !app.should_quit {
        // Update animation
        app.tick();

        // Draw
        terminal.draw(|frame| app.draw(frame))?;

        // Handle input
        app.handle_input()?;
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
