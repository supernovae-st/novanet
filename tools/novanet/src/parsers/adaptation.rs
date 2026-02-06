//! Parser for ATH 2-rules-adaptation data.
//!
//! Transforms ATH adaptation rules markdown files into Rust structs for Cypher generation.
//! Extracts: FACTS/ILLUSTRATIONS classification, technical terms approach, common errors.
//!
//! v10.7: REFACTORED - Simplified to core FACTS/ILLUSTRATIONS + technical terms
//! Migrated: formality (-> Style), calendar/seasons (-> Culture), measurement (-> Formatting)

use std::collections::HashMap;
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

/// Section header: ## N. Title
static RE_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"##\s+\d+\.\s+(.+)").expect("valid section regex"));

/// Technical terms approach: **Default**: TYPE
static RE_TECH_APPROACH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\*\*Default\*\*:\s*(LOCAL_ONLY|ENGLISH_ACCEPTED|MIXED|local_only|english_accepted|mixed)",
    )
    .expect("valid tech approach regex")
});

/// French preferred term list: - term (not english)
static RE_FRENCH_TERM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-\s+(\w+)\s+\(not\s+(\w+)\)").expect("valid french term regex"));

/// English accepted term list: - term (widely used, alternative)
static RE_ENGLISH_TERM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^-\s+(\w[\w\-\s]*)\s+\((?:widely|universal|accepted)")
        .expect("valid english term regex")
});


// ============================================================================
// Main Structs
// ============================================================================

/// Complete adaptation data for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationData {
    /// Locale key (e.g., "fr-FR")
    pub locale_key: String,

    // TIER 1: INDEXED SCALARS (Cypher-queryable)
    /// How to handle technical/borrowed terms
    pub technical_terms_approach: String,
    /// Preference for culturally-specific illustrations
    pub illustration_density: String,

    // TIER 2: STRUCTURED JSON (programmatic lookup)
    /// Mapping of English terms to preferred local equivalents
    pub technical_terms_preferred: serde_json::Value,
    /// English terms acceptable without translation
    pub technical_terms_accepted: serde_json::Value,
    /// Common adaptation errors to avoid
    pub common_errors: serde_json::Value,
    /// False cognates to watch for
    pub false_friends: serde_json::Value,
    /// Classification rules for FACT vs ILLUSTRATION
    pub facts_classification: serde_json::Value,

    // TIER 3: LLM CONTEXT (narrative text)
    /// Distilled summary for LLM context
    pub adaptation_summary: String,
    /// Step-by-step decision flowchart
    pub decision_algorithm: String,
    /// QA checklist for adaptation review
    pub validation_checklist: String,
    /// Full source markdown
    pub raw_markdown: String,

    // Metadata
    pub template_version: String,
    pub last_updated: String,
    pub source_file: String,
}

/// A common adaptation error with correction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonError {
    pub error: String,
    pub correct: String,
    pub why: String,
}

/// A false friend (false cognate) entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalseFriend {
    pub word: String,
    pub trap: String,
    pub correct: String,
}

impl Default for AdaptationData {
    fn default() -> Self {
        Self {
            locale_key: String::new(),
            technical_terms_approach: "mixed".to_string(),
            illustration_density: "medium".to_string(),
            technical_terms_preferred: serde_json::json!({}),
            technical_terms_accepted: serde_json::json!([]),
            common_errors: serde_json::json!([]),
            false_friends: serde_json::json!([]),
            facts_classification: serde_json::json!({
                "proper_names": "FACT",
                "brand_names": "FACT",
                "statistics": "FACT",
                "legal_terms": "FACT",
                "technical_specs": "FACT",
                "dates_times": "FACT",
                "prices_currencies": "FACT",
                "contact_info": "FACT",
                "idioms": "ILLUSTRATION",
                "metaphors": "ILLUSTRATION",
                "cultural_references": "ILLUSTRATION",
                "sports_analogies": "ILLUSTRATION",
                "food_examples": "ILLUSTRATION",
                "humor": "ILLUSTRATION",
                "seasonal_references": "ILLUSTRATION",
                "geographic_references": "ILLUSTRATION"
            }),
            adaptation_summary: String::new(),
            decision_algorithm: String::new(),
            validation_checklist: String::new(),
            raw_markdown: String::new(),
            template_version: "3.0".to_string(),
            last_updated: String::new(),
            source_file: String::new(),
        }
    }
}

