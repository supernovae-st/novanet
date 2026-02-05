//! Parser for 2-rules-formatting/*.md files.
//!
//! Extracts formatting rules: numbers, dates, time, currency, phone, address, validation.

use super::markdown::{extract_list_values, extract_section, parse_tables};
use super::parse_frontmatter;
use serde::Serialize;
use std::collections::HashMap;

/// Parsed formatting data from a locale's rules-formatting markdown.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FormattingData {
    pub locale: String,

    // Number formatting
    pub decimal_separator: String,
    pub thousands_separator: String,
    pub grouping_size: Option<i32>,
    pub negative_format: Option<String>,
    pub percentage_space: Option<bool>,

    // Date formatting
    pub date_format: String,
    pub date_formats: Option<HashMap<String, String>>,
    pub date_separator: Option<String>,
    pub month_names: Option<Vec<String>>,
    pub month_names_abbr: Option<Vec<String>>,
    pub day_names: Option<Vec<String>>,
    pub day_names_abbr: Option<Vec<String>>,

    // Time formatting
    pub time_system: String,
    pub time_format: Option<String>,
    pub time_separator: Option<String>,

    // Currency formatting
    pub currency_code: String,
    pub currency_symbol: String,
    pub currency_position: String,
    pub currency_space: Option<bool>,
    pub currency_format: Option<String>,

    // Phone formatting
    pub country_code: Option<String>,
    pub phone_format: Option<String>,
    pub phone_international: Option<String>,

    // Address formatting
    pub address_format: Option<serde_json::Value>,
    pub street_types: Option<Vec<String>>,
    pub postal_code_format: Option<String>,
    pub measurement_system: String,
    pub temperature_unit: Option<String>,

    // National identifiers
    pub national_id_patterns: Option<HashMap<String, String>>,

    // Validation patterns
    pub validation_patterns: Option<HashMap<String, String>>,

    // Raw content for LLM
    pub raw_markdown: String,
}

/// Parse a formatting markdown file into structured data.
pub fn parse_formatting(content: &str) -> crate::Result<FormattingData> {
    let (frontmatter, body) = parse_frontmatter(content)
        .ok_or_else(|| crate::NovaNetError::Validation("Missing frontmatter".to_string()))?;

    let mut data = FormattingData {
        locale: frontmatter.locale.clone(),
        raw_markdown: content.to_string(),
        ..Default::default()
    };

    // Parse Number Formatting section
    if let Some(section) = extract_section(body, "Number Formatting") {
        parse_number_formatting(&section, &mut data);
    }

    // Parse Date Formatting section
    if let Some(section) = extract_section(body, "Date Formatting") {
        parse_date_formatting(&section, &mut data);
    }

    // Parse Time Formatting section
    if let Some(section) = extract_section(body, "Time Formatting") {
        parse_time_formatting(&section, &mut data);
    }

    // Parse Currency Formatting section
    if let Some(section) = extract_section(body, "Currency Formatting") {
        parse_currency_formatting(&section, &mut data);
    }

    // Parse Phone Number Formatting section
    if let Some(section) = extract_section(body, "Phone Number Formatting") {
        parse_phone_formatting(&section, &mut data);
    }

    // Parse Address Formatting section
    if let Some(section) = extract_section(body, "Address Formatting") {
        parse_address_formatting(&section, &mut data);
    }

    // Parse Measurement System section
    if let Some(section) = extract_section(body, "Measurement System") {
        parse_measurement_system(&section, &mut data);
    }

    // Parse Validation Patterns section
    if let Some(section) = extract_section(body, "Validation Patterns") {
        parse_validation_patterns(&section, &mut data);
    }

    // Apply defaults for required fields
    apply_defaults(&mut data);

    Ok(data)
}

fn parse_number_formatting(section: &str, data: &mut FormattingData) {
    let values = extract_list_values(section);

    if let Some(v) = values.get("decimal_separator") {
        data.decimal_separator = clean_value(v);
    }
    if let Some(v) = values.get("thousands_separator") {
        data.thousands_separator = clean_value(v);
    }
    if let Some(v) = values.get("grouping_pattern") {
        data.grouping_size = v.parse().ok();
    }
    if let Some(v) = values.get("negative_sign") {
        data.negative_format = Some(format!("{}{{n}}", v));
    }
    if let Some(v) = values.get("positive_sign") {
        // Store if present but typically not used
        let _ = v;
    }

    // Check for percentage formatting (space before %)
    if section.to_lowercase().contains("space") && section.contains('%') {
        data.percentage_space = Some(true);
    }
}

