//! NovaNet TUI v2 — rebuilt from scratch for stability.
//!
//! Phase 1: Exploration (MVP)
//! - Header with mode tabs [1-4]
//! - Taxonomy tree navigation
//! - Detail panel with edges
//! - Status bar with stats

mod app;
mod data;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures::StreamExt;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use std::path::Path;

use crate::db::Db;
use app::App;
use data::TaxonomyTree;

/// Entry point: initialize terminal, run event loop, restore terminal.
pub async fn run(db: &Db, root_path: &Path) -> crate::Result<()> {
    // Setup terminal
    enable_raw_mode().map_err(crate::NovaNetError::Io)?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(crate::NovaNetError::Io)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(crate::NovaNetError::Io)?;

    // Run the app
    let result = run_app(&mut terminal, db, root_path).await;

    // Restore terminal
    disable_raw_mode().map_err(crate::NovaNetError::Io)?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(crate::NovaNetError::Io)?;
    terminal.show_cursor().map_err(crate::NovaNetError::Io)?;

    result
}

/// Main event loop — simple and non-blocking.
async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    db: &Db,
    root_path: &Path,
) -> crate::Result<()> {
    // Load taxonomy tree from Neo4j
    let tree = TaxonomyTree::load(db).await?;
    let root_str = root_path.display().to_string();
    let mut app = App::new(tree, root_str);
    let mut event_stream = EventStream::new();

    // Initial render
    terminal
        .draw(|f| ui::render(f, &mut app))
        .map_err(crate::NovaNetError::Io)?;

    loop {
        // Wait for events (non-blocking with timeout for future animations)
        let event = tokio::time::timeout(
            Duration::from_millis(100),
            event_stream.next()
        ).await;

        match event {
            Ok(Some(Ok(Event::Key(key)))) => {
                // Ctrl+C or 'q' quits
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }
                if key.code == KeyCode::Char('q') {
                    break;
                }

                // Handle other keys
                if app.handle_key(key) {
                    terminal
                        .draw(|f| ui::render(f, &mut app))
                        .map_err(crate::NovaNetError::Io)?;
                }
            }
            Ok(Some(Ok(Event::Resize(_, _)))) => {
                // Re-render on resize
                terminal
                    .draw(|f| ui::render(f, &mut app))
                    .map_err(crate::NovaNetError::Io)?;
            }
            Ok(Some(Err(_))) => {
                // Event error, continue
            }
            Ok(None) => {
                // Stream ended
                break;
            }
            Err(_) => {
                // Timeout, continue (for future animations)
            }
            _ => {
                // Ignore other events (mouse, focus, paste)
            }
        }
    }

    Ok(())
}
