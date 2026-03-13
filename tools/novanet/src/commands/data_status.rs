//! Data status command: `novanet data status`.
//!
//! Compares the current Neo4j state against previously backed-up YAML files,
//! reporting additions, modifications, and deletions per class.
//!
//! Part of the Data Management System (simplified 2-command flow).
//!
//! **Flow**: Load backup YAML → Query Neo4j → Compare by key → Report

use clap::Parser;
use colored::Colorize;
use indexmap::IndexMap;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::instrument;

use crate::commands::data_common::{self, ExportDocument, ExportedNode};
use crate::db::Db;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Fields to skip during property comparison (change frequently, not meaningful).
const SKIP_COMPARISON_FIELDS: &[&str] = &["created_at", "updated_at", "node_class"];

/// Maximum display length for property values in table output.
const DIFF_VALUE_DISPLAY_MAX_LEN: usize = 60;

// =============================================================================
// CLI ARGUMENTS
// =============================================================================

/// Compare Neo4j state against backed-up YAML files.
#[derive(Debug, Clone, Parser)]
#[command(
    about = "Check what changed since last backup",
    long_about = "Check what changed since last backup.\n\n\
        Data management workflow:\n\
        \n\
          Database (Neo4j)  ──backup──>  private-data/data/\n\
                             <──status──\n\
        \n\
          backup  Save database content to files\n\
          status  Check what changed since last backup   << YOU ARE HERE\n\
        \n\
        Compares your saved files against the live database to see\n\
        if anything was added, removed, or modified since the last backup.\n\n\
        Examples:\n  \
          novanet data status                  # Compare everything\n  \
          novanet data status --verbose        # Show property-level changes\n  \
          novanet data status --class=Entity   # Specific type only"
)]
pub struct DataStatusArgs {
    /// Filter by node class (Entity, EntityNative, Page...)
    #[arg(long, value_delimiter = ',')]
    pub class: Option<Vec<String>>,

    /// Output format
    #[arg(long, value_enum, default_value_t = StatusFormat::Table)]
    pub format: StatusFormat,

    /// Show property-level changes for modified nodes
    #[arg(long)]
    pub verbose: bool,

    /// Source directory for backed-up YAML (default: private-data/data)
    #[arg(long)]
    pub source: Option<PathBuf>,
}

/// Output format for status results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum StatusFormat {
    Table,
    Json,
}

// =============================================================================
// STATUS RESULT TYPES
// =============================================================================

/// Summary of differences for a single class.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClassStatusResult {
    pub class: String,
    pub neo4j_count: usize,
    pub yaml_count: usize,
    /// Keys only in Neo4j (not yet backed up / new since backup).
    pub only_in_neo4j: Vec<String>,
    /// Keys only in YAML (deleted from Neo4j since backup).
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

/// Run the data status command.
#[instrument(skip(db))]
pub async fn run_data_status(db: &Db, args: DataStatusArgs) -> crate::Result<()> {
    use crate::core::ux;

    let source_dir = resolve_source_dir(&args)?;

    if !source_dir.exists() {
        ux::step_warn("No backup", &format!("No backup directory at {}", ux::fmt_path(&source_dir)));
        ux::print_next_step("run", "novanet data backup");
        return Ok(());
    }

    // Discover classes to compare
    let classes = discover_classes(&args, &source_dir)?;

    if classes.is_empty() {
        ux::step_warn("No data", "No backed-up YAML files found to compare");
        return Ok(());
    }

    // -- FLOW DIAGRAM --
    ux::print_data_flow(Some(2));

    // -- BANNER --
    ux::print_banner(
        "NOVANET DATA STATUS",
        "Check what changed since last backup",
        &[
            ("Backup", format!("YAML files ({})", ux::fmt_path(&source_dir))),
            ("Database", "Neo4j in Docker".to_string()),
            ("Classes", classes.iter().map(|c| ux::class_label(c)).collect::<Vec<_>>().join("  ")),
        ],
    );

    // Compare each class
    let mut results = Vec::new();

    for class in &classes {
        let yaml_file = source_dir.join(format!("{class}.yaml"));
        if !yaml_file.exists() {
            continue;
        }

        let result = status_class(db, class, &yaml_file).await?;
        results.push(result);
    }

    // Output
    match args.format {
        StatusFormat::Table => print_table(&results, args.verbose),
        StatusFormat::Json => print_json(&results)?,
    }

    Ok(())
}

// =============================================================================
// HELPERS
// =============================================================================

/// Resolve source directory: --source flag > {monorepo_root}/private-data/data
fn resolve_source_dir(args: &DataStatusArgs) -> crate::Result<PathBuf> {
    if let Some(ref source) = args.source {
        return Ok(source.clone());
    }

    let root = crate::config::resolve_root(None)?;
    Ok(root.join("private-data").join("data"))
}

