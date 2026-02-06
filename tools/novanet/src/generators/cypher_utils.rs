//! Shared Cypher formatting utilities for seed generators.
//!
//! Provides common helper functions used across organizing.rs, kind.rs, and arc_schema.rs
//! to avoid code duplication.

use std::fmt::Write;

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

/// Write a MERGE statement for a `:Meta:<Label>` node with ON CREATE/ON MATCH SET.
///
/// Properties are formatted as `{var}.{name} = '{value}'`.
/// Automatically adds `created_at` on CREATE and `updated_at` on MATCH.
pub fn write_merge_meta(
    out: &mut String,
    var: &str,
    label: &str,
    key: &str,
    props: &[(&str, &str)],
) {
    writeln!(out, "MERGE ({var}:Meta:{label} {{key: '{key}'}})").unwrap();

    writeln!(out, "ON CREATE SET").unwrap();
    for (name, value) in props {
        writeln!(out, "  {var}.{name} = '{value}',").unwrap();
    }
    writeln!(out, "  {var}.created_at = datetime()").unwrap();

    writeln!(out, "ON MATCH SET").unwrap();
    for (name, value) in props {
        writeln!(out, "  {var}.{name} = '{value}',").unwrap();
    }
    writeln!(out, "  {var}.updated_at = datetime();").unwrap();
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
            "global",
            &[("display_name", "Global"), ("emoji", "globe")],
        );
        assert!(out.contains("MERGE (r:Meta:Realm {key: 'global'})"));
        assert!(out.contains("r.display_name = 'Global'"));
        assert!(out.contains("r.emoji = 'globe'"));
        assert!(out.contains("created_at = datetime()"));
        assert!(out.contains("updated_at = datetime()"));
    }

    #[test]
    fn write_section_header_format() {
        let mut out = String::new();
        write_section_header(&mut out, "TEST SECTION");
        assert!(out.contains("// TEST SECTION"));
        assert!(out.contains("// ─────"));
    }
}
