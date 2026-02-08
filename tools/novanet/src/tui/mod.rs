//! NovaNet TUI v2 — rebuilt from scratch for stability.
//!
//! Phase 1: Exploration (MVP)
//! - Header with mode tabs [1-4]
//! - Taxonomy tree navigation
//! - Detail panel with edges
//! - Status bar with stats
//!
//! Phase 2: Atlas Mode
//! - `5` Atlas mode with 6 interactive architecture views
//! - Spreading Activation, Knowledge Atoms, Generation Pipeline
//! - View Traversal, Page Composition, Realm Map
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
pub mod atlas;
mod audit;
mod data;
pub mod guide;
mod schema;
pub mod theme;
mod ui;
mod yaml;

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

/// Event polling timeout in milliseconds.
/// Short timeout enables smooth animation (spinners) during async loading.
const EVENT_TIMEOUT_MS: u64 = 100;

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
        eprintln!(
            "\n\x1b[33mCrash log saved to: {}\x1b[0m",
            crash_log_path.display()
        );
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
    // Load taxonomy tree from Neo4j with graceful error handling
    let tree = match TaxonomyTree::load(db).await {
        Ok(tree) => tree,
        Err(e) => {
            // Restore terminal before printing error
            crossterm::terminal::disable_raw_mode().ok();
            crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen).ok();

            eprintln!("\n\x1b[1;31m❌ Failed to load graph data from Neo4j:\x1b[0m");
            eprintln!("   {}\n", e);
            eprintln!("\x1b[1;33m💡 Troubleshooting:\x1b[0m");
            eprintln!("   • Check Neo4j is running: \x1b[36mdocker ps\x1b[0m");
            eprintln!("   • Verify credentials: \x1b[36mneo4j / novanetpassword\x1b[0m");
            eprintln!("   • Run seed: \x1b[36mpnpm infra:seed\x1b[0m");
            eprintln!("   • Check connection: \x1b[36mcargo run -- meta\x1b[0m\n");

            return Err(e);
        }
    };
    let root_str = root_path.display().to_string();
    let mut app = App::new(tree, root_str);
    let mut event_stream = EventStream::new();

    // Initial render
    terminal
        .draw(|f| ui::render(f, &mut app))
        .map_err(crate::NovaNetError::Io)?;

    loop {
        // Wait for events (non-blocking with timeout for future animations)
        let event =
            tokio::time::timeout(Duration::from_millis(EVENT_TIMEOUT_MS), event_stream.next())
                .await;

        match event {
            Ok(Some(Ok(Event::Key(key)))) => {
                // Ctrl+C always quits
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }
                // 'q' quits only when no overlay is open
                if key.code == KeyCode::Char('q') && !app.has_overlay_open() {
                    break;
                }

                // Handle other keys
                if app.handle_key(key) {
                    // Parallel load: instances + arcs (both triggered when Kind selected)
                    let instance_key = app.take_pending_instance_load();
                    let arcs_key = app.take_pending_arcs_load();

                    if instance_key.is_some() || arcs_key.is_some() {
                        let (inst_result, arcs_result) = tokio::join!(
                            async {
                                match &instance_key {
                                    Some(k) => Some(TaxonomyTree::load_instances(db, k).await),
                                    None => None,
                                }
                            },
                            async {
                                match &arcs_key {
                                    Some(k) => Some(TaxonomyTree::load_kind_arcs(db, k).await),
                                    None => None,
                                }
                            }
                        );

                        if let Some(k) = &instance_key {
                            match inst_result {
                                Some(Ok((instances, total))) => {
                                    app.tree.set_instances(k, instances, total);
                                }
                                Some(Err(e)) => {
                                    app.set_status_error(&format!("Load instances: {}", e));
                                }
                                None => {}
                            }
                        }
                        match arcs_result {
                            Some(Ok(arcs)) => {
                                app.set_kind_arcs(arcs);
                            }
                            Some(Err(e)) => {
                                app.set_status_error(&format!("Load arcs: {}", e));
                            }
                            None => {}
                        }
                    }

                    // Sequential loads for other details (typically only one fires at a time)
                    if let Some(arc_key) = app.take_pending_arc_kind_load() {
                        match TaxonomyTree::load_arc_kind_details(db, &arc_key).await {
                            Ok(details) => app.set_arc_kind_details(details),
                            Err(e) => app.set_status_error(&format!("Load arc: {}", e)),
                        }
                    }

                    if let Some(realm_key) = app.take_pending_realm_load() {
                        match TaxonomyTree::load_realm_details(db, &realm_key).await {
                            Ok(details) => app.set_realm_details(details),
                            Err(e) => app.set_status_error(&format!("Load realm: {}", e)),
                        }
                    }

                    if let Some(layer_key) = app.take_pending_layer_load() {
                        match TaxonomyTree::load_layer_details(db, &layer_key).await {
                            Ok(details) => app.set_layer_details(details),
                            Err(e) => app.set_status_error(&format!("Load layer: {}", e)),
                        }
                    }

                    // Parallel load: Atlas realm stats + pages list (both triggered when entering Atlas)
                    let load_atlas_stats = app.take_pending_atlas_realm_stats_load();
                    let load_atlas_pages = app.take_pending_atlas_pages_list_load();

                    if load_atlas_stats || load_atlas_pages {
                        let (stats_result, pages_result) = tokio::join!(
                            async {
                                if load_atlas_stats {
                                    Some(TaxonomyTree::load_atlas_realm_stats(db).await)
                                } else {
                                    None
                                }
                            },
                            async {
                                if load_atlas_pages {
                                    Some(TaxonomyTree::load_atlas_pages_list(db).await)
                                } else {
                                    None
                                }
                            }
                        );

                        match stats_result {
                            Some(Ok(stats)) => app.set_atlas_realm_stats(stats),
                            Some(Err(e)) => {
                                app.set_status_error(&format!("Load atlas stats: {}", e))
                            }
                            None => {}
                        }
                        match pages_result {
                            Some(Ok(pages)) => app.set_atlas_pages_list(pages),
                            Some(Err(e)) => {
                                app.set_status_error(&format!("Load atlas pages: {}", e))
                            }
                            None => {}
                        }
                    }

                    // Atlas page composition (individual load, depends on page_key)
                    if let Some((page_key, locale)) = app.take_pending_atlas_page_load() {
                        match TaxonomyTree::load_atlas_page_composition(db, &page_key, &locale)
                            .await
                        {
                            Ok(data) => app.set_atlas_page_composition(data),
                            Err(e) => app.set_status_error(&format!("Load page: {}", e)),
                        }
                    }

                    // Audit mode: load global audit stats
                    if app.take_pending_audit_load() {
                        match audit::load_audit_stats(db).await {
                            Ok(stats) => app.set_audit_stats(stats),
                            Err(e) => app.set_status_error(&format!("Load audit: {}", e)),
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
                // Timeout - increment tick for animations
                app.tick = app.tick.wrapping_add(1);

                // Clear expired status messages
                app.clear_expired_status();

                // Re-render if there's a pending load (animates spinner) or status message
                if app.has_pending_load() || app.status_message.is_some() {
                    terminal
                        .draw(|f| ui::render(f, &mut app))
                        .map_err(crate::NovaNetError::Io)?;
                }
            }
            _ => {
                // Ignore other events (mouse, focus, paste)
            }
        }
    }

    Ok(())
}
