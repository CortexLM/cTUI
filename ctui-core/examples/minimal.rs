//! Minimal example demonstrating basic cTUI usage
//!
//! This example shows how to:
//! - Initialize a terminal with CrosstermBackend
//! - Render simple text to the screen
//! - Handle keyboard input
//! - Properly clean up on exit
//!
//! Run with: cargo run --example minimal

use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ctui_core::{backend::CrosstermBackend, terminal::Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let buffer = frame.buffer_mut();
            for (i, ch) in "Hello, cTUI!".chars().enumerate() {
                if let Some(cell) = buffer.get_mut(i as u16, 0) {
                    cell.symbol = ch.to_string();
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    execute!(terminal.backend_mut().writer_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
