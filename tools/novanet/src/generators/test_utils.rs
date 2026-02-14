//! Shared test utilities for generator tests.
//!
//! Provides factory functions for creating test fixtures:
//! - `make_node` — Create a `ParsedNode` with trait
//! - `make_node_simple` — Create a `ParsedNode` with Invariant trait
//! - `make_node_with_props` — Create a `ParsedNode` with properties
//! - `make_rel` — Create an `ArcDef` with string source/target
//! - `make_rel_full` — Create an `ArcDef` with NodeRef and cardinality

use std::collections::BTreeMap;
use std::path::PathBuf;

use indexmap::IndexMap;

use crate::parsers::arcs::{ArcDef, ArcFamily, Cardinality, NodeRef};
use crate::parsers::yaml_node::{NodeDef, NodeTrait, ParsedNode, PropertyDef};

// =============================================================================
// NODE FIXTURES
// =============================================================================

/// Create a `ParsedNode` for testing with explicit trait.
///
/// # Example
/// ```ignore
/// let node = make_node("Page", "org", "structure", NodeTrait::Defined);
/// assert_eq!(node.def.name, "Page");
/// ```
pub fn make_node(name: &str, realm: &str, layer: &str, behavior: NodeTrait) -> ParsedNode {
    ParsedNode {
        def: NodeDef {
            name: name.to_string(),
            realm: realm.to_string(),
            layer: layer.to_string(),
            node_trait: behavior,
            knowledge_tier: None,
            icon: None,
            description: format!("{name} description."),
            standard_properties: None,
            properties: None,
            neo4j: None,
            example: None,
        },
        realm: realm.to_string(),
        layer: layer.to_string(),
        source_path: PathBuf::from(format!(
            "packages/core/models/node-classes/{realm}/{layer}/{}.yaml",
            name.to_lowercase()
        )),
    }
}

/// Create a `ParsedNode` for testing with default Invariant trait.
///
/// Shorthand for `make_node(name, realm, layer, NodeTrait::Defined)`.
pub fn make_node_simple(name: &str, realm: &str, layer: &str) -> ParsedNode {
    make_node(name, realm, layer, NodeTrait::Defined)
}

/// Create a `ParsedNode` for testing with explicit properties.
///
/// # Arguments
/// * `props` — Vec of (name, type, required) tuples
///
/// # Example
/// ```ignore
/// let node = make_node_with_props(
///     "Page", "org", "structure", NodeTrait::Defined,
///     vec![("key", "string", true), ("title", "string", false)]
/// );
/// ```
pub fn make_node_with_props(
    name: &str,
    realm: &str,
    layer: &str,
    behavior: NodeTrait,
    props: Vec<(&str, &str, bool)>,
) -> ParsedNode {
    // Use IndexMap to preserve YAML definition order
    let mut properties = IndexMap::new();
    for (pname, ptype, req) in props {
        properties.insert(
            pname.to_string(),
            PropertyDef {
                prop_type: ptype.to_string(),
                required: Some(req),
                description: None,
                extra: BTreeMap::new(),
            },
        );
    }

    let mut node = make_node(name, realm, layer, behavior);
    node.def.properties = Some(properties);
    node
}

// =============================================================================
// ARC FIXTURES
// =============================================================================

/// Create an `ArcDef` for testing with string source/target.
///
/// Defaults to `Cardinality::OneToMany`.
///
/// # Example
/// ```ignore
/// let arc = make_rel("HAS_PAGE", ArcFamily::Ownership, "Project", "Page");
/// assert_eq!(arc.arc_type, "HAS_PAGE");
/// ```
pub fn make_rel(rel_type: &str, family: ArcFamily, source: &str, target: &str) -> ArcDef {
    ArcDef {
        arc_type: rel_type.to_string(),
        family,
        scope: None,
        source: NodeRef::Single(source.to_string()),
        target: NodeRef::Single(target.to_string()),
        cardinality: Cardinality::OneToMany,
        llm_context: format!("{rel_type} context."),
        properties: None,
        is_self_referential: None,
        inverse_of: None,
        inverse_name: None,
    }
}

/// Create an `ArcDef` for testing with full NodeRef and cardinality control.
///
/// # Example
/// ```ignore
/// let arc = make_rel_full(
///     "HAS_BLOCK",
///     ArcFamily::Ownership,
///     NodeRef::Multiple(vec!["Page".into(), "Block".into()]),
///     NodeRef::Single("Block".into()),
///     Cardinality::OneToMany,
/// );
/// ```
pub fn make_rel_full(
    rel_type: &str,
    family: ArcFamily,
    source: NodeRef,
    target: NodeRef,
    cardinality: Cardinality,
) -> ArcDef {
    ArcDef {
        arc_type: rel_type.to_string(),
        family,
        scope: None,
        source,
        target,
        cardinality,
        llm_context: format!("{rel_type} context."),
        properties: None,
        is_self_referential: None,
        inverse_of: None,
        inverse_name: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_node_basic() {
        let node = make_node("Page", "org", "structure", NodeTrait::Defined);
        assert_eq!(node.def.name, "Page");
        assert_eq!(node.realm, "org");
        assert_eq!(node.layer, "structure");
        assert_eq!(node.def.node_trait, NodeTrait::Defined);
    }

    #[test]
    fn test_make_node_simple_defaults_to_invariant() {
        let node = make_node_simple("Term", "shared", "knowledge");
        assert_eq!(node.def.node_trait, NodeTrait::Defined);
    }

    #[test]
    fn test_make_node_with_props() {
        let node = make_node_with_props(
            "Entity",
            "org",
            "semantic",
            NodeTrait::Defined,
            vec![("key", "string", true), ("name", "string", false)],
        );
        let props = node.def.properties.unwrap();
        assert_eq!(props.len(), 2);
        assert!(props.get("key").unwrap().required.unwrap());
        assert!(!props.get("name").unwrap().required.unwrap());
    }

    #[test]
    fn test_make_rel_basic() {
        let arc = make_rel("HAS_PAGE", ArcFamily::Ownership, "Project", "Page");
        assert_eq!(arc.arc_type, "HAS_PAGE");
        assert_eq!(arc.family, ArcFamily::Ownership);
        assert_eq!(arc.cardinality, Cardinality::OneToMany);
    }

    #[test]
    fn test_make_rel_full() {
        let arc = make_rel_full(
            "USES_ENTITY",
            ArcFamily::Semantic,
            NodeRef::Multiple(vec!["Page".into(), "Block".into()]),
            NodeRef::Single("Entity".into()),
            Cardinality::ManyToMany,
        );
        assert_eq!(arc.cardinality, Cardinality::ManyToMany);
        assert!(matches!(arc.source, NodeRef::Multiple(_)));
    }
}
