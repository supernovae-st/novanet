//! Parser for 2-rules-adaptation/*.md files.
//!
//! Extracts adaptation rules: FACTS vs ILLUSTRATIONS, priority hierarchy, formality, typography.

use super::markdown::{extract_list_values, extract_section, parse_bullet_list, parse_tables};
use super::parse_frontmatter;
use serde::Serialize;
use std::collections::HashMap;

/// Parsed adaptation data from a locale's rules-adaptation markdown.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AdaptationData {
    pub locale: String,

    // Core framework
    pub facts_categories: HashMap<String, String>,
    pub illustrations_categories: HashMap<String, String>,
    pub priority_hierarchy: HashMap<String, String>,

    // Formality
    pub formality_default: String,
    pub formality_rules: Option<HashMap<String, String>>,

    // Typography
    pub punctuation_spacing: Option<HashMap<String, String>>,
    pub typography_conventions: Option<HashMap<String, String>>,

    // Technical terms
    pub technical_terms_approach: String,
    pub technical_terms_preferred: Option<HashMap<String, String>>,
    pub technical_terms_accepted: Option<Vec<String>>,

    // Measurement
    pub measurement_system: String,
    pub measurement_exceptions: Option<HashMap<String, String>>,

    // Calendar
    pub hemisphere: String,
    pub work_week: String,
    pub major_shopping_events: Option<Vec<String>>,
    pub cultural_calendar: Option<Vec<String>>,

    // Length preferences
    pub length_multiplier: Option<f32>,
    pub headline_max_chars: Option<i32>,
    pub cta_max_chars: Option<i32>,

    // Structure
    pub paragraph_style: Option<String>,
    pub heading_capitalization: String,

    // Legal
    pub legal_compliance: Option<HashMap<String, String>>,

    // Common errors
    pub common_errors: Option<Vec<String>>,

    // Raw content for LLM
    pub raw_markdown: String,
}

/// Parse an adaptation markdown file into structured data.
pub fn parse_adaptation(content: &str) -> crate::Result<AdaptationData> {
    let (frontmatter, body) = parse_frontmatter(content)
        .ok_or_else(|| crate::NovaNetError::Validation("Missing frontmatter".to_string()))?;

    let mut data = AdaptationData {
        locale: frontmatter.locale.clone(),
        raw_markdown: content.to_string(),
        // Defaults
        formality_default: "formal".to_string(),
        technical_terms_approach: "mixed".to_string(),
        measurement_system: "metric".to_string(),
        hemisphere: "northern".to_string(),
        work_week: "MON-FRI".to_string(),
        heading_capitalization: "sentence".to_string(),
        ..Default::default()
    };

    // Parse Core Framework section
    if let Some(section) = extract_section(body, "Core Framework") {
        parse_core_framework(&section, &mut data);
    }

    // Parse FACTS section
    if let Some(section) = extract_section(body, "FACTS") {
        parse_facts_section(&section, &mut data);
    }

    // Parse ILLUSTRATIONS section
    if let Some(section) = extract_section(body, "ILLUSTRATIONS") {
        parse_illustrations_section(&section, &mut data);
    }

    // Parse Priority Hierarchy section
    if let Some(section) = extract_section(body, "Priority Hierarchy") {
        parse_priority_hierarchy(&section, &mut data);
    }

    // Parse Formality section
    if let Some(section) = extract_section(body, "Formality") {
        parse_formality(&section, &mut data);
    }

    // Parse Locale-Specific Parameters section
    if let Some(section) = extract_section(body, "Locale-Specific Parameters") {
        parse_locale_specific(&section, &mut data);
    }

    // Parse Measurement System section
    if let Some(section) = extract_section(body, "Measurement System") {
        parse_measurement(&section, &mut data);
    }

    // Parse Technical Terms section
    if let Some(section) = extract_section(body, "Technical Terms") {
        parse_technical_terms(&section, &mut data);
    }

    // Parse Calendar section
    if let Some(section) = extract_section(body, "Calendar") {
        parse_calendar(&section, &mut data);
    }

    // Parse Common Errors section
    if let Some(section) = extract_section(body, "Common Errors") {
        parse_common_errors(&section, &mut data);
    }

    // Parse typography from various sections
    parse_typography(body, &mut data);

    // Parse legal compliance
    parse_legal_compliance(body, &mut data);

    Ok(data)
}

fn parse_core_framework(_section: &str, _data: &mut AdaptationData) {
    // This section typically introduces FACTS vs ILLUSTRATIONS
    // The actual categories are in sub-sections
    // No structured data to extract here - it's just introductory text
}

