//! Auto-fixer for provenance property violations.
//!
//! Handles PROVENANCE_REQUIRED and PROVENANCE_JSON rules by adding or validating
//! the provenance property in standard_properties.
//!
//! v0.19.0: `provenance` REPLACES `created_by` + `created_by_file`.
//! Format: JSON object with required 'source' field.

use super::{AutoFix, Change, FixAction};
use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::{ParsedNode, PropertyDef};
use serde_yaml::Value;
use std::collections::BTreeMap;

// ─────────────────────────────────────────────────────────────────────────────
// ProvenanceFixer Implementation
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-fixer for provenance property violations.
///
/// Adds missing `provenance` property to standard_properties.
/// v0.19.0: `provenance` replaces `created_by` + `created_by_file`.
///
/// Provenance must be a JSON object with at least a 'source' field:
/// ```json
/// {
///   "source": "human" | "llm" | "import" | "migration",
///   "agent": "optional agent name",
///   "timestamp": "optional ISO timestamp"
/// }
/// ```
pub struct ProvenanceFixer;

impl AutoFix for ProvenanceFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "PROVENANCE_REQUIRED" || issue.rule == "PROVENANCE_JSON"
    }

    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        match issue.rule {
            "PROVENANCE_REQUIRED" => {
                // Check if provenance already exists
                if props.contains_key("provenance") {
                    return Ok(FixAction::Skipped {
                        reason: "Node already has 'provenance' property".to_string(),
                    });
                }

                // Add provenance property with JSON type
                let mut extra = BTreeMap::new();
                extra.insert(
                    "example".to_string(),
                    serde_yaml::to_value(r#"{"source": "human"}"#).unwrap(),
                );

                props.insert(
                    "provenance".to_string(),
                    PropertyDef {
                        prop_type: "json".to_string(),
                        required: Some(true),
                        description: Some(
                            "Provenance tracking: source (human|llm|import|migration), optional agent and timestamp"
                                .to_string(),
                        ),
                        extra,
                    },
                );

                Ok(FixAction::Modified {
                    changes: vec![Change {
                        field: "provenance".to_string(),
                        old_value: None,
                        new_value: Value::String("json (required) - v0.19.0".to_string()),
                    }],
                })
            }
            "PROVENANCE_JSON" => {
                // Provenance exists but is not valid JSON
                // This fixer can't auto-fix invalid JSON content - that requires manual review
                Ok(FixAction::Skipped {
                    reason: "Invalid provenance JSON requires manual review".to_string(),
                })
            }
            _ => Ok(FixAction::Skipped {
                reason: format!("Unknown rule: {}", issue.rule),
            }),
        }
    }

    fn description(&self) -> &str {
        "Adds missing 'provenance' property to standard_properties (v0.19.0)"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;
    use crate::parsers::yaml_node::{NodeDef, ParsedNode, PropertyDef};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create a node without provenance property.
    fn create_node_without_provenance() -> ParsedNode {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Test key".to_string()),
                extra: BTreeMap::new(),
            },
        );
        props.insert(
            "display_name".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Display name".to_string()),
                extra: BTreeMap::new(),
            },
        );

        ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                knowledge_tier: None,
                icon: None,
                description: "Test node without provenance property".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/test-node.yaml"),
        }
    }

    #[test]
    fn test_adds_missing_provenance() {
        let mut node = create_node_without_provenance();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "PROVENANCE_REQUIRED",
            message: "Missing 'provenance' in standard_properties".into(),
        };

        let fixer = ProvenanceFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert!(!changes.is_empty());

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("provenance"));

                let provenance_prop = &props["provenance"];
                assert_eq!(provenance_prop.prop_type, "json");
                assert_eq!(provenance_prop.required, Some(true));
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_provenance_present() {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Key".to_string()),
                extra: BTreeMap::new(),
            },
        );
        props.insert(
            "provenance".to_string(),
            PropertyDef {
                prop_type: "json".to_string(),
                required: Some(true),
                description: Some("Provenance".to_string()),
                extra: BTreeMap::new(),
            },
        );

        let mut node = ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                knowledge_tier: None,
                icon: None,
                description: "Test".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/test.yaml"),
        };

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "PROVENANCE_REQUIRED",
            message: "Test".into(),
        };

        let fixer = ProvenanceFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already has"));
            }
            _ => panic!("Expected Skipped for existing provenance"),
        }
    }

    #[test]
    fn test_skips_provenance_json_validation() {
        let mut node = create_node_without_provenance();

        // Add provenance property for this test
        if let Some(ref mut props) = node.def.standard_properties {
            props.insert(
                "provenance".to_string(),
                PropertyDef {
                    prop_type: "json".to_string(),
                    required: Some(true),
                    description: Some("Invalid provenance".to_string()),
                    extra: BTreeMap::new(),
                },
            );
        }

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "PROVENANCE_JSON",
            message: "provenance must be valid JSON with 'source' field".into(),
        };

        let fixer = ProvenanceFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("manual review"));
            }
            _ => panic!("Expected Skipped for PROVENANCE_JSON rule"),
        }
    }

    #[test]
    fn test_can_fix_correct_rules() {
        let fixer = ProvenanceFixer;

        let issue1 = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "PROVENANCE_REQUIRED",
            message: "Test".into(),
        };
        assert!(fixer.can_fix(&issue1));

        let issue2 = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "PROVENANCE_JSON",
            message: "Test".into(),
        };
        assert!(fixer.can_fix(&issue2));
    }

    #[test]
    fn test_does_not_fix_wrong_rule() {
        let fixer = ProvenanceFixer;

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "CONTENT_REQUIRED",
            message: "Test".into(),
        };

        assert!(!fixer.can_fix(&issue));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Generate random node names for testing
    fn prop_node_names() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "Entity".to_string(),
            "Page".to_string(),
            "Block".to_string(),
            "Locale".to_string(),
            "Project".to_string(),
        ])
    }

    /// Create node with given name and no provenance property
    fn create_node_with_name(name: String) -> ParsedNode {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Test key".to_string()),
                extra: BTreeMap::new(),
            },
        );

        ParsedNode {
            def: NodeDef {
                name,
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                knowledge_tier: None,
                icon: None,
                description: "Test node".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/test.yaml"),
        }
    }

    proptest! {
        /// Property: Fixer always adds provenance property
        #[test]
        fn prop_always_adds_provenance(name in prop_node_names()) {
            let mut node = create_node_with_name(name);

            let issue = SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "PROVENANCE_REQUIRED",
                message: "Test".into(),
            };

            let fixer = ProvenanceFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify provenance property exists
            let props = node.def.standard_properties.as_ref().unwrap();
            prop_assert!(props.contains_key("provenance"));
            prop_assert_eq!(&props["provenance"].prop_type, "json");
            prop_assert_eq!(props["provenance"].required, Some(true));
        }

        /// Property: Applying fix twice is idempotent
        #[test]
        fn prop_idempotent(name in prop_node_names()) {
            let mut node1 = create_node_with_name(name.clone());
            let mut node2 = create_node_with_name(name);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "PROVENANCE_REQUIRED",
                message: "Test".into(),
            };

            let fixer = ProvenanceFixer;

            // Apply once to node1
            let _ = fixer.fix(&mut node1, &issue).unwrap();

            // Apply twice to node2
            let _ = fixer.fix(&mut node2, &issue).unwrap();
            let result2 = fixer.fix(&mut node2, &issue).unwrap();

            // Second application should be skipped
            match result2 {
                FixAction::Skipped { .. } => {},
                _ => prop_assert!(false, "Expected Skipped on second application"),
            }

            // Both should have provenance property
            let props1 = node1.def.standard_properties.as_ref().unwrap();
            let props2 = node2.def.standard_properties.as_ref().unwrap();
            prop_assert!(props1.contains_key("provenance"));
            prop_assert!(props2.contains_key("provenance"));
        }

        /// Property: Fix preserves node identity (name, realm, layer)
        #[test]
        fn prop_preserves_node_identity(name in prop_node_names()) {
            let mut node = create_node_with_name(name);

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "PROVENANCE_REQUIRED",
                message: "Test".into(),
            };

            let fixer = ProvenanceFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
        }
    }
}
