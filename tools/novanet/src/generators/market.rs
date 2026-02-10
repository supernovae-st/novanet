//! Generator for Market Cypher statements.
//!
//! Transforms parsed ATH 5-market data into Neo4j seed file.

#![allow(clippy::needless_raw_string_hashes)]

use std::path::PathBuf;

use chrono::Local;

use crate::config::resolve_ath_path;
use crate::generators::cypher_utils::escape_cypher;
use crate::parsers::market::{MarketData, load_all_markets};
use crate::{NovaNetError, Result};

/// Generate Cypher for Market nodes.
pub struct MarketGenerator {
    ath_path: PathBuf,
}

impl MarketGenerator {
    /// Create a generator with ATH path from env var or explicit path.
    pub fn new(explicit_path: Option<&str>) -> Result<Self> {
        Ok(Self {
            ath_path: resolve_ath_path(explicit_path)?,
        })
    }

    /// Generate the complete Cypher file content.
    pub fn generate(&self) -> Result<String> {
        // Load all market files
        let markets = load_all_markets(&self.ath_path)?;

        if markets.is_empty() {
            return Err(NovaNetError::Validation(
                "No market files found".to_string(),
            ));
        }

        // Generate Cypher
        let mut output = String::new();

        // Header
        output.push_str(&self.generate_header(&markets));

        // Part 1: Market nodes
        output.push_str(&self.generate_market_nodes_section(&markets));

        // Part 2: Locale → Market arcs
        output.push_str(&self.generate_locale_arcs_section(&markets));

        Ok(output)
    }

    /// Generate file header.
    fn generate_header(&self, markets: &[MarketData]) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let locale_count = markets.len();

