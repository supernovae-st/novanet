//! Schema property matching for TUI overlay.
//!
//! This module handles matching instance properties against Kind schema definitions,
//! enabling the Data mode overlay that shows:
//! - Filled properties (value present)
//! - Empty optional properties (dim)
//! - Missing required properties (red warning)

#![allow(dead_code)] // WIP: Schema overlay implementation

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
    /// Property description
    pub description: Option<String>,
    /// Enum values if prop_type is "enum"
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
    /// Required properties missing
    pub missing_required: usize,
    /// Coverage percentage (0-100)
    pub percent: u8,
}

impl CoverageStats {
    /// Calculate coverage stats from matched properties.
    pub fn from_matched(props: &[MatchedProperty]) -> Self {
        let total = props.len();
        let filled = props.iter().filter(|p| p.status == PropertyStatus::Filled).count();
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
#[derive(Debug, Deserialize)]
struct YamlNode {
    #[serde(default)]
    standard_properties: Option<BTreeMap<String, YamlProperty>>,
    #[serde(default)]
    properties: Option<BTreeMap<String, YamlProperty>>,
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
            let items: Vec<String> = seq
                .into_iter()
                .map(|v| format!("{:?}", v))
                .collect();
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
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_YAML: &str = r#"
node:
  name: Culture
  realm: global
  layer: config
  trait: knowledge

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

        // Check standard_properties come first
        assert_eq!(props[0].name, "description");
        assert!(props[0].required);

        // Check properties section
        let hemisphere = props.iter().find(|p| p.name == "hemisphere").unwrap();
        assert!(hemisphere.required);
        assert_eq!(hemisphere.enum_values.as_ref().unwrap().len(), 2);

        let holidays = props.iter().find(|p| p.name == "holidays").unwrap();
        assert!(!holidays.required);
        assert_eq!(holidays.prop_type, "json");
    }

    #[test]
    fn test_match_properties() {
        let props = parse_schema_properties(SAMPLE_YAML);

        let mut instance = BTreeMap::new();
        instance.insert("key".to_string(), JsonValue::String("af-ZA".to_string()));
        instance.insert("display_name".to_string(), JsonValue::String("Afrikaans".to_string()));
        instance.insert("hemisphere".to_string(), JsonValue::String("southern".to_string()));
        // description and holidays missing

        let matched = match_properties(&props, &instance);

        // Count statuses
        let filled = matched.iter().filter(|p| p.status == PropertyStatus::Filled).count();
        let missing_req = matched.iter().filter(|p| p.status == PropertyStatus::MissingRequired).count();
        let empty_opt = matched.iter().filter(|p| p.status == PropertyStatus::EmptyOptional).count();

        assert_eq!(filled, 3); // key, display_name, hemisphere
        assert_eq!(missing_req, 1); // description (required but missing)
        assert_eq!(empty_opt, 2); // holidays, seasonal_greetings
    }

    #[test]
    fn test_coverage_stats() {
        let props = parse_schema_properties(SAMPLE_YAML);

        let mut instance = BTreeMap::new();
        instance.insert("key".to_string(), JsonValue::String("af-ZA".to_string()));
        instance.insert("display_name".to_string(), JsonValue::String("Afrikaans".to_string()));
        instance.insert("description".to_string(), JsonValue::String("Test".to_string()));
        instance.insert("hemisphere".to_string(), JsonValue::String("southern".to_string()));

        let matched = match_properties(&props, &instance);
        let stats = CoverageStats::from_matched(&matched);

        assert_eq!(stats.total, 6);
        assert_eq!(stats.filled, 4);
        assert_eq!(stats.missing_required, 0);
        assert_eq!(stats.percent, 66); // 4/6 = 66%
    }
}