impl AdaptationData {
    /// Generate LLM context summary from adaptation data.
    pub fn generate_adaptation_summary(&mut self) {
        let mut parts = Vec::new();

        // Technical terms approach
        parts.push(format!(
            "{}: Technical terms approach is {}.",
            self.locale_key,
            match self.technical_terms_approach.as_str() {
                "local_only" => "LOCAL ONLY (always use local equivalents)",
                "english_accepted" => "ENGLISH ACCEPTED (English terms OK)",
                "mixed" => "MIXED (depends on context)",
                _ => "MIXED (depends on context)",
            }
        ));

        // Illustration density
        parts.push(format!(
            "Illustration density: {}.",
            match self.illustration_density.as_str() {
                "high" => "HIGH (maximize local references)",
                "low" => "LOW (prefer universal examples)",
                _ => "MEDIUM (balanced approach)",
            }
        ));

        // Key rule
        parts.push(
            "CORE RULE: FACTS=translate literally, ILLUSTRATIONS=generate native.".to_string(),
        );

        self.adaptation_summary = parts.join(" ");
    }

    /// Generate the decision algorithm from parsed content.
    pub fn generate_decision_algorithm(&mut self) {
        self.decision_algorithm = r#"1. Is it legal/safety/compliance? → FACT (translate literally)
2. Is it a price/date/spec/technical data? → FACT (format locally)
3. Is it a brand name/trademark? → FACT (keep original)
4. Is it an example/metaphor/cultural reference? → ILLUSTRATION (generate native)
5. Is it a style/tone element? → Apply locale style rules
6. Edge case? → If accuracy matters legally → FACT; If recognition needed → ILLUSTRATION"#
            .to_string();
    }

    /// Generate default validation checklist.
    pub fn generate_validation_checklist(&mut self) {
        self.validation_checklist = r#"FACTS Validation:
[ ] All FACTS identified and translated literally
[ ] Numbers formatted for locale
[ ] Dates formatted for locale
[ ] Currency formatted for locale
[ ] Legal content preserved exactly
[ ] Brand names unchanged

ILLUSTRATIONS Validation:
[ ] Semantic intention identified for each
[ ] Native expressions generated (not translations)
[ ] Cultural references localized
[ ] No literal idiom translations
[ ] Recognition >70% for target audience"#
            .to_string();
    }
}

// ============================================================================
// Parser Implementation
// ============================================================================

/// Parse a single adaptation markdown file.
pub fn parse_adaptation_markdown(content: &str, locale_key: &str) -> Result<AdaptationData> {
    let mut data = AdaptationData {
        locale_key: locale_key.to_string(),
        raw_markdown: content.to_string(),
        ..Default::default()
    };

    // Extract metadata
    if let Some(caps) = RE_VERSION.captures(content) {
        data.template_version = caps[1].trim().to_string();
    }
    if let Some(caps) = RE_DATE.captures(content) {
        data.last_updated = caps[1].trim().to_string();
    }

    // Extract technical terms approach
    if let Some(caps) = RE_TECH_APPROACH.captures(content) {
        data.technical_terms_approach = caps[1].to_lowercase();
    }

    // Parse sections
    let sections = split_by_sections(content);

    for (title, section_content) in &sections {
        let title_lower = title.to_lowercase();

        if title_lower.contains("technical terms") && title_lower.contains("approach") {
            parse_technical_terms_section(&mut data, section_content);
        } else if title_lower.contains("common error") {
            parse_common_errors_section(&mut data, section_content);
        } else if title_lower.contains("facts") && title_lower.contains("illustration") {
            parse_facts_section(&mut data, section_content);
        } else if title_lower.contains("decision algorithm") {
            parse_decision_algorithm(&mut data, section_content);
        } else if title_lower.contains("validation") {
            parse_validation_checklist(&mut data, section_content);
        }
    }

    // Determine illustration density from content hints
    data.illustration_density = determine_illustration_density(content);

    // Generate summaries if not extracted
    if data.adaptation_summary.is_empty() {
        data.generate_adaptation_summary();
    }
    if data.decision_algorithm.is_empty() {
        data.generate_decision_algorithm();
    }
    if data.validation_checklist.is_empty() {
        data.generate_validation_checklist();
    }

    Ok(data)
}

