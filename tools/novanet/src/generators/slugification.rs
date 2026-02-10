//! Generator for SlugRule and Slugification Cypher statements.
//!
//! Transforms parsed ATH 2-rules-slug data into Neo4j seed file.

#![allow(clippy::needless_raw_string_hashes)]

use std::path::PathBuf;

use chrono::Local;

use crate::config::resolve_ath_path;
use crate::generators::cypher_utils::escape_cypher;
use crate::parsers::slugification::{
    RegionalAddition, ScriptConfig, SlugExample, SlugRule, Slugification, Warning,
    aggregate_slug_rules, load_all_slugifications,
};
use crate::{NovaNetError, Result};

/// Generate Cypher for SlugRule and Slugification nodes.
pub struct SlugificationGenerator {
    ath_path: PathBuf,
}

impl SlugificationGenerator {
    /// Create a generator with ATH path from env var or explicit path.
    pub fn new(explicit_path: Option<&str>) -> Result<Self> {
        Ok(Self {
            ath_path: resolve_ath_path(explicit_path)?,
        })
    }

    /// Generate the complete Cypher file content.
    pub fn generate(&self) -> Result<String> {
        // Load all slugification files
        let slugifications = load_all_slugifications(&self.ath_path)?;

        if slugifications.is_empty() {
            return Err(NovaNetError::Validation(
                "No slugification files found".to_string(),
            ));
        }

        // Aggregate into SlugRules
        let slug_rules = aggregate_slug_rules(&slugifications);

        // Generate Cypher
        let mut output = String::new();

        // Header
        output.push_str(&self.generate_header(&slugifications));

        // Part 1: SlugRule nodes
        output.push_str(&self.generate_slug_rules_section(&slug_rules));

        // Part 2: Slugification nodes
        output.push_str(&self.generate_slugifications_section(&slugifications));

        // Part 3: Locale → Slugification arcs
        output.push_str(&self.generate_locale_arcs_section(&slugifications));

        // Part 4: Slugification → SlugRule arcs
        output.push_str(&self.generate_rule_arcs_section(&slugifications));

        Ok(output)
    }

    /// Generate file header.
    fn generate_header(&self, slugifications: &[Slugification]) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let locale_count = slugifications.len();

