//! Meta-graph reader for traversal rules.
//!
//! v10: Reads Kind, ArcFamily, and ArcKind rules from Neo4j meta-graph
//! to enable data-driven context assembly.

use crate::error::Result;
use neo4rs::Graph;
use std::collections::HashMap;
use std::sync::Arc;

use super::types::TraversalMode;

/// Rules for a Kind (node type).
#[derive(Debug, Clone)]
pub struct KindRules {
    pub key: String,
    pub trait_type: String,
    pub traversal_depth: u8,
    pub context_budget: u32,
    pub token_estimate: u32,
}

/// Rules for an ArcFamily.
#[derive(Debug, Clone)]
pub struct ArcFamilyRules {
    pub key: String,
    pub default_traversal: TraversalMode,
}

/// Rules for an ArcKind.
#[derive(Debug, Clone)]
pub struct ArcKindRules {
    pub key: String,
    pub family: String,
    pub source: String,
    pub target: String,
    pub temperature_threshold: Option<f32>,
}

/// Reads traversal rules from Neo4j meta-graph.
pub struct MetaGraphReader {
    graph: Arc<Graph>,
    kind_rules: HashMap<String, KindRules>,
    family_rules: HashMap<String, ArcFamilyRules>,
    arc_rules: HashMap<String, ArcKindRules>,
}

impl MetaGraphReader {
    /// Create a new reader and load rules from Neo4j.
    pub async fn new(graph: Arc<Graph>) -> Result<Self> {
        let mut reader = Self {
            graph,
            kind_rules: HashMap::new(),
            family_rules: HashMap::new(),
            arc_rules: HashMap::new(),
        };
        reader.load_rules().await?;
        Ok(reader)
    }

    /// Load all rules from meta-graph.
    async fn load_rules(&mut self) -> Result<()> {
        self.load_kind_rules().await?;
        self.load_family_rules().await?;
        self.load_arc_rules().await?;
        Ok(())
    }

    async fn load_kind_rules(&mut self) -> Result<()> {
        let query = r#"
            MATCH (k:Kind:Meta)-[:HAS_TRAIT]->(t:Trait:Meta)
            RETURN k.key AS key, t.key AS trait_type,
                   coalesce(k.traversal_depth, 2) AS traversal_depth,
                   coalesce(k.context_budget, 500) AS context_budget,
                   coalesce(k.token_estimate, 100) AS token_estimate
        "#;

        let mut result = self.graph.execute(neo4rs::query(query)).await?;
        while let Some(row) = result.next().await? {
            let rules = KindRules {
                key: row.get("key")?,
                trait_type: row.get("trait_type")?,
                traversal_depth: row.get::<i64>("traversal_depth")? as u8,
                context_budget: row.get::<i64>("context_budget")? as u32,
                token_estimate: row.get::<i64>("token_estimate")? as u32,
            };
            self.kind_rules.insert(rules.key.clone(), rules);
        }
        Ok(())
    }

    async fn load_family_rules(&mut self) -> Result<()> {
        let query = r#"
            MATCH (af:ArcFamily:Meta)
            RETURN af.key AS key,
                   coalesce(af.default_traversal, 'lazy') AS default_traversal
        "#;

        let mut result = self.graph.execute(neo4rs::query(query)).await?;
        while let Some(row) = result.next().await? {
            let traversal_str: String = row.get("default_traversal")?;
            let rules = ArcFamilyRules {
                key: row.get("key")?,
                default_traversal: match traversal_str.as_str() {
                    "eager" => TraversalMode::Eager,
                    "skip" => TraversalMode::Skip,
                    _ => TraversalMode::Lazy,
                },
            };
            self.family_rules.insert(rules.key.clone(), rules);
        }
        Ok(())
    }

    async fn load_arc_rules(&mut self) -> Result<()> {
        let query = r#"
            MATCH (ak:ArcKind:Meta)-[:IN_FAMILY]->(af:ArcFamily:Meta)
            MATCH (ak)-[:FROM_KIND]->(source:Kind:Meta)
            MATCH (ak)-[:TO_KIND]->(target:Kind:Meta)
            RETURN ak.key AS key, af.key AS family,
                   source.key AS source, target.key AS target,
                   ak.temperature_threshold AS temperature_threshold
        "#;

        let mut result = self.graph.execute(neo4rs::query(query)).await?;
        while let Some(row) = result.next().await? {
            let rules = ArcKindRules {
                key: row.get("key")?,
                family: row.get("family")?,
                source: row.get("source")?,
                target: row.get("target")?,
                temperature_threshold: row
                    .get::<Option<f64>>("temperature_threshold")?
                    .map(|v| v as f32),
            };
            self.arc_rules.insert(rules.key.clone(), rules);
        }
        Ok(())
    }

