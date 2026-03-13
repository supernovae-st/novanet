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

use crate::core::ux;

// =============================================================================
// CLI ARGUMENTS
// =============================================================================

/// Promote exported YAML to version-controlled private-data.
#[derive(Debug, Clone, Parser)]
#[command(
    about = "Copy local files to git for version control",
    long_about = "Copy local files to git for version control.\n\n\
        Step 3 of 3 in the data management workflow:\n\
        \n\
          Database (Neo4j)  ──1──>  Local backup  ──3──>  Git repo\n\
                            <──2──\n\
        \n\
          1. export   Save database content to local files\n\
          2. diff     Check what changed since last export\n\
          3. promote  Copy local files to git for version control   << YOU ARE HERE\n\
        \n\
        Copies your saved files from ~/.novanet/export/ into private-data/data/\n\
        so they can be tracked with git (committed, pushed, reviewed).\n\n\
        Examples:\n  \
          novanet data promote              # Copy all files to git folder\n  \
          novanet data promote --dry-run    # Preview without writing\n  \
          novanet data promote --no-diff    # Skip diff output"
)]
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
        ux::step_warn(
            "No export",
            &format!("No export directory at {}", ux::fmt_path(&source_dir)),
        );
        ux::print_next_step("run", "novanet data export");
        return Ok(());
    }

    // Discover exported YAML files
    let classes = discover_yaml_files(&source_dir)?;

    if classes.is_empty() {
        ux::step_warn("No data", "No exported YAML files found to promote");
        return Ok(());
    }

    // -- BANNER --
    ux::print_banner(
        "NOVANET DATA PROMOTE  (step 3 of 3)",
        "Copy local files to git for version control",
        &[
            ("From", format!("Local backup ({})", ux::fmt_path(&source_dir))),
            ("To", format!("Git repo ({})", ux::fmt_path(&target_dir))),
            ("Files", format!("{} to copy", classes.len())),
        ],
    );

    if args.dry_run {
        ux::print_dry_run_notice();
    }

    // Ensure target directory exists
    if !args.dry_run {
        std::fs::create_dir_all(&target_dir)?;
    }

    // -- STEP-BY-STEP --
    let mut results = Vec::new();

    for class in &classes {
        let source_file = source_dir.join(format!("{class}.yaml"));
        let target_file = target_dir.join(format!("{class}.yaml"));

        let result = promote_file(class, &source_file, &target_file, &args)?;
        results.push(result);
    }

    // -- SUMMARY BOX --
    print_summary(&results, &target_dir, args.dry_run);

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
        let status = if args.dry_run {
            PromoteStatus::DryRun(Box::new(PromoteStatus::New))
        } else {
            if let Some(parent) = target_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(target_file, &source_content)?;
            PromoteStatus::New
        };

        ux::step_ok(
            class,
            &format!("new file ({} bytes)", ux::fmt_count(source_content.len())),
        );

        return Ok(FilePromoteResult {
            class: class.to_string(),
            status,
        });
    }

    // File exists — compare content
    let target_content = std::fs::read_to_string(target_file)?;

    if source_content == target_content {
        ux::step_skip(class, "unchanged");
        return Ok(FilePromoteResult {
            class: class.to_string(),
            status: PromoteStatus::Unchanged,
        });
    }

    // Modified — write + show diff
    let status = if args.dry_run {
        PromoteStatus::DryRun(Box::new(PromoteStatus::Updated))
    } else {
        std::fs::write(target_file, &source_content)?;
        PromoteStatus::Updated
    };

    ux::step_warn(class, "updated (content changed)");
    if args.show_diff {
        print_unified_diff(&target_content, &source_content);
    }

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
fn print_summary(results: &[FilePromoteResult], target_dir: &std::path::Path, dry_run: bool) {
    let new_count = results
        .iter()
        .filter(|r| {
            matches!(r.status, PromoteStatus::New)
                || matches!(&r.status, PromoteStatus::DryRun(s) if matches!(**s, PromoteStatus::New))
        })
        .count();
    let updated_count = results
        .iter()
        .filter(|r| {
            matches!(r.status, PromoteStatus::Updated)
                || matches!(&r.status, PromoteStatus::DryRun(s) if matches!(**s, PromoteStatus::Updated))
        })
        .count();
    let unchanged_count = results
        .iter()
        .filter(|r| matches!(r.status, PromoteStatus::Unchanged))
        .count();

    let verb = if dry_run { "Would promote" } else { "Promote complete" };
    let changed = new_count + updated_count;

    let mut summary_lines = vec![format!(
        "{verb} -- {changed} changed, {unchanged_count} unchanged"
    )];

    if new_count > 0 {
        summary_lines.push(format!(
            "  {} new {}",
            new_count,
            if new_count == 1 { "file" } else { "files" }
        ));
    }
    if updated_count > 0 {
        summary_lines.push(format!(
            "  {} updated {}",
            updated_count,
            if updated_count == 1 { "file" } else { "files" }
        ));
    }

    summary_lines.push(format!("Output: {}", ux::fmt_path(target_dir)));

    if dry_run {
        summary_lines.push("Re-run without --dry-run to write files".to_string());
    }

    ux::print_summary_box(&summary_lines);

    // -- NEXT STEP --
    if !dry_run && changed > 0 {
        ux::print_next_step("review and commit", "git diff private-data/");
    } else if !dry_run && changed == 0 {
        ux::print_next_step("everything up to date, run", "novanet data diff");
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
