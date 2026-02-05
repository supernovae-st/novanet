//! NovaNet TUI v2 — rebuilt from scratch for stability.
//!
//! Phase 1: Exploration (MVP)
//! - Header with mode tabs [1-4]
//! - Taxonomy tree navigation
//! - Detail panel with edges
//! - Status bar with stats
//!
//! ## Crash Recovery
//!
//! The TUI installs a panic hook that:
//! 1. Restores terminal state (disables raw mode, leaves alternate screen)
//! 2. Logs panic info to `~/.novanet/crash.log`
//! 3. Displays helpful error message with log path
//!
//! This ensures terminal isn't left in corrupted state after panics.

mod app;
mod data;
pub mod theme;
mod ui;

use std::io::{self, Write};
use std::panic;
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

/// Install panic hook that restores terminal and logs crash info.
fn install_panic_hook() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        // 1. Restore terminal state first (critical for usability)
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);

        // 2. Log crash to file
        let crash_log_path = dirs::home_dir()
            .map(|h| h.join(".novanet").join("crash.log"))
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp/novanet-crash.log"));

        if let Some(parent) = crash_log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let crash_info = format!(
            "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             CRASH: {}\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             {:#?}\n\n\
             Backtrace:\n{}\n",
            timestamp,
            panic_info,
            std::backtrace::Backtrace::force_capture()
        );

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&crash_log_path)
        {
            let _ = file.write_all(crash_info.as_bytes());
        }

        // 3. Print user-friendly message
        eprintln!("\n\x1b[1;31m💥 NovaNet TUI crashed!\x1b[0m");
        eprintln!("{panic_info}");
        eprintln!("\n\x1b[33mCrash log saved to: {}\x1b[0m", crash_log_path.display());
        eprintln!("\x1b[36mPlease report this issue with the crash log.\x1b[0m\n");

        // 4. Call original hook (for color_eyre integration)
        original_hook(panic_info);
    }));
}

/// Entry point: initialize terminal, run event loop, restore terminal.
pub async fn run(db: &Db, root_path: &Path) -> crate::Result<()> {
    // Install panic hook BEFORE entering raw mode
    install_panic_hook();

    // Setup terminal
    enable_raw_mode().map_err(crate::NovaNetError::Io)?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(crate::NovaNetError::Io)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(crate::NovaNetError::Io)?;

    // Run the app
    let result = run_app(&mut terminal, db, root_path).await;

    // Restore terminal (also done in panic hook, but needed for normal exit)
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
        let event = tokio::time::timeout(Duration::from_millis(100), event_stream.next()).await;

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
                    // Check for pending instance load (Data mode)
                    if let Some(kind_label) = app.take_pending_instance_load() {
                        if let Ok(instances) = TaxonomyTree::load_instances(db, &kind_label).await {
                            app.tree.set_instances(&kind_label, instances);
                        }
                    }

                    // Check for pending arcs load (Kind selected → load from Neo4j)
                    if let Some(kind_label) = app.take_pending_arcs_load() {
                        if let Ok(arcs) = TaxonomyTree::load_kind_arcs(db, &kind_label).await {
                            app.set_kind_arcs(arcs);
                        }
                    }

                    // Check for pending arc kind details load (ArcKind selected → load from Neo4j)
                    if let Some(arc_key) = app.take_pending_arc_kind_load() {
                        if let Ok(details) = TaxonomyTree::load_arc_kind_details(db, &arc_key).await
                        {
                            app.set_arc_kind_details(details);
                        }
                    }

                    // Check for pending Realm details load (Realm selected → load from Neo4j)
                    if let Some(realm_key) = app.take_pending_realm_load() {
                        if let Ok(details) = TaxonomyTree::load_realm_details(db, &realm_key).await {
                            app.set_realm_details(details);
                        }
                    }

                    // Check for pending Layer details load (Layer selected → load from Neo4j)
                    if let Some(layer_key) = app.take_pending_layer_load() {
                        if let Ok(details) = TaxonomyTree::load_layer_details(db, &layer_key).await {
                            app.set_layer_details(details);
                        }
                    }

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