fn parse_date_formatting(section: &str, data: &mut FormattingData) {
    let values = extract_list_values(section);

    if let Some(v) = values.get("pattern") {
        data.date_format = clean_value(v);
    }
    if let Some(v) = values.get("date_separator") {
        data.date_separator = Some(clean_value(v));
    }

    // Build date_formats from various patterns
    let mut formats = HashMap::new();
    if let Some(v) = values.get("short_pattern") {
        formats.insert("short".to_string(), clean_value(v));
    }
    if let Some(v) = values.get("pattern") {
        formats.insert("medium".to_string(), clean_value(v));
    }
    if let Some(v) = values.get("long_pattern") {
        formats.insert("long".to_string(), clean_value(v));
    }
    if let Some(v) = values.get("full_pattern") {
        formats.insert("full".to_string(), clean_value(v));
    }
    if !formats.is_empty() {
        data.date_formats = Some(formats);
    }

    // Parse month names from tables or lists
    parse_month_day_names(section, data);
}

fn parse_month_day_names(section: &str, data: &mut FormattingData) {
    // Look for month names in the content
    let month_markers = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    let mut full_months = Vec::new();
    let mut abbr_months = Vec::new();
    let mut full_days = Vec::new();
    let mut abbr_days = Vec::new();

    for line in section.lines() {
        let line_lower = line.to_lowercase();

        // Match "January: janvier" or "- January: janvier"
        for (idx, marker) in month_markers.iter().enumerate() {
            if line_lower.contains(marker) && line.contains(':') {
                if let Some(value) = line.split(':').nth(1) {
                    let clean = value.trim().to_string();
                    // Ensure we're inserting at the right position
                    while full_months.len() < idx {
                        full_months.push(String::new());
                    }
                    if full_months.len() == idx {
                        full_months.push(clean);
                    }
                }
            }
        }

        // Match "Monday: lundi"
        let day_markers = [
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ];
        for (idx, marker) in day_markers.iter().enumerate() {
            if line_lower.contains(marker) && line.contains(':') {
                if let Some(value) = line.split(':').nth(1) {
                    let clean = value.trim().to_string();
                    while full_days.len() < idx {
                        full_days.push(String::new());
                    }
                    if full_days.len() == idx {
                        full_days.push(clean);
                    }
                }
            }
        }
    }

    // Parse abbreviated names from list like "janv., févr., mars, ..."
    if let Some(abbr_section) = extract_subsection(section, "Abbreviated Month Names") {
        abbr_months = parse_comma_list(&abbr_section);
    }
    if let Some(abbr_section) = extract_subsection(section, "Abbreviated Day Names") {
        abbr_days = parse_comma_list(&abbr_section);
    }

    // Alternative: parse from inline list after colon
    for line in section.lines() {
        if line.to_lowercase().contains("abbreviated month") || line.contains("janv.") {
            let parts: Vec<&str> = line.split(&['-', ':'][..]).collect();
            if parts.len() > 1 {
                let list = parse_comma_list(parts.last().unwrap_or(&""));
                if list.len() == 12 {
                    abbr_months = list;
                }
            }
        }
        if line.to_lowercase().contains("abbreviated day") || line.contains("lun.") {
            let parts: Vec<&str> = line.split(&['-', ':'][..]).collect();
            if parts.len() > 1 {
                let list = parse_comma_list(parts.last().unwrap_or(&""));
                if list.len() == 7 {
                    abbr_days = list;
                }
            }
        }
    }

    if full_months.len() == 12 {
        data.month_names = Some(full_months);
    }
    if abbr_months.len() == 12 {
        data.month_names_abbr = Some(abbr_months);
    }
    if full_days.len() == 7 {
        data.day_names = Some(full_days);
    }
    if abbr_days.len() == 7 {
        data.day_names_abbr = Some(abbr_days);
    }
}

