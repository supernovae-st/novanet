//! Parse taxonomy.yaml (v9.5 replacement for organizing-principles.yaml).
//!
//! Handles:
//! - `node_realms` with nested layers
//! - `node_traits` with border styles for visual encoding
//! - `arc_families` with stroke styles, arrow styles, and default_traversal
//! - `arc_scopes` and `arc_cardinalities` for arc classification
//! - `terminal` palette for TUI graceful degradation
//! - `kind_retrieval_defaults` per-trait context assembly settings (v9.9)

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs (taxonomy.yaml)
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level document for taxonomy.yaml.
#[derive(Debug, Deserialize)]
pub struct TaxonomyDoc {
    pub version: String,
    pub node_realms: Vec<NodeRealmDef>,
    pub node_traits: Vec<NodeTraitDef>,
    /// v9.9: Per-trait retrieval defaults for context assembly.
    #[serde(default)]
    pub kind_retrieval_defaults: Option<HashMap<String, KindRetrievalDefaults>>,
    pub arc_families: Vec<ArcFamilyDef>,
    #[serde(default)]
    pub arc_scopes: Vec<ArcScopeDef>,
    #[serde(default)]
    pub arc_cardinalities: Vec<ArcCardinalityDef>,
    #[serde(default)]
    pub terminal: Option<TerminalPalette>,
}

/// v9.9: Per-trait retrieval settings for context assembly.
#[derive(Debug, Clone, Deserialize)]
pub struct KindRetrievalDefaults {
    /// Maximum hops for structural traversal.
    #[serde(default)]
    pub traversal_depth: Option<u8>,
    /// Default token allocation for this trait type.
    #[serde(default)]
    pub context_budget: Option<u32>,
    /// Estimated tokens per instance.
    #[serde(default)]
    pub token_estimate: Option<u32>,
}

/// Realm definition with nested layers.
#[derive(Debug, Clone, Deserialize)]
pub struct NodeRealmDef {
    pub key: String,
    pub display_name: String,
    pub emoji: String,
    pub color: String,
    pub llm_context: String,
    pub layers: Vec<NodeLayerDef>,
}

/// Layer definition (nested under its realm).
#[derive(Debug, Clone, Deserialize)]
pub struct NodeLayerDef {
    pub key: String,
    pub display_name: String,
    pub emoji: String,
    pub color: String,
    pub llm_context: String,
}

/// Trait (locale behavior) definition with visual encoding.
#[derive(Debug, Clone, Deserialize)]
pub struct NodeTraitDef {
    pub key: String,
    pub display_name: String,
    pub color: String,
    #[serde(default)]
    pub border_style: Option<String>,
    #[serde(default)]
    pub border_width: Option<u8>,
    #[serde(default)]
    pub unicode_border: Option<String>,
    pub llm_context: String,
}

/// Arc family definition with visual encoding.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcFamilyDef {
    pub key: String,
    pub display_name: String,
    pub color: String,
    #[serde(default)]
    pub stroke_style: Option<String>,
    #[serde(default)]
    pub stroke_width: Option<u8>,
    pub arrow_style: String,
    /// v9.9: Default traversal behavior (eager/lazy/skip).
    #[serde(default)]
    pub default_traversal: Option<String>,
    pub llm_context: String,
}

/// Arc scope definition (intra_realm / cross_realm).
#[derive(Debug, Clone, Deserialize)]
pub struct ArcScopeDef {
    pub key: String,
    pub display_name: String,
    pub description: String,
    #[serde(default)]
    pub stroke_modifier: Option<String>,
}

/// Arc cardinality definition.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcCardinalityDef {
    pub key: String,
    pub display_name: String,
    pub description: String,
}

/// Terminal palette for TUI graceful degradation.
#[derive(Debug, Clone, Deserialize)]
pub struct TerminalPalette {
    pub palette_256: HashMap<String, u8>,
    pub palette_16: HashMap<String, u8>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load and validate taxonomy.yaml.
pub fn load_taxonomy(root: &Path) -> crate::Result<TaxonomyDoc> {
    let path = crate::config::taxonomy_path(root);

    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "taxonomy.yaml not found: {}",
            path.display()
        )));
    }

    let doc: TaxonomyDoc = super::utils::load_yaml(&path)?;

    // Fail-fast validation
    if doc.node_realms.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "taxonomy.yaml has no node_realms".to_string(),
        ));
    }
    for realm in &doc.node_realms {
        if realm.layers.is_empty() {
            return Err(crate::NovaNetError::Validation(format!(
                "realm '{}' has no layers",
                realm.key
            )));
        }
    }
    if doc.node_traits.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "taxonomy.yaml has no node_traits".to_string(),
        ));
    }
    if doc.arc_families.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "taxonomy.yaml has no arc_families".to_string(),
        ));
    }

    Ok(doc)
}

// ─────────────────────────────────────────────────────────────────────────────
// OrganizingDoc Conversion — generator-friendly format
// ─────────────────────────────────────────────────────────────────────────────

