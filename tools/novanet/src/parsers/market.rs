//! Parser for ATH 5-market data.
//!
//! Transforms ATH market markdown files into Rust structs for Cypher generation.
//! Extracts: demographics, digital maturity, e-commerce, payment methods, platforms.

use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{NovaNetError, Result};

// ============================================================================
// Lazy-compiled Regex Patterns
// ============================================================================

/// Template version extraction
static RE_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"template_version:\s*(.+)").expect("valid version regex"));

/// Last updated date extraction
static RE_DATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"last_updated:\s*(.+)").expect("valid date regex"));

/// Population extraction: "68.2 million" or "335.9 million"
static RE_POPULATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Total population\s*\|\s*([\d.,]+)\s*million").expect("valid population regex")
});

/// Median age extraction: "42.3 years" or "38.9 years"
static RE_MEDIAN_AGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Median age\s*\|\s*([\d.]+)\s*years").expect("valid median regex")
});

/// Urban rate extraction from Urban/Rural table
static RE_URBAN_RATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Urban\s*\|\s*(\d+)%").expect("valid urban regex"));

/// Internet penetration extraction
static RE_INTERNET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Internet users\s*\|\s*(\d+)%").expect("valid internet regex"));

/// Mobile penetration (smartphone) extraction
static RE_MOBILE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Smartphone penetration\s*\|\s*(\d+)%").expect("valid mobile regex")
});

/// E-commerce online shoppers percentage
static RE_ECOMMERCE_SHOPPERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Online shoppers\s*\|\s*(\d+)%").expect("valid shoppers regex"));

/// Overall ROI Priority Score
static RE_ROI_SCORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Overall\*\*\s*\|\s*\*\*([\d.]+)\*\*").expect("valid roi regex")
});

// ============================================================================
// Main Structs
// ============================================================================

/// Complete market data for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    /// Locale key (e.g., "fr-FR")
    pub locale_key: String,

    // TIER 1: INDEXED SCALARS (Cypher-queryable)
    /// Population in millions
    pub population_millions: f64,
    /// GDP per capita in USD (optional, not in all files)
    pub gdp_per_capita_usd: Option<i32>,
    /// Internet penetration rate (0.0-1.0)
    pub internet_penetration: f64,
    /// Mobile device penetration rate (0.0-1.0)
    pub mobile_penetration: f64,
    /// E-commerce market maturity level
    pub ecommerce_maturity: String,
    /// Digital maturity score 0-100 (derived from ROI score)
    pub digital_maturity_score: i32,

    // TIER 2: STRUCTURED JSON (programmatic lookup)
    /// Payment method preferences with market share
    pub payment_methods: serde_json::Value,
    /// Popular e-commerce platforms with market share
    pub popular_platforms: serde_json::Value,
    /// Social network penetration rates
    pub social_networks: serde_json::Value,
    /// Key demographic indicators
    pub demographics: serde_json::Value,

    // TIER 3: LLM CONTEXT (narrative text)
    /// Summary of market context for LLM
    pub market_summary: String,
    /// Consumer expectations and purchasing patterns
    pub buying_behavior: Option<String>,
    /// Full source markdown
    pub raw_markdown: String,

    // Metadata
    pub template_version: String,
    pub last_updated: String,
    pub source_file: String,
}

/// Payment method with usage percentage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub method: String,
    pub share: f64,
    pub trend: String,
}

/// Social network with penetration rate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNetwork {
    pub name: String,
    pub penetration: f64,
    pub audience: String,
}

/// E-commerce platform with market share.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommercePlatform {
    pub name: String,
    pub market_share: f64,
    pub strength: String,
}

impl Default for MarketData {
    fn default() -> Self {
        Self {
            locale_key: String::new(),
            population_millions: 0.0,
            gdp_per_capita_usd: None,
            internet_penetration: 0.0,
            mobile_penetration: 0.0,
            ecommerce_maturity: "growing".to_string(),
            digital_maturity_score: 50,
            payment_methods: serde_json::json!([]),
            popular_platforms: serde_json::json!([]),
            social_networks: serde_json::json!([]),
            demographics: serde_json::json!({}),
            market_summary: String::new(),
            buying_behavior: None,
            raw_markdown: String::new(),
            template_version: "2.0".to_string(),
            last_updated: String::new(),
            source_file: String::new(),
        }
    }
}

