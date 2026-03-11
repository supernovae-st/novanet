//! Generator for Culture Cypher statements.
//!
//! Transforms parsed ATH 4-culture-norms data into Neo4j seed file.

#![allow(clippy::needless_raw_string_hashes)]

use std::path::PathBuf;

use chrono::Local;

use crate::config::resolve_ath_path;
use crate::generators::cypher_utils::escape_cypher;
use crate::parsers::culture::{CultureData, load_all_cultures};
use crate::{NovaNetError, Result};

/// Generate Cypher for Culture nodes.
pub struct CultureGenerator {
    ath_path: PathBuf,
}

impl CultureGenerator {
    /// Create a generator with ATH path from env var or explicit path.
    pub fn new(explicit_path: Option<&str>) -> Result<Self> {
        Ok(Self {
            ath_path: resolve_ath_path(explicit_path)?,
        })
    }

    /// Generate the complete Cypher file content.
    pub fn generate(&self) -> Result<String> {
        // Load all culture files
        let cultures = load_all_cultures(&self.ath_path)?;

        if cultures.is_empty() {
            return Err(NovaNetError::Validation(
                "No culture files found".to_string(),
            ));
        }

        // Generate Cypher
        let mut output = String::new();

        // Header
        output.push_str(&self.generate_header(&cultures));

        // Part 1: Culture nodes
        output.push_str(&self.generate_culture_nodes_section(&cultures));

        // Part 2: Locale → Culture arcs
        output.push_str(&self.generate_locale_arcs_section(&cultures));

        Ok(output)
    }

    /// Generate file header.
    fn generate_header(&self, cultures: &[CultureData]) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let locale_count = cultures.len();

