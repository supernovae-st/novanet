//! Data promote command: `novanet data promote`.
//!
//! Promotes exported YAML from `.novanet/export/` into version-controlled
//! `private-data/data/` directory, with optional diff review.
//!
//! Part of the Data Management System (Track A: YAML Governance).
//!
//! **Flow**: Load export → Diff against target → Copy/Merge → Summary

use clap::Parser;
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;
use tracing::instrument;

// =============================================================================
// CLI ARGUMENTS
// =============================================================================

/// Promote exported YAML to version-controlled private-data.
#[derive(Debug, Clone, Parser)]
#[command(about = "Promote exported YAML to version-controlled private-data")]
pub struct DataPromoteArgs {
    /// Preview without writing
    #[arg(long)]
    pub dry_run: bool,

    /// Source directory for exported YAML (default: ~/.novanet/export)
    #[arg(long)]
    pub source: Option<PathBuf>,

    /// Target directory for promoted data (default: private-data/data)
    #[arg(long)]
    pub target: Option<PathBuf>,

    /// Show unified diff for modified files
    #[arg(long, default_value_t = true)]
    pub show_diff: bool,
}

// =============================================================================
// RESULT TYPES
// =============================================================================

/// Summary of a single file promotion.
#[derive(Debug, Clone)]
struct FilePromoteResult {
    #[allow(dead_code)] // useful for debugging/logging
    class: String,
    status: PromoteStatus,
}

/// Status of each promoted file.
#[derive(Debug, Clone)]
enum PromoteStatus {
    /// File is new (not present in target).
    New,
    /// File exists and was modified.
    Updated,
    /// File is identical (no changes).
    Unchanged,
    /// Dry run — would have been written.
    DryRun(Box<PromoteStatus>),
}

// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

/// Run the data promote command.
#[instrument]
pub async fn run_data_promote(args: DataPromoteArgs) -> crate::Result<()> {
    let source_dir = resolve_source_dir(&args)?;
    let target_dir = resolve_target_dir(&args)?;

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

    // Discover exported YAML files
    let classes = discover_yaml_files(&source_dir)?;

    if classes.is_empty() {
        eprintln!("{}", "No exported YAML files found to promote.".yellow());
        return Ok(());
    }

    eprintln!(
        "{}",
        format!(
            "Promoting {} files from {} → {}",
            classes.len(),
            source_dir.display(),
            target_dir.display()
        )
        .bold()
    );

    if args.dry_run {
        eprintln!("{}", "  (dry run — no files will be written)".dimmed());
    }
    eprintln!();

    // Ensure target directory exists
    if !args.dry_run {
        std::fs::create_dir_all(&target_dir)?;
    }

    // Promote each file
    let mut results = Vec::new();

    for class in &classes {
        let source_file = source_dir.join(format!("{class}.yaml"));
        let target_file = target_dir.join(format!("{class}.yaml"));

        let result = promote_file(class, &source_file, &target_file, &args)?;
        results.push(result);
    }

    // Print summary
    print_summary(&results);

    Ok(())
}

// =============================================================================
// HELPERS
// =============================================================================

/// Resolve source directory: --source flag > ~/.novanet/export
fn resolve_source_dir(args: &DataPromoteArgs) -> crate::Result<PathBuf> {
    if let Some(ref source) = args.source {
        return Ok(source.clone());
    }

    let home = dirs::home_dir().ok_or_else(|| {
        crate::NovaNetError::Validation("Cannot determine home directory".to_string())
    })?;
    Ok(home.join(".novanet").join("export"))
}

/// Resolve target directory: --target flag > {monorepo_root}/private-data/data
fn resolve_target_dir(args: &DataPromoteArgs) -> crate::Result<PathBuf> {
    if let Some(ref target) = args.target {
        return Ok(target.clone());
    }

    // Default: private-data/data relative to monorepo root
    let root = crate::config::resolve_root(None)?;
    Ok(root.join("private-data").join("data"))
}