impl MarketData {
    /// Generate LLM context summary from market data.
    pub fn generate_market_summary(&mut self) {
        let mut parts = Vec::new();

        // Market size
        parts.push(format!(
            "{}: Population {:.1}M.",
            self.locale_key, self.population_millions
        ));

        // Digital penetration
        parts.push(format!(
            "Internet: {:.0}%, Mobile: {:.0}%.",
            self.internet_penetration * 100.0,
            self.mobile_penetration * 100.0
        ));

        // E-commerce maturity
        parts.push(format!("E-commerce maturity: {}.", self.ecommerce_maturity));

        // Digital score
        parts.push(format!(
            "Digital maturity score: {}/100.",
            self.digital_maturity_score
        ));

        self.market_summary = parts.join(" ");
    }

    /// Infer e-commerce maturity from online shoppers percentage.
    pub fn infer_ecommerce_maturity(online_shoppers_pct: f64) -> String {
        if online_shoppers_pct >= 0.80 {
            "advanced".to_string()
        } else if online_shoppers_pct >= 0.65 {
            "mature".to_string()
        } else if online_shoppers_pct >= 0.40 {
            "growing".to_string()
        } else {
            "emerging".to_string()
        }
    }
}

// ============================================================================
// Loading Functions
// ============================================================================

