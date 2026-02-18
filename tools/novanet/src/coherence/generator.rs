//! Coherence script generator — produces idempotent Cypher from drift reports.
//!
//! Given a `NodeDrift`, `CoherenceGenerator` emits a Cypher script that:
//! - REMOVEs orphan properties (Axis A)
//! - SETDs default values for missing required properties (Axis B)
//! - Produces type constraint ALTER statements (Axis G)
//!
//! Scripts are idempotent: safe to run multiple times.

use crate::coherence::drift::{ArcDrift, DriftKind, NodeDrift, PropertyDrift};

/// Generated Cypher script for a node class migration.
#[derive(Debug, Clone)]
pub struct CoherenceScript {
    /// Human-readable label for what this script does.
    pub label: String,
    /// Full Cypher content, ready to execute.
    pub content: String,
    /// Number of operations in this script.
    pub operation_count: usize,
}

/// Generates idempotent Cypher scripts from drift reports.
pub struct CoherenceGenerator;

impl CoherenceGenerator {
    /// Generate a Cypher script to fix all drift for a node class.
    ///
    /// Returns `None` if there is no drift to fix.
    pub fn generate_for_node(node_drift: &NodeDrift) -> Option<CoherenceScript> {
        if !node_drift.has_drift() {
            return None;
        }

        let mut lines: Vec<String> = vec![
            format!("// coherence-{}.cypher", to_kebab(&node_drift.class)),
            format!("// NovaNet v0.13.1 coherence — {}", node_drift.class),
            "// Idempotent: safe to run multiple times".to_string(),
            String::new(),
        ];

        let orphans: Vec<&PropertyDrift> = node_drift
            .drifts
            .iter()
            .filter(|d| d.drift == DriftKind::Orphan)
            .collect();

        let missing: Vec<&PropertyDrift> = node_drift
            .drifts
            .iter()
            .filter(|d| matches!(d.drift, DriftKind::Missing { .. }))
            .collect();

        let mut op_count = 0;

        // STEP 1: Remove orphan props
        if !orphans.is_empty() {
            let remove_list = orphans
                .iter()
                .map(|d| format!("n.{}", d.prop))
                .collect::<Vec<_>>()
                .join(", ");

            let where_clause = orphans
                .iter()
                .map(|d| format!("n.{} IS NOT NULL", d.prop))
                .collect::<Vec<_>>()
                .join(" OR ");

            lines.push("// STEP 1: Remove orphan properties".to_string());
            lines.push(format!("MATCH (n:{})", node_drift.class));
            lines.push(format!("WHERE {}", where_clause));
            lines.push(format!("REMOVE {}", remove_list));
            lines.push("RETURN count(n) AS orphans_removed".to_string());
            lines.push(";".to_string());
            lines.push(String::new());
            op_count += 1;
        }

        // STEP 2: Add missing props with defaults
        for pd in &missing {
            if let DriftKind::Missing { yaml_type } = &pd.drift {
                let default_expr = default_for_type(yaml_type);
                lines.push(format!("// STEP: Add missing '{}' ({})", pd.prop, yaml_type));
                lines.push(format!("MATCH (n:{})", node_drift.class));
                lines.push(format!("WHERE n.{} IS NULL", pd.prop));
                lines.push(format!("SET n.{} = {}", pd.prop, default_expr));
                lines.push(format!(
                    "RETURN count(n) AS {}_added",
                    pd.prop
                ));
                lines.push(";".to_string());
                lines.push(String::new());
                op_count += 1;
            }
        }

        // STEP LAST: Verification query (returns 0 rows if clean)
        if !orphans.is_empty() {
            let orphan_props = orphans
                .iter()
                .map(|d| format!("'{}'", d.prop))
                .collect::<Vec<_>>()
                .join(", ");

            lines.push("// VERIFICATION: Should return 0 rows if clean".to_string());
            lines.push(format!("MATCH (n:{})", node_drift.class));
            lines.push(format!(
                "WITH n, [k IN keys(n) WHERE k IN [{}]] AS remaining",
                orphan_props
            ));
            lines.push("WHERE size(remaining) > 0".to_string());
            lines.push("RETURN n.key AS still_dirty, remaining LIMIT 5".to_string());
            lines.push(";".to_string());
        }

        Some(CoherenceScript {
            label: format!("Fix {}", node_drift.class),
            content: lines.join("\n"),
            operation_count: op_count,
        })
    }

    /// Generate a type constraint statement for a node property.
    ///
    /// Returns the Cypher `CREATE CONSTRAINT` statement.
    pub fn generate_type_constraint(
        node_label: &str,
        prop: &str,
        neo4j_type: &str,
    ) -> String {
        let constraint_name = format!(
            "{}_{}_{}_type",
            to_snake(node_label),
            to_snake(prop),
            neo4j_type.to_lowercase().replace([':', '<', '>', ' '], "_").trim_matches('_')
        );
        format!(
            "CREATE CONSTRAINT {constraint_name} IF NOT EXISTS FOR (n:{node_label}) REQUIRE n.{prop} {neo4j_type};"
        )
    }

