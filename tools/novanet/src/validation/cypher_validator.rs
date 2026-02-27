//! Cypher seed file validator against YAML definitions.
//!
//! Validates that Cypher seed files:
//! 1. Only use properties defined in YAML
//! 2. Set all required properties from YAML
//! 3. Use consistent property types
//!
//! v0.13.1 — ADR-003 (YAML-First Architecture) coherence validation.

use crate::parsers::yaml_node::{ParsedNode, PropertyDef};
use indexmap::IndexMap;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use walkdir::WalkDir;

// ─────────────────────────────────────────────────────────────────────────────
// Compiled Regex Patterns (LazyLock for one-time compilation)
// ─────────────────────────────────────────────────────────────────────────────

/// Match MERGE/CREATE (n:Label {key: 'value'}) or MERGE (n:Label {key: $param})
static RE_MERGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:MERGE|CREATE)\s*\(\s*\w*\s*:\s*([A-Z][A-Za-z0-9]*)\s*\{([^}]*)\}\s*\)")
        .expect("valid merge regex")
});

/// Match SET n.property = 'value' or SET n.property = expression
static RE_SET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(\w+)\.(\w+)\s*=\s*([^,;\n]+)").expect("valid set regex"));

/// Match ON CREATE SET / ON MATCH SET blocks
static RE_ON_SET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)ON\s+(?:CREATE|MATCH)\s+SET\s+((?:\s*\w+\.\w+\s*=\s*[^,;\n]+,?\s*)+)")
        .expect("valid on_set regex")
});

/// Match inline property: value patterns
static RE_PROP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\w+)\s*:\s*([^,}]+)").expect("valid prop regex"));

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

/// A property usage found in a Cypher file.
#[derive(Debug, Clone)]
pub struct CypherPropertyUsage {
    /// Property name (e.g., "display_name")
    pub property: String,
    /// Value as string (for type inference)
    pub value: Option<String>,
    /// File path where found
    pub file: PathBuf,
    /// Line number in file
    pub line: usize,
    /// The node label this property belongs to
    pub label: String,
}

/// A validation issue found during Cypher validation.
#[derive(Debug)]
pub struct CypherValidationIssue {
    /// Severity level
    pub severity: IssueSeverity,
    /// Rule that was violated
    pub rule: CypherValidationRule,
    /// Human-readable message
    pub message: String,
    /// File where the issue was found
    pub file: PathBuf,
    /// Line number (if applicable)
    pub line: Option<usize>,
    /// Node label involved
    pub label: String,
    /// Property name involved (if applicable)
    pub property: Option<String>,
}

/// Severity of validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    /// Critical issue that must be fixed.
    Error,
    /// Non-critical issue that should be reviewed.
    Warning,
}

/// Validation rules for Cypher files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CypherValidationRule {
    /// Property used in Cypher but not defined in YAML.
    UndefinedProperty,
    /// Required property from YAML not set in Cypher.
    MissingRequiredProperty,
    /// Property type in Cypher doesn't match YAML definition.
    TypeMismatch,
    /// Unknown node label (not in YAML definitions).
    UnknownLabel,
}

impl std::fmt::Display for CypherValidationRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedProperty => write!(f, "UNDEFINED_PROPERTY"),
            Self::MissingRequiredProperty => write!(f, "MISSING_REQUIRED"),
            Self::TypeMismatch => write!(f, "TYPE_MISMATCH"),
            Self::UnknownLabel => write!(f, "UNKNOWN_LABEL"),
        }
    }
}

/// A Cypher statement parsed from a seed file.
#[allow(dead_code)]
#[derive(Debug)]
struct CypherStatement {
    /// Node labels found in MERGE/CREATE clauses
    labels: Vec<String>,
    /// Properties set (property name -> value string)
    properties: HashMap<String, String>,
    /// Line number where statement starts
    line: usize,
}