    /// Get rules for a Kind.
    pub fn get_kind_rules(&self, key: &str) -> Option<&KindRules> {
        self.kind_rules.get(key)
    }

    /// Get rules for an ArcFamily.
    pub fn get_family_rules(&self, key: &str) -> Option<&ArcFamilyRules> {
        self.family_rules.get(key)
    }

    /// Get rules for an ArcKind.
    pub fn get_arc_rules(&self, key: &str) -> Option<&ArcKindRules> {
        self.arc_rules.get(key)
    }

    /// Check if an arc should be traversed given current temperature.
    pub fn should_traverse_arc(&self, arc_key: &str, temperature: f32) -> bool {
        if let Some(rules) = self.arc_rules.get(arc_key) {
            // Check family default
            if let Some(family_rules) = self.family_rules.get(&rules.family) {
                if family_rules.default_traversal == TraversalMode::Skip {
                    return false;
                }
            }
            // Check temperature threshold
            if let Some(threshold) = rules.temperature_threshold {
                return temperature >= threshold;
            }
        }
        true // Default: traverse
    }

    /// Get all loaded kind rules count (for debugging/testing).
    pub fn kind_count(&self) -> usize {
        self.kind_rules.len()
    }

    /// Get all loaded family rules count (for debugging/testing).
    pub fn family_count(&self) -> usize {
        self.family_rules.len()
    }

    /// Get all loaded arc rules count (for debugging/testing).
    pub fn arc_count(&self) -> usize {
        self.arc_rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_kind_rules() -> HashMap<String, KindRules> {
        let mut rules = HashMap::new();
        rules.insert(
            "Block".to_string(),
            KindRules {
                key: "Block".to_string(),
                trait_type: "invariant".to_string(),
                traversal_depth: 3,
                context_budget: 800,
                token_estimate: 150,
            },
        );
        rules
    }

    fn mock_family_rules() -> HashMap<String, ArcFamilyRules> {
        let mut rules = HashMap::new();
        rules.insert(
            "semantic".to_string(),
            ArcFamilyRules {
                key: "semantic".to_string(),
                default_traversal: TraversalMode::Lazy,
            },
        );
        rules.insert(
            "mining".to_string(),
            ArcFamilyRules {
                key: "mining".to_string(),
                default_traversal: TraversalMode::Skip,
            },
        );
        rules
    }

    fn mock_arc_rules() -> HashMap<String, ArcKindRules> {
        let mut rules = HashMap::new();
        rules.insert(
            "USES_ENTITY".to_string(),
            ArcKindRules {
                key: "USES_ENTITY".to_string(),
                family: "semantic".to_string(),
                source: "Block".to_string(),
                target: "Entity".to_string(),
                temperature_threshold: Some(0.0),
            },
        );
        rules.insert(
            "HAS_METRICS".to_string(),
            ArcKindRules {
                key: "HAS_METRICS".to_string(),
                family: "mining".to_string(),
                source: "SEOKeyword".to_string(),
                target: "SEOKeywordMetrics".to_string(),
                temperature_threshold: None,
            },
        );
        rules
    }

    #[test]
    fn test_kind_rules_lookup() {
        let rules = mock_kind_rules();
        let block = rules.get("Block").unwrap();
        assert_eq!(block.trait_type, "invariant");
        assert_eq!(block.traversal_depth, 3);
    }

    #[test]
    fn test_family_rules_lookup() {
        let rules = mock_family_rules();
        let semantic = rules.get("semantic").unwrap();
        assert_eq!(semantic.default_traversal, TraversalMode::Lazy);

        let mining = rules.get("mining").unwrap();
        assert_eq!(mining.default_traversal, TraversalMode::Skip);
    }

    #[test]
    fn test_arc_rules_lookup() {
        let rules = mock_arc_rules();
        let uses = rules.get("USES_ENTITY").unwrap();
        assert_eq!(uses.family, "semantic");
        assert_eq!(uses.temperature_threshold, Some(0.0));
    }
}