/// Load all market files from ATH data directory.
pub fn load_all_markets(ath_path: &Path) -> Result<Vec<MarketData>> {
    let market_dir = ath_path.join("5-market");

    if !market_dir.exists() {
        return Err(NovaNetError::Validation(format!(
            "Market directory not found: {}",
            market_dir.display()
        )));
    }

    // Collect all .md files
    let files: Vec<_> = WalkDir::new(&market_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();

    // Parse in parallel
    let markets: Vec<MarketData> = files
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            match parse_market_file(path) {
                Ok(m) => Some(m),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();

    Ok(markets)
}

/// Parse a single market markdown file.
pub fn parse_market_file(path: &Path) -> Result<MarketData> {
    let content = fs::read_to_string(path)?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Extract locale from filename (e.g., "fr-FR.md" -> "fr-FR")
    let locale = filename.trim_end_matches(".md");

    parse_market_markdown(&content, locale, filename)
}

/// Parse market markdown content.
pub fn parse_market_markdown(
    content: &str,
    locale_key: &str,
    source_file: &str,
) -> Result<MarketData> {
    // Parse frontmatter
    let (template_version, last_updated) = parse_frontmatter(content);

    // Extract scalar values
    let population_millions = extract_population(content);
    let median_age = extract_median_age(content);
    let urban_rate = extract_urban_rate(content);
    let internet_penetration = extract_internet_penetration(content);
    let mobile_penetration = extract_mobile_penetration(content);
    let online_shoppers = extract_online_shoppers(content);
    let roi_score = extract_roi_score(content);

    // Infer e-commerce maturity from online shoppers
    let ecommerce_maturity = MarketData::infer_ecommerce_maturity(online_shoppers);

    // Parse structured data
    let payment_methods = parse_payment_methods(content);
    let social_networks = parse_social_networks(content);
    let popular_platforms = parse_popular_platforms(content);

    // Build demographics JSON
    let demographics = serde_json::json!({
        "median_age": median_age,
        "urban_rate": urban_rate,
        "literacy_rate": 0.99, // Assumed for most markets
    });

    let mut data = MarketData {
        locale_key: locale_key.to_string(),
        population_millions,
        gdp_per_capita_usd: None, // Not in current ATH format
        internet_penetration,
        mobile_penetration,
        ecommerce_maturity,
        digital_maturity_score: roi_score.round() as i32,
        payment_methods,
        popular_platforms,
        social_networks,
        demographics,
        market_summary: String::new(),
        buying_behavior: None,
        raw_markdown: content.to_string(),
        template_version,
        last_updated,
        source_file: source_file.to_string(),
    };

    // Generate summary
    data.generate_market_summary();

    Ok(data)
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse YAML frontmatter.
fn parse_frontmatter(content: &str) -> (String, String) {
    let version = RE_VERSION
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "2.0".to_string());

    let date = RE_DATE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    (version, date)
}

/// Extract population in millions.
fn extract_population(content: &str) -> f64 {
    RE_POPULATION
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().replace(',', ".").parse::<f64>().ok())
        .unwrap_or(0.0)
}

/// Extract median age.
fn extract_median_age(content: &str) -> f64 {
    RE_MEDIAN_AGE
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(35.0)
}

/// Extract urban rate as decimal (0.0-1.0).
fn extract_urban_rate(content: &str) -> f64 {
    RE_URBAN_RATE
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .map(|p| p / 100.0)
        .unwrap_or(0.5)
}

/// Extract internet penetration as decimal (0.0-1.0).
fn extract_internet_penetration(content: &str) -> f64 {
    RE_INTERNET
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .map(|p| p / 100.0)
        .unwrap_or(0.5)
}

/// Extract mobile/smartphone penetration as decimal (0.0-1.0).
fn extract_mobile_penetration(content: &str) -> f64 {
    RE_MOBILE
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .map(|p| p / 100.0)
        .unwrap_or(0.5)
}

/// Extract online shoppers percentage as decimal (0.0-1.0).
fn extract_online_shoppers(content: &str) -> f64 {
    RE_ECOMMERCE_SHOPPERS
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .map(|p| p / 100.0)
        .unwrap_or(0.5)
}

/// Extract ROI Priority Score as digital maturity indicator.
fn extract_roi_score(content: &str) -> f64 {
    RE_ROI_SCORE
        .captures(content)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(50.0)
}

/// Parse payment methods from the Payment Methods table.
fn parse_payment_methods(content: &str) -> serde_json::Value {
    let mut methods: Vec<PaymentMethod> = Vec::new();

    // Find the payment methods section
    let Some(start) = content.find("### 2.4 Payment Methods") else {
        return serde_json::json!([]);
    };

    // Find the next section (## 3.) to limit our search
    let section_end = content[start..]
        .find("\n## ")
        .map(|i| start + i)
        .unwrap_or(content.len());

    let section = &content[start..section_end];

    // Parse table rows: | Method | Usage | Trend |
    let table_re = Regex::new(r"\|\s*([^|]+)\s*\|\s*(\d+)%\s*\|\s*([^|]+)\s*\|").unwrap();

    for caps in table_re.captures_iter(section) {
        let method = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let usage = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("0");
        let trend = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");

        // Skip header row
        if method.to_lowercase() == "method" || method.contains("---") || method.is_empty() {
            continue;
        }

        let share = usage.parse::<f64>().unwrap_or(0.0) / 100.0;

        methods.push(PaymentMethod {
            method: method.to_string(),
            share,
            trend: trend.to_string(),
        });
    }

    serde_json::to_value(&methods).unwrap_or_else(|_| serde_json::json!([]))
}

/// Parse social networks from the Social Media Platforms table.
fn parse_social_networks(content: &str) -> serde_json::Value {
    let mut networks: Vec<SocialNetwork> = Vec::new();

    // Find the social media section
    let Some(start) = content.find("### 4.1 Social Media Platforms") else {
        return serde_json::json!([]);
    };

    // Find the next section to limit our search
    let section_end = content[start..]
        .find("\n### 4.2")
        .map(|i| start + i)
        .unwrap_or(content.len());

    let section = &content[start..section_end];

    // Parse table rows: | Platform | Penetration | Primary Audience | Best for |
    let table_re =
        Regex::new(r"\|\s*([^|]+)\s*\|\s*(\d+)%\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|").unwrap();

    for caps in table_re.captures_iter(section) {
        let platform = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let penetration = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("0");
        let audience = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");

        // Skip header row
        if platform.to_lowercase() == "platform" || platform.contains("---") || platform.is_empty()
        {
            continue;
        }

        let penetration_pct = penetration.parse::<f64>().unwrap_or(0.0) / 100.0;

        networks.push(SocialNetwork {
            name: platform.to_string(),
            penetration: penetration_pct,
            audience: audience.to_string(),
        });
    }

    serde_json::to_value(&networks).unwrap_or_else(|_| serde_json::json!([]))
}

/// Parse popular platforms from the Major Players table.
fn parse_popular_platforms(content: &str) -> serde_json::Value {
    let mut platforms: Vec<EcommercePlatform> = Vec::new();

    // Find the major players section
    let Some(start) = content.find("### 7.1 Major Players") else {
        return serde_json::json!([]);
    };

    // Find the next section to limit our search
    let section_end = content[start..]
        .find("\n### 7.2")
        .map(|i| start + i)
        .unwrap_or(content.len());

    let section = &content[start..section_end];

    // Parse table rows: | Company | Market Share | Strength |
    let table_re = Regex::new(r"\|\s*([^|]+)\s*\|\s*(\d+)%\s*\|\s*([^|]+)\s*\|").unwrap();

    for caps in table_re.captures_iter(section) {
        let company = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let share = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("0");
        let strength = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");

        // Skip header row
        if company.to_lowercase() == "company" || company.contains("---") || company.is_empty() {
            continue;
        }

        let market_share = share.parse::<f64>().unwrap_or(0.0) / 100.0;

        platforms.push(EcommercePlatform {
            name: company.to_string(),
            market_share,
            strength: strength.to_string(),
        });
    }

    serde_json::to_value(&platforms).unwrap_or_else(|_| serde_json::json!([]))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const FR_FR_SAMPLE: &str = r#"---
locale: fr-FR
type: market
template_version: 2.0
last_updated: 2025-01-10
---

# Market: fr-FR

## 1. Demographics

### 1.1 Population & Growth

| Metric | Value | Source |
|--------|-------|--------|
| Total population | 68.2 million | INSEE (2024) |
| Growth rate | 0.28% | INSEE (2024) |
| Median age | 42.3 years | Eurostat (2024) |

### 1.4 Urban/Rural Split

| Type | Percentage | E-commerce Adoption |
|------|------------|---------------------|
| Urban | 81% | Very High |
| Suburban | 12% | High |
| Rural | 7% | Medium |

---

## 2. Digital Maturity

### 2.1 Internet Penetration

| Metric | Value | Trend |
|--------|-------|-------|
| Internet users | 93% | Stable |
| Broadband | 87% | Growing |
| Mobile internet | 89% | Growing |

### 2.2 Mobile Usage

| Metric | Value | Notes |
|--------|-------|-------|
| Smartphone penetration | 92% | Near saturation |
| Mobile-first users | 58% | Especially under 35 |

### 2.3 E-commerce Adoption

| Metric | Value | Growth |
|--------|-------|--------|
| Online shoppers | 77% | +3% YoY |
| E-commerce revenue | 159.9 billion EUR | +10.5% YoY |

### 2.4 Payment Methods

| Method | Usage | Trend |
|--------|-------|-------|
| Credit cards | 62% | Stable |
| Debit cards | 48% | Stable |
| Digital wallets | 29% | Growing fast |
| BNPL | 18% | Growing |
| Cash on delivery | 3% | Declining |

---

## 3. Market Potential

### 3.3 ROI Priority Score

| Factor | Score (0-100) | Weight |
|--------|---------------|--------|
| Market size | 92 | 0.30 |
| Growth rate | 78 | 0.25 |
| Competition | 55 | 0.20 |
| Localization cost | 85 | 0.25 |
| **Overall** | **78.6** | - |

---

## 4. Channel Preferences

### 4.1 Social Media Platforms

| Platform | Penetration | Primary Audience | Best for |
|----------|-------------|------------------|----------|
| Facebook | 73% | 25-54 | Broad reach, retargeting |
| Instagram | 61% | 18-34 | Visual products, lifestyle |
| YouTube | 79% | All ages | Video content, tutorials |
| TikTok | 39% | 16-24 | Gen Z engagement, virality |
| LinkedIn | 31% | 25-54 professionals | B2B, career-related |

### 4.2 Messaging Apps

| App | Penetration | Use Case |
|-----|-------------|----------|
| WhatsApp | 68% | Personal messaging, customer service |

---

## 7. Competitive Landscape

### 7.1 Major Players

| Company | Market Share | Strength |
|---------|--------------|----------|
| Amazon.fr | 22% | Logistics, Prime ecosystem |
| Cdiscount | 8% | French origin, Cnova group |
| Fnac-Darty | 6% | Electronics, omnichannel |

### 7.2 Market Concentration

| Metric | Value | Implication |
|--------|-------|-------------|
| Top 3 share | 36% | Moderate concentration |
"#;

    #[test]
    fn test_parse_market_markdown_basic() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.locale_key, "fr-FR");
        assert_eq!(result.template_version, "2.0");
        assert_eq!(result.last_updated, "2025-01-10");
    }

    #[test]
    fn test_parse_population() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // Should extract 68.2 million
        assert!((result.population_millions - 68.2).abs() < 0.1);
    }

    #[test]
    fn test_parse_internet_penetration() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // Should extract 93% = 0.93
        assert!((result.internet_penetration - 0.93).abs() < 0.01);
    }

    #[test]
    fn test_parse_mobile_penetration() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // Should extract 92% = 0.92
        assert!((result.mobile_penetration - 0.92).abs() < 0.01);
    }

    #[test]
    fn test_parse_ecommerce_maturity() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // 77% online shoppers = mature
        assert_eq!(result.ecommerce_maturity, "mature");
    }

    #[test]
    fn test_parse_digital_maturity_score() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // Should extract ROI score 78.6 -> 79
        assert_eq!(result.digital_maturity_score, 79);
    }

    #[test]
    fn test_parse_payment_methods() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let methods = result.payment_methods.as_array().unwrap();
        assert_eq!(methods.len(), 5);

        // Check first method
        assert_eq!(methods[0]["method"], "Credit cards");
        assert!((methods[0]["share"].as_f64().unwrap() - 0.62).abs() < 0.01);
    }

    #[test]
    fn test_parse_social_networks() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let networks = result.social_networks.as_array().unwrap();
        assert_eq!(networks.len(), 5);

        // Check YouTube (highest penetration)
        let youtube = networks.iter().find(|n| n["name"] == "YouTube").unwrap();
        assert!((youtube["penetration"].as_f64().unwrap() - 0.79).abs() < 0.01);
    }

    #[test]
    fn test_parse_popular_platforms() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let platforms = result.popular_platforms.as_array().unwrap();
        assert_eq!(platforms.len(), 3);

        // Check Amazon
        let amazon = platforms.iter().find(|p| p["name"] == "Amazon.fr").unwrap();
        assert!((amazon["market_share"].as_f64().unwrap() - 0.22).abs() < 0.01);
    }

    #[test]
    fn test_parse_demographics_json() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!((result.demographics["median_age"].as_f64().unwrap() - 42.3).abs() < 0.1);
        assert!((result.demographics["urban_rate"].as_f64().unwrap() - 0.81).abs() < 0.01);
    }

    #[test]
    fn test_generate_market_summary() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.market_summary.is_empty());
        assert!(result.market_summary.contains("fr-FR"));
        assert!(result.market_summary.contains("68.2M"));
        assert!(result.market_summary.contains("mature"));
    }

    #[test]
    fn test_raw_markdown_preserved() {
        let result = parse_market_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.raw_markdown.is_empty());
        assert!(result.raw_markdown.contains("Market: fr-FR"));
    }

    // Test with en-US style content
    const EN_US_SAMPLE: &str = r#"---
