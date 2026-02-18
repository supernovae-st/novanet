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

/// Valid realm values (ADR-012).
const VALID_REALMS: &[&str] = &["shared", "org"];

/// Valid layer values for the shared realm (ADR-012, v11.5).
const SHARED_LAYERS: &[&str] = &["config", "locale", "geography", "knowledge"];

/// Valid layer values for the org realm (ADR-012, v11.5).
const ORG_LAYERS: &[&str] = &["config", "foundation", "structure", "semantic", "instruction", "output"];

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

    // Rule 6: Realm must be valid (ADR-012)
    if !VALID_REALMS.contains(&node.def.realm.as_str()) {
        issues.push(SchemaIssue {
            node_name: node.def.name.clone(),
            severity: IssueSeverity::Error,
            rule: "REALM_VALID",
            message: format!(
                "Invalid realm '{}'. Must be one of: {:?}",
                node.def.realm, VALID_REALMS
            ),
        });
    }

    // Rule 7: Layer must be valid (ADR-012)
    let all_valid_layers: Vec<&str> = SHARED_LAYERS.iter().chain(ORG_LAYERS.iter()).copied().collect();
    if !all_valid_layers.contains(&node.def.layer.as_str()) {
        issues.push(SchemaIssue {
            node_name: node.def.name.clone(),
            severity: IssueSeverity::Error,
            rule: "LAYER_VALID",
            message: format!(
                "Invalid layer '{}'. Must be one of: {:?}",
                node.def.layer, all_valid_layers
            ),
        });
    }

    // Rule 8: Layer must be valid for the declared realm (ADR-012)
    let valid_layers_for_realm: &[&str] = match node.def.realm.as_str() {
        "shared" => SHARED_LAYERS,
        "org" => ORG_LAYERS,
        _ => &[], // already caught by REALM_VALID
    };
    if !valid_layers_for_realm.is_empty()
        && !valid_layers_for_realm.contains(&node.def.layer.as_str())
    {
        issues.push(SchemaIssue {
            node_name: node.def.name.clone(),
            severity: IssueSeverity::Error,
            rule: "REALM_LAYER_COHERENCE",
            message: format!(
                "Layer '{}' is not valid for realm '{}'. Valid layers: {:?}",
                node.def.layer, node.def.realm, valid_layers_for_realm
            ),
        });
    }

    // Rule 9: Icon must be present (ADR-013 — dual format: web + terminal)
    if node.def.icon.is_none() {
        issues.push(SchemaIssue {
            node_name: node.def.name.clone(),
            severity: IssueSeverity::Warning,
            rule: "ICON_REQUIRED",
            message: "Missing icon section. Expected: icon: { web: \"lucide-name\", terminal: \"◆\" }".into(),
        });
    } else if let Some(ref icon) = node.def.icon {
        if icon.web.is_empty() {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Warning,
                rule: "ICON_REQUIRED",
                message: "icon.web must not be empty (Lucide icon name required)".into(),
            });
        }
        if icon.terminal.is_empty() {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Warning,
                rule: "ICON_REQUIRED",
                message: "icon.terminal must not be empty (Unicode symbol required)".into(),
            });
        }
    }

    // Rule 10: display_name must be in standard_properties for non-keyless nodes
    if !KEYLESS_NODES.contains(&node.def.name.as_str()) {
        if let Some(ref sp) = node.def.standard_properties {
            if !sp.contains_key("display_name") {
                issues.push(SchemaIssue {
                    node_name: node.def.name.clone(),
                    severity: IssueSeverity::Error,
                    rule: "DISPLAY_NAME_REQUIRED",
                    message: "Missing 'display_name' in standard_properties".into(),
                });
            }
        }
    }

    // Rule 11: description must not be empty
    if node.def.description.trim().is_empty() {
        issues.push(SchemaIssue {
            node_name: node.def.name.clone(),
            severity: IssueSeverity::Warning,
            rule: "DESCRIPTION_NOT_EMPTY",
            message: "Node 'description' field is empty. Provide a one-line description.".into(),
        });
    }

    // Rule 12: key property (when present) must be type "string"
    if let Some(ref sp) = node.def.standard_properties {
        if let Some(key_prop) = sp.get("key") {
            if key_prop.prop_type != "string" {
                issues.push(SchemaIssue {
                    node_name: node.def.name.clone(),
                    severity: IssueSeverity::Error,
                    rule: "KEY_TYPE_STRING",
                    message: format!(
                        "'key' property must have type 'string', found: '{}'",
                        key_prop.prop_type
                    ),
                });
            }
        }
    }

    // Rule 13: created_at and updated_at must be type "datetime"
    if let Some(ref sp) = node.def.standard_properties {
        for ts_prop in &["created_at", "updated_at"] {
            if let Some(prop) = sp.get(*ts_prop) {
                if prop.prop_type != "datetime" {
                    issues.push(SchemaIssue {
                        node_name: node.def.name.clone(),
                        severity: IssueSeverity::Error,
                        rule: "TIMESTAMPS_DATETIME",
                        message: format!(
                            "'{}' property must have type 'datetime', found: '{}'",
                            ts_prop, prop.prop_type
                        ),
                    });
                }
            }
        }
    }

    // Rule 14: Entity must declare denomination_forms in standard_properties;
    //          EntityNative must declare denomination_forms in properties (ADR-033).
    //          ABSOLUTE RULE: LLM must use only these forms — missing declaration = warning.
    match node.def.name.as_str() {
        "Entity" => {
            let has = node
                .def
                .standard_properties
                .as_ref()
                .map(|sp| sp.contains_key("denomination_forms"))
                .unwrap_or(false);
            if !has {
                issues.push(SchemaIssue {
                    node_name: node.def.name.clone(),
                    severity: IssueSeverity::Warning,
                    rule: "DENOMINATION_FORMS_REQUIRED",
                    message: "Entity.standard_properties must declare 'denomination_forms' \
                              (ADR-033: ABSOLUTE RULE — LLM must use only these forms)"
                        .into(),
                });
            }
        }
        "EntityNative" => {
            let has = node
                .def
                .properties
                .as_ref()
                .map(|p| p.contains_key("denomination_forms"))
                .unwrap_or(false);
            if !has {
                issues.push(SchemaIssue {
                    node_name: node.def.name.clone(),
                    severity: IssueSeverity::Warning,
                    rule: "DENOMINATION_FORMS_REQUIRED",
                    message: "EntityNative.properties must declare 'denomination_forms' \
                              (ADR-033: ABSOLUTE RULE — LLM must use only these forms)"
                        .into(),
                });
            }
        }
        _ => {}
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

    // ─────────────────────────────────────────────────────────────────────────
    // Rules 6-13: New validation rules
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn all_nodes_have_valid_realm() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "REALM_VALID" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors { eprintln!("REALM_VALID ERROR: {} — {}", e.node_name, e.message); }
            panic!("Found {} REALM_VALID errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_valid_layer() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "LAYER_VALID" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors { eprintln!("LAYER_VALID ERROR: {} — {}", e.node_name, e.message); }
            panic!("Found {} LAYER_VALID errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_coherent_realm_layer() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "REALM_LAYER_COHERENCE" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors { eprintln!("REALM_LAYER_COHERENCE ERROR: {} — {}", e.node_name, e.message); }
            panic!("Found {} REALM_LAYER_COHERENCE errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_icon() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let warnings: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "ICON_REQUIRED")
            .collect();
        if !warnings.is_empty() {
            for w in &warnings { eprintln!("ICON_REQUIRED WARNING: {} — {}", w.node_name, w.message); }
            panic!("Found {} nodes with missing/empty icon", warnings.len());
        }
    }

    #[test]
    fn all_non_keyless_nodes_have_display_name() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "DISPLAY_NAME_REQUIRED" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors { eprintln!("DISPLAY_NAME_REQUIRED ERROR: {} — {}", e.node_name, e.message); }
            panic!("Found {} DISPLAY_NAME_REQUIRED errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_non_empty_description() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let warnings: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "DESCRIPTION_NOT_EMPTY")
            .collect();
        if !warnings.is_empty() {
            for w in &warnings { eprintln!("DESCRIPTION_NOT_EMPTY WARNING: {} — {}", w.node_name, w.message); }
            panic!("Found {} nodes with empty description", warnings.len());
        }
    }

    #[test]
    fn key_properties_are_type_string() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "KEY_TYPE_STRING" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors { eprintln!("KEY_TYPE_STRING ERROR: {} — {}", e.node_name, e.message); }
            panic!("Found {} KEY_TYPE_STRING errors", errors.len());
        }
    }

    #[test]
    fn timestamps_are_type_datetime() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes = crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes.iter().flat_map(validate_node)
            .filter(|i| i.rule == "TIMESTAMPS_DATETIME" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors { eprintln!("TIMESTAMPS_DATETIME ERROR: {} — {}", e.node_name, e.message); }
            panic!("Found {} TIMESTAMPS_DATETIME errors", errors.len());
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Rule 14: DENOMINATION_FORMS_REQUIRED (ADR-033)
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn entity_has_denomination_forms_in_standard_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let entity = nodes.iter().find(|n| n.def.name == "Entity")
            .expect("Entity node must exist");

        let sp = entity.def.standard_properties.as_ref()
            .expect("Entity must have standard_properties");

        assert!(
            sp.contains_key("denomination_forms"),
            "Entity.standard_properties must contain 'denomination_forms' (ADR-033 ABSOLUTE RULE)"
        );
    }

    #[test]
    fn entity_native_has_denomination_forms_in_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let entity_native = nodes.iter().find(|n| n.def.name == "EntityNative")
            .expect("EntityNative node must exist");

        let props = entity_native.def.properties.as_ref()
            .expect("EntityNative must have properties");

        assert!(
            props.contains_key("denomination_forms"),
            "EntityNative.properties must contain 'denomination_forms' (ADR-033 ABSOLUTE RULE)"
        );
    }

    #[test]
    fn denomination_forms_validation_catches_missing_entity() {
        // Unit test: validate_node should emit DENOMINATION_FORMS_REQUIRED for Entity
        // that lacks denomination_forms.
        use crate::parsers::yaml_node::{NodeDef, NodeTrait, PropertyDef};
        use indexmap::IndexMap;
        use std::collections::BTreeMap;

        let mut sp: IndexMap<String, PropertyDef> = IndexMap::new();
        let make_prop = |t: &str| PropertyDef {
            prop_type: t.into(),
            required: Some(true),
            description: None,
            extra: BTreeMap::new(),
        };
        sp.insert("key".into(), make_prop("string"));
        sp.insert("display_name".into(), make_prop("string"));
        sp.insert("description".into(), make_prop("string"));
        sp.insert("created_at".into(), make_prop("datetime"));
        sp.insert("updated_at".into(), make_prop("datetime"));
        // NOTE: denomination_forms intentionally absent — that's what we're testing.

        let node = ParsedNode {
            def: NodeDef {
                name: "Entity".into(),
                realm: "org".into(),
                layer: "semantic".into(),
                node_trait: NodeTrait::Defined,
                knowledge_tier: None,
                icon: None,
                description: "An entity".into(),
                standard_properties: Some(sp),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".into(),
            layer: "semantic".into(),
            source_path: std::path::PathBuf::from("entity.yaml"),
        };

        let issues = validate_node(&node);
        let denom_errors: Vec<_> = issues.iter()
            .filter(|i| i.rule == "DENOMINATION_FORMS_REQUIRED")
            .collect();

        assert!(
            !denom_errors.is_empty(),
            "validate_node should emit DENOMINATION_FORMS_REQUIRED for Entity missing denomination_forms"
        );
    }
}
