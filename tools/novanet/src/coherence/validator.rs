//! YAML schema coherence validator.
//!
//! Compares YAML-defined properties against Neo4j live data to detect drift.
//! All functions in this module are pure (no Neo4j calls) — they operate on
//! data already fetched and passed as arguments.

use crate::coherence::drift::{DriftKind, NodeDrift, PropertyDrift};

/// A property as defined in a YAML node-class file.
#[derive(Debug, Clone)]
pub struct YamlPropDef {
    /// Property name.
    pub name: String,
    /// YAML type string (e.g. "string", "datetime", "int").
    pub yaml_type: String,
    /// Whether this property is required.
    pub required: bool,
}

/// A snapshot of properties found on Neo4j instances of a class.
#[derive(Debug, Clone)]
pub struct Neo4jPropSnapshot {
    /// Class name (e.g. "EntityNative").
    pub class: String,
    /// Total number of instances.
    pub instance_count: usize,
    /// Properties found across all instances.
    /// Keys are property names, value is the Neo4j type string seen.
    pub props_seen: std::collections::HashMap<String, String>,
    /// Number of distinct property sets across all instances.
    pub variant_count: usize,
}

/// Validate coherence between YAML definition and Neo4j snapshot.
///
/// Returns a `NodeDrift` with all detected issues. Empty drifts = clean.
pub fn validate_node_coherence(
    class: &str,
    yaml_props: &[YamlPropDef],
    snapshot: &Neo4jPropSnapshot,
) -> NodeDrift {
    let mut drifts: Vec<PropertyDrift> = Vec::new();

    let yaml_prop_names: std::collections::HashSet<&str> =
        yaml_props.iter().map(|p| p.name.as_str()).collect();

    // Axis A: Orphans — props in Neo4j not in YAML
    for neo4j_prop in snapshot.props_seen.keys() {
        if !yaml_prop_names.contains(neo4j_prop.as_str()) {
            drifts.push(PropertyDrift {
                prop: neo4j_prop.clone(),
                drift: DriftKind::Orphan,
            });
        }
    }

    // Axis B: Missing — required YAML props absent from Neo4j
    for yaml_prop in yaml_props {
        if yaml_prop.required && !snapshot.props_seen.contains_key(&yaml_prop.name) {
            drifts.push(PropertyDrift {
                prop: yaml_prop.name.clone(),
                drift: DriftKind::Missing {
                    yaml_type: yaml_prop.yaml_type.clone(),
                },
            });
        }
    }

    // Axis C: TypeMismatch — type differs between YAML and Neo4j
    for yaml_prop in yaml_props {
        if let Some(neo4j_type) = snapshot.props_seen.get(&yaml_prop.name) {
            let expected = expected_neo4j_type(&yaml_prop.yaml_type);
            if let Some(expected) = expected {
                // Neo4j returns types like "STRING NOT NULL" or "ZONED DATETIME NOT NULL"
                // We compare the base type (strip "NOT NULL" suffix for comparison)
                let neo4j_base = neo4j_type
                    .replace(" NOT NULL", "")
                    .replace(" NULL", "")
                    .trim()
                    .to_string();
                if !neo4j_base.eq_ignore_ascii_case(expected) {
                    drifts.push(PropertyDrift {
                        prop: yaml_prop.name.clone(),
                        drift: DriftKind::TypeMismatch {
                            yaml_type: yaml_prop.yaml_type.clone(),
                            neo4j_type: neo4j_type.clone(),
                        },
                    });
                }
            }
        }
    }

    // Axis F: Heterogeneous instances
    if snapshot.variant_count > 1 {
        drifts.push(PropertyDrift {
            prop: String::new(), // no specific prop
            drift: DriftKind::HeterogeneousInstances {
                variant_count: snapshot.variant_count,
            },
        });
    }

    // Sort for deterministic output
    drifts.sort_by(|a, b| a.prop.cmp(&b.prop));

    NodeDrift {
        class: class.to_string(),
        instance_count: snapshot.instance_count,
        drifts,
    }
}