fn parse_time_formatting(section: &str, data: &mut FormattingData) {
    let values = extract_list_values(section);

    if let Some(v) = values.get("system") {
        data.time_system = if v.contains("24") { "24h" } else { "12h" }.to_string();
    }
    if let Some(v) = values.get("pattern") {
        data.time_format = Some(clean_value(v));
    }
    if let Some(v) = values.get("time_separator") {
        data.time_separator = Some(clean_value(v));
    }

    // Default time system from context
    if data.time_system.is_empty() {
        if section.to_lowercase().contains("24-hour") || section.contains("24h") {
            data.time_system = "24h".to_string();
        } else if section.to_lowercase().contains("12-hour") {
            data.time_system = "12h".to_string();
        }
    }
}

fn parse_currency_formatting(section: &str, data: &mut FormattingData) {
    let values = extract_list_values(section);

    if let Some(v) = values.get("code") {
        data.currency_code = clean_value(v);
    }
    if let Some(v) = values.get("symbol") {
        data.currency_symbol = clean_value(v);
    }
    if let Some(v) = values.get("symbol_position") {
        let lower = v.to_lowercase();
        data.currency_position = if lower.contains("after") {
            "after".to_string()
        } else {
            "before".to_string()
        };
        // Check for space
        if lower.contains("space") || lower.contains("non-breaking") {
            data.currency_space = Some(true);
        }
    }
}

fn parse_phone_formatting(section: &str, data: &mut FormattingData) {
    let values = extract_list_values(section);

    if let Some(v) = values.get("country_code") {
        data.country_code = Some(clean_value(v));
    }
    if let Some(v) = values.get("national_pattern") {
        data.phone_format = Some(clean_value(v));
    }
    if let Some(v) = values.get("international_pattern") {
        data.phone_international = Some(clean_value(v));
    }
}

fn parse_address_formatting(section: &str, data: &mut FormattingData) {
    let values = extract_list_values(section);

    // Extract postal code format
    if let Some(v) = values.get("postal_code_pattern") {
        data.postal_code_format = Some(clean_value(v));
    }

    // Extract street types from the section
    if section.to_lowercase().contains("street types") || section.contains("voies") {
        for line in section.lines() {
            if line.contains("rue") || line.contains("avenue") || line.contains("boulevard") {
                let parts: Vec<&str> = line
                    .split(&['-', ':'][..])
                    .next_back()
                    .unwrap_or("")
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !parts.is_empty() {
                    data.street_types = Some(parts.iter().map(|s| s.to_string()).collect());
                    break;
                }
            }
        }
    }
}

fn parse_measurement_system(section: &str, data: &mut FormattingData) {
    let tables = parse_tables(section);

    // Look for measurement system indicator
    let lower = section.to_lowercase();
    if lower.contains("metric") {
        data.measurement_system = "metric".to_string();
    } else if lower.contains("imperial") {
        data.measurement_system = "imperial".to_string();
    }

    // Temperature unit
    if lower.contains("celsius") {
        data.temperature_unit = Some("celsius".to_string());
    } else if lower.contains("fahrenheit") {
        data.temperature_unit = Some("fahrenheit".to_string());
    }

    // Parse units table if present
    for table in tables {
        let kv = table.to_key_value_map();
        if let Some(temp) = kv.get("temperature") {
            if temp.to_lowercase().contains("celsius") {
                data.temperature_unit = Some("celsius".to_string());
            }
        }
    }
}

fn parse_validation_patterns(section: &str, data: &mut FormattingData) {
    let tables = parse_tables(section);

    let mut patterns = HashMap::new();

    for table in tables {
        for row in &table.rows {
            if row.len() >= 2 {
                let key = row[0].to_lowercase().replace(' ', "_");
                let pattern = row[1].clone();
                // Clean up pattern (remove backticks)
                let clean = pattern.trim_matches('`').to_string();
                patterns.insert(key, clean);
            }
        }
    }

    if !patterns.is_empty() {
        data.validation_patterns = Some(patterns);
    }
}

