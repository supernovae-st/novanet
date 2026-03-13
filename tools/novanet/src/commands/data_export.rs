//! Data export command: `novanet data export`.
//!
//! Exports node data from Neo4j to YAML files for version-controlled governance.
//! Part of the Data Management System (Track A: YAML Governance).
//!
//! **Flow**: Neo4j → query per class → transform → YAML files → checkpoint
//!
//! Supports incremental export via checkpoint timestamps.

use chrono::{DateTime, Utc};
use clap::Parser;
use colored::Colorize;
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::instrument;

use crate::core::checkpoint::ExportCheckpoint;
use crate::db::Db;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Default exportable classes (org realm data nodes).
const DEFAULT_EXPORT_CLASSES: &[&str] = &[
    "Entity",
    "EntityNative",
    "Page",
    "PageNative",
    "Block",
    "BlockNative",
];

/// Standard properties present on all nodes.
const STANDARD_FIELDS: &[&str] = &[
    "key",
    "display_name",
    "content",
    "llm_context",
    "node_class",
];

/// Timestamp fields (handled separately for ISO formatting).
const TIMESTAMP_FIELDS: &[&str] = &["created_at", "updated_at"];

/// Regex pattern for valid Neo4j labels (PascalCase).
const LABEL_PATTERN: &str = r"^[A-Z][A-Za-z0-9]*$";

// =============================================================================
// CLI ARGUMENTS
// =============================================================================

/// Export node data from Neo4j to YAML files.
#[derive(Debug, Clone, Parser)]
#[command(about = "Export node data from Neo4j to YAML files")]
pub struct DataExportArgs {
    /// Filter by node class (Entity, EntityNative, Page, PageNative, Block...)
    #[arg(long, value_delimiter = ',')]
    pub class: Option<Vec<String>>,

    /// Filter by project key (matches key prefix, e.g. "qrcode-ai")
    #[arg(long)]
    pub project: Option<String>,

    /// Filter by locale (for *Native classes, matches key suffix)
    #[arg(long)]
    pub locale: Option<String>,

    /// Only export nodes updated since this ISO 8601 date
    #[arg(long)]
    pub since: Option<String>,

    /// Use checkpoint from last export (incremental)
    #[arg(long)]
    pub incremental: bool,

    /// Output directory (default: ~/.novanet/export)
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Preview without writing files
    #[arg(long)]
    pub dry_run: bool,
}

// =============================================================================
// TYPES
// =============================================================================

/// A single exported node with ordered properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedNode(pub IndexMap<String, serde_json::Value>);

/// YAML document written per class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDocument {
    /// ISO 8601 timestamp of the export.
    pub exported_at: String,
    /// Node class name.
    pub class: String,
    /// Optional project filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Optional locale filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Exported nodes.
    pub nodes: Vec<ExportedNode>,
}

/// Summary of a single class export.
#[derive(Debug)]
struct ClassExportResult {
    class: String,
    count: usize,
    file_path: PathBuf,
}

// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

/// Run the data export command.
#[instrument(skip(db))]
pub async fn run_data_export(db: &Db, args: DataExportArgs) -> crate::Result<()> {
    // 1. Resolve output directory
    let output_dir = resolve_output_dir(&args)?;

    // 2. Validate class names
    let classes = resolve_classes(&args)?;

    // 3. Load checkpoint if incremental
    let mut checkpoint = if args.incremental {
        ExportCheckpoint::load(&output_dir)?
    } else {
        ExportCheckpoint::new()
    };

    // 4. Progress bar
    let pb = ProgressBar::new(classes.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:30.cyan/blue}] {pos}/{len} {msg}")
            .expect("valid progress template")
            .progress_chars("█▓░"),
    );

    // 5. Export each class
    let mut results = Vec::new();
    let export_time = Utc::now();

    for class in &classes {
        pb.set_message(class.clone());

        // Determine "since" timestamp
        let since = resolve_since(&args, &checkpoint, class);

        // Build and execute query
        let rows = query_class(db, class, &args.project, &args.locale, since.as_ref()).await?;

        if rows.is_empty() {
            pb.inc(1);
            continue;
        }

        // Transform to exported nodes
        let nodes = transform_rows(&rows, class);

        // Build export document
        let doc = ExportDocument {
            exported_at: export_time.to_rfc3339(),
            class: class.clone(),
            project: args.project.clone(),
            locale: args.locale.clone(),
            nodes,
        };

        // Determine output file path
        let file_path = output_dir.join(format!("{class}.yaml"));

        if args.dry_run {
            eprintln!(
                "  {} {} ({} nodes → {})",
                "would write".yellow(),
                class,
                doc.nodes.len(),
                file_path.display()
            );
        } else {
            write_yaml(&file_path, &doc)?;
        }

        results.push(ClassExportResult {
            class: class.clone(),
            count: doc.nodes.len(),
            file_path,
        });

        // Update checkpoint
        checkpoint.record(
            class,
            doc.nodes.len(),
            args.project.as_deref(),
            args.locale.as_deref(),
        );

        pb.inc(1);
    }

    pb.finish_and_clear();

    // 6. Save checkpoint (unless dry run)
    if !args.dry_run && !results.is_empty() {
        checkpoint.save(&output_dir)?;
    }

    // 7. Print summary
    print_summary(&results, &output_dir, args.dry_run);

    Ok(())
}

