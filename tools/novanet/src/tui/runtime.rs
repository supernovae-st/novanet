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
use crate::tui::events::{self, Action};
use crate::tui::tree::{MetaRow, TaxonomyTree};
use crate::tui::ui;

/// Messages from background tasks to the render loop.
enum BgMessage {
    MetaLoaded(Vec<MetaRow>),
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

    // Initial fetch: load meta-graph taxonomy
    spawn_meta_fetch(db, bg_tx.clone());

    // Render initial frame
    terminal
        .draw(|f| ui::render(f, &state))
        .map_err(crate::NovaNetError::Io)?;

    loop {
        // Check for background messages (non-blocking)
        while let Ok(msg) = bg_rx.try_recv() {
            match msg {
                BgMessage::MetaLoaded(rows) => {
                    let tree = TaxonomyTree::from_meta_rows(&rows);
                    state = AppState::ready(tree);
                }
                BgMessage::Error(e) => {
                    state = AppState::loading(format!("Error: {e}"));
                }
            }
            terminal
                .draw(|f| ui::render(f, &state))
                .map_err(crate::NovaNetError::Io)?;
        }

        // Poll for keyboard events (100ms timeout for responsiveness)
        if event::poll(std::time::Duration::from_millis(100)).map_err(crate::NovaNetError::Io)? {
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
                    Action::None => {}
                }
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
            Ok(rows) => {
                let _ = tx.send(BgMessage::MetaLoaded(rows)).await;
            }
            Err(e) => {
                let _ = tx.send(BgMessage::Error(e.to_string())).await;
            }
        }
    });
}

/// Fetch the Realm > Layer > Kind hierarchy from Neo4j.
async fn fetch_taxonomy(db: &Db) -> crate::Result<Vec<MetaRow>> {
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

    Ok(meta_rows)
}
