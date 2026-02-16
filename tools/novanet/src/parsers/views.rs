//! Parse view YAML definitions from `packages/core/models/views/`.
//!
//! Each view defines a traversal pattern from a root node type,
//! used for documentation (Mermaid diagrams) and runtime context loading.

use serde::{Deserialize, Serialize};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────────────────────────────────────

/// Direction for traversing a relation in an include rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

/// Icon with dual format (web + terminal).
///
/// ```yaml
/// icon:
///   web: diamond
///   terminal: "◆"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewIcon {
    pub web: String,
    pub terminal: String,
}

impl Default for ViewIcon {
    fn default() -> Self {
        Self {
            web: "circle".to_string(),
            terminal: "●".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Root node type specification.
#[derive(Debug, Clone, Deserialize)]
pub struct RootDef {
    #[serde(rename = "type")]
    pub node_type: String,
}

/// A single include rule (recursive: can contain nested rules).
#[derive(Debug, Clone, Deserialize)]
pub struct IncludeRule {
    /// Relation type to follow (e.g., "HAS_PAGE", "HAS_CONTENT").
    pub relation: String,

    /// Traversal direction from current frontier.
    pub direction: Direction,

    /// Max hops for this relation (runtime only; ignored for schema diagrams).
    #[serde(default)]
    pub depth: Option<u32>,

    /// Restrict to specific target types (optional filter).
    #[serde(default)]
    pub target_types: Option<Vec<String>>,

    /// Nested include rules applied to newly discovered types.
    #[serde(default)]
    pub include: Option<Vec<IncludeRule>>,
}

/// A visual layer for docs rendering.
#[derive(Debug, Clone, Deserialize)]
pub struct ViewLayer {
    pub name: String,
    pub nodes: Vec<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Example Cypher query in docs section.
#[derive(Debug, Clone, Deserialize)]
pub struct ViewExample {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub query: String,
    #[serde(default)]
    pub params: Option<serde_yaml::Value>,
}

/// Documentation section of a view definition.
#[derive(Debug, Clone, Deserialize)]
pub struct ViewDocs {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub layers: Option<Vec<ViewLayer>>,
    #[serde(default)]
    pub examples: Option<Vec<ViewExample>>,
    #[serde(default)]
    pub notes: Option<Vec<String>>,
}

/// A complete view definition parsed from `<view-id>.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ViewDef {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub version: Option<String>,
    pub root: RootDef,
    pub include: Vec<IncludeRule>,
    #[serde(default)]
    pub filters: Option<serde_yaml::Value>,
    #[serde(default)]
    pub docs: Option<ViewDocs>,
}

/// Entry in `_registry.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ViewRegistryEntry {
    pub id: String,
    /// File path (optional in new format where views are inline).
    #[serde(default)]
    pub file: Option<String>,
    pub description: String,
    pub category: String,
    /// Icon with web (Lucide) and terminal (Unicode) variants.
    #[serde(default)]
    pub icon: Option<ViewIcon>,
    /// Color for the view (hex string).
    #[serde(default)]
    pub color: Option<String>,
    /// Navigation modes that show this view (data/meta/overlay/query).
    #[serde(default)]
    pub modes: Option<Vec<String>>,
    /// Cypher query template (parameterized).
    #[serde(default)]
    pub cypher: Option<String>,
    /// Parameter names for the Cypher template.
    #[serde(default)]
    pub params: Option<Vec<String>>,
    /// Whether this view is contextual (appears in node sidebar).
    #[serde(default)]
    pub contextual: Option<bool>,
    /// Node types this view applies to.
    #[serde(default)]
    pub applicable_types: Option<Vec<String>>,
}

/// The `_registry.yaml` document.
#[derive(Debug, Clone, Deserialize)]
pub struct ViewRegistry {
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    pub views: Vec<ViewRegistryEntry>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Simplified views.yaml (v0.12.5 - Single Source of Truth)
// ─────────────────────────────────────────────────────────────────────────────

/// Category definition in views.yaml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewCategoryDef {
    pub label: String,
    #[serde(default)]
    pub icon: Option<ViewIcon>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Single view entry in views.yaml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleViewEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(default)]
    pub icon: Option<ViewIcon>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub root_type: Option<String>,
    #[serde(default)]
    pub contextual: Option<bool>,
    #[serde(default)]
    pub applicable_types: Option<Vec<String>>,
    #[serde(default)]
    pub cypher: Option<String>,
}

/// The simplified `views.yaml` document (v0.12.5).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleViewsFile {
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub categories: std::collections::HashMap<String, ViewCategoryDef>,
    pub views: Vec<SimpleViewEntry>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Loaders
// ─────────────────────────────────────────────────────────────────────────────

// NOTE: The old view loader functions (load_all_views, load_view, load_registry)
// were removed in v0.12.5. The views/ directory was replaced by a single views.yaml file.
// Use load_simple_views() to load the new unified views format.

/// Load the simplified views file (`views.yaml`).
///
/// This is the single source of truth for TUI and Studio (v0.12.5).
pub fn load_simple_views(root: &Path) -> crate::Result<SimpleViewsFile> {
    let path = crate::config::models_dir(root).join("views.yaml");
    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "views.yaml not found: {}",
            path.display()
        )));
    }
    super::utils::load_yaml(&path)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_view() {
        let yaml = r#"
id: test-view
name: Test View
description: A test view
version: "1.0"
root:
  type: Block
include:
  - relation: HAS_INSTRUCTION
    direction: outgoing
"#;
        let view: ViewDef = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(view.id, "test-view");
        assert_eq!(view.root.node_type, "Block");
        assert_eq!(view.include.len(), 1);
        assert_eq!(view.include[0].relation, "HAS_INSTRUCTION");
        assert_eq!(view.include[0].direction, Direction::Outgoing);
        assert!(view.docs.is_none());
    }

    #[test]
    fn parse_nested_includes() {
        let yaml = r#"
id: nested
name: Nested
description: Test nested includes
root:
  type: Project
include:
  - relation: HAS_PAGE
    direction: outgoing
    include:
      - relation: HAS_BLOCK
        direction: outgoing
      - relation: HAS_INSTRUCTION
        direction: outgoing
"#;
        let view: ViewDef = serde_yaml::from_str(yaml).unwrap();
        let nested = view.include[0].include.as_ref().unwrap();
        assert_eq!(nested.len(), 2);
        assert_eq!(nested[0].relation, "HAS_BLOCK");
        assert_eq!(nested[1].relation, "HAS_INSTRUCTION");
    }

    #[test]
    fn parse_direction_variants() {
        for (yaml, expected) in [
            ("outgoing", Direction::Outgoing),
            ("incoming", Direction::Incoming),
            ("both", Direction::Both),
        ] {
            let result: Direction = serde_yaml::from_str(yaml).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn parse_view_with_docs() {
        let yaml = r#"
id: doc-view
name: Doc View
description: Has docs
root:
  type: Locale
include:
  - relation: HAS_IDENTITY
    direction: outgoing
docs:
  title: "My View"
  category: knowledge
  description: "A description"
  layers:
    - name: Config
      nodes: [Locale]
      color: blue
    - name: Knowledge
      nodes: [Formatting, Style]
  notes:
    - "Note 1"
    - "Note 2"
"#;
        let view: ViewDef = serde_yaml::from_str(yaml).unwrap();
        let docs = view.docs.as_ref().unwrap();
        assert_eq!(docs.title.as_deref(), Some("My View"));
        assert_eq!(docs.category.as_deref(), Some("knowledge"));
        let layers = docs.layers.as_ref().unwrap();
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].nodes, vec!["Locale"]);
        assert_eq!(layers[1].nodes, vec!["Formatting", "Style"]);
        let notes = docs.notes.as_ref().unwrap();
        assert_eq!(notes.len(), 2);
    }

    #[test]
    fn parse_registry() {
        let yaml = r#"
version: "8.1.0"
description: NovaNet Core View Definitions
views:
  - id: complete-graph
    file: complete-graph.yaml
    description: Full graph
    category: scope
  - id: block-generation
    file: block-generation.yaml
    description: Block context
    category: generation
"#;
        let reg: ViewRegistry = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(reg.version, "8.1.0");
        assert_eq!(reg.views.len(), 2);
        assert_eq!(reg.views[0].id, "complete-graph");
        assert_eq!(reg.views[0].category, "scope");
    }

    #[test]
    fn parse_include_with_depth() {
        let yaml = r#"
id: depth-view
name: Depth
description: Test depth
root:
  type: Block
include:
  - relation: USES_ENTITY
    direction: outgoing
    depth: 2
"#;
        let view: ViewDef = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(view.include[0].depth, Some(2));
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

    // NOTE: Old integration tests for load_all_views, load_view, load_registry
    // were removed in v0.12.5. The views/ directory was replaced by views.yaml.
    // See load_simple_views_integration for the new unified views format test.

    #[test]
    fn parse_view_icon_object_format() {
        let yaml = r#"
web: diamond
terminal: "◆"
"#;
        let icon: ViewIcon = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(icon.web, "diamond");
        assert_eq!(icon.terminal, "◆");
    }

    #[test]
    fn parse_registry_entry_with_icon() {
        let yaml = r##"
id: schema-complete
description: Complete schema-graph
icon:
  web: diamond
  terminal: "◆"
color: "#8b5cf6"
category: schema
modes: [schema]
cypher: |
  MATCH (n:Schema)
  RETURN n
"##;
        let entry: ViewRegistryEntry = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(entry.id, "schema-complete");
        let icon = entry.icon.unwrap();
        assert_eq!(icon.web, "diamond");
        assert_eq!(icon.terminal, "◆");
        assert_eq!(entry.color, Some("#8b5cf6".to_string()));
        assert!(entry.cypher.is_some());
    }

    #[test]
    fn parse_registry_with_new_format() {
        let yaml = r##"
version: "0.12.0"
description: NovaNet Unified View System
views:
  - id: schema-complete
    description: Complete schema-graph
    icon:
      web: diamond
      terminal: "◆"
    color: "#8b5cf6"
    category: schema
    modes: [schema]
    cypher: |
      MATCH (n:Schema)
      RETURN n
  - id: data-complete
    description: All instance nodes
    icon:
      web: globe
      terminal: "●"
    color: "#6366f1"
    category: data
    modes: [data]
    cypher: |
      MATCH (n) WHERE NOT n:Schema
      RETURN n
"##;
        let reg: ViewRegistry = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(reg.version, "0.12.0");
        assert_eq!(reg.views.len(), 2);

        // First view
        let v1 = &reg.views[0];
        assert_eq!(v1.id, "schema-complete");
        let icon1 = v1.icon.as_ref().unwrap();
        assert_eq!(icon1.web, "diamond");
        assert_eq!(icon1.terminal, "◆");
        assert_eq!(v1.color.as_deref(), Some("#8b5cf6"));

        // Second view
        let v2 = &reg.views[1];
        assert_eq!(v2.id, "data-complete");
        let icon2 = v2.icon.as_ref().unwrap();
        assert_eq!(icon2.web, "globe");
        assert_eq!(icon2.terminal, "●");
    }

    #[test]
    fn parse_simple_views_file() {
        let yaml = r##"
version: "0.12.5"
description: NovaNet Essential Views (10 views)
categories:
  schema:
    label: Schema
    icon: { web: database, terminal: "◆" }
    color: "#8b5cf6"
    description: Schema exploration
  generation:
    label: Generation
    icon: { web: sparkles, terminal: "⚡" }
    color: "#ec4899"
views:
  - id: schema-complete
    name: Complete Schema
    description: All 61 Classes and 156 ArcClasses
    category: schema
    icon: { web: diamond, terminal: "◆" }
    color: "#8b5cf6"
    root_type: null
    contextual: false
    cypher: |
      MATCH (n:Schema) RETURN n
  - id: gen-page
    name: Page Context
    description: Full context for orchestrator
    category: generation
    icon: { web: layout, terminal: "P" }
    color: "#06b6d4"
    root_type: Page
    contextual: true
    applicable_types: [Page]
    cypher: |
      MATCH (p:Page {key: $nodeKey}) RETURN p
"##;
        let file: SimpleViewsFile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(file.version, "0.12.5");
        assert_eq!(file.views.len(), 2);
        assert_eq!(file.categories.len(), 2);

        // Check categories
        let schema_cat = file.categories.get("schema").unwrap();
        assert_eq!(schema_cat.label, "Schema");
        assert_eq!(schema_cat.color.as_deref(), Some("#8b5cf6"));

        // Check first view
        let v1 = &file.views[0];
        assert_eq!(v1.id, "schema-complete");
        assert_eq!(v1.name, "Complete Schema");
        assert_eq!(v1.category, "schema");
        assert!(!v1.contextual.unwrap_or(false));

        // Check second view
        let v2 = &file.views[1];
        assert_eq!(v2.id, "gen-page");
        assert!(v2.contextual.unwrap_or(false));
        assert_eq!(v2.applicable_types.as_ref().unwrap(), &["Page"]);
    }

    #[test]
    fn load_simple_views_integration() {
        let Some(root) = test_root() else { return };
        let file = load_simple_views(&root).expect("should load views.yaml");
        assert_eq!(file.views.len(), 11, "expected 11 views");
        assert_eq!(file.categories.len(), 4, "expected 4 categories");

        // Check categories exist
        assert!(file.categories.contains_key("schema"));
        assert!(file.categories.contains_key("data"));
        assert!(file.categories.contains_key("generation"));
        assert!(file.categories.contains_key("contextual"));

        // Check all views have required fields
        for view in &file.views {
            assert!(!view.id.is_empty(), "view has empty id");
            assert!(!view.name.is_empty(), "view {} has empty name", view.id);
            assert!(
                file.categories.contains_key(&view.category),
                "view {} has invalid category {}",
                view.id,
                view.category
            );
        }
    }
}
