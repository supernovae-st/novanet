//! Parse arc family YAML definitions from `packages/core/models/arc-families/`.
//!
//! Each arc family defines a functional group of arcs (ownership, localization, semantic, etc.).
//! v0.12.5: Individual arc family files replace taxonomy.yaml arc_families section.

use serde::{Deserialize, Serialize};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Dual-format icon (web + terminal).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArcFamilyIcon {
    pub web: String,
    pub terminal: String,
}

impl Default for ArcFamilyIcon {
    fn default() -> Self {
        Self {
            web: "arrow-right".to_string(),
            terminal: "→".to_string(),
        }
    }
}

/// Default traversal behavior for arcs in this family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TraversalMode {
    #[default]
    Eager,
    Lazy,
    OnDemand,
    Skip,
}

/// An arc family definition parsed from `<family-key>.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcFamilyDef {
    /// Unique key (e.g., "ownership", "localization", "semantic").
    pub key: String,

    /// Human-readable name.
    pub display_name: String,

    /// Multi-line description.
    pub description: String,

    /// Hex color for visual encoding (arc stroke color).
    pub color: String,

    /// Stroke style for arc rendering (solid, dashed).
    #[serde(default = "default_stroke_style")]
    pub stroke_style: String,

    /// Stroke width in pixels.
    #[serde(default = "default_stroke_width")]
    pub stroke_width: u32,

    /// Arrow style for diagrams (e.g., "-->", "-.->").
    #[serde(default = "default_arrow_style")]
    pub arrow_style: String,

    /// Default traversal behavior.
    #[serde(default)]
    pub default_traversal: TraversalMode,

    /// Dual-format icon.
    #[serde(default)]
    pub icon: ArcFamilyIcon,

    /// Arc count in this family (computed by generator).
    #[serde(default)]
    pub arc_count: u32,

    /// Inverse naming convention (e.g., "*_OF" for ownership).
    #[serde(default)]
    pub inverse_convention: Option<String>,

    /// Tier 1 inverse pairs (required per ADR-026).
    #[serde(default)]
    pub tier_1_inverses: Vec<String>,

    /// Key arcs in this family (most commonly used).
    #[serde(default)]
    pub key_arcs: Vec<String>,

    /// v0.20.0: Machine-readable routing keywords (max 10, lowercase, English).
    #[serde(default)]
    pub triggers: Vec<String>,
}

fn default_stroke_style() -> String {
    "solid".to_string()
}

fn default_stroke_width() -> u32 {
    2
}

fn default_arrow_style() -> String {
    "-->".to_string()
}

/// Document wrapper for arc family YAML files.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcFamilyDoc {
    pub arc_family: ArcFamilyDef,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loaders
// ─────────────────────────────────────────────────────────────────────────────

/// Load all arc family YAML files from `packages/core/models/arc-families/`.
/// Skips files starting with `_` (index files).
pub fn load_all_arc_families(root: &Path) -> crate::Result<Vec<ArcFamilyDef>> {
    let arc_families_dir = crate::config::arc_families_dir(root);
    if !arc_families_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "arc-families directory not found: {}",
            arc_families_dir.display()
        )));
    }

    let mut entries: Vec<_> = std::fs::read_dir(&arc_families_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".yaml") && !name.starts_with('_')
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut families = Vec::with_capacity(entries.len());
    for entry in entries {
        let doc: ArcFamilyDoc = super::utils::load_yaml(&entry.path())?;
        families.push(doc.arc_family);
    }

    Ok(families)
}

/// Load a single arc family by key.
pub fn load_arc_family(root: &Path, key: &str) -> crate::Result<ArcFamilyDef> {
    let path = crate::config::arc_families_dir(root).join(format!("{key}.yaml"));
    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "arc family '{key}' not found (expected: {})",
            path.display()
        )));
    }
    let doc: ArcFamilyDoc = super::utils::load_yaml(&path)?;
    Ok(doc.arc_family)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_arc_family() {
        let yaml = r##"
arc_family:
  key: test
  display_name: Test Family
  description: A test arc family
  color: "#ff0000"
"##;
        let doc: ArcFamilyDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.arc_family.key, "test");
        assert_eq!(doc.arc_family.display_name, "Test Family");
        assert_eq!(doc.arc_family.color, "#ff0000");
        assert_eq!(doc.arc_family.stroke_style, "solid");
        assert_eq!(doc.arc_family.stroke_width, 2);
        assert_eq!(doc.arc_family.arrow_style, "-->");
    }

    #[test]
    fn parse_arc_family_with_all_fields() {
        let yaml = r##"
arc_family:
  key: ownership
  display_name: Ownership
  description: |
    Parent-Child hierarchy relationships.
  color: "#3b82f6"
  stroke_style: solid
  stroke_width: 2
  arrow_style: "-->"
  default_traversal: eager
  icon:
    web: git-branch
    terminal: "->"
  arc_count: 43
  inverse_convention: "*_OF"
  tier_1_inverses:
    - HAS_ENTITY / ENTITY_OF
    - HAS_PAGE / PAGE_OF
  key_arcs:
    - HAS_PROJECT
    - HAS_PAGE
    - HAS_ENTITY
  triggers: ["parent", "child", "contains", "belongs to", "owns"]
"##;
        let doc: ArcFamilyDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.arc_family.key, "ownership");
        assert_eq!(doc.arc_family.default_traversal, TraversalMode::Eager);
        assert_eq!(doc.arc_family.arc_count, 43);
        assert_eq!(doc.arc_family.inverse_convention, Some("*_OF".to_string()));
        assert_eq!(doc.arc_family.tier_1_inverses.len(), 2);
        assert_eq!(doc.arc_family.key_arcs.len(), 3);
        assert_eq!(doc.arc_family.triggers.len(), 5);
        assert_eq!(doc.arc_family.icon.terminal, "->");
    }

    #[test]
    fn parse_traversal_modes() {
        for (mode_str, expected) in [
            ("eager", TraversalMode::Eager),
            ("lazy", TraversalMode::Lazy),
            ("ondemand", TraversalMode::OnDemand),
        ] {
            let yaml = format!(
                r##"
arc_family:
  key: test
  display_name: Test
  description: Test
  color: "#000"
  default_traversal: {mode_str}
"##
            );
            let doc: ArcFamilyDoc = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(doc.arc_family.default_traversal, expected);
        }
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
    fn load_all_arc_families_integration() {
        let Some(root) = test_root() else { return };
        let families = load_all_arc_families(&root).expect("should load all arc families");
        assert_eq!(
            families.len(),
            6,
            "expected 6 arc families (ownership, localization, semantic, generation, mining, schema)"
        );

        let keys: Vec<&str> = families.iter().map(|f| f.key.as_str()).collect();
        assert!(keys.contains(&"ownership"));
        assert!(keys.contains(&"localization"));
        assert!(keys.contains(&"semantic"));
        assert!(keys.contains(&"generation"));
        assert!(keys.contains(&"mining"));
    }

    #[test]
    fn load_single_arc_family_integration() {
        let Some(root) = test_root() else { return };
        let family = load_arc_family(&root, "ownership").expect("should load ownership family");
        assert_eq!(family.key, "ownership");
        assert_eq!(family.display_name, "Ownership");
        assert!(!family.key_arcs.is_empty());
    }

    #[test]
    fn load_nonexistent_arc_family_returns_error() {
        let Some(root) = test_root() else { return };
        let result = load_arc_family(&root, "nonexistent");
        assert!(result.is_err());
    }
}
