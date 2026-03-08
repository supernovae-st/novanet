# Unified Backup System — Implementation Plan

**Date**: 2026-03-08
**Version**: v1.0.0
**Status**: Ready for Implementation
**Author**: Claude + Thibaut

---

## Executive Summary

Implement a unified backup system for the SuperNovae ecosystem via `spn backup` commands. The system orchestrates backups across NovaNet (knowledge graph), Nika (workflows), and spn (daemon state) through a centralized BackupManager in the spn daemon.

### Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Storage Location** | `~/.spn/backups/` | Centralized, predictable, travels with user |
| **Automation Level** | Manual + before_seed hook | Simple v1, safety without complexity |
| **Architecture** | Daemon-orchestrated | Single point of coordination, atomic operations |
| **Secrets** | Excluded (stay in Keychain) | Security best practice |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  UNIFIED BACKUP ARCHITECTURE                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  User: spn backup create                                                        │
│         │                                                                       │
│         ▼                                                                       │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  spn daemon (BackupManager)                                              │   │
│  │  ├── validate_state()      → Check all systems accessible                │   │
│  │  ├── pause_operations()    → Prevent writes during backup                │   │
│  │  ├── collect_manifests()   → Ask each adapter what to backup             │   │
│  │  ├── create_snapshot()     → Atomic snapshot of all data                 │   │
│  │  ├── compress_archive()    → tar.gz with metadata                        │   │
│  │  └── resume_operations()   → Unlock systems                              │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│         │                                                                       │
│         ├──────────────────┬──────────────────┬─────────────────────────────   │
│         ▼                  ▼                  ▼                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                         │
│  │ NovaNet     │    │ Nika        │    │ spn         │                         │
│  │ Adapter     │    │ Adapter     │    │ Adapter     │                         │
│  ├─────────────┤    ├─────────────┤    ├─────────────┤                         │
│  │ • YAML SOT  │    │ • .nika/    │    │ • jobs.json │                         │
│  │ • Neo4j dump│    │ • sessions  │    │ • memory    │                         │
│  │ • seeds     │    │ • traces    │    │ • mcp.yaml  │                         │
│  └─────────────┘    └─────────────┘    └─────────────┘                         │
│                                                                                 │
│  Output: ~/.spn/backups/backup-2026-03-08T14-30-00.tar.gz                       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Backup Contents

```
backup-2026-03-08T14-30-00.tar.gz
├── manifest.json              # Metadata: version, timestamp, checksums
├── novanet/
│   ├── brain/
│   │   ├── models/            # Schema YAML (node-classes, arc-classes)
│   │   └── seed/              # Seed data YAML
│   └── neo4j-dump.cypher      # Optional: full database export
├── nika/
│   ├── workflows/             # .nika.yaml files
│   ├── sessions/              # Chat sessions
│   └── traces/                # Execution traces
└── spn/
    ├── config.toml            # User config (sans secrets)
    ├── mcp.yaml               # MCP server configs
    └── jobs.json              # Scheduled jobs
```

---

## Phase 1: BackupManager Core

**Goal**: Create the core BackupManager subsystem in spn daemon.

### Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `supernovae-cli/crates/spn-core/src/backup/mod.rs` | Create | Module exports |
| `supernovae-cli/crates/spn-core/src/backup/manager.rs` | Create | BackupManager struct |
| `supernovae-cli/crates/spn-core/src/backup/manifest.rs` | Create | Manifest types |
| `supernovae-cli/crates/spn-core/src/backup/error.rs` | Create | Backup-specific errors |
| `supernovae-cli/crates/spn-core/src/lib.rs` | Modify | Add backup module |

### Implementation Details

