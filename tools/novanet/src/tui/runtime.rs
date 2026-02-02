//! Async runtime: event loop + Neo4j channel bridge.
//!
//! Architecture:
//! - Main thread: ratatui render loop + crossterm event polling
//! - Background task: Neo4j queries via mpsc channel
//! - Dirty flag: only re-render when state changes

use std::io;

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc;

use crate::db::Db;
use crate::tui::app::AppState;
use crate::tui::dashboard::{self, DashboardStats};
use crate::tui::detail;
use crate::tui::dialogs::DialogKind;
use crate::tui::events::{self, Action};
use crate::tui::onboarding::{self, OnboardingState};
use crate::tui::tree::{MetaRow, TaxonomyTree};
use crate::tui::ui;

/// Messages from background tasks to the render loop.
enum BgMessage {
    MetaLoaded(Vec<MetaRow>, Vec<String>),
    KindDetailLoaded(detail::KindDetail),
    StatsLoaded(DashboardStats),
    MutationSuccess(String),
    MutationError(String),
    Error(String),
}

/// Entry point: initialize terminal, run event loop, restore terminal.
pub async fn run(db: &Db) -> crate::Result<()> {
    // Setup terminal
    enable_raw_mode().map_err(crate::NovaNetError::Io)?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(crate::NovaNetError::Io)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(crate::NovaNetError::Io)?;

    // Run the app
    let result = run_app(&mut terminal, db).await;

    // Restore terminal
    disable_raw_mode().map_err(crate::NovaNetError::Io)?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(crate::NovaNetError::Io)?;
    terminal.show_cursor().map_err(crate::NovaNetError::Io)?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    db: &Db,
) -> crate::Result<()> {
    let mut state = AppState::loading("Connecting to Neo4j...");

    // Channel for background tasks
    let (bg_tx, mut bg_rx) = mpsc::channel::<BgMessage>(16);

    // Initial fetch: load meta-graph taxonomy + dashboard stats
    spawn_meta_fetch(db, bg_tx.clone());
    spawn_stats_fetch(db, bg_tx.clone());

    // Render initial frame
    terminal
        .draw(|f| ui::render(f, &state))
        .map_err(crate::NovaNetError::Io)?;

    loop {
        // Check for background messages (non-blocking)
        while let Ok(msg) = bg_rx.try_recv() {
            match msg {
                BgMessage::MetaLoaded(rows, edge_keys) => {
                    let tree = TaxonomyTree::from_meta_rows(&rows);
                    if matches!(state, AppState::Loading { .. }) {
                        // First load → boot animation
                        let (w, h) = terminal
                            .size()
                            .map_or((80u16, 24u16), |r| (r.width, r.height));
                        state = AppState::booting(tree, edge_keys, w, h);
                    } else {
                        // Subsequent loads (refresh) → go directly to Ready
                        state = AppState::ready(tree);
                        if let AppState::Ready { edge_kind_keys, .. } = &mut state {
                            *edge_kind_keys = edge_keys;
                        }
                    }
                }
                BgMessage::StatsLoaded(stats) => {
                    if let AppState::Ready {
                        dashboard_stats,
                        status,
                        ..
                    } = &mut state
                    {
                        *status = stats.summary();
                        *dashboard_stats = Some(stats);
                    }
                }
                BgMessage::KindDetailLoaded(kd) => {
                    if let AppState::Ready {
                        detail_lines,
                        kind_detail,
                        ..
                    } = &mut state
                    {
                        *detail_lines = detail::format_detail_lines(&kd);
                        *kind_detail = Some(Box::new(kd));
                    }
                }
                BgMessage::MutationSuccess(msg) => {
                    // Trigger screen shake on delete operations
                    let was_delete = matches!(
                        &state,
                        AppState::Ready {
                            dialog: Some(dlg), ..
                        } if matches!(
                            dlg.kind,
                            DialogKind::DeleteNode { .. } | DialogKind::DeleteRelation { .. }
                        )
                    );
                    if let AppState::Ready {
                        dialog,
                        status,
                        effects,
                        ..
                    } = &mut state
                    {
                        *dialog = None;
                        *status = msg;
                        if was_delete {
                            effects.trigger_shake();
                        }
                    }
                    // Re-fetch taxonomy + stats to reflect changes
                    spawn_meta_fetch(db, bg_tx.clone());
                    spawn_stats_fetch(db, bg_tx.clone());
                }
                BgMessage::MutationError(msg) => {
                    if let AppState::Ready {
                        dialog: Some(dlg), ..
                    } = &mut state
                    {
                        dlg.submitting = false;
                        dlg.error = Some(msg);
                    }
                }
                BgMessage::Error(e) => {
                    state = AppState::loading(format!("Error: {e}"));
                }
            }
            terminal
                .draw(|f| ui::render(f, &state))
                .map_err(crate::NovaNetError::Io)?;
        }

        // Adaptive poll interval: 33ms (~30fps) during animations, 100ms idle
        let poll_ms = match &state {
            AppState::Booting { .. } => 33,
            AppState::Ready { effects, .. } if effects.is_animating() => 33,
            _ => 100,
        };

        if event::poll(std::time::Duration::from_millis(poll_ms))
            .map_err(crate::NovaNetError::Io)?
        {
            if let Event::Key(key) = event::read().map_err(crate::NovaNetError::Io)? {
                match events::handle_key(&mut state, key) {
                    Action::Quit => break,
                    Action::Render => {
                        terminal
                            .draw(|f| ui::render(f, &state))
                            .map_err(crate::NovaNetError::Io)?;
                    }
                    Action::Fetch => {
                        // Trigger background fetch based on current mode
                        spawn_meta_fetch(db, bg_tx.clone());
                        terminal
                            .draw(|f| ui::render(f, &state))
                            .map_err(crate::NovaNetError::Io)?;
                    }
                    Action::FetchDetail(label) => {
                        spawn_detail_fetch(db, bg_tx.clone(), label);
                        terminal
                            .draw(|f| ui::render(f, &state))
                            .map_err(crate::NovaNetError::Io)?;
                    }
                    Action::SubmitDialog => {
                        if let AppState::Ready {
                            dialog: Some(dlg), ..
                        } = &mut state
                        {
                            dlg.submitting = true;
                            spawn_mutation(db, bg_tx.clone(), dlg);
                        }
                        terminal
                            .draw(|f| ui::render(f, &state))
                            .map_err(crate::NovaNetError::Io)?;
                    }
                    Action::CloseDialog => {
                        if let AppState::Ready { dialog, .. } = &mut state {
                            *dialog = None;
                        }
                        terminal
                            .draw(|f| ui::render(f, &state))
                            .map_err(crate::NovaNetError::Io)?;
                    }
                    Action::None => {}
                }
            }
        } else {
            // Timeout: advance animations (no key event received)
            let ticked = match &mut state {
                AppState::Booting { boot, .. } => {
                    boot.advance();
                    true
                }
                AppState::Ready { effects, tick, .. } => {
                    if effects.is_animating() {
                        *tick += 1;
                        effects.tick();
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if ticked {
                terminal
                    .draw(|f| ui::render(f, &state))
                    .map_err(crate::NovaNetError::Io)?;
            }
        }

        // Boot completion → transition Booting to Ready
        if let AppState::Booting { boot, .. } = &state {
            if boot.is_complete() {
                let old = std::mem::replace(&mut state, AppState::loading(""));
                if let AppState::Booting {
                    tree,
                    edge_kind_keys,
                    ..
                } = old
                {
                    state = AppState::ready(tree);
                    if let AppState::Ready {
                        edge_kind_keys: ek,
                        onboarding: ob,
                        ..
                    } = &mut state
                    {
                        *ek = edge_kind_keys;
                        // Show onboarding on first run
                        if onboarding::is_first_run() {
                            *ob = Some(OnboardingState::new_welcome());
                        }
                    }
                    // Re-fetch stats for dashboard (may have arrived during boot)
                    spawn_stats_fetch(db, bg_tx.clone());
                }
                terminal
                    .draw(|f| ui::render(f, &state))
                    .map_err(crate::NovaNetError::Io)?;
            }
        }
    }

    Ok(())
}

/// Spawn a background task to fetch meta-graph taxonomy from Neo4j.
fn spawn_meta_fetch(db: &Db, tx: mpsc::Sender<BgMessage>) {
    let db = db.clone();
    tokio::spawn(async move {
        match fetch_taxonomy(&db).await {
            Ok((rows, edge_keys)) => {
                let _ = tx.send(BgMessage::MetaLoaded(rows, edge_keys)).await;
            }
            Err(e) => {
                let _ = tx.send(BgMessage::Error(e.to_string())).await;
            }
        }
    });
}

/// Spawn a background task to fetch Kind detail from Neo4j.
fn spawn_detail_fetch(db: &Db, tx: mpsc::Sender<BgMessage>, label: String) {
    let db = db.clone();
    tokio::spawn(async move {
        match detail::fetch_kind_detail(&db, &label).await {
            Ok(kd) => {
                let _ = tx.send(BgMessage::KindDetailLoaded(kd)).await;
            }
            Err(e) => {
                let _ = tx.send(BgMessage::Error(format!("Detail: {e}"))).await;
            }
        }
    });
}

/// Spawn a background task to execute a CRUD mutation from a dialog.
fn spawn_mutation(db: &Db, tx: mpsc::Sender<BgMessage>, dlg: &crate::tui::dialogs::DialogState) {
    let db = db.clone();
    let kind = dlg.kind.clone();
    let field_values: Vec<(String, String)> = dlg
        .fields
        .iter()
        .map(|f| (f.label.clone(), f.value.clone()))
        .collect();

    tokio::spawn(async move {
        let get_field = |label: &str| -> String {
            field_values
                .iter()
                .find(|(l, _)| l == label)
                .map(|(_, v)| v.clone())
                .unwrap_or_default()
        };

        let result = match kind {
            DialogKind::CreateNode => {
                let kind_val = get_field("Kind");
                let key = get_field("Key");
                let display_name = get_field("Display Name");
                let desc = get_field("Description");
                let mut props = serde_json::Map::new();
                if !display_name.is_empty() {
                    props.insert(
                        "display_name".to_string(),
                        serde_json::Value::String(display_name),
                    );
                }
                if !desc.is_empty() {
                    props.insert("description".to_string(), serde_json::Value::String(desc));
                }
                let props_json = serde_json::Value::Object(props).to_string();
                crate::commands::node::run_create(&db, &kind_val, &key, &props_json).await
            }
            DialogKind::EditNode { ref key, .. } => {
                let props_json = get_field("Properties (JSON)");
                crate::commands::node::run_edit(&db, key, &props_json).await
            }
            DialogKind::DeleteNode { ref key, .. } => {
                crate::commands::node::run_delete(&db, key, true).await
            }
            DialogKind::CreateRelation { .. } => {
                let from = get_field("From Key");
                let to = get_field("To Key");
                let rt = get_field("Relation Type");
                crate::commands::relation::run_create(&db, &from, &to, &rt, "{}").await
            }
            DialogKind::DeleteRelation {
                ref from_key,
                ref to_key,
                ref rel_type,
            } => crate::commands::relation::run_delete(&db, from_key, to_key, rel_type).await,
        };

        match result {
            Ok(()) => {
                let _ = tx
                    .send(BgMessage::MutationSuccess(
                        "Operation successful".to_string(),
                    ))
                    .await;
            }
            Err(e) => {
                let _ = tx.send(BgMessage::MutationError(e.to_string())).await;
            }
        }
    });
}

/// Spawn a background task to fetch dashboard statistics from Neo4j.
fn spawn_stats_fetch(db: &Db, tx: mpsc::Sender<BgMessage>) {
    let db = db.clone();
    tokio::spawn(async move {
        match dashboard::fetch_stats(&db).await {
            Ok(stats) => {
                let _ = tx.send(BgMessage::StatsLoaded(stats)).await;
            }
            Err(e) => {
                let _ = tx.send(BgMessage::Error(format!("Stats: {e}"))).await;
            }
        }
    });
}

/// Fetch the Realm > Layer > Kind hierarchy from Neo4j.
async fn fetch_taxonomy(db: &Db) -> crate::Result<(Vec<MetaRow>, Vec<String>)> {
    let cypher = "\
MATCH (r:Realm)<-[:IN_REALM]-(k:Kind)-[:IN_LAYER]->(l:Layer)
RETURN 'Realm' AS type, r.key AS key, r.display_name AS display_name, null AS parent_key
UNION ALL
MATCH (l:Layer)<-[:IN_LAYER]-(k:Kind)-[:IN_REALM]->(r:Realm)
RETURN 'Layer' AS type, l.key AS key, l.display_name AS display_name, r.key AS parent_key
UNION ALL
MATCH (k:Kind)-[:IN_LAYER]->(l:Layer)
RETURN 'Kind' AS type, k.label AS key, coalesce(k.display_name, k.label) AS display_name, l.key AS parent_key
";

    // Deduplicate: UNION ALL may produce duplicates from multiple realm/layer combos
    let rows = db.execute(cypher).await?;
    let mut meta_rows = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for row in &rows {
        let label: String = row.get("type").unwrap_or_default();
        let key: String = row.get("key").unwrap_or_default();
        let display_name: String = row.get("display_name").unwrap_or_default();
        let parent_key: Option<String> = row.get("parent_key").ok();

        let dedup_key = format!("{label}:{key}:{}", parent_key.as_deref().unwrap_or(""));
        if seen.insert(dedup_key) {
            meta_rows.push(MetaRow {
                label,
                key,
                display_name,
                parent_key,
            });
        }
    }

    // Fetch EdgeKind keys for dialog dropdowns (non-fatal if missing)
    let edge_kind_keys = match db
        .execute("MATCH (ek:EdgeKind) RETURN ek.key AS key ORDER BY ek.key")
        .await
    {
        Ok(ek_rows) => ek_rows
            .iter()
            .filter_map(|r| {
                let k: String = r.get("key").ok()?;
                Some(k)
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    Ok((meta_rows, edge_kind_keys))
}
