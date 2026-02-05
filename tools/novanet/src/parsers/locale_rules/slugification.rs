//! Parser for 2-rules-slug/*.md files.
//!
//! Extracts slugification rules: slug_rule, stopwords, transliteration, validation.

use super::markdown::{extract_list_values, extract_section, parse_tables};
use super::parse_frontmatter;
use serde::Serialize;
use std::collections::HashMap;

/// Parsed slugification data from a locale's rules-slug markdown.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SlugificationData {
    pub locale: String,

    // Core slug rule
    pub slug_rule: String,
    pub preserve_diacritics: bool,
    pub unicode_normalization: String,

    // Length constraints
    pub min_length: i32,
    pub max_length: i32,

    // Character handling
    pub separator: String,
    pub lowercase: bool,
    pub preserve_numbers: bool,
    pub character_filter_regex: Option<String>,

    // Stopwords
    pub stop_words: Vec<String>,
    pub locale_specific_stopwords: Option<Vec<String>>,
    pub stopwords_by_category: Option<HashMap<String, Vec<String>>>,

    // Transliteration (for latin_strip/transliterate rules)
    pub transliteration_map: Option<HashMap<String, String>>,

    // Raw content for LLM
    pub raw_markdown: String,
}

/// Parse a slugification markdown file into structured data.
pub fn parse_slugification(content: &str) -> crate::Result<SlugificationData> {
    let (frontmatter, body) = parse_frontmatter(content)
        .ok_or_else(|| crate::NovaNetError::Validation("Missing frontmatter".to_string()))?;

    let mut data = SlugificationData {
        locale: frontmatter.locale.clone(),
        raw_markdown: content.to_string(),
        // Defaults
        min_length: 3,
        max_length: 80,
        separator: "-".to_string(),
        lowercase: true,
        preserve_numbers: true,
        unicode_normalization: "NFC".to_string(),
        ..Default::default()
    };

    // Parse Base Rule section
    if let Some(section) = extract_section(body, "Base Rule") {
        parse_base_rule(&section, &mut data);
    }

    // Also check for slug rule in the header area
    parse_slug_rule_from_header(body, &mut data);

    // Parse Stopwords section
    if let Some(section) = extract_section(body, "Stopwords") {
        parse_stopwords(&section, &mut data);
    }

    // Parse Validation Rules section
    if let Some(section) = extract_section(body, "Validation Rules") {
        parse_validation_rules(&section, &mut data);
    }

    // Parse Character Handling table if present
    parse_character_handling(body, &mut data);

    // Apply defaults
    apply_defaults(&mut data);

    Ok(data)
}

fn parse_slug_rule_from_header(content: &str, data: &mut SlugificationData) {
    // Look for "Slug Rule: latin_preserve" style
    for line in content.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains("slug rule") || line_lower.contains("slug_rule") {
            if line_lower.contains("latin_preserve") {
                data.slug_rule = "latin_preserve".to_string();
                data.preserve_diacritics = true;
            } else if line_lower.contains("latin_strip") {
                data.slug_rule = "latin_strip".to_string();
                data.preserve_diacritics = false;
            } else if line_lower.contains("native_script") {
                data.slug_rule = "native_script".to_string();
                data.preserve_diacritics = true;
            } else if line_lower.contains("transliterate") {
                data.slug_rule = "transliterate".to_string();
                data.preserve_diacritics = false;
            }
        }
    }
}

fn parse_base_rule(section: &str, data: &mut SlugificationData) {
    let values = extract_list_values(section);
    let tables = parse_tables(section);

    // Parse from list values
    if let Some(v) = values.get("output_encoding") {
        if v.to_uppercase().contains("UTF") {
            data.unicode_normalization = "NFC".to_string();
        }
    }

    // Parse from behavior table
    for table in &tables {
        let kv = table.to_key_value_map();

        if let Some(v) = kv.get("diacritics") {
            data.preserve_diacritics = v.to_lowercase().contains("preserve");
        }
        if let Some(v) = kv.get("case") {
            data.lowercase = v.to_lowercase().contains("lower");
        }
        if let Some(v) = kv.get("spaces") {
            if v.contains("hyphen") || v.contains("-") {
                data.separator = "-".to_string();
            } else if v.contains("underscore") || v.contains("_") {
                data.separator = "_".to_string();
            }
        }
    }

    // Parse character filter regex from implementation code
    if section.contains("[^\\p{L}\\p{N}\\s-]") {
        data.character_filter_regex = Some("[^\\p{L}\\p{N}\\s-]".to_string());
    }

    // Parse transliteration map from Character Handling table
    for table in &tables {
        if table.headers.iter().any(|h| h.to_lowercase() == "input")
            || table
                .headers
                .iter()
                .any(|h| h.to_lowercase().contains("output"))
        {
            let mut transliteration = HashMap::new();
            for row in &table.rows {
                if row.len() >= 2 {
                    // Input might be "À, Á, Â, Ã, Ä, Å" and output "à, á, â, ã, ä, å"
                    let inputs: Vec<&str> = row[0].split(',').map(|s| s.trim()).collect();
                    let outputs: Vec<&str> = row[1].split(',').map(|s| s.trim()).collect();

                    // If preserving, no transliteration needed
                    if row.len() > 2 && row[2].to_lowercase().contains("preserve") {
                        continue;
                    }

                    // Map each input to its output
                    for (inp, out) in inputs.iter().zip(outputs.iter()) {
                        if inp != out && !inp.is_empty() && !out.is_empty() {
                            transliteration.insert(inp.to_string(), out.to_string());
                        }
                    }
                }
            }
            if !transliteration.is_empty() {
                data.transliteration_map = Some(transliteration);
            }
        }
    }
}

