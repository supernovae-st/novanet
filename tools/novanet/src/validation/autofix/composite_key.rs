//! Auto-fixer for composite key format violations.
//!
//! Handles COMPOSITE_KEY_FORMAT rule violations for EntityNative, PageNative, BlockNative.

use super::{AutoFix, Change, FixAction};
use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::ParsedNode;
use serde_yaml::Value;

// ─────────────────────────────────────────────────────────────────────────────
// CompositeKeyFixer Implementation (GREEN Phase)
// ─────────────────────────────────────────────────────────────────────────────

/// Auto-fixer for composite key format violations.
///
/// Adds missing `pattern` property to composite key nodes (EntityNative, PageNative, BlockNative).
pub struct CompositeKeyFixer;

/// Composite key patterns for each node type.
const COMPOSITE_PATTERNS: &[(&str, &str)] = &[
    ("EntityNative", "^entity:[^@]+@[a-z]{2}-[A-Z]{2}$"),
    ("PageNative", "^page:[^@]+@[a-z]{2}-[A-Z]{2}$"),
    ("BlockNative", "^block:[^@]+@[a-z]{2}-[A-Z]{2}$"),
];

impl AutoFix for CompositeKeyFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "COMPOSITE_KEY_FORMAT"
    }

    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        // Find the pattern for this node type
        let pattern = COMPOSITE_PATTERNS
            .iter()
            .find(|(name, _)| *name == node.def.name.as_str())
            .map(|(_, p)| *p);

        let Some(pattern_str) = pattern else {
            return Ok(FixAction::Skipped {
                reason: format!("No pattern defined for node type: {}", node.def.name),
            });
        };

        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        // Get mutable reference to the key property
        let Some(key_prop) = props.get_mut("key") else {
            return Ok(FixAction::Skipped {
                reason: "Node has no 'key' property".to_string(),
            });
        };

        // Case 1: Fix invalid examples (Example[N] in message)
        if issue.message.contains("Example[") {
            let node_name = node.def.name.clone(); // Clone to avoid borrow conflict
            return self.fix_invalid_examples(node, &node_name);
        }

        // Case 2: Add missing pattern
        if key_prop.extra.contains_key("pattern") {
            return Ok(FixAction::Skipped {
                reason: "Pattern already exists".to_string(),
            });
        }

        // Add the pattern
        let old_value = key_prop.extra.get("pattern").cloned();
        key_prop.extra.insert(
            "pattern".to_string(),
            Value::String(pattern_str.to_string()),
        );

        Ok(FixAction::Modified {
            changes: vec![Change {
                field: "key.pattern".to_string(),
                old_value,
                new_value: Value::String(pattern_str.to_string()),
            }],
        })
    }

    fn description(&self) -> &str {
        "Adds missing regex pattern to composite key properties"
    }
}