locale: en-US
type: market
template_version: 2.0
last_updated: 2026-01-10
---

# Market: en-US

## 1. Demographics

### 1.1 Population & Growth

| Metric | Value | Source |
|--------|-------|--------|
| Total population | 335.9 million | US Census Bureau 2024 |
| Growth rate | 0.5% | US Census Bureau 2024 |
| Median age | 38.9 years | US Census Bureau 2024 |

### 1.4 Urban/Rural Split

| Type | Percentage | E-commerce Adoption |
|------|------------|---------------------|
| Urban | 57% | Very High |
| Suburban | 24% | Very High |
| Rural | 19% | Moderate |

---

## 2. Digital Maturity

### 2.1 Internet Penetration

| Metric | Value | Trend |
|--------|-------|-------|
| Internet users | 92% | Stable |
| Broadband | 90% | Stable |
| Mobile internet | 97% | Growing |

### 2.2 Mobile Usage

| Metric | Value | Notes |
|--------|-------|-------|
| Smartphone penetration | 91% | Near saturation |
| Mobile-first users | 55% | Growing |

### 2.3 E-commerce Adoption

| Metric | Value | Growth |
|--------|-------|--------|
| Online shoppers | 85% | +4% YoY |
| E-commerce revenue | $1.12 trillion USD | +9% YoY |