/// Convert TaxonomyDoc to OrganizingDoc format for generators.
impl TaxonomyDoc {
    pub fn to_organizing_doc(&self) -> crate::parsers::organizing::OrganizingDoc {
        crate::parsers::organizing::OrganizingDoc {
            version: self.version.clone(),
            realms: self
                .node_realms
                .iter()
                .map(|r| crate::parsers::organizing::RealmDef {
                    key: r.key.clone(),
                    display_name: r.display_name.clone(),
                    emoji: r.emoji.clone(),
                    color: r.color.clone(),
                    llm_context: r.llm_context.clone(),
                    layers: r
                        .layers
                        .iter()
                        .map(|l| crate::parsers::organizing::LayerDef {
                            key: l.key.clone(),
                            display_name: l.display_name.clone(),
                            emoji: l.emoji.clone(),
                            color: l.color.clone(),
                            llm_context: l.llm_context.clone(),
                        })
                        .collect(),
                })
                .collect(),
            traits: self
                .node_traits
                .iter()
                .map(|t| crate::parsers::organizing::TraitDef {
                    key: t.key.clone(),
                    display_name: t.display_name.clone(),
                    color: t.color.clone(),
                    llm_context: t.llm_context.clone(),
                })
                .collect(),
            arc_families: self
                .arc_families
                .iter()
                .map(|f| crate::parsers::organizing::ArcFamilyDef {
                    key: f.key.clone(),
                    display_name: f.display_name.clone(),
                    color: f.color.clone(),
                    arrow_style: f.arrow_style.clone(),
                    llm_context: f.llm_context.clone(),
                })
                .collect(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_taxonomy_yaml() {
        let yaml = r##"
version: "9.5.0"
node_realms:
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"
    llm_context: "Global context."
    layers:
      - key: config
        display_name: Configuration
        emoji: "⚙️"
        color: "#64748b"
        llm_context: "Config layer."
node_traits:
  - key: invariant
    display_name: Invariant
    color: "#3b82f6"
    border_style: solid
    border_width: 2
    unicode_border: "─"
    llm_context: "Invariant nodes."
arc_families:
  - key: ownership
    display_name: Ownership
    color: "#3b82f6"
    stroke_style: solid
    stroke_width: 2
    arrow_style: "-->"
    llm_context: "Ownership arcs."
arc_scopes:
  - key: intra_realm
    display_name: Intra-Realm
    description: "Same realm"
arc_cardinalities:
  - key: one_to_many
    display_name: "1:N"
    description: "One to many"
terminal:
  palette_256:
    cyan: 37
  palette_16:
    cyan: 6
"##;
        let doc: TaxonomyDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.version, "9.5.0");
        assert_eq!(doc.node_realms.len(), 1);
        assert_eq!(doc.node_realms[0].key, "global");
        assert_eq!(doc.node_realms[0].layers.len(), 1);
        assert_eq!(doc.node_realms[0].layers[0].key, "config");
        assert_eq!(doc.node_traits.len(), 1);
        assert_eq!(doc.node_traits[0].key, "invariant");
        assert_eq!(doc.node_traits[0].border_style, Some("solid".to_string()));
        assert_eq!(doc.node_traits[0].unicode_border, Some("─".to_string()));
        assert_eq!(doc.arc_families.len(), 1);
        assert_eq!(doc.arc_families[0].arrow_style, "-->");
        assert_eq!(doc.arc_families[0].stroke_style, Some("solid".to_string()));
        assert_eq!(doc.arc_scopes.len(), 1);
        assert_eq!(doc.arc_cardinalities.len(), 1);
        let terminal = doc.terminal.unwrap();
        assert_eq!(terminal.palette_256.get("cyan"), Some(&37));
        assert_eq!(terminal.palette_16.get("cyan"), Some(&6));
    }

    #[test]
    fn parse_minimal_taxonomy() {
        let yaml = r##"
version: "9.5.0"
node_realms:
  - key: test
    display_name: Test
    emoji: "🧪"
    color: "#000"
    llm_context: "Test."
    layers:
      - key: base
        display_name: Base
        emoji: "📋"
        color: "#111"
        llm_context: "Base."
node_traits:
  - key: fixed
    display_name: Fixed
    color: "#222"
    llm_context: "Fixed."
arc_families:
  - key: owns
    display_name: Owns
    color: "#333"
    arrow_style: "-->"
    llm_context: "Ownership."
"##;
        let doc: TaxonomyDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.version, "9.5.0");
        assert!(doc.arc_scopes.is_empty());
        assert!(doc.arc_cardinalities.is_empty());
        assert!(doc.terminal.is_none());
        // Border style is optional
        assert!(doc.node_traits[0].border_style.is_none());
    }

    #[test]
    fn to_organizing_doc_conversion() {
        let yaml = r##"
version: "9.5.0"
node_realms:
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"
    llm_context: "Global."
    layers:
      - key: config
        display_name: Configuration
        emoji: "⚙️"
        color: "#64748b"
        llm_context: "Config."
node_traits:
  - key: invariant
    display_name: Invariant
    color: "#3b82f6"
    llm_context: "Invariant."
arc_families:
  - key: ownership
    display_name: Ownership
    color: "#3b82f6"
    arrow_style: "-->"
    llm_context: "Ownership."
"##;
        let doc: TaxonomyDoc = serde_yaml::from_str(yaml).unwrap();
        let organizing = doc.to_organizing_doc();

        assert_eq!(organizing.version, "9.5.0");
        assert_eq!(organizing.realms.len(), 1);
        assert_eq!(organizing.realms[0].key, "global");
        assert_eq!(organizing.traits.len(), 1);
        assert_eq!(organizing.traits[0].key, "invariant");
        assert_eq!(organizing.arc_families.len(), 1);
        assert_eq!(organizing.arc_families[0].key, "ownership");
    }

    #[test]
    fn load_taxonomy_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_taxonomy(root).expect("should load taxonomy.yaml");

        assert_eq!(doc.version, "10.6.0");
        assert_eq!(doc.node_realms.len(), 2); // v10.6: 2 realms (global, tenant)
        assert_eq!(doc.node_traits.len(), 5);
        assert_eq!(doc.arc_families.len(), 5);
        assert_eq!(doc.arc_scopes.len(), 2);
        assert_eq!(doc.arc_cardinalities.len(), 4);

        let total_layers: usize = doc.node_realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 9); // v10.6: 3 global + 6 tenant layers

