//! Auto-fixer for property ordering violations.
//!
//! Handles PROP_ORDER rule violations by reordering standard_properties
//! to match the canonical order defined in schema_rules.rs.

use super::{AutoFix, Change, FixAction};
use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::ParsedNode;
use indexmap::IndexMap;

// ─────────────────────────────────────────────────────────────────────────────
// PropertyOrderFixer Implementation (GREEN Phase)
// ─────────────────────────────────────────────────────────────────────────────

/// Canonical order for standard properties.
const STANDARD_PROPS_ORDER: &[&str] = &[
    "key",
    "entity_key",
    "page_key",
    "block_key",
    "locale_key",
    "display_name",
    "description",
    "created_at",
    "updated_at",
];

/// Auto-fixer for property ordering violations.
///
/// Reorders standard_properties to match the canonical order defined in STANDARD_PROPS_ORDER.
pub struct PropertyOrderFixer;

impl AutoFix for PropertyOrderFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "PROP_ORDER"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        // Get current property keys (clone to owned Strings to avoid borrowing props)
        let current_keys: Vec<String> = props.keys().map(|k| k.to_string()).collect();
        let current_keys_str: Vec<&str> = current_keys.iter().map(|s| s.as_str()).collect();

        // Build expected order (only keys that exist in current properties)
        let expected_order: Vec<&str> = STANDARD_PROPS_ORDER
            .iter()
            .filter(|k| current_keys_str.contains(k))
            .copied()
            .collect();

        // Check if already in correct order
        if current_keys_str == expected_order {
            return Ok(FixAction::Skipped {
                reason: "Properties already in correct order".to_string(),
            });
        }

        // Reorder properties
        let mut reordered = IndexMap::new();
        for &key in &expected_order {
            if let Some(prop_def) = props.shift_remove(key) {
                reordered.insert(key.to_string(), prop_def);
            }
        }

        // Add any remaining properties that aren't in STANDARD_PROPS_ORDER (preserve them at the end)
        for (key, prop_def) in props.drain(..) {
            reordered.insert(key, prop_def);
        }

        // Replace with reordered properties
        *props = reordered;

        Ok(FixAction::Modified {
            changes: vec![Change {
                field: "standard_properties".to_string(),
                old_value: None,
                new_value: serde_yaml::Value::String(format!(
                    "Reordered from {:?} to {:?}",
                    current_keys_str, expected_order
                )),
            }],
        })
    }

    fn description(&self) -> &str {
        "Reorders standard properties to match canonical order"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (GREEN Phase: Tests should now PASS with implementation above)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;
    use crate::parsers::yaml_node::{NodeDef, NodeTrait, ParsedNode, PropertyDef};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create a node with properties in wrong order.
    /// Current: description, key, display_name, updated_at, created_at
    /// Expected: key, display_name, description, created_at, updated_at
    fn create_node_with_wrong_order() -> ParsedNode {
        let mut props = IndexMap::new();

        // Intentionally wrong order
        props.insert(
            "description".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Test description".to_string()),
                extra: BTreeMap::new(),
            },
        );

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
                description: Some("Test display name".to_string()),
                extra: BTreeMap::new(),
            },
        );

        props.insert(
            "updated_at".to_string(),
            PropertyDef {
                prop_type: "datetime".to_string(),
                required: Some(true),
                description: Some("Last update time".to_string()),
                extra: BTreeMap::new(),
            },
        );

        props.insert(
            "created_at".to_string(),
            PropertyDef {
                prop_type: "datetime".to_string(),
                required: Some(true),
                description: Some("Creation time".to_string()),
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
                description: "Test node with wrong property order".to_string(),
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
    fn test_reorders_standard_properties() {
        let mut node = create_node_with_wrong_order();

        // Verify wrong order before fix
        let keys_before: Vec<&str> = node
            .def
            .standard_properties
            .as_ref()
            .unwrap()
            .keys()
            .map(|k| k.as_str())
            .collect();
        assert_eq!(
            keys_before,
            vec![
                "description",
                "key",
                "display_name",
                "updated_at",
                "created_at"
            ]
        );

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "PROP_ORDER",
            message: "Standard properties out of order".into(),
        };

        let fixer = PropertyOrderFixer; // ← This doesn't exist yet (RED)
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes: _ } => {
                // Verify correct order after fix
                let keys_after: Vec<&str> = node
                    .def
                    .standard_properties
                    .as_ref()
                    .unwrap()
                    .keys()
                    .map(|k| k.as_str())
                    .collect();

                assert_eq!(
                    keys_after,
                    vec![
                        "key",
                        "display_name",
                        "description",
                        "created_at",
                        "updated_at"
                    ]
                );
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_already_ordered() {
        // Create node with correct order
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
            "display_name".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Name".to_string()),
                extra: BTreeMap::new(),
            },
        );

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
            severity: IssueSeverity::Warning,
            rule: "PROP_ORDER",
            message: "Test".into(),
        };

        let fixer = PropertyOrderFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already in correct order"));
            }
            _ => panic!("Expected Skipped for already-ordered properties"),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (REFACTOR Phase)
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Generate a random permutation of standard property names
    fn prop_names_permutation() -> impl Strategy<Value = Vec<String>> {
        prop::collection::vec(
            prop::sample::select(vec![
                "key".to_string(),
                "display_name".to_string(),
                "description".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ]),
            5..=5, // Exactly 5 properties
        )
        .prop_filter("All properties must be unique", |names| {
            let unique: std::collections::HashSet<_> = names.iter().collect();
            unique.len() == 5
        })
    }

    /// Create a node with properties in the given order
    fn create_node_with_order(order: Vec<String>) -> ParsedNode {
        let mut props = IndexMap::new();
        for name in order {
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
        /// Property: Fixer always produces canonical order (key, display_name, description, created_at, updated_at)
        #[test]
        fn prop_always_produces_canonical_order(order in prop_names_permutation()) {
            let mut node = create_node_with_order(order);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "PROP_ORDER",
                message: "Test".into(),
            };

            let fixer = PropertyOrderFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify canonical order after fix
            let keys_after: Vec<&str> = node
                .def
                .standard_properties
                .as_ref()
                .unwrap()
                .keys()
                .map(|k| k.as_str())
                .collect();

            prop_assert_eq!(
                keys_after,
                vec!["key", "display_name", "description", "created_at", "updated_at"]
            );
        }

        /// Property: Applying fix twice is idempotent (same result as applying once)
        #[test]
        fn prop_idempotent(order in prop_names_permutation()) {
            let mut node1 = create_node_with_order(order.clone());
            let mut node2 = create_node_with_order(order);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "PROP_ORDER",
                message: "Test".into(),
            };

            let fixer = PropertyOrderFixer;

            // Apply fix once to node1
            let _ = fixer.fix(&mut node1, &issue).unwrap();

            // Apply fix twice to node2
            let _ = fixer.fix(&mut node2, &issue).unwrap();
            let _ = fixer.fix(&mut node2, &issue).unwrap();

            // Both should have same property order
            let keys1: Vec<&str> = node1
                .def
                .standard_properties
                .as_ref()
                .unwrap()
                .keys()
                .map(|k| k.as_str())
                .collect();

            let keys2: Vec<&str> = node2
                .def
                .standard_properties
                .as_ref()
                .unwrap()
                .keys()
                .map(|k| k.as_str())
                .collect();

            prop_assert_eq!(keys1, keys2);
        }

        /// Property: Fix preserves node identity (name, realm, layer, trait)
        #[test]
        fn prop_preserves_node_identity(order in prop_names_permutation()) {
            let mut node = create_node_with_order(order);

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();
            let trait_before = node.def.node_trait;

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "PROP_ORDER",
                message: "Test".into(),
            };

            let fixer = PropertyOrderFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved after fix
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
            prop_assert_eq!(node.def.node_trait, trait_before);
        }
    }
}