### 2.4 Payment Methods

| Method | Usage | Trend |
|--------|-------|-------|
| Credit cards | 38% | Stable, rewards-driven |
| Debit cards | 28% | Stable |
| Digital wallets | 24% | Growing |
| BNPL | 15% | Strong growth |
| Cash on delivery | 1% | Rare |

---

## 3. Market Potential

### 3.3 ROI Priority Score

| Factor | Score (0-100) | Weight |
|--------|---------------|--------|
| Market size | 100 | Very High |
| Growth rate | 85 | High |
| Competition | 35 | Very High (saturated) |
| Localization cost | 100 | None (source locale) |
| **Overall** | **95** | - |

---

## 4. Channel Preferences

### 4.1 Social Media Platforms

| Platform | Penetration | Primary Audience | Best for |
|----------|-------------|------------------|----------|
| YouTube | 83% | All ages | Product reviews, tutorials |
| Facebook | 68% | 30-65 | Community, marketplace |
| Instagram | 62% | 18-44 | Visual products, influencer |
| TikTok | 50% | 16-34 | Viral content, trends |
| LinkedIn | 32% | 25-54 professionals | B2B marketing |

### 4.2 Messaging Apps

| App | Penetration | Use Case |
|-----|-------------|----------|
| iMessage | 58% | Primary personal messaging |

