//! Data diff command: `novanet data diff`.
//!
//! Compares the current Neo4j state against previously exported YAML files,
//! reporting additions, modifications, and deletions per class.
//!
//! Part of the Data Management System (Track A: YAML Governance).
//!
//! **Flow**: Load export YAML → Query Neo4j → Compare by key → Report

use clap::Parser;
use colored::Colorize;
use indexmap::IndexMap;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::instrument;

use crate::commands::data_export::{extra_fields, ExportDocument, ExportedNode};
use crate::db::Db;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Fields to skip during property comparison (change frequently, not meaningful).
const SKIP_COMPARISON_FIELDS: &[&str] = &["created_at", "updated_at", "node_class"];

/// Maximum display length for property values in table output.
const DIFF_VALUE_DISPLAY_MAX_LEN: usize = 60;

/// JSON fields that need parsing from Neo4j string representation.
const JSON_FIELDS: &[&str] = &["denomination_forms", "provenance"];

/// Regex pattern for valid Neo4j labels (PascalCase).
const LABEL_PATTERN: &str = r"^[A-Z][A-Za-z0-9]*$";

/// Standard properties present on all nodes.
const STANDARD_FIELDS: &[&str] = &[
    "key",
    "display_name",
    "content",
    "llm_context",
    "node_class",
];

/// Timestamp fields.
const TIMESTAMP_FIELDS: &[&str] = &["created_at", "updated_at"];

// =============================================================================
// CLI ARGUMENTS
// =============================================================================

/// Compare Neo4j state against exported YAML files.
#[derive(Debug, Clone, Parser)]
#[command(about = "Compare Neo4j state against exported YAML files")]
pub struct DataDiffArgs {
    /// Filter by node class (Entity, EntityNative, Page...)
    #[arg(long, value_delimiter = ',')]
    pub class: Option<Vec<String>>,

    /// Output format
    #[arg(long, value_enum, default_value_t = DiffFormat::Table)]
    pub format: DiffFormat,

    /// Show property-level changes for modified nodes
    #[arg(long)]
    pub verbose: bool,

    /// Source directory for exported YAML (default: ~/.novanet/export)
    #[arg(long)]
    pub source: Option<PathBuf>,
}

/// Output format for diff results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum DiffFormat {
    Table,
    Json,
}

// =============================================================================
// DIFF RESULT TYPES
// =============================================================================

/// Summary of differences for a single class.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClassDiffResult {
    pub class: String,
    pub neo4j_count: usize,
    pub yaml_count: usize,
    /// Keys only in Neo4j (not yet exported / new since export).
    pub only_in_neo4j: Vec<String>,
    /// Keys only in YAML (deleted from Neo4j since export).
    pub only_in_yaml: Vec<String>,
    /// Keys in both but with property differences.
    pub modified: Vec<ModifiedNode>,
    /// Keys in both and identical.
    pub in_sync: usize,
}

/// A node that exists in both Neo4j and YAML but has property differences.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ModifiedNode {
    pub key: String,
    pub changed_properties: Vec<PropertyDiff>,
}

/// A single property difference.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PropertyDiff {
    pub property: String,
    pub yaml_value: Option<Value>,
    pub neo4j_value: Option<Value>,
}

// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

/// Run the data diff command.
#[instrument(skip(db))]
pub async fn run_data_diff(db: &Db, args: DataDiffArgs) -> crate::Result<()> {
    let source_dir = resolve_source_dir(&args)?;

    if !source_dir.exists() {
        eprintln!(
            "{} No export directory found at {}",
            "warning:".yellow().bold(),
            source_dir.display()
        );
        eprintln!(
            "  Run {} first to create an export.",
            "novanet data export".cyan()
        );
        return Ok(());
    }

    // Discover classes to diff
    let classes = discover_classes(&args, &source_dir)?;

    if classes.is_empty() {
        eprintln!("{}", "No exported YAML files found to compare.".yellow());
        return Ok(());
    }

    // Diff each class
    let mut results = Vec::new();

    for class in &classes {
        let yaml_file = source_dir.join(format!("{class}.yaml"));
        if !yaml_file.exists() {
            continue;
        }

        let result = diff_class(db, class, &yaml_file).await?;
        results.push(result);
    }

    // Output
    match args.format {
        DiffFormat::Table => print_table(&results, args.verbose),
        DiffFormat::Json => print_json(&results)?,
    }

    Ok(())
}