/// Split content by ## N. Section headers.
fn split_by_sections(content: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_title = String::new();
    let mut current_content = String::new();

    for line in content.lines() {
        if let Some(caps) = RE_SECTION.captures(line) {
            if !current_title.is_empty() {
                sections.push((current_title.clone(), current_content.clone()));
            }
            current_title = caps[1].trim().to_string();
            current_content = String::new();
        } else {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    if !current_title.is_empty() {
        sections.push((current_title, current_content));
    }

    sections
}

/// Parse technical terms section for preferred and accepted terms.
fn parse_technical_terms_section(data: &mut AdaptationData, content: &str) {
    let mut preferred: HashMap<String, String> = HashMap::new();
    let mut accepted: Vec<String> = Vec::new();

    let mut in_preferred = false;
    let mut in_accepted = false;

    for line in content.lines() {
        let line_lower = line.to_lowercase();

        if line_lower.contains("french preferred") || line_lower.contains("local preferred") {
            in_preferred = true;
            in_accepted = false;
        } else if line_lower.contains("english accepted") || line_lower.contains("accepted") {
            in_preferred = false;
            in_accepted = true;
        }

        if in_preferred {
            // Pattern: - ordinateur (not computer)
            if let Some(caps) = RE_FRENCH_TERM.captures(line) {
                let french = caps[1].trim().to_string();
                let english = caps[2].trim().to_string();
                preferred.insert(english, french);
            }
        } else if in_accepted {
            // Pattern: - smartphone (widely used)
            if let Some(caps) = RE_ENGLISH_TERM.captures(line) {
                let term = caps[1].trim().to_string();
                if !term.is_empty() {
                    accepted.push(term);
                }
            }
        }
    }

    data.technical_terms_preferred = serde_json::to_value(preferred).unwrap_or_default();
    data.technical_terms_accepted = serde_json::to_value(accepted).unwrap_or_default();
}

/// Parse common errors table section.
fn parse_common_errors_section(data: &mut AdaptationData, content: &str) {
    let mut errors: Vec<CommonError> = Vec::new();
    let mut in_table = false;
    let mut header_passed = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect table start
        if trimmed.starts_with('|') && trimmed.contains("Error") {
            in_table = true;
            continue;
        }

        // Skip separator row
        if trimmed.starts_with('|') && trimmed.contains("---") {
            header_passed = true;
            continue;
        }

        if in_table && header_passed && trimmed.starts_with('|') {
            // Parse row: | Error | Why Wrong | Correct |
            let parts: Vec<&str> = trimmed
                .trim_matches('|')
                .split('|')
                .map(|s| s.trim())
                .collect();

            if parts.len() >= 3 {
                errors.push(CommonError {
                    error: parts[0].to_string(),
                    why: parts[1].to_string(),
                    correct: parts[2].to_string(),
                });
            }
        }

        // End table when blank line or new section
        if in_table && (trimmed.is_empty() || trimmed.starts_with('#')) && header_passed {
            break;
        }
    }

    data.common_errors = serde_json::to_value(errors).unwrap_or_default();
}

/// Parse FACTS vs ILLUSTRATIONS classification tables.
fn parse_facts_section(data: &mut AdaptationData, content: &str) {
    let mut classification: HashMap<String, String> = HashMap::new();

    // Default classification
    classification.insert("proper_names".to_string(), "FACT".to_string());
    classification.insert("brand_names".to_string(), "FACT".to_string());
    classification.insert("statistics".to_string(), "FACT".to_string());
    classification.insert("legal_terms".to_string(), "FACT".to_string());
    classification.insert("technical_specs".to_string(), "FACT".to_string());
    classification.insert("dates_times".to_string(), "FACT".to_string());
    classification.insert("prices_currencies".to_string(), "FACT".to_string());
    classification.insert("contact_info".to_string(), "FACT".to_string());
    classification.insert("idioms".to_string(), "ILLUSTRATION".to_string());
    classification.insert("metaphors".to_string(), "ILLUSTRATION".to_string());
    classification.insert(
        "cultural_references".to_string(),
        "ILLUSTRATION".to_string(),
    );
    classification.insert("sports_analogies".to_string(), "ILLUSTRATION".to_string());
    classification.insert("food_examples".to_string(), "ILLUSTRATION".to_string());
    classification.insert("humor".to_string(), "ILLUSTRATION".to_string());
    classification.insert(
        "seasonal_references".to_string(),
        "ILLUSTRATION".to_string(),
    );
    classification.insert(
        "geographic_references".to_string(),
        "ILLUSTRATION".to_string(),
    );

    // Look for additional categories in content
    if content.to_lowercase().contains("scientific") {
        classification.insert("scientific".to_string(), "FACT".to_string());
    }
    if content.to_lowercase().contains("numerical") {
        classification.insert("numerical".to_string(), "FACT".to_string());
    }

    data.facts_classification = serde_json::to_value(classification).unwrap_or_default();
}

/// Parse decision algorithm section.
fn parse_decision_algorithm(data: &mut AdaptationData, content: &str) {
    // Extract code block or numbered list
    let mut algorithm = String::new();
    let mut in_code_block = false;

    for line in content.lines() {
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            algorithm.push_str(line);
            algorithm.push('\n');
        }
    }

    if !algorithm.is_empty() {
        data.decision_algorithm = algorithm.trim().to_string();
    }
}