        format!(
            r"// ============================================================================
// CULTURE SEED - Generated from ATH 4-culture-norms
// Generated: {}
// Source: {}/4-culture-norms/
// Locales: {}
// ============================================================================

",
            timestamp,
            self.ath_path.display(),
            locale_count
        )
    }

    /// Generate Culture nodes section.
    fn generate_culture_nodes_section(&self, cultures: &[CultureData]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            r"// ----------------------------------------------------------------------------
// PART 1: Culture nodes ({} locales)
// ----------------------------------------------------------------------------

",
            cultures.len()
        ));

        for c in cultures {
            output.push_str(&self.generate_culture_cypher(c));
            output.push('\n');
        }

        output
    }

    /// Generate Cypher for a single Culture node.
    fn generate_culture_cypher(&self, c: &CultureData) -> String {
        // Serialize structured data to JSON
        let seasons_json = serde_json::to_string(&c.seasons).unwrap_or_else(|_| "{}".to_string());
        let holidays_json = serde_json::to_string(&c.holidays).unwrap_or_else(|_| "[]".to_string());
        let business_hours_json =
            serde_json::to_string(&c.business_hours).unwrap_or_else(|_| "{}".to_string());
        let values_json = serde_json::to_string(&c.values).unwrap_or_else(|_| "[]".to_string());
        let communication_norms_json =
            serde_json::to_string(&c.communication_norms).unwrap_or_else(|_| "{}".to_string());

        // Generate llm_context for retrieval guidance
        let llm_context = format!(
            "USE: {} culture ({} hemisphere, {} communication, {} hierarchy). \
             TRIGGERS: regional holidays, seasonal references, formality levels. \
             NOT: generic greetings, universal values.",
            c.locale_key, c.hemisphere, c.communication_directness, c.hierarchy_importance
        );

        format!(
            r"MERGE (c:Culture {{key: '{}'}})
SET c.display_name = '{}',
    c.content = '{}',
    c.llm_context = '{}',
    c.hemisphere = '{}',
    c.work_week_start = '{}',
    c.communication_directness = '{}',
    c.hierarchy_importance = '{}',
    c.individualism_level = '{}',
    c.seasons = '{}',
    c.holidays = '{}',
    c.business_hours = '{}',
    c.values = '{}',
    c.communication_norms = '{}',
    c.culture_summary = '{}',
    c.taboos_summary = '{}',
    c.template_version = '{}',
    c.source_file = '{}',
    c.last_updated = '{}';
",
            c.locale_key,
            escape_cypher(&format!("{} Culture Norms", c.locale_key)),
            escape_cypher(&format!("Cultural context and norms for {}", c.locale_key)),
            escape_cypher(&llm_context),
            c.hemisphere,
            c.work_week_start,
            c.communication_directness,
            c.hierarchy_importance,
            c.individualism_level,
            escape_cypher(&seasons_json),
            escape_cypher(&holidays_json),
            escape_cypher(&business_hours_json),
            escape_cypher(&values_json),
            escape_cypher(&communication_norms_json),
            escape_cypher(&c.culture_summary),
            escape_cypher(&c.taboos_summary),
            c.template_version,
            c.source_file,
            c.last_updated,
        )
    }

    /// Generate Locale → Culture arcs section.
    fn generate_locale_arcs_section(&self, cultures: &[CultureData]) -> String {
        let mut output = String::new();

        output.push_str(
            r"// ----------------------------------------------------------------------------
// PART 2: Arcs Locale → Culture
// ----------------------------------------------------------------------------

",
        );

        for c in cultures {
            output.push_str(&format!(
                r"MATCH (l:Locale {{key: '{}'}})
MATCH (c:Culture {{key: '{}'}})
MERGE (l)-[:HAS_CULTURE]->(c);

",
                c.locale_key, c.locale_key
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
    use crate::parsers::culture::CoreValue;

    fn create_test_culture() -> CultureData {
        CultureData {
            locale_key: "fr-FR".to_string(),
            hemisphere: "northern".to_string(),
            work_week_start: "monday".to_string(),
            communication_directness: "balanced".to_string(),
            hierarchy_importance: "medium".to_string(),
            individualism_level: "collectivist".to_string(),
            seasons: serde_json::json!({}),
            holidays: serde_json::json!([]),
            business_hours: serde_json::json!({"start": "09:00", "end": "18:00"}),
            values: vec![CoreValue {
                value: "Liberté".to_string(),
                importance: "High".to_string(),
                expression: "Personal autonomy".to_string(),
                marketing_angle: "Emphasize choice".to_string(),
            }],
            communication_norms: serde_json::json!({
                "directness": "BALANCED",
                "context_type": "HIGH_CONTEXT"
            }),
            culture_summary: "fr-FR: Communication style is balanced.".to_string(),
            taboos_summary: "CRITICAL TABOOS: Personal income.".to_string(),
            raw_markdown: "# Culture Norms: fr-FR".to_string(),
            template_version: "3.1".to_string(),
            last_updated: "2025-01-10".to_string(),
            source_file: "4-culture-norms/fr-FR.md".to_string(),
        }
    }

    #[test]
    fn test_generate_culture_cypher() {
        let culture = create_test_culture();
        let generator = CultureGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_culture_cypher(&culture);

        assert!(cypher.contains("MERGE (c:Culture {key: 'fr-FR'})"));
        assert!(cypher.contains("c.hemisphere = 'northern'"));
        assert!(cypher.contains("c.communication_directness = 'balanced'"));
        assert!(cypher.contains("c.hierarchy_importance = 'medium'"));
        assert!(cypher.contains("c.template_version = '3.1'"));
    }

    #[test]
    fn test_generate_locale_arcs() {
        let cultures = vec![create_test_culture()];
        let generator = CultureGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_locale_arcs_section(&cultures);

        assert!(cypher.contains("MATCH (l:Locale {key: 'fr-FR'})"));
        assert!(cypher.contains("MATCH (c:Culture {key: 'fr-FR'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_CULTURE]->(c)"));
    }

    #[test]
    fn test_generate_header() {
        let cultures = vec![create_test_culture()];
        let generator = CultureGenerator::new(Some("/tmp/test")).unwrap();
        let header = generator.generate_header(&cultures);

        assert!(header.contains("CULTURE SEED"));
        assert!(header.contains("Locales: 1"));
    }

    #[test]
    fn test_json_serialization() {
        let culture = create_test_culture();
        let generator = CultureGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_culture_cypher(&culture);

        // Check JSON fields are serialized
        assert!(cypher.contains("business_hours"));
        assert!(cypher.contains("09:00"));
        assert!(cypher.contains("communication_norms"));
        assert!(cypher.contains("HIGH_CONTEXT"));
    }
}