fn parse_stopwords(section: &str, data: &mut SlugificationData) {
    let tables = parse_tables(section);

    let mut stopwords = Vec::new();
    let mut by_category: HashMap<String, Vec<String>> = HashMap::new();

    for table in &tables {
        // Check if this is a categorized stopwords table
        let has_category = table.headers.iter().any(|h| {
            let lower = h.to_lowercase();
            lower == "category" || lower == "type"
        });

        for row in &table.rows {
            if row.is_empty() {
                continue;
            }

            let word = row[0].trim().to_string();
            if word.is_empty() || word == "-" {
                continue;
            }

            stopwords.push(word.clone());

            // Categorize if we have category info
            if has_category && row.len() >= 2 {
                let category = row[1].to_lowercase();
                by_category.entry(category).or_default().push(word);
            } else if row.len() >= 2 {
                // Second column might be the category
                let second = row[1].to_lowercase();
                if ["article", "preposition", "conjunction", "pronoun", "verb"]
                    .contains(&second.as_str())
                {
                    by_category.entry(second).or_default().push(word);
                }
            }
        }
    }

    if !stopwords.is_empty() {
        data.stop_words = stopwords;
    }

    if !by_category.is_empty() {
        data.stopwords_by_category = Some(by_category);
    }
}

fn parse_validation_rules(section: &str, data: &mut SlugificationData) {
    let tables = parse_tables(section);

    for table in &tables {
        let kv = table.to_key_value_map();

        if let Some(v) = kv.get("max_length") {
            if let Some(num) = extract_number(v) {
                data.max_length = num;
            }
        }
        if let Some(v) = kv.get("min_length") {
            if let Some(num) = extract_number(v) {
                data.min_length = num;
            }
        }
    }

    // Also look in list format
    let values = extract_list_values(section);
    if let Some(v) = values.get("max_length") {
        if let Some(num) = extract_number(v) {
            data.max_length = num;
        }
    }
    if let Some(v) = values.get("min_length") {
        if let Some(num) = extract_number(v) {
            data.min_length = num;
        }
    }
}

fn parse_character_handling(content: &str, data: &mut SlugificationData) {
    // Look for character handling section that might not be under a ## heading
    if let Some(section) = extract_section(content, "Character Handling") {
        let tables = parse_tables(&section);

        for table in &tables {
            // Build transliteration map from Input | Output columns
            if table.headers.iter().any(|h| h.to_lowercase() == "input") {
                let mut transliteration = HashMap::new();

                for row in &table.rows {
                    if row.len() >= 2 {
                        let inputs: Vec<&str> = row[0].split(',').map(|s| s.trim()).collect();
                        let outputs: Vec<&str> = row[1].split(',').map(|s| s.trim()).collect();

                        // Check if notes column says "preserve"
                        let is_preserve =
                            row.len() > 2 && row[2].to_lowercase().contains("preserve");

                        if is_preserve {
                            // For latin_preserve, uppercase maps to lowercase
                            for (inp, out) in inputs.iter().zip(outputs.iter()) {
                                if inp
                                    .chars()
                                    .next()
                                    .map(|c| c.is_uppercase())
                                    .unwrap_or(false)
                                    && out
                                        .chars()
                                        .next()
                                        .map(|c| c.is_lowercase())
                                        .unwrap_or(false)
                                {
                                    transliteration.insert(inp.to_string(), out.to_string());
                                }
                            }
                        }
                    }
                }

                if !transliteration.is_empty() && data.transliteration_map.is_none() {
                    data.transliteration_map = Some(transliteration);
                }
            }
        }
    }
}

fn apply_defaults(data: &mut SlugificationData) {
    if data.slug_rule.is_empty() {
        data.slug_rule = "latin_preserve".to_string();
    }
    if data.unicode_normalization.is_empty() {
        data.unicode_normalization = "NFC".to_string();
    }
}

/// Extract a number from a string like "≤ 80 characters" or "80"
fn extract_number(s: &str) -> Option<i32> {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_slugification_basic() {
        let content = r#"---
locale: fr-FR
type: rules-slug
template_version: "2.0"
---

# Slug Rules: fr-FR

**Slug Rule**: latin_preserve

## 1. Base Rule: latin_preserve

**Output encoding**: UTF-8

### Behavior

| Aspect | Treatment |
|--------|-----------|
| Diacritics | preserve |
| Case | lowercase |
| Spaces | hyphen |

## 2. Stopwords: fr-FR

### Base Stopwords (French)

| Word | Category |
|------|----------|
| le | article |
| la | article |
| de | preposition |
| et | conjunction |

## 3. Validation Rules

### Required

| Rule | Constraint |
|------|------------|
| Max length | ≤ 80 characters |
| Min length | ≥ 3 characters |
"#;

        let data = parse_slugification(content).unwrap();

        assert_eq!(data.locale, "fr-FR");
        assert_eq!(data.slug_rule, "latin_preserve");
        assert!(data.preserve_diacritics);
        assert_eq!(data.separator, "-");
        assert!(data.lowercase);
        assert_eq!(data.max_length, 80);
        assert_eq!(data.min_length, 3);
        assert!(data.stop_words.contains(&"le".to_string()));
        assert!(data.stop_words.contains(&"de".to_string()));

        // Check categorized stopwords
        let by_cat = data.stopwords_by_category.as_ref().unwrap();
        assert!(by_cat.get("article").unwrap().contains(&"le".to_string()));
        assert!(
            by_cat
                .get("preposition")
                .unwrap()
                .contains(&"de".to_string())
        );
    }

    #[test]
    fn test_extract_number() {
        assert_eq!(extract_number("≤ 80 characters"), Some(80));
        assert_eq!(extract_number("80"), Some(80));
        assert_eq!(extract_number("≥ 3 characters"), Some(3));
    }
}
