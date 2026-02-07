//! Generator for Formatting Cypher statements.
//!
//! Transforms parsed ATH 2-rules-formatting data into Neo4j seed file.

use std::path::PathBuf;

use chrono::Local;

use crate::config::resolve_ath_path;
use crate::generators::cypher_utils::escape_cypher;
use crate::parsers::formatting::{Formatting, load_all_formattings};
use crate::{NovaNetError, Result};

/// Generate Cypher for Formatting nodes.
pub struct FormattingGenerator {
    ath_path: PathBuf,
}

impl FormattingGenerator {
    /// Create a generator with ATH path from env var or explicit path.
    pub fn new(explicit_path: Option<&str>) -> Result<Self> {
        Ok(Self {
            ath_path: resolve_ath_path(explicit_path)?,
        })
    }

    /// Generate the complete Cypher file content.
    pub fn generate(&self) -> Result<String> {
        // Load all formatting files
        let formattings = load_all_formattings(&self.ath_path)?;

        if formattings.is_empty() {
            return Err(NovaNetError::Validation(
                "No formatting files found".to_string(),
            ));
        }

        // Generate Cypher
        let mut output = String::new();

        // Header
        output.push_str(&self.generate_header(&formattings));

        // Part 1: Formatting nodes
        output.push_str(&self.generate_formatting_nodes_section(&formattings));

        // Part 2: Locale → Formatting arcs
        output.push_str(&self.generate_locale_arcs_section(&formattings));

        Ok(output)
    }

    /// Generate file header.
    fn generate_header(&self, formattings: &[Formatting]) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let locale_count = formattings.len();