```rust
// supernovae-cli/crates/spn-core/src/backup/manager.rs

use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Backup manager orchestrating all backup operations.
pub struct BackupManager {
    backup_dir: PathBuf,
    adapters: Vec<Box<dyn BackupAdapter>>,
}

impl BackupManager {
    pub fn new() -> Self {
        let backup_dir = dirs::home_dir()
            .expect("home dir")
            .join(".spn")
            .join("backups");

        Self {
            backup_dir,
            adapters: Vec::new(),
        }
    }

    /// Register a backup adapter for a subsystem.
    pub fn register_adapter(&mut self, adapter: Box<dyn BackupAdapter>) {
        self.adapters.push(adapter);
    }

    /// Create a new backup of all registered systems.
    pub async fn create_backup(&self, label: Option<&str>) -> Result<BackupInfo, BackupError> {
        // 1. Validate all systems are accessible
        self.validate_systems().await?;

        // 2. Create timestamp-based backup name
        let timestamp = Utc::now();
        let name = self.generate_backup_name(&timestamp, label);
        let backup_path = self.backup_dir.join(&name);

        // 3. Create temp directory for staging
        let staging_dir = tempfile::tempdir()?;

        // 4. Collect data from each adapter
        for adapter in &self.adapters {
            adapter.collect(staging_dir.path()).await?;
        }

        // 5. Create manifest
        let manifest = self.create_manifest(&timestamp, &staging_dir)?;

        // 6. Compress to tar.gz
        self.compress_archive(&staging_dir, &backup_path)?;

        Ok(BackupInfo {
            path: backup_path,
            timestamp,
            size_bytes: std::fs::metadata(&backup_path)?.len(),
            manifest,
        })
    }

    /// Restore from a backup archive.
    pub async fn restore_backup(&self, backup_path: &Path) -> Result<RestoreInfo, BackupError> {
        // 1. Validate backup exists and is valid
        self.validate_backup(backup_path)?;

        // 2. Extract to temp directory
        let staging_dir = tempfile::tempdir()?;
        self.extract_archive(backup_path, staging_dir.path())?;

        // 3. Read manifest
        let manifest = self.read_manifest(staging_dir.path())?;

        // 4. Restore each subsystem
        for adapter in &self.adapters {
            adapter.restore(staging_dir.path()).await?;
        }

        Ok(RestoreInfo {
            backup_path: backup_path.to_path_buf(),
            restored_at: Utc::now(),
            manifest,
        })
    }

    /// List all available backups.
    pub fn list_backups(&self) -> Result<Vec<BackupInfo>, BackupError> {
        let mut backups = Vec::new();

        if !self.backup_dir.exists() {
            return Ok(backups);
        }

        for entry in std::fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |e| e == "gz") {
                if let Ok(info) = self.read_backup_info(&path) {
                    backups.push(info);
                }
            }
        }

        // Sort by timestamp, newest first
        backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(backups)
    }
}
```

```rust
// supernovae-cli/crates/spn-core/src/backup/manifest.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Backup manifest containing metadata about the backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    /// Manifest format version
    pub version: String,

    /// When the backup was created
    pub created_at: DateTime<Utc>,

    /// Optional user-provided label
    pub label: Option<String>,

    /// Machine hostname
    pub hostname: String,

    /// SuperNovae component versions
    pub versions: ComponentVersions,

    /// Checksums for each backed-up file
    pub checksums: HashMap<String, String>,

    /// What was included in the backup
    pub contents: BackupContents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentVersions {
    pub novanet: Option<String>,
    pub nika: Option<String>,
    pub spn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupContents {
    pub novanet: NovaNetContents,
    pub nika: NikaContents,
    pub spn: SpnContents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovaNetContents {
    pub schema_files: u32,
    pub seed_files: u32,
    pub neo4j_dump: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NikaContents {
    pub workflow_files: u32,
    pub session_count: u32,
    pub trace_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpnContents {
    pub has_config: bool,
    pub has_mcp_yaml: bool,
    pub has_jobs: bool,
}
```

