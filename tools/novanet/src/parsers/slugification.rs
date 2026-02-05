//! Parser for ATH 2-rules-slug markdown files.
//!
//! Parses localization slug rules from ATH outputs into structured data
//! for Neo4j ingestion as SlugRule and Slugification nodes.

use std::collections::HashMap;
use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{NovaNetError, Result};

// ============================================================================
// Data Structures
// ============================================================================

/// Aggregated slug rule (5 total: latin_strip, latin_preserve, latin_transform, native_script, romanized)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlugRule {
    pub key: String,
    pub display_name: String,
    pub output_encoding: String,
    pub has_case: bool,
    pub normalization: String,
    pub diacritics: String,
    pub non_latin: String,
    pub case_handling: String,
    pub spaces: String,
    pub special_chars: String,
    pub char_transforms: Option<HashMap<String, String>>,
    pub romanization_systems: Option<HashMap<String, String>>,
    pub locale_count: u32,
    pub locale_examples: Vec<String>,
    pub llm_context: String,
}

impl SlugRule {
    /// Create a new SlugRule from its key, inferring properties from the rule type.
    pub fn new(key: &str) -> Self {
        let (display_name, output_encoding, has_case, normalization, diacritics, non_latin, char_transforms, romanization_systems) = match key {
            "latin_strip" => (
                "Latin Strip",
                "ASCII",
                true,
                "NFD",
                "remove",
                "transliterate",
                None,
                None,
            ),
            "latin_preserve" => (
                "Latin Preserve",
                "UTF-8",
                true,
                "NFC",
                "preserve",
                "transliterate",
                None,
                None,
            ),
            "latin_transform" => (
                "Latin Transform",
                "ASCII",
                true,
                "custom",
                "transform",
                "transliterate",
                Some(HashMap::from([
                    ("ß".to_string(), "ss".to_string()),
                    ("ü".to_string(), "ue".to_string()),
                    ("ö".to_string(), "oe".to_string()),
                    ("ä".to_string(), "ae".to_string()),
                ])),
                None,
            ),
            "native_script" => (
                "Native Script",
                "UTF-8",
                false,
                "NFC",
                "script_specific",
                "keep",
                None,
                None,
            ),
            "romanized" => (
                "Romanized",
                "ASCII",
                true,
                "NFD",
                "remove",
                "romanize",
                None,
                Some(HashMap::from([
                    ("ja".to_string(), "hepburn".to_string()),
                    ("zh".to_string(), "pinyin".to_string()),
                    ("ko".to_string(), "revised".to_string()),
                ])),
            ),
            _ => (
                key,
                "ASCII",
                true,
                "NFD",
                "remove",
                "transliterate",
                None,
                None,
            ),
        };

        Self {
            key: key.to_string(),
            display_name: display_name.to_string(),
            output_encoding: output_encoding.to_string(),
            has_case,
            normalization: normalization.to_string(),
            diacritics: diacritics.to_string(),
            non_latin: non_latin.to_string(),
            case_handling: if has_case { "lowercase" } else { "none" }.to_string(),
            spaces: "hyphen".to_string(),
            special_chars: "removed".to_string(),
            char_transforms,
            romanization_systems,
            locale_count: 0,
            locale_examples: Vec::new(),
            llm_context: String::new(),
        }
    }

    /// Generate llm_context from aggregated data.
    pub fn generate_llm_context(&mut self) {
        let diacritics_desc = match self.diacritics.as_str() {
            "remove" => "removed via NFD normalization",
            "preserve" => "preserved in UTF-8",
            "transform" => "transformed to ASCII equivalents (ß→ss, ü→ue)",
            "script_specific" => "handled per-script (Arabic removes tashkeel, Thai converts numerals)",
            _ => &self.diacritics,
        };

        let non_latin_desc = match self.non_latin.as_str() {
            "transliterate" => "transliterated to ASCII",
            "keep" => "preserved in native script",
            "romanize" => "romanized via standard systems (Hepburn, Pinyin, Revised)",
            _ => &self.non_latin,
        };

        let examples = if self.locale_examples.len() >= 3 {
            self.locale_examples[..3].join(", ")
        } else {
            self.locale_examples.join(", ")
        };

        self.llm_context = format!(
            "The {} rule produces {} slugs. Diacritics are {}. Non-Latin scripts are {}. Used by {} locales including {}.",
            self.display_name,
            self.output_encoding,
            diacritics_desc,
            non_latin_desc,
            self.locale_count,
            examples
        );
    }
}