fn parse_facts_section(section: &str, data: &mut AdaptationData) {
    let tables = parse_tables(section);

    for table in &tables {
        for row in &table.rows {
            if row.len() >= 2 {
                let category = row[0]
                    .trim()
                    .trim_start_matches("**")
                    .trim_end_matches("**")
                    .to_lowercase()
                    .replace(' ', "_");
                let description = row[1].clone();
                // Also include treatment if available
                let full_desc = if row.len() >= 3 {
                    format!("{} - {}", row[1], row[2])
                } else {
                    description
                };
                data.facts_categories.insert(category, full_desc);
            }
        }
    }
}

fn parse_illustrations_section(section: &str, data: &mut AdaptationData) {
    let tables = parse_tables(section);

    for table in &tables {
        for row in &table.rows {
            if row.len() >= 2 {
                let category = row[0]
                    .trim()
                    .trim_start_matches("**")
                    .trim_end_matches("**")
                    .to_lowercase()
                    .replace(' ', "_");
                let description = row[1].clone();
                let full_desc = if row.len() >= 3 {
                    format!("{} - {}", row[1], row[2])
                } else {
                    description
                };
                data.illustrations_categories.insert(category, full_desc);
            }
        }
    }
}

fn parse_priority_hierarchy(section: &str, data: &mut AdaptationData) {
    // Look for level markers
    for line in section.lines() {
        let lower = line.to_lowercase();

        if lower.contains("level 1") || lower.contains("legal") && lower.contains("safety") {
            data.priority_hierarchy
                .insert("level_1".to_string(), extract_level_description(line));
        } else if lower.contains("level 2") || lower.contains("brand") {
            data.priority_hierarchy
                .insert("level_2".to_string(), extract_level_description(line));
        } else if lower.contains("level 3") || lower.contains("cultural") {
            data.priority_hierarchy
                .insert("level_3".to_string(), extract_level_description(line));
        } else if lower.contains("level 4") || lower.contains("style") && lower.contains("tone") {
            data.priority_hierarchy
                .insert("level_4".to_string(), extract_level_description(line));
        }
    }
}

fn extract_level_description(line: &str) -> String {
    // Extract description after ":" or "->"
    if let Some(idx) = line.find(':') {
        line[idx + 1..].trim().to_string()
    } else if let Some(idx) = line.find("->") {
        line[idx + 2..].trim().to_string()
    } else {
        line.trim().to_string()
    }
}

fn parse_formality(section: &str, data: &mut AdaptationData) {
    let values = extract_list_values(section);
    let tables = parse_tables(section);

    // Check for default formality
    if let Some(v) = values.get("default_formality") {
        data.formality_default = normalize_formality(v);
    }

    // Parse formality rules from table
    let mut rules = HashMap::new();
    for table in &tables {
        for row in &table.rows {
            if row.len() >= 2 {
                let context = row[0].to_lowercase().replace(' ', "_");
                let formality = normalize_formality(&row[1]);
                rules.insert(context, formality);
            }
        }
    }

    // Also extract from content patterns
    let lower = section.to_lowercase();
    if (lower.contains("vous") || lower.contains("vouvoiement"))
        && (lower.contains("default") || lower.contains("b2b"))
    {
        data.formality_default = "formal".to_string();
    }

    if !rules.is_empty() {
        data.formality_rules = Some(rules);
    }
}

fn normalize_formality(s: &str) -> String {
    let lower = s.to_lowercase();
    // Check for formal indicators (French: vouvoiement, vous)
    if lower.contains("formal") || lower.contains("vous") || lower.contains("vouvoiement") {
        "formal".to_string()
    } else if lower.contains("informal")
        || lower.contains("casual")
        || lower.contains("tu")
        || lower.contains("tutoiement")
    {
        "informal".to_string()
    } else {
        "neutral".to_string()
    }
}

fn parse_locale_specific(section: &str, data: &mut AdaptationData) {
    // This is a large section, delegate to sub-parsers
    parse_formality(section, data);
    parse_measurement(section, data);
    parse_technical_terms(section, data);
    parse_calendar(section, data);
}

fn parse_measurement(section: &str, data: &mut AdaptationData) {
    let lower = section.to_lowercase();

    if lower.contains("metric") {
        data.measurement_system = "metric".to_string();
    } else if lower.contains("imperial") {
        data.measurement_system = "imperial".to_string();
    } else if lower.contains("mixed") {
        data.measurement_system = "mixed".to_string();
    }

    // Parse exceptions
    let mut exceptions = HashMap::new();
    let tables = parse_tables(section);
    for table in &tables {
        for row in &table.rows {
            if row.len() >= 2 {
                let category = row[0].to_lowercase();
                // Look for exceptions like "screens: inches"
                if category.contains("screen")
                    || category.contains("aviation")
                    || category.contains("maritime")
                {
                    exceptions.insert(category, row[1].to_lowercase());
                }
            }
        }
    }

    if !exceptions.is_empty() {
        data.measurement_exceptions = Some(exceptions);
    }
}

