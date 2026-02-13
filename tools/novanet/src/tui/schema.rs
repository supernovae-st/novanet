//! Schema property matching for TUI overlay.
//!
//! This module handles matching instance properties against Kind schema definitions,
//! enabling the Data mode overlay that shows:
//! - Filled properties (value present)
//! - Empty optional properties (dim)
//! - Missing required properties (red warning)
//!
//! Also provides Kind validation (Neo4j ↔ YAML source of truth).

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Property definition from YAML schema.
#[derive(Debug, Clone)]
pub struct SchemaProperty {
    /// Property name (e.g., "key", "display_name", "hemisphere")
    pub name: String,
    /// Property type (e.g., "string", "json", "datetime", "enum")
    pub prop_type: String,
    /// Whether this property is required
    pub required: bool,
    /// Example value from schema (for display when empty)
    pub example: Option<String>,
    /// Property description (parsed for future tooltip/detail view)
    #[allow(dead_code)]
    pub description: Option<String>,
    /// Enum values if prop_type is "enum" (parsed for future dropdown/validation)
    #[allow(dead_code)]
    pub enum_values: Option<Vec<String>>,
}

/// Matched property combining schema definition with actual instance value.
#[derive(Debug, Clone)]
pub struct MatchedProperty {
    /// Schema definition for this property
    pub schema: SchemaProperty,
    /// Actual value from instance (None = missing)
    pub value: Option<String>,
    /// Status for display coloring
    pub status: PropertyStatus,
}

/// Property status for visual encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyStatus {
    /// Has a value (display normally)
    Filled,
    /// No value, but not required (display dim)
    EmptyOptional,
    /// No value and required (display red with warning)
    MissingRequired,
}

/// Coverage statistics for an instance.
#[derive(Debug, Clone, Default)]
pub struct CoverageStats {
    /// Total properties in schema
    pub total: usize,
    /// Properties with values
    pub filled: usize,
    /// Required properties missing (used in tests)
    #[allow(dead_code)]
    pub missing_required: usize,
    /// Coverage percentage (0-100) — calculated for future progress bar display
    #[allow(dead_code)]
    pub percent: u8,
}

impl CoverageStats {
    /// Calculate coverage stats from matched properties.
    pub fn from_matched(props: &[MatchedProperty]) -> Self {
        let total = props.len();
        let filled = props
            .iter()
            .filter(|p| p.status == PropertyStatus::Filled)
            .count();
        let missing_required = props
            .iter()
            .filter(|p| p.status == PropertyStatus::MissingRequired)
            .count();
        let percent = if total > 0 {
            ((filled * 100) / total) as u8
        } else {
            100
        };

        Self {
            total,
            filled,
            missing_required,
            percent,
        }
    }
}

// =============================================================================
// Kind Property Validation (Neo4j ↔ YAML)
// =============================================================================

/// Validation status for Kind properties (Neo4j vs YAML source of truth).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    /// Property exists in both YAML schema and Neo4j Kind node
    Sync,
    /// YAML defines this property, but Neo4j Kind node is missing it
    Missing,
    /// Neo4j Kind node has this property, but not defined in YAML schema
    Extra,
}

/// Validated property combining YAML schema with Neo4j validation status.
#[derive(Debug, Clone)]
pub struct ValidatedProperty {
    /// Property name
    pub name: String,
    /// Property type from YAML (e.g., "string", "boolean", "datetime")
    /// For Extra properties, this will be "?"
    pub prop_type: String,
    /// Whether this property is required (from YAML)
    pub required: bool,
    /// Example value from YAML schema
    pub example: Option<String>,
    /// Validation status (Sync, Missing, or Extra)
    pub status: ValidationStatus,
}

/// Validation statistics for a Kind.
#[derive(Debug, Clone, Default)]
pub struct ValidationStats {
    /// Properties in sync (YAML = Neo4j)
    pub sync_count: usize,
    /// Properties missing in Neo4j
    pub missing_count: usize,
    /// Extra properties in Neo4j (not in YAML)
    pub extra_count: usize,
}

impl ValidationStats {
    /// Calculate validation stats from validated properties.
    /// Uses single-pass iteration for efficiency.
    pub fn from_validated(props: &[ValidatedProperty]) -> Self {
        let (sync_count, missing_count, extra_count) =
            props.iter().fold((0, 0, 0), |(s, m, e), p| match p.status {
                ValidationStatus::Sync => (s + 1, m, e),
                ValidationStatus::Missing => (s, m + 1, e),
                ValidationStatus::Extra => (s, m, e + 1),
            });

        Self {
            sync_count,
            missing_count,
            extra_count,
        }
    }
}

