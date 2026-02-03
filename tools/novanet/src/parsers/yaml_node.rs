//! Parse 35 YAML node definitions with locale_behavior validation.
//!
//! Fails fast if any YAML is missing `locale_behavior`, `realm`, or `layer` — no silent defaults.
//! Each file at `packages/core/models/nodes/<realm>/<layer>/<name>.yaml`
//! is deserialized into a [`ParsedNode`] with realm/layer read from the YAML content.
//! Validation ensures the file path matches the YAML-declared realm/layer.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// ─────────────────────────────────────────────────────────────────────────────
// LocaleBehavior (v9 Trait)
// ─────────────────────────────────────────────────────────────────────────────

/// The 5 locale behavior traits in v9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocaleBehavior {
    Invariant,
    Localized,
    Knowledge,
    Derived,
    Job,
}

impl std::fmt::Display for LocaleBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invariant => write!(f, "invariant"),
            Self::Localized => write!(f, "localized"),
            Self::Knowledge => write!(f, "knowledge"),
            Self::Derived => write!(f, "derived"),
            Self::Job => write!(f, "job"),
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
    /// Neo4j label (PascalCase), e.g. "Project", "PageL10n".
    pub name: String,

    /// Realm classification (global, project, shared) — explicit in YAML.
    pub realm: String,

    /// Layer classification (config, knowledge, foundation, etc.) — explicit in YAML.
    pub layer: String,

    /// v9 trait — required, fail-fast if missing.
    pub locale_behavior: LocaleBehavior,

    /// Emoji icon for Mermaid diagrams.
    #[serde(default)]
    pub icon: Option<String>,

    /// Human-readable description.
    pub description: String,

    /// Standard properties (key, display_name, llm_context, etc.).
    #[serde(default)]
    pub standard_properties: Option<BTreeMap<String, PropertyDef>>,

    /// Node-specific business properties.
    #[serde(default)]
    pub properties: Option<BTreeMap<String, PropertyDef>>,

    /// Relations declared in this file (format varies — canonical source is relations.yaml).
    #[serde(default)]
    pub relations: Option<serde_yml::Value>,

    /// Incoming relations (some files use this instead of nesting under `relations`).
    #[serde(default)]
    pub incoming_relations: Option<serde_yml::Value>,

    /// Neo4j configuration (indexes, constraints).
    #[serde(default)]
    pub neo4j: Option<serde_yml::Value>,

    /// Example data and Cypher queries.
    #[serde(default)]
    pub example: Option<serde_yml::Value>,
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
    pub extra: BTreeMap<String, serde_yml::Value>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ParsedNode (enriched with path metadata)
// ─────────────────────────────────────────────────────────────────────────────

/// A fully parsed node with realm/layer from YAML content (validated against file path).
#[derive(Debug)]
pub struct ParsedNode {
    /// The deserialized node definition.
    pub def: NodeDef,
    /// Realm from directory: "global", "project", "shared".
    pub realm: String,
    /// Layer from directory: "config", "knowledge", "foundation", etc.
    pub layer: String,
    /// Source file path (for error reporting).
    pub source_path: PathBuf,
}

