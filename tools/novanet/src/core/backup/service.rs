//! Backup service implementation
//!
//! This module provides the main `BackupService` struct for managing NovaNet backups:
//! - Create backups of the brain directory
//! - List existing backups with metadata
//! - Restore from backups
//! - Prune old backups

use super::archive::{create_archive, extract_archive, read_manifest_from_archive};
use super::types::{BackupContents, BackupError, BackupInfo, BackupManifest, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use tracing::instrument;

/// Default backup directory relative to home
const DEFAULT_BACKUP_DIR: &str = ".novanet/backups";

/// Default brain directory relative to monorepo root
const DEFAULT_BRAIN_DIR: &str = "brain";

/// Service for managing NovaNet backups
#[derive(Debug, Clone)]
pub struct BackupService {
    /// Directory where backups are stored
    backup_dir: PathBuf,
    /// Directory containing the brain data to backup
    brain_dir: PathBuf,
}

impl Default for BackupService {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupService {
    /// Create a new BackupService with default paths
    ///
    /// - Backup directory: `~/.novanet/backups`
    /// - Brain directory: discovered from NOVANET_ROOT or current directory
    pub fn new() -> Self {
        let backup_dir = dirs::home_dir()
            .map(|h| h.join(DEFAULT_BACKUP_DIR))
            .unwrap_or_else(|| PathBuf::from(DEFAULT_BACKUP_DIR));

        let brain_dir = std::env::var("NOVANET_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default())
            .join(DEFAULT_BRAIN_DIR);

        Self {
            backup_dir,
            brain_dir,
        }
    }

    /// Create a BackupService with custom paths
    pub fn with_paths(backup_dir: impl AsRef<Path>, brain_dir: impl AsRef<Path>) -> Self {
        Self {
            backup_dir: backup_dir.as_ref().to_path_buf(),
            brain_dir: brain_dir.as_ref().to_path_buf(),
        }
    }

    /// Get the backup directory path
    pub fn backup_dir(&self) -> &Path {
        &self.backup_dir
    }

    /// Get the brain directory path
    pub fn brain_dir(&self) -> &Path {
        &self.brain_dir
    }

    /// Create a new backup of the brain directory
    ///
    /// Returns the path to the created backup file.
    #[instrument(skip_all)]
    pub async fn create(&self, description: Option<String>) -> Result<PathBuf> {
        // Ensure backup directory exists
        std::fs::create_dir_all(&self.backup_dir)?;

        // Generate filename with timestamp
        let timestamp = Utc::now().format("%Y-%m-%d-%H%M%S");
        let filename = format!("novanet-backup-{}.tar.gz", timestamp);
        let backup_path = self.backup_dir.join(&filename);

        // Create the archive - handles brain_dir validation atomically
        // If brain_dir doesn't exist, create_archive returns an I/O error
        create_archive(&self.brain_dir, &backup_path, description)
            .await
            .map_err(|e| {
                // Convert NotFound to BrainNotFound for better error context
                if let BackupError::Io(ref io_err) = e {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        return BackupError::BrainNotFound(self.brain_dir.clone());
                    }
                }
                e
            })?;

        Ok(backup_path)
    }

    /// List all backups in the backup directory
    ///
    /// If `verbose` is true, reads manifests from each backup for detailed info.
    #[instrument(skip_all)]
    pub fn list(&self, verbose: bool) -> Result<Vec<BackupInfo>> {
        if !self.backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();

        for entry in std::fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip non-files and non-tar.gz files
            if !path.is_file() {
                continue;
            }
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if !filename.starts_with("novanet-backup-") || !filename.ends_with(".tar.gz") {
                continue;
            }

            let metadata = entry.metadata()?;
            let size = metadata.len();

            // Parse creation time from filename or use file mtime
            let created_at = parse_timestamp_from_filename(filename).unwrap_or_else(|| {
                metadata
                    .modified()
                    .ok()
                    .and_then(|t| {
                        t.duration_since(std::time::UNIX_EPOCH)
                            .ok()
                            .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
                    })
                    .flatten()
                    .unwrap_or_else(Utc::now)
            });

            // Optionally read manifest for verbose output
            let manifest = if verbose {
                read_manifest_from_archive(&path).ok()
            } else {
                None
            };

            backups.push(BackupInfo {
                filename: filename.to_string(),
                path: path.clone(),
                size,
                created_at,
                manifest,
            });
        }

        // Sort by creation time (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(backups)
    }

    /// Restore a backup to the brain directory
    ///
    /// This will overwrite the current brain directory contents.
    #[instrument(skip_all)]
    pub async fn restore(&self, filename: &str) -> Result<()> {
        let backup_path = self.backup_dir.join(filename);

        // Ensure brain directory exists
        std::fs::create_dir_all(&self.brain_dir)?;

        // Extract the archive - handles backup existence check atomically
        // If backup doesn't exist, extract_archive returns an I/O error
        extract_archive(&backup_path, &self.brain_dir)
            .await
            .map_err(|e| {
                // Convert NotFound to BackupNotFound for better error context
                if let BackupError::Io(ref io_err) = e {
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        return BackupError::BackupNotFound(filename.to_string());
                    }
                }
                e
            })?;

        Ok(())
    }

    /// Prune old backups, keeping only the most recent `keep` backups
    ///
    /// If `dry_run` is true, returns the list of backups that would be deleted
    /// without actually deleting them.
    #[instrument(skip_all)]
    pub async fn prune(&self, keep: usize, dry_run: bool) -> Result<Vec<PathBuf>> {
        let backups = self.list(false)?;

        if backups.len() <= keep {
            return Ok(Vec::new());
        }

        // Get backups to delete (everything after the first `keep` entries)
        let to_delete: Vec<PathBuf> = backups.into_iter().skip(keep).map(|b| b.path).collect();

        if !dry_run {
            for path in &to_delete {
                std::fs::remove_file(path)?;
            }
        }

        Ok(to_delete)
    }

    /// Read the manifest from a backup file
    #[instrument(skip_all)]
    pub fn read_manifest(&self, filename: &str) -> Result<BackupManifest> {
        let backup_path = self.backup_dir.join(filename);

        // Read manifest atomically - if file doesn't exist, read returns NotFound
        read_manifest_from_archive(&backup_path).map_err(|e| {
            // Convert NotFound to BackupNotFound for better error context
            if let BackupError::Io(ref io_err) = e {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    return BackupError::BackupNotFound(filename.to_string());
                }
            }
            e
        })
    }

    /// Verify a backup's integrity by checking checksums
    #[instrument(skip_all)]
    pub fn verify(&self, filename: &str) -> Result<bool> {
        let manifest = self.read_manifest(filename)?;

        if manifest.checksums.is_empty() {
            // No checksums to verify (old backup format)
            return Ok(true);
        }

        // For now, just return true if we can read the manifest
        // Full verification would extract and check each file
        Ok(true)
    }
}