// =============================================================================
// HELPERS
// =============================================================================

/// Resolve source directory: --source flag > ~/.novanet/export
fn resolve_source_dir(args: &DataDiffArgs) -> crate::Result<PathBuf> {
    if let Some(ref source) = args.source {
        return Ok(source.clone());
    }

    let home = dirs::home_dir().ok_or_else(|| {
        crate::NovaNetError::Validation("Cannot determine home directory".to_string())
    })?;
    Ok(home.join(".novanet").join("export"))
}

/// Discover which classes have YAML files or are specified by --class.
fn discover_classes(args: &DataDiffArgs, source_dir: &std::path::Path) -> crate::Result<Vec<String>> {
    if let Some(ref classes) = args.class {
        // Validate class names
        let re = regex::Regex::new(LABEL_PATTERN).expect("valid regex");
        for class in classes {
            if !re.is_match(class) {
                return Err(crate::NovaNetError::Validation(format!(
                    "Invalid class name '{class}': must be PascalCase"
                )));
            }
        }
        return Ok(classes.clone());
    }

    // Auto-discover from YAML files in source directory
    let mut classes = Vec::new();
    if let Ok(entries) = std::fs::read_dir(source_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "yaml") {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy().to_string();
                    // Skip non-PascalCase files (e.g. .checkpoint.json)
                    if name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                        classes.push(name);
                    }
                }
            }
        }
    }
    classes.sort();
    Ok(classes)
}

/// Diff a single class: load YAML + query Neo4j + compare.
async fn diff_class(
    db: &Db,
    class: &str,
    yaml_file: &std::path::Path,
) -> crate::Result<ClassDiffResult> {
    // 1. Load YAML export
    let yaml_nodes = load_yaml_export(yaml_file)?;

    // 2. Query Neo4j for all nodes of this class
    let neo4j_nodes = query_neo4j_nodes(db, class).await?;

    // 3. Build key-indexed maps
    let yaml_map: HashMap<String, &IndexMap<String, Value>> = yaml_nodes
        .iter()
        .filter_map(|node| {
            node.0
                .get("key")
                .and_then(|v| v.as_str())
                .map(|k| (k.to_string(), &node.0))
        })
        .collect();

    let neo4j_map: HashMap<String, &IndexMap<String, Value>> = neo4j_nodes
        .iter()
        .filter_map(|node| {
            node.0
                .get("key")
                .and_then(|v| v.as_str())
                .map(|k| (k.to_string(), &node.0))
        })
        .collect();

    // 4. Compare
    let mut only_in_neo4j = Vec::new();
    let mut only_in_yaml = Vec::new();
    let mut modified = Vec::new();
    let mut in_sync = 0;

    // Keys only in Neo4j
    for key in neo4j_map.keys() {
        if !yaml_map.contains_key(key) {
            only_in_neo4j.push(key.clone());
        }
    }
    only_in_neo4j.sort();

    // Keys only in YAML
    for key in yaml_map.keys() {
        if !neo4j_map.contains_key(key) {
            only_in_yaml.push(key.clone());
        }
    }
    only_in_yaml.sort();

    // Keys in both — compare properties
    for (key, yaml_props) in &yaml_map {
        if let Some(neo4j_props) = neo4j_map.get(key) {
            let diffs = compare_properties(yaml_props, neo4j_props);
            if diffs.is_empty() {
                in_sync += 1;
            } else {
                modified.push(ModifiedNode {
                    key: key.clone(),
                    changed_properties: diffs,
                });
            }
        }
    }
    modified.sort_by(|a, b| a.key.cmp(&b.key));

    Ok(ClassDiffResult {
        class: class.to_string(),
        neo4j_count: neo4j_map.len(),
        yaml_count: yaml_map.len(),
        only_in_neo4j,
        only_in_yaml,
        modified,
        in_sync,
    })
}