impl ParsedNode {
    /// Returns all property names (standard + business), sorted.
    pub fn all_property_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = Vec::new();
        if let Some(ref sp) = self.def.standard_properties {
            names.extend(sp.keys().map(|k| k.as_str()));
        }
        if let Some(ref p) = self.def.properties {
            names.extend(p.keys().map(|k| k.as_str()));
        }
        names.sort();
        names
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load and parse all YAML node definitions under `<root>/packages/core/models/nodes/`.
///
/// Walks the directory recursively, deserializes each `.yaml` file, and
/// reads `realm`/`layer` from the YAML content (source of truth).
/// Validates that the file path matches the YAML-declared realm/layer.
///
/// # Errors
///
/// - `NovaNetError::Validation` if the nodes directory doesn't exist or is empty
/// - `NovaNetError::Schema` if any YAML file fails to parse (including missing `locale_behavior`)
/// - `NovaNetError::Io` on filesystem errors
pub fn load_all_nodes(root: &Path) -> crate::Result<Vec<ParsedNode>> {
    let nodes_dir = crate::config::nodes_dir(root);

    if !nodes_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "nodes directory not found: {}",
            nodes_dir.display()
        )));
    }

    let mut nodes = Vec::new();

    for entry in WalkDir::new(&nodes_dir)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "yaml")
        })
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path)?;

        let doc: NodeDocument =
            serde_yml::from_str(&content).map_err(|e| crate::NovaNetError::Schema {
                path: path.display().to_string(),
                source: e,
            })?;

        // Realm and layer are now explicit in YAML (source of truth)
        let realm = doc.node.realm.clone();
        let layer = doc.node.layer.clone();

        // Validate path matches YAML content
        let rel = path.strip_prefix(&nodes_dir).unwrap_or(path);
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

        nodes.push(ParsedNode {
            def: doc.node,
            realm,
            layer,
            source_path: path.to_path_buf(),
        });
    }

    if nodes.is_empty() {
        return Err(crate::NovaNetError::Validation(format!(
            "no YAML node files found under {}",
            nodes_dir.display()
        )));
    }

    // Sort by name for deterministic output across all generators
    nodes.sort_by(|a, b| a.def.name.cmp(&b.def.name));

    Ok(nodes)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_behavior_deserialize() {
        let yaml = "node:\n  name: Test\n  realm: project\n  layer: foundation\n  locale_behavior: invariant\n  description: test";
        let doc: NodeDocument = serde_yml::from_str(yaml).unwrap();
        assert_eq!(doc.node.locale_behavior, LocaleBehavior::Invariant);
        assert_eq!(doc.node.name, "Test");
        assert_eq!(doc.node.realm, "project");
        assert_eq!(doc.node.layer, "foundation");
    }

    #[test]
    fn locale_behavior_all_variants() {
        for variant in ["invariant", "localized", "knowledge", "derived", "job"] {
            let yaml = format!("node:\n  name: T\n  realm: global\n  layer: config\n  locale_behavior: {variant}\n  description: d");
            let doc: NodeDocument = serde_yml::from_str(&yaml).unwrap();
            assert_eq!(doc.node.locale_behavior.to_string(), variant);
        }
    }

    #[test]
    fn missing_locale_behavior_fails() {
        let yaml = "node:\n  name: Test\n  realm: project\n  layer: foundation\n  description: test";
        let result = serde_yml::from_str::<NodeDocument>(yaml);
        assert!(result.is_err(), "should fail without locale_behavior");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("locale_behavior"),
            "error should mention locale_behavior: {err_msg}"
        );
    }

    #[test]
    fn missing_realm_fails() {
        let yaml = "node:\n  name: Test\n  layer: foundation\n  locale_behavior: invariant\n  description: test";
        let result = serde_yml::from_str::<NodeDocument>(yaml);
        assert!(result.is_err(), "should fail without realm");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("realm"),
            "error should mention realm: {err_msg}"
        );
    }

    #[test]
    fn invalid_locale_behavior_fails() {
        let yaml = "node:\n  name: Test\n  realm: project\n  layer: foundation\n  locale_behavior: banana\n  description: test";
        let result = serde_yml::from_str::<NodeDocument>(yaml);
        assert!(result.is_err(), "should fail with invalid locale_behavior");
    }

    #[test]
    fn optional_fields_default_to_none() {
        let yaml = "node:\n  name: Minimal\n  realm: shared\n  layer: seo\n  locale_behavior: job\n  description: d";
        let doc: NodeDocument = serde_yml::from_str(yaml).unwrap();
        assert!(doc.node.icon.is_none());
        assert!(doc.node.standard_properties.is_none());
        assert!(doc.node.properties.is_none());
        assert!(doc.node.relations.is_none());
        assert!(doc.node.incoming_relations.is_none());
        assert!(doc.node.neo4j.is_none());
        assert!(doc.node.example.is_none());
    }

    #[test]
    fn properties_parse_typed() {
        let yaml = r#"
node:
  name: Test
  realm: project
  layer: semantic
  locale_behavior: invariant
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
        let doc: NodeDocument = serde_yml::from_str(yaml).unwrap();
        let props = doc.node.properties.unwrap();
        assert_eq!(props.len(), 2);
        assert_eq!(props["volume"].prop_type, "int");
        assert_eq!(props["volume"].required, Some(true));
        assert_eq!(props["status"].prop_type, "string");
        // enum is captured in extra
        assert!(props["status"].extra.contains_key("enum"));
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

        let nodes = load_all_nodes(root).expect("should parse all 35 nodes");
        assert_eq!(nodes.len(), 35, "expected 35 YAML node files");

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

        // Verify trait distribution matches _index.yaml counts
        let count = |t: LocaleBehavior| nodes.iter().filter(|n| n.def.locale_behavior == t).count();
        assert_eq!(count(LocaleBehavior::Invariant), 11, "invariant count");
        assert_eq!(count(LocaleBehavior::Localized), 6, "localized count");
        assert_eq!(count(LocaleBehavior::Knowledge), 14, "knowledge count");
        assert_eq!(count(LocaleBehavior::Derived), 2, "derived count");
        assert_eq!(count(LocaleBehavior::Job), 2, "job count");

        // Verify realm distribution
        let realm_count = |r: &str| nodes.iter().filter(|n| n.realm == r).count();
        assert_eq!(realm_count("global"), 15, "global realm count");
        assert_eq!(realm_count("project"), 14, "project realm count");
        assert_eq!(realm_count("shared"), 6, "shared realm count");

        // Spot-check a few known nodes
        let project = nodes.iter().find(|n| n.def.name == "Project").unwrap();
        assert_eq!(project.realm, "project");
        assert_eq!(project.layer, "foundation");
        assert_eq!(project.def.locale_behavior, LocaleBehavior::Invariant);

        let voice = nodes.iter().find(|n| n.def.name == "LocaleVoice").unwrap();
        assert_eq!(voice.realm, "global");
        assert_eq!(voice.layer, "knowledge");
        assert_eq!(voice.def.locale_behavior, LocaleBehavior::Knowledge);
    }
}