/// Per-locale slugification configuration (200 nodes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slugification {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub llm_context: String,
    pub slug_rule: String,
    pub stopwords: HashMap<String, Vec<String>>,
    pub stopwords_count: u32,
    pub regional_additions: Vec<RegionalAddition>,
    pub script_config: Option<ScriptConfig>,
    pub validation_overrides: Option<HashMap<String, String>>,
    pub warnings: Vec<Warning>,
    pub examples: Vec<SlugExample>,
    pub template_version: String,
    pub source_file: String,
    pub last_updated: String,
}

impl Slugification {
    /// Generate llm_context from parsed data.
    pub fn generate_llm_context(&mut self) {
        let categories: Vec<&str> = self.stopwords.keys().map(|s| s.as_str()).collect();
        let category_count = categories.len();

        let top_categories = if categories.len() >= 3 {
            categories[..3].join(", ")
        } else {
            categories.join(", ")
        };

        let regional_note = if !self.regional_additions.is_empty() {
            format!(
                " Has {} regional additions.",
                self.regional_additions.len()
            )
        } else {
            String::new()
        };

        self.llm_context = format!(
            "URL slugification rules for {}. Uses {} rule. {} stopwords across {} categories including {}.{}",
            self.display_name.replace(" Slugification", ""),
            self.slug_rule,
            self.stopwords_count,
            category_count,
            top_categories,
            regional_note
        );
    }
}

/// Regional stopword addition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalAddition {
    pub word: String,
    pub category: String,
    pub reason: String,
}

/// Script-specific configuration for non-Latin locales.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptConfig {
    pub primary_script: String,
    pub diacritic_handling: Option<String>,
    pub numeral_handling: Option<String>,
    pub special_chars: Option<Vec<String>>,
}

/// Validation warning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub condition: String,
    pub message: String,
}

/// Slug example from documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlugExample {
    pub input: String,
    pub output: String,
    pub rules_applied: Vec<String>,
}