---

## 7. Competitive Landscape

### 7.1 Major Players

| Company | Market Share | Strength |
|---------|--------------|----------|
| Amazon | 38% | Selection, Prime ecosystem |
| Walmart | 7% | Omnichannel, grocery |
| Apple | 4% | Hardware ecosystem |

### 7.2 Market Concentration

| Metric | Value | Implication |
|--------|-------|-------------|
| Top 3 share | 49% | Moderately consolidated |
"#;

    #[test]
    fn test_parse_large_population() {
        let result = parse_market_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        // Should extract 335.9 million
        assert!((result.population_millions - 335.9).abs() < 0.1);
    }

    #[test]
    fn test_parse_advanced_ecommerce() {
        let result = parse_market_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        // 85% online shoppers = advanced
        assert_eq!(result.ecommerce_maturity, "advanced");
    }

    #[test]
    fn test_parse_high_roi_score() {
        let result = parse_market_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        // Should extract ROI score 95
        assert_eq!(result.digital_maturity_score, 95);
    }

    #[test]
    fn test_infer_ecommerce_maturity_emerging() {
        assert_eq!(MarketData::infer_ecommerce_maturity(0.30), "emerging");
    }

    #[test]
    fn test_infer_ecommerce_maturity_growing() {
        assert_eq!(MarketData::infer_ecommerce_maturity(0.50), "growing");
    }

    #[test]
    fn test_infer_ecommerce_maturity_mature() {
        assert_eq!(MarketData::infer_ecommerce_maturity(0.70), "mature");
    }

    #[test]
    fn test_infer_ecommerce_maturity_advanced() {
        assert_eq!(MarketData::infer_ecommerce_maturity(0.85), "advanced");
    }
}