// =============================================================================
// YAML Parsing Structures
// =============================================================================

/// Raw YAML property definition (for deserialization).
#[derive(Debug, Deserialize)]
struct YamlProperty {
    #[serde(rename = "type", default)]
    prop_type: Option<String>,
    #[serde(default)]
    required: Option<bool>,
    #[serde(default)]
    example: Option<serde_yaml::Value>,
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "enum", default)]
    enum_values: Option<Vec<String>>,
}

/// Raw YAML node structure (for deserialization).
/// Uses IndexMap to preserve property order from YAML files.
#[derive(Debug, Deserialize)]
struct YamlNode {
    #[serde(default)]
    standard_properties: Option<IndexMap<String, YamlProperty>>,
    #[serde(default)]
    properties: Option<IndexMap<String, YamlProperty>>,
}

/// Root YAML structure.
#[derive(Debug, Deserialize)]
struct YamlRoot {
    #[serde(default)]
    node: Option<YamlNode>,
}

// =============================================================================
// Public Functions
// =============================================================================

/// Load schema properties from a YAML file.
/// Returns properties in definition order (standard_properties first, then properties).
pub fn load_schema_properties(root_path: &str, yaml_path: &str) -> Vec<SchemaProperty> {
    let full_path = Path::new(root_path).join(yaml_path);

    let content = match fs::read_to_string(&full_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    parse_schema_properties(&content)
}

/// Parse schema properties from YAML content string.
pub fn parse_schema_properties(content: &str) -> Vec<SchemaProperty> {
    let root: YamlRoot = match serde_yaml::from_str(content) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };

    let Some(node) = root.node else {
        return Vec::new();
    };

    let mut properties = Vec::new();

    // Process standard_properties first (these are typically required)
    if let Some(std_props) = node.standard_properties {
        for (name, prop) in std_props {
            properties.push(yaml_to_schema_property(name, prop));
        }
    }

    // Then process custom properties
    if let Some(custom_props) = node.properties {
        for (name, prop) in custom_props {
            properties.push(yaml_to_schema_property(name, prop));
        }
    }

    properties
}

/// Convert YAML property to schema property.
fn yaml_to_schema_property(name: String, prop: YamlProperty) -> SchemaProperty {
    // Convert example to string representation
    let example = prop.example.map(|v| match v {
        serde_yaml::Value::String(s) => s,
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Sequence(seq) => {
            // Format as JSON array
            let items: Vec<String> = seq.into_iter().map(|v| format!("{:?}", v)).collect();
            format!("[{}]", items.join(", "))
        }
        serde_yaml::Value::Mapping(map) => {
            // Format as JSON object (simplified)
            let items: Vec<String> = map
                .into_iter()
                .map(|(k, v)| format!("{:?}: {:?}", k, v))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
        serde_yaml::Value::Null => "null".to_string(),
        serde_yaml::Value::Tagged(t) => format!("{:?}", t.value),
    });

    SchemaProperty {
        name,
        prop_type: prop.prop_type.unwrap_or_else(|| "string".to_string()),
        required: prop.required.unwrap_or(false),
        example,
        description: prop.description,
        enum_values: prop.enum_values,
    }
}

/// Match instance properties against schema.
/// Returns matched properties in schema order with status indicators.
pub fn match_properties(
    schema: &[SchemaProperty],
    instance_props: &BTreeMap<String, JsonValue>,
) -> Vec<MatchedProperty> {
    schema
        .iter()
        .map(|prop| {
            let value = instance_props.get(&prop.name).map(json_value_to_string);
            let has_value = value.as_ref().is_some_and(|v| !v.is_empty() && v != "null");

            let status = if has_value {
                PropertyStatus::Filled
            } else if prop.required {
                PropertyStatus::MissingRequired
            } else {
                PropertyStatus::EmptyOptional
            };

            MatchedProperty {
                schema: prop.clone(),
                value: if has_value { value } else { None },
                status,
            }
        })
        .collect()
}

/// Convert JSON value to display string.
fn json_value_to_string(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => String::new(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => s.clone(),
        JsonValue::Array(arr) => {
            // Compact array representation
            serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string())
        }
        JsonValue::Object(obj) => {
            // Compact object representation
            serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string())
        }
    }
}

// =============================================================================
// Kind Validation Functions
// =============================================================================