/// YAML frontmatter from markdown files.
#[derive(Debug, Clone, Deserialize)]
struct Frontmatter {
    locale: String,
    #[serde(rename = "type")]
    _type: String,
    template_version: String,
    last_updated: String,
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse a single 2-rules-slug markdown file into a Slugification struct.
pub fn parse_slugification_file(path: &Path) -> Result<Slugification> {
    let content = std::fs::read_to_string(path)?;
    parse_slugification(&content, path)
}

/// Parse slugification from markdown content.
pub fn parse_slugification(content: &str, source_path: &Path) -> Result<Slugification> {
    // 1. Parse YAML frontmatter
    let frontmatter = parse_frontmatter(content)?;

    // 2. Extract slug rule from "**Slug Rule**: {rule}"
    let slug_rule = extract_slug_rule(content)?;

    // 3. Extract stopwords from tables
    let stopwords = extract_stopwords(content);

    // 4. Extract regional additions
    let regional_additions = extract_regional_additions(content);

    // 5. Extract examples
    let examples = extract_examples(content);

    // 6. Extract warnings from validation section
    let warnings = extract_warnings(content);

    // 7. Extract script config for non-Latin
    let script_config = extract_script_config(content, &slug_rule);

    // 8. Count stopwords
    let stopwords_count: u32 = stopwords.values().map(|v| v.len() as u32).sum();

    // 9. Get locale display name
    let display_name = get_locale_display_name(&frontmatter.locale);

    // 10. Build Slugification
    let mut slugification = Slugification {
        key: frontmatter.locale.clone(),
        display_name: format!("{} Slugification", display_name),
        description: format!("URL slug generation rules for {}", frontmatter.locale),
        llm_context: String::new(),
        slug_rule,
        stopwords,
        stopwords_count,
        regional_additions,
        script_config,
        validation_overrides: None,
        warnings,
        examples,
        template_version: frontmatter.template_version,
        source_file: format!(
            "2-rules-slug/{}.md",
            source_path.file_stem().unwrap_or_default().to_string_lossy()
        ),
        last_updated: frontmatter.last_updated,
    };

    slugification.generate_llm_context();

    Ok(slugification)
}

/// Parse YAML frontmatter from markdown.
fn parse_frontmatter(content: &str) -> Result<Frontmatter> {
    let re = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();

    let caps = re.captures(content).ok_or_else(|| NovaNetError::Validation(
        "No YAML frontmatter found".to_string()
    ))?;

    let yaml_str = caps.get(1).unwrap().as_str();
    serde_yaml::from_str(yaml_str).map_err(|e| NovaNetError::Validation(
        format!("Failed to parse frontmatter: {}", e)
    ))
}

/// Extract slug rule from "**Slug Rule**: {rule}".
fn extract_slug_rule(content: &str) -> Result<String> {
    let re = Regex::new(r"\*\*Slug Rule\*\*:\s*(\w+)").unwrap();

    re.captures(content)
        .and_then(|caps: regex::Captures| caps.get(1))
        .map(|m: regex::Match| m.as_str().to_string())
        .ok_or_else(|| NovaNetError::Validation(
            "Could not find Slug Rule in content".to_string()
        ))
}

/// Extract stopwords from markdown tables.
fn extract_stopwords(content: &str) -> HashMap<String, Vec<String>> {
    let mut stopwords: HashMap<String, Vec<String>> = HashMap::new();

    // Match table rows: | word | category |
    let re = Regex::new(r"\|\s*([^\|]+?)\s*\|\s*(article|preposition|conjunction|pronoun|verb|contraction|demonstrative|auxiliary|possessive|interrogative|negation|adverb|particle_\w+|honorific|classifier|copula|proper_noun|currency|relative_pronoun|indefinite|quantifier|interjection|abbreviation|filler|honorific_suffix)\s*\|").unwrap();

    for caps in re.captures_iter(content) {
        let word = caps.get(1).unwrap().as_str().trim().to_string();
        let category = caps.get(2).unwrap().as_str().trim().to_string();

        // Skip header rows
        if word == "Word" || word.contains("---") {
            continue;
        }

        stopwords
            .entry(category)
            .or_insert_with(Vec::new)
            .push(word);
    }

    stopwords
}

/// Extract regional additions from "Locale-Specific Additions" table.
fn extract_regional_additions(content: &str) -> Vec<RegionalAddition> {
    let mut additions = Vec::new();

    // Find the Locale-Specific Additions section
    if !content.contains("Locale-Specific Additions") {
        return additions;
    }

    // Match table rows with reason: | word | category | reason |
    let re = Regex::new(r"\|\s*([^\|]+?)\s*\|\s*(\w+)\s*\|\s*([^\|]+?)\s*\|").unwrap();

    // Find the section start
    let section_start = content.find("Locale-Specific Additions").unwrap_or(0);
    let section_content = &content[section_start..];

    // Find the end of section (next ## or end)
    let section_end = section_content[1..]
        .find("\n## ")
        .map(|i| i + 1)
        .unwrap_or(section_content.len());
    let section = &section_content[..section_end];

    for caps in re.captures_iter(section) {
        let word = caps.get(1).unwrap().as_str().trim().to_string();
        let category = caps.get(2).unwrap().as_str().trim().to_string();
        let reason = caps.get(3).unwrap().as_str().trim().to_string();

        // Skip header rows
        if word == "Word" || word.contains("---") || category == "Category" {
            continue;
        }

        additions.push(RegionalAddition {
            word,
            category,
            reason,
        });
    }

    additions
}

/// Extract examples from the Examples section.
fn extract_examples(content: &str) -> Vec<SlugExample> {
    let mut examples = Vec::new();

    // Find the Examples section
    let section_start = match content.find("## 4. Examples") {
        Some(pos) => pos,
        None => return examples,
    };

    let section_content = &content[section_start..];
    let section_end = section_content[1..]
        .find("\n## ")
        .map(|i| i + 1)
        .unwrap_or(section_content.len());
    let section = &section_content[..section_end];

    // Match table rows: | input | output | rules |
    let re = Regex::new(r"\|\s*([^\|]+?)\s*\|\s*([^\|]+?)\s*\|\s*([^\|]+?)\s*\|").unwrap();

    for caps in re.captures_iter(section) {
        let input = caps.get(1).unwrap().as_str().trim().to_string();
        let output = caps.get(2).unwrap().as_str().trim().to_string();
        let rules_str = caps.get(3).unwrap().as_str().trim();

        // Skip header rows
        if input == "Input" || input.contains("---") {
            continue;
        }

        let rules_applied: Vec<String> = rules_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        examples.push(SlugExample {
            input,
            output,
            rules_applied,
        });
    }

    examples
}

/// Extract warnings from validation section.
fn extract_warnings(content: &str) -> Vec<Warning> {
    let mut warnings = Vec::new();

    // Find the Warnings subsection
    let warnings_start = match content.find("### Warnings") {
        Some(pos) => pos,
        None => return warnings,
    };

    let section_content = &content[warnings_start..];
    let section_end = section_content[1..]
        .find("\n## ")
        .or_else(|| section_content[1..].find("\n### "))
        .map(|i| i + 1)
        .unwrap_or(section_content.len());
    let section = &section_content[..section_end];

    // Match table rows: | condition | warning |
    let re = Regex::new(r"\|\s*([^\|]+?)\s*\|\s*([^\|]+?)\s*\|").unwrap();

    for caps in re.captures_iter(section) {
        let condition = caps.get(1).unwrap().as_str().trim().to_string();
        let message = caps.get(2).unwrap().as_str().trim().to_string();

        // Skip header rows
        if condition == "Condition" || condition.contains("---") {
            continue;
        }

        warnings.push(Warning { condition, message });
    }

    warnings
}

/// Extract script configuration for non-Latin locales.
fn extract_script_config(content: &str, slug_rule: &str) -> Option<ScriptConfig> {
    // Only native_script and romanized rules have script config
    if slug_rule != "native_script" && slug_rule != "romanized" {
        return None;
    }

    // Detect primary script from content
    let primary_script = if content.contains("Arabic") || content.contains("العربية") {
        "arabic"
    } else if content.contains("Hebrew") || content.contains("עברית") {
        "hebrew"
    } else if content.contains("Thai") || content.contains("ไทย") {
        "thai"
    } else if content.contains("Japanese") || content.contains("日本語") {
        "japanese"
    } else if content.contains("Chinese") || content.contains("中文") {
        "chinese"
    } else if content.contains("Korean") || content.contains("한국어") {
        "korean"
    } else if content.contains("Persian") || content.contains("فارسی") {
        "persian"
    } else if content.contains("Greek") || content.contains("Ελληνικά") {
        "greek"
    } else {
        return None;
    };

    // Extract diacritic handling
    let diacritic_handling = if content.contains("tashkeel") || content.contains("[\u{064B}-\u{065F}") {
        Some("remove_tashkeel".to_string())
    } else {
        None
    };

    // Extract numeral handling
    let numeral_handling = if content.contains("Thai numerals") || content.contains("๐-๙") {
        Some("convert_thai".to_string())
    } else if content.contains("Arabic-Indic numerals") || content.contains("٠-٩") {
        Some("preserve_arabic_indic".to_string())
    } else {
        None
    };

    // Extract special chars
    let special_chars = if content.contains("ZWNJ") || content.contains("U+200C") {
        Some(vec!["ZWNJ".to_string()])
    } else {
        None
    };

    Some(ScriptConfig {
        primary_script: primary_script.to_string(),
        diacritic_handling,
        numeral_handling,
        special_chars,
    })
}

/// Get human-readable locale display name.
fn get_locale_display_name(locale: &str) -> String {
    // This is a simplified mapping - in production, use a proper locale library
    let display = match locale {
        "en-US" => "English (United States)",
        "en-GB" => "English (United Kingdom)",
        "fr-FR" => "French (France)",
        "fr-CA" => "French (Canada)",
        "de-DE" => "German (Germany)",
        "de-AT" => "German (Austria)",
        "de-CH" => "German (Switzerland)",
        "es-ES" => "Spanish (Spain)",
        "es-MX" => "Spanish (Mexico)",
        "it-IT" => "Italian (Italy)",
        "pt-BR" => "Portuguese (Brazil)",
        "pt-PT" => "Portuguese (Portugal)",
        "nl-NL" => "Dutch (Netherlands)",
        "pl-PL" => "Polish (Poland)",
        "ru-RU" => "Russian (Russia)",
        "ja-JP" => "Japanese (Japan)",
        "zh-CN" => "Chinese (Simplified)",
        "zh-TW" => "Chinese (Traditional)",
        "ko-KR" => "Korean (South Korea)",
        "ar-SA" => "Arabic (Saudi Arabia)",
        "ar-AE" => "Arabic (UAE)",
        "he-IL" => "Hebrew (Israel)",
        "th-TH" => "Thai (Thailand)",
        "vi-VN" => "Vietnamese (Vietnam)",
        "tr-TR" => "Turkish (Turkey)",
        "hi-IN" => "Hindi (India)",
        "bn-BD" => "Bengali (Bangladesh)",
        "id-ID" => "Indonesian (Indonesia)",
        "ms-MY" => "Malay (Malaysia)",
        "fa-IR" => "Persian (Iran)",
        "el-GR" => "Greek (Greece)",
        "uk-UA" => "Ukrainian (Ukraine)",
        "cs-CZ" => "Czech (Czechia)",
        "sv-SE" => "Swedish (Sweden)",
        "da-DK" => "Danish (Denmark)",
        "fi-FI" => "Finnish (Finland)",
        "no-NO" => "Norwegian (Norway)",
        "hu-HU" => "Hungarian (Hungary)",
        "ro-RO" => "Romanian (Romania)",
        "sk-SK" => "Slovak (Slovakia)",
        "bg-BG" => "Bulgarian (Bulgaria)",
        "hr-HR" => "Croatian (Croatia)",
        "sl-SI" => "Slovenian (Slovenia)",
        "et-EE" => "Estonian (Estonia)",
        "lv-LV" => "Latvian (Latvia)",
        "lt-LT" => "Lithuanian (Lithuania)",
        "af-ZA" => "Afrikaans (South Africa)",
        "sw-KE" => "Swahili (Kenya)",
        _ => {
            // Try to extract language and region
            let parts: Vec<&str> = locale.split('-').collect();
            if parts.len() == 2 {
                return format!("{} ({})", parts[0].to_uppercase(), parts[1]);
            }
            locale
        }
    };

    display.to_string()
}

// ============================================================================
// Aggregation Functions
// ============================================================================

/// Aggregate slugification data into SlugRules.
pub fn aggregate_slug_rules(slugifications: &[Slugification]) -> Vec<SlugRule> {
    let mut rules: HashMap<String, SlugRule> = HashMap::new();

    for s in slugifications {
        let rule = rules
            .entry(s.slug_rule.clone())
            .or_insert_with(|| SlugRule::new(&s.slug_rule));

        rule.locale_count += 1;

        // Add locale example (keep first 10 for reference)
        if rule.locale_examples.len() < 10 {
            rule.locale_examples.push(s.key.clone());
        }
    }

    // Generate llm_context for each rule
    let mut result: Vec<SlugRule> = rules.into_values().collect();
    for rule in &mut result {
        rule.generate_llm_context();
    }

    // Sort by key for deterministic output
    result.sort_by(|a, b| a.key.cmp(&b.key));

    result
}

/// Load all slugification files from ATH directory.
pub fn load_all_slugifications(ath_path: &Path) -> Result<Vec<Slugification>> {
    let slug_dir = ath_path.join("2-rules-slug");

    if !slug_dir.exists() {
        return Err(NovaNetError::Validation(
            format!("ATH slug directory not found: {}", slug_dir.display())
        ));
    }

    let mut slugifications = Vec::new();

    for entry in std::fs::read_dir(&slug_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "md").unwrap_or(false) {
            match parse_slugification_file(&path) {
                Ok(s) => slugifications.push(s),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }

    // Sort by key for deterministic output
    slugifications.sort_by(|a, b| a.key.cmp(&b.key));

    Ok(slugifications)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_FR_FR: &str = r#"---
locale: fr-FR
type: rules-slug
template_version: 2.0
last_updated: 2025-12-08
---

# Slug Rules: fr-FR

**Slug Rule**: latin_preserve

---

## 1. Base Rule: latin_preserve

**Output encoding**: UTF-8

## 2. Stopwords: fr-FR

| Word | Category |
|------|----------|
| le | article |
| la | article |
| de | preposition |
| et | conjunction |

### Locale-Specific Additions

No regional additions for fr-FR.

## 3. Validation Rules

### Warnings

| Condition | Warning |
|-----------|---------|
| Slug < 3 chars | Too short |

## 4. Examples: fr-FR

| Input | Slug | Applied Rules |
|-------|------|---------------|
| Meilleurs Cafés de Paris | meilleurs-cafés-paris | Stopwords removed |
"#;

    #[test]
    fn test_parse_frontmatter() {
        let fm = parse_frontmatter(SAMPLE_FR_FR).unwrap();
        assert_eq!(fm.locale, "fr-FR");
        assert_eq!(fm.template_version, "2.0");
        assert_eq!(fm.last_updated, "2025-12-08");
    }

    #[test]
    fn test_extract_slug_rule() {
        let rule = extract_slug_rule(SAMPLE_FR_FR).unwrap();
        assert_eq!(rule, "latin_preserve");
    }

    #[test]
    fn test_extract_stopwords() {
        let stopwords = extract_stopwords(SAMPLE_FR_FR);
        assert!(stopwords.contains_key("article"));
        assert!(stopwords.contains_key("preposition"));
        assert!(stopwords.contains_key("conjunction"));
        assert_eq!(stopwords["article"], vec!["le", "la"]);
    }

    #[test]
    fn test_extract_examples() {
        let examples = extract_examples(SAMPLE_FR_FR);
        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].input, "Meilleurs Cafés de Paris");
        assert_eq!(examples[0].output, "meilleurs-cafés-paris");
    }

