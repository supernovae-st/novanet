//! Locale commands: `novanet locale list`, `novanet locale import`.
//!
//! Lists locales with their knowledge satellite status from Neo4j,
//! and imports locale data from Cypher files.

use std::path::Path;

use crate::db::Db;
use crate::output::OutputFormat;

/// A locale row for display.
#[derive(Debug, Clone, serde::Serialize, tabled::Tabled)]
pub struct LocaleRow {
    pub key: String,
    pub display_name: String,
    pub language_code: String,
    pub country_code: String,
    pub identity: String,
    pub voice: String,
    pub culture: String,
    pub market: String,
    pub lexicon: String,
}

/// List all locales with their knowledge satellite status.
pub async fn run_list(db: &Db, format: OutputFormat) -> crate::Result<()> {
    let cypher = "\
MATCH (l:Locale)
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(i:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(m:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(x:LocaleLexicon)
RETURN l.key AS key,
       l.display_name AS display_name,
       l.language_code AS language_code,
       l.country_code AS country_code,
       CASE WHEN i IS NOT NULL THEN 'yes' ELSE '-' END AS identity,
       CASE WHEN v IS NOT NULL THEN 'yes' ELSE '-' END AS voice,
       CASE WHEN c IS NOT NULL THEN 'yes' ELSE '-' END AS culture,
       CASE WHEN m IS NOT NULL THEN 'yes' ELSE '-' END AS market,
       CASE WHEN x IS NOT NULL THEN 'yes' ELSE '-' END AS lexicon
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
            identity: row.get("identity").unwrap_or_default(),
            voice: row.get("voice").unwrap_or_default(),
            culture: row.get("culture").unwrap_or_default(),
            market: row.get("market").unwrap_or_default(),
            lexicon: row.get("lexicon").unwrap_or_default(),
        })
        .collect()
}