// =============================================================================
// HELPERS
// =============================================================================

/// Resolve output directory: --output flag > ~/.novanet/export
fn resolve_output_dir(args: &DataExportArgs) -> crate::Result<PathBuf> {
    if let Some(ref output) = args.output {
        return Ok(output.clone());
    }

    let home = dirs::home_dir().ok_or_else(|| {
        crate::NovaNetError::Validation("Cannot determine home directory".to_string())
    })?;
    Ok(home.join(".novanet").join("export"))
}

/// Resolve which classes to export.
fn resolve_classes(args: &DataExportArgs) -> crate::Result<Vec<String>> {
    let classes = match &args.class {
        Some(classes) => classes.clone(),
        None => DEFAULT_EXPORT_CLASSES
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    // Validate each class name against injection
    let re = regex::Regex::new(LABEL_PATTERN)
        .expect("valid regex");

    for class in &classes {
        if !re.is_match(class) {
            return Err(crate::NovaNetError::Validation(format!(
                "Invalid class name '{class}': must be PascalCase (e.g. Entity, EntityNative)"
            )));
        }
    }

    Ok(classes)
}

/// Determine the "since" timestamp for a class export.
fn resolve_since(
    args: &DataExportArgs,
    checkpoint: &ExportCheckpoint,
    class: &str,
) -> Option<DateTime<Utc>> {
    // Explicit --since flag takes priority
    if let Some(ref since_str) = args.since {
        return since_str.parse::<DateTime<Utc>>().ok();
    }

    // Then checkpoint (if --incremental)
    if args.incremental {
        return checkpoint.get_since(class);
    }

    None
}

/// Additional fields to extract for specific classes.
pub fn extra_fields(class: &str) -> &'static [&'static str] {
    match class {
        "EntityNative" => &["entity_key", "locale", "denomination_forms", "provenance"],
        "PageNative" => &["page_key", "locale", "slug", "meta_title", "meta_description"],
        "BlockNative" => &["block_key", "locale", "block_type"],
        "Entity" => &["project_key"],
        "Page" => &["project_key"],
        "Block" => &["page_key", "block_type", "sort_order"],
        _ => &[],
    }
}

