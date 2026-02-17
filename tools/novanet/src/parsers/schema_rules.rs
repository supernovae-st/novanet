//! Schema validation rules for v0.13.1 standardization.
//!
//! Validates:
//! - Standard properties presence (key, created_at, updated_at)
//! - Composite key denormalization (entity_key, page_key, block_key, locale_key)
//! - Property ordering (key → *_key → display_name → ...)
//!
//! # Usage
//!
//! ```ignore
//! let nodes = load_all_nodes(&root)?;
//! let issues = schema_rules::validate_all_nodes(&nodes);
//!
//! for issue in issues {
//!     if issue.severity == IssueSeverity::Error {
//!         eprintln!("ERROR: {} - {}", issue.node_name, issue.message);
//!     }
//! }
//! ```

use crate::parsers::yaml_node::ParsedNode;

/// Validation issue for schema standardization.
#[derive(Debug, Clone)]
pub struct SchemaIssue {
    /// Node name where the issue was found.
    pub node_name: String,
    /// Severity of the issue.
    pub severity: IssueSeverity,
    /// Rule code (e.g., "KEY_REQUIRED", "DENORM_REQUIRED").
    pub rule: &'static str,
    /// Human-readable description of the issue.
    pub message: String,
}

/// Severity of a schema validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    /// Blocking issue - must be fixed.
    Error,
    /// Non-blocking issue - should be fixed.
    Warning,
}

/// Nodes that intentionally don't have a `key` property.
/// These are identified by relation chains instead.
const KEYLESS_NODES: &[&str] = &[
    "ProjectNative",
    "BlockRules",
    "TermSet",
    "ExpressionSet",
    "PatternSet",
    "CultureSet",
    "TabooSet",
    "AudienceSet",
];

/// Nodes with composite keys that MUST have denormalized properties.
/// Format: (NodeName, prefix, parent_key_name, &[required_denormalized_properties])
const COMPOSITE_KEY_NODES: &[(&str, &str, &str, &[&str])] = &[
    ("EntityNative", "entity", "entity_key", &["entity_key", "locale_key"]),
    ("PageNative", "page", "page_key", &["page_key", "locale_key"]),
    ("BlockNative", "block", "block_key", &["block_key", "locale_key"]),
];

/// Expected order for standard properties.
/// Properties present in the node must appear in this order (others can be interspersed).
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

