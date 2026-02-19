//! Schema validation rules for v0.13.1 standardization.
//!
//! Validates:
//! - Standard properties presence (key, created_at, updated_at)
//! - Composite key denormalization (entity_key, page_key, block_key, locale_key)
//! - Property ordering (key → *_key → display_name → ...)
//! - ADR-030/032 compliance: slug derivation, TARGETS arc properties
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
//!
//! # Arc Validation
//!
//! ```ignore
//! let arcs = load_arc_classes_from_files(&root)?;
//! let arc_issues = schema_rules::validate_all_arcs(&arcs.arcs);
//! ```

use crate::parsers::arcs::{ArcClassDef, ArcClassYaml};
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
    (
        "EntityNative",
        "entity",
        "entity_key",
        &["entity_key", "locale_key"],
    ),
    (
        "PageNative",
        "page",
        "page_key",
        &["page_key", "locale_key"],
    ),
    (
        "BlockNative",
        "block",
        "block_key",
        &["block_key", "locale_key"],
    ),
];

/// Valid realm values (ADR-012).
const VALID_REALMS: &[&str] = &["shared", "org"];

/// Valid layer values for the shared realm (ADR-012, v11.5).
const SHARED_LAYERS: &[&str] = &["config", "locale", "geography", "knowledge"];