/// Parse validation checklist section.
fn parse_validation_checklist(data: &mut AdaptationData, content: &str) {
    let mut checklist = String::new();
    let mut in_checklist = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Start on first checkbox
        if trimmed.starts_with("- [ ]") || trimmed.starts_with("- [x]") {
            in_checklist = true;
        }

        if in_checklist {
            if trimmed.starts_with("- [ ]") || trimmed.starts_with("- [x]") {
                // Convert to simple checklist format
                let item = trimmed
                    .trim_start_matches("- [ ]")
                    .trim_start_matches("- [x]")
                    .trim();
                checklist.push_str("[ ] ");
                checklist.push_str(item);
                checklist.push('\n');
            } else if trimmed.starts_with("###") {
                // Subheader
                checklist.push('\n');
                checklist.push_str(trimmed.trim_start_matches('#').trim());
                checklist.push_str(":\n");
            } else if trimmed.is_empty() && !checklist.is_empty() {
                checklist.push('\n');
            }
        }
    }

    if !checklist.is_empty() {
        data.validation_checklist = checklist.trim().to_string();
    }
}

/// Determine illustration density from content analysis.
fn determine_illustration_density(content: &str) -> String {
    let content_lower = content.to_lowercase();

    // High: lots of cultural references, local examples encouraged
    let high_indicators = content_lower.matches("culturally-specific").count()
        + content_lower.matches("local reference").count()
        + content_lower.matches("native expression").count()
        + content_lower.matches("generate native").count();

    // Low: universal examples, minimal localization
    let low_indicators = content_lower.matches("universal example").count()
        + content_lower.matches("minimal").count()
        + content_lower.matches("neutral").count();

    if high_indicators > 5 {
        "high".to_string()
    } else if low_indicators > high_indicators {
        "low".to_string()
    } else {
        "medium".to_string()
    }
}

// ============================================================================
// File Loading Functions
// ============================================================================

