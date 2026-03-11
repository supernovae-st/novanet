//! Backup command for NovaNet CLI.
//!
//! Creates, lists, restores, and prunes tar.gz backups of the brain/ directory
//! (models/, seed/) with SHA256 checksums and JSON manifests.
//!
//! Storage location: `~/.novanet/backups/`

use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing::instrument;

use crate::core::backup::{BackupError, BackupService};
use crate::error::Result;

// ============================================================================
// CLI Arguments
// ============================================================================

/// Backup management commands
#[derive(Debug, Clone, Parser)]
pub struct BackupArgs {
    #[command(subcommand)]
    pub command: BackupSubcommand,
}

/// Backup subcommands
#[derive(Debug, Clone, Subcommand)]
pub enum BackupSubcommand {
    /// Create a new backup of the brain/ directory
    Create(CreateArgs),
    /// List existing backups
    List(ListArgs),
    /// Restore a backup
    Restore(RestoreArgs),
    /// Prune old backups (keep N most recent)
    Prune(PruneArgs),
}

/// Arguments for backup create
#[derive(Debug, Clone, Parser)]
pub struct CreateArgs {
    /// Optional description for the backup
    #[arg(short, long)]
    pub description: Option<String>,

    /// Custom brain directory path (defaults to ./brain)
    #[arg(long)]
    pub brain_dir: Option<std::path::PathBuf>,

    /// Custom backup directory path (defaults to ~/.novanet/backups)
    #[arg(long)]
    pub backup_dir: Option<std::path::PathBuf>,
}

/// Arguments for backup list
#[derive(Debug, Clone, Parser)]
pub struct ListArgs {
    /// Include detailed manifest information
    #[arg(short, long)]
    pub verbose: bool,

    /// Custom backup directory path
    #[arg(long)]
    pub backup_dir: Option<std::path::PathBuf>,
}

/// Arguments for backup restore
#[derive(Debug, Clone, Parser)]
pub struct RestoreArgs {
    /// Backup filename to restore (e.g., novanet-backup-2026-03-11-143022.tar.gz)
    pub filename: String,

    /// Target directory for restoration (defaults to ./brain)
    #[arg(short, long)]
    pub target: Option<std::path::PathBuf>,

    /// Custom backup directory path
    #[arg(long)]
    pub backup_dir: Option<std::path::PathBuf>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,
}

/// Arguments for backup prune
#[derive(Debug, Clone, Parser)]
pub struct PruneArgs {
    /// Number of backups to keep (default: 5)
    #[arg(short, long, default_value = "5")]
    pub keep: usize,

    /// Custom backup directory path
    #[arg(long)]
    pub backup_dir: Option<std::path::PathBuf>,

    /// Dry run - show what would be deleted without deleting
    #[arg(long)]
    pub dry_run: bool,
}

// ============================================================================
// Command Execution
// ============================================================================

/// Run the backup command
#[instrument(skip_all)]
pub async fn run_backup(args: BackupArgs) -> Result<()> {
    match args.command {
        BackupSubcommand::Create(create_args) => run_create(create_args).await,
        BackupSubcommand::List(list_args) => run_list(list_args).await,
        BackupSubcommand::Restore(restore_args) => run_restore(restore_args).await,
        BackupSubcommand::Prune(prune_args) => run_prune(prune_args).await,
    }
}

/// Create a new backup
#[instrument(skip_all)]
async fn run_create(args: CreateArgs) -> Result<()> {
    let service = create_service(args.backup_dir, args.brain_dir)?;

    println!("{}", "Creating backup...".cyan());

    match service.create(args.description).await {
        Ok(info) => {
            println!("{} {}", "✓".green(), "Backup created successfully".green());
            println!("  {} {}", "File:".dimmed(), info.filename);
            println!("  {} {}", "Size:".dimmed(), format_size(info.size));
            println!("  {} {}", "Path:".dimmed(), info.path.display());
            Ok(())
        },
        Err(e) => {
            eprintln!(
                "{} {}",
                "✗".red(),
                format!("Failed to create backup: {}", e).red()
            );
            Err(map_backup_error(e))
        },
    }
}

/// List existing backups
#[instrument(skip_all)]
async fn run_list(args: ListArgs) -> Result<()> {
    let service = create_service(args.backup_dir, None)?;

    let backups = service.list(args.verbose).map_err(map_backup_error)?;

    if backups.is_empty() {
        println!("{}", "No backups found.".yellow());
        return Ok(());
    }

    println!("{} {} backup(s) found:\n", "✓".green(), backups.len());

    for backup in backups {
        println!("  {} {}", "•".cyan(), backup.filename.bold());
        println!("    {} {}", "Size:".dimmed(), format_size(backup.size));
        println!(
            "    {} {}",
            "Created:".dimmed(),
            backup.created_at.format("%Y-%m-%d %H:%M:%S")
        );

        if args.verbose {
            if let Some(manifest) = &backup.manifest {
                println!("    {} v{}", "Version:".dimmed(), manifest.version);
                if let Some(desc) = &manifest.description {
                    println!("    {} {}", "Description:".dimmed(), desc);
                }
                println!(
                    "    {} {} files, {} bytes total",
                    "Contents:".dimmed(),
                    manifest.contents.file_count,
                    manifest.contents.total_size
                );
            }
        }
        println!();
    }

    Ok(())
}