        // Check border styles
        let invariant = doc
            .node_traits
            .iter()
            .find(|t| t.key == "invariant")
            .unwrap();
        assert_eq!(invariant.border_style, Some("solid".to_string()));
        assert_eq!(invariant.unicode_border, Some("─".to_string()));

        // Check terminal palette (uses semantic keys like global, tenant, etc.)
        let terminal = doc.terminal.as_ref().expect("should have terminal palette");
        assert!(terminal.palette_256.contains_key("global"));
        assert!(terminal.palette_256.contains_key("tenant"));
        assert!(terminal.palette_16.contains_key("global"));
        assert!(terminal.palette_16.contains_key("tenant"));

        // v9.9: Check kind_retrieval_defaults
        let defaults = doc
            .kind_retrieval_defaults
            .as_ref()
            .expect("should have kind_retrieval_defaults");
        assert!(defaults.contains_key("invariant"));
        assert!(defaults.contains_key("localized"));
        let invariant_defaults = defaults.get("invariant").unwrap();
        assert_eq!(invariant_defaults.traversal_depth, Some(2));
        assert_eq!(invariant_defaults.context_budget, Some(500));
        assert_eq!(invariant_defaults.token_estimate, Some(100));

        // v9.9: Check default_traversal on arc families
        let ownership = doc
            .arc_families
            .iter()
            .find(|f| f.key == "ownership")
            .unwrap();
        assert_eq!(ownership.default_traversal, Some("eager".to_string()));
        let semantic = doc
            .arc_families
            .iter()
            .find(|f| f.key == "semantic")
            .unwrap();
        assert_eq!(semantic.default_traversal, Some("lazy".to_string()));
    }

    #[test]
    fn parse_kind_retrieval_defaults() {
        let yaml = r##"
version: "10.5.0"
node_realms:
  - key: test
    display_name: Test
    emoji: "🧪"
    color: "#000"
    llm_context: "Test."
    layers:
      - key: base
        display_name: Base
        emoji: "📋"
        color: "#111"
        llm_context: "Base."
node_traits:
  - key: invariant
    display_name: Invariant
    color: "#222"
    llm_context: "Invariant."
kind_retrieval_defaults:
  invariant:
    traversal_depth: 2
    context_budget: 500
    token_estimate: 100
  localized:
    traversal_depth: 2
    context_budget: 800
    token_estimate: 150
arc_families:
  - key: ownership
    display_name: Ownership
    color: "#333"
    arrow_style: "-->"
    default_traversal: eager
    llm_context: "Ownership."
  - key: semantic
    display_name: Semantic
    color: "#444"
    arrow_style: ".->"
    default_traversal: lazy
    llm_context: "Semantic."
"##;
        let doc: TaxonomyDoc = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.version, "10.5.0");

        // Check kind_retrieval_defaults
        let defaults = doc.kind_retrieval_defaults.unwrap();
        assert_eq!(defaults.len(), 2);
        let inv = defaults.get("invariant").unwrap();
        assert_eq!(inv.traversal_depth, Some(2));
        assert_eq!(inv.context_budget, Some(500));
        assert_eq!(inv.token_estimate, Some(100));

        // Check default_traversal
        assert_eq!(
            doc.arc_families[0].default_traversal,
            Some("eager".to_string())
        );
        assert_eq!(
            doc.arc_families[1].default_traversal,
            Some("lazy".to_string())
        );
    }
}