/// Aggregate info about a node label in Cypher files.
#[derive(Debug, Default)]
pub struct LabelUsageInfo {
    /// All properties ever used for this label
    pub properties_used: HashSet<String>,
    /// Files where this label appears
    pub files: HashSet<PathBuf>,
    /// Count of usages
    pub count: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// Cypher Parser
// ─────────────────────────────────────────────────────────────────────────────

/// Parse a Cypher file and extract property usage for each node label.
pub fn parse_cypher_file(path: &Path) -> crate::Result<Vec<CypherPropertyUsage>> {
    let content = std::fs::read_to_string(path)?;
    let mut usages = Vec::new();

    // Use static LazyLock patterns (compiled once)

    for (line_idx, line) in content.lines().enumerate() {
        let line_num = line_idx + 1;

        // Skip comments
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.is_empty() {
            continue;
        }

        // Parse MERGE/CREATE statements with inline properties
        for cap in RE_MERGE.captures_iter(line) {
            let label = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let props_str = cap.get(2).map(|m| m.as_str()).unwrap_or_default();

            // Parse inline properties like {key: 'value', name: 'foo'}
            for prop_usage in parse_inline_properties(props_str, &label, path, line_num) {
                usages.push(prop_usage);
            }
        }

        // Parse SET statements
        for cap in RE_SET.captures_iter(line) {
            // We need to infer the label from context, for now use empty string
            // The validator will match by property name patterns
            let property = cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let value = cap.get(3).map(|m| m.as_str().trim().to_string());

            // Try to infer label from variable name or context
            let var_name = cap.get(1).map(|m| m.as_str()).unwrap_or_default();
            let label = infer_label_from_context(&content, var_name, line_idx);

            if !property.is_empty() {
                usages.push(CypherPropertyUsage {
                    property,
                    value,
                    file: path.to_path_buf(),
                    line: line_num,
                    label,
                });
            }
        }
    }

    // Also parse multi-line ON CREATE SET / ON MATCH SET blocks
    for cap in RE_ON_SET.captures_iter(&content) {
        let set_block = cap.get(1).map(|m| m.as_str()).unwrap_or_default();
        for prop_cap in RE_SET.captures_iter(set_block) {
            let var_name = prop_cap.get(1).map(|m| m.as_str()).unwrap_or_default();
            let property = prop_cap
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let value = prop_cap.get(3).map(|m| m.as_str().trim().to_string());

            // Find line number for this usage
            let line_num = content
                .lines()
                .enumerate()
                .find(|(_, l)| l.contains(&format!("{}.{}", var_name, property)))
                .map(|(i, _)| i + 1)
                .unwrap_or(0);

            let label = infer_label_from_context(&content, var_name, line_num.saturating_sub(1));

            if !property.is_empty() {
                usages.push(CypherPropertyUsage {
                    property,
                    value,
                    file: path.to_path_buf(),
                    line: line_num,
                    label,
                });
            }
        }
    }

    Ok(usages)
}

/// Parse inline properties from MERGE/CREATE clause.
fn parse_inline_properties(
    props_str: &str,
    label: &str,
    file: &Path,
    line: usize,
) -> Vec<CypherPropertyUsage> {
    let mut usages = Vec::new();

    // Use static LazyLock pattern (compiled once)
    for cap in RE_PROP.captures_iter(props_str) {
        let property = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let value = cap.get(2).map(|m| m.as_str().trim().to_string());

        if !property.is_empty() {
            usages.push(CypherPropertyUsage {
                property,
                value,
                file: file.to_path_buf(),
                line,
                label: label.to_string(),
            });
        }
    }

    usages
}

/// Try to infer the node label from context (look for previous MERGE/CREATE with same variable).
fn infer_label_from_context(content: &str, var_name: &str, _line_idx: usize) -> String {
    // Look for MERGE (var:Label pattern in the file
    let pattern = format!(
        r"(?i)(?:MERGE|CREATE)\s*\(\s*{}\s*:\s*([A-Z][A-Za-z0-9]*)",
        regex::escape(var_name)
    );
    if let Ok(re) = Regex::new(&pattern) {
        if let Some(cap) = re.captures(content) {
            return cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
        }
    }

    // Common variable name patterns
    match var_name {
        "e" | "entity" => "Entity".to_string(),
        "l" | "locale" => "Locale".to_string(),
        "p" | "proj" | "project" => "Project".to_string(),
        "kw" | "keyword" => "SEOKeyword".to_string(),
        "bn" | "block" => "BlockNative".to_string(),
        "en" | "ec" => "EntityNative".to_string(),
        "pn" => "PageNative".to_string(),
        "s" | "slug" => "Slugification".to_string(),
        "f" | "fmt" => "Formatting".to_string(),
        "c" | "culture" => "Culture".to_string(),
        "m" | "mkt" | "market" => "Market".to_string(),
        "expr" | "ex" => "Expression".to_string(),
        _ => String::new(),
    }
}

/// Infer the Cypher type from a value string.
fn infer_cypher_type(value: &str) -> &'static str {
    let value = value.trim();