        format!(
            r#"// ============================================================================
// FORMATTING SEED - Generated from ATH 2-rules-formatting
// Generated: {}
// Source: {}/2-rules-formatting/
// Locales: {}
// ============================================================================

"#,
            timestamp,
            self.ath_path.display(),
            locale_count
        )
    }

    /// Generate Formatting nodes section.
    fn generate_formatting_nodes_section(&self, formattings: &[Formatting]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            r#"// ----------------------------------------------------------------------------
// PART 1: Formatting nodes ({} locales)
// ----------------------------------------------------------------------------

"#,
            formattings.len()
        ));

        for f in formattings {
            output.push_str(&self.generate_formatting_cypher(f));
            output.push('\n');
        }

        output
    }

    /// Generate Cypher for a single Formatting node.
    fn generate_formatting_cypher(&self, f: &Formatting) -> String {
        // Serialize sections to JSON
        let number_json = serde_json::to_string(&f.number).unwrap_or_else(|_| "{}".to_string());
        let date_json = serde_json::to_string(&f.date).unwrap_or_else(|_| "{}".to_string());
        let time_json = serde_json::to_string(&f.time).unwrap_or_else(|_| "{}".to_string());
        let currency_json = serde_json::to_string(&f.currency).unwrap_or_else(|_| "{}".to_string());
        let phone_json = serde_json::to_string(&f.phone).unwrap_or_else(|_| "{}".to_string());
        let address_json = serde_json::to_string(&f.address).unwrap_or_else(|_| "{}".to_string());
        let measurement_json =
            serde_json::to_string(&f.measurement).unwrap_or_else(|_| "{}".to_string());
        let percentage_json =
            serde_json::to_string(&f.percentage).unwrap_or_else(|_| "{}".to_string());
        let temperature_json =
            serde_json::to_string(&f.temperature).unwrap_or_else(|_| "{}".to_string());
        let validation_json =
            serde_json::to_string(&f.validation_patterns).unwrap_or_else(|_| "{}".to_string());
        let data_sources_json =
            serde_json::to_string(&f.data_sources).unwrap_or_else(|_| "[]".to_string());

        format!(
            r#"MERGE (f:Formatting {{key: '{}'}})
SET f.display_name = '{}',
    f.description = '{}',
    f.llm_context = '{}',
    f.data_sources = '{}',
    f.number = '{}',
    f.date = '{}',
    f.time = '{}',
    f.currency = '{}',
    f.phone = '{}',
    f.address = '{}',
    f.measurement = '{}',
    f.percentage = '{}',
    f.temperature = '{}',
    f.validation_patterns = '{}',
    f.template_version = '{}',
    f.source_file = '{}',
    f.last_updated = '{}';
"#,
            f.key,
            escape_cypher(&f.display_name),
            escape_cypher(&f.description),
            escape_cypher(&f.llm_context),
            escape_cypher(&data_sources_json),
            escape_cypher(&number_json),
            escape_cypher(&date_json),
            escape_cypher(&time_json),
            escape_cypher(&currency_json),
            escape_cypher(&phone_json),
            escape_cypher(&address_json),
            escape_cypher(&measurement_json),
            escape_cypher(&percentage_json),
            escape_cypher(&temperature_json),
            escape_cypher(&validation_json),
            f.template_version,
            f.source_file,
            f.last_updated
        )
    }

    /// Generate Locale → Formatting arcs section.
    fn generate_locale_arcs_section(&self, formattings: &[Formatting]) -> String {
        let mut output = String::new();

        output.push_str(
            r#"// ----------------------------------------------------------------------------
// PART 2: Arcs Locale → Formatting
// ----------------------------------------------------------------------------

"#,
        );

        for f in formattings {
            output.push_str(&format!(
                r#"MATCH (l:Locale {{key: '{}'}})
MATCH (f:Formatting {{key: '{}'}})
MERGE (l)-[:HAS_FORMATTING]->(f);

"#,
                f.key, f.key
            ));
        }

        output
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::formatting::{
        CurrencyFormatting, DateFormatting, FormatExample, NumberFormatting, TimeFormatting,
    };
    use std::collections::HashMap;

    fn create_test_formatting() -> Formatting {
        Formatting {
            key: "fr-FR".to_string(),
            display_name: "French (France) Formatting".to_string(),
            description: "Formatting rules for fr-FR".to_string(),
            llm_context: "Test context".to_string(),
            data_sources: vec!["CLDR".to_string()],
            number: NumberFormatting {
                decimal_separator: ",".to_string(),
                thousands_separator: " ".to_string(),
                negative_sign: "-".to_string(),
                positive_sign: "+".to_string(),
                grouping_pattern: 3,
                numeral_system: None,
                correct_examples: vec![FormatExample {
                    input: "1234.56".to_string(),
                    output: "1 234,56".to_string(),
                }],
                incorrect_examples: vec![],
            },
            date: DateFormatting::default(),
            time: TimeFormatting::default(),
            currency: CurrencyFormatting {
                code: "EUR".to_string(),
                symbol: "€".to_string(),
                symbol_position: "after".to_string(),
                space_between: true,
                decimal_places: 2,
                subunit: Some("centime".to_string()),
                correct_examples: vec![],
                incorrect_examples: vec![],
            },
            phone: Default::default(),
            address: Default::default(),
            measurement: Default::default(),
            percentage: Default::default(),
            temperature: Default::default(),
            validation_patterns: HashMap::new(),
            template_version: "2.0".to_string(),
            source_file: "2-rules-formatting/fr-FR.md".to_string(),
            last_updated: "2026-01-11".to_string(),
        }
    }

    #[test]
    fn test_escape_cypher() {
        assert_eq!(escape_cypher("it's"), "it\\'s");
        assert_eq!(escape_cypher("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_generate_formatting_cypher() {
        let formatting = create_test_formatting();
        let generator = FormattingGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_formatting_cypher(&formatting);

        assert!(cypher.contains("MERGE (f:Formatting {key: 'fr-FR'})"));
        assert!(cypher.contains("f.display_name = 'French (France) Formatting'"));
        assert!(cypher.contains("f.template_version = '2.0'"));
        // Check JSON serialization
        assert!(cypher.contains("decimal_separator"));
        assert!(cypher.contains("EUR"));
    }

    #[test]
    fn test_generate_locale_arcs() {
        let formattings = vec![create_test_formatting()];
        let generator = FormattingGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_locale_arcs_section(&formattings);

        assert!(cypher.contains("MATCH (l:Locale {key: 'fr-FR'})"));
        assert!(cypher.contains("MATCH (f:Formatting {key: 'fr-FR'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_FORMATTING]->(f)"));
    }

    #[test]
    fn test_generate_header() {
        let formattings = vec![create_test_formatting()];
        let generator = FormattingGenerator::new(Some("/tmp/test")).unwrap();
        let header = generator.generate_header(&formattings);

        assert!(header.contains("FORMATTING SEED"));
        assert!(header.contains("Locales: 1"));
    }
}
