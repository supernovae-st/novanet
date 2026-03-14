//! Auto-fixer for example data violations.
//!
//! Handles EXAMPLE_DATA rule violations by adding minimal example
//! data to node definitions that lack it.

use super::{AutoFix, Change, FixAction};
use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::ParsedNode;

// ─────────────────────────────────────────────────────────────────────────────
// ExampleDataFixer Implementation (GREEN Phase)
// ─────────────────────────────────────────────────────────────────────────────

use serde_yaml::{Mapping, Value};

/// Auto-fixer for example data violations.
///
/// Adds minimal example data to node definitions that lack it.
pub struct ExampleDataFixer;

impl AutoFix for ExampleDataFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "EXAMPLE_DATA"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Check if example already exists
        if node.def.example.is_some() {
            return Ok(FixAction::Skipped {
                reason: "Node already has example data".to_string(),
            });
        }

        // Get required properties from standard_properties
        let Some(ref props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties to generate example from".to_string(),
            });
        };

        // Create example data with all required properties
        let mut example_data = Mapping::new();

        for (prop_name, prop_def) in props {
            if prop_def.required == Some(true) {
                // Generate example value based on type
                let example_value = match prop_def.prop_type.as_str() {
                    "string" => Value::String(format!("example-{}", prop_name)),
                    "integer" => Value::Number(serde_yaml::Number::from(1)),
                    "boolean" => Value::Bool(false),
                    "datetime" => Value::String("2024-01-01T00:00:00Z".to_string()),
                    _ => Value::String(format!("example-{}", prop_name)),
                };

                example_data.insert(Value::String(prop_name.clone()), example_value);
            }
        }

        // Create example mapping with data field
        let mut example_mapping = Mapping::new();
        example_mapping.insert(
            Value::String("data".to_string()),
            Value::Mapping(example_data),
        );

        // Update node
        node.def.example = Some(Value::Mapping(example_mapping.clone()));

        Ok(FixAction::Modified {
            changes: vec![Change {
                field: "example".to_string(),
                old_value: None,
                new_value: Value::Mapping(example_mapping),
            }],
        })
    }

    fn description(&self) -> &str {
        "Adds missing example data to node definitions"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (RED Phase: These tests should FAIL initially)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;
    use crate::parsers::yaml_node::{NodeDef, ParsedNode, PropertyDef};
    use indexmap::IndexMap;
    use serde_yaml::{Mapping, Value};
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create a node without example data.
    fn create_node_without_example() -> ParsedNode {
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
                name: "TestNode".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                knowledge_tier: None,
                icon: None,
                description: "Test node".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None, // ← No example!
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/test-node.yaml"),
        }
    }

    #[test]
    fn test_adds_missing_example() {
        let mut node = create_node_without_example();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "EXAMPLE_DATA",
            message: "Node missing example data".into(),
        };

        let fixer = ExampleDataFixer; // ← This doesn't exist yet (RED)
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert_eq!(changes.len(), 1);
                assert_eq!(changes[0].field, "example");

                // Verify example was added
                assert!(node.def.example.is_some());
                let example = node.def.example.as_ref().unwrap();

                // Should have data field with key property
                if let Value::Mapping(data) = example {
                    if let Some(Value::Mapping(example_data)) = data.get("data") {
                        assert!(example_data.contains_key(Value::String("key".to_string())));
                    } else {
                        panic!("Example should have 'data' field");
                    }
                } else {
                    panic!("Example should be a mapping");
                }
            },
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_example_present() {
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

        // Create example as serde_yaml::Value with data field
        let mut example_data = Mapping::new();
        example_data.insert(
            Value::String("key".to_string()),
            Value::String("existing-key".to_string()),
        );

        let mut example_mapping = Mapping::new();
        example_mapping.insert(
            Value::String("data".to_string()),
            Value::Mapping(example_data),
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
                example: Some(Value::Mapping(example_mapping)),
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/test.yaml"),
        };

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "EXAMPLE_DATA",
            message: "Test".into(),
        };

        let fixer = ExampleDataFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already has") || reason.contains("present"));
            },
            _ => panic!("Expected Skipped for existing example"),
        }
    }

    #[test]
    fn test_example_includes_required_properties() {
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
        };

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "EXAMPLE_DATA",
            message: "Missing example".into(),
        };

        let fixer = ExampleDataFixer;
        let _ = fixer.fix(&mut node, &issue).unwrap();

        // Verify example includes all required properties
        let example = node.def.example.as_ref().unwrap();
        if let Value::Mapping(map) = example {
            if let Some(Value::Mapping(data)) = map.get("data") {
                assert!(data.contains_key(Value::String("key".to_string())));
                assert!(data.contains_key(Value::String("display_name".to_string())));
            } else {
                panic!("Example should have 'data' field");
            }
        } else {
            panic!("Example should be a mapping");
        }
    }

    #[test]
    fn test_can_fix_correct_rule() {
        let fixer = ExampleDataFixer;

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "EXAMPLE_DATA",
            message: "Test".into(),
        };

        assert!(fixer.can_fix(&issue));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (REFACTOR Phase - will add after GREEN)
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Generate random property counts for testing
    fn prop_property_count() -> impl Strategy<Value = usize> {
        1..=5_usize
    }

    /// Create node with N required properties and no example
    fn create_node_with_properties(count: usize) -> ParsedNode {
        let mut props = IndexMap::new();

        // Always include key
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Key".to_string()),
                extra: BTreeMap::new(),
            },
        );

        // Add additional required properties
        for i in 1..count {
            props.insert(
                format!("prop{}", i),
                PropertyDef {
                    prop_type: "string".to_string(),
                    required: Some(true),
                    description: Some(format!("Property {}", i)),
                    extra: BTreeMap::new(),
                },
            );
        }

        ParsedNode {
            def: NodeDef {
                name: "TestNode".to_string(),
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
        /// Property: Fixer always adds example with all required properties
        #[test]
        fn prop_always_adds_example_with_required_props(count in prop_property_count()) {
            let mut node = create_node_with_properties(count);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "EXAMPLE_DATA",
                message: "Test".into(),
            };

            let fixer = ExampleDataFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify example exists
            prop_assert!(node.def.example.is_some());

            let example = node.def.example.as_ref().unwrap();

            // Verify all required properties are in example data
            let required_props: Vec<_> = node.def.standard_properties
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(_, def)| def.required == Some(true))
                .map(|(name, _)| name.clone())
                .collect();

            if let Value::Mapping(map) = example {
                if let Some(Value::Mapping(data)) = map.get("data") {
                    for prop_name in required_props {
                        prop_assert!(
                            data.contains_key(Value::String(prop_name.clone())),
                            "Example should contain required property: {}", prop_name
                        );
                    }
                } else {
                    prop_assert!(false, "Example should have 'data' field");
                }
            } else {
                prop_assert!(false, "Example should be a mapping");
            }
        }

        /// Property: Applying fix twice is idempotent
        #[test]
        fn prop_idempotent(count in prop_property_count()) {
            let mut node1 = create_node_with_properties(count);
            let mut node2 = create_node_with_properties(count);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "EXAMPLE_DATA",
                message: "Test".into(),
            };

            let fixer = ExampleDataFixer;

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

            // Both should have same example
            prop_assert_eq!(
                node1.def.example.as_ref().unwrap(),
                node2.def.example.as_ref().unwrap()
            );
        }

        /// Property: Fix preserves node identity (name, realm, layer)
        #[test]
        fn prop_preserves_node_identity(count in prop_property_count()) {
            let mut node = create_node_with_properties(count);

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "EXAMPLE_DATA",
                message: "Test".into(),
            };

            let fixer = ExampleDataFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
        }

        /// Property: Fix preserves all properties
        #[test]
        fn prop_preserves_properties(count in prop_property_count()) {
            let mut node = create_node_with_properties(count);

            // Count properties before
            let props_count_before = node.def.standard_properties.as_ref().unwrap().len();

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "EXAMPLE_DATA",
                message: "Test".into(),
            };

            let fixer = ExampleDataFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify properties unchanged
            let props_count_after = node.def.standard_properties.as_ref().unwrap().len();
            prop_assert_eq!(props_count_before, props_count_after);
        }
    }
}
