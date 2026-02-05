//! Locale commands: `novanet locale list`, `novanet locale import`.
//!
//! Lists locales with their knowledge satellite status from Neo4j,
//! and imports locale data from Cypher files.
//!
//! v10.4: Uses tiered knowledge model:
//! - Technical tier: Formatting, Slugification, Adaptation
//! - Style tier: Style
//! - Semantic tier: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet

use std::path::Path;

use crate::db::Db;
use crate::output::OutputFormat;

/// A locale row for display (v10.4 tiered knowledge model).
#[derive(Debug, Clone, serde::Serialize, tabled::Tabled)]
pub struct LocaleRow {
    pub key: String,
    pub display_name: String,
    pub language_code: String,
    pub country_code: String,
    /// Technical tier (Formatting, Slugification, Adaptation)
    pub technical: String,
    /// Style tier (Style)
    pub style: String,
    /// Semantic tier (TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet)
    pub semantic: String,
}

/// List all locales with their knowledge satellite status (v10.4 tiered model).
pub async fn run_list(db: &Db, format: OutputFormat) -> crate::Result<()> {
    let cypher = "\
MATCH (l:Locale)
// Technical tier
OPTIONAL MATCH (l)-[:HAS_FORMATTING]->(f:Formatting)
OPTIONAL MATCH (l)-[:HAS_SLUGIFICATION]->(sl:Slugification)
OPTIONAL MATCH (l)-[:HAS_ADAPTATION]->(a:Adaptation)
// Style tier
OPTIONAL MATCH (l)-[:HAS_STYLE]->(st:Style)
// Semantic tier (6 sets)
OPTIONAL MATCH (l)-[:HAS_TERMS]->(t:TermSet)
OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(ex:ExpressionSet)
OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(pa:PatternSet)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:CultureSet)
OPTIONAL MATCH (l)-[:HAS_TABOOS]->(tb:TabooSet)
OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(au:AudienceSet)
RETURN l.key AS key,
       l.display_name AS display_name,
       l.language_code AS language_code,
       l.country_code AS country_code,
       CASE WHEN f IS NOT NULL OR sl IS NOT NULL OR a IS NOT NULL THEN 'yes' ELSE '-' END AS technical,
       CASE WHEN st IS NOT NULL THEN 'yes' ELSE '-' END AS style,
       CASE WHEN t IS NOT NULL OR ex IS NOT NULL OR pa IS NOT NULL OR c IS NOT NULL OR tb IS NOT NULL OR au IS NOT NULL THEN 'yes' ELSE '-' END AS semantic
ORDER BY l.key";

    match format {
        OutputFormat::Cypher => {
            crate::output::print_output(cypher);
        }
        OutputFormat::Table => {
            let rows = db.execute(cypher).await?;
            let locale_rows = extract_locale_rows(&rows);
            let table = tabled::Table::new(&locale_rows)
                .with(tabled::settings::Style::rounded())
                .to_string();
            crate::output::print_output(&table);
            eprintln!("{} locale(s)", locale_rows.len());
        }
        OutputFormat::Json => {
            let rows = db.execute(cypher).await?;
            let locale_rows = extract_locale_rows(&rows);
            crate::output::print_output(&crate::output::format_json(&locale_rows));
        }
    }

    Ok(())
}

/// Import locale data from a Cypher file.
/// Reuses the statement splitter from the db module.
pub async fn run_import(db: &Db, file: &Path) -> crate::Result<()> {
    let content = std::fs::read_to_string(file).map_err(crate::NovaNetError::Io)?;
    let statements = crate::commands::db::split_cypher_statements(&content);

    if statements.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "locale import: file contains no Cypher statements".to_string(),
        ));
    }

    eprintln!(
        "Importing {} statement(s) from {}",
        statements.len(),
        file.display()
    );

    let start = std::time::Instant::now();
    let mut executed = 0u64;

    for stmt in &statements {
        db.execute(stmt).await.map_err(|e| {
            crate::NovaNetError::Validation(format!(
                "Failed at statement #{} in {}:\n  {:.120}\n  Error: {e}",
                executed + 1,
                file.display(),
                stmt,
            ))
        })?;
        executed += 1;
    }

    let elapsed = start.elapsed();
    eprintln!(
        "Import complete: {executed} statement(s) in {:.1}s",
        elapsed.as_secs_f64()
    );

    Ok(())
}

fn extract_locale_rows(rows: &[neo4rs::Row]) -> Vec<LocaleRow> {
    rows.iter()
        .map(|row| LocaleRow {
            key: row.get("key").unwrap_or_default(),
            display_name: row.get("display_name").unwrap_or_default(),
            language_code: row.get("language_code").unwrap_or_default(),
            country_code: row.get("country_code").unwrap_or_default(),
            technical: row.get("technical").unwrap_or_default(),
            style: row.get("style").unwrap_or_default(),
            semantic: row.get("semantic").unwrap_or_default(),
        })
        .collect()
}
