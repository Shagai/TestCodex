//! Simple example that renders a single paragraph using Ratatui.
//!
//! Run with `cargo run --example hello`.

use std::io::{self, stdout};
use crossterm::{execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{prelude::*, widgets::Paragraph};
use testcodex::build_paragraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal applications typically disable line buffering and other
    // interactive features. `enable_raw_mode` switches the terminal into this
    // raw mode state.
    terminal::enable_raw_mode()?;

    // Set up the Crossterm backend which Ratatui uses to draw to the terminal.
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Rendering happens inside the closure passed to `draw`. Here we create the
    // paragraph widget using our helper and render it to fill the entire screen.
    terminal.draw(|f| {
        let size = f.size();
        let paragraph: Paragraph = build_paragraph("Hello from Ratatui!");
        f.render_widget(paragraph, size);
    })?;

    // Restore the original terminal state before exiting.
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
