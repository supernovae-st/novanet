//! Data backup command: `novanet data backup`.
//!
//! Backs up node data from Neo4j directly to version-controlled
//! `private-data/data/` directory as YAML files.
//!
//! Part of the Data Management System (simplified 2-command flow).
//!
//! **Flow**: Neo4j → query per class → transform → YAML files

use chrono::{DateTime, Utc};
use clap::Parser;
use std::path::PathBuf;
use tracing::instrument;

use crate::commands::data_common::{
    self, ExportDocument, DEFAULT_EXPORT_CLASSES,
};
use crate::db::Db;

// =============================================================================
// CLI ARGUMENTS
// =============================================================================

/// Back up node data from Neo4j to YAML files.
#[derive(Debug, Clone, Parser)]
#[command(
    about = "Save database content to version-controlled files",
    long_about = "Save database content to version-controlled files.\n\n\
        Data management workflow:\n\
        \n\
          Database (Neo4j)  ──backup──>  private-data/data/\n\
                             <──status──\n\
        \n\
          backup  Save database content to files   << YOU ARE HERE\n\
          status  Check what changed since last backup\n\
        \n\
        Reads your content (Entity, Page, Block...) from the database and\n\
        saves it as YAML files in private-data/data/.\n\n\
        Examples:\n  \
          novanet data backup                      # Save everything\n  \
          novanet data backup --class=Entity,Page  # Specific types only\n  \
          novanet data backup --dry-run            # Preview without saving"
)]
pub struct DataBackupArgs {
    /// Filter by node class (Entity, EntityNative, Page, PageNative, Block...)
    #[arg(long, value_delimiter = ',')]
    pub class: Option<Vec<String>>,

    /// Filter by project key (matches key prefix, e.g. "qrcode-ai")
    #[arg(long)]
    pub project: Option<String>,

    /// Filter by locale (for *Native classes, matches key suffix)
    #[arg(long)]
    pub locale: Option<String>,

    /// Only back up nodes updated since this ISO 8601 date
    #[arg(long)]
    pub since: Option<String>,

    /// Output directory (default: private-data/data)
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Preview without writing files
    #[arg(long)]
    pub dry_run: bool,
}

// =============================================================================
// TYPES
// =============================================================================

/// Summary of a single class backup.
#[derive(Debug)]
struct ClassBackupResult {
    count: usize,
}

// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

/// Run the data backup command.
#[instrument(skip(db))]
pub async fn run_data_backup(db: &Db, args: DataBackupArgs) -> crate::Result<()> {
    use crate::core::ux;

    // 1. Resolve output directory
    let output_dir = resolve_output_dir(&args)?;

    // 2. Validate class names
    let classes = resolve_classes(&args)?;

    // -- FLOW DIAGRAM --
    ux::print_data_flow(Some(1));

    // -- BANNER --
    let mut metadata: Vec<(&str, String)> = vec![
        ("From", "Database (Neo4j in Docker)".to_string()),
        ("To", format!("YAML files ({})", ux::fmt_path(&output_dir))),
    ];
    if let Some(ref project) = args.project {
        metadata.push(("Project", project.clone()));
    }
    if let Some(ref locale) = args.locale {
        metadata.push(("Locale", locale.clone()));
    }
    metadata.push(("Classes", classes.iter().map(|c| ux::class_label(c)).collect::<Vec<_>>().join("  ")));
    ux::print_banner(
        "NOVANET DATA BACKUP",
        "Save database content to version-controlled files",
        &metadata,
    );

    if args.dry_run {
        ux::print_dry_run_notice();
    }

    // 3. Back up each class
    let mut results = Vec::new();
    let backup_time = Utc::now();

    for class in &classes {
        let since = resolve_since(&args);

        let nodes = data_common::query_nodes(
            db,
            class,
            args.project.as_deref(),
            args.locale.as_deref(),
            since.as_ref(),
        )
        .await?;

        if nodes.is_empty() {
            ux::step_skip(&ux::class_label(class), "0 nodes");
            continue;
        }

        // Build export document
        let doc = ExportDocument {
            exported_at: backup_time.to_rfc3339(),
            class: class.clone(),
            project: args.project.clone(),
            locale: args.locale.clone(),
            nodes,
        };

        // Determine output file path
        let file_path = output_dir.join(format!("{class}.yaml"));

        if !args.dry_run {
            write_yaml(&file_path, &doc)?;
        }

        // -- STEP --
        ux::step_ok_count(&ux::class_label(class), doc.nodes.len(), &format!("-> {class}.yaml"));

        results.push(ClassBackupResult {
            count: doc.nodes.len(),
        });
    }

    // -- SUMMARY BOX --
    if results.is_empty() {
        eprintln!();
        ux::step_warn("No data", "No nodes found to back up");
        return Ok(());
    }

    let total: usize = results.iter().map(|r| r.count).sum();

    if args.dry_run {
        let summary_lines = vec![
            format!(
                "Would back up {} nodes across {} files",
                ux::fmt_count(total),
                results.len()
            ),
            format!("Target: {}", ux::fmt_path(&output_dir)),
        ];
        ux::print_summary_box(&summary_lines);
    } else {
        ux::print_sync_bar(total, total);
        let summary_lines = vec![
            format!(
                "Backup complete -- {} nodes -> {} files",
                ux::fmt_count(total),
                results.len()
            ),
            format!("Output: {}", ux::fmt_path(&output_dir)),
        ];
        ux::print_summary_ok(&summary_lines);
    }

    // -- NEXT STEP --
    if !args.dry_run {
        ux::print_next_step(
            "check for drift with",
            "novanet data status",
        );
    }

    Ok(())
}

