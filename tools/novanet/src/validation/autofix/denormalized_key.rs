//! Auto-fixer for denormalized key property violations.
//!
//! Handles DENORM_REQUIRED rule violations by adding missing denormalized
//! properties (entity_key, page_key, block_key, locale_key) to composite key nodes.

use super::{AutoFix, Change, FixAction};
use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::{ParsedNode, PropertyDef};
use serde_yaml::Value;
use std::collections::BTreeMap;

// ─────────────────────────────────────────────────────────────────────────────
// DenormalizedKeyFixer Implementation (GREEN Phase - not yet implemented)
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-fixer for denormalized key property violations.
///
/// Adds missing denormalized properties to composite key nodes.
/// - EntityNative: entity_key, locale_key
/// - PageNative: page_key, locale_key
/// - BlockNative: block_key, locale_key
pub struct DenormalizedKeyFixer;

/// Mapping of node types to their required denormalized properties.
const DENORM_REQUIREMENTS: &[(&str, &[&str])] = &[
    ("EntityNative", &["entity_key", "locale_key"]),
    ("PageNative", &["page_key", "locale_key"]),
    ("BlockNative", &["block_key", "locale_key"]),
];

impl AutoFix for DenormalizedKeyFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "DENORM_REQUIRED"
    }

    fn fix(&self, node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
        // Find the denormalized requirements for this node type
        let requirements = DENORM_REQUIREMENTS
            .iter()
            .find(|(name, _)| *name == node.def.name.as_str())
            .map(|(_, props)| *props);

        let Some(required_props) = requirements else {
            return Ok(FixAction::Skipped {
                reason: format!(
                    "No denormalized requirements for node type: {}",
                    node.def.name
                ),
            });
        };

        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        let mut changes = Vec::new();

        // Add missing denormalized properties
        for &prop_name in required_props {
            if !props.contains_key(prop_name) {
                props.insert(
                    prop_name.to_string(),
                    PropertyDef {
                        prop_type: "string".to_string(),
                        required: Some(true),
                        description: Some(format!("Denormalized {} from composite key", prop_name)),
                        extra: BTreeMap::new(),
                    },
                );

                changes.push(Change {
                    field: prop_name.to_string(),
                    old_value: None,
                    new_value: Value::String("string (required)".to_string()),
                });
            }
        }

        if changes.is_empty() {
            Ok(FixAction::Skipped {
                reason: "Denormalized properties already present".to_string(),
            })
        } else {
            Ok(FixAction::Modified { changes })
        }
    }

    fn description(&self) -> &str {
        "Adds missing denormalized key properties to composite key nodes"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (RED Phase: Tests should FAIL until implementation added)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;
    // v0.17.3 (ADR-036): NodeTrait removed, provenance is per-instance
    use crate::parsers::yaml_node::{NodeDef, ParsedNode, PropertyDef};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create EntityNative node without denormalized keys
    fn create_entity_native_without_denorm() -> ParsedNode {
        let mut props = IndexMap::new();

        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Composite key".to_string()),
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
                name: "EntityNative".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                // v0.17.3 (ADR-036): node_trait removed
                knowledge_tier: None,
                icon: None,
                description: "EntityNative without denormalized keys".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/entity-native.yaml"),
        }
    }

    /// Create PageNative node without denormalized keys
    fn create_page_native_without_denorm() -> ParsedNode {
        let mut props = IndexMap::new();

        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Composite key".to_string()),
                extra: BTreeMap::new(),
            },
        );

        ParsedNode {
            def: NodeDef {
                name: "PageNative".to_string(),
                realm: "org".to_string(),
                layer: "output".to_string(),
                // v0.17.3 (ADR-036): node_trait removed
                knowledge_tier: None,
                icon: None,
                description: "PageNative without denormalized keys".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "output".to_string(),
            source_path: PathBuf::from("/test/page-native.yaml"),
        }
    }

    /// Create BlockNative node without denormalized keys
    fn create_block_native_without_denorm() -> ParsedNode {
        let mut props = IndexMap::new();

        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Composite key".to_string()),
                extra: BTreeMap::new(),
            },
        );

        ParsedNode {
            def: NodeDef {
                name: "BlockNative".to_string(),
                realm: "org".to_string(),
                layer: "output".to_string(),
                // v0.17.3 (ADR-036): node_trait removed
                knowledge_tier: None,
                icon: None,
                description: "BlockNative without denormalized keys".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "output".to_string(),
            source_path: PathBuf::from("/test/block-native.yaml"),
        }
    }

    #[test]
    fn test_adds_denorm_keys_to_entity_native() {
        let mut node = create_entity_native_without_denorm();

        let issue = SchemaIssue {
            node_name: "EntityNative".into(),
            severity: IssueSeverity::Error,
            rule: "DENORM_REQUIRED",
            message: "Missing 'entity_key' in standard_properties".into(),
        };

        let fixer = DenormalizedKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert!(!changes.is_empty());

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("entity_key"), "Should have entity_key");
                assert!(props.contains_key("locale_key"), "Should have locale_key");

                let entity_key = &props["entity_key"];
                assert_eq!(entity_key.prop_type, "string");
                assert_eq!(entity_key.required, Some(true));

                let locale_key = &props["locale_key"];
                assert_eq!(locale_key.prop_type, "string");
                assert_eq!(locale_key.required, Some(true));
            },
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_adds_denorm_keys_to_page_native() {
        let mut node = create_page_native_without_denorm();

        let issue = SchemaIssue {
            node_name: "PageNative".into(),
            severity: IssueSeverity::Error,
            rule: "DENORM_REQUIRED",
            message: "Missing 'page_key' in standard_properties".into(),
        };

        let fixer = DenormalizedKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert!(!changes.is_empty());

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("page_key"), "Should have page_key");
                assert!(props.contains_key("locale_key"), "Should have locale_key");

                let page_key = &props["page_key"];
                assert_eq!(page_key.prop_type, "string");
                assert_eq!(page_key.required, Some(true));

                let locale_key = &props["locale_key"];
                assert_eq!(locale_key.prop_type, "string");
                assert_eq!(locale_key.required, Some(true));
            },
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_adds_denorm_keys_to_block_native() {
        let mut node = create_block_native_without_denorm();

        let issue = SchemaIssue {
            node_name: "BlockNative".into(),
            severity: IssueSeverity::Error,
            rule: "DENORM_REQUIRED",
            message: "Missing 'block_key' in standard_properties".into(),
        };

        let fixer = DenormalizedKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert!(!changes.is_empty());

                let props = node.def.standard_properties.as_ref().unwrap();
                assert!(props.contains_key("block_key"), "Should have block_key");
                assert!(props.contains_key("locale_key"), "Should have locale_key");

                let block_key = &props["block_key"];
                assert_eq!(block_key.prop_type, "string");
                assert_eq!(block_key.required, Some(true));

                let locale_key = &props["locale_key"];
                assert_eq!(locale_key.prop_type, "string");
                assert_eq!(locale_key.required, Some(true));
            },
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_skips_if_denorm_keys_present() {
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
            "entity_key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Entity key".to_string()),
                extra: BTreeMap::new(),
            },
        );
        props.insert(
            "locale_key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Locale key".to_string()),
                extra: BTreeMap::new(),
            },
        );

        let mut node = ParsedNode {
            def: NodeDef {
                name: "EntityNative".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                // v0.17.3 (ADR-036): node_trait removed
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
            node_name: "EntityNative".into(),
            severity: IssueSeverity::Error,
            rule: "DENORM_REQUIRED",
            message: "Test".into(),
        };

        let fixer = DenormalizedKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("already present") || reason.contains("Denormalized"));
            },
            _ => panic!("Expected Skipped for existing denorm keys"),
        }
    }

    #[test]
    fn test_skips_non_composite_key_nodes() {
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

        let mut node = ParsedNode {
            def: NodeDef {
                name: "Entity".to_string(), // Not a composite key node
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                // v0.17.3 (ADR-036): node_trait removed
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
            node_name: "Entity".into(),
            severity: IssueSeverity::Error,
            rule: "DENORM_REQUIRED",
            message: "Test".into(),
        };

        let fixer = DenormalizedKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(
                    reason.contains("not a composite key node")
                        || reason.contains("No denormalized")
                );
            },
            _ => panic!("Expected Skipped for non-composite key node"),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (REFACTOR Phase)
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Generate random composite key node names
    fn prop_composite_node_name() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "EntityNative".to_string(),
            "PageNative".to_string(),
            "BlockNative".to_string(),
        ])
    }

    /// Create a node with given name (without denormalized keys)
    fn create_node_without_denorm_keys(node_name: String) -> ParsedNode {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Composite key".to_string()),
                extra: BTreeMap::new(),
            },
        );

        // v0.17.3 (ADR-036): trait removed, just need realm/layer
        let (realm, layer) = match node_name.as_str() {
            "EntityNative" => ("org", "semantic"),
            "PageNative" => ("org", "output"),
            "BlockNative" => ("org", "output"),
            _ => ("org", "semantic"),
        };

        ParsedNode {
            def: NodeDef {
                name: node_name,
                realm: realm.to_string(),
                layer: layer.to_string(),
                // v0.17.3 (ADR-036): node_trait removed
                knowledge_tier: None,
                icon: None,
                description: "Test node".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: realm.to_string(),
            layer: layer.to_string(),
            source_path: PathBuf::from("/test/test.yaml"),
        }
    }

    proptest! {
        /// Property: Fixer always adds required denormalized keys
        #[test]
        fn prop_always_adds_required_denorm_keys(node_name in prop_composite_node_name()) {
            let mut node = create_node_without_denorm_keys(node_name.clone());

            let issue = SchemaIssue {
                node_name: node_name.clone(),
                severity: IssueSeverity::Error,
                rule: "DENORM_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DenormalizedKeyFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify required denormalized keys are present
            let props = node.def.standard_properties.as_ref().unwrap();
            prop_assert!(props.contains_key("locale_key"));

            // Verify node-specific key
            match node_name.as_str() {
                "EntityNative" => prop_assert!(props.contains_key("entity_key")),
                "PageNative" => prop_assert!(props.contains_key("page_key")),
                "BlockNative" => prop_assert!(props.contains_key("block_key")),
                _ => prop_assert!(false, "Unknown node type"),
            }

            // Verify correct type
            prop_assert_eq!(&props["locale_key"].prop_type, "string");
            prop_assert_eq!(props["locale_key"].required, Some(true));
        }

        /// Property: Applying fix twice is idempotent
        #[test]
        fn prop_idempotent(node_name in prop_composite_node_name()) {
            let mut node1 = create_node_without_denorm_keys(node_name.clone());
            let mut node2 = create_node_without_denorm_keys(node_name.clone());

            let issue = SchemaIssue {
                node_name,
                severity: IssueSeverity::Error,
                rule: "DENORM_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DenormalizedKeyFixer;

            // Apply once to node1
            let _ = fixer.fix(&mut node1, &issue).unwrap();

            // Apply twice to node2
            let _ = fixer.fix(&mut node2, &issue).unwrap();
            let _ = fixer.fix(&mut node2, &issue).unwrap();

            // Both should have same properties
            let props1 = node1.def.standard_properties.as_ref().unwrap();
            let props2 = node2.def.standard_properties.as_ref().unwrap();

            prop_assert_eq!(props1.len(), props2.len());
            prop_assert!(props1.contains_key("locale_key"));
            prop_assert!(props2.contains_key("locale_key"));
        }

        /// Property: Fix preserves node identity (name, realm, layer)
        /// v0.17.3 (ADR-036): trait removed, provenance is per-instance
        #[test]
        fn prop_preserves_node_identity(node_name in prop_composite_node_name()) {
            let mut node = create_node_without_denorm_keys(node_name.clone());

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();

            let issue = SchemaIssue {
                node_name,
                severity: IssueSeverity::Error,
                rule: "DENORM_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DenormalizedKeyFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved after fix
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
        }

        /// Property: Fix preserves existing properties
        #[test]
        fn prop_preserves_existing_properties(node_name in prop_composite_node_name()) {
            let mut node = create_node_without_denorm_keys(node_name.clone());

            let issue = SchemaIssue {
                node_name,
                severity: IssueSeverity::Error,
                rule: "DENORM_REQUIRED",
                message: "Test".into(),
            };

            let fixer = DenormalizedKeyFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify original key property still exists
            let props = node.def.standard_properties.as_ref().unwrap();
            prop_assert!(props.contains_key("key"), "Expected original 'key' property to be preserved");
        }
    }
}