### Tests for Phase 1

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_backup_manager_new() {
        let manager = BackupManager::new();
        assert!(manager.backup_dir.ends_with("backups"));
    }

    #[test]
    fn test_generate_backup_name() {
        let manager = BackupManager::new();
        let timestamp = Utc::now();

        let name = manager.generate_backup_name(&timestamp, None);
        assert!(name.starts_with("backup-"));
        assert!(name.ends_with(".tar.gz"));

        let name_with_label = manager.generate_backup_name(&timestamp, Some("pre-refacto"));
        assert!(name_with_label.contains("pre-refacto"));
    }

    #[test]
    fn test_manifest_serialization() {
        let manifest = BackupManifest {
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            label: Some("test".to_string()),
            hostname: "dev-machine".to_string(),
            versions: ComponentVersions {
                novanet: Some("0.17.2".to_string()),
                nika: Some("0.21.1".to_string()),
                spn: "0.14.3".to_string(),
            },
            checksums: HashMap::new(),
            contents: BackupContents {
                novanet: NovaNetContents {
                    schema_files: 57,
                    seed_files: 10,
                    neo4j_dump: false,
                },
                nika: NikaContents {
                    workflow_files: 5,
                    session_count: 10,
                    trace_count: 25,
                },
                spn: SpnContents {
                    has_config: true,
                    has_mcp_yaml: true,
                    has_jobs: false,
                },
            },
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        assert!(json.contains("1.0.0"));

        let parsed: BackupManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, "1.0.0");
    }

    #[tokio::test]
    async fn test_list_backups_empty() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = BackupManager::new();
        manager.backup_dir = temp_dir.path().to_path_buf();

        let backups = manager.list_backups().unwrap();
        assert!(backups.is_empty());
    }
}
```

---

## Phase 2: System Adapters

**Goal**: Create adapters for NovaNet, Nika, and spn subsystems.

### Files to Create

| File | Description |
|------|-------------|
| `spn-core/src/backup/adapter.rs` | BackupAdapter trait |
| `spn-core/src/backup/adapters/mod.rs` | Adapter exports |
| `spn-core/src/backup/adapters/novanet.rs` | NovaNet adapter |
| `spn-core/src/backup/adapters/nika.rs` | Nika adapter |
| `spn-core/src/backup/adapters/spn.rs` | spn adapter |

### BackupAdapter Trait

```rust
// supernovae-cli/crates/spn-core/src/backup/adapter.rs

use async_trait::async_trait;
use std::path::Path;

/// Trait for subsystem backup adapters.
#[async_trait]
pub trait BackupAdapter: Send + Sync {
    /// Unique identifier for this adapter.
    fn name(&self) -> &str;

    /// Check if the subsystem is available for backup.
    async fn is_available(&self) -> bool;

    /// Get the version of the subsystem (for manifest).
    async fn version(&self) -> Option<String>;

    /// Collect data to the staging directory.
    ///
    /// The adapter should create a subdirectory with its name:
    /// `staging_dir/{adapter_name}/...`
    async fn collect(&self, staging_dir: &Path) -> Result<AdapterContents, BackupError>;

    /// Restore data from the staging directory.
    async fn restore(&self, staging_dir: &Path) -> Result<(), BackupError>;
}

/// Adapter-specific contents summary.
#[derive(Debug, Clone)]
pub enum AdapterContents {
    NovaNet(NovaNetContents),
    Nika(NikaContents),
    Spn(SpnContents),
}
```

### NovaNet Adapter

```rust
// supernovae-cli/crates/spn-core/src/backup/adapters/novanet.rs

use super::*;
use std::process::Command;

pub struct NovaNetAdapter {
    /// Path to novanet project root
    project_root: Option<PathBuf>,
    /// Path to brain/ directory (YAML SOT)
    brain_path: Option<PathBuf>,
}

impl NovaNetAdapter {
    pub fn new() -> Self {
        Self {
            project_root: Self::find_novanet_root(),
            brain_path: Self::find_brain_path(),
        }
    }

    fn find_novanet_root() -> Option<PathBuf> {
        // Look for novanet in common locations
        let candidates = [
            dirs::home_dir().map(|h| h.join("dev/supernovae/novanet")),
            std::env::current_dir().ok(),
        ];

        for candidate in candidates.into_iter().flatten() {
            if candidate.join("tools/novanet/Cargo.toml").exists() {
                return Some(candidate);
            }
        }
        None
    }