/// Map YAML type → expected Neo4j base type (for comparison).
fn expected_neo4j_type(yaml_type: &str) -> Option<&'static str> {
    match yaml_type {
        "string" => Some("STRING"),
        "int" => Some("INTEGER"),
        "float" => Some("FLOAT"),
        "boolean" => Some("BOOLEAN"),
        "datetime" => Some("ZONED DATETIME"),
        "string[]" => Some("LIST<STRING NOT NULL>"),
        "int[]" => Some("LIST<INTEGER NOT NULL>"),
        _ => None,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_snapshot(class: &str, props: &[(&str, &str)], variant_count: usize) -> Neo4jPropSnapshot {
        Neo4jPropSnapshot {
            class: class.to_string(),
            instance_count: 324,
            props_seen: props
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            variant_count,
        }
    }

    fn make_yaml_props(props: &[(&str, &str, bool)]) -> Vec<YamlPropDef> {
        props
            .iter()
            .map(|(name, t, req)| YamlPropDef {
                name: name.to_string(),
                yaml_type: t.to_string(),
                required: *req,
            })
            .collect()
    }

    #[test]
    fn test_clean_node_has_no_drift() {
        let yaml_props = make_yaml_props(&[
            ("key", "string", true),
            ("created_at", "datetime", true),
        ]);
        let snapshot = make_snapshot(
            "EntityNative",
            &[
                ("key", "STRING NOT NULL"),
                ("created_at", "ZONED DATETIME NOT NULL"),
            ],
            1,
        );

        let result = validate_node_coherence("EntityNative", &yaml_props, &snapshot);
        assert!(!result.has_drift(), "Expected no drift but got: {:?}", result.drifts);
    }

    #[test]
    fn test_orphan_detected() {
        let yaml_props = make_yaml_props(&[("key", "string", true)]);
        let snapshot = make_snapshot(
            "EntityNative",
            &[("key", "STRING NOT NULL"), ("locale", "STRING NOT NULL")],
            1,
        );

        let result = validate_node_coherence("EntityNative", &yaml_props, &snapshot);
        assert_eq!(result.count_orphans(), 1);
        assert_eq!(result.drifts[0].prop, "locale");
        assert_eq!(result.drifts[0].drift, DriftKind::Orphan);
    }

    #[test]
    fn test_missing_required_detected() {
        let yaml_props = make_yaml_props(&[
            ("key", "string", true),
            ("created_at", "datetime", true),
        ]);
        let snapshot = make_snapshot("EntityNative", &[("key", "STRING NOT NULL")], 1);

        let result = validate_node_coherence("EntityNative", &yaml_props, &snapshot);
        assert_eq!(result.count_missing(), 1);

        let missing_drift = result.drifts.iter().find(|d| d.prop == "created_at").unwrap();
        assert!(matches!(&missing_drift.drift, DriftKind::Missing { yaml_type } if yaml_type == "datetime"));
    }

    #[test]
    fn test_optional_missing_not_flagged() {
        // Optional (required=false) props absent from Neo4j should NOT be Missing drift
        let yaml_props = make_yaml_props(&[
            ("key", "string", true),
            ("llm_context", "string", false), // optional
        ]);
        let snapshot = make_snapshot("EntityNative", &[("key", "STRING NOT NULL")], 1);

        let result = validate_node_coherence("EntityNative", &yaml_props, &snapshot);
        // No drift: key is present, llm_context is optional so absence is fine
        assert!(!result.has_drift(), "Optional missing prop should not cause drift: {:?}", result.drifts);
    }

    #[test]
    fn test_heterogeneous_instances_detected() {
        let yaml_props = make_yaml_props(&[("key", "string", true)]);
        let snapshot = make_snapshot(
            "EntityNative",
            &[("key", "STRING NOT NULL")],
            3, // 3 different prop sets
        );

        let result = validate_node_coherence("EntityNative", &yaml_props, &snapshot);
        let het_drift = result
            .drifts
            .iter()
            .find(|d| matches!(&d.drift, DriftKind::HeterogeneousInstances { .. }));
        assert!(het_drift.is_some(), "Should detect heterogeneous instances");
    }

    #[test]
    fn test_entity_native_real_case() {
        // Exact case from BATCH 0 audit: EntityNative with locale + title as orphans
        let yaml_props = make_yaml_props(&[
            ("key", "string", true),
            ("entity_key", "string", true),
            ("locale_key", "string", true),
            ("display_name", "string", true),
            ("description", "string", true),
            ("created_at", "datetime", true),
            ("updated_at", "datetime", true),
        ]);

        let mut props = HashMap::new();
        props.insert("key".to_string(), "STRING NOT NULL".to_string());
        props.insert("entity_key".to_string(), "STRING NOT NULL".to_string());
        props.insert("locale_key".to_string(), "STRING NOT NULL".to_string());
        props.insert("display_name".to_string(), "STRING NOT NULL".to_string());
        props.insert("description".to_string(), "STRING NOT NULL".to_string());
        props.insert("created_at".to_string(), "ZONED DATETIME NOT NULL".to_string());
        props.insert("updated_at".to_string(), "ZONED DATETIME NOT NULL".to_string());
        // Orphans from audit:
        props.insert("locale".to_string(), "STRING NOT NULL".to_string());
        props.insert("title".to_string(), "STRING NOT NULL".to_string());

        let snapshot = Neo4jPropSnapshot {
            class: "EntityNative".to_string(),
            instance_count: 324,
            props_seen: props,
            variant_count: 2,
        };

        let result = validate_node_coherence("EntityNative", &yaml_props, &snapshot);
        assert_eq!(result.count_orphans(), 2, "Should detect locale + title as orphans");
        // Heterogeneous (variant_count=2) should also be flagged
        assert!(result.drifts.iter().any(|d| matches!(d.drift, DriftKind::HeterogeneousInstances { .. })));
    }
}