/// Restore a backup
#[instrument(skip_all)]
async fn run_restore(args: RestoreArgs) -> Result<()> {
    let service = create_service(args.backup_dir, args.target)?;

    // Show what will be restored
    println!("{} {}", "Restoring backup:".cyan(), args.filename);

    // Read manifest for confirmation
    match service.read_manifest(&args.filename) {
        Ok(manifest) => {
            println!("  {} v{}", "Version:".dimmed(), manifest.version);
            println!("  {} {}", "Created:".dimmed(), manifest.created_at);
            if let Some(desc) = &manifest.description {
                println!("  {} {}", "Description:".dimmed(), desc);
            }
            println!(
                "  {} {} files",
                "Files:".dimmed(),
                manifest.contents.file_count
            );
        },
        Err(e) => {
            eprintln!("{} Could not read manifest: {}", "⚠".yellow(), e);
        },
    }

    // Confirmation prompt (unless --yes)
    if !args.yes {
        println!();
        println!(
            "{}",
            "This will overwrite existing files in the target directory.".yellow()
        );
        print!("Continue? [y/N] ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Restore cancelled.".yellow());
            return Ok(());
        }
    }

    match service.restore(&args.filename).await {
        Ok(()) => {
            println!("{} {}", "✓".green(), "Backup restored successfully".green());
            Ok(())
        },
        Err(e) => {
            eprintln!(
                "{} {}",
                "✗".red(),
                format!("Failed to restore backup: {}", e).red()
            );
            Err(map_backup_error(e))
        },
    }
}

/// Prune old backups
#[instrument(skip_all)]
async fn run_prune(args: PruneArgs) -> Result<()> {
    let service = create_service(args.backup_dir, None)?;

    if args.dry_run {
        println!(
            "{} {} (keeping {} most recent)",
            "Dry run:".cyan(),
            "would prune backups".dimmed(),
            args.keep
        );
    } else {
        println!(
            "{} {} (keeping {} most recent)",
            "Pruning backups...".cyan(),
            "".dimmed(),
            args.keep
        );
    }

    match service.prune(args.keep, args.dry_run).await {
        Ok(deleted) => {
            if deleted.is_empty() {
                println!("{} {}", "✓".green(), "No backups to prune".green());
            } else if args.dry_run {
                println!("{} Would delete {} backup(s):", "→".cyan(), deleted.len());
                for filename in &deleted {
                    println!("  {} {}", "•".yellow(), filename);
                }
            } else {
                println!("{} Deleted {} backup(s):", "✓".green(), deleted.len());
                for filename in &deleted {
                    println!("  {} {}", "•".dimmed(), filename);
                }
            }
            Ok(())
        },
        Err(e) => {
            eprintln!(
                "{} {}",
                "✗".red(),
                format!("Failed to prune backups: {}", e).red()
            );
            Err(map_backup_error(e))
        },
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a BackupService with optional custom paths
fn create_service(
    backup_dir: Option<std::path::PathBuf>,
    brain_dir: Option<std::path::PathBuf>,
) -> Result<BackupService> {
    match (backup_dir, brain_dir) {
        (Some(backup), Some(brain)) => Ok(BackupService::with_paths(&backup, &brain)),
        (Some(backup), None) => {
            let brain = std::path::PathBuf::from("brain");
            Ok(BackupService::with_paths(&backup, &brain))
        },
        (None, Some(brain)) => {
            let backup = dirs::home_dir()
                .ok_or_else(|| {
                    crate::error::NovaNetError::Config("Cannot determine home directory".into())
                })?
                .join(".novanet")
                .join("backups");
            Ok(BackupService::with_paths(&backup, &brain))
        },
        (None, None) => Ok(BackupService::new()),
    }
}

/// Format file size in human-readable format
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Map BackupError to NovaNetError
fn map_backup_error(e: BackupError) -> crate::error::NovaNetError {
    crate::error::NovaNetError::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        e.to_string(),
    ))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 bytes");
        assert_eq!(format_size(512), "512 bytes");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_backup_args_parsing() {
        use clap::Parser;

        // Test create subcommand
        let args = BackupArgs::try_parse_from(["backup", "create"]).unwrap();
        assert!(matches!(args.command, BackupSubcommand::Create(_)));

        // Test create with description
        let args = BackupArgs::try_parse_from(["backup", "create", "-d", "test backup"]).unwrap();
        if let BackupSubcommand::Create(create_args) = args.command {
            assert_eq!(create_args.description, Some("test backup".to_string()));
        }

        // Test list subcommand
        let args = BackupArgs::try_parse_from(["backup", "list"]).unwrap();
        assert!(matches!(args.command, BackupSubcommand::List(_)));

        // Test list verbose
        let args = BackupArgs::try_parse_from(["backup", "list", "-v"]).unwrap();
        if let BackupSubcommand::List(list_args) = args.command {
            assert!(list_args.verbose);
        }

        // Test restore subcommand
        let args = BackupArgs::try_parse_from(["backup", "restore", "test.tar.gz"]).unwrap();
        if let BackupSubcommand::Restore(restore_args) = args.command {
            assert_eq!(restore_args.filename, "test.tar.gz");
            assert!(!restore_args.yes);
        }

        // Test restore with --yes
        let args = BackupArgs::try_parse_from(["backup", "restore", "test.tar.gz", "-y"]).unwrap();
        if let BackupSubcommand::Restore(restore_args) = args.command {
            assert!(restore_args.yes);
        }

        // Test prune subcommand
        let args = BackupArgs::try_parse_from(["backup", "prune"]).unwrap();
        if let BackupSubcommand::Prune(prune_args) = args.command {
            assert_eq!(prune_args.keep, 5); // default
            assert!(!prune_args.dry_run);
        }

        // Test prune with options
        let args = BackupArgs::try_parse_from(["backup", "prune", "-k", "3", "--dry-run"]).unwrap();
        if let BackupSubcommand::Prune(prune_args) = args.command {
            assert_eq!(prune_args.keep, 3);
            assert!(prune_args.dry_run);
        }
    }
}
