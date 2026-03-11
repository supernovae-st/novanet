//! Auto-fixer for content property violations.
//!
//! Handles CONTENT_REQUIRED rule violations by adding missing
//! content property to standard_properties.
//!
//! v0.19.0: `content` REPLACES `description` as a standard property.

use super::{AutoFix, Change, FixAction};
use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::{ParsedNode, PropertyDef};
use serde_yaml::Value;
use std::collections::BTreeMap;

// ─────────────────────────────────────────────────────────────────────────────
// ContentFixer Implementation
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-fixer for content property violations.
///
/// Adds missing `content` property to standard_properties.
/// v0.19.0: `content` replaces the old `description` property.
pub struct ContentFixer;

impl AutoFix for ContentFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "CONTENT_REQUIRED"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        // Check if content already exists
        if props.contains_key("content") {
            return Ok(FixAction::Skipped {
                reason: "Node already has 'content' property".to_string(),
            });
        }

        // Add content property
        props.insert(
            "content".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Human-readable content for this node instance".to_string()),
                extra: BTreeMap::new(),
            },
        );

        Ok(FixAction::Modified {
            changes: vec![Change {
                field: "content".to_string(),
                old_value: None,
                new_value: Value::String("string (required)".to_string()),
            }],
        })
    }

    fn description(&self) -> &str {
        "Adds missing 'content' property to standard_properties (v0.19.0)"
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

    /// Create a node without content property.
    fn create_node_without_content() -> ParsedNode {
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
                description: "Test node without content property".to_string(),
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
    fn test_adds_missing_content() {
        let mut node = create_node_without_content();

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "CONTENT_REQUIRED",
            message: "Missing 'content' in standard_properties".into(),
        };

        let fixer = ContentFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert!(!changes.is_empty());

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("content"));

                let content_prop = &props["content"];
                assert_eq!(content_prop.prop_type, "string");
                assert_eq!(content_prop.required, Some(true));
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_content_present() {
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
            "content".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Content".to_string()),
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
            rule: "CONTENT_REQUIRED",
            message: "Test".into(),
        };

        let fixer = ContentFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already has"));
            }
            _ => panic!("Expected Skipped for existing content"),
        }
    }

    #[test]
    fn test_can_fix_correct_rule() {
        let fixer = ContentFixer;

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "CONTENT_REQUIRED",
            message: "Test".into(),
        };

        assert!(fixer.can_fix(&issue));
    }

    #[test]
    fn test_does_not_fix_wrong_rule() {
        let fixer = ContentFixer;

        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "DESCRIPTION_REQUIRED",
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

    /// Create node with given name and no content property
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
        /// Property: Fixer always adds content property
        #[test]
        fn prop_always_adds_content(name in prop_node_names()) {
            let mut node = create_node_with_name(name);

            let issue = SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "CONTENT_REQUIRED",
                message: "Test".into(),
            };

            let fixer = ContentFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify content property exists
            let props = node.def.standard_properties.as_ref().unwrap();
            prop_assert!(props.contains_key("content"));
            prop_assert_eq!(&props["content"].prop_type, "string");
            prop_assert_eq!(props["content"].required, Some(true));
        }

        /// Property: Applying fix twice is idempotent
        #[test]
        fn prop_idempotent(name in prop_node_names()) {
            let mut node1 = create_node_with_name(name.clone());
            let mut node2 = create_node_with_name(name);

            let issue = SchemaIssue {
                node_name: "TestNode".into(),
                severity: IssueSeverity::Error,
                rule: "CONTENT_REQUIRED",
                message: "Test".into(),
            };

            let fixer = ContentFixer;

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

            // Both should have content property
            let props1 = node1.def.standard_properties.as_ref().unwrap();
            let props2 = node2.def.standard_properties.as_ref().unwrap();
            prop_assert!(props1.contains_key("content"));
            prop_assert!(props2.contains_key("content"));
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
                rule: "CONTENT_REQUIRED",
                message: "Test".into(),
            };

            let fixer = ContentFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
        }
    }
}
