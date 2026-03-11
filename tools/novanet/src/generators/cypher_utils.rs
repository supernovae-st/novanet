//! Shared Cypher formatting utilities for seed generators.
//!
//! Provides common helper functions used across organizing.rs, kind.rs, and arc_schema.rs
//! to avoid code duplication.

#![allow(clippy::needless_raw_string_hashes)]

use serde::Serialize;
use std::fmt::Write;

// ─────────────────────────────────────────────────────────────────────────────
// JSON Serialization Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Serialize data to JSON string with fallback value on failure.
///
/// Use this instead of `serde_json::to_string(...).unwrap_or_else(|_| "...".to_string())`
/// to reduce boilerplate.
///
/// # Examples
/// ```ignore
/// let json = serialize_json(&data, "{}");     // Object fallback
/// let json = serialize_json(&items, "[]");    // Array fallback
/// let json = serialize_json(&value, "null");  // Nullable fallback
/// ```
pub fn serialize_json<T: Serialize>(data: &T, fallback: &str) -> String {
    serde_json::to_string(data).unwrap_or_else(|_| fallback.to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// String Formatting
// ─────────────────────────────────────────────────────────────────────────────

/// Escape single quotes and newlines for Cypher strings.
///
/// Use this when you want to preserve the original structure but escape special characters.
/// For collapsing whitespace, use `cypher_str` instead.
///
/// # Example
/// ```ignore
/// assert_eq!(escape_cypher("it's\nok"), "it\\'s\\nok");
/// ```
pub fn escape_cypher(s: &str) -> String {
    s.replace('\'', "\\'").replace('\n', "\\n")
}

/// Collapse multiline YAML strings into single-line Cypher values and escape single quotes.
///
/// # Example
/// ```ignore
/// let input = "Line one\n  Line two";
/// assert_eq!(cypher_str(input), "Line one Line two");
/// ```
pub fn cypher_str(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\'', "\\'")
}

/// Format a slice of string references as a Cypher list literal: `['a', 'b', 'c']`.
///
/// Each element is escaped via `cypher_str`.
pub fn cypher_list_ref(items: &[&str]) -> String {
    if items.is_empty() {
        return "[]".to_string();
    }
    let inner: Vec<String> = items
        .iter()
        .map(|s| format!("'{}'", cypher_str(s)))
        .collect();
    format!("[{}]", inner.join(", "))
}

/// Format a slice of owned strings as a Cypher list literal: `['a', 'b', 'c']`.
///
/// Each element is escaped via `cypher_str`.
pub fn cypher_list_owned(items: &[String]) -> String {
    if items.is_empty() {
        return "[]".to_string();
    }
    let inner: Vec<String> = items
        .iter()
        .map(|s| format!("'{}'", cypher_str(s)))
        .collect();
    format!("[{}]", inner.join(", "))
}

// ─────────────────────────────────────────────────────────────────────────────
// MERGE Statement Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Write a MERGE statement for a `:Schema:<Label>` node with ON CREATE/ON MATCH SET.
///
/// Properties are formatted as `{var}.{name} = '{value}'`.
/// v0.19.0 (ADR-037): Adds `node_class` (lowercase of label) for SCHEMA nodes.
/// Automatically adds `created_by` (provenance), `created_at` on CREATE and `updated_at` on MATCH.
pub fn write_merge_meta(
    out: &mut String,
    var: &str,
    label: &str,
    key: &str,
    props: &[(&str, &str)],
) {
    // v0.19.0 (ADR-037): node_class is lowercase for SCHEMA nodes
    // Convert PascalCase/CamelCase to snake_case for node_class
    let node_class = to_snake_case(label);

    writeln!(out, "MERGE ({var}:Schema:{label} {{key: '{key}'}})").unwrap();

    writeln!(out, "ON CREATE SET").unwrap();
    for (name, value) in props {
        writeln!(out, "  {var}.{name} = '{value}',").unwrap();
    }
    // v0.19.0 (ADR-037): node_class discriminator (lowercase = SCHEMA node)
    writeln!(out, "  {var}.node_class = '{node_class}',").unwrap();
    // v0.17.3 (ADR-036): Add provenance tracking
    writeln!(out, "  {var}.created_by = 'seed:schema',").unwrap();
    writeln!(out, "  {var}.created_at = datetime()").unwrap();

    writeln!(out, "ON MATCH SET").unwrap();
    for (name, value) in props {
        writeln!(out, "  {var}.{name} = '{value}',").unwrap();
    }
    // v0.19.0 (ADR-037): Always set node_class on match too
    writeln!(out, "  {var}.node_class = '{node_class}',").unwrap();
    writeln!(out, "  {var}.updated_at = datetime();").unwrap();
}

/// Convert PascalCase or CamelCase to snake_case.
///
/// # Examples
/// ```ignore
/// assert_eq!(to_snake_case("Realm"), "realm");
/// assert_eq!(to_snake_case("ArcClass"), "arc_class");
/// assert_eq!(to_snake_case("ArcCardinality"), "arc_cardinality");
/// ```
fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Write a section header comment in Cypher format.
pub fn write_section_header(out: &mut String, title: &str) {
    writeln!(
        out,
        "// ─────────────────────────────────────────────────────────────────────────────"
    )
    .unwrap();
    writeln!(out, "// {title}").unwrap();
    writeln!(
        out,
        "// ─────────────────────────────────────────────────────────────────────────────"
    )
    .unwrap();
    writeln!(out).unwrap();
}

/// Write a visual section header with item count (double-line style).
///
/// # Example
/// ```ignore
/// write_section_header_counted(&mut out, "Class Nodes", 58);
/// // Output:
/// // ═══════════════════════════════════════════════════════════════════════════════
/// // Class Nodes (58)
/// // ═══════════════════════════════════════════════════════════════════════════════
/// ```
pub fn write_section_header_counted(out: &mut String, title: &str, count: usize) {
    let bar = "// ═══════════════════════════════════════════════════════════════════════════════";
    writeln!(out, "{bar}").unwrap();
    writeln!(out, "// {title} ({count})").unwrap();
    writeln!(out, "{bar}").unwrap();
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn cypher_str_collapses_whitespace() {
        let input = "Line one\n  Line two\n    Line three";
        assert_eq!(cypher_str(input), "Line one Line two Line three");
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Property-Based Tests (proptest)
    // ─────────────────────────────────────────────────────────────────────────────

    proptest! {
        /// cypher_str should never produce output containing unescaped single quotes.
        #[test]
        fn prop_cypher_str_escapes_all_quotes(s in ".*") {
            let result = cypher_str(&s);
            // Count unescaped quotes (not preceded by backslash)
            let unescaped_quotes: Vec<_> = result
                .chars()
                .enumerate()
                .filter(|(i, c)| *c == '\'' && (*i == 0 || result.chars().nth(i - 1) != Some('\\')))
                .collect();
            prop_assert!(unescaped_quotes.is_empty(), "Found unescaped quotes in: {result}");
        }

        /// cypher_str output should never contain newlines or multiple consecutive spaces.
        #[test]
        fn prop_cypher_str_no_newlines(s in ".*") {
            let result = cypher_str(&s);
            prop_assert!(!result.contains('\n'), "Output contains newline: {result}");
            prop_assert!(!result.contains("  "), "Output contains double space: {result}");
        }

        /// cypher_list_ref should produce valid Cypher list syntax.
        #[test]
        fn prop_cypher_list_valid_syntax(items in prop::collection::vec("\\PC*", 0..10)) {
            let refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
            let result = cypher_list_ref(&refs);

            // Must start with [ and end with ]
            prop_assert!(result.starts_with('['), "List doesn't start with [: {result}");
            prop_assert!(result.ends_with(']'), "List doesn't end with ]: {result}");

            // Empty list is exactly "[]"
            if items.is_empty() {
                prop_assert_eq!(result, "[]");
            }
        }

        /// cypher_list_owned should match cypher_list_ref behavior.
        #[test]
        fn prop_cypher_list_owned_matches_ref(items in prop::collection::vec("[a-zA-Z0-9_]*", 0..5)) {
            let refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
            let result_ref = cypher_list_ref(&refs);
            let result_owned = cypher_list_owned(&items);
            prop_assert_eq!(result_ref, result_owned);
        }
    }

    #[test]
    fn cypher_str_escapes_single_quotes() {
        assert_eq!(cypher_str("It's a test"), "It\\'s a test");
    }

    #[test]
    fn cypher_str_preserves_simple_text() {
        assert_eq!(cypher_str("simple text"), "simple text");
    }

    #[test]
    fn cypher_list_ref_empty() {
        assert_eq!(cypher_list_ref(&[]), "[]");
    }

    #[test]
    fn cypher_list_ref_single() {
        assert_eq!(cypher_list_ref(&["Project"]), "['Project']");
    }

    #[test]
    fn cypher_list_ref_multiple() {
        assert_eq!(cypher_list_ref(&["Page", "Block"]), "['Page', 'Block']");
    }

    #[test]
    fn cypher_list_ref_escapes_quotes() {
        assert_eq!(cypher_list_ref(&["It's"]), "['It\\'s']");
    }

    #[test]
    fn cypher_list_owned_empty() {
        let items: Vec<String> = vec![];
        assert_eq!(cypher_list_owned(&items), "[]");
    }

    #[test]
    fn cypher_list_owned_multiple() {
        let items = vec!["Page".to_string(), "Block".to_string()];
        assert_eq!(cypher_list_owned(&items), "['Page', 'Block']");
    }

    #[test]
    fn write_merge_meta_format() {
        let mut out = String::new();
        write_merge_meta(
            &mut out,
            "r",
            "Realm",
            "shared",
            &[("display_name", "Shared"), ("emoji", "globe")],
        );
        assert!(out.contains("MERGE (r:Schema:Realm {key: 'shared'})"));
        assert!(out.contains("r.display_name = 'Shared'"));
        assert!(out.contains("r.emoji = 'globe'"));
        // v0.19.0 (ADR-037): node_class discriminator
        assert!(out.contains("r.node_class = 'realm'"));
        // v0.17.3 (ADR-036): Provenance tracking
        assert!(out.contains("r.created_by = 'seed:schema'"));
        assert!(out.contains("created_at = datetime()"));
        assert!(out.contains("updated_at = datetime()"));
    }

    #[test]
    fn to_snake_case_simple() {
        assert_eq!(to_snake_case("Realm"), "realm");
        assert_eq!(to_snake_case("Layer"), "layer");
        assert_eq!(to_snake_case("Class"), "class");
    }

    #[test]
    fn to_snake_case_multi_word() {
        assert_eq!(to_snake_case("ArcClass"), "arc_class");
        assert_eq!(to_snake_case("ArcFamily"), "arc_family");
        assert_eq!(to_snake_case("ArcScope"), "arc_scope");
        assert_eq!(to_snake_case("ArcCardinality"), "arc_cardinality");
    }

    #[test]
    fn write_section_header_format() {
        let mut out = String::new();
        write_section_header(&mut out, "TEST SECTION");
        assert!(out.contains("// TEST SECTION"));
        assert!(out.contains("// ─────"));
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // serialize_json Tests
    // ─────────────────────────────────────────────────────────────────────────────

    #[test]
    fn serialize_json_object() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("key", "value");
        let result = serialize_json(&map, "{}");
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }

    #[test]
    fn serialize_json_array() {
        let items = vec!["a", "b", "c"];
        let result = serialize_json(&items, "[]");
        assert_eq!(result, r#"["a","b","c"]"#);
    }

    #[test]
    fn serialize_json_empty_vec() {
        let items: Vec<String> = vec![];
        let result = serialize_json(&items, "[]");
        assert_eq!(result, "[]");
    }

    #[test]
    fn serialize_json_null_fallback() {
        // Option::None serializes to "null"
        let opt: Option<String> = None;
        let result = serialize_json(&opt, "null");
        assert_eq!(result, "null");
    }
}
