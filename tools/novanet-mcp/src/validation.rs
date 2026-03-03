//! Input validation for NovaNet MCP write operations
//!
//! Validates class names, arc names, and other inputs to prevent
//! Cypher injection attacks.

use once_cell::sync::Lazy;
use regex::Regex;

/// Regex for valid Neo4j node class/label names: PascalCase, alphanumeric only
/// Must start with uppercase letter, followed by letters or digits
static CLASS_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z][A-Za-z0-9]*$").expect("Invalid regex"));

/// Regex for valid Neo4j relationship/arc class names: SCREAMING_SNAKE_CASE
/// Must start with uppercase letter, can contain uppercase letters, digits, and underscores
/// Examples: HAS_NATIVE, BELONGS_TO, TARGETS, HAS_CONTENT_1
static ARC_CLASS_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z][A-Z0-9_]*$").expect("Invalid regex"));

/// Validate a node class name for safe use in Cypher queries
///
/// Valid: Entity, EntityNative, SEOKeyword, A, ABC123
/// Invalid: entity, 123Entity, Entity-Native, Entity}
pub fn is_valid_class_name(name: &str) -> bool {
    !name.is_empty() && CLASS_NAME_REGEX.is_match(name)
}

/// Validate an arc class name for safe use in Cypher queries
///
/// Arc classes use SCREAMING_SNAKE_CASE with underscores allowed.
/// Valid: HAS_NATIVE, BELONGS_TO, TARGETS, A, ABC_123
/// Invalid: has_native, 123_ARC, ARC-CLASS, ARC}
pub fn is_valid_arc_class_name(name: &str) -> bool {
    !name.is_empty() && ARC_CLASS_NAME_REGEX.is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_class_names() {
        assert!(is_valid_class_name("Entity"));
        assert!(is_valid_class_name("EntityNative"));
        assert!(is_valid_class_name("SEOKeyword"));
        assert!(is_valid_class_name("A"));
        assert!(is_valid_class_name("ABC123"));
    }

    #[test]
    fn test_invalid_class_names() {
        assert!(!is_valid_class_name(""));
        assert!(!is_valid_class_name("entity")); // lowercase start
        assert!(!is_valid_class_name("123Entity")); // number start
        assert!(!is_valid_class_name("Entity-Native")); // hyphen
        assert!(!is_valid_class_name("Entity_Native")); // underscore
        assert!(!is_valid_class_name("Entity Native")); // space
        assert!(!is_valid_class_name("Entity}")); // injection attempt
        assert!(!is_valid_class_name("Entity]")); // bracket injection
        assert!(!is_valid_class_name("Entity:Foo")); // colon injection
    }

    #[test]
    fn test_cypher_injection_attempts() {
        // These are potential Cypher injection payloads that MUST be rejected
        assert!(!is_valid_class_name("Entity}DETACH DELETE"));
        assert!(!is_valid_class_name("a]//injection"));
        assert!(!is_valid_class_name("Entity]RETURN"));
        assert!(!is_valid_class_name("Entity\nRETURN"));
        assert!(!is_valid_class_name("Entity`"));
        assert!(!is_valid_class_name("Entity'"));
        assert!(!is_valid_class_name("Entity\""));
        assert!(!is_valid_class_name("Entity;DROP"));
        assert!(!is_valid_class_name("Entity--comment"));
    }

    #[test]
    fn test_valid_arc_class_names() {
        // SCREAMING_SNAKE_CASE is valid for arc classes
        assert!(is_valid_arc_class_name("HAS_NATIVE"));
        assert!(is_valid_arc_class_name("BELONGS_TO"));
        assert!(is_valid_arc_class_name("TARGETS"));
        assert!(is_valid_arc_class_name("HAS_CONTENT_1"));
        assert!(is_valid_arc_class_name("A"));
        assert!(is_valid_arc_class_name("ABC"));
    }

    #[test]
    fn test_invalid_arc_class_names() {
        assert!(!is_valid_arc_class_name("")); // empty
        assert!(!is_valid_arc_class_name("has_native")); // lowercase
        assert!(!is_valid_arc_class_name("Has_Native")); // mixed case
        assert!(!is_valid_arc_class_name("123_ARC")); // number start
        assert!(!is_valid_arc_class_name("ARC-CLASS")); // hyphen
        assert!(!is_valid_arc_class_name("HAS_NATIVE}RETURN")); // injection
        assert!(!is_valid_arc_class_name("HAS_NATIVE]")); // bracket injection
        assert!(!is_valid_arc_class_name("HAS_NATIVE:Foo")); // colon injection
        assert!(!is_valid_arc_class_name("arc class")); // space
    }
}