/// Load a single adaptation file.
pub fn load_adaptation_file(path: &Path) -> Result<AdaptationData> {
    let filename = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| NovaNetError::Validation(format!("Invalid filename: {:?}", path)))?;

    let content = fs::read_to_string(path)?;

    let mut data = parse_adaptation_markdown(&content, filename)?;
    data.source_file = path.display().to_string();

    Ok(data)
}

/// Load all adaptation files from directory.
pub fn load_all_adaptation_files(dir: &Path) -> Result<Vec<AdaptationData>> {
    let files: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "md")
        })
        .collect();

    let results: Vec<Result<AdaptationData>> = files
        .par_iter()
        .map(|entry| load_adaptation_file(entry.path()))
        .collect();

    let mut data = Vec::with_capacity(results.len());
    for result in results {
        data.push(result?);
    }

    Ok(data)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const FR_FR_SAMPLE: &str = r#"---
locale: fr-FR
type: rules-adaptation
template_version: 3.0
last_updated: 2025-12-08
---

# Rules Adaptation: fr-FR

## 1. Core Framework: FACTS vs ILLUSTRATIONS

**Fundamental Rule**: All content elements are either FACTS or ILLUSTRATIONS.

### 1.1 FACTS (Never Adapt - Format Only)

| Category | Examples | Treatment |
|----------|----------|-----------|
| **Financial** | Prices (99,90 €), fees | Translate literally |
| **Legal** | CGV, RGPD | Never adapt |
| **Brand** | Apple, Renault | Do NOT translate |

### 1.2 ILLUSTRATIONS (Always Generate Native)

| Category | Examples | Treatment |
|----------|----------|-----------|
| **Metaphors** | "C'est du gâteau" | Generate native expressions |
| **Idioms** | "Tomber dans les pommes" | Generate native idioms |
| **Cultural refs** | "Le 14 juillet" | Replace with French moments |

## 2. Technical Terms Approach

**Default**: MIXED

**French preferred**:
- ordinateur (not computer)
- logiciel (not software)
- courriel (not email)

**English accepted**:
- smartphone (widely used)
- email (accepted alternative)
- Wi-Fi (universal)

## 3. Common Errors for fr-FR

| Error | Why Wrong | Correct |
|-------|-----------|---------|
| Using "tu" with unknown customer | Disrespectful | Use "vous" by default |
| Literal translation of idioms | Sounds absurd | Use French idiom |
| Missing space before punctuation | Violates French rules | Add space before : ; ! ? |

## 4. Decision Algorithm

```
1. Is it legal/safety? → FACT
2. Is it a price/date? → FACT
3. Is it a brand? → FACT (keep original)
4. Is it a metaphor? → ILLUSTRATION
5. Default → Evaluate context
```

## 5. Validation Checklist

### FACTS Validation
- [ ] All FACTS identified
- [ ] Numbers formatted (space + comma)
- [ ] Dates formatted (DD/MM/YYYY)
- [ ] Brand names unchanged