/// Build and execute the export Cypher query for a class.
async fn query_class(
    db: &Db,
    class: &str,
    project: &Option<String>,
    locale: &Option<String>,
    since: Option<&DateTime<Utc>>,
) -> crate::Result<Vec<neo4rs::Row>> {
    // Build RETURN clause from standard + extra fields
    let all_fields: Vec<&str> = STANDARD_FIELDS
        .iter()
        .chain(TIMESTAMP_FIELDS.iter())
        .chain(extra_fields(class).iter())
        .copied()
        .collect();

    let return_clause: String = all_fields
        .iter()
        .map(|f| format!("n.{f} AS {f}"))
        .collect::<Vec<_>>()
        .join(", ");

    // Build WHERE conditions (parameterized where possible)
    let mut conditions = Vec::new();

    if project.is_some() {
        conditions.push("n.key STARTS WITH $project".to_string());
    }

    if locale.is_some() {
        conditions.push("n.key ENDS WITH $locale".to_string());
    }

    if since.is_some() {
        conditions.push("n.updated_at > datetime($since)".to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Note: class name is validated via regex above, safe for interpolation.
    // Neo4j doesn't support parameterized labels.
    let cypher = format!(
        "MATCH (n:{class}) {where_clause} RETURN {return_clause} ORDER BY n.key"
    );

    // Build params for neo4rs
    let mut q = neo4rs::query(&cypher);
    if let Some(p) = project {
        q = q.param("project", p.as_str());
    }
    if let Some(l) = locale {
        // Locale is matched as key suffix: "@fr-FR"
        let locale_suffix = format!("@{l}");
        q = q.param("locale", locale_suffix.as_str());
    }
    if let Some(s) = since {
        q = q.param("since", s.to_rfc3339().as_str());
    }

    // Execute manually (we built the query ourselves, can't use Db::execute_with_params
    // because we already have a neo4rs::Query with params attached)
    let mut result = db
        .graph()
        .execute(q)
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?;

    let mut rows = Vec::new();
    while let Some(row) = result
        .next()
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?
    {
        rows.push(row);
    }

    Ok(rows)
}

/// Transform Neo4j rows into ExportedNode structs.
fn transform_rows(rows: &[neo4rs::Row], class: &str) -> Vec<ExportedNode> {
    let all_fields: Vec<&str> = STANDARD_FIELDS
        .iter()
        .chain(TIMESTAMP_FIELDS.iter())
        .chain(extra_fields(class).iter())
        .copied()
        .collect();

    rows.iter()
        .map(|row| {
            let mut map = IndexMap::new();

            for field in &all_fields {
                // Try string first (most common)
                if let Ok(val) = row.get::<String>(field) {
                    if !val.is_empty() {
                        // Check if it looks like JSON (denomination_forms, provenance)
                        if (*field == "denomination_forms" || *field == "provenance")
                            && val.starts_with('{') || val.starts_with('[')
                        {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&val) {
                                map.insert(field.to_string(), json);
                                continue;
                            }
                        }
                        map.insert(
                            field.to_string(),
                            serde_json::Value::String(val),
                        );
                    }
                    continue;
                }

                // Try integer
                if let Ok(val) = row.get::<i64>(field) {
                    map.insert(
                        field.to_string(),
                        serde_json::Value::Number(val.into()),
                    );
                    continue;
                }

                // Try boolean
                if let Ok(val) = row.get::<bool>(field) {
                    map.insert(field.to_string(), serde_json::Value::Bool(val));
                }

                // Skip if not found (field doesn't exist on this node)
            }

            ExportedNode(map)
        })
        .collect()
}

/// Write an export document to a YAML file.
fn write_yaml(path: &std::path::Path, doc: &ExportDocument) -> crate::Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let yaml = serde_yaml::to_string(doc).map_err(|e| {
        crate::NovaNetError::Validation(format!("Failed to serialize YAML: {e}"))
    })?;

    // Write with a header comment
    let header = format!(
        "# NovaNet Data Export\n# Class: {} | Exported: {}\n# Generated by `novanet data export`\n",
        doc.class, doc.exported_at
    );

    std::fs::write(path, format!("{header}{yaml}"))?;

    Ok(())
}

