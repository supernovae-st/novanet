//! Parse 60 YAML node definitions with trait validation (v0.12.0).
//!
//! Fails fast if any YAML is missing `trait`, `realm`, or `layer` — no silent defaults.
//! Each file at `packages/core/models/node-kinds/<realm>/<layer>/<name>.yaml`
//! is deserialized into a [`ParsedNode`] with realm/layer read from the YAML content.
//! Validation ensures the file path matches the YAML-declared realm/layer.

use indexmap::IndexMap;
use rayon::prelude::*;
use serde::Deserialize;
use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ─────────────────────────────────────────────────────────────────────────────
// NodeTrait — Data Origin classification (v0.12.0 ADR-024)
// ─────────────────────────────────────────────────────────────────────────────

/// The 5 node traits (v0.12.0 ADR-024): defined, authored, imported, generated, retrieved.
///
/// v0.12.0 renames (with serde aliases for backward compatibility):
///   - invariant → defined (human-created once)
///   - localized → authored (human-written per locale)
///   - knowledge → imported (external data brought in)
///   - generated → generated (our LLM produces this)
///   - aggregated → retrieved (fetched from external APIs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeTrait {
    #[serde(alias = "invariant")]
    Defined,
    #[serde(alias = "localized")]
    Authored,
    #[serde(alias = "knowledge")]
    Imported,
    Generated,
    #[serde(alias = "aggregated")]
    Retrieved,
}

impl std::fmt::Display for NodeTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Defined => write!(f, "defined"),
            Self::Authored => write!(f, "authored"),
            Self::Imported => write!(f, "imported"),
            Self::Generated => write!(f, "generated"),
            Self::Retrieved => write!(f, "retrieved"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// KnowledgeTier (v10 — locale knowledge classification)
// ─────────────────────────────────────────────────────────────────────────────

/// The 3 knowledge tiers in v10 (for knowledge trait nodes only).
///
/// Used to group locale knowledge in TUI and control contextual retrieval:
/// - Technical: formatting, slugification, adaptation
/// - Style: voice/identity merged, term glossaries
/// - Semantic: expressions, patterns, culture, taboos, audience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KnowledgeTier {
    Technical,
    Style,
    Semantic,
}