fn apply_defaults(data: &mut FormattingData) {
    // Apply reasonable defaults for required fields
    if data.decimal_separator.is_empty() {
        data.decimal_separator = ".".to_string();
    }
    if data.thousands_separator.is_empty() {
        data.thousands_separator = ",".to_string();
    }
    if data.date_format.is_empty() {
        data.date_format = "YYYY-MM-DD".to_string();
    }
    if data.time_system.is_empty() {
        data.time_system = "24h".to_string();
    }
    if data.currency_code.is_empty() {
        data.currency_code = "USD".to_string();
    }
    if data.currency_symbol.is_empty() {
        data.currency_symbol = "$".to_string();
    }
    if data.currency_position.is_empty() {
        data.currency_position = "before".to_string();
    }
    if data.measurement_system.is_empty() {
        data.measurement_system = "metric".to_string();
    }
}

/// Clean a value extracted from markdown (remove backticks, parentheses content, etc.)
fn clean_value(s: &str) -> String {
    let result = s.trim();

    // If the value is wrapped in backticks like `,`, extract the content
    // Pattern: starts with ` and contains another ` somewhere
    if let Some(stripped) = result.strip_prefix('`') {
        // Find the closing backtick
        if let Some(close_idx) = stripped.find('`') {
            // Extract content between backticks
            return stripped[..close_idx].to_string();
        }
    }

    // Otherwise, remove any backticks and parentheses content
    let mut cleaned = result.replace('`', "");

    // Remove content in parentheses like "(comma)" or "(e.g., 15/01/2026)"
    if let Some(paren_start) = cleaned.find('(') {
        cleaned = cleaned[..paren_start].trim().to_string();
    }

    cleaned.trim().to_string()
}

/// Extract a subsection (### heading) from content
fn extract_subsection(content: &str, subsection_name: &str) -> Option<String> {
    let marker = format!("### {}", subsection_name);
    let marker_lower = marker.to_lowercase();

    let mut in_section = false;
    let mut section_content = String::new();

    for line in content.lines() {
        let trimmed_lower = line.trim().to_lowercase();

        if trimmed_lower.starts_with(&marker_lower) {
            in_section = true;
            continue;
        }

        if in_section {
            if line.trim().starts_with("###") || line.trim().starts_with("##") {
                break;
            }
            section_content.push_str(line);
            section_content.push('\n');
        }
    }

    if section_content.is_empty() {
        None
    } else {
        Some(section_content.trim().to_string())
    }
}

/// Parse a comma-separated list into Vec
fn parse_comma_list(s: &str) -> Vec<String> {
    s.split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_formatting_basic() {
        let content = r#"---
locale: fr-FR
type: rules-formatting
template_version: "2.0"
---

# Rules Formatting: fr-FR

## 1. Number Formatting

### Separators & Signs

- **decimal_separator**: `,` (comma)
- **thousands_separator**: ` ` (fine space)
- **grouping_pattern**: 3

## 2. Date Formatting

### Date Patterns

- **pattern**: DD/MM/YYYY (e.g., 15/01/2026)
- **short_pattern**: DD/MM/YY
- **long_pattern**: D MMMM YYYY
- **date_separator**: `/` (slash)

## 3. Time Formatting

### Time System

- **system**: 24-hour (exclusively in France)
- **pattern**: HH:mm

## 4. Currency Formatting

### Currency Details

- **code**: EUR (Euro)
- **symbol**: `€`
- **symbol_position**: AFTER amount WITH non-breaking space
"#;

        let data = parse_formatting(content).unwrap();

        assert_eq!(data.locale, "fr-FR");
        assert_eq!(data.decimal_separator, ",");
        assert_eq!(data.date_format, "DD/MM/YYYY");
        assert_eq!(data.time_system, "24h");
        assert_eq!(data.currency_code, "EUR");
        assert_eq!(data.currency_symbol, "€");
        assert_eq!(data.currency_position, "after");
    }

    #[test]
    fn test_clean_value() {
        assert_eq!(clean_value("`€`"), "€");
        assert_eq!(clean_value("DD/MM/YYYY (e.g., 15/01/2026)"), "DD/MM/YYYY");
        assert_eq!(clean_value("  test  "), "test");
    }

    #[test]
    fn test_parse_comma_list() {
        let items = parse_comma_list("janv., févr., mars, avr.");
        assert_eq!(items.len(), 4);
        assert_eq!(items[0], "janv.");
        assert_eq!(items[2], "mars");
    }
}