    // datetime() function
    if value.starts_with("datetime(") || value == "datetime()" {
        return "datetime";
    }

    // date() function
    if value.starts_with("date(") {
        return "datetime";
    }

    // Boolean literals
    if value == "true" || value == "false" {
        return "boolean";
    }

    // String literals (quoted)
    if (value.starts_with('\'') && value.ends_with('\''))
        || (value.starts_with('"') && value.ends_with('"'))
    {
        return "string";
    }

    // Integer literals
    if value.parse::<i64>().is_ok() {
        return "int";
    }

    // Float literals
    if value.parse::<f64>().is_ok() && value.contains('.') {
        return "float";
    }

    // Array/list
    if value.starts_with('[') && value.ends_with(']') {
        return "array";
    }

    // JSON object
    if value.starts_with('{') && value.ends_with('}') {
        return "json";
    }

    // Default to string
    "string"
}

// ─────────────────────────────────────────────────────────────────────────────
// Validator
// ─────────────────────────────────────────────────────────────────────────────

/// Build a map of label -> properties from YAML definitions.
fn build_yaml_property_map(nodes: &[ParsedNode]) -> HashMap<String, IndexMap<String, PropertyDef>> {
    let mut map = HashMap::new();

    for node in nodes {
        let mut all_props = IndexMap::new();

        // Merge standard_properties
        if let Some(ref sp) = node.def.standard_properties {
            for (name, def) in sp {
                all_props.insert(name.clone(), def.clone());
            }
        }

        // Merge properties
        if let Some(ref p) = node.def.properties {
            for (name, def) in p {
                all_props.insert(name.clone(), def.clone());
            }
        }

        map.insert(node.def.name.clone(), all_props);
    }

    map
}

/// Validate Cypher seed files against YAML definitions.
///
/// # Arguments
/// * `root` - Monorepo root path
/// * `seed_files` - Optional list of specific files to validate (all if None)
///
/// # Returns
/// List of validation issues found.
pub fn validate_cypher_files(
    root: &Path,
    seed_files: Option<Vec<PathBuf>>,
) -> crate::Result<Vec<CypherValidationIssue>> {
    // Load YAML node definitions
    let nodes = crate::parsers::yaml_node::load_all_nodes(root)?;
    let yaml_props = build_yaml_property_map(&nodes);
    let known_labels: HashSet<String> = nodes.iter().map(|n| n.def.name.clone()).collect();

    // Find seed files
    let seed_dir = crate::config::seed_dir(root);
    let files: Vec<PathBuf> = if let Some(specific) = seed_files {
        specific
    } else {
        WalkDir::new(&seed_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "cypher")
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    let mut issues = Vec::new();

    // Aggregate property usage by label
    let mut label_usage: HashMap<String, LabelUsageInfo> = HashMap::new();

    // Parse all Cypher files
    for file in &files {
        let usages = parse_cypher_file(file)?;

        for usage in usages {
            if usage.label.is_empty() {
                continue;
            }

            // Track usage
            let info = label_usage.entry(usage.label.clone()).or_default();
            info.properties_used.insert(usage.property.clone());
            info.files.insert(usage.file.clone());
            info.count += 1;

            // Check if label is known
            if !known_labels.contains(&usage.label) {
                // Only report once per label per file
                let key = format!("{}:{}", usage.file.display(), usage.label);
                static REPORTED: std::sync::OnceLock<std::sync::Mutex<HashSet<String>>> =
                    std::sync::OnceLock::new();
                let reported = REPORTED.get_or_init(|| std::sync::Mutex::new(HashSet::new()));
                let mut reported = reported.lock().unwrap();
                if !reported.contains(&key) {
                    reported.insert(key);
                    issues.push(CypherValidationIssue {
                        severity: IssueSeverity::Warning,
                        rule: CypherValidationRule::UnknownLabel,
                        message: format!("Label '{}' not found in YAML definitions", usage.label),
                        file: usage.file.clone(),
                        line: Some(usage.line),
                        label: usage.label.clone(),
                        property: None,
                    });
                }
                continue;
            }

            // Check if property is defined
            if let Some(props) = yaml_props.get(&usage.label) {
                if !props.contains_key(&usage.property) {
                    issues.push(CypherValidationIssue {
                        severity: IssueSeverity::Error,
                        rule: CypherValidationRule::UndefinedProperty,
                        message: format!(
                            "Property '{}' on '{}' not defined in YAML",
                            usage.property, usage.label
                        ),
                        file: usage.file.clone(),
                        line: Some(usage.line),
                        label: usage.label.clone(),
                        property: Some(usage.property.clone()),
                    });
                } else if let Some(value) = &usage.value {
                    // Check type compatibility
                    let yaml_type = props
                        .get(&usage.property)
                        .map(|p| p.prop_type.as_str())
                        .unwrap_or("string");
                    let cypher_type = infer_cypher_type(value);

                    if !types_compatible(yaml_type, cypher_type) {
                        issues.push(CypherValidationIssue {
                            severity: IssueSeverity::Warning,
                            rule: CypherValidationRule::TypeMismatch,
                            message: format!(
                                "Property '{}' on '{}': YAML type '{}' vs Cypher value type '{}'",
                                usage.property, usage.label, yaml_type, cypher_type
                            ),
                            file: usage.file.clone(),
                            line: Some(usage.line),
                            label: usage.label.clone(),
                            property: Some(usage.property.clone()),
                        });
                    }
                }
            }
        }
    }

    // Check for missing required properties (aggregate across all files for each label)
    for (label, props) in &yaml_props {
        if let Some(usage_info) = label_usage.get(label) {
            // Get required properties from YAML
            let required_props: Vec<&str> = props
                .iter()
                .filter(|(name, def)| {
                    def.required.unwrap_or(false)
                        // Skip timestamps - they're often set via datetime()
                        && *name != "created_at"
                        && *name != "updated_at"
                })
                .map(|(name, _)| name.as_str())
                .collect();

            for prop in required_props {
                if !usage_info.properties_used.contains(prop) {
                    // Report for first file that uses this label
                    if let Some(file) = usage_info.files.iter().next() {
                        issues.push(CypherValidationIssue {
                            severity: IssueSeverity::Warning,
                            rule: CypherValidationRule::MissingRequiredProperty,
                            message: format!(
                                "Required property '{}' on '{}' never set in seed files",
                                prop, label
                            ),
                            file: file.clone(),
                            line: None,
                            label: label.clone(),
                            property: Some(prop.to_string()),
                        });
                    }
                }
            }
        }
    }

    // Sort issues by file, then line
    issues.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then(a.line.cmp(&b.line))
            .then(a.rule.to_string().cmp(&b.rule.to_string()))
    });

    Ok(issues)
}

