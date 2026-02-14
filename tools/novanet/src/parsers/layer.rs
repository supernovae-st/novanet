//! Parse layer YAML definitions from `packages/core/models/layers/`.
//!
//! Each layer defines a functional category within a realm.
//! v0.12.5: Individual layer files replace taxonomy.yaml node_layers section.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Dual-format icon (web + terminal).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerIcon {
    pub web: String,
    pub terminal: String,
}

impl Default for LayerIcon {
    fn default() -> Self {
        Self {
            web: "layers".to_string(),
            terminal: "◇".to_string(),
        }
    }
}

/// Classes can be a simple list or grouped by realm.
/// Note: ByRealm must come first for serde untagged to try map before sequence.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LayerClasses {
    /// Classes grouped by realm (e.g., { shared: [...], org: [...] }).
    /// Must be first to be tried before Simple (serde untagged order matters).
    ByRealm(HashMap<String, Vec<String>>),
    /// Simple list of class names.
    Simple(Vec<String>),
}

impl Default for LayerClasses {
    fn default() -> Self {
        Self::Simple(Vec::new())
    }
}

impl LayerClasses {
    /// Get all classes as a flat list.
    pub fn all(&self) -> Vec<&str> {
        match self {
            Self::Simple(classes) => classes.iter().map(|s| s.as_str()).collect(),
            Self::ByRealm(map) => map.values().flatten().map(|s| s.as_str()).collect(),
        }
    }

    /// Get classes for a specific realm.
    pub fn for_realm(&self, realm: &str) -> Vec<&str> {
        match self {
            Self::Simple(classes) => classes.iter().map(|s| s.as_str()).collect(),
            Self::ByRealm(map) => map
                .get(realm)
                .map(|v| v.iter().map(|s| s.as_str()).collect())
                .unwrap_or_default(),
        }
    }

    /// Check if there are no classes.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Simple(classes) => classes.is_empty(),
            Self::ByRealm(map) => map.values().all(|v| v.is_empty()),
        }
    }

    /// Get total number of classes.
    pub fn len(&self) -> usize {
        match self {
            Self::Simple(classes) => classes.len(),
            Self::ByRealm(map) => map.values().map(|v| v.len()).sum(),
        }
    }

    /// Check if a class is contained.
    pub fn contains(&self, class: &str) -> bool {
        self.all().contains(&class)
    }
}

/// A layer definition parsed from `<layer-key>.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct LayerDef {
    /// Unique key (e.g., "config", "semantic", "output").
    pub key: String,

    /// Human-readable name.
    pub display_name: String,

    /// Multi-line description.
    pub description: String,

    /// Hex color for visual encoding (fill color for nodes).
    pub color: String,

    /// Dual-format icon.
    #[serde(default)]
    pub icon: LayerIcon,

    /// Realms this layer appears in (keys).
    #[serde(default)]
    pub realms: Vec<String>,

    /// Classes belonging to this layer (simple list or by realm).
    #[serde(default)]
    pub classes: LayerClasses,

    /// Class count (computed by generator).
    #[serde(default)]
    pub class_count: u32,

    /// LLM context string (USE/TRIGGERS/NOT/RELATES pattern).
    #[serde(default)]
    pub llm_context: Option<String>,
}

/// Document wrapper for layer YAML files.
#[derive(Debug, Clone, Deserialize)]
pub struct LayerDoc {
    pub layer: LayerDef,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loaders
// ─────────────────────────────────────────────────────────────────────────────

/// Load all layer YAML files from `packages/core/models/layers/`.
/// Skips files starting with `_` (index files).
pub fn load_all_layers(root: &Path) -> crate::Result<Vec<LayerDef>> {
    let layers_dir = crate::config::layers_dir(root);
    if !layers_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "layers directory not found: {}",
            layers_dir.display()
        )));
    }

    let mut entries: Vec<_> = std::fs::read_dir(&layers_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".yaml") && !name.starts_with('_')
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut layers = Vec::with_capacity(entries.len());
    for entry in entries {
        let doc: LayerDoc = super::utils::load_yaml(&entry.path())?;
        layers.push(doc.layer);
    }

    Ok(layers)
}