/// Discover PascalCase YAML files in the source directory.
fn discover_yaml_files(source_dir: &std::path::Path) -> crate::Result<Vec<String>> {
    let mut classes = Vec::new();
    if let Ok(entries) = std::fs::read_dir(source_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "yaml") {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy().to_string();
                    // Only PascalCase files (skip .checkpoint.json, etc.)
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

/// Promote a single file: compare source vs target, copy if different.
fn promote_file(
    class: &str,
    source_file: &std::path::Path,
    target_file: &std::path::Path,
    args: &DataPromoteArgs,
) -> crate::Result<FilePromoteResult> {
    let source_content = std::fs::read_to_string(source_file)?;

    // Check if target exists
    if !target_file.exists() {
        // New file
        if args.show_diff {
            eprintln!("  {} {}", "+".green().bold(), class.bold());
            eprintln!("    {} (new file, {} bytes)", "NEW".green(), source_content.len());
        }

        let status = if args.dry_run {
            PromoteStatus::DryRun(Box::new(PromoteStatus::New))
        } else {
            if let Some(parent) = target_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(target_file, &source_content)?;
            PromoteStatus::New
        };

        return Ok(FilePromoteResult {
            class: class.to_string(),
            status,
        });
    }

    // File exists — compare content
    let target_content = std::fs::read_to_string(target_file)?;

    if source_content == target_content {
        if args.show_diff {
            eprintln!("  {} {}", "=".dimmed(), class.dimmed());
        }
        return Ok(FilePromoteResult {
            class: class.to_string(),
            status: PromoteStatus::Unchanged,
        });
    }

    // Modified — show diff
    if args.show_diff {
        eprintln!("  {} {}", "~".yellow().bold(), class.bold());
        print_unified_diff(&target_content, &source_content);
    }

    let status = if args.dry_run {
        PromoteStatus::DryRun(Box::new(PromoteStatus::Updated))
    } else {
        std::fs::write(target_file, &source_content)?;
        PromoteStatus::Updated
    };

    Ok(FilePromoteResult {
        class: class.to_string(),
        status,
    })
}

/// Print a unified diff between old and new content.
fn print_unified_diff(old: &str, new: &str) {
    let diff = TextDiff::from_lines(old, new);

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                eprint!("    {}", format!("-{change}").red());
            }
            ChangeTag::Insert => {
                eprint!("    {}", format!("+{change}").green());
            }
            ChangeTag::Equal => {
                // Skip equal lines to keep output compact
            }
        }
    }
    eprintln!();
}

