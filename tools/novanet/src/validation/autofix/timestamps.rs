//! Auto-fixer for timestamp property violations.
//!
//! Handles TIMESTAMP_REQUIRED rule violations by adding missing
//! created_at and updated_at properties to standard_properties.

use super::{AutoFix, FixAction, Change};
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::{ParsedNode, PropertyDef};
use crate::Result;
use std::collections::BTreeMap;
use serde_yaml::Value;

// ─────────────────────────────────────────────────────────────────────────────
// TimestampFixer Implementation (GREEN Phase)
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-fixer for timestamp property violations.
///
/// Adds missing created_at and updated_at properties to standard_properties.
pub struct TimestampFixer;

impl AutoFix for TimestampFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "TIMESTAMP_REQUIRED"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        let mut changes = Vec::new();

        // Add created_at if missing
        if !props.contains_key("created_at") {
            props.insert(
                "created_at".to_string(),
                PropertyDef {
                    prop_type: "datetime".to_string(),
                    required: Some(true),
                    description: Some("Timestamp when the node was created".to_string()),
                    extra: BTreeMap::new(),
                },
            );

            changes.push(Change {
                field: "created_at".to_string(),
                old_value: None,
                new_value: Value::String("datetime (required)".to_string()),
            });
        }

        // Add updated_at if missing
        if !props.contains_key("updated_at") {
            props.insert(
                "updated_at".to_string(),
                PropertyDef {
                    prop_type: "datetime".to_string(),
                    required: Some(true),
                    description: Some("Timestamp when the node was last updated".to_string()),
                    extra: BTreeMap::new(),
                },
            );

            changes.push(Change {
                field: "updated_at".to_string(),
                old_value: None,
                new_value: Value::String("datetime (required)".to_string()),
            });
        }

        if changes.is_empty() {
            Ok(FixAction::Skipped {
                reason: "Timestamps already present".to_string(),
            })
        } else {
            Ok(FixAction::Modified { changes })
        }
    }

    fn description(&self) -> &str {
        "Adds missing created_at/updated_at timestamps"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (GREEN Phase: Tests should now PASS with implementation above)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;
    use crate::parsers::yaml_node::{ParsedNode, NodeDef, PropertyDef, NodeTrait};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create a node without timestamp properties.
    fn create_node_without_timestamps() -> ParsedNode {
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
                description: Some("Test name".to_string()),
                extra: BTreeMap::new(),
            },
        );

        ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                node_trait: NodeTrait::Defined,
                knowledge_tier: None,
                icon: None,
                description: "Test node without timestamps".to_string(),
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
    fn test_adds_missing_timestamps() {
        let mut node = create_node_without_timestamps();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "TIMESTAMP_REQUIRED",
            message: "Missing 'created_at' in standard_properties".into(),
        };

        let fixer = TimestampFixer; // ← This doesn't exist yet (RED)
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert!(changes.len() >= 1);

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("created_at"));
                assert!(props.contains_key("updated_at"));

                let created_at = &props["created_at"];
                assert_eq!(created_at.prop_type, "datetime");
                assert_eq!(created_at.required, Some(true));

                let updated_at = &props["updated_at"];
                assert_eq!(updated_at.prop_type, "datetime");
                assert_eq!(updated_at.required, Some(true));
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_timestamps_present() {
        let mut props = IndexMap::new();
        props.insert("key".to_string(), PropertyDef {
            prop_type: "string".to_string(),
            required: Some(true),
            description: Some("Key".to_string()),
            extra: BTreeMap::new(),
        });
        props.insert("created_at".to_string(), PropertyDef {
            prop_type: "datetime".to_string(),
            required: Some(true),
            description: Some("Created".to_string()),
            extra: BTreeMap::new(),
        });
        props.insert("updated_at".to_string(), PropertyDef {
            prop_type: "datetime".to_string(),
            required: Some(true),
            description: Some("Updated".to_string()),
            extra: BTreeMap::new(),
        });

        let mut node = ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                node_trait: NodeTrait::Defined,
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
            rule: "TIMESTAMP_REQUIRED",
            message: "Test".into(),
        };

        let fixer = TimestampFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already present"));
            }
            _ => panic!("Expected Skipped for existing timestamps"),
        }
    }

    #[test]
    fn test_adds_only_missing_timestamp() {
        let mut props = IndexMap::new();
        props.insert("key".to_string(), PropertyDef {
            prop_type: "string".to_string(),
            required: Some(true),
            description: Some("Key".to_string()),
            extra: BTreeMap::new(),
        });
        // Only created_at, missing updated_at
        props.insert("created_at".to_string(), PropertyDef {
            prop_type: "datetime".to_string(),
            required: Some(true),
            description: Some("Created".to_string()),
            extra: BTreeMap::new(),
        });

        let mut node = ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                node_trait: NodeTrait::Defined,
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
            rule: "TIMESTAMP_REQUIRED",
            message: "Missing 'updated_at'".into(),
        };

        let fixer = TimestampFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert_eq!(changes.len(), 1); // Only updated_at added

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("created_at")); // Still present
                assert!(props.contains_key("updated_at"));  // Now added

                let updated_at = &props["updated_at"];
                assert_eq!(updated_at.prop_type, "datetime");
                assert_eq!(updated_at.required, Some(true));
            }
            _ => panic!("Expected Modified"),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (REFACTOR Phase)
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Generate a random set of property names (excluding timestamps)
    fn prop_property_names() -> impl Strategy<Value = Vec<String>> {
        prop::collection::vec(
            prop::sample::select(vec![
                "key".to_string(),
                "display_name".to_string(),
                "description".to_string(),
                "status".to_string(),
                "version".to_string(),
            ]),
            1..=5,
        )
    }

    /// Create a node with given properties (no timestamps)
    fn create_node_with_properties(prop_names: Vec<String>) -> ParsedNode {
        let mut props = IndexMap::new();
        for name in prop_names {
            props.insert(
                name.clone(),
                PropertyDef {
                    prop_type: "string".to_string(),
                    required: Some(true),
                    description: Some(format!("{} property", name)),
                    extra: BTreeMap::new(),
                },
            );
        }

        ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                node_trait: NodeTrait::Defined,
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
        /// Property: Fixer always adds both timestamps
        #[test]
        fn prop_always_adds_both_timestamps(prop_names in prop_property_names()) {
            let mut node = create_node_with_properties(prop_names);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Test".into(),
            };

            let fixer = TimestampFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify both timestamps present
            let props = node.def.standard_properties.as_ref().unwrap();
            prop_assert!(props.contains_key("created_at"));
            prop_assert!(props.contains_key("updated_at"));

            // Verify correct type
            prop_assert_eq!(&props["created_at"].prop_type, "datetime");
            prop_assert_eq!(&props["updated_at"].prop_type, "datetime");

            // Verify required
            prop_assert_eq!(props["created_at"].required, Some(true));
            prop_assert_eq!(props["updated_at"].required, Some(true));
        }

        /// Property: Applying fix twice is idempotent (same result as applying once)
        #[test]
        fn prop_idempotent(prop_names in prop_property_names()) {
            let mut node1 = create_node_with_properties(prop_names.clone());
            let mut node2 = create_node_with_properties(prop_names);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Test".into(),
            };

            let fixer = TimestampFixer;

            // Apply once to node1
            let _ = fixer.fix(&mut node1, &issue).unwrap();

            // Apply twice to node2
            let _ = fixer.fix(&mut node2, &issue).unwrap();
            let _ = fixer.fix(&mut node2, &issue).unwrap();

            // Both should have same properties
            let props1 = node1.def.standard_properties.as_ref().unwrap();
            let props2 = node2.def.standard_properties.as_ref().unwrap();

            prop_assert_eq!(props1.len(), props2.len());
            prop_assert!(props1.contains_key("created_at"));
            prop_assert!(props1.contains_key("updated_at"));
            prop_assert!(props2.contains_key("created_at"));
            prop_assert!(props2.contains_key("updated_at"));
        }

        /// Property: Fix preserves node identity (name, realm, layer, trait)
        #[test]
        fn prop_preserves_node_identity(prop_names in prop_property_names()) {
            let mut node = create_node_with_properties(prop_names);

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();
            let trait_before = node.def.node_trait;

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Test".into(),
            };

            let fixer = TimestampFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved after fix
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
            prop_assert_eq!(node.def.node_trait, trait_before);
        }

        /// Property: Fix preserves all existing properties
        #[test]
        fn prop_preserves_existing_properties(prop_names in prop_property_names()) {
            let mut node = create_node_with_properties(prop_names.clone());

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Test".into(),
            };

            let fixer = TimestampFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify all original properties still exist
            let props = node.def.standard_properties.as_ref().unwrap();
            for prop_name in &prop_names {
                prop_assert!(
                    props.contains_key(prop_name),
                    "Expected property {} to be preserved", prop_name
                );
            }
        }
    }
}