    fn find_brain_path() -> Option<PathBuf> {
        // brain/ is typically at workspace root
        let candidates = [
            dirs::home_dir().map(|h| h.join("dev/supernovae/brain")),
        ];

        for candidate in candidates.into_iter().flatten() {
            if candidate.join("models").exists() {
                return Some(candidate);
            }
        }
        None
    }
}

#[async_trait]
impl BackupAdapter for NovaNetAdapter {
    fn name(&self) -> &str {
        "novanet"
    }

    async fn is_available(&self) -> bool {
        self.brain_path.is_some()
    }

    async fn version(&self) -> Option<String> {
        // Read version from Cargo.toml or CHANGELOG
        self.project_root.as_ref().and_then(|root| {
            let cargo_toml = root.join("tools/novanet/Cargo.toml");
            std::fs::read_to_string(cargo_toml).ok().and_then(|content| {
                content.lines()
                    .find(|l| l.starts_with("version = "))
                    .map(|l| l.trim_start_matches("version = ").trim_matches('"').to_string())
            })
        })
    }

    async fn collect(&self, staging_dir: &Path) -> Result<AdapterContents, BackupError> {
        let brain_path = self.brain_path.as_ref()
            .ok_or(BackupError::NotAvailable("NovaNet brain/ not found".into()))?;

        let novanet_staging = staging_dir.join("novanet");
        std::fs::create_dir_all(&novanet_staging)?;

        // Copy brain/models/
        let models_src = brain_path.join("models");
        let models_dst = novanet_staging.join("brain/models");
        if models_src.exists() {
            copy_dir_recursive(&models_src, &models_dst)?;
        }

        // Copy brain/seed/
        let seed_src = brain_path.join("seed");
        let seed_dst = novanet_staging.join("brain/seed");
        if seed_src.exists() {
            copy_dir_recursive(&seed_src, &seed_dst)?;
        }

        // Count files for manifest
        let schema_files = count_yaml_files(&models_dst);
        let seed_files = count_yaml_files(&seed_dst);

        Ok(AdapterContents::NovaNet(NovaNetContents {
            schema_files,
            seed_files,
            neo4j_dump: false, // Optional, not implemented in v1
        }))
    }

    async fn restore(&self, staging_dir: &Path) -> Result<(), BackupError> {
        let brain_path = self.brain_path.as_ref()
            .ok_or(BackupError::NotAvailable("NovaNet brain/ not found".into()))?;

        let novanet_staging = staging_dir.join("novanet");

        // Restore brain/models/
        let models_src = novanet_staging.join("brain/models");
        let models_dst = brain_path.join("models");
        if models_src.exists() {
            // Backup existing before overwrite
            copy_dir_recursive(&models_src, &models_dst)?;
        }

        // Restore brain/seed/
        let seed_src = novanet_staging.join("brain/seed");
        let seed_dst = brain_path.join("seed");
        if seed_src.exists() {
            copy_dir_recursive(&seed_src, &seed_dst)?;
        }

        Ok(())
    }
}
```

### Nika Adapter

```rust
// supernovae-cli/crates/spn-core/src/backup/adapters/nika.rs

pub struct NikaAdapter {
    /// Path to .nika/ directory
    nika_dir: Option<PathBuf>,
    /// Path to nika project root
    project_root: Option<PathBuf>,
}

impl NikaAdapter {
    pub fn new() -> Self {
        Self {
            nika_dir: Self::find_nika_dir(),
            project_root: Self::find_nika_root(),
        }
    }

    fn find_nika_dir() -> Option<PathBuf> {
        // .nika/ in home directory
        dirs::home_dir().map(|h| h.join(".nika"))
            .filter(|p| p.exists())
    }

    fn find_nika_root() -> Option<PathBuf> {
        let candidates = [
            dirs::home_dir().map(|h| h.join("dev/supernovae/nika")),
        ];

        for candidate in candidates.into_iter().flatten() {
            if candidate.join("tools/nika/Cargo.toml").exists() {
                return Some(candidate);
            }
        }
        None
    }
}

