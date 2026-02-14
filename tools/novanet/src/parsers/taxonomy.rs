//! Parse taxonomy.yaml (v9.5 replacement for organizing-principles.yaml).
//!
//! Handles:
//! - `node_realms` with nested layers
//! - `node_traits` with border styles for visual encoding
//! - `arc_families` with stroke styles, arrow styles, and default_traversal
//! - `arc_scopes` and `arc_cardinalities` for arc classification
//! - `terminal` palette for TUI graceful degradation
//! - `class_retrieval_defaults` per-trait context assembly settings (v0.12.0, was kind_retrieval_defaults)
//!
//! ## Migration to Individual Files (v0.12.5)
//!
//! Use `load_taxonomy_from_files()` to load taxonomy data from individual YAML files:
//! - `realms/*.yaml` → Realm definitions
//! - `layers/*.yaml` → Layer definitions (with `realms` field)
//! - `traits/*.yaml` → Trait definitions
//! - `arc-families/*.yaml` → Arc family definitions
//!
//! The function constructs a `TaxonomyDoc` compatible with existing generators.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Dual-format Icon (v0.12.5)
// ─────────────────────────────────────────────────────────────────────────────

/// Dual-format icon for taxonomy definitions.
/// v0.12.5: Legacy emoji format removed. All icons use { web, terminal } format.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TaxonomyIcon {
    /// Web icon (Lucide name): "diamond"
    pub web: String,
    /// Terminal icon (Unicode symbol): "◆"
    pub terminal: String,
}

impl TaxonomyIcon {
    /// Get the terminal icon (Unicode symbol).
    pub fn terminal(&self) -> &str {
        &self.terminal
    }

    /// Get the web icon (Lucide name).
    pub fn web(&self) -> &str {
        &self.web
    }
}