/// Print export summary.
fn print_summary(results: &[ClassExportResult], output_dir: &std::path::Path, dry_run: bool) {
    if results.is_empty() {
        eprintln!("{}", "No nodes found to export.".yellow());
        return;
    }

    let verb = if dry_run { "Would export" } else { "Exported" };
    let total: usize = results.iter().map(|r| r.count).sum();

    eprintln!();
    eprintln!("{}", format!("{verb} {total} nodes:").green().bold());
    for r in results {
        eprintln!(
            "  {} {} → {}",
            format!("{:>6}", r.count).cyan(),
            r.class,
            r.file_path.display()
        );
    }
    eprintln!();
    eprintln!("  Output: {}", output_dir.display());
    if !dry_run {
        eprintln!(
            "  Checkpoint: {}",
            output_dir.join(".checkpoint.json").display()
        );
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_class_names_rejects_injection() {
        let args = DataExportArgs {
            class: Some(vec!["Entity}DETACH DELETE n".to_string()]),
            project: None,
            locale: None,
            since: None,
            incremental: false,
            output: None,
            dry_run: false,
        };
        let result = resolve_classes(&args);
        assert!(result.is_err());
    }

    #[test]
    fn validate_class_names_accepts_pascal_case() {
        let args = DataExportArgs {
            class: Some(vec![
                "Entity".to_string(),
                "EntityNative".to_string(),
                "PageNative".to_string(),
            ]),
            project: None,
            locale: None,
            since: None,
            incremental: false,
            output: None,
            dry_run: false,
        };
        let result = resolve_classes(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn default_classes_when_none_specified() {
        let args = DataExportArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            incremental: false,
            output: None,
            dry_run: false,
        };
        let classes = resolve_classes(&args).unwrap();
        assert_eq!(classes.len(), DEFAULT_EXPORT_CLASSES.len());
        assert_eq!(classes[0], "Entity");
    }

    #[test]
    fn extra_fields_entity_native() {
        let fields = extra_fields("EntityNative");
        assert!(fields.contains(&"entity_key"));
        assert!(fields.contains(&"locale"));
        assert!(fields.contains(&"denomination_forms"));
    }

    #[test]
    fn extra_fields_unknown_class() {
        let fields = extra_fields("CustomClass");
        assert!(fields.is_empty());
    }

    #[test]
    fn resolve_output_dir_with_flag() {
        let args = DataExportArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            incremental: false,
            output: Some(PathBuf::from("/tmp/novanet-export")),
            dry_run: false,
        };
        let dir = resolve_output_dir(&args).unwrap();
        assert_eq!(dir, PathBuf::from("/tmp/novanet-export"));
    }

    #[test]
    fn resolve_output_dir_default() {
        let args = DataExportArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            incremental: false,
            output: None,
            dry_run: false,
        };
        let dir = resolve_output_dir(&args).unwrap();
        assert!(dir.to_string_lossy().ends_with(".novanet/export"));
    }

    #[test]
    fn export_document_serializes_to_yaml() {
        let mut node = IndexMap::new();
        node.insert(
            "key".to_string(),
            serde_json::Value::String("qr-code".to_string()),
        );
        node.insert(
            "display_name".to_string(),
            serde_json::Value::String("QR Code".to_string()),
        );

        let doc = ExportDocument {
            exported_at: "2026-03-13T14:30:22Z".to_string(),
            class: "Entity".to_string(),
            project: Some("qrcode-ai".to_string()),
            locale: None,
            nodes: vec![ExportedNode(node)],
        };

        let yaml = serde_yaml::to_string(&doc).unwrap();
        assert!(yaml.contains("class: Entity"));
        assert!(yaml.contains("qr-code"));
        assert!(yaml.contains("QR Code"));
    }

    #[test]
    fn resolve_since_explicit_flag_takes_priority() {
        let args = DataExportArgs {
            class: None,
            project: None,
            locale: None,
            since: Some("2026-03-01T00:00:00Z".to_string()),
            incremental: true,
            output: None,
            dry_run: false,
        };
        let mut checkpoint = ExportCheckpoint::new();
        checkpoint.record("Entity", 10, None, None);

        let since = resolve_since(&args, &checkpoint, "Entity");
        assert!(since.is_some());
        // Should use the explicit flag, not the checkpoint
        assert_eq!(since.unwrap().date_naive().to_string(), "2026-03-01");
    }

    #[test]
    fn resolve_since_incremental_uses_checkpoint() {
        let args = DataExportArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            incremental: true,
            output: None,
            dry_run: false,
        };
        let mut checkpoint = ExportCheckpoint::new();
        checkpoint.record("Entity", 10, None, None);

        let since = resolve_since(&args, &checkpoint, "Entity");
        assert!(since.is_some());
    }

    #[test]
    fn resolve_since_no_incremental_returns_none() {
        let args = DataExportArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            incremental: false,
            output: None,
            dry_run: false,
        };
        let checkpoint = ExportCheckpoint::new();

        let since = resolve_since(&args, &checkpoint, "Entity");
        assert!(since.is_none());
    }
}