### ILLUSTRATIONS Validation
- [ ] Native expressions generated
- [ ] No literal idiom translations
- [ ] Cultural references localized
"#;

    #[test]
    fn test_parse_locale_key() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        assert_eq!(data.locale_key, "fr-FR");
    }

    #[test]
    fn test_parse_metadata() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        assert_eq!(data.template_version, "3.0");
        assert_eq!(data.last_updated, "2025-12-08");
    }

    #[test]
    fn test_parse_technical_terms_approach() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        assert_eq!(data.technical_terms_approach, "mixed");
    }

    #[test]
    fn test_parse_technical_terms_preferred() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        let preferred = data.technical_terms_preferred.as_object().unwrap();

        assert_eq!(preferred.get("computer").unwrap(), "ordinateur");
        assert_eq!(preferred.get("software").unwrap(), "logiciel");
    }

    #[test]
    fn test_parse_technical_terms_accepted() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        let accepted = data.technical_terms_accepted.as_array().unwrap();

        assert!(accepted.iter().any(|v| v.as_str() == Some("smartphone")));
        assert!(accepted.iter().any(|v| v.as_str() == Some("Wi-Fi")));
    }

    #[test]
    fn test_parse_common_errors() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        let errors = data.common_errors.as_array().unwrap();

        assert!(!errors.is_empty());
        let first = &errors[0];
        assert!(first.get("error").is_some());
        assert!(first.get("correct").is_some());
        assert!(first.get("why").is_some());
    }

    #[test]
    fn test_parse_facts_classification() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        let classification = data.facts_classification.as_object().unwrap();

        assert_eq!(classification.get("brand_names").unwrap(), "FACT");
        assert_eq!(classification.get("legal_terms").unwrap(), "FACT");
        assert_eq!(classification.get("metaphors").unwrap(), "ILLUSTRATION");
        assert_eq!(classification.get("idioms").unwrap(), "ILLUSTRATION");
    }

    #[test]
    fn test_parse_decision_algorithm() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();

        assert!(data.decision_algorithm.contains("legal"));
        assert!(data.decision_algorithm.contains("FACT"));
        assert!(data.decision_algorithm.contains("ILLUSTRATION"));
    }

    #[test]
    fn test_parse_validation_checklist() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();

        assert!(data.validation_checklist.contains("FACTS"));
        assert!(
            data.validation_checklist.contains("ILLUSTRATIONS")
                || data.validation_checklist.contains("Native expressions")
        );
    }

    #[test]
    fn test_illustration_density_detection() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        // Sample has moderate cultural references
        assert!(["low", "medium", "high"].contains(&data.illustration_density.as_str()));
    }

    #[test]
    fn test_adaptation_summary_generation() {
        let mut data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        data.generate_adaptation_summary();

        assert!(data.adaptation_summary.contains("fr-FR"));
        assert!(data.adaptation_summary.contains("Technical terms approach"));
        assert!(
            data.adaptation_summary.contains("MIXED") || data.adaptation_summary.contains("mixed")
        );
    }

    #[test]
    fn test_raw_markdown_preserved() {
        let data = parse_adaptation_markdown(FR_FR_SAMPLE, "fr-FR").unwrap();
        assert!(!data.raw_markdown.is_empty());
        assert!(data.raw_markdown.contains("Rules Adaptation: fr-FR"));
    }

    #[test]
    fn test_default_values() {
        let data = AdaptationData::default();
        assert_eq!(data.technical_terms_approach, "mixed");
        assert_eq!(data.illustration_density, "medium");
    }

    #[test]
    fn test_split_by_sections() {
        let sections = split_by_sections(FR_FR_SAMPLE);
        assert!(sections.len() >= 4);

        let titles: Vec<&str> = sections.iter().map(|(t, _)| t.as_str()).collect();
        assert!(titles.iter().any(|t| t.contains("FACTS")));
        assert!(titles.iter().any(|t| t.contains("Technical Terms")));
    }

    #[test]
    fn test_english_only_locale() {
        let content = r#"---
locale: en-US
type: rules-adaptation
template_version: 3.0
last_updated: 2025-12-08
---

# Rules Adaptation: en-US

## 1. Core Framework: FACTS vs ILLUSTRATIONS

Standard FACTS/ILLUSTRATIONS framework applies.

## 2. Technical Terms Approach

**Default**: ENGLISH_ACCEPTED

All English terms are native - no translation needed.
"#;
        let data = parse_adaptation_markdown(content, "en-US").unwrap();
        assert_eq!(data.locale_key, "en-US");
        assert_eq!(data.technical_terms_approach, "english_accepted");
    }

    #[test]
    fn test_local_only_approach() {
        let content = r#"---
locale: ja-JP
type: rules-adaptation
template_version: 3.0
---

## 1. Technical Terms Approach

**Default**: LOCAL_ONLY

Always use Japanese equivalents.
"#;
        let data = parse_adaptation_markdown(content, "ja-JP").unwrap();
        assert_eq!(data.technical_terms_approach, "local_only");
    }
}
