//! Read commands: `novanet data`, `novanet meta`, `novanet overlay`, `novanet query`.
//!
//! Each mode builds a Cypher query, executes it against Neo4j, and formats the
//! output as Table, JSON, or raw Cypher.

use crate::cypher::{self, CypherStatement};
use crate::db::Db;
use crate::facets::FacetFilter;
use crate::output::{self, NodeRow, OutputFormat, OverlayRow};
use tracing::instrument;

const DEFAULT_LIMIT: i64 = 500;

/// Mode 1: Data nodes only (WHERE NOT n:Meta).
#[instrument(skip(db))]
pub async fn run_data(db: &Db, format: OutputFormat) -> crate::Result<()> {
    let stmt = cypher::data_query(DEFAULT_LIMIT);
    dispatch(db, &stmt, format, extract_node_rows).await
}

/// Mode 2: Meta-graph only (MATCH (n:Meta)).
#[instrument(skip(db))]
pub async fn run_meta(db: &Db, format: OutputFormat) -> crate::Result<()> {
    let stmt = cypher::meta_query();
    dispatch(db, &stmt, format, extract_node_rows).await
}

/// Mode 3: Data + Meta overlay.
#[instrument(skip(db))]
pub async fn run_overlay(db: &Db, format: OutputFormat) -> crate::Result<()> {
    let stmt = cypher::overlay_query(DEFAULT_LIMIT);

    match format {
        OutputFormat::Cypher => {
            output::print_output(&output::format_cypher(&stmt));
        }
        OutputFormat::Table => {
            let rows = db.execute_statement(&stmt).await?;
            let overlay_rows = extract_overlay_rows(&rows);
            output::print_output(&output::format_overlay_table(&overlay_rows));
            eprintln!("{} row(s)", overlay_rows.len());
        }
        OutputFormat::Json => {
            let rows = db.execute_statement(&stmt).await?;
            let overlay_rows = extract_overlay_rows(&rows);
            output::print_output(&output::format_json(&overlay_rows));
        }
    }
    Ok(())
}

/// Mode 4: Faceted query driven by realm/layer/trait/edge-family/kind filters.
#[instrument(skip(db))]
pub async fn run_query(db: &Db, filter: FacetFilter, format: OutputFormat) -> crate::Result<()> {
    let stmt = cypher::faceted_query(&filter, DEFAULT_LIMIT);
    eprintln!(
        "novanet query (facets: {} active, resolved via meta-graph)",
        filter.active_count()
    );
    dispatch(db, &stmt, format, extract_node_rows).await
}

/// Shared dispatch: Cypher → display | Table/Json → execute + format.
async fn dispatch(
    db: &Db,
    stmt: &CypherStatement,
    format: OutputFormat,
    extractor: fn(&[neo4rs::Row]) -> Vec<NodeRow>,
) -> crate::Result<()> {
    match format {
        OutputFormat::Cypher => {
            output::print_output(&output::format_cypher(stmt));
        }
        OutputFormat::Table => {
            let rows = db.execute_statement(stmt).await?;
            let node_rows = extractor(&rows);
            output::print_output(&output::format_table(&node_rows));
            eprintln!("{} row(s)", node_rows.len());
        }
        OutputFormat::Json => {
            let rows = db.execute_statement(stmt).await?;
            let node_rows = extractor(&rows);
            output::print_output(&output::format_json(&node_rows));
        }
    }
    Ok(())
}

/// Extract NodeRow from neo4rs Row (used by data, meta, and query modes).
fn extract_node_rows(rows: &[neo4rs::Row]) -> Vec<NodeRow> {
    rows.iter()
        .map(|row| NodeRow {
            label: row.get::<String>("label").unwrap_or_default(),
            key: row.get::<String>("key").unwrap_or_default(),
            display_name: row.get::<String>("display_name").unwrap_or_default(),
            description: row.get::<String>("description").unwrap_or_default(),
        })
        .collect()
}

/// Extract OverlayRow from neo4rs Row (includes is_meta flag).
fn extract_overlay_rows(rows: &[neo4rs::Row]) -> Vec<OverlayRow> {
    rows.iter()
        .map(|row| OverlayRow {
            label: row.get::<String>("label").unwrap_or_default(),
            key: row.get::<String>("key").unwrap_or_default(),
            display_name: row.get::<String>("display_name").unwrap_or_default(),
            description: row.get::<String>("description").unwrap_or_default(),
            is_meta: row.get::<bool>("is_meta").unwrap_or(false),
        })
        .collect()
}