/// Validate a single node against schema rules.
pub fn validate_node(node: &ParsedNode) -> Vec<SchemaIssue> {
    let mut issues = Vec::new();

    // Rule 1: Check for key property (unless in KEYLESS_NODES)
    if !KEYLESS_NODES.contains(&node.def.name.as_str()) {
        if let Some(ref sp) = node.def.standard_properties {
            if !sp.contains_key("key") {
                issues.push(SchemaIssue {
                    node_name: node.def.name.clone(),
                    severity: IssueSeverity::Error,
                    rule: "KEY_REQUIRED",
                    message: "Missing 'key' property in standard_properties".into(),
                });
            }
        } else {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "STANDARD_PROPS_REQUIRED",
                message: "Missing standard_properties section".into(),
            });
        }
    }

    // Rule 2: Composite key nodes must have denormalized properties
    for (composite_node, _prefix, _parent_key_name, required_props) in COMPOSITE_KEY_NODES {
        if node.def.name == *composite_node {
            if let Some(ref sp) = node.def.standard_properties {
                for prop in *required_props {
                    if !sp.contains_key(*prop) {
                        issues.push(SchemaIssue {
                            node_name: node.def.name.clone(),
                            severity: IssueSeverity::Error,
                            rule: "DENORM_REQUIRED",
                            message: format!(
                                "Composite key node missing denormalized property: {}",
                                prop
                            ),
                        });
                    }
                }
            }
        }
    }

    // Rule 3: Check timestamps (created_at, updated_at)
    if let Some(ref sp) = node.def.standard_properties {
        if !sp.contains_key("created_at") {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Missing 'created_at' in standard_properties".into(),
            });
        }
        if !sp.contains_key("updated_at") {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Missing 'updated_at' in standard_properties".into(),
            });
        }
    }

    // Rule 4: Check property order (warning only)
    if let Some(ref sp) = node.def.standard_properties {
        let actual_order: Vec<&str> = sp.keys().map(|k| k.as_str()).collect();

        // Filter to only properties that exist in STANDARD_PROPS_ORDER
        let expected_in_actual: Vec<&str> = STANDARD_PROPS_ORDER
            .iter()
            .filter(|p| actual_order.contains(p))
            .copied()
            .collect();

        let actual_filtered: Vec<&str> = actual_order
            .iter()
            .filter(|p| STANDARD_PROPS_ORDER.contains(p))
            .copied()
            .collect();

        if expected_in_actual != actual_filtered {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Warning,
                rule: "PROP_ORDER",
                message: format!(
                    "Standard properties out of order. Expected: {:?}, Got: {:?}",
                    expected_in_actual, actual_filtered
                ),
            });
        }
    }

    // Rule 5: Composite key format validation
    // Validates that composite key nodes follow the pattern: {type}:{parent_key}@{locale}
    // and that denormalized properties match their respective parts
    for (composite_node, prefix, parent_key_name, _required_props) in COMPOSITE_KEY_NODES {
        if node.def.name == *composite_node {
            if let Some(ref sp) = node.def.standard_properties {
                // Get the key property
                if let Some(key_prop) = sp.get("key") {
                    if let Some(key_pattern) = key_prop.extra.get("pattern").and_then(|v| v.as_str()) {
                        // Expected pattern: "^{prefix}:[^@]+@[a-z]{2}-[A-Z]{2}$"
                        let expected_prefix = format!("{}:", prefix);

                        // Check if pattern starts with correct prefix
                        if !key_pattern.starts_with(&format!("^{}", expected_prefix)) {
                            issues.push(SchemaIssue {
                                node_name: node.def.name.clone(),
                                severity: IssueSeverity::Error,
                                rule: "COMPOSITE_KEY_FORMAT",
                                message: format!(
                                    "Composite key pattern should start with '{}'. Found: {}",
                                    expected_prefix, key_pattern
                                ),
                            });
                        }

                        // Check if pattern includes @ separator
                        if !key_pattern.contains('@') {
                            issues.push(SchemaIssue {
                                node_name: node.def.name.clone(),
                                severity: IssueSeverity::Error,
                                rule: "COMPOSITE_KEY_FORMAT",
                                message: format!(
                                    "Composite key pattern must include '@' separator. Found: {}",
                                    key_pattern
                                ),
                            });
                        }
                    } else {
                        issues.push(SchemaIssue {
                            node_name: node.def.name.clone(),
                            severity: IssueSeverity::Warning,
                            rule: "COMPOSITE_KEY_FORMAT",
                            message: format!(
                                "Composite key node should have 'pattern' regex: ^{}:[^@]+@[a-z]{{2}}-[A-Z]{{2}}$",
                                prefix
                            ),
                        });
                    }

                    // Check examples if present
                    if let Some(examples) = key_prop.extra.get("examples").and_then(|v| v.as_sequence()) {
                        for (idx, example) in examples.iter().enumerate() {
                            if let Some(example_str) = example.as_str() {
                                // Validate format: {prefix}:{key}@{locale}
                                if !example_str.starts_with(&format!("{}:", prefix)) {
                                    issues.push(SchemaIssue {
                                        node_name: node.def.name.clone(),
                                        severity: IssueSeverity::Error,
                                        rule: "COMPOSITE_KEY_FORMAT",
                                        message: format!(
                                            "Example[{}] '{}' should start with '{}:'",
                                            idx, example_str, prefix
                                        ),
                                    });
                                }

                                if !example_str.contains('@') {
                                    issues.push(SchemaIssue {
                                        node_name: node.def.name.clone(),
                                        severity: IssueSeverity::Error,
                                        rule: "COMPOSITE_KEY_FORMAT",
                                        message: format!(
                                            "Example[{}] '{}' must include '@' separator",
                                            idx, example_str
                                        ),
                                    });
                                }

                                // Validate that parts match denormalized properties if available
                                if let Some(at_pos) = example_str.rfind('@') {
                                    let before_at = &example_str[..at_pos];
                                    let locale_part = &example_str[at_pos + 1..];

                                    // Extract parent key part (everything after "prefix:")
                                    if let Some(parent_part) = before_at.strip_prefix(&format!("{}:", prefix)) {
                                        // Check if parent_key matches (if present in example description)
                                        if let Some(parent_key_prop) = sp.get(*parent_key_name) {
                                            if let Some(parent_examples) = parent_key_prop.extra
                                                .get("examples")
                                                .and_then(|v| v.as_sequence())
                                            {
                                                // At least one parent example should match
                                                let any_match = parent_examples
                                                    .iter()
                                                    .any(|e| e.as_str() == Some(parent_part));

                                                if !any_match && !parent_examples.is_empty() {
                                                    issues.push(SchemaIssue {
                                                        node_name: node.def.name.clone(),
                                                        severity: IssueSeverity::Warning,
                                                        rule: "COMPOSITE_KEY_FORMAT",
                                                        message: format!(
                                                            "Example[{}] parent part '{}' doesn't match any {} examples",
                                                            idx, parent_part, parent_key_name
                                                        ),
                                                    });
                                                }
                                            }
                                        }

                                        // Check if locale part matches locale_key examples
                                        if let Some(locale_key_prop) = sp.get("locale_key") {
                                            if let Some(locale_examples) = locale_key_prop.extra
                                                .get("examples")
                                                .and_then(|v| v.as_sequence())
                                            {
                                                let any_match = locale_examples
                                                    .iter()
                                                    .any(|e| e.as_str() == Some(locale_part));

                                                if !any_match && !locale_examples.is_empty() {
                                                    issues.push(SchemaIssue {
                                                        node_name: node.def.name.clone(),
                                                        severity: IssueSeverity::Warning,
                                                        rule: "COMPOSITE_KEY_FORMAT",
                                                        message: format!(
                                                            "Example[{}] locale part '{}' doesn't match any locale_key examples",
                                                            idx, locale_part
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    issues
}

/// Validate all nodes against schema rules.
pub fn validate_all_nodes(nodes: &[ParsedNode]) -> Vec<SchemaIssue> {
    nodes.iter().flat_map(validate_node).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_root() -> Option<std::path::PathBuf> {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // tools/novanet -> tools -> novanet-hq
        let root = manifest_dir.parent().and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn all_nodes_pass_schema_rules() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let issues = validate_all_nodes(&nodes);

        // Filter to errors only
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .collect();

        if !errors.is_empty() {
            for err in &errors {
                eprintln!("ERROR [{}] {}: {}", err.rule, err.node_name, err.message);
            }
            panic!("Found {} schema errors", errors.len());
        }
    }

    #[test]
    fn composite_key_nodes_have_denormalized_props() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        for (node_name, _prefix, _parent_key_name, required_props) in COMPOSITE_KEY_NODES {
            let node = nodes.iter().find(|n| n.def.name == *node_name);
            assert!(node.is_some(), "Node {} not found", node_name);

            let node = node.unwrap();
            let sp = node
                .def
                .standard_properties
                .as_ref()
                .unwrap_or_else(|| panic!("{} should have standard_properties", node_name));

            for prop in *required_props {
                assert!(
                    sp.contains_key(*prop),
                    "{} missing denormalized property: {}",
                    node_name,
                    prop
                );
            }
        }
    }

    #[test]
    fn keyless_nodes_are_valid() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        for keyless in KEYLESS_NODES {
            let node = nodes.iter().find(|n| n.def.name == *keyless);
            if let Some(node) = node {
                // Should not error about missing key
                let issues = validate_node(node);
                let key_errors: Vec<_> = issues
                    .iter()
                    .filter(|i| i.rule == "KEY_REQUIRED")
                    .collect();
                assert!(
                    key_errors.is_empty(),
                    "{} should not error about missing key",
                    keyless
                );
            }
        }
    }
}