        format!(
            r#"// ============================================================================
// MARKET SEED - Generated from ATH 5-market
// Generated: {}
// Source: {}/5-market/
// Locales: {}
// ============================================================================

"#,
            timestamp,
            self.ath_path.display(),
            locale_count
        )
    }

    /// Generate Market nodes section.
    fn generate_market_nodes_section(&self, markets: &[MarketData]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            r#"// ----------------------------------------------------------------------------
// PART 1: Market nodes ({} locales)
// ----------------------------------------------------------------------------

"#,
            markets.len()
        ));

        for m in markets {
            output.push_str(&self.generate_market_cypher(m));
            output.push('\n');
        }

        output
    }

    /// Generate Cypher for a single Market node.
    fn generate_market_cypher(&self, m: &MarketData) -> String {
        // Serialize structured data to JSON
        let payment_methods_json =
            serde_json::to_string(&m.payment_methods).unwrap_or_else(|_| "[]".to_string());
        let popular_platforms_json =
            serde_json::to_string(&m.popular_platforms).unwrap_or_else(|_| "[]".to_string());
        let social_networks_json =
            serde_json::to_string(&m.social_networks).unwrap_or_else(|_| "[]".to_string());
        let demographics_json =
            serde_json::to_string(&m.demographics).unwrap_or_else(|_| "{}".to_string());

        // Generate llm_context for retrieval guidance
        let llm_context = format!(
            "USE: {} market (pop {}M, internet {}%, {} e-commerce). \
             TRIGGERS: pricing strategy, payment methods, platform references. \
             NOT: generic business terms, universal e-commerce patterns.",
            m.locale_key,
            m.population_millions,
            (m.internet_penetration * 100.0) as i32,
            m.ecommerce_maturity
        );

        // Handle optional GDP
        let gdp_property = match m.gdp_per_capita_usd {
            Some(gdp) => format!("    m.gdp_per_capita_usd = {},\n", gdp),
            None => String::new(),
        };

        // Handle optional buying behavior
        let buying_behavior_property = match &m.buying_behavior {
            Some(bb) => format!("    m.buying_behavior = '{}',\n", escape_cypher(bb)),
            None => String::new(),
        };

        format!(
            r#"MERGE (m:Market {{key: '{}'}})
SET m.display_name = '{}',
    m.description = '{}',
    m.llm_context = '{}',
    m.population_millions = {},
{}    m.internet_penetration = {},
    m.mobile_penetration = {},
    m.ecommerce_maturity = '{}',
    m.digital_maturity_score = {},
    m.payment_methods = '{}',
    m.popular_platforms = '{}',
    m.social_networks = '{}',
    m.demographics = '{}',
    m.market_summary = '{}',
{}    m.template_version = '{}',
    m.source_file = '{}',
    m.last_updated = '{}';
"#,
            m.locale_key,
            escape_cypher(&format!("{} Market Data", m.locale_key)),
            escape_cypher(&format!("Market intelligence for {}", m.locale_key)),
            escape_cypher(&llm_context),
            m.population_millions,
            gdp_property,
            m.internet_penetration,
            m.mobile_penetration,
            m.ecommerce_maturity,
            m.digital_maturity_score,
            escape_cypher(&payment_methods_json),
            escape_cypher(&popular_platforms_json),
            escape_cypher(&social_networks_json),
            escape_cypher(&demographics_json),
            escape_cypher(&m.market_summary),
            buying_behavior_property,
            m.template_version,
            m.source_file,
            m.last_updated
        )
    }

    /// Generate Locale → Market arcs section.
    fn generate_locale_arcs_section(&self, markets: &[MarketData]) -> String {
        let mut output = String::new();

        output.push_str(
            r#"// ----------------------------------------------------------------------------
// PART 2: Arcs Locale → Market
// ----------------------------------------------------------------------------

"#,
        );

        for m in markets {
            output.push_str(&format!(
                r#"MATCH (l:Locale {{key: '{}'}})
MATCH (m:Market {{key: '{}'}})
MERGE (l)-[:HAS_MARKET]->(m);

"#,
                m.locale_key, m.locale_key
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

    fn create_test_market() -> MarketData {
        MarketData {
            locale_key: "fr-FR".to_string(),
            population_millions: 68.2,
            gdp_per_capita_usd: Some(45000),
            internet_penetration: 0.93,
            mobile_penetration: 0.92,
            ecommerce_maturity: "mature".to_string(),
            digital_maturity_score: 79,
            payment_methods: serde_json::json!([
                {"method": "Credit cards", "share": 0.62, "trend": "Stable"}
            ]),
            popular_platforms: serde_json::json!([
                {"name": "Amazon.fr", "market_share": 0.22, "strength": "Logistics"}
            ]),
            social_networks: serde_json::json!([
                {"name": "YouTube", "penetration": 0.79, "audience": "All ages"}
            ]),
            demographics: serde_json::json!({
                "median_age": 42.3,
                "urban_rate": 0.81
            }),
            market_summary: "fr-FR: Population 68.2M. Internet: 93%, Mobile: 92%.".to_string(),
            buying_behavior: Some("Quality-focused, brand-loyal".to_string()),
            raw_markdown: "# Market: fr-FR".to_string(),
            template_version: "2.0".to_string(),
            last_updated: "2025-01-10".to_string(),
            source_file: "5-market/fr-FR.md".to_string(),
        }
    }

    #[test]
    fn test_generate_market_cypher() {
        let market = create_test_market();
        let generator = MarketGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_market_cypher(&market);

        assert!(cypher.contains("MERGE (m:Market {key: 'fr-FR'})"));
        assert!(cypher.contains("m.population_millions = 68.2"));
        assert!(cypher.contains("m.internet_penetration = 0.93"));
        assert!(cypher.contains("m.ecommerce_maturity = 'mature'"));
        assert!(cypher.contains("m.digital_maturity_score = 79"));
    }

    #[test]
    fn test_generate_locale_arcs() {
        let markets = vec![create_test_market()];
        let generator = MarketGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_locale_arcs_section(&markets);

        assert!(cypher.contains("MATCH (l:Locale {key: 'fr-FR'})"));
        assert!(cypher.contains("MATCH (m:Market {key: 'fr-FR'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_MARKET]->(m)"));
    }

    #[test]
    fn test_generate_header() {
        let markets = vec![create_test_market()];
        let generator = MarketGenerator::new(Some("/tmp/test")).unwrap();
        let header = generator.generate_header(&markets);

        assert!(header.contains("MARKET SEED"));
        assert!(header.contains("Locales: 1"));
    }

    #[test]
    fn test_optional_gdp_present() {
        let market = create_test_market();
        let generator = MarketGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_market_cypher(&market);

        assert!(cypher.contains("m.gdp_per_capita_usd = 45000"));
    }

    #[test]
    fn test_optional_gdp_absent() {
        let mut market = create_test_market();
        market.gdp_per_capita_usd = None;
        let generator = MarketGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_market_cypher(&market);

        assert!(!cypher.contains("gdp_per_capita_usd"));
    }

    #[test]
    fn test_json_serialization() {
        let market = create_test_market();
        let generator = MarketGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_market_cypher(&market);

        // Check JSON fields are serialized
        assert!(cypher.contains("payment_methods"));
        assert!(cypher.contains("Credit cards"));
        assert!(cypher.contains("popular_platforms"));
        assert!(cypher.contains("Amazon.fr"));
    }
}