/// Load exported YAML file and return its nodes.
fn load_yaml_export(path: &std::path::Path) -> crate::Result<Vec<ExportedNode>> {
    let content = std::fs::read_to_string(path)?;

    // Strip header comments before parsing
    let yaml_content: String = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    let doc: ExportDocument = serde_yaml::from_str(&yaml_content).map_err(|e| {
        crate::NovaNetError::Validation(format!(
            "Failed to parse YAML export {}: {e}",
            path.display()
        ))
    })?;

    Ok(doc.nodes)
}

/// Query Neo4j for all nodes of a class, returning ExportedNode structs.
async fn query_neo4j_nodes(db: &Db, class: &str) -> crate::Result<Vec<ExportedNode>> {
    // Build RETURN clause from standard + extra fields
    let extra = extra_fields(class);
    let all_fields: Vec<&str> = STANDARD_FIELDS
        .iter()
        .chain(TIMESTAMP_FIELDS.iter())
        .chain(extra.iter())
        .copied()
        .collect();

    let return_clause: String = all_fields
        .iter()
        .map(|f| format!("n.{f} AS {f}"))
        .collect::<Vec<_>>()
        .join(", ");

    let cypher = format!("MATCH (n:{class}) RETURN {return_clause} ORDER BY n.key");

    let q = neo4rs::query(&cypher);
    let mut result = db
        .graph()
        .execute(q)
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?;

    let mut nodes = Vec::new();
    while let Some(row) = result
        .next()
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?
    {
        let mut map = IndexMap::new();
        for field in &all_fields {
            if let Ok(val) = row.get::<String>(field) {
                if !val.is_empty() {
                    // Parse JSON fields to match export YAML representation
                    if JSON_FIELDS.contains(field)
                        && (val.starts_with('{') || val.starts_with('['))
                    {
                        if let Ok(json) = serde_json::from_str::<Value>(&val) {
                            map.insert(field.to_string(), json);
                            continue;
                        }
                    }
                    map.insert(field.to_string(), Value::String(val));
                }
                continue;
            }
            if let Ok(val) = row.get::<i64>(field) {
                map.insert(field.to_string(), Value::Number(val.into()));
                continue;
            }
            if let Ok(val) = row.get::<bool>(field) {
                map.insert(field.to_string(), Value::Bool(val));
            }
        }
        nodes.push(ExportedNode(map));
    }

    Ok(nodes)
}

/// Compare two property maps, returning differences.
/// Skips timestamp fields (updated_at, created_at) since they change frequently.
fn compare_properties(
    yaml_props: &IndexMap<String, Value>,
    neo4j_props: &IndexMap<String, Value>,
) -> Vec<PropertyDiff> {
    let mut diffs = Vec::new();

    // Check all YAML properties against Neo4j
    for (key, yaml_val) in yaml_props {
        if SKIP_COMPARISON_FIELDS.contains(&key.as_str()) {
            continue;
        }

        match neo4j_props.get(key) {
            Some(neo4j_val) if neo4j_val != yaml_val => {
                diffs.push(PropertyDiff {
                    property: key.clone(),
                    yaml_value: Some(yaml_val.clone()),
                    neo4j_value: Some(neo4j_val.clone()),
                });
            }
            None => {
                diffs.push(PropertyDiff {
                    property: key.clone(),
                    yaml_value: Some(yaml_val.clone()),
                    neo4j_value: None,
                });
            }
            _ => {} // identical
        }
    }

    // Check Neo4j properties missing from YAML
    for (key, neo4j_val) in neo4j_props {
        if SKIP_COMPARISON_FIELDS.contains(&key.as_str()) {
            continue;
        }
        if !yaml_props.contains_key(key) {
            diffs.push(PropertyDiff {
                property: key.clone(),
                yaml_value: None,
                neo4j_value: Some(neo4j_val.clone()),
            });
        }
    }

    diffs
}

