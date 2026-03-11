//! Backup types and error definitions
//!
//! This module defines the core types used by the backup system:
//! - `BackupError` - Error enum for backup operations
//! - `BackupManifest` - JSON manifest stored in archives
//! - `BackupInfo` - Metadata about a backup file
//! - `BackupContents` - File counts and sizes in a backup

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during backup operations
#[derive(Debug, Error)]
pub enum BackupError {
    /// I/O error reading or writing files
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Backup directory not found or not accessible
    #[error("Backup directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    /// Backup file not found
    #[error("Backup file not found: {0}")]
    BackupNotFound(String),

    /// Invalid backup format or corrupted archive
    #[error("Invalid backup format: {0}")]
    InvalidFormat(String),

    /// Checksum mismatch when verifying backup
    #[error("Checksum mismatch for {filename}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        filename: String,
        expected: String,
        actual: String,
    },

    /// Brain directory not found for backup
    #[error("Brain directory not found: {0}")]
    BrainNotFound(PathBuf),

    /// Archive operation error
    #[error("Archive error: {0}")]
    Archive(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Manifest corrupted or missing
    #[error("Manifest corrupted: {0}")]
    ManifestCorrupted(String),
}

/// Result type for backup operations
pub type Result<T> = std::result::Result<T, BackupError>;

/// Contents summary for a backup archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupContents {
    /// Number of files in the backup
    pub file_count: u64,
    /// Total size of all files in bytes
    pub total_size: u64,
    /// List of directories in the backup
    #[serde(default)]
    pub directories: Vec<String>,
}

/// Manifest stored inside each backup archive as manifest.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    /// Manifest format version
    pub version: String,
    /// When the backup was created
    pub created_at: DateTime<Utc>,
    /// Optional user-provided description
    pub description: Option<String>,
    /// Contents summary
    pub contents: BackupContents,
    /// SHA256 checksums of backed up files
    #[serde(default)]
    pub checksums: std::collections::HashMap<String, String>,
}

impl BackupManifest {
    /// Current manifest format version
    pub const VERSION: &'static str = "1.0";

    /// Create a new manifest with the given description and contents
    pub fn new(description: Option<String>, contents: BackupContents) -> Self {
        Self {
            version: Self::VERSION.to_string(),
            created_at: Utc::now(),
            description,
            contents,
            checksums: std::collections::HashMap::new(),
        }
    }
}

/// Information about a backup file
#[derive(Debug, Clone)]
pub struct BackupInfo {
    /// Filename of the backup (e.g., "novanet-backup-2024-01-15-143022.tar.gz")
    pub filename: String,
    /// Full path to the backup file
    pub path: PathBuf,
    /// Size of the backup file in bytes
    pub size: u64,
    /// When the backup was created (from filesystem or manifest)
    pub created_at: DateTime<Utc>,
    /// Parsed manifest from the backup (if available)
    pub manifest: Option<BackupManifest>,
}

impl BackupInfo {
    /// Format the size for human-readable display
    pub fn formatted_size(&self) -> String {
        format_size(self.size)
    }

    /// Format the creation time for display
    pub fn formatted_time(&self) -> String {
        self.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

/// Format a byte size into human-readable form (KB, MB, GB)
pub fn format_size(bytes: u64) -> String {
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
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_backup_manifest_new() {
        let contents = BackupContents {
            file_count: 10,
            total_size: 1024,
            directories: vec!["brain".to_string()],
        };
        let manifest = BackupManifest::new(Some("Test backup".to_string()), contents);

        assert_eq!(manifest.version, "1.0");
        assert_eq!(manifest.description, Some("Test backup".to_string()));
        assert_eq!(manifest.contents.file_count, 10);
        assert_eq!(manifest.contents.total_size, 1024);
    }

    #[test]
    fn test_backup_info_formatted() {
        let info = BackupInfo {
            filename: "test.tar.gz".to_string(),
            path: PathBuf::from("/tmp/test.tar.gz"),
            size: 1024 * 512, // 512 KB
            created_at: Utc::now(),
            manifest: None,
        };

        assert_eq!(info.formatted_size(), "512.00 KB");
        assert!(!info.formatted_time().is_empty());
    }
}
