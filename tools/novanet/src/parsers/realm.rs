//! Parse realm YAML definitions from `packages/core/models/realms/`.
//!
//! Each realm defines a scope (shared vs org) with associated layers.
//! v0.12.5: Individual realm files replace taxonomy.yaml node_realms section.

use serde::{Deserialize, Serialize};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Dual-format icon (web + terminal).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealmIcon {
    pub web: String,
    pub terminal: String,
}

impl Default for RealmIcon {
    fn default() -> Self {
        Self {
            web: "globe".to_string(),
            terminal: "●".to_string(),
        }
    }
}

/// A realm definition parsed from `<realm-key>.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct RealmDef {
    /// Unique key (e.g., "shared", "org").
    pub key: String,

    /// Human-readable name.
    pub display_name: String,

    /// Multi-line description.
    pub description: String,

    /// Hex color for visual encoding.
    pub color: String,

    /// Dual-format icon.
    #[serde(default)]
    pub icon: RealmIcon,

    /// Layers belonging to this realm (keys).
    #[serde(default)]
    pub layers: Vec<String>,

    /// Node count (computed by generator).
    #[serde(default)]
    pub node_count: u32,

    /// LLM context string (USE/TRIGGERS/NOT/RELATES pattern).
    #[serde(default)]
    pub llm_context: Option<String>,
}

/// Document wrapper for realm YAML files.
#[derive(Debug, Clone, Deserialize)]
pub struct RealmDoc {
    pub realm: RealmDef,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loaders
// ─────────────────────────────────────────────────────────────────────────────

/// Load all realm YAML files from `packages/core/models/realms/`.
/// Skips files starting with `_` (index files).
pub fn load_all_realms(root: &Path) -> crate::Result<Vec<RealmDef>> {
    let realms_dir = crate::config::realms_dir(root);
    if !realms_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "realms directory not found: {}",
            realms_dir.display()
        )));
    }

    let mut entries: Vec<_> = std::fs::read_dir(&realms_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".yaml") && !name.starts_with('_')
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut realms = Vec::with_capacity(entries.len());
    for entry in entries {
        let doc: RealmDoc = super::utils::load_yaml(&entry.path())?;
        realms.push(doc.realm);
    }

    Ok(realms)
}

/// Load a single realm by key.
pub fn load_realm(root: &Path, key: &str) -> crate::Result<RealmDef> {
    let path = crate::config::realms_dir(root).join(format!("{key}.yaml"));
    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "realm '{key}' not found (expected: {})",
            path.display()
        )));
    }
    let doc: RealmDoc = super::utils::load_yaml(&path)?;
    Ok(doc.realm)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_realm() {
        let yaml = r##"
realm:
  key: test
  display_name: Test Realm
  description: A test realm
  color: "#ff0000"
"##;
        let doc: RealmDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.realm.key, "test");
        assert_eq!(doc.realm.display_name, "Test Realm");
        assert_eq!(doc.realm.color, "#ff0000");
        assert!(doc.realm.layers.is_empty());
    }

    #[test]
    fn parse_realm_with_layers() {
        let yaml = r##"
realm:
  key: shared
  display_name: Shared
  description: |
    Universal knowledge (READ-ONLY).
  color: "#2aa198"
  icon:
    web: globe
    terminal: "O"
  layers:
    - config
    - locale
    - geography
    - knowledge
  node_count: 39
  llm_context: |
    USE: when accessing universal locale knowledge.
    TRIGGERS: "shared data", "universal", "read-only".
"##;
        let doc: RealmDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.realm.key, "shared");
        assert_eq!(doc.realm.layers.len(), 4);
        assert_eq!(doc.realm.layers[0], "config");
        assert_eq!(doc.realm.node_count, 39);
        assert!(doc.realm.llm_context.is_some());
        assert_eq!(doc.realm.icon.web, "globe");
        assert_eq!(doc.realm.icon.terminal, "O");
    }

    fn test_root() -> Option<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn load_all_realms_integration() {
        let Some(root) = test_root() else { return };
        let realms = load_all_realms(&root).expect("should load all realms");
        assert_eq!(realms.len(), 2, "expected 2 realms (shared, org)");

        let keys: Vec<&str> = realms.iter().map(|r| r.key.as_str()).collect();
        assert!(keys.contains(&"shared"));
        assert!(keys.contains(&"org"));
    }

    #[test]
    fn load_single_realm_integration() {
        let Some(root) = test_root() else { return };
        let realm = load_realm(&root, "shared").expect("should load shared realm");
        assert_eq!(realm.key, "shared");
        assert_eq!(realm.display_name, "Shared");
        assert!(!realm.layers.is_empty());
    }

    #[test]
    fn load_nonexistent_realm_returns_error() {
        let Some(root) = test_root() else { return };
        let result = load_realm(&root, "nonexistent");
        assert!(result.is_err());
    }
}
