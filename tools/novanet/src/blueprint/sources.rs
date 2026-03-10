//! Data sources for blueprint command.
//!
//! Loads and aggregates data from:
//! - YAML node-classes and arc-classes
//! - taxonomy.yaml
//! - Neo4j schema nodes (optional)

use crate::db::Db;
use crate::parsers::arcs::{ArcDef, ArcFamily, Cardinality};
use crate::parsers::taxonomy::{ArcFamilyDef, NodeRealmDef, TaxonomyDoc};
use crate::parsers::yaml_node::ParsedNode;
use std::collections::HashMap;
use std::path::Path;

/// Aggregated data from all sources for blueprint rendering.
#[derive(Debug)]
pub struct BlueprintData {
    /// All parsed node classes from YAML.
    pub node_classes: Vec<ParsedNode>,
    /// All arc definitions from YAML.
    pub arc_defs: Vec<ArcDef>,
    /// Taxonomy (realms, layers, traits, arc families).
    pub taxonomy: TaxonomyDoc,
    /// Neo4j schema node counts (if connected).
    pub neo4j_counts: Option<Neo4jCounts>,
}

/// Counts from Neo4j for validation.
#[derive(Debug, Clone)]
pub struct Neo4jCounts {
    pub node_class_count: usize,
    pub arc_class_count: usize,
    pub node_class_names: Vec<String>,
    pub arc_class_names: Vec<String>,
}

impl BlueprintData {
    /// Load from YAML only (no Neo4j connection required).
    pub fn from_yaml(root: &Path) -> crate::Result<Self> {
        let node_classes = crate::parsers::yaml_node::load_all_nodes(root)?;
        let arcs_doc = crate::parsers::arcs::load_arc_classes_from_files(root)?;
        // v0.12.5: Load from individual YAML files
        let taxonomy = crate::parsers::taxonomy::load_taxonomy_from_files(root)?;

        Ok(Self {
            node_classes,
            arc_defs: arcs_doc.arcs,
            taxonomy,
            neo4j_counts: None,
        })
    }

    /// Load from YAML + Neo4j.
    pub async fn from_all(root: &Path, db: &Db) -> crate::Result<Self> {
        let mut data = Self::from_yaml(root)?;
        data.neo4j_counts = Some(Self::query_neo4j_counts(db).await?);
        Ok(data)
    }

    /// Query Neo4j for schema node counts.
    ///
    /// Uses correct labels and property names (ADR-023 v0.12.0):
    /// - `:Schema:Class` nodes have `label` property (e.g., "Page", "Entity")
    /// - `:Schema:ArcClass` nodes have `key` property (e.g., "HAS_PAGE", "USES_ENTITY")
    async fn query_neo4j_counts(db: &Db) -> crate::Result<Neo4jCounts> {
        // Count Class schema nodes (double-label :Schema:Class, property: label)
        let node_query = "MATCH (c:Schema:Class) RETURN c.label AS name ORDER BY c.label";
        let node_rows = db.execute(node_query).await?;
        let node_class_names: Vec<String> = node_rows
            .iter()
            .filter_map(|row: &neo4rs::Row| row.get::<String>("name").ok())
            .collect();

        // Count ArcClass schema nodes (double-label :Schema:ArcClass, property: key)
        let arc_query = "MATCH (ac:Schema:ArcClass) RETURN ac.key AS name ORDER BY ac.key";
        let arc_rows = db.execute(arc_query).await?;
        let arc_class_names: Vec<String> = arc_rows
            .iter()
            .filter_map(|row: &neo4rs::Row| row.get::<String>("name").ok())
            .collect();

        Ok(Neo4jCounts {
            node_class_count: node_class_names.len(),
            arc_class_count: arc_class_names.len(),
            node_class_names,
            arc_class_names,
        })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Statistics helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Total node classes count.
    pub fn node_class_count(&self) -> usize {
        self.node_classes.len()
    }

    /// Total arc definitions count.
    pub fn arc_count(&self) -> usize {
        self.arc_defs.len()
    }

    /// Count of realms.
    pub fn realm_count(&self) -> usize {
        self.taxonomy.node_realms.len()
    }

    /// Total layer count across all realms.
    pub fn layer_count(&self) -> usize {
        self.taxonomy
            .node_realms
            .iter()
            .map(|r| r.layers.len())
            .sum()
    }

    /// Node classes grouped by realm.
    pub fn nodes_by_realm(&self) -> HashMap<&str, Vec<&ParsedNode>> {
        let mut map: HashMap<&str, Vec<&ParsedNode>> = HashMap::new();
        for node in &self.node_classes {
            map.entry(node.realm.as_str()).or_default().push(node);
        }
        map
    }

    /// Node classes grouped by layer.
    pub fn nodes_by_layer(&self) -> HashMap<&str, Vec<&ParsedNode>> {
        let mut map: HashMap<&str, Vec<&ParsedNode>> = HashMap::new();
        for node in &self.node_classes {
            map.entry(node.layer.as_str()).or_default().push(node);
        }
        map
    }

    /// Arc definitions grouped by family.
    pub fn arcs_by_family(&self) -> HashMap<ArcFamily, Vec<&ArcDef>> {
        let mut map: HashMap<ArcFamily, Vec<&ArcDef>> = HashMap::new();
        for arc in &self.arc_defs {
            map.entry(arc.family).or_default().push(arc);
        }
        map
    }

    /// Arc definitions grouped by cardinality.
    pub fn arcs_by_cardinality(&self) -> HashMap<Cardinality, Vec<&ArcDef>> {
        let mut map: HashMap<Cardinality, Vec<&ArcDef>> = HashMap::new();
        for arc in &self.arc_defs {
            map.entry(arc.cardinality).or_default().push(arc);
        }
        map
    }

    /// Get realm definition by key.
    pub fn get_realm(&self, key: &str) -> Option<&NodeRealmDef> {
        self.taxonomy.node_realms.iter().find(|r| r.key == key)
    }

    /// Get arc family definition by key.
    pub fn get_arc_family(&self, key: &str) -> Option<&ArcFamilyDef> {
        self.taxonomy.arc_families.iter().find(|f| f.key == key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires private brain/ YAML models (run locally)"]
    fn test_blueprint_data_from_yaml() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        assert!(data.node_class_count() > 0, "Should have node classes");
        assert!(data.arc_count() > 0, "Should have arc definitions");
        assert!(
            data.realm_count() == 2,
            "Should have 2 realms (shared, org)"
        );
        assert!(
            data.layer_count() == 10,
            "Should have 10 layers (v11.4: 4 shared + 6 org)"
        );
    }

    #[test]
    #[ignore = "requires private brain/ YAML models (run locally)"]
    fn test_nodes_by_realm() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let by_realm = data.nodes_by_realm();
        assert!(by_realm.contains_key("shared"), "Should have shared realm");
        assert!(by_realm.contains_key("org"), "Should have org realm");
    }

    #[test]
    #[ignore = "requires private brain/ YAML models (run locally)"]
    fn test_arcs_by_family() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let by_family = data.arcs_by_family();
        assert!(
            by_family.contains_key(&ArcFamily::Ownership),
            "Should have ownership arcs"
        );
    }
}