#[async_trait]
impl BackupAdapter for NikaAdapter {
    fn name(&self) -> &str {
        "nika"
    }

    async fn is_available(&self) -> bool {
        self.nika_dir.is_some()
    }

    async fn version(&self) -> Option<String> {
        self.project_root.as_ref().and_then(|root| {
            let cargo_toml = root.join("tools/nika/Cargo.toml");
            std::fs::read_to_string(cargo_toml).ok().and_then(|content| {
                content.lines()
                    .find(|l| l.starts_with("version = "))
                    .map(|l| l.trim_start_matches("version = ").trim_matches('"').to_string())
            })
        })
    }

    async fn collect(&self, staging_dir: &Path) -> Result<AdapterContents, BackupError> {
        let nika_dir = self.nika_dir.as_ref()
            .ok_or(BackupError::NotAvailable("Nika .nika/ not found".into()))?;

        let nika_staging = staging_dir.join("nika");
        std::fs::create_dir_all(&nika_staging)?;

        // Copy sessions/
        let sessions_src = nika_dir.join("sessions");
        let sessions_dst = nika_staging.join("sessions");
        let session_count = if sessions_src.exists() {
            copy_dir_recursive(&sessions_src, &sessions_dst)?;
            count_files(&sessions_dst)
        } else {
            0
        };

        // Copy traces/
        let traces_src = nika_dir.join("traces");
        let traces_dst = nika_staging.join("traces");
        let trace_count = if traces_src.exists() {
            copy_dir_recursive(&traces_src, &traces_dst)?;
            count_files(&traces_dst)
        } else {
            0
        };

        // Copy workflows from project
        let workflow_count = if let Some(root) = &self.project_root {
            let workflows_src = root.join("workflows");
            let workflows_dst = nika_staging.join("workflows");
            if workflows_src.exists() {
                copy_dir_recursive(&workflows_src, &workflows_dst)?;
                count_yaml_files(&workflows_dst)
            } else {
                0
            }
        } else {
            0
        };

        Ok(AdapterContents::Nika(NikaContents {
            workflow_files: workflow_count,
            session_count,
            trace_count,
        }))
    }

    async fn restore(&self, staging_dir: &Path) -> Result<(), BackupError> {
        let nika_dir = self.nika_dir.as_ref()
            .ok_or(BackupError::NotAvailable("Nika .nika/ not found".into()))?;

        let nika_staging = staging_dir.join("nika");

        // Restore sessions/
        let sessions_src = nika_staging.join("sessions");
        if sessions_src.exists() {
            copy_dir_recursive(&sessions_src, &nika_dir.join("sessions"))?;
        }

        // Restore traces/
        let traces_src = nika_staging.join("traces");
        if traces_src.exists() {
            copy_dir_recursive(&traces_src, &nika_dir.join("traces"))?;
        }

        Ok(())
    }
}
```

### spn Adapter

```rust
// supernovae-cli/crates/spn-core/src/backup/adapters/spn.rs

pub struct SpnAdapter {
    /// Path to ~/.spn/ directory
    spn_dir: PathBuf,
}

impl SpnAdapter {
    pub fn new() -> Self {
        Self {
            spn_dir: dirs::home_dir()
                .expect("home dir")
                .join(".spn"),
        }
    }
}

#[async_trait]
impl BackupAdapter for SpnAdapter {
    fn name(&self) -> &str {
        "spn"
    }

    async fn is_available(&self) -> bool {
        self.spn_dir.exists()
    }

    async fn version(&self) -> Option<String> {
        // Read from spn binary or Cargo.toml
        Some(env!("CARGO_PKG_VERSION").to_string())
    }

    async fn collect(&self, staging_dir: &Path) -> Result<AdapterContents, BackupError> {
        let spn_staging = staging_dir.join("spn");
        std::fs::create_dir_all(&spn_staging)?;

        // Copy config.toml (without secrets)
        let config_path = self.spn_dir.join("config.toml");
        let has_config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            // Strip any password fields before backup
            let sanitized = sanitize_config(&content);
            std::fs::write(spn_staging.join("config.toml"), sanitized)?;
            true
        } else {
            false
        };