impl Default for TaxonomyIcon {
    fn default() -> Self {
        Self {
            web: "circle".to_string(),
            terminal: "●".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs (taxonomy.yaml)
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level document for taxonomy.yaml (full format, legacy).
#[derive(Debug, Deserialize)]
pub struct TaxonomyDoc {
    pub version: String,
    pub node_realms: Vec<NodeRealmDef>,
    pub node_traits: Vec<NodeTraitDef>,
    /// v0.12.0: Per-trait retrieval defaults for context assembly.
    #[serde(default)]
    pub class_retrieval_defaults: Option<HashMap<String, ClassRetrievalDefaults>>,
    pub arc_families: Vec<ArcFamilyDef>,
    #[serde(default)]
    pub arc_scopes: Vec<ArcScopeDef>,
    #[serde(default)]
    pub arc_cardinalities: Vec<ArcCardinalityDef>,
    #[serde(default)]
    pub terminal: Option<TerminalPalette>,
}

/// Minimal taxonomy.yaml format (v0.12.5+).
///
/// Contains only centralized config that isn't per-item:
/// - arc_scopes, arc_cardinalities (small enums)
/// - terminal palette (TUI graceful degradation)
/// - class_retrieval_defaults (context assembly config)
///
/// Node realms, traits, and arc families are loaded from individual files.
#[derive(Debug, Deserialize)]
pub struct MinimalTaxonomyDoc {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub arc_scopes: Vec<ArcScopeDef>,
    #[serde(default)]
    pub arc_cardinalities: Vec<ArcCardinalityDef>,
    #[serde(default)]
    pub terminal: Option<TerminalPalette>,
    #[serde(default)]
    pub class_retrieval_defaults: Option<HashMap<String, ClassRetrievalDefaults>>,
}

fn default_version() -> String {
    "0.12.5".to_string()
}

/// Per-trait retrieval settings for context assembly.
#[derive(Debug, Clone, Deserialize)]
pub struct ClassRetrievalDefaults {
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
    /// v0.12.5: Dual format icon { web, terminal }.
    pub icon: TaxonomyIcon,
    pub color: String,
    pub llm_context: String,
    pub layers: Vec<NodeLayerDef>,
}

impl NodeRealmDef {
    /// Get terminal icon (Unicode symbol for TUI display).
    pub fn emoji(&self) -> &str {
        self.icon.terminal()
    }
}

/// Layer definition (nested under its realm).
#[derive(Debug, Clone, Deserialize)]
pub struct NodeLayerDef {
    pub key: String,
    pub display_name: String,
    /// v0.12.5: Dual format icon { web, terminal }.
    pub icon: TaxonomyIcon,
    pub color: String,
    pub llm_context: String,
}

impl NodeLayerDef {
    /// Get terminal icon (Unicode symbol for TUI display).
    pub fn emoji(&self) -> &str {
        self.icon.terminal()
    }
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

/// Load and validate taxonomy (backwards-compatible wrapper).
///
/// **v0.12.5**: This function now delegates to `load_taxonomy_from_files()`.
/// The legacy `taxonomy.yaml` format (with node_realms, node_traits, arc_families)
/// is no longer supported. Use `load_taxonomy_from_files()` directly for clarity.
pub fn load_taxonomy(root: &Path) -> crate::Result<TaxonomyDoc> {
    load_taxonomy_from_files(root)
}

// ─────────────────────────────────────────────────────────────────────────────
// Individual File Loader (v0.12.5)
// ─────────────────────────────────────────────────────────────────────────────

/// Load taxonomy from individual YAML files (v0.12.5 migration).
///
/// Reads from:
/// - `realms/*.yaml` → Realm definitions
/// - `layers/*.yaml` → Layer definitions
/// - `traits/*.yaml` → Trait definitions
/// - `arc-families/*.yaml` → Arc family definitions
/// - `taxonomy.yaml` → arc_scopes, arc_cardinalities, terminal (still centralized)
///
/// Returns a `TaxonomyDoc` compatible with existing generators.
pub fn load_taxonomy_from_files(root: &Path) -> crate::Result<TaxonomyDoc> {
    use super::{arc_family, layer, realm, trait_def};

    // Load individual files
    let realms = realm::load_all_realms(root)?;
    let layers = layer::load_all_layers(root)?;
    let traits = trait_def::load_all_traits(root)?;
    let arc_families = arc_family::load_all_arc_families(root)?;

    // Load arc_scopes, arc_cardinalities, terminal from taxonomy.yaml
    // (these are small and rarely change, kept centralized for now)
    // v0.12.5: taxonomy.yaml is now minimal format (MinimalTaxonomyDoc)
    let taxonomy_path = crate::config::taxonomy_path(root);
    let (arc_scopes, arc_cardinalities, terminal, class_retrieval_defaults, version) =
        if taxonomy_path.exists() {
            let minimal: MinimalTaxonomyDoc = super::utils::load_yaml(&taxonomy_path)?;
            (
                minimal.arc_scopes,
                minimal.arc_cardinalities,
                minimal.terminal,
                minimal.class_retrieval_defaults,
                minimal.version,
            )
        } else {
            (vec![], vec![], None, None, "0.12.5".to_string())
        };

    // Build realm→layers mapping from layer.realms field
    let mut realm_layers: HashMap<String, Vec<NodeLayerDef>> = HashMap::new();
    for realm in &realms {
        realm_layers.insert(realm.key.clone(), vec![]);
    }

    for layer in &layers {
        let llm_context = layer.llm_context.clone().unwrap_or_default();
        let layer_def = NodeLayerDef {
            key: layer.key.clone(),
            display_name: layer.display_name.clone(),
            icon: TaxonomyIcon {
                web: layer.icon.web.clone(),
                terminal: layer.icon.terminal.clone(),
            },
            color: layer.color.clone(),
            llm_context,
        };

        for realm_key in &layer.realms {
            if let Some(layers) = realm_layers.get_mut(realm_key) {
                layers.push(layer_def.clone());
            }
        }
    }

    // Convert realms to NodeRealmDef with nested layers
    let node_realms: Vec<NodeRealmDef> = realms
        .into_iter()
        .map(|r| {
            let layers = realm_layers.remove(&r.key).unwrap_or_default();
            NodeRealmDef {
                key: r.key,
                display_name: r.display_name,
                icon: TaxonomyIcon {
                    web: r.icon.web,
                    terminal: r.icon.terminal,
                },
                color: r.color,
                llm_context: r.llm_context.unwrap_or_default(),
                layers,
            }
        })
        .collect();

    // Convert traits to NodeTraitDef
    let node_traits: Vec<NodeTraitDef> = traits
        .into_iter()
        .map(|t| NodeTraitDef {
            key: t.key,
            display_name: t.display_name,
            color: t.color,
            border_style: Some(t.border_style),
            border_width: Some(t.border_width as u8),
            unicode_border: t.unicode_border,
            llm_context: t.llm_context.unwrap_or_default(),
        })
        .collect();

    // Convert arc families to ArcFamilyDef
    let arc_families: Vec<ArcFamilyDef> = arc_families
        .into_iter()
        .map(|f| {
            let default_traversal = match f.default_traversal {
                super::arc_family::TraversalMode::Eager => Some("eager".to_string()),
                super::arc_family::TraversalMode::Lazy => Some("lazy".to_string()),
                super::arc_family::TraversalMode::OnDemand => Some("ondemand".to_string()),
                super::arc_family::TraversalMode::Skip => Some("skip".to_string()),
            };
            ArcFamilyDef {
                key: f.key,
                display_name: f.display_name,
                color: f.color,
                stroke_style: Some(f.stroke_style),
                stroke_width: Some(f.stroke_width as u8),
                arrow_style: f.arrow_style,
                default_traversal,
                llm_context: f.llm_context.unwrap_or_default(),
            }
        })
        .collect();

    // Validate
    if node_realms.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "no realms found in realms/".to_string(),
        ));
    }
    for realm in &node_realms {
        if realm.layers.is_empty() {
            return Err(crate::NovaNetError::Validation(format!(
                "realm '{}' has no layers",
                realm.key
            )));
        }
    }
    if node_traits.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "no traits found in traits/".to_string(),
        ));
    }
    if arc_families.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "no arc families found in arc-families/".to_string(),
        ));
    }

    Ok(TaxonomyDoc {
        version,
        node_realms,
        node_traits,
        class_retrieval_defaults,
        arc_families,
        arc_scopes,
        arc_cardinalities,
        terminal,
    })
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
                    emoji: r.emoji().to_string(),
                    color: r.color.clone(),
                    llm_context: r.llm_context.clone(),
                    layers: r
                        .layers
                        .iter()
                        .map(|l| crate::parsers::organizing::LayerDef {
                            key: l.key.clone(),
                            display_name: l.display_name.clone(),
                            emoji: l.emoji().to_string(),
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
  - key: shared
    display_name: Shared
    icon:
      web: globe
      terminal: "🌍"
    color: "#2aa198"
    llm_context: "Shared context."
    layers:
      - key: config
        display_name: Configuration
        icon:
          web: settings
          terminal: "⚙️"
        color: "#64748b"
        llm_context: "Config layer."
node_traits:
  - key: defined
    display_name: Defined
    color: "#3b82f6"
    border_style: solid
    border_width: 2
    unicode_border: "─"
    llm_context: "Defined nodes (v11.8: was invariant)."
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
        assert_eq!(doc.node_realms[0].key, "shared");
        assert_eq!(doc.node_realms[0].layers.len(), 1);
        assert_eq!(doc.node_realms[0].layers[0].key, "config");
        assert_eq!(doc.node_traits.len(), 1);
        assert_eq!(doc.node_traits[0].key, "defined"); // v11.8: was invariant
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
    icon:
      web: flask
      terminal: "🧪"
    color: "#000"
    llm_context: "Test."
    layers:
      - key: base
        display_name: Base
        icon:
          web: clipboard
          terminal: "📋"
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
  - key: shared
    display_name: Shared
    icon:
      web: globe
      terminal: "🌍"
    color: "#2aa198"
    llm_context: "Shared."
    layers:
      - key: config
        display_name: Configuration
        icon:
          web: settings
          terminal: "⚙️"
        color: "#64748b"
        llm_context: "Config."
node_traits:
  - key: defined
    display_name: Defined
    color: "#3b82f6"
    llm_context: "Defined."
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
        assert_eq!(organizing.realms[0].key, "shared");
        assert_eq!(organizing.traits.len(), 1);
        assert_eq!(organizing.traits[0].key, "defined"); // v11.8: was invariant
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

        // v0.12.5: load_taxonomy() now delegates to load_taxonomy_from_files()
        let doc = load_taxonomy(root).expect("should load taxonomy from individual files");

        // v0.12.5: Version comes from minimal taxonomy.yaml
        assert_eq!(doc.version, "0.12.5");
        assert_eq!(doc.node_realms.len(), 2); // v11.2: 2 realms (shared, org)
        assert_eq!(doc.node_traits.len(), 5); // v11.2: split derived → generated + aggregated
        assert_eq!(doc.arc_families.len(), 5);
        assert_eq!(doc.arc_scopes.len(), 2);
        assert_eq!(doc.arc_cardinalities.len(), 5); // zero_to_one, one_to_one, one_to_many, many_to_one, many_to_many

        let total_layers: usize = doc.node_realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 10); // v11.4: 4 shared + 6 org layers

        // Check border styles (v0.12.0: invariant → defined)
        let defined = doc.node_traits.iter().find(|t| t.key == "defined").unwrap();
        assert_eq!(defined.border_style, Some("solid".to_string()));
        assert_eq!(defined.unicode_border, Some("─".to_string()));

        // Check terminal palette (uses semantic keys like global, tenant, etc.)
        let terminal = doc.terminal.as_ref().expect("should have terminal palette");
        assert!(terminal.palette_256.contains_key("shared"));
        assert!(terminal.palette_256.contains_key("org"));
        assert!(terminal.palette_16.contains_key("shared"));
        assert!(terminal.palette_16.contains_key("org"));

        // v0.12.0: kind_retrieval_defaults → class_retrieval_defaults
        let defaults = doc
            .class_retrieval_defaults
            .as_ref()
            .expect("should have class_retrieval_defaults");
        assert!(defaults.contains_key("defined"));
        assert!(defaults.contains_key("authored"));
        let defined_defaults = defaults.get("defined").unwrap();
        assert_eq!(defined_defaults.traversal_depth, Some(2));
        assert_eq!(defined_defaults.context_budget, Some(500));
        assert_eq!(defined_defaults.token_estimate, Some(100));

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
    fn parse_class_retrieval_defaults() {
        // v0.12.0: Test class_retrieval_defaults field
        let yaml = r##"
version: "10.5.0"
node_realms:
  - key: test
    display_name: Test
    icon:
      web: flask
      terminal: "🧪"
    color: "#000"
    llm_context: "Test."
    layers:
      - key: base
        display_name: Base
        icon:
          web: clipboard
          terminal: "📋"
        color: "#111"
        llm_context: "Base."
node_traits:
  - key: defined
    display_name: Defined
    color: "#222"
    llm_context: "Defined."
class_retrieval_defaults:
  defined:
    traversal_depth: 2
    context_budget: 500
    token_estimate: 100
  authored:
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

        // Check class_retrieval_defaults (v0.12.0: renamed from kind_retrieval_defaults)
        let defaults = doc.class_retrieval_defaults.unwrap();
        assert_eq!(defaults.len(), 2);
        let def = defaults.get("defined").unwrap();
        assert_eq!(def.traversal_depth, Some(2));
        assert_eq!(def.context_budget, Some(500));
        assert_eq!(def.token_estimate, Some(100));

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

    #[test]
    fn load_taxonomy_from_files_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_taxonomy_from_files(root).expect("should load from individual files");

        // Same expectations as load_taxonomy_integration
        assert_eq!(doc.node_realms.len(), 2, "expected 2 realms (shared, org)");
        assert_eq!(doc.node_traits.len(), 5, "expected 5 traits");
        assert_eq!(doc.arc_families.len(), 5, "expected 5 arc families");

        let total_layers: usize = doc.node_realms.iter().map(|r| r.layers.len()).sum();
        assert_eq!(total_layers, 10, "expected 10 layers (4 shared + 6 org)");

        // Check realms have their layers
        let shared = doc.node_realms.iter().find(|r| r.key == "shared").unwrap();
        assert_eq!(shared.layers.len(), 4, "shared should have 4 layers");
        let layer_keys: Vec<&str> = shared.layers.iter().map(|l| l.key.as_str()).collect();
        assert!(layer_keys.contains(&"config"));
        assert!(layer_keys.contains(&"locale"));
        assert!(layer_keys.contains(&"geography"));
        assert!(layer_keys.contains(&"knowledge"));

        let org = doc.node_realms.iter().find(|r| r.key == "org").unwrap();
        assert_eq!(org.layers.len(), 6, "org should have 6 layers");

        // Check trait visual encoding
        let defined = doc.node_traits.iter().find(|t| t.key == "defined").unwrap();
        assert_eq!(defined.border_style, Some("solid".to_string()));

        // Check arc family properties
        let ownership = doc
            .arc_families
            .iter()
            .find(|f| f.key == "ownership")
            .unwrap();
        assert_eq!(ownership.default_traversal, Some("eager".to_string()));

        // arc_scopes and arc_cardinalities come from taxonomy.yaml
        assert_eq!(doc.arc_scopes.len(), 2);
        assert_eq!(doc.arc_cardinalities.len(), 5);
    }

    // v0.12.5: load_taxonomy_from_files_matches_load_taxonomy test REMOVED
    // since load_taxonomy() now delegates to load_taxonomy_from_files()
}