/// Check if YAML type is compatible with inferred Cypher type.
fn types_compatible(yaml_type: &str, cypher_type: &str) -> bool {
    match (yaml_type, cypher_type) {
        // Exact matches
        (a, b) if a == b => true,
        // String is compatible with most things
        ("string", _) => true,
        // datetime type
        ("datetime", "datetime") => true,
        ("datetime", "string") => true, // ISO strings are OK
        // Numeric types
        ("int", "int") => true,
        ("integer", "int") => true,
        ("float", "float") => true,
        ("float", "int") => true, // int can be used as float
        // Boolean
        ("boolean", "boolean") => true,
        ("bool", "boolean") => true,
        // Arrays
        ("array", "array") => true,
        ("string[]", "array") => true,
        // JSON objects
        ("json", "json") => true,
        ("json", "string") => true, // JSON can be stored as string
        ("object", "json") => true,
        // Default: not compatible
        _ => false,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Summary Report
// ─────────────────────────────────────────────────────────────────────────────

/// Generate a summary report of Cypher validation results.
pub fn format_summary(issues: &[CypherValidationIssue]) -> String {
    let errors = issues
        .iter()
        .filter(|i| i.severity == IssueSeverity::Error)
        .count();
    let warnings = issues
        .iter()
        .filter(|i| i.severity == IssueSeverity::Warning)
        .count();

    let mut by_rule: HashMap<CypherValidationRule, usize> = HashMap::new();
    for issue in issues {
        *by_rule.entry(issue.rule).or_insert(0) += 1;
    }

    let mut summary = String::new();
    summary.push_str(&format!(
        "\nCypher Validation Summary: {} errors, {} warnings\n",
        errors, warnings
    ));
    summary.push_str("─────────────────────────────────────────────────────\n");

    if !by_rule.is_empty() {
        summary.push_str("By Rule:\n");
        let mut rules: Vec<_> = by_rule.iter().collect();
        rules.sort_by(|a, b| b.1.cmp(a.1));
        for (rule, count) in rules {
            summary.push_str(&format!("  {:25} {}\n", rule.to_string(), count));
        }
    }

    summary
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_cypher_type_datetime() {
        assert_eq!(infer_cypher_type("datetime()"), "datetime");
        assert_eq!(infer_cypher_type("datetime('2024-01-01')"), "datetime");
        assert_eq!(infer_cypher_type("date('2024-01-01')"), "datetime");
    }

    #[test]
    fn test_infer_cypher_type_boolean() {
        assert_eq!(infer_cypher_type("true"), "boolean");
        assert_eq!(infer_cypher_type("false"), "boolean");
    }

    #[test]
    fn test_infer_cypher_type_string() {
        assert_eq!(infer_cypher_type("'hello world'"), "string");
        assert_eq!(infer_cypher_type("\"quoted\""), "string");
    }

    #[test]
    fn test_infer_cypher_type_int() {
        assert_eq!(infer_cypher_type("123"), "int");
        assert_eq!(infer_cypher_type("-456"), "int");
        assert_eq!(infer_cypher_type("0"), "int");
    }

    #[test]
    fn test_infer_cypher_type_float() {
        assert_eq!(infer_cypher_type("123.45"), "float");
        assert_eq!(infer_cypher_type("-0.5"), "float");
    }

    #[test]
    fn test_infer_cypher_type_array() {
        assert_eq!(infer_cypher_type("['a', 'b']"), "array");
        assert_eq!(infer_cypher_type("[1, 2, 3]"), "array");
    }

    #[test]
    fn test_infer_cypher_type_json() {
        assert_eq!(infer_cypher_type("{\"key\": \"value\"}"), "json");
    }

    #[test]
    fn test_types_compatible() {
        assert!(types_compatible("string", "string"));
        assert!(types_compatible("int", "int"));
        assert!(types_compatible("integer", "int"));
        assert!(types_compatible("datetime", "datetime"));
        assert!(types_compatible("float", "int")); // int can be float
        assert!(!types_compatible("int", "string"));
        assert!(!types_compatible("boolean", "int"));
    }

    #[test]
    fn test_parse_inline_properties() {
        let props_str = "key: 'test-key', display_name: 'Test Name'";
        let usages = parse_inline_properties(props_str, "Entity", Path::new("test.cypher"), 1);

        assert_eq!(usages.len(), 2);
        assert_eq!(usages[0].property, "key");
        assert_eq!(usages[0].label, "Entity");
        assert_eq!(usages[1].property, "display_name");
    }

    #[test]
    fn test_infer_label_from_context() {
        let content = "MERGE (e:Entity {key: 'test'})\nSET e.display_name = 'Test'";
        let label = infer_label_from_context(content, "e", 1);
        assert_eq!(label, "Entity");
    }

    #[test]
    fn test_infer_label_common_vars() {
        assert_eq!(infer_label_from_context("", "l", 0), "Locale");
        assert_eq!(infer_label_from_context("", "kw", 0), "SEOKeyword");
        assert_eq!(infer_label_from_context("", "bn", 0), "BlockNative");
    }

    fn test_root() -> Option<PathBuf> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn test_parse_cypher_file_integration() {
        let Some(root) = test_root() else { return };

        // Parse a known seed file
        let seed_file = root.join("packages/db/seed/20-locales.cypher");
        if !seed_file.exists() {
            return;
        }

        let usages = parse_cypher_file(&seed_file).expect("should parse cypher file");

        // Should find Locale properties
        let locale_usages: Vec<_> = usages.iter().filter(|u| u.label == "Locale").collect();

        assert!(!locale_usages.is_empty(), "should find Locale usages");

        // Check for expected properties
        let props: HashSet<_> = locale_usages.iter().map(|u| u.property.as_str()).collect();
        assert!(props.contains("key"), "should find key property");
        assert!(
            props.contains("display_name"),
            "should find display_name property"
        );
    }

    #[test]
    fn test_validate_cypher_files_integration() {
        let Some(root) = test_root() else { return };

        let issues = validate_cypher_files(&root, None).expect("should validate cypher files");

        // There may or may not be issues - just verify it runs
        // In a clean repo, there should be minimal issues
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .collect();

        // Print issues for debugging
        for issue in &errors {
            eprintln!(
                "[{}] {}: {} at {}:{}",
                issue.rule,
                issue.label,
                issue.message,
                issue.file.display(),
                issue.line.unwrap_or(0)
            );
        }

        // Test passes if validation completes without panicking
        // No assertion needed - reaching this point means success
    }
}