    /// Generate a coherence script for an arc class drift.
    pub fn generate_for_arc(arc_drift: &ArcDrift) -> Option<CoherenceScript> {
        if arc_drift.drifts.is_empty() {
            return None;
        }

        let orphans: Vec<&PropertyDrift> = arc_drift
            .drifts
            .iter()
            .filter(|d| matches!(d.drift, DriftKind::ArcOrphanProp { .. }))
            .collect();

        if orphans.is_empty() {
            return None;
        }

        let remove_list = orphans
            .iter()
            .map(|d| format!("r.{}", d.prop))
            .collect::<Vec<_>>()
            .join(", ");

        let where_clause = orphans
            .iter()
            .map(|d| format!("r.{} IS NOT NULL", d.prop))
            .collect::<Vec<_>>()
            .join(" OR ");

        let content = format!(
            "// coherence-arc-{arc}.cypher\n\
             // Remove orphan props from :{arc} arcs\n\
             MATCH ()-[r:{arc}]->()\n\
             WHERE {where_clause}\n\
             REMOVE {remove_list}\n\
             RETURN count(r) AS arc_orphans_removed\n;",
            arc = arc_drift.arc_type,
        );

        Some(CoherenceScript {
            label: format!("Fix arc {}", arc_drift.arc_type),
            content,
            operation_count: 1,
        })
    }
}

/// Convert PascalCase to kebab-case for file names.
fn to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('-');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

/// Convert PascalCase to snake_case for constraint names.
fn to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

/// Default Cypher expression for a missing required property.
fn default_for_type(yaml_type: &str) -> &'static str {
    match yaml_type {
        "string" => "''",
        "int" => "1",
        "float" => "0.0",
        "boolean" => "false",
        "datetime" => "datetime()",
        "string[]" => "[]",
        "int[]" => "[]",
        _ => "''",
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coherence::drift::PropertyDrift;

    fn entity_native_drift() -> NodeDrift {
        NodeDrift {
            class: "EntityNative".to_string(),
            instance_count: 324,
            drifts: vec![
                PropertyDrift { prop: "locale".to_string(), drift: DriftKind::Orphan },
                PropertyDrift { prop: "title".to_string(), drift: DriftKind::Orphan },
                PropertyDrift {
                    prop: "created_at".to_string(),
                    drift: DriftKind::Missing { yaml_type: "datetime".to_string() },
                },
            ],
        }
    }

    #[test]
    fn test_generate_for_node_returns_none_when_clean() {
        let clean = NodeDrift {
            class: "BlockType".to_string(),
            instance_count: 5,
            drifts: vec![],
        };
        assert!(CoherenceGenerator::generate_for_node(&clean).is_none());
    }

    #[test]
    fn test_generate_for_node_entity_native_contains_remove() {
        let script = CoherenceGenerator::generate_for_node(&entity_native_drift()).unwrap();
        assert!(script.content.contains("REMOVE n.locale, n.title")
            || script.content.contains("REMOVE n.title, n.locale"),
            "Script should REMOVE orphan props: {}", script.content);
    }

    #[test]
    fn test_generate_for_node_contains_set_datetime_default() {
        let script = CoherenceGenerator::generate_for_node(&entity_native_drift()).unwrap();
        assert!(
            script.content.contains("n.created_at = datetime()"),
            "Script should SET datetime() for missing created_at: {}",
            script.content
        );
    }

    #[test]
    fn test_generate_for_node_is_idempotent_header() {
        let script = CoherenceGenerator::generate_for_node(&entity_native_drift()).unwrap();
        assert!(script.content.contains("Idempotent"));
    }

    #[test]
    fn test_generate_for_node_has_verification_query() {
        let script = CoherenceGenerator::generate_for_node(&entity_native_drift()).unwrap();
        assert!(
            script.content.contains("VERIFICATION"),
            "Script should include verification query"
        );
    }

    #[test]
    fn test_generate_type_constraint_datetime() {
        let stmt = CoherenceGenerator::generate_type_constraint(
            "EntityNative",
            "created_at",
            "IS :: ZONED DATETIME",
        );
        assert!(stmt.contains("EntityNative"));
        assert!(stmt.contains("created_at"));
        assert!(stmt.contains("IS :: ZONED DATETIME"));
        assert!(stmt.contains("IF NOT EXISTS"));
    }

    #[test]
    fn test_to_kebab_pascal_case() {
        assert_eq!(to_kebab("EntityNative"), "entity-native");
        assert_eq!(to_kebab("BlockNative"), "block-native");
        assert_eq!(to_kebab("PageNative"), "page-native");
    }

    #[test]
    fn test_operation_count_reflects_steps() {
        let script = CoherenceGenerator::generate_for_node(&entity_native_drift()).unwrap();
        // 1 REMOVE step + 1 SET step for created_at
        assert_eq!(script.operation_count, 2);
    }
}