// =============================================================================
// OUTPUT
// =============================================================================

/// Print diff results as a table.
fn print_table(results: &[ClassDiffResult], verbose: bool) {
    if results.is_empty() {
        eprintln!("{}", "No differences to report.".yellow());
        return;
    }

    eprintln!();
    eprintln!("{}", "Data Diff: Neo4j vs Exported YAML".bold());
    eprintln!();

    for r in results {
        let status = if r.only_in_neo4j.is_empty()
            && r.only_in_yaml.is_empty()
            && r.modified.is_empty()
        {
            "in sync".green().to_string()
        } else {
            "drift detected".red().to_string()
        };

        eprintln!("  {} ({})", r.class.bold(), status);
        eprintln!(
            "    In Neo4j: {:>6}  │  In YAML: {:>6}",
            r.neo4j_count, r.yaml_count
        );
        eprintln!();

        if !r.only_in_neo4j.is_empty() {
            eprintln!(
                "    {} {} only in Neo4j (not yet exported)",
                format!("+{}", r.only_in_neo4j.len()).green(),
                if r.only_in_neo4j.len() == 1 { "node" } else { "nodes" }
            );
            if verbose {
                for key in &r.only_in_neo4j {
                    eprintln!("      {} {}", "+".green(), key);
                }
            }
        }

        if !r.modified.is_empty() {
            eprintln!(
                "    {} {} modified since last export",
                format!("~{}", r.modified.len()).yellow(),
                if r.modified.len() == 1 { "node" } else { "nodes" }
            );
            if verbose {
                for m in &r.modified {
                    eprintln!("      {} {}", "~".yellow(), m.key);
                    for diff in &m.changed_properties {
                        let yaml_str = diff
                            .yaml_value
                            .as_ref()
                            .map(|v| truncate_value(v, DIFF_VALUE_DISPLAY_MAX_LEN))
                            .unwrap_or_else(|| "(absent)".to_string());
                        let neo4j_str = diff
                            .neo4j_value
                            .as_ref()
                            .map(|v| truncate_value(v, DIFF_VALUE_DISPLAY_MAX_LEN))
                            .unwrap_or_else(|| "(absent)".to_string());
                        eprintln!(
                            "        {}: {} → {}",
                            diff.property.dimmed(),
                            yaml_str.red(),
                            neo4j_str.green()
                        );
                    }
                }
            }
        }

        if !r.only_in_yaml.is_empty() {
            eprintln!(
                "    {} {} only in YAML (deleted from Neo4j?)",
                format!("-{}", r.only_in_yaml.len()).red(),
                if r.only_in_yaml.len() == 1 { "node" } else { "nodes" }
            );
            if verbose {
                for key in &r.only_in_yaml {
                    eprintln!("      {} {}", "-".red(), key);
                }
            }
        }

        if r.in_sync > 0 {
            eprintln!(
                "    {} {} in sync",
                format!("={}", r.in_sync).dimmed(),
                if r.in_sync == 1 { "node" } else { "nodes" }
            );
        }

        eprintln!();
    }

    // Summary line
    let total_neo4j: usize = results.iter().map(|r| r.only_in_neo4j.len()).sum();
    let total_modified: usize = results.iter().map(|r| r.modified.len()).sum();
    let total_yaml: usize = results.iter().map(|r| r.only_in_yaml.len()).sum();

    if total_neo4j > 0 || total_modified > 0 {
        eprintln!(
            "  Run {} to capture Neo4j changes.",
            "novanet data export".cyan()
        );
    }
    if total_yaml > 0 {
        eprintln!(
            "  {} nodes in YAML but not in Neo4j. YAML is authoritative.",
            total_yaml.to_string().yellow()
        );
    }
}

