//! Parse trait YAML definitions from `packages/core/models/traits/`.
//!
//! Each trait defines a "data origin" classification (defined/authored/imported/generated/retrieved).
//! v0.12.5: Individual trait files replace taxonomy.yaml node_traits section.
//!
//! Note: Module named `trait_def` because `trait` is a Rust keyword.

use serde::{Deserialize, Serialize};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Dual-format icon (web + terminal).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitIcon {
    pub web: String,
    pub terminal: String,
}

impl Default for TraitIcon {
    fn default() -> Self {
        Self {
            web: "square".to_string(),
            terminal: "■".to_string(),
        }
    }
}

/// Context budget configuration for a trait.
#[derive(Debug, Clone, Deserialize)]
pub struct ContextBudget {
    pub min: u32,
    pub max: u32,
    pub default: u32,
}

impl Default for ContextBudget {
    fn default() -> Self {
        Self {
            min: 500,
            max: 2000,
            default: 1000,
        }
    }
}

/// Default retrieval settings for a trait.
#[derive(Debug, Clone, Deserialize)]
pub struct RetrievalDefaults {
    pub traversal_depth: u32,
    pub context_budget: u32,
    pub token_estimate: u32,
}

impl Default for RetrievalDefaults {
    fn default() -> Self {
        Self {
            traversal_depth: 2,
            context_budget: 500,
            token_estimate: 100,
        }
    }
}

/// A trait definition parsed from `<trait-key>.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct TraitDef {
    /// Unique key (e.g., "defined", "authored", "imported", "generated", "retrieved").
    pub key: String,

    /// Human-readable name.
    pub display_name: String,

    /// Multi-line description.
    pub description: String,

    /// Hex color for visual encoding.
    pub color: String,

    /// Border style for node rendering (solid, dashed, dotted, double).
    #[serde(default = "default_border_style")]
    pub border_style: String,

    /// Border width in pixels.
    #[serde(default = "default_border_width")]
    pub border_width: u32,

    /// Unicode character for terminal border rendering.
    #[serde(default)]
    pub unicode_border: Option<String>,

    /// Dual-format icon.
    #[serde(default)]
    pub icon: TraitIcon,

    /// Node count with this trait (computed by generator).
    #[serde(default)]
    pub node_count: u32,

    /// Context budget configuration.
    #[serde(default)]
    pub context_budget: ContextBudget,

    /// Default retrieval settings.
    #[serde(default)]
    pub retrieval_defaults: RetrievalDefaults,

    /// Example classes with this trait.
    #[serde(default)]
    pub examples: Vec<String>,

    /// LLM context string (USE/TRIGGERS/NOT/RELATES pattern).
    #[serde(default)]
    pub llm_context: Option<String>,
}

fn default_border_style() -> String {
    "solid".to_string()
}

fn default_border_width() -> u32 {
    2
}

/// Document wrapper for trait YAML files.
#[derive(Debug, Clone, Deserialize)]
pub struct TraitDoc {
    #[serde(rename = "trait")]
    pub trait_def: TraitDef,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loaders
// ─────────────────────────────────────────────────────────────────────────────

/// Load all trait YAML files from `packages/core/models/traits/`.
/// Skips files starting with `_` (index files).
pub fn load_all_traits(root: &Path) -> crate::Result<Vec<TraitDef>> {
    let traits_dir = crate::config::traits_dir(root);
    if !traits_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "traits directory not found: {}",
            traits_dir.display()
        )));
    }

    let mut entries: Vec<_> = std::fs::read_dir(&traits_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".yaml") && !name.starts_with('_')
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut traits = Vec::with_capacity(entries.len());
    for entry in entries {
        let doc: TraitDoc = super::utils::load_yaml(&entry.path())?;
        traits.push(doc.trait_def);
    }

    Ok(traits)
}

