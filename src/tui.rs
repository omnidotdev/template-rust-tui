//! TUI event loop.

use std::{io, time::Duration};

use crossterm::event;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, error::Result, events::handle_event, ui::render};

/// Run the TUI event loop.
pub async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> Result<()> {
    while app.running {
        // Calculate content height from terminal size.
        // Layout: header (3) + content (flexible) + input (3).
        let term_height = terminal.size()?.height;
        let content_height = term_height.saturating_sub(6).saturating_sub(2); // Subtract header/input + borders.

        // Render.
        terminal.draw(|frame| render(frame, &mut app))?;

        // Poll for events.
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            handle_event(&mut app, &event, content_height);
        }
    }

    Ok(())
}