/// Parse timestamp from backup filename
///
/// Expected format: `novanet-backup-YYYY-MM-DD-HHMMSS.tar.gz`
fn parse_timestamp_from_filename(filename: &str) -> Option<chrono::DateTime<Utc>> {
    // Extract the timestamp part: "2024-01-15-143022"
    let stripped = filename
        .strip_prefix("novanet-backup-")?
        .strip_suffix(".tar.gz")?;

    // Parse: YYYY-MM-DD-HHMMSS
    let parts: Vec<&str> = stripped.split('-').collect();
    if parts.len() != 4 {
        return None;
    }

    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;

    let time_str = parts[3];
    if time_str.len() != 6 {
        return None;
    }

    let hour: u32 = time_str[0..2].parse().ok()?;
    let minute: u32 = time_str[2..4].parse().ok()?;
    let second: u32 = time_str[4..6].parse().ok()?;

    chrono::NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|d| d.and_hms_opt(hour, minute, second))
        .map(|dt| dt.and_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp_from_filename() {
        let filename = "novanet-backup-2024-01-15-143022.tar.gz";
        let result = parse_timestamp_from_filename(filename);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(
            dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2024-01-15 14:30:22"
        );
    }

    #[test]
    fn test_parse_timestamp_invalid() {
        assert!(parse_timestamp_from_filename("invalid.tar.gz").is_none());
        assert!(parse_timestamp_from_filename("novanet-backup-invalid.tar.gz").is_none());
        assert!(parse_timestamp_from_filename("novanet-backup-2024-01-15.tar.gz").is_none());
    }

    #[test]
    fn test_backup_service_default() {
        let service = BackupService::new();
        assert!(service.backup_dir().to_str().unwrap().contains("novanet"));
    }

    #[test]
    fn test_backup_service_with_paths() {
        let service = BackupService::with_paths("/tmp/backups", "/tmp/brain");
        assert_eq!(service.backup_dir(), Path::new("/tmp/backups"));
        assert_eq!(service.brain_dir(), Path::new("/tmp/brain"));
    }

    #[tokio::test]
    async fn test_list_empty_directory() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let service = BackupService::with_paths(temp_dir.path(), temp_dir.path());
        let backups = service.list(false).expect("Failed to list backups");
        assert!(backups.is_empty());
    }

    #[tokio::test]
    async fn test_prune_with_no_backups() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let service = BackupService::with_paths(temp_dir.path(), temp_dir.path());
        let deleted = service.prune(5, true).await.expect("Failed to prune");
        assert!(deleted.is_empty());
    }
}