        // Copy mcp.yaml
        let mcp_path = self.spn_dir.join("mcp.yaml");
        let has_mcp_yaml = if mcp_path.exists() {
            std::fs::copy(&mcp_path, spn_staging.join("mcp.yaml"))?;
            true
        } else {
            false
        };

        // Copy jobs.json
        let jobs_path = self.spn_dir.join("jobs.json");
        let has_jobs = if jobs_path.exists() {
            std::fs::copy(&jobs_path, spn_staging.join("jobs.json"))?;
            true
        } else {
            false
        };

        Ok(AdapterContents::Spn(SpnContents {
            has_config,
            has_mcp_yaml,
            has_jobs,
        }))
    }

    async fn restore(&self, staging_dir: &Path) -> Result<(), BackupError> {
        let spn_staging = staging_dir.join("spn");

        // Restore config.toml
        let config_src = spn_staging.join("config.toml");
        if config_src.exists() {
            std::fs::copy(&config_src, self.spn_dir.join("config.toml"))?;
        }

        // Restore mcp.yaml
        let mcp_src = spn_staging.join("mcp.yaml");
        if mcp_src.exists() {
            std::fs::copy(&mcp_src, self.spn_dir.join("mcp.yaml"))?;
        }

        // Restore jobs.json
        let jobs_src = spn_staging.join("jobs.json");
        if jobs_src.exists() {
            std::fs::copy(&jobs_src, self.spn_dir.join("jobs.json"))?;
        }

        Ok(())
    }
}

