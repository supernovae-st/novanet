//! Incremental export checkpoint tracking.
//!
//! Tracks the last export timestamp per node class so that subsequent
//! exports can use `--incremental` to only fetch newly updated nodes.
//!
//! Checkpoint file: `<export_dir>/.checkpoint.json`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Root checkpoint structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCheckpoint {
    /// Schema version for forward compatibility.
    pub version: u32,
    /// Per-class export records.
    pub exports: HashMap<String, ClassCheckpoint>,
}

/// Per-class checkpoint data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassCheckpoint {
    /// ISO 8601 timestamp of last export.
    pub last_export: DateTime<Utc>,
    /// Number of nodes exported.
    pub node_count: usize,
    /// Filters that were active during export.
    pub filters: CheckpointFilters,
}

/// Filters captured at export time.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckpointFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

const CHECKPOINT_FILENAME: &str = ".checkpoint.json";
const CHECKPOINT_VERSION: u32 = 1;

impl ExportCheckpoint {
    /// Create an empty checkpoint.
    pub fn new() -> Self {
        Self {
            version: CHECKPOINT_VERSION,
            exports: HashMap::new(),
        }
    }

    /// Load checkpoint from the export directory.
    ///
    /// Returns an empty checkpoint if the file does not exist.
    pub fn load(export_dir: &Path) -> crate::Result<Self> {
        let path = export_dir.join(CHECKPOINT_FILENAME);
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = std::fs::read_to_string(&path)?;
        let checkpoint: Self = serde_json::from_str(&content).map_err(|e| {
            crate::NovaNetError::Validation(format!(
                "Invalid checkpoint file {}: {e}",
                path.display()
            ))
        })?;
        Ok(checkpoint)
    }

    /// Save checkpoint to the export directory.
    pub fn save(&self, export_dir: &Path) -> crate::Result<()> {
        std::fs::create_dir_all(export_dir)?;
        let path = export_dir.join(CHECKPOINT_FILENAME);
        let content = serde_json::to_string_pretty(self).map_err(|e| {
            crate::NovaNetError::Validation(format!("Failed to serialize checkpoint: {e}"))
        })?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// Get the last export timestamp for a given class.
    pub fn get_since(&self, class: &str) -> Option<DateTime<Utc>> {
        self.exports.get(class).map(|c| c.last_export)
    }

    /// Record an export for a class.
    pub fn record(
        &mut self,
        class: &str,
        count: usize,
        project: Option<&str>,
        locale: Option<&str>,
    ) {
        self.exports.insert(
            class.to_string(),
            ClassCheckpoint {
                last_export: Utc::now(),
                node_count: count,
                filters: CheckpointFilters {
                    project: project.map(|s| s.to_string()),
                    locale: locale.map(|s| s.to_string()),
                },
            },
        );
    }
}

impl Default for ExportCheckpoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn new_checkpoint_is_empty() {
        let cp = ExportCheckpoint::new();
        assert_eq!(cp.version, CHECKPOINT_VERSION);
        assert!(cp.exports.is_empty());
    }

    #[test]
    fn load_missing_file_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let cp = ExportCheckpoint::load(tmp.path()).unwrap();
        assert!(cp.exports.is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let mut cp = ExportCheckpoint::new();
        cp.record("Entity", 42, Some("qrcode-ai"), None);
        cp.save(tmp.path()).unwrap();

        let loaded = ExportCheckpoint::load(tmp.path()).unwrap();
        assert_eq!(loaded.version, CHECKPOINT_VERSION);
        assert_eq!(loaded.exports.len(), 1);
        let entity = loaded.exports.get("Entity").unwrap();
        assert_eq!(entity.node_count, 42);
        assert_eq!(entity.filters.project.as_deref(), Some("qrcode-ai"));
        assert!(entity.filters.locale.is_none());
    }

    #[test]
    fn get_since_returns_none_for_unknown_class() {
        let cp = ExportCheckpoint::new();
        assert!(cp.get_since("Unknown").is_none());
    }

    #[test]
    fn get_since_returns_timestamp_for_recorded_class() {
        let mut cp = ExportCheckpoint::new();
        cp.record("EntityNative", 100, None, Some("fr-FR"));
        assert!(cp.get_since("EntityNative").is_some());
    }

    #[test]
    fn record_overwrites_previous_entry() {
        let mut cp = ExportCheckpoint::new();
        cp.record("Entity", 10, None, None);
        cp.record("Entity", 20, Some("new-project"), None);
        let entry = cp.exports.get("Entity").unwrap();
        assert_eq!(entry.node_count, 20);
        assert_eq!(entry.filters.project.as_deref(), Some("new-project"));
    }
}