impl std::fmt::Display for KnowledgeTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Technical => write!(f, "technical"),
            Self::Style => write!(f, "style"),
            Self::Semantic => write!(f, "semantic"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level YAML document: every node file has `node:` as root key.
#[derive(Debug, Deserialize)]
pub struct NodeDocument {
    pub node: NodeDef,
}

/// Core node definition from YAML.
#[derive(Debug, Deserialize)]
pub struct NodeDef {
    /// Neo4j label (PascalCase), e.g. "Project", "PageGenerated".
    pub name: String,

    /// Realm classification (shared, org) — explicit in YAML.
    pub realm: String,

    /// Layer classification (config, knowledge, foundation, etc.) — explicit in YAML.
    pub layer: String,

    /// Node trait (v10.4) — required, fail-fast if missing.
    #[serde(rename = "trait")]
    pub node_trait: NodeTrait,

    /// v10 knowledge tier — optional, only for knowledge trait nodes.
    /// Groups locale knowledge: technical, style, semantic.
    #[serde(default)]
    pub knowledge_tier: Option<KnowledgeTier>,

    /// Emoji icon for Mermaid diagrams.
    #[serde(default)]
    pub icon: Option<String>,

    /// Human-readable description.
    pub description: String,

    /// Standard properties (key, display_name, llm_context, etc.).
    /// Uses IndexMap to preserve YAML definition order.
    #[serde(default)]
    pub standard_properties: Option<IndexMap<String, PropertyDef>>,

    /// Node-specific business properties.
    /// Uses IndexMap to preserve YAML definition order.
    #[serde(default)]
    pub properties: Option<IndexMap<String, PropertyDef>>,

    /// Neo4j configuration (indexes, constraints).
    #[serde(default)]
    pub neo4j: Option<serde_yaml::Value>,

    /// Example data and Cypher queries.
    #[serde(default)]
    pub example: Option<serde_yaml::Value>,
}

/// A single property definition.
///
/// Captures the typed fields generators need; additional YAML fields
/// (example, enum, pattern, examples, default) are collected in `extra`.
#[derive(Debug, Clone, Deserialize)]
pub struct PropertyDef {
    /// YAML type string: "string", "int", "float", "boolean", "datetime", "json", "string[]".
    #[serde(rename = "type")]
    pub prop_type: String,

    /// Whether this property is required.
    #[serde(default)]
    pub required: Option<bool>,

    /// Human-readable description.
    #[serde(default)]
    pub description: Option<String>,

    /// All other fields (example, enum, pattern, default, etc.).
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ParsedNode (enriched with path metadata)
// ─────────────────────────────────────────────────────────────────────────────

/// A fully parsed node with realm/layer from YAML content (validated against file path).
#[derive(Debug)]
pub struct ParsedNode {
    /// The deserialized node definition.
    pub def: NodeDef,
    /// Realm from directory: "shared", "org".
    pub realm: String,
    /// Layer from directory: "config", "knowledge", "foundation", etc.
    pub layer: String,
    /// Source file path (for error reporting).
    pub source_path: PathBuf,
}

impl ParsedNode {
    /// Returns all property names (standard + business) in YAML definition order.
    /// Uses SmallVec (stack-allocated for ≤16 properties) since nodes typically have 5-15 props.
    /// Order: standard_properties first, then properties (preserves YAML order via IndexMap).
    pub fn all_property_names(&self) -> SmallVec<[&str; 16]> {
        let mut names: SmallVec<[&str; 16]> = SmallVec::new();
        if let Some(ref sp) = self.def.standard_properties {
            names.extend(sp.keys().map(|k| k.as_str()));
        }
        if let Some(ref p) = self.def.properties {
            names.extend(p.keys().map(|k| k.as_str()));
        }
        // No sorting — preserve YAML definition order (IndexMap already maintains order)
        names
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load and parse all YAML node definitions under `<root>/packages/core/models/node-kinds/`.
///
/// Walks the directory recursively, deserializes each `.yaml` file, and
/// reads `realm`/`layer` from the YAML content (source of truth).
/// Validates that the file path matches the YAML-declared realm/layer.
///
/// # Errors
///
/// - `NovaNetError::Validation` if the nodes directory doesn't exist or is empty
/// - `NovaNetError::Schema` if any YAML file fails to parse (including missing `trait`)
/// - `NovaNetError::Io` on filesystem errors
pub fn load_all_nodes(root: &Path) -> crate::Result<Vec<ParsedNode>> {
    let nodes_dir = crate::config::node_kinds_dir(root);

    if !nodes_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "nodes directory not found: {}",
            nodes_dir.display()
        )));
    }

    // Collect paths first (WalkDir is not Send, so collect before parallel)
    let paths: Vec<PathBuf> = WalkDir::new(&nodes_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "yaml")
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if paths.is_empty() {
        return Err(crate::NovaNetError::Validation(format!(
            "no YAML node files found under {}",
            nodes_dir.display()
        )));
    }

    // Parse in parallel with rayon (~4x speedup for 43 nodes)
    let results: Vec<crate::Result<ParsedNode>> = paths
        .par_iter()
        .map(|path| parse_single_node(path, &nodes_dir))
        .collect();

    // Collect results, fail on first error
    let mut nodes: Vec<ParsedNode> = Vec::with_capacity(results.len());
    for result in results {
        nodes.push(result?);
    }

    // Sort by name for deterministic output across all generators
    nodes.sort_by(|a, b| a.def.name.cmp(&b.def.name));

    Ok(nodes)
}