/// Remove password/secret fields from config content.
fn sanitize_config(content: &str) -> String {
    content.lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            !lower.contains("password") &&
            !lower.contains("secret") &&
            !lower.contains("token") &&
            !lower.contains("api_key")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

---

## Phase 3: CLI Commands

**Goal**: Implement `spn backup create/restore/list` commands.

### Files to Modify

| File | Description |
|------|-------------|
| `spn-cli/src/commands/mod.rs` | Add backup subcommand |
| `spn-cli/src/commands/backup.rs` | Create backup command handlers |

### Implementation

```rust
// supernovae-cli/crates/spn-cli/src/commands/backup.rs

use clap::{Parser, Subcommand};
use spn_core::backup::{BackupManager, NovaNetAdapter, NikaAdapter, SpnAdapter};

#[derive(Parser)]
#[command(about = "Backup and restore SuperNovae data")]
pub struct BackupCommand {
    #[command(subcommand)]
    command: BackupSubcommand,
}

#[derive(Subcommand)]
enum BackupSubcommand {
    /// Create a new backup
    Create {
        /// Optional label for the backup
        #[arg(short, long)]
        label: Option<String>,

        /// Include Neo4j database dump (slower)
        #[arg(long)]
        include_neo4j: bool,
    },

    /// Restore from a backup
    Restore {
        /// Path to backup file, or "latest" for most recent
        #[arg(default_value = "latest")]
        backup: String,

        /// Force restore without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// List available backups
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,

        /// Maximum number of backups to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Delete old backups
    Prune {
        /// Keep only the N most recent backups
        #[arg(short, long, default_value = "5")]
        keep: usize,

        /// Actually delete (dry-run by default)
        #[arg(long)]
        execute: bool,
    },
}

impl BackupCommand {
    pub async fn run(self) -> Result<()> {
        // Initialize backup manager with all adapters
        let mut manager = BackupManager::new();
        manager.register_adapter(Box::new(NovaNetAdapter::new()));
        manager.register_adapter(Box::new(NikaAdapter::new()));
        manager.register_adapter(Box::new(SpnAdapter::new()));

        match self.command {
            BackupSubcommand::Create { label, include_neo4j } => {
                println!("🔄 Creating backup...");

                let info = manager.create_backup(label.as_deref()).await?;

                println!("✅ Backup created successfully!");
                println!();
                println!("   📦 File: {}", info.path.display());
                println!("   📊 Size: {}", format_bytes(info.size_bytes));
                println!("   🕐 Time: {}", info.timestamp.format("%Y-%m-%d %H:%M:%S"));

                if let Some(label) = &info.manifest.label {
                    println!("   🏷️  Label: {}", label);
                }

                Ok(())
            }

            BackupSubcommand::Restore { backup, force } => {
                let backup_path = if backup == "latest" {
                    manager.list_backups()?
                        .first()
                        .map(|b| b.path.clone())
                        .ok_or(BackupError::NoBackupsFound)?
                } else {
                    PathBuf::from(&backup)
                };

                if !force {
                    println!("⚠️  This will overwrite existing data!");
                    println!("   Backup: {}", backup_path.display());
                    print!("   Continue? [y/N] ");

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;

                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("Cancelled.");
                        return Ok(());
                    }
                }

                println!("🔄 Restoring from backup...");

                let info = manager.restore_backup(&backup_path).await?;

                println!("✅ Restore completed successfully!");
                println!();
                println!("   📦 From: {}", backup_path.display());
                println!("   🕐 Original backup: {}", info.manifest.created_at.format("%Y-%m-%d %H:%M:%S"));

                Ok(())
            }

            BackupSubcommand::List { verbose, limit } => {
                let backups = manager.list_backups()?;

                if backups.is_empty() {
                    println!("No backups found in ~/.spn/backups/");
                    return Ok(());
                }

                println!("📦 Available backups ({} total):", backups.len());
                println!();

                for (i, backup) in backups.iter().take(limit).enumerate() {
                    let marker = if i == 0 { "→" } else { " " };
                    println!(
                        "{} {} ({}, {})",
                        marker,
                        backup.path.file_name().unwrap().to_string_lossy(),
                        format_bytes(backup.size_bytes),
                        backup.timestamp.format("%Y-%m-%d %H:%M")
                    );

                    if verbose {
                        println!("     NovaNet: {} schema, {} seed files",
                            backup.manifest.contents.novanet.schema_files,
                            backup.manifest.contents.novanet.seed_files
                        );
                        println!("     Nika: {} workflows, {} sessions",
                            backup.manifest.contents.nika.workflow_files,
                            backup.manifest.contents.nika.session_count
                        );
                    }
                }

                Ok(())
            }

            BackupSubcommand::Prune { keep, execute } => {
                let backups = manager.list_backups()?;

                if backups.len() <= keep {
                    println!("✅ Nothing to prune ({} backups, keeping {})", backups.len(), keep);
                    return Ok(());
                }

                let to_delete = &backups[keep..];

                if execute {
                    println!("🗑️  Deleting {} old backups...", to_delete.len());
                    for backup in to_delete {
                        std::fs::remove_file(&backup.path)?;
                        println!("   Deleted: {}", backup.path.file_name().unwrap().to_string_lossy());
                    }
                    println!("✅ Pruned {} backups", to_delete.len());
                } else {
                    println!("🔍 Dry run - would delete {} backups:", to_delete.len());
                    for backup in to_delete {
                        println!("   {}", backup.path.file_name().unwrap().to_string_lossy());
                    }
                    println!();
                    println!("Run with --execute to actually delete");
                }

                Ok(())
            }
        }
    }
}
```

---

## Phase 4: before_seed Hook

**Goal**: Auto-backup before any `novanet seed` operation.

### Files to Modify

| File | Description |
|------|-------------|
| `novanet/tools/novanet/src/commands/seed.rs` | Add before_seed hook |

### Implementation

```rust
// In tools/novanet/src/commands/seed.rs

use std::process::Command;

/// Check if spn daemon is running and trigger backup.
fn trigger_before_seed_backup() -> Result<()> {
    // Check if hook is enabled in config
    if !is_before_seed_hook_enabled() {
        return Ok(());
    }

    println!("🔄 Creating pre-seed backup...");

    // Call spn backup create via CLI
    let output = Command::new("spn")
        .args(["backup", "create", "--label", "before-seed"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("✅ Pre-seed backup created");
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("⚠️  Pre-seed backup failed: {}", stderr);
            // Don't block seed on backup failure
            Ok(())
        }
        Err(e) => {
            eprintln!("⚠️  spn not found, skipping backup: {}", e);
            Ok(())
        }
    }
}

fn is_before_seed_hook_enabled() -> bool {
    // Check ~/.novanet/config.toml for:
    // [hooks]
    // before_seed_backup = true
    let config_path = dirs::home_dir()
        .map(|h| h.join(".novanet/config.toml"));

    config_path
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|content| content.contains("before_seed_backup = true"))
        .unwrap_or(false)
}

// In the seed command handler:
pub async fn execute(&self, args: &SeedArgs) -> Result<()> {
    // Trigger pre-seed backup hook
    trigger_before_seed_backup()?;

    // Continue with normal seed operation...
    match &args.command {
        SeedSubcommand::Generate { .. } => self.generate(args).await,
        SeedSubcommand::Apply { .. } => self.apply(args).await,
        SeedSubcommand::Diff { .. } => self.diff(args).await,
        // ...
    }
}
```

### Configuration

```toml
# ~/.novanet/config.toml
[hooks]
before_seed_backup = true
```

---

## Testing Strategy

### Unit Tests

| Test Suite | Coverage |
|------------|----------|
| `backup::manager` | BackupManager lifecycle |
| `backup::manifest` | Serialization/deserialization |
| `backup::adapters::*` | Each adapter collect/restore |
| `backup::error` | Error handling |

### Integration Tests

```rust
#[tokio::test]
async fn test_full_backup_restore_cycle() {
    let temp_dir = TempDir::new().unwrap();

    // Setup mock data
    setup_mock_novanet_data(&temp_dir);
    setup_mock_nika_data(&temp_dir);
    setup_mock_spn_data(&temp_dir);

    // Create backup
    let mut manager = BackupManager::new();
    manager.backup_dir = temp_dir.path().join("backups");
    // ... register adapters

    let backup = manager.create_backup(Some("test")).await.unwrap();
    assert!(backup.path.exists());

    // Corrupt original data
    corrupt_mock_data(&temp_dir);

    // Restore
    manager.restore_backup(&backup.path).await.unwrap();

    // Verify restoration
    assert_data_matches_original(&temp_dir);
}
```

### CLI Tests

```bash
# Test create
spn backup create --label "test-backup"
# Expected: Backup created successfully

# Test list
spn backup list
# Expected: Shows backup with label

# Test restore
spn backup restore latest --force
# Expected: Restore completed

# Test prune
spn backup prune --keep 1 --execute
# Expected: Older backups deleted
```

---

## Rollout Plan

### Week 1: Core Implementation
- [ ] Phase 1: BackupManager core
- [ ] Phase 2: System adapters
- [ ] Unit tests for all components

### Week 2: CLI & Integration
- [ ] Phase 3: CLI commands
- [ ] Phase 4: before_seed hook
- [ ] Integration tests

### Week 3: Polish & Release
- [ ] Documentation updates
- [ ] CHANGELOG update
- [ ] Version bump (spn 0.15.0)
- [ ] Release

---

## Success Criteria

| Criterion | Metric |
|-----------|--------|
| **Functionality** | `spn backup create/restore/list` work end-to-end |
| **Safety** | before_seed hook auto-backups work |
| **Performance** | Backup < 30s for typical project |
| **Reliability** | Restore recovers exact state |
| **Security** | No secrets in backup files |
| **UX** | Clear CLI output with progress |

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Large Neo4j dumps | v1 excludes Neo4j, add optional flag later |
| Disk space | Retention policy with prune command |
| Cross-version restore | Manifest tracks versions, warn on mismatch |
| Partial restore failure | Atomic staging dir, rollback on error |

---

## Future Enhancements (v2+)

- [ ] Neo4j dump/restore (optional)
- [ ] Scheduling (`spn backup schedule daily`)
- [ ] Remote storage (S3, GCS)
- [ ] Encryption for backups
- [ ] Differential backups
- [ ] Cross-machine sync (`spn backup push/pull`)