/// Discover which classes have YAML files or are specified by --class.
fn discover_classes(args: &DataStatusArgs, source_dir: &std::path::Path) -> crate::Result<Vec<String>> {
    if let Some(ref classes) = args.class {
        data_common::validate_class_names(classes)?;
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

/// Compare a single class: load YAML + query Neo4j + compare.
async fn status_class(
    db: &Db,
    class: &str,
    yaml_file: &std::path::Path,
) -> crate::Result<ClassStatusResult> {
    // 1. Load YAML backup
    let yaml_nodes = load_yaml_backup(yaml_file)?;

    // 2. Query Neo4j for all nodes of this class
    let neo4j_nodes = data_common::query_nodes(db, class, None, None, None).await?;

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

    Ok(ClassStatusResult {
        class: class.to_string(),
        neo4j_count: neo4j_map.len(),
        yaml_count: yaml_map.len(),
        only_in_neo4j,
        only_in_yaml,
        modified,
        in_sync,
    })
}

/// Load backed-up YAML file and return its nodes.
fn load_yaml_backup(path: &std::path::Path) -> crate::Result<Vec<ExportedNode>> {
    let content = std::fs::read_to_string(path)?;

    // Strip header comments before parsing
    let yaml_content: String = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");

    let doc: ExportDocument = serde_yaml::from_str(&yaml_content).map_err(|e| {
        crate::NovaNetError::Validation(format!(
            "Failed to parse YAML backup {}: {e}",
            path.display()
        ))
    })?;

    Ok(doc.nodes)
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

/// Print status results as a table.
fn print_table(results: &[ClassStatusResult], verbose: bool) {
    use crate::core::ux;

    if results.is_empty() {
        ux::step_warn("No data", "No differences to report");
        return;
    }

    // -- PER-CLASS STATUS (git-status style) --
    ux::print_section("Per-class comparison");

    for r in results {
        let is_clean = r.only_in_neo4j.is_empty()
            && r.only_in_yaml.is_empty()
            && r.modified.is_empty();

        if is_clean {
            ux::step_class_status(
                &r.class,
                &format!("{} nodes in sync", r.in_sync),
                true,
            );
        } else {
            let diff_str = ux::fmt_diff_counts(
                r.only_in_neo4j.len(),
                r.modified.len(),
                r.only_in_yaml.len(),
            );
            ux::step_class_status(
                &r.class,
                &diff_str,
                false,
            );
        }

        // Verbose: drill down into individual nodes
        if verbose {
            if !r.only_in_neo4j.is_empty() {
                for key in &r.only_in_neo4j {
                    eprintln!("        {} {}", "+".green(), key);
                }
            }
            if !r.modified.is_empty() {
                for m in &r.modified {
                    eprintln!("        {} {}", "~".yellow(), m.key);
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
                            "          {}: {} -> {}",
                            diff.property.dimmed(),
                            yaml_str.red(),
                            neo4j_str.green()
                        );
                    }
                }
            }
            if !r.only_in_yaml.is_empty() {
                for key in &r.only_in_yaml {
                    eprintln!("        {} {}", "-".red(), key);
                }
            }
        }
    }

    // -- SYNC BAR --
    let total_neo4j: usize = results.iter().map(|r| r.only_in_neo4j.len()).sum();
    let total_modified: usize = results.iter().map(|r| r.modified.len()).sum();
    let total_yaml: usize = results.iter().map(|r| r.only_in_yaml.len()).sum();
    let total_sync: usize = results.iter().map(|r| r.in_sync).sum();
    let total_all = total_sync + total_modified + total_neo4j + total_yaml;

    eprintln!();
    ux::print_sync_bar(total_sync, total_all);

    // -- SUMMARY BOX --
    let has_drift = total_neo4j > 0 || total_modified > 0 || total_yaml > 0;

    if has_drift {
        let summary_lines = vec![format!(
            "Drift detected -- {}",
            ux::fmt_diff_counts(total_neo4j, total_modified, total_yaml)
        )];
        ux::print_summary_warn(&summary_lines);
    } else {
        let summary_lines = vec![format!(
            "All {} nodes in sync across {} classes",
            ux::fmt_count(total_sync),
            results.len()
        )];
        ux::print_summary_ok(&summary_lines);
    }

    // -- NEXT STEP --
    if total_neo4j > 0 || total_modified > 0 {
        ux::print_next_step(
            "update backup with",
            "novanet data backup",
        );
    } else if !has_drift {
        ux::print_next_step(
            "everything is in sync, review with",
            "git diff private-data/",
        );
    }
}

/// Print status results as JSON.
fn print_json(results: &[ClassStatusResult]) -> crate::Result<()> {
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
        let args = DataStatusArgs {
            class: Some(vec!["Bad}Name".to_string()]),
            format: StatusFormat::Table,
            verbose: false,
            source: None,
        };
        let result = discover_classes(&args, std::path::Path::new("/tmp"));
        assert!(result.is_err());
    }

    #[test]
    fn load_yaml_backup_parses_document() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let content = r#"exported_at: "2026-03-13T14:30:22Z"
class: Entity
nodes:
  - key: qr-code
    display_name: QR Code
"#;
        std::fs::write(tmp.path(), content).unwrap();

        let nodes = load_yaml_backup(tmp.path()).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0].0.get("key").unwrap(),
            &Value::String("qr-code".into())
        );
    }
}