fn parse_technical_terms(section: &str, data: &mut AdaptationData) {
    let lower = section.to_lowercase();

    // Determine approach
    if lower.contains("mixed") {
        data.technical_terms_approach = "mixed".to_string();
    } else if lower.contains("local_only") || lower.contains("french only") {
        data.technical_terms_approach = "local_only".to_string();
    } else if lower.contains("english_accepted") {
        data.technical_terms_approach = "english_accepted".to_string();
    }

    // Parse preferred terms
    let mut preferred = HashMap::new();
    let mut accepted = Vec::new();

    // Look for "French preferred" section
    if let Some(pref_section) = extract_subsection(section, "French preferred") {
        let values = extract_list_values(&pref_section);
        for (k, v) in values {
            preferred.insert(k, v);
        }
    }

    // Look for "English accepted" section
    if let Some(acc_section) = extract_subsection(section, "English accepted") {
        accepted = parse_bullet_list(&acc_section);
        if accepted.is_empty() {
            // Try parsing from inline list
            for line in acc_section.lines() {
                if line.contains(',') {
                    accepted = line
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    break;
                }
            }
        }
    }

    if !preferred.is_empty() {
        data.technical_terms_preferred = Some(preferred);
    }
    if !accepted.is_empty() {
        data.technical_terms_accepted = Some(accepted);
    }
}

fn parse_calendar(section: &str, data: &mut AdaptationData) {
    let values = extract_list_values(section);
    let lower = section.to_lowercase();

    // Hemisphere
    if lower.contains("northern") {
        data.hemisphere = "northern".to_string();
    } else if lower.contains("southern") {
        data.hemisphere = "southern".to_string();
    }

    // Work week
    if let Some(v) = values.get("work_week") {
        data.work_week = v.to_uppercase();
    } else if lower.contains("mon-fri") || lower.contains("monday") && lower.contains("friday") {
        data.work_week = "MON-FRI".to_string();
    } else if lower.contains("sun-thu") {
        data.work_week = "SUN-THU".to_string();
    }

    // Shopping events and cultural calendar
    let mut shopping_events = Vec::new();
    let mut cultural_events = Vec::new();

    let tables = parse_tables(section);
    for table in &tables {
        for row in &table.rows {
            if !row.is_empty() {
                let item = row[0].clone();
                let lower_item = item.to_lowercase();

                // Categorize based on keywords
                if lower_item.contains("sale")
                    || lower_item.contains("solde")
                    || lower_item.contains("black friday")
                    || lower_item.contains("shopping")
                    || lower_item.contains("rentr")
                {
                    shopping_events.push(item);
                } else if lower_item.contains("juillet")
                    || lower_item.contains("bastille")
                    || lower_item.contains("christmas")
                    || lower_item.contains("noel")
                    || lower_item.contains("holiday")
                    || lower_item.contains("saint")
                {
                    cultural_events.push(item);
                }
            }
        }
    }

    // Also parse from bullet lists
    let bullets = parse_bullet_list(section);
    for item in bullets {
        let lower_item = item.to_lowercase();
        if lower_item.contains("sale") || lower_item.contains("solde") {
            shopping_events.push(item);
        } else if lower_item.contains("juillet") || lower_item.contains("christmas") {
            cultural_events.push(item);
        }
    }

    if !shopping_events.is_empty() {
        data.major_shopping_events = Some(shopping_events);
    }
    if !cultural_events.is_empty() {
        data.cultural_calendar = Some(cultural_events);
    }
}

fn parse_common_errors(section: &str, data: &mut AdaptationData) {
    let tables = parse_tables(section);
    let mut errors = Vec::new();

    for table in &tables {
        for row in &table.rows {
            if !row.is_empty() {
                errors.push(row[0].clone());
            }
        }
    }

    // Also parse from bullet list
    let bullets = parse_bullet_list(section);
    errors.extend(bullets);

    if !errors.is_empty() {
        data.common_errors = Some(errors);
    }
}