/// Load a single trait by key.
pub fn load_trait(root: &Path, key: &str) -> crate::Result<TraitDef> {
    let path = crate::config::traits_dir(root).join(format!("{key}.yaml"));
    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "trait '{key}' not found (expected: {})",
            path.display()
        )));
    }
    let doc: TraitDoc = super::utils::load_yaml(&path)?;
    Ok(doc.trait_def)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_trait() {
        let yaml = r##"
trait:
  key: test
  display_name: Test Trait
  description: A test trait
  color: "#ff0000"
"##;
        let doc: TraitDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.trait_def.key, "test");
        assert_eq!(doc.trait_def.display_name, "Test Trait");
        assert_eq!(doc.trait_def.color, "#ff0000");
        assert_eq!(doc.trait_def.border_style, "solid");
        assert_eq!(doc.trait_def.border_width, 2);
    }

    #[test]
    fn parse_trait_with_all_fields() {
        let yaml = r##"
trait:
  key: defined
  display_name: Defined
  description: |
    Human-created ONCE. Structural definitions.
  color: "#3b82f6"
  border_style: solid
  border_width: 2
  unicode_border: "-"
  icon:
    web: square
    terminal: "S"
  node_count: 31
  context_budget:
    min: 500
    max: 2000
    default: 1000
  retrieval_defaults:
    traversal_depth: 2
    context_budget: 500
    token_estimate: 100
  examples:
    - Page
    - Block
    - Entity
  llm_context: |
    USE: when working with structural definitions.
    TRIGGERS: "template", "structure", "schema".
"##;
        let doc: TraitDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.trait_def.key, "defined");
        assert_eq!(doc.trait_def.border_style, "solid");
        assert_eq!(doc.trait_def.unicode_border, Some("-".to_string()));
        assert_eq!(doc.trait_def.node_count, 31);
        assert_eq!(doc.trait_def.context_budget.min, 500);
        assert_eq!(doc.trait_def.context_budget.max, 2000);
        assert_eq!(doc.trait_def.retrieval_defaults.traversal_depth, 2);
        assert_eq!(doc.trait_def.examples.len(), 3);
        assert!(doc.trait_def.examples.contains(&"Page".to_string()));
        assert!(doc.trait_def.llm_context.is_some());
        assert_eq!(doc.trait_def.icon.terminal, "S");
    }

    #[test]
    fn parse_trait_with_different_border_styles() {
        for (style, width) in [
            ("solid", 2),
            ("dashed", 2),
            ("dotted", 2),
            ("double", 3),
        ] {
            let yaml = format!(
                r##"
trait:
  key: test
  display_name: Test
  description: Test
  color: "#000"
  border_style: {style}
  border_width: {width}
"##
            );
            let doc: TraitDoc = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(doc.trait_def.border_style, style);
            assert_eq!(doc.trait_def.border_width, width);
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
    fn load_all_traits_integration() {
        let Some(root) = test_root() else { return };
        let traits = load_all_traits(&root).expect("should load all traits");
        assert_eq!(
            traits.len(),
            5,
            "expected 5 traits (defined, authored, imported, generated, retrieved)"
        );

        let keys: Vec<&str> = traits.iter().map(|t| t.key.as_str()).collect();
        assert!(keys.contains(&"defined"));
        assert!(keys.contains(&"authored"));
        assert!(keys.contains(&"imported"));
        assert!(keys.contains(&"generated"));
        assert!(keys.contains(&"retrieved"));
    }

    #[test]
    fn load_single_trait_integration() {
        let Some(root) = test_root() else { return };
        let trait_def = load_trait(&root, "defined").expect("should load defined trait");
        assert_eq!(trait_def.key, "defined");
        assert_eq!(trait_def.display_name, "Defined");
        assert_eq!(trait_def.border_style, "solid");
    }

    #[test]
    fn load_nonexistent_trait_returns_error() {
        let Some(root) = test_root() else { return };
        let result = load_trait(&root, "nonexistent");
        assert!(result.is_err());
    }
}
