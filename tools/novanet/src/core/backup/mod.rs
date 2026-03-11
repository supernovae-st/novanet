//! Backup module for NovaNet brain directory
//!
//! This module provides functionality to backup and restore the brain directory:
//! - Create tar.gz backups with JSON manifests
//! - List and inspect existing backups
//! - Restore from backups
//! - Prune old backups
//!
//! # Architecture
//!
//! ```text
//! BackupService
//!     ├── create()    → Creates tar.gz with manifest.json
//!     ├── list()      → Lists backups in backup directory
//!     ├── restore()   → Extracts backup to brain directory
//!     ├── prune()     → Removes old backups (keeps N most recent)
//!     └── verify()    → Validates checksums in backup
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use novanet::core::backup::BackupService;
//!
//! // Create a new backup
//! let service = BackupService::new();
//! let backup_path = service.create(Some("Pre-migration backup".into())).await?;
//!
//! // List existing backups
//! for backup in service.list(true)? {
//!     println!("{}: {}", backup.filename, backup.formatted_size());
//! }
//!
//! // Restore from a backup
//! service.restore("novanet-backup-2024-01-15-143022.tar.gz").await?;
//!
//! // Prune old backups (keep last 5)
//! let removed = service.prune(5, false).await?;
//! ```

mod archive;
mod service;
mod types;

// Re-export public API
pub use service::BackupService;
pub use types::{BackupContents, BackupError, BackupInfo, BackupManifest, Result, format_size};