        format!(
            r#"// ============================================================================
// SLUGIFICATION SEED - Generated from ATH 2-rules-slug
// Generated: {}
// Source: {}/2-rules-slug/
// Locales: {}
// ============================================================================

"#,
            timestamp,
            self.ath_path.display(),
            locale_count
        )
    }

    /// Generate SlugRule nodes section.
    fn generate_slug_rules_section(&self, rules: &[SlugRule]) -> String {
        let mut output = String::new();

        output.push_str(
            r#"// ----------------------------------------------------------------------------
// PART 1: SlugRule nodes (meta-level slugification rules)
// ----------------------------------------------------------------------------

"#,
        );

        for rule in rules {
            output.push_str(&self.generate_slug_rule_cypher(rule));
            output.push('\n');
        }

        output
    }

    /// Generate Cypher for a single SlugRule.
    fn generate_slug_rule_cypher(&self, rule: &SlugRule) -> String {
        let char_transforms = rule
            .char_transforms
            .as_ref()
            .map(|t| serde_json::to_string(t).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| "null".to_string());

        let romanization = rule
            .romanization_systems
            .as_ref()
            .map(|r| serde_json::to_string(r).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| "null".to_string());

        let examples_json =
            serde_json::to_string(&rule.locale_examples).unwrap_or_else(|_| "[]".to_string());

        format!(
            r#"MERGE (sr:SlugRule {{key: '{}'}})
SET sr.display_name = '{}',
    sr.output_encoding = '{}',
    sr.has_case = {},
    sr.normalization = '{}',
    sr.diacritics = '{}',
    sr.non_latin = '{}',
    sr.case_handling = '{}',
    sr.spaces = '{}',
    sr.special_chars = '{}',
    sr.char_transforms = '{}',
    sr.romanization_systems = '{}',
    sr.locale_count = {},
    sr.locale_examples = '{}',
    sr.llm_context = '{}';
"#,
            rule.key,
            escape_cypher(&rule.display_name),
            rule.output_encoding,
            rule.has_case,
            rule.normalization,
            rule.diacritics,
            rule.non_latin,
            rule.case_handling,
            rule.spaces,
            rule.special_chars,
            escape_cypher(&char_transforms),
            escape_cypher(&romanization),
            rule.locale_count,
            escape_cypher(&examples_json),
            escape_cypher(&rule.llm_context)
        )
    }

    /// Generate Slugification nodes section.
    fn generate_slugifications_section(&self, slugifications: &[Slugification]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            r#"// ----------------------------------------------------------------------------
// PART 2: Slugification nodes ({} locales)
// ----------------------------------------------------------------------------

"#,
            slugifications.len()
        ));

        for s in slugifications {
            output.push_str(&self.generate_slugification_cypher(s));
            output.push('\n');
        }

        output
    }

    /// Generate Cypher for a single Slugification.
    fn generate_slugification_cypher(&self, s: &Slugification) -> String {
        let stopwords_json =
            serde_json::to_string(&s.stopwords).unwrap_or_else(|_| "{}".to_string());

        let regional_json = serialize_regional_additions(&s.regional_additions);
        let script_json = serialize_script_config(&s.script_config);
        let warnings_json = serialize_warnings(&s.warnings);
        let examples_json = serialize_examples(&s.examples);

        format!(
            r#"MERGE (s:Slugification {{key: '{}'}})
SET s.display_name = '{}',
    s.description = '{}',
    s.slug_rule = '{}',
    s.stopwords = '{}',
    s.stopwords_count = {},
    s.regional_additions = '{}',
    s.script_config = '{}',
    s.warnings = '{}',
    s.examples = '{}',
    s.template_version = '{}',
    s.source_file = '{}',
    s.last_updated = '{}',
    s.llm_context = '{}';
"#,
            s.key,
            escape_cypher(&s.display_name),
            escape_cypher(&s.description),
            s.slug_rule,
            escape_cypher(&stopwords_json),
            s.stopwords_count,
            escape_cypher(&regional_json),
            escape_cypher(&script_json),
            escape_cypher(&warnings_json),
            escape_cypher(&examples_json),
            s.template_version,
            s.source_file,
            s.last_updated,
            escape_cypher(&s.llm_context)
        )
    }

    /// Generate Locale → Slugification arcs section.
    fn generate_locale_arcs_section(&self, slugifications: &[Slugification]) -> String {
        let mut output = String::new();

        output.push_str(
            r#"// ----------------------------------------------------------------------------
// PART 3: Arcs Locale → Slugification
// ----------------------------------------------------------------------------

"#,
        );

        for s in slugifications {
            output.push_str(&format!(
                r#"MATCH (l:Locale {{key: '{}'}})
MATCH (s:Slugification {{key: '{}'}})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

"#,
                s.key, s.key
            ));
        }

        output
    }

    /// Generate Slugification → SlugRule arcs section.
    fn generate_rule_arcs_section(&self, slugifications: &[Slugification]) -> String {
        let mut output = String::new();

        output.push_str(
            r#"// ----------------------------------------------------------------------------
// PART 4: Arcs Slugification → SlugRule
// ----------------------------------------------------------------------------

"#,
        );

        for s in slugifications {
            output.push_str(&format!(
                r#"MATCH (s:Slugification {{key: '{}'}})
MATCH (sr:SlugRule {{key: '{}'}})
MERGE (s)-[:FOLLOWS_RULE]->(sr);

"#,
                s.key, s.slug_rule
            ));
        }

        output
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Serialize regional additions to JSON string.
fn serialize_regional_additions(additions: &[RegionalAddition]) -> String {
    serde_json::to_string(additions).unwrap_or_else(|_| "[]".to_string())
}

/// Serialize script config to JSON string.
fn serialize_script_config(config: &Option<ScriptConfig>) -> String {
    match config {
        Some(c) => serde_json::to_string(c).unwrap_or_else(|_| "null".to_string()),
        None => "null".to_string(),
    }
}

/// Serialize warnings to JSON string.
fn serialize_warnings(warnings: &[Warning]) -> String {
    serde_json::to_string(warnings).unwrap_or_else(|_| "[]".to_string())
}

/// Serialize examples to JSON string.
fn serialize_examples(examples: &[SlugExample]) -> String {
    serde_json::to_string(examples).unwrap_or_else(|_| "[]".to_string())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_escape_cypher() {
        assert_eq!(escape_cypher("it's"), "it\\'s");
        assert_eq!(escape_cypher("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_generate_slug_rule_cypher() {
        let mut rule = SlugRule::new("latin_strip");
        rule.locale_count = 150;
        rule.locale_examples = vec!["en-US".to_string(), "es-ES".to_string()];
        rule.generate_llm_context();

        let generator = SlugificationGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_slug_rule_cypher(&rule);

        assert!(cypher.contains("MERGE (sr:SlugRule {key: 'latin_strip'})"));
        assert!(cypher.contains("sr.output_encoding = 'ASCII'"));
        assert!(cypher.contains("sr.locale_count = 150"));
    }

    #[test]
    fn test_generate_slugification_cypher() {
        let mut s = Slugification {
            key: "fr-FR".to_string(),
            display_name: "French (France) Slugification".to_string(),
            description: "URL slug generation rules for fr-FR".to_string(),
            llm_context: String::new(),
            slug_rule: "latin_preserve".to_string(),
            stopwords: HashMap::from([(
                "article".to_string(),
                vec!["le".to_string(), "la".to_string()],
            )]),
            stopwords_count: 2,
            regional_additions: vec![],
            script_config: None,
            validation_overrides: None,
            warnings: vec![],
            examples: vec![],
            template_version: "2.0".to_string(),
            source_file: "2-rules-slug/fr-FR.md".to_string(),
            last_updated: "2025-12-08".to_string(),
        };
        s.generate_llm_context();

        let generator = SlugificationGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_slugification_cypher(&s);

        assert!(cypher.contains("MERGE (s:Slugification {key: 'fr-FR'})"));
        assert!(cypher.contains("s.slug_rule = 'latin_preserve'"));
        assert!(cypher.contains("s.stopwords_count = 2"));
    }

    #[test]
    fn test_serialize_regional_additions() {
        let additions = vec![RegionalAddition {
            word: "test".to_string(),
            category: "noun".to_string(),
            reason: "Test reason".to_string(),
        }];

        let json = serialize_regional_additions(&additions);
        assert!(json.contains("test"));
        assert!(json.contains("noun"));
    }

    #[test]
    fn test_serialize_script_config() {
        let config = Some(ScriptConfig {
            primary_script: "arabic".to_string(),
            diacritic_handling: Some("remove_tashkeel".to_string()),
            numeral_handling: None,
            special_chars: None,
        });

        let json = serialize_script_config(&config);
        assert!(json.contains("arabic"));
        assert!(json.contains("remove_tashkeel"));

        let none_json = serialize_script_config(&None);
        assert_eq!(none_json, "null");
    }
}