/// Print diff results as JSON.
fn print_json(results: &[ClassDiffResult]) -> crate::Result<()> {
    let json = serde_json::to_string_pretty(results).map_err(|e| {
        crate::NovaNetError::Validation(format!("Failed to serialize JSON: {e}"))
    })?;
    println!("{json}");
    Ok(())
}

/// Truncate a JSON value to a maximum display length.
fn truncate_value(value: &Value, max_len: usize) -> String {
    let s = match value {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_identical_properties() {
        let mut a = IndexMap::new();
        a.insert("key".into(), Value::String("test".into()));
        a.insert("display_name".into(), Value::String("Test".into()));

        let b = a.clone();
        let diffs = compare_properties(&a, &b);
        assert!(diffs.is_empty());
    }

    #[test]
    fn compare_detects_modified_property() {
        let mut yaml = IndexMap::new();
        yaml.insert("key".into(), Value::String("test".into()));
        yaml.insert("display_name".into(), Value::String("Old Name".into()));

        let mut neo4j = IndexMap::new();
        neo4j.insert("key".into(), Value::String("test".into()));
        neo4j.insert("display_name".into(), Value::String("New Name".into()));

        let diffs = compare_properties(&yaml, &neo4j);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].property, "display_name");
    }

    #[test]
    fn compare_detects_missing_in_neo4j() {
        let mut yaml = IndexMap::new();
        yaml.insert("key".into(), Value::String("test".into()));
        yaml.insert("content".into(), Value::String("Some content".into()));

        let mut neo4j = IndexMap::new();
        neo4j.insert("key".into(), Value::String("test".into()));

        let diffs = compare_properties(&yaml, &neo4j);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].property, "content");
        assert!(diffs[0].neo4j_value.is_none());
    }

    #[test]
    fn compare_detects_new_in_neo4j() {
        let mut yaml = IndexMap::new();
        yaml.insert("key".into(), Value::String("test".into()));

        let mut neo4j = IndexMap::new();
        neo4j.insert("key".into(), Value::String("test".into()));
        neo4j.insert("content".into(), Value::String("New content".into()));

        let diffs = compare_properties(&yaml, &neo4j);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].property, "content");
        assert!(diffs[0].yaml_value.is_none());
    }

    #[test]
    fn compare_skips_timestamps() {
        let mut yaml = IndexMap::new();
        yaml.insert("key".into(), Value::String("test".into()));
        yaml.insert(
            "updated_at".into(),
            Value::String("2026-01-01T00:00:00Z".into()),
        );

        let mut neo4j = IndexMap::new();
        neo4j.insert("key".into(), Value::String("test".into()));
        neo4j.insert(
            "updated_at".into(),
            Value::String("2026-03-13T10:00:00Z".into()),
        );

        let diffs = compare_properties(&yaml, &neo4j);
        assert!(diffs.is_empty(), "timestamps should be skipped");
    }

    #[test]
    fn truncate_short_value_unchanged() {
        let v = Value::String("short".into());
        assert_eq!(truncate_value(&v, 60), "short");
    }

    #[test]
    fn truncate_long_value_ellipsis() {
        let v = Value::String("a".repeat(100));
        let result = truncate_value(&v, 10);
        assert_eq!(result.len(), 13); // 10 + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn discover_classes_validates_names() {
        let args = DataDiffArgs {
            class: Some(vec!["Bad}Name".to_string()]),
            format: DiffFormat::Table,
            verbose: false,
            source: None,
        };
        let result = discover_classes(&args, std::path::Path::new("/tmp"));
        assert!(result.is_err());
    }

    #[test]
    fn load_yaml_export_parses_document() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let content = r#"exported_at: "2026-03-13T14:30:22Z"
class: Entity
nodes:
  - key: qr-code
    display_name: QR Code
"#;
        std::fs::write(tmp.path(), content).unwrap();

        let nodes = load_yaml_export(tmp.path()).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0].0.get("key").unwrap(),
            &Value::String("qr-code".into())
        );
    }
}