impl CompositeKeyFixer {
    /// Fix invalid examples in the key property.
    ///
    /// Corrects examples that don't match the composite key format by:
    /// - Adding missing prefix ("entity:", "page:", "block:")
    /// - Adding missing locale suffix ("@en-US")
    fn fix_invalid_examples(&self, node: &mut ParsedNode, node_name: &str) -> Result<FixAction> {
        // Determine the prefix for this node type
        let prefix = match node_name {
            "EntityNative" => "entity",
            "PageNative" => "page",
            "BlockNative" => "block",
            _ => {
                return Ok(FixAction::Skipped {
                    reason: format!("No prefix defined for node type: {}", node_name),
                });
            }
        };

        // Get mutable reference to standard_properties
        let Some(ref mut props) = node.def.standard_properties else {
            return Ok(FixAction::Skipped {
                reason: "Node has no standard_properties".to_string(),
            });
        };

        // Get mutable reference to the key property
        let Some(key_prop) = props.get_mut("key") else {
            return Ok(FixAction::Skipped {
                reason: "Node has no 'key' property".to_string(),
            });
        };

        // Get mutable reference to examples
        let Some(Value::Sequence(examples)) = key_prop.extra.get_mut("examples") else {
            return Ok(FixAction::Skipped {
                reason: "No examples to fix".to_string(),
            });
        };

        let mut changes = Vec::new();
        let default_locale = "en-US"; // Default locale for composite keys

        for (idx, example) in examples.iter_mut().enumerate() {
            if let Some(s) = example.as_str() {
                let mut fixed = s.to_string();
                let old_value = s.to_string();

                // Add prefix if missing
                if !fixed.starts_with(&format!("{}:", prefix)) {
                    fixed = format!("{}:{}", prefix, fixed);
                }

                // Add locale if missing (check for "@" pattern)
                if !fixed.contains('@') {
                    fixed = format!("{}@{}", fixed, default_locale);
                }

                // Update the example if it changed
                if fixed != old_value {
                    *example = Value::String(fixed.clone());
                    changes.push(Change {
                        field: format!("key.examples[{}]", idx),
                        old_value: Some(Value::String(old_value)),
                        new_value: Value::String(fixed),
                    });
                }
            }
        }

        if changes.is_empty() {
            Ok(FixAction::Skipped {
                reason: "All examples are already valid".to_string(),
            })
        } else {
            Ok(FixAction::Modified { changes })
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (GREEN Phase: Tests should now PASS with implementation above)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::{IssueSeverity, SchemaIssue};
    // v0.17.3 (ADR-036): NodeTrait removed, provenance is per-instance
    use crate::parsers::yaml_node::{NodeDef, ParsedNode, PropertyDef};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create an EntityNative node without pattern property.
    fn create_entity_native_without_pattern() -> ParsedNode {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Test key property".to_string()),
                extra: BTreeMap::new(), // ← No pattern!
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
                description: "Test node".to_string(),
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

    /// Create an EntityNative node with invalid examples (missing prefix/locale).
    fn create_entity_native_with_bad_examples() -> ParsedNode {
        let mut extra = BTreeMap::new();
        extra.insert(
            "pattern".to_string(),
            Value::String("^entity:[^@]+@[a-z]{2}-[A-Z]{2}$".to_string()),
        );
        extra.insert(
            "examples".to_string(),
            Value::Sequence(vec![
                Value::String("qr-code-instagram".to_string()), // ← Bad: missing "entity:" and "@locale"
                Value::String("entity:pricing".to_string()),    // ← Bad: missing "@locale"
            ]),
        );

        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Test key property".to_string()),
                extra,
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
                description: "Test node".to_string(),
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

    #[test]
    fn test_adds_missing_pattern_for_entity_native() {
        let mut node = create_entity_native_without_pattern();

        let issue = SchemaIssue {
            node_name: "EntityNative".into(),
            severity: IssueSeverity::Warning,
            rule: "COMPOSITE_KEY_FORMAT",
            message:
                "Composite key node should have 'pattern' regex: ^entity:[^@]+@[a-z]{2}-[A-Z]{2}$"
                    .into(),
        };

        let fixer = CompositeKeyFixer; // ← This doesn't exist yet (RED)
        assert!(fixer.can_fix(&issue));

        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert_eq!(changes.len(), 1);
                assert_eq!(changes[0].field, "key.pattern");

                // Verify pattern was added
                let pattern = node
                    .def
                    .standard_properties
                    .as_ref()
                    .unwrap()
                    .get("key")
                    .unwrap()
                    .extra
                    .get("pattern")
                    .unwrap()
                    .as_str()
                    .unwrap();

                assert_eq!(pattern, "^entity:[^@]+@[a-z]{2}-[A-Z]{2}$");
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }

    #[test]
    fn test_adds_missing_pattern_for_page_native() {
        let mut node = create_entity_native_without_pattern();
        node.def.name = "PageNative".into();

        let issue = SchemaIssue {
            node_name: "PageNative".into(),
            severity: IssueSeverity::Warning,
            rule: "COMPOSITE_KEY_FORMAT",
            message: "Composite key node should have 'pattern' regex".into(),
        };

        let fixer = CompositeKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes: _ } => {
                let pattern = node
                    .def
                    .standard_properties
                    .as_ref()
                    .unwrap()
                    .get("key")
                    .unwrap()
                    .extra
                    .get("pattern")
                    .unwrap()
                    .as_str()
                    .unwrap();

                assert_eq!(pattern, "^page:[^@]+@[a-z]{2}-[A-Z]{2}$");
            }
            _ => panic!("Expected Modified for PageNative"),
        }
    }

    #[test]
    fn test_skips_non_composite_key_nodes() {
        let mut node = create_entity_native_without_pattern();
        node.def.name = "Entity".into(); // Not a composite key node

        let issue = SchemaIssue {
            node_name: "Entity".into(),
            severity: IssueSeverity::Warning,
            rule: "COMPOSITE_KEY_FORMAT",
            message: "Test".into(),
        };

        let fixer = CompositeKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("No pattern defined"));
            }
            _ => panic!("Expected Skipped for non-composite node"),
        }
    }

    #[test]
    fn test_fixes_invalid_composite_key_examples() {
        let mut node = create_entity_native_with_bad_examples();

        // Issue indicates invalid example
        let issue = SchemaIssue {
            node_name: "EntityNative".into(),
            severity: IssueSeverity::Error,
            rule: "COMPOSITE_KEY_FORMAT",
            message: "Example[0] 'qr-code-instagram' should match pattern ^entity:[^@]+@[a-z]{2}-[A-Z]{2}$".into(),
        };

        let fixer = CompositeKeyFixer;
        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes: _ } => {
                // Verify examples were fixed
                let examples = node
                    .def
                    .standard_properties
                    .as_ref()
                    .unwrap()
                    .get("key")
                    .unwrap()
                    .extra
                    .get("examples")
                    .unwrap()
                    .as_sequence()
                    .unwrap();

                // First example should be corrected: "qr-code-instagram" → "entity:qr-code-instagram@en-US"
                assert_eq!(
                    examples[0].as_str().unwrap(),
                    "entity:qr-code-instagram@en-US"
                );

                // Second example should be corrected: "entity:pricing" → "entity:pricing@en-US"
                assert_eq!(examples[1].as_str().unwrap(), "entity:pricing@en-US");
            }
            _ => panic!("Expected Modified for invalid examples"),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (REFACTOR Phase)
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Generate random node names that should get composite key patterns
    /// (Only nodes with composite keys: entity:{key}@{locale}, page:{key}@{locale}, block:{key}@{locale})
    fn prop_node_name() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "EntityNative".to_string(),
            "PageNative".to_string(),
            "BlockNative".to_string(),
            // NOTE: ProjectNative excluded - it's a satellite node with no key property
        ])
    }

    /// Create a node without a pattern (needs fixing)
    fn create_node_without_pattern(node_name: String) -> ParsedNode {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Composite key".to_string()),
                extra: BTreeMap::new(), // No pattern!
            },
        );

        ParsedNode {
            def: NodeDef {
                name: node_name.clone(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                // v0.17.3 (ADR-036): node_trait removed
                knowledge_tier: None,
                icon: None,
                description: format!("{} node", node_name),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from(format!("/test/{}.yaml", node_name)),
        }
    }

    proptest! {
        /// Property: Fixer always produces a valid regex pattern
        #[test]
        fn prop_produces_valid_pattern(node_name in prop_node_name()) {
            let mut node = create_node_without_pattern(node_name.clone());

            let issue = SchemaIssue {
                node_name: node_name.clone(),
                severity: IssueSeverity::Warning,
                rule: "COMPOSITE_KEY_FORMAT",
                message: "Missing pattern".into(),
            };

            let fixer = CompositeKeyFixer;
            let result = fixer.fix(&mut node, &issue).unwrap();

            match result {
                FixAction::Modified { .. } => {
                    // Verify pattern exists and is valid
                    let props = node.def.standard_properties.as_ref().unwrap();
                    let key_prop = props.get("key").unwrap();
                    let pattern = key_prop
                        .extra
                        .get("pattern")
                        .expect("pattern should exist after fix");

                    let pattern_str = pattern.as_str().expect("pattern should be a string");

                    // Verify pattern is a valid regex
                    let regex_result = regex::Regex::new(pattern_str);
                    prop_assert!(regex_result.is_ok(), "Pattern should be a valid regex: {}", pattern_str);

                    // Verify pattern matches expected format for this node type
                    let prefix = node_name.trim_end_matches("Native").to_lowercase();
                    prop_assert!(
                        pattern_str.contains(&prefix),
                        "Pattern should contain prefix '{}': {}",
                        prefix,
                        pattern_str
                    );
                }
                FixAction::Skipped { .. } => {
                    prop_assert!(false, "Should modify node without pattern");
                }
            }
        }

        /// Property: Applying fix twice is idempotent
        #[test]
        fn prop_idempotent(node_name in prop_node_name()) {
            let mut node1 = create_node_without_pattern(node_name.clone());
            let mut node2 = create_node_without_pattern(node_name.clone());

            let issue = SchemaIssue {
                node_name: node_name.clone(),
                severity: IssueSeverity::Warning,
                rule: "COMPOSITE_KEY_FORMAT",
                message: "Missing pattern".into(),
            };

            let fixer = CompositeKeyFixer;

            // Apply fix once to node1
            let _ = fixer.fix(&mut node1, &issue).unwrap();

            // Apply fix twice to node2
            let _ = fixer.fix(&mut node2, &issue).unwrap();
            let _ = fixer.fix(&mut node2, &issue).unwrap();

            // Both should have same pattern
            let pattern1 = node1
                .def
                .standard_properties
                .as_ref()
                .unwrap()
                .get("key")
                .unwrap()
                .extra
                .get("pattern")
                .unwrap()
                .as_str()
                .unwrap();

            let pattern2 = node2
                .def
                .standard_properties
                .as_ref()
                .unwrap()
                .get("key")
                .unwrap()
                .extra
                .get("pattern")
                .unwrap()
                .as_str()
                .unwrap();

            prop_assert_eq!(pattern1, pattern2);
        }

        /// Property: Fix preserves node identity (name, realm, layer)
        /// v0.17.3 (ADR-036): trait removed, provenance is per-instance
        #[test]
        fn prop_preserves_node_identity(node_name in prop_node_name()) {
            let mut node = create_node_without_pattern(node_name.clone());

            // Capture identity before fix
            let name_before = node.def.name.clone();
            let realm_before = node.realm.clone();
            let layer_before = node.layer.clone();

            let issue = SchemaIssue {
                node_name: node_name.clone(),
                severity: IssueSeverity::Warning,
                rule: "COMPOSITE_KEY_FORMAT",
                message: "Missing pattern".into(),
            };

            let fixer = CompositeKeyFixer;
            let _ = fixer.fix(&mut node, &issue).unwrap();

            // Verify identity preserved after fix
            prop_assert_eq!(&node.def.name, &name_before);
            prop_assert_eq!(&node.realm, &realm_before);
            prop_assert_eq!(&node.layer, &layer_before);
        }
    }
}