// =============================================================================
// HELPERS
// =============================================================================

/// Resolve output directory: --output flag > {monorepo_root}/private-data/data
fn resolve_output_dir(args: &DataBackupArgs) -> crate::Result<PathBuf> {
    if let Some(ref output) = args.output {
        return Ok(output.clone());
    }

    let root = crate::config::resolve_root(None)?;
    Ok(root.join("private-data").join("data"))
}

/// Resolve which classes to back up.
fn resolve_classes(args: &DataBackupArgs) -> crate::Result<Vec<String>> {
    let classes = match &args.class {
        Some(classes) => classes.clone(),
        None => DEFAULT_EXPORT_CLASSES
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    data_common::validate_class_names(&classes)?;
    Ok(classes)
}

/// Determine the "since" timestamp for filtering.
fn resolve_since(args: &DataBackupArgs) -> Option<DateTime<Utc>> {
    args.since
        .as_ref()
        .and_then(|s| s.parse::<DateTime<Utc>>().ok())
}

/// Write a backup document to a YAML file.
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
        "# NovaNet Data Backup\n# Class: {} | Backed up: {}\n# Generated by `novanet data backup`\n",
        doc.class, doc.exported_at
    );

    std::fs::write(path, format!("{header}{yaml}"))?;

    Ok(())
}


// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_class_names_rejects_injection() {
        let args = DataBackupArgs {
            class: Some(vec!["Entity}DETACH DELETE n".to_string()]),
            project: None,
            locale: None,
            since: None,
            output: None,
            dry_run: false,
        };
        let result = resolve_classes(&args);
        assert!(result.is_err());
    }

    #[test]
    fn validate_class_names_accepts_pascal_case() {
        let args = DataBackupArgs {
            class: Some(vec![
                "Entity".to_string(),
                "EntityNative".to_string(),
                "PageNative".to_string(),
            ]),
            project: None,
            locale: None,
            since: None,
            output: None,
            dry_run: false,
        };
        let result = resolve_classes(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn default_classes_when_none_specified() {
        let args = DataBackupArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            output: None,
            dry_run: false,
        };
        let classes = resolve_classes(&args).unwrap();
        assert_eq!(classes.len(), DEFAULT_EXPORT_CLASSES.len());
        assert_eq!(classes[0], "Entity");
    }

    #[test]
    fn resolve_output_dir_with_flag() {
        let args = DataBackupArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            output: Some(PathBuf::from("/tmp/novanet-backup")),
            dry_run: false,
        };
        let dir = resolve_output_dir(&args).unwrap();
        assert_eq!(dir, PathBuf::from("/tmp/novanet-backup"));
    }

    #[test]
    fn resolve_output_dir_default() {
        let args = DataBackupArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            output: None,
            dry_run: false,
        };
        let dir = resolve_output_dir(&args).unwrap();
        assert!(dir.to_string_lossy().ends_with("private-data/data"));
    }

    #[test]
    fn resolve_since_explicit_flag() {
        let args = DataBackupArgs {
            class: None,
            project: None,
            locale: None,
            since: Some("2026-03-01T00:00:00Z".to_string()),
            output: None,
            dry_run: false,
        };

        let since = resolve_since(&args);
        assert!(since.is_some());
        assert_eq!(since.unwrap().date_naive().to_string(), "2026-03-01");
    }

    #[test]
    fn resolve_since_none_returns_none() {
        let args = DataBackupArgs {
            class: None,
            project: None,
            locale: None,
            since: None,
            output: None,
            dry_run: false,
        };

        let since = resolve_since(&args);
        assert!(since.is_none());
    }
}