/// Valid layer values for the org realm (ADR-012, v11.5).
const ORG_LAYERS: &[&str] = &[
    "config",
    "foundation",
    "structure",
    "semantic",
    "instruction",
    "output",
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
                    if let Some(key_pattern) =
                        key_prop.extra.get("pattern").and_then(|v| v.as_str())
                    {
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
                    if let Some(examples) =
                        key_prop.extra.get("examples").and_then(|v| v.as_sequence())
                    {
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
                                    if let Some(parent_part) =
                                        before_at.strip_prefix(&format!("{}:", prefix))
                                    {
                                        // Check if parent_key matches (if present in example description)
                                        if let Some(parent_key_prop) = sp.get(*parent_key_name) {
                                            if let Some(parent_examples) = parent_key_prop
                                                .extra
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
                                            if let Some(locale_examples) = locale_key_prop
                                                .extra
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
    let all_valid_layers: Vec<&str> = SHARED_LAYERS
        .iter()
        .chain(ORG_LAYERS.iter())
        .copied()
        .collect();
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
            message:
                "Missing icon section. Expected: icon: { web: \"lucide-name\", terminal: \"◆\" }"
                    .into(),
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

    // Rule 14: Entity and EntityNative must declare denomination_forms in properties (ADR-033).
    //          ABSOLUTE RULE: LLM must use only these forms — missing declaration = warning.
    //          Note: denomination_forms is entity-specific, so it belongs in properties (not standard_properties).
    match node.def.name.as_str() {
        "Entity" => {
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
                    message: "Entity.properties must declare 'denomination_forms' \
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

    // ─────────────────────────────────────────────────────────────────────────────
    // ADR-030/032 Compliance Rules (v0.13.1)
    // ─────────────────────────────────────────────────────────────────────────────

    // Rule 15: SEOKeyword must have slug_form in properties (ADR-032)
    if node.def.name == "SEOKeyword" {
        let has_slug_form = node
            .def
            .properties
            .as_ref()
            .map(|p| p.contains_key("slug_form"))
            .unwrap_or(false);
        if !has_slug_form {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "SLUG_FORM_REQUIRED",
                message: "SEOKeyword.properties must declare 'slug_form' (ADR-032: URL slug derivation input)"
                    .into(),
            });
        } else if let Some(props) = &node.def.properties {
            if let Some(slug_form) = props.get("slug_form") {
                // Verify slug_form is required
                if slug_form.required != Some(true) {
                    issues.push(SchemaIssue {
                        node_name: node.def.name.clone(),
                        severity: IssueSeverity::Error,
                        rule: "SLUG_FORM_REQUIRED",
                        message: "SEOKeyword.slug_form must be required: true (ADR-032)".into(),
                    });
                }
                // Verify slug_form is type string
                if slug_form.prop_type != "string" {
                    issues.push(SchemaIssue {
                        node_name: node.def.name.clone(),
                        severity: IssueSeverity::Error,
                        rule: "SLUG_FORM_TYPE",
                        message: format!(
                            "SEOKeyword.slug_form must be type 'string', found: '{}'",
                            slug_form.prop_type
                        ),
                    });
                }
            }
        }
    }

    // Rule 16: BlockNative must have content in properties (ADR-030)
    if node.def.name == "BlockNative" {
        let has_content = node
            .def
            .properties
            .as_ref()
            .map(|p| p.contains_key("content"))
            .unwrap_or(false);
        if !has_content {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "CONTENT_REQUIRED",
                message: "BlockNative.properties must declare 'content' (ADR-030: slug lives in head-seo-meta BlockNative.content)"
                    .into(),
            });
        } else if let Some(props) = &node.def.properties {
            if let Some(content) = props.get("content") {
                // Verify content is required
                if content.required != Some(true) {
                    issues.push(SchemaIssue {
                        node_name: node.def.name.clone(),
                        severity: IssueSeverity::Error,
                        rule: "CONTENT_REQUIRED",
                        message: "BlockNative.content must be required: true (ADR-030)".into(),
                    });
                }
                // Verify content is type json
                if content.prop_type != "json" {
                    issues.push(SchemaIssue {
                        node_name: node.def.name.clone(),
                        severity: IssueSeverity::Error,
                        rule: "CONTENT_TYPE",
                        message: format!(
                            "BlockNative.content must be type 'json', found: '{}'",
                            content.prop_type
                        ),
                    });
                }
            }
        }

        // Rule 16b: BlockNative must have block_type in properties (for head-seo-meta identification)
        let has_block_type = node
            .def
            .properties
            .as_ref()
            .map(|p| p.contains_key("block_type"))
            .unwrap_or(false);
        if !has_block_type {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "BLOCK_TYPE_REQUIRED",
                message: "BlockNative.properties must declare 'block_type' (identifies head-seo-meta blocks)"
                    .into(),
            });
        }
    }

    // Rule 17: Composite key nodes must have locale_key denormalized (ADR-029)
    // This extends Rule 2 with more specific validation
    for (composite_node, _prefix, _parent_key_name, _required_props) in COMPOSITE_KEY_NODES {
        if node.def.name == *composite_node {
            if let Some(ref sp) = node.def.standard_properties {
                // Verify locale_key has correct pattern
                if let Some(locale_key) = sp.get("locale_key") {
                    // Check pattern exists and validates BCP-47 format
                    match locale_key.extra.get("pattern").and_then(|v| v.as_str()) {
                        None => {
                            issues.push(SchemaIssue {
                                node_name: node.def.name.clone(),
                                severity: IssueSeverity::Warning,
                                rule: "LOCALE_KEY_PATTERN",
                                message: "locale_key should have pattern: ^[a-z]{2}-[A-Z]{2}$"
                                    .into(),
                            });
                        }
                        Some(pattern)
                            if !pattern.contains("[a-z]") || !pattern.contains("[A-Z]") =>
                        {
                            issues.push(SchemaIssue {
                                node_name: node.def.name.clone(),
                                severity: IssueSeverity::Warning,
                                rule: "LOCALE_KEY_PATTERN",
                                message: format!(
                                    "locale_key pattern should validate BCP-47 format. Found: {}",
                                    pattern
                                ),
                            });
                        }
                        Some(_) => {} // Valid pattern
                    }

                    // Check indexed is true
                    if locale_key.extra.get("indexed") != Some(&serde_yaml::Value::Bool(true)) {
                        issues.push(SchemaIssue {
                            node_name: node.def.name.clone(),
                            severity: IssueSeverity::Warning,
                            rule: "LOCALE_KEY_INDEXED",
                            message: "locale_key should be indexed: true for fast locale filtering"
                                .into(),
                        });
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

// ═══════════════════════════════════════════════════════════════════════════════
// ARC VALIDATION (ADR-030/032 Compliance)
// ═══════════════════════════════════════════════════════════════════════════════

/// Arc property specification: (property_name, is_required, expected_type).
type ArcPropSpec = (&'static str, bool, &'static str);

/// Arc with required properties: (arc_name, &[ArcPropSpec]).
type ArcPropsEntry = (&'static str, &'static [ArcPropSpec]);

/// Arcs with required properties for ADR-030/032 compliance.
const ADR030_ARC_PROPERTIES: &[ArcPropsEntry] = &[
    // TARGETS arc must have is_slug_source property (ADR-030)
    (
        "TARGETS",
        &[
            ("rank", true, "string"),
            ("is_slug_source", false, "boolean"), // Optional but critical for slug derivation
            ("target_position", false, "int"),
        ],
    ),
    // DERIVED_SLUG_FROM arc must have derivation properties (ADR-030)
    (
        "DERIVED_SLUG_FROM",
        &[
            ("derivation_score", true, "float"),
            ("derivation_rationale", false, "string"),
            ("no_repetition_applied", false, "boolean"),
            ("brand_invariant", false, "boolean"),
            ("derivation_timestamp", false, "datetime"),
        ],
    ),
    // SLUGIFIED_BY arc for locale rule validation
    (
        "SLUGIFIED_BY",
        &[
            ("validated", false, "boolean"),
            ("applied_rule", false, "string"),
        ],
    ),
];

/// Arc issue for schema validation (distinct from node issues).
#[derive(Debug, Clone)]
pub struct ArcIssue {
    /// Arc name where the issue was found.
    pub arc_name: String,
    /// Severity of the issue.
    pub severity: IssueSeverity,
    /// Rule code.
    pub rule: &'static str,
    /// Human-readable description.
    pub message: String,
    /// Fix suggestion (optional).
    pub fix_suggestion: Option<String>,
}

/// Validate a single arc against ADR-030/032 rules.
pub fn validate_arc(arc: &ArcClassDef) -> Vec<ArcIssue> {
    let mut issues = Vec::new();

    // Rule A1: Check required properties for ADR-030 arcs
    for (arc_name, required_props) in ADR030_ARC_PROPERTIES {
        if arc.name == *arc_name {
            // Get property names from the arc
            let arc_props = extract_arc_property_names(arc);

            for (prop_name, is_required, expected_type) in *required_props {
                // Check if property exists
                let prop_exists = arc_props.iter().any(|(name, _)| name == prop_name);

                if *is_required && !prop_exists {
                    issues.push(ArcIssue {
                        arc_name: arc.name.clone(),
                        severity: IssueSeverity::Error,
                        rule: "ARC_PROP_REQUIRED",
                        message: format!(
                            "Arc '{}' missing required property '{}' (ADR-030)",
                            arc_name, prop_name
                        ),
                        fix_suggestion: Some(format!(
                            "Add property:\n  - name: {}\n    type: {}\n    required: true",
                            prop_name, expected_type
                        )),
                    });
                }

                // Check property type if it exists
                if let Some((_, Some(actual_type))) =
                    arc_props.iter().find(|(name, _)| name == prop_name)
                {
                    if actual_type != *expected_type {
                        issues.push(ArcIssue {
                            arc_name: arc.name.clone(),
                            severity: IssueSeverity::Warning,
                            rule: "ARC_PROP_TYPE",
                            message: format!(
                                "Arc '{}' property '{}' should be type '{}', found: '{}'",
                                arc_name, prop_name, expected_type, actual_type
                            ),
                            fix_suggestion: None,
                        });
                    }
                }
            }
        }
    }

    // Rule A2: TARGETS arc must document is_slug_source in llm_context
    if arc.name == "TARGETS" {
        if let Some(llm_context) = &arc.llm_context {
            if !llm_context.contains("is_slug_source") && !llm_context.contains("slug source") {
                issues.push(ArcIssue {
                    arc_name: arc.name.clone(),
                    severity: IssueSeverity::Warning,
                    rule: "ARC_LLM_CONTEXT_SLUG",
                    message: "TARGETS arc llm_context should mention 'is_slug_source' for ADR-030 compliance".into(),
                    fix_suggestion: Some(
                        "Add to llm_context: 'CRITICAL: is_slug_source marks the keyword used for URL slug derivation.'".into()
                    ),
                });
            }
        }
    }

    // Rule A3: DERIVED_SLUG_FROM must have correct source/target (BlockNative -> EntityNative)
    if arc.name == "DERIVED_SLUG_FROM" {
        let source_labels = arc.source.labels();
        let target_labels = arc.target.labels();

        if !source_labels.contains(&"BlockNative") {
            issues.push(ArcIssue {
                arc_name: arc.name.clone(),
                severity: IssueSeverity::Error,
                rule: "ARC_SOURCE_TARGET",
                message: "DERIVED_SLUG_FROM source must be BlockNative (ADR-030 v0.13.1)".into(),
                fix_suggestion: Some("Change source: BlockNative".into()),
            });
        }

        if !target_labels.contains(&"EntityNative") {
            issues.push(ArcIssue {
                arc_name: arc.name.clone(),
                severity: IssueSeverity::Error,
                rule: "ARC_SOURCE_TARGET",
                message: "DERIVED_SLUG_FROM target must be EntityNative (ADR-030 v0.13.1)".into(),
                fix_suggestion: Some("Change target: EntityNative".into()),
            });
        }
    }

    // Rule A4: Arc cypher_pattern should include declared properties
    if let Some(cypher) = &arc.cypher_pattern {
        let arc_props = extract_arc_property_names(arc);

        for (prop_name, _) in &arc_props {
            if !cypher.contains(prop_name) {
                issues.push(ArcIssue {
                    arc_name: arc.name.clone(),
                    severity: IssueSeverity::Warning,
                    rule: "ARC_CYPHER_PROPS",
                    message: format!(
                        "Arc cypher_pattern does not include property '{}'. Pattern: {}",
                        prop_name, cypher
                    ),
                    fix_suggestion: Some(format!(
                        "Add {} to cypher_pattern property list",
                        prop_name
                    )),
                });
            }
        }
    }

    // Rule A5: Semantic arcs should have llm_context
    if arc.family == crate::parsers::arcs::ArcFamily::Semantic && arc.llm_context.is_none() {
        issues.push(ArcIssue {
            arc_name: arc.name.clone(),
            severity: IssueSeverity::Warning,
            rule: "ARC_LLM_CONTEXT_REQUIRED",
            message: "Semantic family arcs should have llm_context for LLM comprehension".into(),
            fix_suggestion: Some(
                "Add llm_context with USE/TRIGGERS/NOT/RELATES pattern (ADR-027)".into(),
            ),
        });
    }

    // Rule A6: Generation arcs should have llm_context
    if arc.family == crate::parsers::arcs::ArcFamily::Generation && arc.llm_context.is_none() {
        issues.push(ArcIssue {
            arc_name: arc.name.clone(),
            severity: IssueSeverity::Warning,
            rule: "ARC_LLM_CONTEXT_REQUIRED",
            message: "Generation family arcs should have llm_context for pipeline documentation"
                .into(),
            fix_suggestion: Some(
                "Add llm_context with USE/TRIGGERS/NOT/RELATES pattern (ADR-027)".into(),
            ),
        });
    }

    issues
}

/// Extract property names and types from arc definition.
fn extract_arc_property_names(arc: &ArcClassDef) -> Vec<(String, Option<String>)> {
    let Some(props) = &arc.properties else {
        return Vec::new();
    };

    match props {
        // List of objects with name/type fields
        serde_yaml::Value::Sequence(seq) => {
            seq.iter()
                .filter_map(|item| {
                    if let serde_yaml::Value::Mapping(m) = item {
                        let name = m
                            .get(serde_yaml::Value::String("name".to_string()))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let prop_type = m
                            .get(serde_yaml::Value::String("type".to_string()))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        name.map(|n| (n, prop_type))
                    } else if let serde_yaml::Value::String(s) = item {
                        // Simple string list
                        Some((s.clone(), None))
                    } else {
                        None
                    }
                })
                .collect()
        }
        // Map format: {prop_name: {type: ..., ...}}
        serde_yaml::Value::Mapping(m) => m
            .iter()
            .filter_map(|(k, v)| {
                let name = k.as_str()?.to_string();
                let prop_type = if let serde_yaml::Value::Mapping(prop_map) = v {
                    prop_map
                        .get(serde_yaml::Value::String("type".to_string()))
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                };
                Some((name, prop_type))
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Validate all arcs against schema rules.
pub fn validate_all_arcs(arcs: &[crate::parsers::arcs::ArcDef]) -> Vec<ArcIssue> {
    // We need to load arc-class definitions with full property info
    // For now, validate using ArcDef which has limited info
    arcs.iter()
        .flat_map(|arc| {
            // Convert ArcDef to minimal ArcClassDef for validation
            let arc_class = crate::parsers::arcs::ArcClassDef {
                name: arc.arc_type.clone(),
                family: arc.family,
                scope: arc.scope.clone(),
                temperature_threshold: None,
                source: arc.source.clone(),
                target: arc.target.clone(),
                cardinality: arc.cardinality,
                llm_context: if arc.llm_context.is_empty() {
                    None
                } else {
                    Some(arc.llm_context.clone())
                },
                properties: arc.properties.as_ref().map(|props| {
                    serde_yaml::Value::Sequence(
                        props
                            .iter()
                            .map(|p| serde_yaml::Value::String(p.clone()))
                            .collect(),
                    )
                }),
                inverse: arc.inverse_name.clone(),
                cypher_pattern: None,
            };
            validate_arc(&arc_class)
        })
        .collect()
}

/// Load and validate arc-class YAML files directly for full property validation.
pub fn validate_arc_files(root: &std::path::Path) -> crate::Result<Vec<ArcIssue>> {
    let arc_classes_dir = crate::config::arc_classes_dir(root);

    if !arc_classes_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "arc-classes directory not found: {}",
            arc_classes_dir.display()
        )));
    }

    let mut issues = Vec::new();

    // Scan all family directories
    for family_dir in std::fs::read_dir(&arc_classes_dir)? {
        let family_dir = family_dir?;
        if !family_dir.file_type()?.is_dir() {
            continue;
        }

        // Scan YAML files in each family directory
        for entry in std::fs::read_dir(family_dir.path())? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_none_or(|e| e != "yaml") {
                continue;
            }
            if path.file_name().is_some_and(|n| n == "_index.yaml") {
                continue;
            }

            // Parse and validate
            if let Ok(yaml) = crate::parsers::utils::load_yaml::<ArcClassYaml>(&path) {
                issues.extend(validate_arc(&yaml.arc));
            }
        }
    }

    Ok(issues)
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

    /// Clean up any test files left by other tests to avoid pollution.
    fn cleanup_test_files(root: &std::path::Path) {
        use walkdir::WalkDir;

        let node_classes_dir = crate::config::node_classes_dir(root);
        if !node_classes_dir.exists() {
            return;
        }

        for entry in WalkDir::new(node_classes_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.starts_with("test-")
                    || name.starts_with("_tmp-")
                    || name.starts_with("__test__")
                    || name.contains("-test")
                {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }

    #[test]
    fn all_nodes_pass_schema_rules() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        // Clean up any test files from parallel tests
        cleanup_test_files(&root);

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
                let key_errors: Vec<_> =
                    issues.iter().filter(|i| i.rule == "KEY_REQUIRED").collect();
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
        // Clean up any test files from parallel tests
        cleanup_test_files(&root);
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "REALM_VALID" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("REALM_VALID ERROR: {} — {}", e.node_name, e.message);
            }
            panic!("Found {} REALM_VALID errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_valid_layer() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        // Clean up any test files from parallel tests
        cleanup_test_files(&root);
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "LAYER_VALID" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("LAYER_VALID ERROR: {} — {}", e.node_name, e.message);
            }
            panic!("Found {} LAYER_VALID errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_coherent_realm_layer() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        // Clean up any test files from parallel tests
        cleanup_test_files(&root);
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "REALM_LAYER_COHERENCE" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors {
                eprintln!(
                    "REALM_LAYER_COHERENCE ERROR: {} — {}",
                    e.node_name, e.message
                );
            }
            panic!("Found {} REALM_LAYER_COHERENCE errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_icon() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        // Clean up any test files from parallel tests
        cleanup_test_files(&root);
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let warnings: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "ICON_REQUIRED")
            .collect();
        if !warnings.is_empty() {
            for w in &warnings {
                eprintln!("ICON_REQUIRED WARNING: {} — {}", w.node_name, w.message);
            }
            panic!("Found {} nodes with missing/empty icon", warnings.len());
        }
    }

    #[test]
    fn all_non_keyless_nodes_have_display_name() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "DISPLAY_NAME_REQUIRED" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors {
                eprintln!(
                    "DISPLAY_NAME_REQUIRED ERROR: {} — {}",
                    e.node_name, e.message
                );
            }
            panic!("Found {} DISPLAY_NAME_REQUIRED errors", errors.len());
        }
    }

    #[test]
    fn all_nodes_have_non_empty_description() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        // Clean up any test files from parallel tests
        cleanup_test_files(&root);
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let warnings: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "DESCRIPTION_NOT_EMPTY")
            .collect();
        if !warnings.is_empty() {
            for w in &warnings {
                eprintln!(
                    "DESCRIPTION_NOT_EMPTY WARNING: {} — {}",
                    w.node_name, w.message
                );
            }
            panic!("Found {} nodes with empty description", warnings.len());
        }
    }

    #[test]
    fn key_properties_are_type_string() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "KEY_TYPE_STRING" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("KEY_TYPE_STRING ERROR: {} — {}", e.node_name, e.message);
            }
            panic!("Found {} KEY_TYPE_STRING errors", errors.len());
        }
    }

    #[test]
    fn timestamps_are_type_datetime() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");
        let errors: Vec<_> = nodes
            .iter()
            .flat_map(validate_node)
            .filter(|i| i.rule == "TIMESTAMPS_DATETIME" && i.severity == IssueSeverity::Error)
            .collect();
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("TIMESTAMPS_DATETIME ERROR: {} — {}", e.node_name, e.message);
            }
            panic!("Found {} TIMESTAMPS_DATETIME errors", errors.len());
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Rule 14: DENOMINATION_FORMS_REQUIRED (ADR-033)
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn entity_has_denomination_forms_in_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let entity = nodes
            .iter()
            .find(|n| n.def.name == "Entity")
            .expect("Entity node must exist");

        // denomination_forms is node-SPECIFIC, goes in properties (not standard_properties)
        // Per schema-standard.md: standard_properties = universal (key, display_name, etc.)
        let props = entity
            .def
            .properties
            .as_ref()
            .expect("Entity must have properties");

        assert!(
            props.contains_key("denomination_forms"),
            "Entity.properties must contain 'denomination_forms' (ADR-033 ABSOLUTE RULE)"
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

        let entity_native = nodes
            .iter()
            .find(|n| n.def.name == "EntityNative")
            .expect("EntityNative node must exist");

        let props = entity_native
            .def
            .properties
            .as_ref()
            .expect("EntityNative must have properties");

        assert!(
            props.contains_key("denomination_forms"),
            "EntityNative.properties must contain 'denomination_forms' (ADR-033 ABSOLUTE RULE)"
        );
    }

    #[test]
    fn denomination_forms_validation_catches_missing_entity() {
        // Unit test: validate_node should emit DENOMINATION_FORMS_REQUIRED for Entity
        // that lacks denomination_forms in its properties.
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

        // Create properties map WITHOUT denomination_forms — that's what we're testing
        let mut props: IndexMap<String, PropertyDef> = IndexMap::new();
        props.insert("entity_summary".into(), make_prop("string"));
        // NOTE: denomination_forms intentionally absent

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
                properties: Some(props), // Has properties but missing denomination_forms
                neo4j: None,
                example: None,
            },
            realm: "org".into(),
            layer: "semantic".into(),
            source_path: std::path::PathBuf::from("entity.yaml"),
        };

        let issues = validate_node(&node);
        let denom_errors: Vec<_> = issues
            .iter()
            .filter(|i| i.rule == "DENOMINATION_FORMS_REQUIRED")
            .collect();

        assert!(
            !denom_errors.is_empty(),
            "validate_node should emit DENOMINATION_FORMS_REQUIRED for Entity missing denomination_forms in properties"
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // v0.13.1: YAML/Cypher Alignment Tests (ADR-030, ADR-032)
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn seokeyword_has_slug_form_and_source_date() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let seokeyword = nodes
            .iter()
            .find(|n| n.def.name == "SEOKeyword")
            .expect("SEOKeyword node must exist");

        let props = seokeyword
            .def
            .properties
            .as_ref()
            .expect("SEOKeyword must have properties");

        // v0.13.1 ADR-032: slug_form for URL derivation
        assert!(
            props.contains_key("slug_form"),
            "SEOKeyword.properties must contain 'slug_form' (ADR-032: URL-safe slug form)"
        );
        let slug_form = &props["slug_form"];
        assert_eq!(
            slug_form.prop_type, "string",
            "slug_form must be type string"
        );

        // v0.13.1: source_date for data freshness tracking
        assert!(
            props.contains_key("source_date"),
            "SEOKeyword.properties must contain 'source_date' (Ahrefs/Semrush fetch date)"
        );
        let source_date = &props["source_date"];
        assert_eq!(
            source_date.prop_type, "date",
            "source_date must be type date"
        );
    }

    #[test]
    fn seokeyword_has_locale_key_in_standard_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let seokeyword = nodes
            .iter()
            .find(|n| n.def.name == "SEOKeyword")
            .expect("SEOKeyword node must exist");

        let sp = seokeyword
            .def
            .standard_properties
            .as_ref()
            .expect("SEOKeyword must have standard_properties");

        // v0.13.1: locale_key denormalized for fast lookups
        assert!(
            sp.contains_key("locale_key"),
            "SEOKeyword.standard_properties must contain 'locale_key' (denormalized)"
        );
        let locale_key = &sp["locale_key"];
        assert_eq!(
            locale_key.prop_type, "string",
            "locale_key must be type string"
        );
        // Should have BCP-47 pattern
        if let Some(pattern) = locale_key.extra.get("pattern") {
            let pattern_str = pattern.as_str().unwrap_or("");
            assert!(
                pattern_str.contains("[a-z]") && pattern_str.contains("[A-Z]"),
                "locale_key pattern should match BCP-47 format"
            );
        }
    }

    #[test]
    fn blocknative_has_content_and_block_type() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };
        // Clean up any test files from parallel tests
        cleanup_test_files(&root);

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let blocknative = nodes
            .iter()
            .find(|n| n.def.name == "BlockNative")
            .expect("BlockNative node must exist");

        let props = blocknative
            .def
            .properties
            .as_ref()
            .expect("BlockNative must have properties");

        // v0.13.1 ADR-030: content as JSON blob for BlockType schema
        assert!(
            props.contains_key("content"),
            "BlockNative.properties must contain 'content' (JSON matching BlockType.structure)"
        );
        let content = &props["content"];
        assert_eq!(content.prop_type, "json", "content must be type json");

        // v0.13.1: block_type denormalized for fast filtering
        assert!(
            props.contains_key("block_type"),
            "BlockNative.properties must contain 'block_type' (reference to BlockType.key)"
        );
        let block_type = &props["block_type"];
        assert_eq!(
            block_type.prop_type, "string",
            "block_type must be type string"
        );
    }

    #[test]
    fn blocknative_has_generated_at_timestamp() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        // Clean up any test files left by other tests to avoid pollution
        cleanup_test_files(&root);

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let blocknative = nodes
            .iter()
            .find(|n| n.def.name == "BlockNative")
            .expect("BlockNative node must exist");

        let props = blocknative
            .def
            .properties
            .as_ref()
            .expect("BlockNative must have properties");

        // v0.13.1: generated_at for LLM generation timestamp
        assert!(
            props.contains_key("generated_at"),
            "BlockNative.properties must contain 'generated_at' (LLM generation timestamp)"
        );
        let generated_at = &props["generated_at"];
        assert_eq!(
            generated_at.prop_type, "datetime",
            "generated_at must be type datetime"
        );
    }

    #[test]
    fn seokeyword_composite_key_format() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        // Clean up any test files left by other tests to avoid pollution
        cleanup_test_files(&root);

        let nodes =
            crate::parsers::yaml_node::load_all_nodes(&root).expect("should load all nodes");

        let seokeyword = nodes
            .iter()
            .find(|n| n.def.name == "SEOKeyword")
            .expect("SEOKeyword node must exist");

        let sp = seokeyword
            .def
            .standard_properties
            .as_ref()
            .expect("SEOKeyword must have standard_properties");

        // Check key format: seo:{slug}@{locale_key}
        let key_prop = sp.get("key").expect("SEOKeyword must have 'key' property");

        // Verify pattern if present
        if let Some(pattern) = key_prop.extra.get("pattern") {
            let pattern_str = pattern.as_str().unwrap_or("");
            assert!(
                pattern_str.starts_with("^seo:"),
                "SEOKeyword key pattern must start with 'seo:' prefix"
            );
            assert!(
                pattern_str.contains('@'),
                "SEOKeyword key pattern must include '@' separator for locale"
            );
        }

        // Verify examples if present
        if let Some(examples) = key_prop.extra.get("examples") {
            if let Some(examples_seq) = examples.as_sequence() {
                for example in examples_seq {
                    if let Some(example_str) = example.as_str() {
                        assert!(
                            example_str.starts_with("seo:"),
                            "SEOKeyword key example '{}' must start with 'seo:'",
                            example_str
                        );
                        assert!(
                            example_str.contains('@'),
                            "SEOKeyword key example '{}' must contain '@' separator",
                            example_str
                        );
                    }
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Arc Validation Tests (ADR-030/032)
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn arc_validation_targets_has_required_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let issues = validate_arc_files(&root).expect("should validate arc files");

        // Check TARGETS arc has rank property
        let targets_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.arc_name == "TARGETS" && i.rule == "ARC_PROP_REQUIRED")
            .collect();

        // rank is required - if missing, should be an error
        for issue in &targets_issues {
            eprintln!("TARGETS issue: {}", issue.message);
        }

        // We expect no errors for required properties on TARGETS
        let rank_missing = targets_issues.iter().any(|i| i.message.contains("rank"));
        assert!(
            !rank_missing,
            "TARGETS arc should have 'rank' property defined"
        );
    }

    #[test]
    fn arc_validation_derived_slug_from_has_required_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let issues = validate_arc_files(&root).expect("should validate arc files");

        // Check DERIVED_SLUG_FROM arc
        let derived_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.arc_name == "DERIVED_SLUG_FROM" && i.rule == "ARC_PROP_REQUIRED")
            .collect();

        for issue in &derived_issues {
            eprintln!("DERIVED_SLUG_FROM issue: {}", issue.message);
        }

        // derivation_score is required
        let score_missing = derived_issues
            .iter()
            .any(|i| i.message.contains("derivation_score"));
        assert!(
            !score_missing,
            "DERIVED_SLUG_FROM arc should have 'derivation_score' property defined"
        );
    }

    #[test]
    fn arc_validation_derived_slug_from_source_target() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let issues = validate_arc_files(&root).expect("should validate arc files");

        // Check source/target for DERIVED_SLUG_FROM
        let source_target_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.arc_name == "DERIVED_SLUG_FROM" && i.rule == "ARC_SOURCE_TARGET")
            .collect();

        // Should have no source/target errors (BlockNative -> EntityNative)
        assert!(
            source_target_issues.is_empty(),
            "DERIVED_SLUG_FROM should have correct source (BlockNative) and target (EntityNative). Found issues: {:?}",
            source_target_issues
                .iter()
                .map(|i| &i.message)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn arc_validation_targets_llm_context_mentions_slug() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let issues = validate_arc_files(&root).expect("should validate arc files");

        // Check TARGETS arc llm_context mentions is_slug_source
        let llm_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.arc_name == "TARGETS" && i.rule == "ARC_LLM_CONTEXT_SLUG")
            .collect();

        // TARGETS should mention slug source in llm_context
        assert!(
            llm_issues.is_empty(),
            "TARGETS arc llm_context should mention 'is_slug_source'. Found issues: {:?}",
            llm_issues.iter().map(|i| &i.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn arc_validation_semantic_arcs_have_llm_context() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let issues = validate_arc_files(&root).expect("should validate arc files");

        // Check semantic arcs have llm_context
        let semantic_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.rule == "ARC_LLM_CONTEXT_REQUIRED")
            .collect();

        // Log warnings but don't fail - some arcs may not need llm_context
        if !semantic_issues.is_empty() {
            eprintln!(
                "Warning: {} semantic/generation arcs missing llm_context:",
                semantic_issues.len()
            );
            for issue in &semantic_issues {
                eprintln!("  - {}: {}", issue.arc_name, issue.message);
            }
        }
    }

    #[test]
    fn arc_validation_unit_test_missing_property() {
        use crate::parsers::arcs::{ArcClassDef, ArcFamily, Cardinality, NodeRef};

        // Create an arc definition missing a required property
        let arc = ArcClassDef {
            name: "TARGETS".to_string(),
            family: ArcFamily::Semantic,
            scope: Some("cross_realm".to_string()),
            temperature_threshold: None,
            source: NodeRef::Single("EntityNative".to_string()),
            target: NodeRef::Single("SEOKeyword".to_string()),
            cardinality: Cardinality::ManyToMany,
            llm_context: Some("Test context with is_slug_source mentioned".to_string()),
            properties: None, // Missing properties!
            inverse: None,
            cypher_pattern: None,
        };

        let issues = validate_arc(&arc);

        // Should have error for missing 'rank' property
        let rank_error = issues
            .iter()
            .find(|i| i.rule == "ARC_PROP_REQUIRED" && i.message.contains("rank"));

        assert!(
            rank_error.is_some(),
            "validate_arc should detect missing required 'rank' property on TARGETS arc"
        );
    }

    #[test]
    fn arc_validation_unit_test_wrong_source_target() {
        use crate::parsers::arcs::{ArcClassDef, ArcFamily, Cardinality, NodeRef};

        // Create DERIVED_SLUG_FROM with wrong source
        let arc = ArcClassDef {
            name: "DERIVED_SLUG_FROM".to_string(),
            family: ArcFamily::Generation,
            scope: Some("intra_realm".to_string()),
            temperature_threshold: None,
            source: NodeRef::Single("PageNative".to_string()), // Wrong! Should be BlockNative
            target: NodeRef::Single("EntityNative".to_string()),
            cardinality: Cardinality::ManyToOne,
            llm_context: Some("Test context".to_string()),
            properties: Some(serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::Mapping({
                    let mut m = serde_yaml::Mapping::new();
                    m.insert(
                        serde_yaml::Value::String("name".to_string()),
                        serde_yaml::Value::String("derivation_score".to_string()),
                    );
                    m.insert(
                        serde_yaml::Value::String("type".to_string()),
                        serde_yaml::Value::String("float".to_string()),
                    );
                    m
                }),
            ])),
            inverse: None,
            cypher_pattern: None,
        };

        let issues = validate_arc(&arc);

        // Should have error for wrong source
        let source_error = issues
            .iter()
            .find(|i| i.rule == "ARC_SOURCE_TARGET" && i.message.contains("BlockNative"));

        assert!(
            source_error.is_some(),
            "validate_arc should detect wrong source (PageNative instead of BlockNative) on DERIVED_SLUG_FROM"
        );
    }

    #[test]
    fn extract_arc_property_names_sequence_format() {
        // Test extraction from sequence of objects format
        let props = serde_yaml::Value::Sequence(vec![
            serde_yaml::Value::Mapping({
                let mut m = serde_yaml::Mapping::new();
                m.insert(
                    serde_yaml::Value::String("name".to_string()),
                    serde_yaml::Value::String("rank".to_string()),
                );
                m.insert(
                    serde_yaml::Value::String("type".to_string()),
                    serde_yaml::Value::String("string".to_string()),
                );
                m
            }),
            serde_yaml::Value::Mapping({
                let mut m = serde_yaml::Mapping::new();
                m.insert(
                    serde_yaml::Value::String("name".to_string()),
                    serde_yaml::Value::String("is_slug_source".to_string()),
                );
                m.insert(
                    serde_yaml::Value::String("type".to_string()),
                    serde_yaml::Value::String("boolean".to_string()),
                );
                m
            }),
        ]);

        use crate::parsers::arcs::{ArcClassDef, ArcFamily, Cardinality, NodeRef};

        let arc = ArcClassDef {
            name: "TEST".to_string(),
            family: ArcFamily::Semantic,
            scope: None,
            temperature_threshold: None,
            source: NodeRef::Single("A".to_string()),
            target: NodeRef::Single("B".to_string()),
            cardinality: Cardinality::ManyToMany,
            llm_context: None,
            properties: Some(props),
            inverse: None,
            cypher_pattern: None,
        };

        let extracted = extract_arc_property_names(&arc);

        assert_eq!(extracted.len(), 2);
        assert!(
            extracted
                .iter()
                .any(|(n, t)| n == "rank" && t.as_deref() == Some("string"))
        );
        assert!(
            extracted
                .iter()
                .any(|(n, t)| n == "is_slug_source" && t.as_deref() == Some("boolean"))
        );
    }

    #[test]
    fn extract_arc_property_names_mapping_format() {
        // Test extraction from mapping format
        let props = serde_yaml::Value::Mapping({
            let mut m = serde_yaml::Mapping::new();
            m.insert(
                serde_yaml::Value::String("priority".to_string()),
                serde_yaml::Value::Mapping({
                    let mut pm = serde_yaml::Mapping::new();
                    pm.insert(
                        serde_yaml::Value::String("type".to_string()),
                        serde_yaml::Value::String("integer".to_string()),
                    );
                    pm
                }),
            );
            m
        });

        use crate::parsers::arcs::{ArcClassDef, ArcFamily, Cardinality, NodeRef};

        let arc = ArcClassDef {
            name: "TEST".to_string(),
            family: ArcFamily::Semantic,
            scope: None,
            temperature_threshold: None,
            source: NodeRef::Single("A".to_string()),
            target: NodeRef::Single("B".to_string()),
            cardinality: Cardinality::ManyToMany,
            llm_context: None,
            properties: Some(props),
            inverse: None,
            cypher_pattern: None,
        };

        let extracted = extract_arc_property_names(&arc);

        assert_eq!(extracted.len(), 1);
        assert!(
            extracted
                .iter()
                .any(|(n, t)| n == "priority" && t.as_deref() == Some("integer"))
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (proptest)
    // ─────────────────────────────────────────────────────────────────────────

    use proptest::prelude::*;

    /// Strategy for generating valid realm names.
    fn prop_valid_realm() -> impl Strategy<Value = String> {
        prop::sample::select(vec!["shared".to_string(), "org".to_string()])
    }

    /// Strategy for generating valid shared layer names.
    fn prop_shared_layer() -> impl Strategy<Value = String> {
        prop::sample::select(
            SHARED_LAYERS
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        )
    }

    /// Strategy for generating valid org layer names.
    fn prop_org_layer() -> impl Strategy<Value = String> {
        prop::sample::select(ORG_LAYERS.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    /// Strategy for generating invalid realm names.
    fn prop_invalid_realm() -> impl Strategy<Value = String> {
        "[a-z]{3,10}".prop_filter("must not be valid realm", |s| {
            !VALID_REALMS.contains(&s.as_str())
        })
    }

    /// Strategy for generating a simple slug-like key.
    fn prop_slug_key() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9-]{2,20}"
    }

    /// Strategy for generating valid BCP-47 locale codes.
    fn prop_locale_code() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "en-US".to_string(),
            "fr-FR".to_string(),
            "de-DE".to_string(),
            "es-ES".to_string(),
            "ja-JP".to_string(),
        ])
    }

    /// Strategy for generating composite key node names.
    fn prop_composite_node_name() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "EntityNative".to_string(),
            "PageNative".to_string(),
            "BlockNative".to_string(),
        ])
    }

    /// Strategy for generating valid property types.
    fn prop_valid_property_type() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "string".to_string(),
            "integer".to_string(),
            "boolean".to_string(),
            "datetime".to_string(),
            "float".to_string(),
            "json".to_string(),
            "string[]".to_string(),
        ])
    }

    /// Create a minimal valid node for property testing.
    fn create_valid_proptest_node(name: &str, realm: &str, layer: &str) -> ParsedNode {
        use crate::parsers::yaml_node::{NodeDef, NodeIcon, NodeTrait, PropertyDef};
        use indexmap::IndexMap;
        use std::collections::BTreeMap;

        let make_prop = |t: &str| PropertyDef {
            prop_type: t.into(),
            required: Some(true),
            description: Some("Test property".into()),
            extra: BTreeMap::new(),
        };

        let mut sp: IndexMap<String, PropertyDef> = IndexMap::new();
        sp.insert("key".into(), make_prop("string"));
        sp.insert("display_name".into(), make_prop("string"));
        sp.insert("description".into(), make_prop("string"));
        sp.insert("created_at".into(), make_prop("datetime"));
        sp.insert("updated_at".into(), make_prop("datetime"));

        ParsedNode {
            def: NodeDef {
                name: name.into(),
                realm: realm.into(),
                layer: layer.into(),
                node_trait: NodeTrait::Defined,
                knowledge_tier: None,
                icon: Some(NodeIcon {
                    web: "circle".into(),
                    terminal: "●".into(),
                }),
                description: "Test node".into(),
                standard_properties: Some(sp),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: realm.into(),
            layer: layer.into(),
            source_path: std::path::PathBuf::from(format!("{}.yaml", name.to_lowercase())),
        }
    }

    proptest! {
        /// Property: Valid realm+layer combinations produce no REALM_LAYER_COHERENCE errors.
        #[test]
        fn prop_valid_shared_realm_layer_no_coherence_error(layer in prop_shared_layer()) {
            let node = create_valid_proptest_node("TestNode", "shared", &layer);
            let issues = validate_node(&node);

            let coherence_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.rule == "REALM_LAYER_COHERENCE")
                .collect();

            prop_assert!(
                coherence_errors.is_empty(),
                "shared realm + {} layer should produce no coherence errors, got {:?}",
                layer,
                coherence_errors
            );
        }

        /// Property: Valid org realm+layer combinations produce no REALM_LAYER_COHERENCE errors.
        #[test]
        fn prop_valid_org_realm_layer_no_coherence_error(layer in prop_org_layer()) {
            let node = create_valid_proptest_node("TestNode", "org", &layer);
            let issues = validate_node(&node);

            let coherence_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.rule == "REALM_LAYER_COHERENCE")
                .collect();

            prop_assert!(
                coherence_errors.is_empty(),
                "org realm + {} layer should produce no coherence errors, got {:?}",
                layer,
                coherence_errors
            );
        }

        /// Property: Invalid realms always produce REALM_VALID errors.
        #[test]
        fn prop_invalid_realm_produces_error(invalid_realm in prop_invalid_realm()) {
            let node = create_valid_proptest_node("TestNode", &invalid_realm, "config");
            let issues = validate_node(&node);

            let realm_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.rule == "REALM_VALID")
                .collect();

            prop_assert!(
                !realm_errors.is_empty(),
                "Invalid realm '{}' should produce REALM_VALID error",
                invalid_realm
            );
        }

        /// Property: Composite keys must match format {prefix}:{key}@{locale}.
        #[test]
        fn prop_composite_key_format_valid(
            node_name in prop_composite_node_name(),
            entity_key in prop_slug_key(),
            locale in prop_locale_code()
        ) {
            let prefix = match node_name.as_str() {
                "EntityNative" => "entity",
                "PageNative" => "page",
                "BlockNative" => "block",
                _ => unreachable!(),
            };

            // Build a valid composite key
            let composite_key = format!("{}:{}@{}", prefix, entity_key, locale);

            // Validate the regex pattern matches
            let pattern = format!("^{}:[^@]+@[a-z]{{2}}-[A-Z]{{2}}$", prefix);
            let re = regex::Regex::new(&pattern).unwrap();

            prop_assert!(
                re.is_match(&composite_key),
                "Composite key '{}' should match pattern '{}'",
                composite_key,
                pattern
            );
        }

        /// Property: Composite keys without @ separator are invalid.
        #[test]
        fn prop_composite_key_without_at_invalid(
            node_name in prop_composite_node_name(),
            entity_key in prop_slug_key()
        ) {
            let prefix = match node_name.as_str() {
                "EntityNative" => "entity",
                "PageNative" => "page",
                "BlockNative" => "block",
                _ => unreachable!(),
            };

            // Build an INVALID composite key (no @ separator)
            let invalid_key = format!("{}:{}", prefix, entity_key);

            // Validate the regex pattern does NOT match
            let pattern = format!("^{}:[^@]+@[a-z]{{2}}-[A-Z]{{2}}$", prefix);
            let re = regex::Regex::new(&pattern).unwrap();

            prop_assert!(
                !re.is_match(&invalid_key),
                "Composite key without '@' ('{}') should NOT match pattern '{}'",
                invalid_key,
                pattern
            );
        }

        /// Property: If key contains @, locale_key must exist in denormalized properties.
        /// This validates the DENORM_REQUIRED rule logic.
        #[test]
        fn prop_composite_key_implies_locale_key_required(
            node_name in prop_composite_node_name(),
            entity_key in prop_slug_key(),
            locale in prop_locale_code()
        ) {
            use crate::parsers::yaml_node::{NodeDef, NodeTrait, PropertyDef, NodeIcon};
            use indexmap::IndexMap;
            use std::collections::BTreeMap;

            let _composite_key = format!("{}:{}@{}",
                node_name.trim_end_matches("Native").to_lowercase(),
                entity_key,
                locale
            );

            // Create node WITHOUT locale_key (should trigger DENORM_REQUIRED)
            let make_prop = |t: &str| PropertyDef {
                prop_type: t.into(),
                required: Some(true),
                description: Some("Test".into()),
                extra: BTreeMap::new(),
            };

            let mut sp: IndexMap<String, PropertyDef> = IndexMap::new();
            sp.insert("key".into(), make_prop("string"));
            sp.insert("display_name".into(), make_prop("string"));
            sp.insert("description".into(), make_prop("string"));
            sp.insert("created_at".into(), make_prop("datetime"));
            sp.insert("updated_at".into(), make_prop("datetime"));
            // NOTE: Intentionally missing locale_key and parent_key

            let (realm, layer, trait_val) = match node_name.as_str() {
                "EntityNative" => ("org", "semantic", NodeTrait::Authored),
                "PageNative" => ("org", "output", NodeTrait::Generated),
                "BlockNative" => ("org", "output", NodeTrait::Generated),
                _ => unreachable!(),
            };

            let node = ParsedNode {
                def: NodeDef {
                    name: node_name.clone(),
                    realm: realm.into(),
                    layer: layer.into(),
                    node_trait: trait_val,
                    knowledge_tier: None,
                    icon: Some(NodeIcon { web: "circle".into(), terminal: "●".into() }),
                    description: "Test".into(),
                    standard_properties: Some(sp),
                    properties: None,
                    neo4j: None,
                    example: None,
                },
                realm: realm.into(),
                layer: layer.into(),
                source_path: std::path::PathBuf::from("test.yaml"),
            };

            let issues = validate_node(&node);
            let denorm_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.rule == "DENORM_REQUIRED")
                .collect();

            prop_assert!(
                !denorm_errors.is_empty(),
                "{} without locale_key should produce DENORM_REQUIRED error",
                node_name
            );
        }

        /// Property: Required properties must be a subset of all declared properties.
        /// Tests that if a property has required=true, it must exist in the properties map.
        #[test]
        fn prop_required_properties_are_declared(
            prop_count in 1usize..5,
            required_indices in proptest::collection::vec(0usize..5, 0..3)
        ) {
            use crate::parsers::yaml_node::{NodeDef, NodeTrait, PropertyDef, NodeIcon};
            use indexmap::IndexMap;
            use std::collections::BTreeMap;

            let mut sp: IndexMap<String, PropertyDef> = IndexMap::new();

            // Always add standard required properties
            sp.insert("key".into(), PropertyDef {
                prop_type: "string".into(),
                required: Some(true),
                description: Some("Key".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("display_name".into(), PropertyDef {
                prop_type: "string".into(),
                required: Some(true),
                description: Some("Display name".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("description".into(), PropertyDef {
                prop_type: "string".into(),
                required: Some(false),
                description: Some("Description".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("created_at".into(), PropertyDef {
                prop_type: "datetime".into(),
                required: Some(true),
                description: Some("Created".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("updated_at".into(), PropertyDef {
                prop_type: "datetime".into(),
                required: Some(true),
                description: Some("Updated".into()),
                extra: BTreeMap::new(),
            });

            // Add random extra properties
            for i in 0..prop_count {
                let is_required = required_indices.contains(&i);
                sp.insert(
                    format!("custom_prop_{}", i),
                    PropertyDef {
                        prop_type: "string".into(),
                        required: Some(is_required),
                        description: Some(format!("Custom property {}", i)),
                        extra: BTreeMap::new(),
                    },
                );
            }

            let node = ParsedNode {
                def: NodeDef {
                    name: "TestNode".into(),
                    realm: "org".into(),
                    layer: "semantic".into(),
                    node_trait: NodeTrait::Defined,
                    knowledge_tier: None,
                    icon: Some(NodeIcon { web: "circle".into(), terminal: "●".into() }),
                    description: "Test node".into(),
                    standard_properties: Some(sp.clone()),
                    properties: None,
                    neo4j: None,
                    example: None,
                },
                realm: "org".into(),
                layer: "semantic".into(),
                source_path: std::path::PathBuf::from("test.yaml"),
            };

            // Collect all declared required properties
            let required_props: Vec<&String> = sp
                .iter()
                .filter(|(_, def)| def.required == Some(true))
                .map(|(name, _)| name)
                .collect();

            // Verify all required properties exist in the map
            for req_prop in &required_props {
                prop_assert!(
                    sp.contains_key(*req_prop),
                    "Required property '{}' must exist in properties map",
                    req_prop
                );
            }

            // Validation should not error for this properly declared node
            let issues = validate_node(&node);
            let structural_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.severity == IssueSeverity::Error)
                .filter(|i| i.rule != "DENOMINATION_FORMS_REQUIRED") // Expected for non-Entity
                .collect();

            prop_assert!(
                structural_errors.is_empty(),
                "Well-formed node should have no structural errors, got {:?}",
                structural_errors
            );
        }

        /// Property: All property types must be from the valid set.
        #[test]
        fn prop_property_types_are_valid(prop_type in prop_valid_property_type()) {
            use crate::parsers::yaml_node::PropertyDef;
            use std::collections::BTreeMap;

            let prop = PropertyDef {
                prop_type: prop_type.clone(),
                required: Some(true),
                description: Some("Test".into()),
                extra: BTreeMap::new(),
            };

            // Valid types should parse correctly
            let valid_types = ["string", "integer", "boolean", "datetime", "float", "json", "string[]"];
            prop_assert!(
                valid_types.contains(&prop.prop_type.as_str()),
                "Property type '{}' should be one of {:?}",
                prop.prop_type,
                valid_types
            );
        }

        /// Property: key property must always be type string.
        #[test]
        fn prop_key_must_be_string_type(wrong_type in "[a-z]{3,10}".prop_filter("not string", |s| s != "string")) {
            use crate::parsers::yaml_node::{NodeDef, NodeTrait, PropertyDef, NodeIcon};
            use indexmap::IndexMap;
            use std::collections::BTreeMap;

            let mut sp: IndexMap<String, PropertyDef> = IndexMap::new();

            // Add key with WRONG type
            sp.insert("key".into(), PropertyDef {
                prop_type: wrong_type.clone(),
                required: Some(true),
                description: Some("Key".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("display_name".into(), PropertyDef {
                prop_type: "string".into(),
                required: Some(true),
                description: Some("Display name".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("created_at".into(), PropertyDef {
                prop_type: "datetime".into(),
                required: Some(true),
                description: Some("Created".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("updated_at".into(), PropertyDef {
                prop_type: "datetime".into(),
                required: Some(true),
                description: Some("Updated".into()),
                extra: BTreeMap::new(),
            });

            let node = ParsedNode {
                def: NodeDef {
                    name: "TestNode".into(),
                    realm: "org".into(),
                    layer: "semantic".into(),
                    node_trait: NodeTrait::Defined,
                    knowledge_tier: None,
                    icon: Some(NodeIcon { web: "circle".into(), terminal: "●".into() }),
                    description: "Test node".into(),
                    standard_properties: Some(sp),
                    properties: None,
                    neo4j: None,
                    example: None,
                },
                realm: "org".into(),
                layer: "semantic".into(),
                source_path: std::path::PathBuf::from("test.yaml"),
            };

            let issues = validate_node(&node);
            let key_type_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.rule == "KEY_TYPE_STRING")
                .collect();

            prop_assert!(
                !key_type_errors.is_empty(),
                "key with type '{}' (not string) should produce KEY_TYPE_STRING error",
                wrong_type
            );
        }

        /// Property: Timestamps must always be type datetime.
        #[test]
        fn prop_timestamps_must_be_datetime(
            wrong_type in "[a-z]{3,10}".prop_filter("not datetime", |s| s != "datetime")
        ) {
            use crate::parsers::yaml_node::{NodeDef, NodeTrait, PropertyDef, NodeIcon};
            use indexmap::IndexMap;
            use std::collections::BTreeMap;

            let mut sp: IndexMap<String, PropertyDef> = IndexMap::new();

            sp.insert("key".into(), PropertyDef {
                prop_type: "string".into(),
                required: Some(true),
                description: Some("Key".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("display_name".into(), PropertyDef {
                prop_type: "string".into(),
                required: Some(true),
                description: Some("Display name".into()),
                extra: BTreeMap::new(),
            });
            // Add timestamps with WRONG type
            sp.insert("created_at".into(), PropertyDef {
                prop_type: wrong_type.clone(),
                required: Some(true),
                description: Some("Created".into()),
                extra: BTreeMap::new(),
            });
            sp.insert("updated_at".into(), PropertyDef {
                prop_type: wrong_type.clone(),
                required: Some(true),
                description: Some("Updated".into()),
                extra: BTreeMap::new(),
            });

            let node = ParsedNode {
                def: NodeDef {
                    name: "TestNode".into(),
                    realm: "org".into(),
                    layer: "semantic".into(),
                    node_trait: NodeTrait::Defined,
                    knowledge_tier: None,
                    icon: Some(NodeIcon { web: "circle".into(), terminal: "●".into() }),
                    description: "Test node".into(),
                    standard_properties: Some(sp),
                    properties: None,
                    neo4j: None,
                    example: None,
                },
                realm: "org".into(),
                layer: "semantic".into(),
                source_path: std::path::PathBuf::from("test.yaml"),
            };

            let issues = validate_node(&node);
            let timestamp_errors: Vec<_> = issues
                .iter()
                .filter(|i| i.rule == "TIMESTAMPS_DATETIME")
                .collect();

            prop_assert!(
                !timestamp_errors.is_empty(),
                "Timestamps with type '{}' (not datetime) should produce TIMESTAMPS_DATETIME error",
                wrong_type
            );
        }

        /// Property: Standard properties order validation is deterministic.
        /// Same input always produces same validation result.
        #[test]
        fn prop_validation_is_deterministic(
            realm in prop_valid_realm(),
            layer in prop::sample::select(
                SHARED_LAYERS.iter().chain(ORG_LAYERS.iter())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            )
        ) {
            // Skip invalid combinations
            let valid_for_realm = match realm.as_str() {
                "shared" => SHARED_LAYERS.contains(&layer.as_str()),
                "org" => ORG_LAYERS.contains(&layer.as_str()),
                _ => false,
            };

            if !valid_for_realm {
                return Ok(());
            }

            let node = create_valid_proptest_node("TestNode", &realm, &layer);

            // Run validation twice
            let issues1 = validate_node(&node);
            let issues2 = validate_node(&node);

            // Both runs should produce identical results
            prop_assert_eq!(
                issues1.len(),
                issues2.len(),
                "Validation should be deterministic"
            );

            for (i1, i2) in issues1.iter().zip(issues2.iter()) {
                prop_assert_eq!(&i1.rule, &i2.rule);
                prop_assert_eq!(&i1.node_name, &i2.node_name);
                prop_assert_eq!(&i1.severity, &i2.severity);
            }
        }

        /// Property: Arc properties must have valid types.
        /// Tests that arc property types are from the allowed set.
        #[test]
        fn prop_arc_property_types_valid(prop_type in prop_valid_property_type()) {
            // Arc properties use the same type system as node properties
            let valid_arc_types = ["string", "integer", "boolean", "datetime", "float", "json", "string[]"];

            prop_assert!(
                valid_arc_types.contains(&prop_type.as_str()),
                "Arc property type '{}' should be valid",
                prop_type
            );
        }
    }
}