/// Load a single layer by key.
pub fn load_layer(root: &Path, key: &str) -> crate::Result<LayerDef> {
    let path = crate::config::layers_dir(root).join(format!("{key}.yaml"));
    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "layer '{key}' not found (expected: {})",
            path.display()
        )));
    }
    let doc: LayerDoc = super::utils::load_yaml(&path)?;
    Ok(doc.layer)
}

/// Load layers for a specific realm.
pub fn load_layers_for_realm(root: &Path, realm_key: &str) -> crate::Result<Vec<LayerDef>> {
    let all_layers = load_all_layers(root)?;
    Ok(all_layers
        .into_iter()
        .filter(|l| l.realms.iter().any(|r| r == realm_key))
        .collect())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_layer() {
        let yaml = r##"
layer:
  key: test
  display_name: Test Layer
  description: A test layer
  color: "#ff0000"
"##;
        let doc: LayerDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.layer.key, "test");
        assert_eq!(doc.layer.display_name, "Test Layer");
        assert_eq!(doc.layer.color, "#ff0000");
        assert!(doc.layer.realms.is_empty());
        assert!(doc.layer.classes.is_empty());
    }

    #[test]
    fn parse_layer_with_classes() {
        let yaml = r##"
layer:
  key: semantic
  display_name: Semantic
  description: |
    Meaning and knowledge relationships.
  color: "#f97316"
  icon:
    web: brain
    terminal: "D"
  realms:
    - org
  classes:
    - Entity
    - EntityContent
    - AudiencePersona
    - ChannelSurface
  class_count: 4
  llm_context: |
    USE: when working with semantic entities.
    TRIGGERS: "entity", "meaning", "audience".
"##;
        let doc: LayerDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.layer.key, "semantic");
        assert_eq!(doc.layer.realms, vec!["org"]);
        assert_eq!(doc.layer.classes.len(), 4);
        assert!(doc.layer.classes.all().contains(&"Entity"));
        assert_eq!(doc.layer.class_count, 4);
        assert!(doc.layer.llm_context.is_some());
        assert_eq!(doc.layer.icon.web, "brain");
    }

    #[test]
    fn parse_layer_with_multiple_realms() {
        let yaml = r##"
layer:
  key: config
  display_name: Config
  description: Configuration layer
  color: "#6366f1"
  realms:
    - shared
    - org
  classes:
    - Locale
    - OrgConfig
"##;
        let doc: LayerDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.layer.realms.len(), 2);
        assert!(doc.layer.realms.contains(&"shared".to_string()));
        assert!(doc.layer.realms.contains(&"org".to_string()));
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
    fn load_all_layers_integration() {
        let Some(root) = test_root() else { return };
        let layers = load_all_layers(&root).expect("should load all layers");
        assert!(
            layers.len() >= 9,
            "expected at least 9 layers, got {}",
            layers.len()
        );

        let keys: Vec<&str> = layers.iter().map(|l| l.key.as_str()).collect();
        assert!(keys.contains(&"config"));
        assert!(keys.contains(&"semantic"));
        assert!(keys.contains(&"output"));
    }

    #[test]
    fn load_single_layer_integration() {
        let Some(root) = test_root() else { return };
        let layer = load_layer(&root, "semantic").expect("should load semantic layer");
        assert_eq!(layer.key, "semantic");
        assert_eq!(layer.display_name, "Semantic");
        assert!(!layer.classes.is_empty());
    }

    #[test]
    fn load_layers_for_realm_integration() {
        let Some(root) = test_root() else { return };
        let org_layers = load_layers_for_realm(&root, "org").expect("should load org layers");
        assert!(
            org_layers.len() >= 5,
            "expected at least 5 org layers, got {}",
            org_layers.len()
        );

        for layer in &org_layers {
            assert!(
                layer.realms.contains(&"org".to_string()),
                "layer {} should be in org realm",
                layer.key
            );
        }
    }

    #[test]
    fn load_nonexistent_layer_returns_error() {
        let Some(root) = test_root() else { return };
        let result = load_layer(&root, "nonexistent");
        assert!(result.is_err());
    }
}