/// Validate Kind properties by comparing YAML schema against Neo4j state.
///
/// Returns validated properties showing:
/// - Sync: property exists in both YAML and Neo4j
/// - Missing: YAML defines it, but Neo4j doesn't have it
/// - Extra: Neo4j has it, but not in YAML schema
pub fn validate_kind_properties(
    schema: &[SchemaProperty],
    neo4j_properties: &[String],
) -> Vec<ValidatedProperty> {
    use rustc_hash::FxHashSet; // 30% faster than std::collections::HashSet for strings

    let neo4j_set: FxHashSet<&str> = neo4j_properties.iter().map(|s| s.as_str()).collect();
    let yaml_set: FxHashSet<&str> = schema.iter().map(|p| p.name.as_str()).collect();

    // Pre-allocate with estimated capacity (schema + potential extras)
    let mut validated = Vec::with_capacity(schema.len() + 8);

    // First: YAML properties (in schema order) - mark as Sync or Missing
    for prop in schema {
        let status = if neo4j_set.contains(prop.name.as_str()) {
            ValidationStatus::Sync
        } else {
            ValidationStatus::Missing
        };

        validated.push(ValidatedProperty {
            name: prop.name.clone(),
            prop_type: prop.prop_type.clone(),
            required: prop.required,
            example: prop.example.clone(),
            status,
        });
    }

    // Second: Extra properties (in Neo4j but not in YAML)
    // Collect and sort for deterministic UI order
    let mut extra_props: Vec<_> = neo4j_properties
        .iter()
        .filter(|p| !yaml_set.contains(p.as_str()))
        .collect();
    extra_props.sort(); // Alphabetical order for consistent display

    for neo4j_prop in extra_props {
        validated.push(ValidatedProperty {
            name: neo4j_prop.clone(),
            prop_type: "?".to_string(),
            required: false,
            example: None,
            status: ValidationStatus::Extra,
        });
    }

    validated
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_YAML: &str = r#"
node:
  name: Culture
  realm: shared
  layer: config
  trait: imported  # v0.12.0 ADR-024: knowledge→imported

  standard_properties:
    key:
      type: string
      required: true
      description: "Unique identifier"
      example: "fr-FR"
    display_name:
      type: string
      required: true
      example: "French (France) Culture"
    description:
      type: string
      required: true
      example: "Cultural context for fr-FR"

  properties:
    hemisphere:
      type: string
      required: true
      enum: ["northern", "southern"]
      example: "northern"
    holidays:
      type: json
      required: false
      example: []
    seasonal_greetings:
      type: string
      required: false
      example: "Joyeux Noel"
"#;

    #[test]
    fn test_parse_schema_properties() {
        let props = parse_schema_properties(SAMPLE_YAML);

        assert_eq!(props.len(), 6);

        // Check properties are in YAML definition order (not alphabetical!)
        // standard_properties come first, then properties
        assert_eq!(props[0].name, "key"); // 1st in standard_properties
        assert_eq!(props[1].name, "display_name"); // 2nd in standard_properties
        assert_eq!(props[2].name, "description"); // 3rd in standard_properties
        assert_eq!(props[3].name, "hemisphere"); // 1st in properties
        assert_eq!(props[4].name, "holidays"); // 2nd in properties
        assert_eq!(props[5].name, "seasonal_greetings"); // 3rd in properties

        // Verify required flags
        assert!(props[0].required); // key
        assert!(props[3].required); // hemisphere

        // Verify enum values
        assert_eq!(props[3].enum_values.as_ref().unwrap().len(), 2);

        // Verify types
        assert_eq!(props[4].prop_type, "json"); // holidays
    }

    #[test]
    fn test_match_properties() {
        let props = parse_schema_properties(SAMPLE_YAML);

        let mut instance = BTreeMap::new();
        instance.insert("key".to_string(), JsonValue::String("af-ZA".to_string()));
        instance.insert(
            "display_name".to_string(),
            JsonValue::String("Afrikaans".to_string()),
        );
        instance.insert(
            "hemisphere".to_string(),
            JsonValue::String("southern".to_string()),
        );
        // description and holidays missing

        let matched = match_properties(&props, &instance);

        // Count statuses
        let filled = matched
            .iter()
            .filter(|p| p.status == PropertyStatus::Filled)
            .count();
        let missing_req = matched
            .iter()
            .filter(|p| p.status == PropertyStatus::MissingRequired)
            .count();
        let empty_opt = matched
            .iter()
            .filter(|p| p.status == PropertyStatus::EmptyOptional)
            .count();

        assert_eq!(filled, 3); // key, display_name, hemisphere
        assert_eq!(missing_req, 1); // description (required but missing)
        assert_eq!(empty_opt, 2); // holidays, seasonal_greetings
    }

    #[test]
    fn test_coverage_stats() {
        let props = parse_schema_properties(SAMPLE_YAML);

        let mut instance = BTreeMap::new();
        instance.insert("key".to_string(), JsonValue::String("af-ZA".to_string()));
        instance.insert(
            "display_name".to_string(),
            JsonValue::String("Afrikaans".to_string()),
        );
        instance.insert(
            "description".to_string(),
            JsonValue::String("Test".to_string()),
        );
        instance.insert(
            "hemisphere".to_string(),
            JsonValue::String("southern".to_string()),
        );

        let matched = match_properties(&props, &instance);
        let stats = CoverageStats::from_matched(&matched);

        assert_eq!(stats.total, 6);
        assert_eq!(stats.filled, 4);
        assert_eq!(stats.missing_required, 0);
        assert_eq!(stats.percent, 66); // 4/6 = 66%
    }

    // =========================================================================
    // PropertyStatus Tests
    // =========================================================================

    /// Helper to create a SchemaProperty for testing.
    fn make_schema_property(name: &str, required: bool) -> SchemaProperty {
        SchemaProperty {
            name: name.to_string(),
            prop_type: "string".to_string(),
            required,
            example: Some("example_value".to_string()),
            description: Some("Test property".to_string()),
            enum_values: None,
        }
    }

    #[test]
    fn test_property_status_filled() {
        let schema = make_schema_property("test_prop", true);
        let matched = MatchedProperty {
            schema,
            value: Some("actual_value".to_string()),
            status: PropertyStatus::Filled,
        };

        assert_eq!(matched.status, PropertyStatus::Filled);
        assert!(matched.value.is_some());
        assert_eq!(matched.value.as_ref().unwrap(), "actual_value");
        assert!(matched.schema.required);
    }

    #[test]
    fn test_property_status_empty_optional() {
        let schema = make_schema_property("optional_prop", false);
        let matched = MatchedProperty {
            schema,
            value: None,
            status: PropertyStatus::EmptyOptional,
        };

        assert_eq!(matched.status, PropertyStatus::EmptyOptional);
        assert!(matched.value.is_none());
        assert!(!matched.schema.required);
    }

    #[test]
    fn test_property_status_missing_required() {
        let schema = make_schema_property("required_prop", true);
        let matched = MatchedProperty {
            schema,
            value: None,
            status: PropertyStatus::MissingRequired,
        };

        assert_eq!(matched.status, PropertyStatus::MissingRequired);
        assert!(matched.value.is_none());
        assert!(matched.schema.required);
    }

    // =========================================================================
    // CoverageStats Unit Tests (Task 4.2)
    // =========================================================================

    /// Helper to create a MatchedProperty with given status for CoverageStats tests.
    fn make_matched_for_coverage(name: &str, status: PropertyStatus) -> MatchedProperty {
        MatchedProperty {
            schema: SchemaProperty {
                name: name.to_string(),
                prop_type: "string".to_string(),
                required: status == PropertyStatus::MissingRequired,
                example: None,
                description: None,
                enum_values: None,
            },
            value: if status == PropertyStatus::Filled {
                Some("value".to_string())
            } else {
                None
            },
            status,
        }
    }

    #[test]
    fn test_coverage_stats_all_filled() {
        let props = vec![
            make_matched_for_coverage("prop1", PropertyStatus::Filled),
            make_matched_for_coverage("prop2", PropertyStatus::Filled),
        ];

        let stats = CoverageStats::from_matched(&props);

        assert_eq!(stats.total, 2);
        assert_eq!(stats.filled, 2);
        assert_eq!(stats.missing_required, 0);
        assert_eq!(stats.percent, 100);
    }

    #[test]
    fn test_coverage_stats_partial() {
        let props = vec![
            make_matched_for_coverage("prop1", PropertyStatus::Filled),
            make_matched_for_coverage("prop2", PropertyStatus::MissingRequired),
        ];

        let stats = CoverageStats::from_matched(&props);

        assert_eq!(stats.total, 2);
        assert_eq!(stats.filled, 1);
        assert_eq!(stats.missing_required, 1);
        assert_eq!(stats.percent, 50);
    }

    #[test]
    fn test_coverage_stats_empty_list() {
        let props: Vec<MatchedProperty> = vec![];

        let stats = CoverageStats::from_matched(&props);

        assert_eq!(stats.total, 0);
        assert_eq!(stats.filled, 0);
        assert_eq!(stats.missing_required, 0);
        assert_eq!(stats.percent, 100); // Convention: empty = 100%
    }
}