/// Print summary of all promotions.
fn print_summary(results: &[FilePromoteResult]) {
    let new_count = results
        .iter()
        .filter(|r| matches!(r.status, PromoteStatus::New)
            || matches!(&r.status, PromoteStatus::DryRun(s) if matches!(**s, PromoteStatus::New)))
        .count();
    let updated_count = results
        .iter()
        .filter(|r| matches!(r.status, PromoteStatus::Updated)
            || matches!(&r.status, PromoteStatus::DryRun(s) if matches!(**s, PromoteStatus::Updated)))
        .count();
    let unchanged_count = results
        .iter()
        .filter(|r| matches!(r.status, PromoteStatus::Unchanged))
        .count();

    eprintln!();
    eprintln!(
        "  {} {} new, {} {} updated, {} {} unchanged",
        new_count.to_string().green(),
        if new_count == 1 { "file" } else { "files" },
        updated_count.to_string().yellow(),
        if updated_count == 1 { "file" } else { "files" },
        unchanged_count.to_string().dimmed(),
        if unchanged_count == 1 { "file" } else { "files" },
    );

    let is_dry_run = results
        .iter()
        .any(|r| matches!(r.status, PromoteStatus::DryRun(_)));
    if is_dry_run {
        eprintln!(
            "  {}",
            "(dry run — no files written, re-run without --dry-run to promote)".dimmed()
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
    fn discover_yaml_files_filters_non_pascal() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Entity.yaml"), "test").unwrap();
        std::fs::write(tmp.path().join("lowercase.yaml"), "test").unwrap();
        std::fs::write(tmp.path().join(".checkpoint.json"), "test").unwrap();

        let classes = discover_yaml_files(tmp.path()).unwrap();
        assert_eq!(classes, vec!["Entity"]);
    }

    #[test]
    fn discover_yaml_files_sorts_alphabetically() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Page.yaml"), "test").unwrap();
        std::fs::write(tmp.path().join("Block.yaml"), "test").unwrap();
        std::fs::write(tmp.path().join("Entity.yaml"), "test").unwrap();

        let classes = discover_yaml_files(tmp.path()).unwrap();
        assert_eq!(classes, vec!["Block", "Entity", "Page"]);
    }

    #[test]
    fn promote_new_file() {
        let source_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        let source_file = source_dir.path().join("Entity.yaml");
        let target_file = target_dir.path().join("Entity.yaml");
        std::fs::write(&source_file, "key: test\n").unwrap();

        let args = DataPromoteArgs {
            dry_run: false,
            source: None,
            target: None,
            show_diff: false,
        };

        let result = promote_file("Entity", &source_file, &target_file, &args).unwrap();
        assert!(matches!(result.status, PromoteStatus::New));
        assert!(target_file.exists());
        assert_eq!(std::fs::read_to_string(&target_file).unwrap(), "key: test\n");
    }

    #[test]
    fn promote_unchanged_file() {
        let source_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        let source_file = source_dir.path().join("Entity.yaml");
        let target_file = target_dir.path().join("Entity.yaml");
        let content = "key: test\ndisplay_name: Test\n";
        std::fs::write(&source_file, content).unwrap();
        std::fs::write(&target_file, content).unwrap();

        let args = DataPromoteArgs {
            dry_run: false,
            source: None,
            target: None,
            show_diff: false,
        };

        let result = promote_file("Entity", &source_file, &target_file, &args).unwrap();
        assert!(matches!(result.status, PromoteStatus::Unchanged));
    }

    #[test]
    fn promote_updated_file() {
        let source_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        let source_file = source_dir.path().join("Entity.yaml");
        let target_file = target_dir.path().join("Entity.yaml");
        std::fs::write(&source_file, "key: test\ndisplay_name: Updated\n").unwrap();
        std::fs::write(&target_file, "key: test\ndisplay_name: Original\n").unwrap();

        let args = DataPromoteArgs {
            dry_run: false,
            source: None,
            target: None,
            show_diff: false,
        };

        let result = promote_file("Entity", &source_file, &target_file, &args).unwrap();
        assert!(matches!(result.status, PromoteStatus::Updated));
        assert_eq!(
            std::fs::read_to_string(&target_file).unwrap(),
            "key: test\ndisplay_name: Updated\n"
        );
    }

    #[test]
    fn promote_dry_run_does_not_write() {
        let source_dir = tempfile::tempdir().unwrap();
        let target_dir = tempfile::tempdir().unwrap();

        let source_file = source_dir.path().join("Entity.yaml");
        let target_file = target_dir.path().join("Entity.yaml");
        std::fs::write(&source_file, "key: test\n").unwrap();

        let args = DataPromoteArgs {
            dry_run: true,
            source: None,
            target: None,
            show_diff: false,
        };

        let result = promote_file("Entity", &source_file, &target_file, &args).unwrap();
        assert!(matches!(result.status, PromoteStatus::DryRun(_)));
        assert!(!target_file.exists());
    }

    #[test]
    fn unified_diff_shows_changes() {
        // Just verify it doesn't panic
        print_unified_diff(
            "line1\nline2\nline3\n",
            "line1\nmodified\nline3\n",
        );
    }
}