    #[test]
    fn test_extract_warnings() {
        let warnings = extract_warnings(SAMPLE_FR_FR);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].condition, "Slug < 3 chars");
    }

    #[test]
    fn test_parse_slugification() {
        let path = Path::new("test/fr-FR.md");
        let s = parse_slugification(SAMPLE_FR_FR, path).unwrap();

        assert_eq!(s.key, "fr-FR");
        assert_eq!(s.slug_rule, "latin_preserve");
        assert_eq!(s.stopwords_count, 4);
        assert!(s.llm_context.contains("latin_preserve"));
    }

    #[test]
    fn test_slug_rule_new() {
        let rule = SlugRule::new("latin_strip");
        assert_eq!(rule.output_encoding, "ASCII");
        assert_eq!(rule.diacritics, "remove");
        assert!(rule.has_case);

        let rule = SlugRule::new("native_script");
        assert_eq!(rule.output_encoding, "UTF-8");
        assert_eq!(rule.non_latin, "keep");
        assert!(!rule.has_case);
    }

    #[test]
    fn test_aggregate_slug_rules() {
        let slugifications = vec![
            Slugification {
                key: "en-US".to_string(),
                display_name: "English (US) Slugification".to_string(),
                description: String::new(),
                llm_context: String::new(),
                slug_rule: "latin_strip".to_string(),
                stopwords: HashMap::new(),
                stopwords_count: 0,
                regional_additions: vec![],
                script_config: None,
                validation_overrides: None,
                warnings: vec![],
                examples: vec![],
                template_version: "2.0".to_string(),
                source_file: String::new(),
                last_updated: String::new(),
            },
            Slugification {
                key: "fr-FR".to_string(),
                display_name: "French (France) Slugification".to_string(),
                description: String::new(),
                llm_context: String::new(),
                slug_rule: "latin_preserve".to_string(),
                stopwords: HashMap::new(),
                stopwords_count: 0,
                regional_additions: vec![],
                script_config: None,
                validation_overrides: None,
                warnings: vec![],
                examples: vec![],
                template_version: "2.0".to_string(),
                source_file: String::new(),
                last_updated: String::new(),
            },
        ];

        let rules = aggregate_slug_rules(&slugifications);
        assert_eq!(rules.len(), 2);

        let strip = rules.iter().find(|r| r.key == "latin_strip").unwrap();
        assert_eq!(strip.locale_count, 1);
        assert!(strip.locale_examples.contains(&"en-US".to_string()));

        let preserve = rules.iter().find(|r| r.key == "latin_preserve").unwrap();
        assert_eq!(preserve.locale_count, 1);
    }

    #[test]
    fn test_get_locale_display_name() {
        assert_eq!(get_locale_display_name("fr-FR"), "French (France)");
        assert_eq!(get_locale_display_name("ja-JP"), "Japanese (Japan)");
        assert_eq!(get_locale_display_name("ar-SA"), "Arabic (Saudi Arabia)");
        // Unknown locale falls back to formatted version
        assert!(get_locale_display_name("xx-YY").contains("XX"));
    }
}
