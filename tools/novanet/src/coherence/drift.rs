//! Drift detection types for YAML ↔ Neo4j property coherence.
//!
//! A "drift" is any divergence between what the YAML schema defines
//! and what Neo4j actually stores in a node or arc.

use serde::{Deserialize, Serialize};

/// Classification of a single drift between YAML and Neo4j.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DriftKind {
    /// Property exists in Neo4j but not defined in YAML.
    Orphan,
    /// Property defined in YAML but absent from Neo4j instances.
    Missing { yaml_type: String },
    /// Property exists in both but Neo4j stores a different type.
    TypeMismatch { yaml_type: String, neo4j_type: String },
    /// Property is defined as enum in YAML but Neo4j has invalid value.
    EnumViolation { invalid_value: String, allowed: Vec<String> },
    /// Composite key doesn't match the expected pattern.
    KeyPatternViolation { actual: String, pattern: String },
    /// Property exists but in wrong position vs BLOC 4 canonical order.
    OrderViolation { expected_position: usize, actual_position: usize },
    /// Instances of the same class have different property sets.
    HeterogeneousInstances { variant_count: usize },
    /// Arc carries a property not defined in the arc YAML schema.
    ArcOrphanProp { arc_type: String, prop: String },
    /// A satellite node (keyless) has a key property it shouldn't.
    SatelliteOrphan,
}

/// Drift found on a specific property of a node class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyDrift {
    /// Property name (e.g. "locale", "title").
    pub prop: String,
    /// What kind of drift this is.
    pub drift: DriftKind,
}

/// All drift for one node class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDrift {
    /// Class name (e.g. "EntityNative", "BlockNative").
    pub class: String,
    /// Total instances in Neo4j.
    pub instance_count: usize,
    /// All detected drifts for this class.
    pub drifts: Vec<PropertyDrift>,
}

impl NodeDrift {
    /// Whether this class has any drift.
    pub fn has_drift(&self) -> bool {
        !self.drifts.is_empty()
    }

    /// Count of drifts by kind discriminant name.
    pub fn count_orphans(&self) -> usize {
        self.drifts
            .iter()
            .filter(|d| d.drift == DriftKind::Orphan)
            .count()
    }

    pub fn count_missing(&self) -> usize {
        self.drifts
            .iter()
            .filter(|d| matches!(d.drift, DriftKind::Missing { .. }))
            .count()
    }
}

/// All drift for one arc class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcDrift {
    /// Arc type name (e.g. "HAS_NATIVE", "DERIVED_SLUG_FROM").
    pub arc_type: String,
    /// All detected drifts for this arc.
    pub drifts: Vec<PropertyDrift>,
}

/// Full drift report for the whole schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    /// When this report was generated.
    pub generated_at: String,
    /// Schema version (from YAML taxonomy or CLAUDE.md).
    pub schema_version: String,
    /// Per-class node drift.
    pub nodes: Vec<NodeDrift>,
    /// Per-arc-type arc drift.
    pub arcs: Vec<ArcDrift>,
    /// Total drift count across all nodes + arcs.
    pub total_drift_count: usize,
    /// Count of orphan-only drift (highest priority to remove).
    pub orphan_count: usize,
    /// Count of missing-only drift (required props absent).
    pub missing_count: usize,
}

impl DriftReport {
    /// Whether the schema is fully coherent (no drift at all).
    pub fn is_clean(&self) -> bool {
        self.total_drift_count == 0
    }

    /// Count of node classes with at least one drift.
    pub fn dirty_class_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.has_drift()).count()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests — RED phase: write tests FIRST, implementation comes after
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drift_kind_orphan_serializes() {
        let d = DriftKind::Orphan;
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"kind\":\"orphan\""));
    }

    #[test]
    fn test_drift_kind_type_mismatch_roundtrip() {
        let d = DriftKind::TypeMismatch {
            yaml_type: "datetime".to_string(),
            neo4j_type: "STRING".to_string(),
        };
        let json = serde_json::to_string(&d).unwrap();
        let back: DriftKind = serde_json::from_str(&json).unwrap();
        assert_eq!(d, back);
    }

    #[test]
    fn test_drift_kind_missing_roundtrip() {
        let d = DriftKind::Missing { yaml_type: "string".to_string() };
        let json = serde_json::to_string(&d).unwrap();
        let back: DriftKind = serde_json::from_str(&json).unwrap();
        assert_eq!(d, back);
    }

    #[test]
    fn test_node_drift_count_orphans() {
        let nd = NodeDrift {
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
        };
        assert_eq!(nd.count_orphans(), 2);
        assert_eq!(nd.count_missing(), 1);
        assert!(nd.has_drift());
    }

    #[test]
    fn test_drift_report_is_clean_when_empty() {
        let report = DriftReport {
            generated_at: "2026-02-17T00:00:00Z".to_string(),
            schema_version: "v0.13.1".to_string(),
            nodes: vec![],
            arcs: vec![],
            total_drift_count: 0,
            orphan_count: 0,
            missing_count: 0,
        };
        assert!(report.is_clean());
        assert_eq!(report.dirty_class_count(), 0);
    }

    #[test]
    fn test_drift_report_dirty_class_count() {
        let report = DriftReport {
            generated_at: "2026-02-17T00:00:00Z".to_string(),
            schema_version: "v0.13.1".to_string(),
            nodes: vec![
                NodeDrift {
                    class: "EntityNative".to_string(),
                    instance_count: 324,
                    drifts: vec![PropertyDrift {
                        prop: "locale".to_string(),
                        drift: DriftKind::Orphan,
                    }],
                },
                NodeDrift {
                    class: "BlockNative".to_string(),
                    instance_count: 9,
                    drifts: vec![],
                },
            ],
            arcs: vec![],
            total_drift_count: 1,
            orphan_count: 1,
            missing_count: 0,
        };
        assert!(!report.is_clean());
        assert_eq!(report.dirty_class_count(), 1); // only EntityNative has drift
    }

    #[test]
    fn test_satellite_orphan_serializes() {
        let d = DriftKind::SatelliteOrphan;
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("satellite_orphan"));
    }
}
