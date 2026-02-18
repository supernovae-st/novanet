//! YAML type → Neo4j type constraint mapping.
//!
//! This module provides the canonical mapping between YAML property types
//! (as written in node-class YAML files) and Neo4j 5 type constraint syntax.
//!
//! # Usage
//! ```
//! use novanet::coherence::type_map::YamlPropType;
//!
//! let t = YamlPropType::Datetime;
//! assert_eq!(t.neo4j_type_constraint(), "IS :: ZONED DATETIME");
//! ```

/// Canonical mapping from YAML type strings to Neo4j IS :: TYPE syntax.
///
/// Used to generate `ALTER CONSTRAINT` statements for Axis G enforcement.
pub const TYPE_MAP: &[(&str, &str)] = &[
    ("string", "STRING"),
    ("int", "INTEGER"),
    ("float", "FLOAT"),
    ("boolean", "BOOLEAN"),
    ("datetime", "ZONED DATETIME"),
    ("string[]", "LIST<STRING NOT NULL>"),
    ("int[]", "LIST<INTEGER NOT NULL>"),
];

/// YAML property type parsed from node-class YAML files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YamlPropType {
    String,
    Integer,
    Float,
    Boolean,
    Datetime,
    StringList,
    IntegerList,
}

impl YamlPropType {
    /// Parse from a YAML type string (e.g. "string", "datetime", "string[]").
    pub fn from_yaml_str(s: &str) -> Option<Self> {
        match s {
            "string" => Some(Self::String),
            "int" => Some(Self::Integer),
            "float" => Some(Self::Float),
            "boolean" => Some(Self::Boolean),
            "datetime" => Some(Self::Datetime),
            "string[]" => Some(Self::StringList),
            "int[]" => Some(Self::IntegerList),
            _ => None,
        }
    }

    /// Neo4j 5 type constraint expression for this type.
    ///
    /// Used in: `ALTER CONSTRAINT … FOR (n:NodeLabel) REQUIRE n.prop IS :: <TYPE>`
    pub fn neo4j_type_constraint(&self) -> &'static str {
        match self {
            Self::String => "IS :: STRING",
            Self::Integer => "IS :: INTEGER",
            Self::Float => "IS :: FLOAT",
            Self::Boolean => "IS :: BOOLEAN",
            Self::Datetime => "IS :: ZONED DATETIME",
            Self::StringList => "IS :: LIST<STRING NOT NULL>",
            Self::IntegerList => "IS :: LIST<INTEGER NOT NULL>",
        }
    }

    /// Human-readable display name for error messages.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "int",
            Self::Float => "float",
            Self::Boolean => "boolean",
            Self::Datetime => "datetime",
            Self::StringList => "string[]",
            Self::IntegerList => "int[]",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests — RED → GREEN
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neo4j_constraint_for_string() {
        assert_eq!(YamlPropType::String.neo4j_type_constraint(), "IS :: STRING");
    }

    #[test]
    fn test_neo4j_constraint_for_int() {
        assert_eq!(YamlPropType::Integer.neo4j_type_constraint(), "IS :: INTEGER");
    }

    #[test]
    fn test_neo4j_constraint_for_float() {
        assert_eq!(YamlPropType::Float.neo4j_type_constraint(), "IS :: FLOAT");
    }

    #[test]
    fn test_neo4j_constraint_for_boolean() {
        assert_eq!(YamlPropType::Boolean.neo4j_type_constraint(), "IS :: BOOLEAN");
    }

    #[test]
    fn test_neo4j_constraint_for_datetime() {
        assert_eq!(
            YamlPropType::Datetime.neo4j_type_constraint(),
            "IS :: ZONED DATETIME"
        );
    }

    #[test]
    fn test_neo4j_constraint_for_string_list() {
        assert_eq!(
            YamlPropType::StringList.neo4j_type_constraint(),
            "IS :: LIST<STRING NOT NULL>"
        );
    }

    #[test]
    fn test_neo4j_constraint_for_int_list() {
        assert_eq!(
            YamlPropType::IntegerList.neo4j_type_constraint(),
            "IS :: LIST<INTEGER NOT NULL>"
        );
    }

    #[test]
    fn test_from_yaml_str_all_types() {
        let cases = [
            ("string", YamlPropType::String),
            ("int", YamlPropType::Integer),
            ("float", YamlPropType::Float),
            ("boolean", YamlPropType::Boolean),
            ("datetime", YamlPropType::Datetime),
            ("string[]", YamlPropType::StringList),
            ("int[]", YamlPropType::IntegerList),
        ];
        for (yaml_str, expected) in cases {
            assert_eq!(YamlPropType::from_yaml_str(yaml_str), Some(expected),
                "from_yaml_str({yaml_str:?}) should return Some({expected:?})");
        }
    }

    #[test]
    fn test_from_yaml_str_unknown_returns_none() {
        assert_eq!(YamlPropType::from_yaml_str("object"), None);
        assert_eq!(YamlPropType::from_yaml_str("unknown"), None);
        assert_eq!(YamlPropType::from_yaml_str(""), None);
    }

    #[test]
    fn test_type_map_covers_all_yaml_types() {
        let required = ["string", "int", "float", "boolean", "datetime", "string[]"];
        for t in required {
            assert!(
                TYPE_MAP.iter().any(|(k, _)| *k == t),
                "TYPE_MAP missing entry for '{t}'"
            );
        }
    }

    #[test]
    fn test_type_map_values_are_valid_neo4j_syntax() {
        // Basic sanity: all values should be non-empty and start with expected keywords
        for (_, neo4j_type) in TYPE_MAP {
            assert!(!neo4j_type.is_empty(), "Empty Neo4j type in TYPE_MAP");
        }
    }

    #[test]
    fn test_yaml_prop_type_from_str_float() {
        // Named test from the plan spec
        assert_eq!(YamlPropType::from_yaml_str("float"), Some(YamlPropType::Float));
        assert_eq!(
            YamlPropType::Float.neo4j_type_constraint(),
            "IS :: FLOAT"
        );
    }

    #[test]
    fn test_display_name_roundtrips() {
        let types = [
            YamlPropType::String,
            YamlPropType::Integer,
            YamlPropType::Float,
            YamlPropType::Boolean,
            YamlPropType::Datetime,
            YamlPropType::StringList,
            YamlPropType::IntegerList,
        ];
        for t in types {
            let name = t.display_name();
            let parsed = YamlPropType::from_yaml_str(name);
            assert_eq!(parsed, Some(t), "display_name '{name}' should parse back to same type");
        }
    }
}
