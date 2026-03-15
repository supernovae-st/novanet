//! NovaNet TUI v2 — rebuilt from scratch for stability.
//!
//! Two navigation modes:
//! - Graph: Unified tree view (Realm > Layer > Class)
//! - Flow: Architecture diagrams (Schema + Pipeline)
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
pub mod cache;
pub mod clipboard;
#[path = "colors.generated.rs"]
pub mod colors;
mod data;
pub mod flow;
mod handlers;
pub mod icons;
pub mod palette;
mod schema;
pub mod theme;
mod ui;
pub mod unicode;
pub mod unified_types;
pub mod widgets;
#[cfg(test)]
pub mod testing;

use std::io::{self, Write};
use std::panic;
use std::time::Duration;

use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyModifiers,
};
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
        let _ = execute!(io::stdout(), DisableMouseCapture, LeaveAlternateScreen);

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
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).map_err(crate::NovaNetError::Io)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(crate::NovaNetError::Io)?;

    // Run the app
    let result = run_app(&mut terminal, db, root_path).await;

    // Restore terminal (also done in panic hook, but needed for normal exit)
    disable_raw_mode().map_err(crate::NovaNetError::Io)?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )
    .map_err(crate::NovaNetError::Io)?;
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
    // Pass root_path to load individual YAML files for content enrichment
    let tree = match TaxonomyTree::load(db, root_path).await {
        Ok(tree) => tree,
        Err(e) => {
            // Restore terminal before printing error
            crossterm::terminal::disable_raw_mode().ok();
            crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen).ok();

            eprintln!("\n\x1b[1;31m✗ Failed to load graph data from Neo4j:\x1b[0m");
            eprintln!("   {}\n", e);
            eprintln!("\x1b[1;33m◆ Troubleshooting:\x1b[0m");
            eprintln!("   • Check Neo4j is running: \x1b[36mdocker ps\x1b[0m");
            eprintln!("   • Verify credentials: \x1b[36mneo4j / novanetpassword\x1b[0m");
            eprintln!("   • Run seed: \x1b[36mpnpm infra:seed\x1b[0m");
            eprintln!("   • Check connection: \x1b[36mcargo run -- blueprint\x1b[0m\n");

            return Err(e);
        },
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
                    // Capture generation BEFORE any async work to detect stale results
                    let nav_gen = app.navigation_generation;

                    // PHASE 1: Fast instance loading (no arcs) + Class arcs
                    let instance_key = app.take_pending_instance_load();
                    let arcs_key = app.take_pending_arcs_load();

                    if instance_key.is_some() || arcs_key.is_some() {
                        let (inst_result, arcs_result) = tokio::join!(
                            async {
                                match &instance_key {
                                    // Use fast loading (no arc queries)
                                    Some(k) => Some(TaxonomyTree::load_instances_fast(db, k).await),
                                    None => None,
                                }
                            },
                            async {
                                match &arcs_key {
                                    Some(k) => Some(TaxonomyTree::load_class_arcs(db, k).await),
                                    None => None,
                                }
                            }
                        );

                        // Check if navigation changed during async load (stale detection)
                        if app.navigation_generation != nav_gen {
                            // User navigated away - discard stale results
                            continue;
                        }

                        if let Some(k) = &instance_key {
                            match inst_result {
                                Some(Ok((instances, total))) => {
                                    // Collect keys for background arc loading
                                    let keys: Vec<String> =
                                        instances.iter().map(|i| i.key.clone()).collect();
                                    app.tree.set_instances(k, instances, total);
                                    // Schedule arc loading in background
                                    if !keys.is_empty() {
                                        app.pending.instance_arcs = Some((k.clone(), keys));
                                    }
                                },
                                Some(Err(e)) => {
                                    app.set_status_error(&format!("Load instances: {}", e));
                                },
                                None => {},
                            }
                        }
                        match arcs_result {
                            Some(Ok(arcs)) => {
                                app.set_class_arcs(arcs);
                            },
                            Some(Err(e)) => {
                                app.set_status_error(&format!("Load arcs: {}", e));
                            },
                            None => {},
                        }
                    }

                    // Sequential loads for other details (typically only one fires at a time)
                    // Each checks generation to avoid applying stale results
                    if let Some(arc_key) = app.take_pending_arc_class_load() {
                        match TaxonomyTree::load_arc_class_details(db, &arc_key).await {
                            Ok(details) if app.navigation_generation == nav_gen => {
                                app.set_arc_class_details(details);
                            },
                            Ok(_) => {}, // Stale result, discard
                            Err(e) => app.set_status_error(&format!("Load arc: {}", e)),
                        }
                    }

                    if let Some(realm_key) = app.take_pending_realm_load() {
                        match TaxonomyTree::load_realm_details(db, &realm_key).await {
                            Ok(details) if app.navigation_generation == nav_gen => {
                                app.set_realm_details(details);
                            },
                            Ok(_) => {}, // Stale result, discard
                            Err(e) => app.set_status_error(&format!("Load realm: {}", e)),
                        }
                    }

                    if let Some(layer_key) = app.take_pending_layer_load() {
                        match TaxonomyTree::load_layer_details(db, &layer_key).await {
                            Ok(details) if app.navigation_generation == nav_gen => {
                                app.set_layer_details(details);
                            },
                            Ok(_) => {}, // Stale result, discard
                            Err(e) => app.set_status_error(&format!("Load layer: {}", e)),
                        }
                    }

                    // Entity category loading (triggered when Entity Class expanded in Data mode)
                    if app.take_pending_entity_categories_load() {
                        match TaxonomyTree::load_entity_categories(db).await {
                            Ok(categories) if app.navigation_generation == nav_gen => {
                                if categories.is_empty() {
                                    // No EntityCategory nodes in DB, fall back to flat Entity instances
                                    app.pending.instance = Some("Entity".to_string());
                                } else {
                                    // Auto-trigger loading of first category's instances
                                    if let Some(first_cat) = categories.first() {
                                        app.pending.category_instances =
                                            Some(first_cat.key.clone());
                                    }
                                }
                                app.tree.entity_categories = categories;
                            },
                            Ok(_) => {}, // Stale result, discard
                            Err(e) => app.set_status_error(&format!("Load categories: {}", e)),
                        }
                    }

                    // Category instances loading (triggered when EntityCategory expanded)
                    if let Some(category_key) = app.take_pending_category_instances_load() {
                        match TaxonomyTree::load_entities_by_category(db, &category_key).await {
                            Ok((instances, _total)) if app.navigation_generation == nav_gen => {
                                app.tree
                                    .entity_category_instances
                                    .insert(category_key, instances);
                                // Auto-load next category (chain loading for Entity expansion)
                                for cat in &app.tree.entity_categories {
                                    if !app.tree.entity_category_instances.contains_key(&cat.key) {
                                        app.pending.category_instances = Some(cat.key.clone());
                                        break;
                                    }
                                }
                            },
                            Ok(_) => {}, // Stale result, discard
                            Err(e) => {
                                app.set_status_error(&format!("Load category instances: {}", e))
                            },
                        }
                    }

                    // EntityNative entity groups loading (triggered when EntityNative Class expanded)
                    // Group by parent Entity instead of locale
                    if app.take_pending_entity_natives_load() {
                        match TaxonomyTree::load_entity_natives_by_entity(db).await {
                            Ok((groups, natives)) if app.navigation_generation == nav_gen => {
                                // Default all entity groups to collapsed
                                for group in &groups {
                                    app.tree
                                        .collapsed
                                        .insert(format!("entity_group:{}", group.entity_key));
                                }
                                app.tree.entity_native_groups = groups;
                                app.tree.entity_native_by_entity = natives;
                            },
                            Ok(_) => {
                                // Stale result, discard
                            },
                            Err(e) => app.set_status_error(&format!("Load entity natives: {}", e)),
                        }
                    }

                    terminal
                        .draw(|f| ui::render(f, &mut app))
                        .map_err(crate::NovaNetError::Io)?;
                }
            },
            Ok(Some(Ok(Event::Resize(_, _)))) => {
                // Re-render on resize
                terminal
                    .draw(|f| ui::render(f, &mut app))
                    .map_err(crate::NovaNetError::Io)?;
            },
            Ok(Some(Err(e))) => {
                // Log terminal event errors but don't crash - terminal corruption shouldn't stop TUI
                tracing::warn!("Event stream error: {}", e);
            },
            Ok(None) => {
                // Stream ended
                break;
            },
            Err(_) => {
                // Timeout - increment tick for animations
                app.tick = app.tick.wrapping_add(1);

                // Clear expired status messages
                app.clear_expired_status();

                // PHASE 2: Background arc loading for instances (deferred from key handler)
                // This runs on timeout so the UI renders with "[...]" first, showing loading state
                if let Some((class_key, keys)) = app.take_pending_instance_arcs_load() {
                    let nav_gen = app.navigation_generation;
                    match TaxonomyTree::load_instance_arcs(db, &class_key, keys).await {
                        Ok(arcs) if app.navigation_generation == nav_gen => {
                            app.tree.update_instance_arcs(&class_key, arcs);
                        },
                        Ok(_) => {
                            // Stale result - user navigated away, discard
                        },
                        Err(e) => {
                            app.set_status_error(&format!("Load arcs: {}", e));
                        },
                    }
                }

                // Re-render if there's a pending load (animates spinner) or status message
                if app.has_pending_load() || app.status_message.is_some() {
                    terminal
                        .draw(|f| ui::render(f, &mut app))
                        .map_err(crate::NovaNetError::Io)?;
                }
            },
            Ok(Some(Ok(Event::Mouse(event)))) => {
                // Handle mouse scroll on panels
                if app.handle_mouse(event) {
                    terminal
                        .draw(|f| ui::render(f, &mut app))
                        .map_err(crate::NovaNetError::Io)?;
                }
            },
            _ => {
                // Ignore other events (focus, paste, resize handled by terminal)
            },
        }
    }

    Ok(())
}