/// Parse a single node YAML file (called in parallel).
fn parse_single_node(path: &Path, nodes_dir: &Path) -> crate::Result<ParsedNode> {
    let doc: NodeDocument = super::utils::load_yaml(path)?;

    // Realm and layer are now explicit in YAML (source of truth)
    let realm = doc.node.realm.clone();
    let layer = doc.node.layer.clone();

    // Validate path matches YAML content
    let rel = path.strip_prefix(nodes_dir).unwrap_or(path);
    let components: Vec<_> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    if components.len() >= 2 {
        let path_realm = &components[0];
        let path_layer = &components[1];
        if path_realm != &realm || path_layer != &layer {
            return Err(crate::NovaNetError::Validation(format!(
                "path/YAML mismatch: file at {}/{} but YAML declares realm={}, layer={}",
                path_realm, path_layer, realm, layer
            )));
        }
    }

    Ok(ParsedNode {
        def: doc.node,
        realm,
        layer,
        source_path: path.to_path_buf(),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_trait_deserialize() {
        // Test with v9.5 `trait` field
        let yaml = "node:\n  name: Test\n  realm: org\n  layer: foundation\n  trait: invariant\n  description: test";
        let doc: NodeDocument = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.node.node_trait, NodeTrait::Defined);
        assert_eq!(doc.node.name, "Test");
        assert_eq!(doc.node.realm, "org");
        assert_eq!(doc.node.layer, "foundation");
    }

    #[test]
    fn node_trait_all_variants() {
        // v0.12.0: 5 traits with new names (ADR-024: Data Origin)
        // New names: defined, authored, imported, generated, retrieved
        for variant in ["defined", "authored", "imported", "generated", "retrieved"] {
            let yaml = format!(
                "node:\n  name: T\n  realm: shared\n  layer: config\n  trait: {variant}\n  description: d"
            );
            let doc: NodeDocument = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(doc.node.node_trait.to_string(), variant);
        }
    }

    #[test]
    fn node_trait_backward_compat_aliases() {
        // v0.12.0: Old trait names work via serde aliases
        let aliases = [
            ("invariant", "defined"),
            ("localized", "authored"),
            ("knowledge", "imported"),
            ("aggregated", "retrieved"),
        ];
        for (old_name, new_name) in aliases {
            let yaml = format!(
                "node:\n  name: T\n  realm: shared\n  layer: config\n  trait: {old_name}\n  description: d"
            );
            let doc: NodeDocument = serde_yaml::from_str(&yaml).unwrap();
            // Old name parses, but to_string() returns new canonical name
            assert_eq!(doc.node.node_trait.to_string(), new_name);
        }
    }

    #[test]
    fn missing_trait_fails() {
        let yaml = "node:\n  name: Test\n  realm: org\n  layer: foundation\n  description: test";
        let result = serde_yaml::from_str::<NodeDocument>(yaml);
        assert!(result.is_err(), "should fail without trait");
        let err_msg = result.unwrap_err().to_string();
        // Error mentions `trait` since that's the serde field name
        assert!(
            err_msg.contains("trait"),
            "error should mention trait: {err_msg}"
        );
    }

    #[test]
    fn missing_realm_fails() {
        let yaml =
            "node:\n  name: Test\n  layer: foundation\n  trait: invariant\n  description: test";
        let result = serde_yaml::from_str::<NodeDocument>(yaml);
        assert!(result.is_err(), "should fail without realm");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("realm"),
            "error should mention realm: {err_msg}"
        );
    }

    #[test]
    fn invalid_trait_fails() {
        let yaml = "node:\n  name: Test\n  realm: org\n  layer: foundation\n  trait: banana\n  description: test";
        let result = serde_yaml::from_str::<NodeDocument>(yaml);
        assert!(result.is_err(), "should fail with invalid trait");
    }

    #[test]
    fn optional_fields_default_to_none() {
        // v11.4: use output layer (seo/geo moved to shared/knowledge)
        let yaml = "node:\n  name: Minimal\n  realm: org\n  layer: output\n  trait: generated\n  description: d";
        let doc: NodeDocument = serde_yaml::from_str(yaml).unwrap();
        assert!(doc.node.icon.is_none());
        assert!(doc.node.standard_properties.is_none());
        assert!(doc.node.properties.is_none());
        assert!(doc.node.neo4j.is_none());
        assert!(doc.node.example.is_none());
    }

    #[test]
    fn properties_parse_typed() {
        let yaml = r#"
node:
  name: Test
  realm: org
  layer: semantic
  trait: invariant
  description: d
  properties:
    volume:
      type: int
      required: true
      description: "Monthly searches"
      example: 12100
    status:
      type: string
      required: true
      description: "Status"
      enum: [draft, published, archived]
"#;
        let doc: NodeDocument = serde_yaml::from_str(yaml).unwrap();
        let props = doc.node.properties.unwrap();
        assert_eq!(props.len(), 2);
        assert_eq!(props["volume"].prop_type, "int");
        assert_eq!(props["volume"].required, Some(true));
        assert_eq!(props["status"].prop_type, "string");
        // enum is captured in extra
        assert!(props["status"].extra.contains_key("enum"));
    }

    #[test]
    fn knowledge_tier_deserialize() {
        let yaml = r#"
node:
  name: Style
  realm: shared
  layer: knowledge
  trait: knowledge
  knowledge_tier: style
  description: "Locale style settings"
"#;
        let doc: NodeDocument = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.node.knowledge_tier, Some(KnowledgeTier::Style));
    }

    #[test]
    fn knowledge_tier_all_variants() {
        for (variant, expected) in [
            ("technical", KnowledgeTier::Technical),
            ("style", KnowledgeTier::Style),
            ("semantic", KnowledgeTier::Semantic),
        ] {
            let yaml = format!(
                "node:\n  name: T\n  realm: shared\n  layer: knowledge\n  trait: knowledge\n  knowledge_tier: {variant}\n  description: d"
            );
            let doc: NodeDocument = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(doc.node.knowledge_tier, Some(expected));
        }
    }

    #[test]
    fn knowledge_tier_optional() {
        // Non-knowledge nodes don't have knowledge_tier
        let yaml = "node:\n  name: Project\n  realm: org\n  layer: foundation\n  trait: invariant\n  description: d";
        let doc: NodeDocument = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.node.knowledge_tier, None);
    }

    #[test]
    fn load_all_nodes_integration() {
        // Requires actual monorepo — finds root from Cargo.toml location
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent() // tools/
            .and_then(|p| p.parent()); // novanet-hq/

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        // v11.5: 60 nodes (39 shared + 21 org)
        // History:
        // - v10.8: Added geographic nodes (+12)
        // - v10.9: Added GEO nodes (+3)
        // - v11.1: Removed job nodes (-3)
        // - v11.2: Renamed global → shared, tenant → org
        // - v11.3: Merged Organization + Tenant → OrgConfig (-1)
        // - v11.4: Added containers (+3), removed obsolete SEO types (-4)
        // - v11.5: Moved Locale to shared/config, consolidated SEO/GEO to shared/knowledge
        let nodes = load_all_nodes(root).expect("should parse all 60 nodes");
        assert_eq!(
            nodes.len(),
            60,
            "expected 60 YAML node files (v11.5: 39 shared + 21 org)"
        );

        // Every node has a non-empty name, realm, and layer
        for node in &nodes {
            assert!(
                !node.def.name.is_empty(),
                "empty name: {:?}",
                node.source_path
            );
            assert!(
                !node.realm.is_empty(),
                "empty realm: {:?}",
                node.source_path
            );
            assert!(
                !node.layer.is_empty(),
                "empty layer: {:?}",
                node.source_path
            );
        }

        // Verify trait distribution (2 realms: shared + org)
        // v11.4: +3 invariant (SEOKeywordFormat, SEOKeywordSet, GEOQuerySet)
        //        -3 knowledge (SEOQuestion, SEOComparison, SEOPreposition)
        //        -1 aggregated (GEOMetrics)
        let count = |t: NodeTrait| nodes.iter().filter(|n| n.def.node_trait == t).count();
        assert_eq!(
            count(NodeTrait::Defined),
            32,
            "invariant count (v11.4: +3 SEO containers/format)"
        );
        assert_eq!(
            count(NodeTrait::Authored),
            2,
            "localized count (ProjectContent + EntityContent)"
        );
        assert_eq!(
            count(NodeTrait::Imported),
            20,
            "knowledge count (v11.4: -3 obsolete SEO types)"
        );
        assert_eq!(
            count(NodeTrait::Generated),
            4,
            "generated count (PageGenerated, BlockGenerated, OutputArtifact, PromptArtifact)"
        );
        assert_eq!(
            count(NodeTrait::Retrieved),
            2,
            "aggregated count (v11.4: GEOAnswer, SEOKeywordMetrics; GEOMetrics removed)"
        );

        // v11.4: Verify realm distribution (SEO/GEO moved to shared/knowledge)
        // shared: +3 (SEOKeywordFormat, SEOKeywordSet, GEOQuerySet) -4 obsolete = net -1
        let realm_count = |r: &str| nodes.iter().filter(|n| n.realm == r).count();
        assert_eq!(
            realm_count("shared"),
            39,
            "shared realm count (v11.4: +3 new containers, -4 obsolete nodes)"
        );
        assert_eq!(
            realm_count("org"),
            21,
            "org realm count (unchanged in v11.4)"
        );

        // Spot-check known nodes
        let project = nodes.iter().find(|n| n.def.name == "Project").unwrap();
        assert_eq!(project.realm, "org");
        assert_eq!(project.layer, "foundation");
        assert_eq!(project.def.node_trait, NodeTrait::Defined);
        assert_eq!(project.def.knowledge_tier, None); // invariant = no tier

        // Check Style node (in shared/locale — v11.3)
        let style = nodes.iter().find(|n| n.def.name == "Style").unwrap();
        assert_eq!(style.realm, "shared");
        assert_eq!(style.layer, "locale");
        assert_eq!(style.def.node_trait, NodeTrait::Imported);
        assert_eq!(style.def.knowledge_tier, None);

        // Check one of the knowledge atoms (in shared/knowledge — v11.3)
        let term = nodes.iter().find(|n| n.def.name == "Term").unwrap();
        assert_eq!(term.realm, "shared");
        assert_eq!(term.layer, "knowledge");
        assert_eq!(term.def.node_trait, NodeTrait::Imported);
    }
}
