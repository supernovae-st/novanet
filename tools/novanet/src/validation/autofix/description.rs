//! Auto-fixer for description field violations.
//!
//! Handles DESCRIPTION_REQUIRED rule violations by adding missing
//! description field to node definitions.

use super::{AutoFix, FixAction, Change};
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::ParsedNode;
use crate::Result;

// ─────────────────────────────────────────────────────────────────────────────
// DescriptionFixer Implementation (GREEN Phase)
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-fixer for description field violations.
///
/// Adds missing description field to node definitions with a generated default.
pub struct DescriptionFixer;

impl AutoFix for DescriptionFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "DESCRIPTION_REQUIRED"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Check if description is missing or empty
        if !node.def.description.is_empty() {
            return Ok(FixAction::Skipped {
                reason: "Node already has description".to_string(),
            });
        }

        // Generate a reasonable default description
        let generated_desc = format!(
            "{} node in the {} layer (realm: {})",
            node.def.name,
            node.def.layer,
            node.realm
        );

        let old_value = if node.def.description.is_empty() {
            None
        } else {
            Some(serde_yaml::Value::String(node.def.description.clone()))
        };

        // Update the description
        node.def.description = generated_desc.clone();

        Ok(FixAction::Modified {
            changes: vec![Change {
                field: "description".to_string(),
                old_value,
                new_value: serde_yaml::Value::String(generated_desc),
            }],
        })
    }

    fn description(&self) -> &str {
        "Adds missing description field to node definitions"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (RED Phase: These tests should FAIL initially)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;
    use crate::parsers::yaml_node::{ParsedNode, NodeDef, PropertyDef, NodeTrait};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create a node without description field.
    fn create_node_without_description() -> ParsedNode {
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
                node_trait: NodeTrait::Defined,
                knowledge_tier: None,
                icon: None,
                description: String::new(), // ← Empty description!
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
    fn test_adds_missing_description() {
        let mut node = create_node_without_description();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "DESCRIPTION_REQUIRED",
            message: "Node missing description".into(),
        };

        let fixer = DescriptionFixer; // ← This doesn't exist yet (RED)
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert_eq!(changes.len(), 1);
                assert_eq!(changes[0].field, "description");

                // Verify description was added
                assert!(!node.def.description.is_empty());
                assert!(node.def.description.len() > 0);
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_description_present() {
        let mut props = IndexMap::new();
        props.insert("key".to_string(), PropertyDef {
            prop_type: "string".to_string(),
            required: Some(true),
            description: Some("Key".to_string()),
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
                description: "This is a test node with description".to_string(),
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
            rule: "DESCRIPTION_REQUIRED",
            message: "Test".into(),
        };

        let fixer = DescriptionFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already has") || reason.contains("present"));
            }
            _ => panic!("Expected Skipped for existing description"),
        }
    }

    #[test]
    fn test_generates_reasonable_description() {
        let mut node = create_node_without_description();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "DESCRIPTION_REQUIRED",
            message: "Missing description".into(),
        };

        let fixer = DescriptionFixer;
        let _ = fixer.fix(&mut node, &issue).unwrap();

        // Generated description should mention the node name
        assert!(node.def.description.contains("TestNode") ||
                node.def.description.to_lowercase().contains("test node"));
    }

    #[test]
    fn test_description_includes_layer_context() {
        let mut node = create_node_without_description();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "DESCRIPTION_REQUIRED",
            message: "Missing description".into(),
        };

        let fixer = DescriptionFixer;
        let _ = fixer.fix(&mut node, &issue).unwrap();

        // Generated description should provide some context
        // (could include layer, realm, or node name)
        assert!(node.def.description.len() >= 10,
                "Description should be reasonably informative");
    }

    #[test]
    fn test_can_fix_correct_rule() {
        let fixer = DescriptionFixer;

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Warning,
            rule: "DESCRIPTION_REQUIRED",
            message: "Test".into(),
        };

        assert!(fixer.can_fix(&issue));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (REFACTOR Phase - will add after GREEN)
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

    /// Create node with given name and empty description
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
                name: name.clone(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                node_trait: NodeTrait::Defined,
                knowledge_tier: None,
                icon: None,
                description: String::new(),
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
        /// Property: Fixer always adds a non-empty description
        #[test]
        fn prop_always_adds_description(name in prop_node_names()) {
            let mut node = create_node_with_name(name);

            let issue = SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Warning,
                rule: "DESCRIPTION_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DescriptionFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify description is non-empty
            prop_assert!(!node.def.description.is_empty());
            prop_assert!(node.def.description.len() >= 5); // At least minimally descriptive
        }

        /// Property: Applying fix twice is idempotent
        #[test]
        fn prop_idempotent(name in prop_node_names()) {
            let mut node1 = create_node_with_name(name.clone());
            let mut node2 = create_node_with_name(name);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "DESCRIPTION_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DescriptionFixer;

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

            // Both should have same description
            prop_assert_eq!(&node1.def.description, &node2.def.description);
        }

        /// Property: Fix preserves node identity
        #[test]
        fn prop_preserves_node_identity(name in prop_node_names()) {
            let mut node = create_node_with_name(name.clone());

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();
            let trait_before = node.def.node_trait;

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "DESCRIPTION_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DescriptionFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
            prop_assert_eq!(node.def.node_trait, trait_before);
        }

        /// Property: Fix preserves all properties
        #[test]
        fn prop_preserves_properties(name in prop_node_names()) {
            let mut node = create_node_with_name(name);

            // Count properties before
            let props_count_before = node.def.standard_properties.as_ref().unwrap().len();

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Warning,
                rule: "DESCRIPTION_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DescriptionFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify properties unchanged
            let props_count_after = node.def.standard_properties.as_ref().unwrap().len();
            prop_assert_eq!(props_count_before, props_count_after);
        }
    }
}