fn parse_typography(content: &str, data: &mut AdaptationData) {
    // Look for punctuation spacing info
    let mut spacing = HashMap::new();
    let mut typography = HashMap::new();

    let lower = content.to_lowercase();

    // French punctuation spacing rules
    if lower.contains("space before") {
        if lower.contains("colon") || lower.contains(":") {
            spacing.insert("colon".to_string(), "space_before".to_string());
        }
        if lower.contains("semicolon") || lower.contains(";") {
            spacing.insert("semicolon".to_string(), "space_before".to_string());
        }
        if lower.contains("exclamation") || lower.contains("!") {
            spacing.insert("exclamation".to_string(), "space_before".to_string());
        }
        if lower.contains("question") || lower.contains("?") {
            spacing.insert("question".to_string(), "space_before".to_string());
        }
    }

    // Comma and period never have space before
    spacing.insert("comma".to_string(), "no_space".to_string());
    spacing.insert("period".to_string(), "no_space".to_string());

    // French typography
    if lower.contains("guillemets") || content.contains("«") || content.contains("»") {
        typography.insert("opening_quote".to_string(), "«".to_string());
        typography.insert("closing_quote".to_string(), "»".to_string());
        typography.insert("quote_spacing".to_string(), "true".to_string());
    }

    if content.contains("…") || lower.contains("ellipsis") {
        typography.insert("ellipsis".to_string(), "…".to_string());
    }

    if !spacing.is_empty() {
        data.punctuation_spacing = Some(spacing);
    }
    if !typography.is_empty() {
        data.typography_conventions = Some(typography);
    }
}

fn parse_legal_compliance(content: &str, data: &mut AdaptationData) {
    let mut legal = HashMap::new();
    let lower = content.to_lowercase();

    if lower.contains("code de la consommation") || lower.contains("consumer") {
        legal.insert(
            "consumer_protection".to_string(),
            "Code de la consommation".to_string(),
        );
    }
    if lower.contains("rgpd") || lower.contains("cnil") || lower.contains("gdpr") {
        legal.insert("data_privacy".to_string(), "RGPD / CNIL".to_string());
    }
    if lower.contains("loi toubon") || lower.contains("language law") {
        legal.insert("language_law".to_string(), "Loi Toubon".to_string());
    }

    if !legal.is_empty() {
        data.legal_compliance = Some(legal);
    }
}

/// Extract a subsection (### heading) from content
fn extract_subsection(content: &str, subsection_name: &str) -> Option<String> {
    let marker_lower = subsection_name.to_lowercase();

    let mut in_section = false;
    let mut section_content = String::new();

    for line in content.lines() {
        let trimmed_lower = line.trim().to_lowercase();

        // Match "### French preferred" or "**French preferred**:"
        if trimmed_lower.contains(&marker_lower)
            && (line.trim().starts_with("###") || line.contains("**"))
        {
            in_section = true;
            continue;
        }

        if in_section {
            if line.trim().starts_with("###")
                || line.trim().starts_with("##")
                || (line.contains("**") && line.contains(":"))
            {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_adaptation_basic() {
        let content = r#"---
locale: fr-FR
type: rules-adaptation
template_version: "3.0"
---

# Rules Adaptation: fr-FR

## 1. Core Framework: FACTS vs ILLUSTRATIONS

### 1.1 FACTS (Never Adapt - Format Only)

| Category | Examples | Treatment |
|----------|----------|-----------|
| **Financial** | Prices, fees | Translate literally |
| **Legal** | CGV, RGPD | NEVER adapt |

### 1.2 ILLUSTRATIONS (Always Generate Native)

| Category | Examples | Treatment |
|----------|----------|-----------|
| **Metaphors** | Idioms | Generate native |
| **Cultural refs** | Holidays | Replace with local |

## 2. Priority Hierarchy

- Level 1: Legal/Safety → Never adapt
- Level 2: Brand → Keep consistent
- Level 3: Cultural → Generate native
- Level 4: Style → Apply local style

## 5. Locale-Specific Parameters for fr-FR

### 5.1 Formality Baseline

**Default formality**: FORMAL (vouvoiement)

| Context | Formality |
|---------|-----------|
| B2B | FORMAL (vous) |
| Youth brand | CASUAL (tu) |

### 5.2 Measurement System

**System**: METRIC

## 6. Common Errors for fr-FR

| Error | Why Wrong |
|-------|-----------|
| Using "tu" with unknown customer | Disrespectful |
| Literal translation of idioms | Sounds absurd |
"#;

        let data = parse_adaptation(content).unwrap();

        assert_eq!(data.locale, "fr-FR");
        assert_eq!(data.formality_default, "formal");
        assert_eq!(data.measurement_system, "metric");

        // Check facts categories
        assert!(data.facts_categories.contains_key("financial"));
        assert!(data.facts_categories.contains_key("legal"));

        // Check illustrations categories
        assert!(data.illustrations_categories.contains_key("metaphors"));

        // Check common errors
        let errors = data.common_errors.as_ref().unwrap();
        assert!(errors.iter().any(|e| e.contains("tu")));
    }

    #[test]
    fn test_normalize_formality() {
        assert_eq!(normalize_formality("FORMAL (vous)"), "formal");
        assert_eq!(normalize_formality("CASUAL (tu)"), "informal");
        assert_eq!(normalize_formality("vouvoiement"), "formal");
    }
}
