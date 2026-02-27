//! Parser for ATH 4-culture-norms data.
//!
//! Transforms ATH culture norms markdown files into Rust structs for Cypher generation.
//! Extracts: values, taboos, communication style, business hours, work week, hierarchy.

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

/// Directness level extraction: **Scale**: LEVEL
static RE_DIRECTNESS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Scale\*\*:\s*(\w+)").expect("valid directness regex"));

/// Context type extraction: **Type**: TYPE
static RE_CONTEXT_TYPE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Type\*\*:\s*(\w+)").expect("valid context type regex"));

/// Hierarchy level extraction: **Level**: LEVEL
static RE_HIERARCHY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*Level\*\*:\s*(\w+)").expect("valid hierarchy regex"));

/// Table row with 4 columns: | text | word | text | text |
static RE_TABLE_4COL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^|]+)\s*\|\s*(\w+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|")
        .expect("valid 4-col table regex")
});

/// Table row with 3 columns: | text | word | text |
static RE_TABLE_3COL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^|]+)\s*\|\s*(\w+)\s*\|\s*([^|]+)\s*\|").expect("valid 3-col table regex")
});

/// Work days extraction
static RE_WORK_DAYS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Work days\s*\|\s*(\w+)").expect("valid work days regex"));

/// Standard hours extraction
static RE_STANDARD_HOURS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*Standard[^|]*\|\s*(\d{1,2}:\d{2})\s*-\s*(\d{1,2}:\d{2})")
        .expect("valid standard hours regex")
});

// ============================================================================
// Main Structs
// ============================================================================

/// Complete culture data for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultureData {
    /// Locale key (e.g., "fr-FR")
    pub locale_key: String,

    // TIER 1: INDEXED SCALARS (Cypher-queryable)
    /// Geographic hemisphere affecting season references
    pub hemisphere: String,
    /// First day of the business week
    pub work_week_start: String,
    /// Cultural communication style preference
    pub communication_directness: String,
    /// Importance of social/business hierarchy
    pub hierarchy_importance: String,
    /// Individual vs collective orientation
    pub individualism_level: String,

    // TIER 2: STRUCTURED JSON (programmatic lookup)
    /// Season definitions with month mappings
    pub seasons: serde_json::Value,
    /// Major holidays with dates and importance
    pub holidays: serde_json::Value,
    /// Typical business operating hours
    pub business_hours: serde_json::Value,
    /// Core cultural values as ordered list
    pub values: Vec<CoreValue>,
    /// Communication conventions
    pub communication_norms: serde_json::Value,

    // TIER 3: LLM CONTEXT (narrative text)
    /// Summary of cultural context for LLM
    pub culture_summary: String,
    /// Summary of taboos and sensitivities
    pub taboos_summary: String,
    /// Full source markdown
    pub raw_markdown: String,

    // Metadata
    pub template_version: String,
    pub last_updated: String,
    pub source_file: String,
}

/// A core cultural value with importance and marketing angle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValue {
    pub value: String,
    pub importance: String,
    pub expression: String,
    pub marketing_angle: String,
}

/// Communication style characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    pub directness_level: String,
    pub context_type: String,
    pub hierarchy_sensitivity: String,
}

/// Taboo topic with severity and notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taboo {
    pub topic: String,
    pub severity: String,
    pub notes: String,
}

impl Default for CultureData {
    fn default() -> Self {
        Self {
            locale_key: String::new(),
            hemisphere: "northern".to_string(),
            work_week_start: "monday".to_string(),
            communication_directness: "balanced".to_string(),
            hierarchy_importance: "medium".to_string(),
            individualism_level: "mixed".to_string(),
            seasons: serde_json::json!({}),
            holidays: serde_json::json!([]),
            business_hours: serde_json::json!({}),
            values: Vec::new(),
            communication_norms: serde_json::json!({}),
            culture_summary: String::new(),
            taboos_summary: String::new(),
            raw_markdown: String::new(),
            template_version: "3.1".to_string(),
            last_updated: String::new(),
            source_file: String::new(),
        }
    }
}

impl CultureData {
    /// Generate LLM context summary from culture data.
    pub fn generate_culture_summary(&mut self) {
        let mut parts = Vec::new();

        // Key characteristics
        parts.push(format!(
            "{}: Communication style is {}.",
            self.locale_key, self.communication_directness
        ));

        // Hierarchy
        parts.push(format!(
            "Hierarchy importance: {}.",
            self.hierarchy_importance
        ));

        // Individualism
        parts.push(format!(
            "Orientation: {}.",
            match self.individualism_level.as_str() {
                "individualist" => "Individualist (focus on 'you')",
                "collectivist" => "Collectivist (focus on 'we/together')",
                _ => "Mixed individual/collective",
            }
        ));

        // Top values
        if !self.values.is_empty() {
            let top_values: Vec<&str> = self
                .values
                .iter()
                .filter(|v| v.importance.to_lowercase() == "high")
                .take(3)
                .map(|v| v.value.as_str())
                .collect();
            if !top_values.is_empty() {
                parts.push(format!("Core values: {}.", top_values.join(", ")));
            }
        }

        self.culture_summary = parts.join(" ");
    }

    /// Generate taboos summary from culture data.
    pub fn generate_taboos_summary(&mut self, taboos: &[Taboo]) {
        let critical: Vec<&str> = taboos
            .iter()
            .filter(|t| t.severity.to_lowercase() == "critical")
            .take(3)
            .map(|t| t.topic.as_str())
            .collect();

        if !critical.is_empty() {
            self.taboos_summary = format!(
                "CRITICAL TABOOS: {}. Always avoid these topics.",
                critical.join(", ")
            );
        }
    }
}

// ============================================================================
// Loading Functions
// ============================================================================

/// Load all culture files from ATH data directory.
pub fn load_all_cultures(ath_path: &Path) -> Result<Vec<CultureData>> {
    let culture_dir = ath_path.join("4-culture-norms");

    if !culture_dir.exists() {
        return Err(NovaNetError::Validation(format!(
            "Culture directory not found: {}",
            culture_dir.display()
        )));
    }

    // Collect all .md files
    let files: Vec<_> = WalkDir::new(&culture_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();

    // Parse in parallel
    let cultures: Vec<CultureData> = files
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            match parse_culture_file(path) {
                Ok(c) => Some(c),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();

    Ok(cultures)
}

/// Parse a single culture markdown file.
pub fn parse_culture_file(path: &Path) -> Result<CultureData> {
    let content = fs::read_to_string(path)?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Extract locale from filename (e.g., "fr-FR.md" -> "fr-FR")
    let locale = filename.trim_end_matches(".md");

    parse_culture_markdown(&content, locale, filename)
}

/// Parse culture markdown content.
pub fn parse_culture_markdown(
    content: &str,
    locale_key: &str,
    source_file: &str,
) -> Result<CultureData> {
    // Parse frontmatter
    let (template_version, last_updated) = parse_frontmatter(content);

    // Split into sections
    let sections = split_sections(content);

    // Parse core values
    let values = parse_values_section(
        sections
            .get("core values")
            .map(|s| s.as_str())
            .unwrap_or(""),
    );

    // Parse communication style
    let communication_section = sections
        .get("communication style")
        .map(|s| s.as_str())
        .unwrap_or("");
    let communication_style = parse_communication_style(communication_section);

    // Parse taboos
    let taboos = parse_taboos_section(sections.get("taboos").map(|s| s.as_str()).unwrap_or(""));

    // Parse time & scheduling
    let time_section = sections.get("time").map(|s| s.as_str()).unwrap_or("");
    let (work_week_start, business_hours) = parse_time_section(time_section);

    // Build communication norms JSON
    let communication_norms = serde_json::json!({
        "directness": communication_style.directness_level,
        "context_type": communication_style.context_type,
        "hierarchy_sensitivity": communication_style.hierarchy_sensitivity,
    });

    // Infer hemisphere from locale (simplified heuristic)
    let hemisphere = infer_hemisphere(locale_key);

    // Infer individualism from context type
    let individualism_level = match communication_style.context_type.to_lowercase().as_str() {
        "low_context" | "low" => "individualist".to_string(),
        "high_context" | "high" => "collectivist".to_string(),
        _ => "mixed".to_string(),
    };

    let mut data = CultureData {
        locale_key: locale_key.to_string(),
        hemisphere,
        work_week_start,
        communication_directness: communication_style.directness_level.to_lowercase(),
        hierarchy_importance: communication_style.hierarchy_sensitivity.to_lowercase(),
        individualism_level,
        seasons: serde_json::json!({}), // Will be populated from other sources if needed
        holidays: serde_json::json!([]), // Will be populated from other sources if needed
        business_hours,
        values,
        communication_norms,
        culture_summary: String::new(),
        taboos_summary: String::new(),
        raw_markdown: content.to_string(),
        template_version,
        last_updated,
        source_file: source_file.to_string(),
    };

    // Generate summaries
    data.generate_culture_summary();
    data.generate_taboos_summary(&taboos);

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
        .unwrap_or_else(|| "3.1".to_string());

    let date = RE_DATE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    (version, date)
}

/// Split content into sections by ## headers.
fn split_sections(content: &str) -> HashMap<String, String> {
    let mut sections = HashMap::new();

    let mut current_section: Option<String> = None;
    let mut current_content = String::new();

    for line in content.lines() {
        if let Some(caps) = RE_SECTION.captures(line) {
            // Save previous section
            if let Some(ref name) = current_section {
                sections.insert(name.clone(), current_content.clone());
            }

            // Start new section
            let section_name = caps
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            current_section = Some(normalize_section_name(&section_name));
            current_content = String::new();
        } else if current_section.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Save last section
    if let Some(name) = current_section {
        sections.insert(name, current_content);
    }

    sections
}

/// Normalize section name to standard key.
fn normalize_section_name(name: &str) -> String {
    if name.contains("core values") || name.contains("values") {
        "core values".to_string()
    } else if name.contains("communication style") || name.contains("communication") {
        "communication style".to_string()
    } else if name.contains("taboos") || name.contains("sensitivities") {
        "taboos".to_string()
    } else if name.contains("time") || name.contains("scheduling") {
        "time".to_string()
    } else if name.contains("social norms") || name.contains("social") {
        "social norms".to_string()
    } else if name.contains("content prohibitions") || name.contains("prohibitions") {
        "prohibitions".to_string()
    } else if name.contains("religious") || name.contains("cultural phrases") {
        "phrases".to_string()
    } else {
        name.to_string()
    }
}

/// Parse core values from section content.
fn parse_values_section(content: &str) -> Vec<CoreValue> {
    let mut values = Vec::new();

    // Parse table rows: | Value | Importance | Expression | Marketing Angle |
    for caps in RE_TABLE_4COL.captures_iter(content) {
        let value = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let importance = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let expression = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");
        let marketing = caps.get(4).map(|m| m.as_str().trim()).unwrap_or("");

        // Skip header row and separator
        if value.to_lowercase() == "value"
            || importance.to_lowercase() == "importance"
            || value.contains("---")
            || value.is_empty()
        {
            continue;
        }

        values.push(CoreValue {
            value: value.to_string(),
            importance: importance.to_string(),
            expression: expression.to_string(),
            marketing_angle: marketing.to_string(),
        });
    }

    values
}

/// Parse communication style from section content.
fn parse_communication_style(content: &str) -> CommunicationStyle {
    // Extract directness level
    let directness = RE_DIRECTNESS
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "BALANCED".to_string());

    // Extract context type
    let context_type = RE_CONTEXT_TYPE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "MEDIUM_CONTEXT".to_string());

    // Extract hierarchy sensitivity
    let hierarchy = RE_HIERARCHY
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "MEDIUM".to_string());

    CommunicationStyle {
        directness_level: directness,
        context_type,
        hierarchy_sensitivity: hierarchy,
    }
}

/// Parse taboos and sensitivities from section content.
fn parse_taboos_section(content: &str) -> Vec<Taboo> {
    let mut taboos = Vec::new();

    // Parse table rows: | Topic | Severity | Notes |
    for caps in RE_TABLE_3COL.captures_iter(content) {
        let topic = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let severity = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let notes = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");

        // Skip header row and separator
        if topic.to_lowercase() == "topic" || topic.contains("---") || topic.is_empty() {
            continue;
        }

        taboos.push(Taboo {
            topic: topic.to_string(),
            severity: severity.to_string(),
            notes: notes.to_string(),
        });
    }

    taboos
}

/// Parse time and scheduling section.
fn parse_time_section(content: &str) -> (String, serde_json::Value) {
    // Default values
    let mut work_week_start = "monday".to_string();
    let mut business_start = "09:00".to_string();
    let mut business_end = "17:00".to_string();
    let mut has_lunch_break = false;

    // Extract work week start from work week rows
    // Look for patterns like "| Work days | Sunday - Thursday |" or "Work days | Monday-Friday"
    if let Some(caps) = RE_WORK_DAYS.captures(content) {
        let first_day = caps
            .get(1)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        if first_day == "sunday" {
            work_week_start = "sunday".to_string();
        } else if first_day == "saturday" {
            work_week_start = "saturday".to_string();
        }
        // Monday is the default, no need to set
    }

    // Parse business hours table: | Type | Hours | Best Contact Time |
    if let Some(caps) = RE_STANDARD_HOURS.captures(content) {
        if let Some(start) = caps.get(1) {
            business_start = start.as_str().to_string();
        }
        if let Some(end) = caps.get(2) {
            business_end = end.as_str().to_string();
        }
    }

    // Check for lunch break mentions
    if content.contains("lunch") || content.contains("Lunch") {
        has_lunch_break = true;
    }

    let business_hours = serde_json::json!({
        "start": business_start,
        "end": business_end,
        "lunch_break": has_lunch_break,
    });

    (work_week_start, business_hours)
}

/// Infer hemisphere from locale key using country code heuristics.
fn infer_hemisphere(locale_key: &str) -> String {
    // Extract country code from locale (e.g., "fr-FR" -> "FR")
    let country = locale_key
        .split('-')
        .nth(1)
        .unwrap_or(locale_key)
        .to_uppercase();

    // Southern hemisphere countries (simplified list)
    let southern = [
        "AU", "NZ", "ZA", "AR", "CL", "BR", "UY", "PY", "PE", "BO", "ID", "TL", "PG", "FJ", "NC",
        "MG", "MZ", "ZW", "BW", "NA",
    ];

    if southern.contains(&country.as_str()) {
        "southern".to_string()
    } else {
        "northern".to_string()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const FR_FR_SAMPLE: &str = r#"---
locale: fr-FR
type: culture-norms
template_version: 3.1
last_updated: 2025-01-10
---

# Culture Norms: fr-FR

## 1. Core Values

### 1.1 Dominant Values

| Value | Importance | Expression | Marketing Angle |
|-------|------------|------------|-----------------|
| Liberte (Freedom) | High | Personal autonomy, freedom of expression | Emphasize choice, autonomy |
| Egalite (Equality) | High | Social equality, meritocracy | Position products as accessible |
| Laicite (Secularism) | High | Separation of religion and state | Keep religious references neutral |
| Excellence / Savoir-faire | High | Quality craftsmanship, expertise | Highlight quality, craftsmanship |

---

## 2. Communication Style

### 2.1 Directness Level

**Scale**: BALANCED

| Aspect | Characteristic | Example |
|--------|----------------|---------|
| Feedback | Diplomatic but clear | "Il serait peut-etre preferable de..." |

### 2.2 Context Culture

**Type**: HIGH_CONTEXT

### 2.3 Hierarchy Sensitivity

**Level**: MEDIUM

---

## 3. Taboos & Sensitivities

### 3.1 Topics to Avoid

| Topic | Severity | Notes |
|-------|----------|-------|
| Personal income/wealth | Critical | Discussing money is vulgar |
| Religious affiliation | Critical | Laicite means religion is private |
| Political extremism | Critical | Far-right and far-left are divisive |

---

## 7. Time & Scheduling

### 7.1 Work Week

| Aspect | Details | Notes |
|--------|---------|-------|
| Work days | Monday-Friday | 35-hour work week standard |
| Weekend | Saturday-Sunday | Sacred family/leisure time |

### 7.2 Business Hours

| Type | Hours | Best Contact Time |
|------|-------|-------------------|
| Standard office | 09:00-18:00 | 10:00-12:00, 14:00-17:00 |

"#;

    #[test]
    fn test_parse_culture_markdown_basic() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.locale_key, "fr-FR");
        assert_eq!(result.template_version, "3.1");
        assert_eq!(result.last_updated, "2025-01-10");
    }

    #[test]
    fn test_parse_communication_directness() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.communication_directness, "balanced");
    }

    #[test]
    fn test_parse_hierarchy_importance() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.hierarchy_importance, "medium");
    }

    #[test]
    fn test_parse_individualism_from_context() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // HIGH_CONTEXT -> collectivist
        assert_eq!(result.individualism_level, "collectivist");
    }

    #[test]
    fn test_parse_core_values() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.values.len(), 4);
        assert_eq!(result.values[0].value, "Liberte (Freedom)");
        assert_eq!(result.values[0].importance, "High");
    }

    #[test]
    fn test_parse_work_week_start() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.work_week_start, "monday");
    }

    #[test]
    fn test_parse_business_hours() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let hours = &result.business_hours;
        assert_eq!(hours["start"], "09:00");
        assert_eq!(hours["end"], "18:00");
    }

    #[test]
    fn test_parse_hemisphere_northern() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.hemisphere, "northern");
    }

    #[test]
    fn test_parse_hemisphere_southern() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "pt-BR", "pt-BR.md").unwrap();

        assert_eq!(result.hemisphere, "southern");
    }

    #[test]
    fn test_generate_culture_summary() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.culture_summary.is_empty());
        assert!(result.culture_summary.contains("balanced"));
    }

    #[test]
    fn test_generate_taboos_summary() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.taboos_summary.is_empty());
        assert!(result.taboos_summary.contains("CRITICAL"));
    }

    #[test]
    fn test_communication_norms_json() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.communication_norms["directness"], "BALANCED");
        assert_eq!(result.communication_norms["context_type"], "HIGH_CONTEXT");
        assert_eq!(
            result.communication_norms["hierarchy_sensitivity"],
            "MEDIUM"
        );
    }

    #[test]
    fn test_raw_markdown_preserved() {
        let result = parse_culture_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.raw_markdown.is_empty());
        assert!(result.raw_markdown.contains("Culture Norms: fr-FR"));
    }

    // Test with en-US style content (different structure)
    const EN_US_SAMPLE: &str = r#"---
locale: en-US
type: culture-norms
template_version: 3.1
last_updated: 2026-01-10
---

# Culture Norms: en-US

## 1. Core Values

### 1.1 Dominant Values

| Value | Importance | Expression | Marketing Angle |
|-------|------------|------------|-----------------|
| Individual Freedom | High | Personal choice paramount | Empower customer choice |
| Achievement/Success | High | Hard work pays off | Aspirational messaging |
| Optimism | High | Future is better | Positive framing |

---

## 2. Communication Style

### 2.1 Directness Level

**Scale**: DIRECT

### 2.2 Context Culture

**Type**: LOW_CONTEXT

### 2.3 Hierarchy Sensitivity

**Level**: LOW

---

## 3. Taboos & Sensitivities

### 3.1 Topics to Avoid

| Topic | Severity | Notes |
|-------|----------|-------|
| Racism and discrimination | Critical | Zero tolerance |
| Partisan politics | High | Country deeply divided |

---

## 7. Time & Scheduling

### 7.1 Work Week

| Aspect | Details | Notes |
|--------|---------|-------|
| Work days | Monday - Friday | Traditional 5-day week |

### 7.2 Business Hours

| Type | Hours | Best Contact Time |
|------|-------|-------------------|
| Standard | 09:00 - 17:00 local time | 10:00 - 15:00 |

"#;

    #[test]
    fn test_parse_direct_communication() {
        let result = parse_culture_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        assert_eq!(result.communication_directness, "direct");
    }

    #[test]
    fn test_parse_low_hierarchy() {
        let result = parse_culture_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        assert_eq!(result.hierarchy_importance, "low");
    }

    #[test]
    fn test_parse_individualist_culture() {
        let result = parse_culture_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        // LOW_CONTEXT -> individualist
        assert_eq!(result.individualism_level, "individualist");
    }

    #[test]
    fn test_infer_hemisphere_australia() {
        assert_eq!(infer_hemisphere("en-AU"), "southern");
    }

    #[test]
    fn test_infer_hemisphere_new_zealand() {
        assert_eq!(infer_hemisphere("en-NZ"), "southern");
    }

    #[test]
    fn test_infer_hemisphere_south_africa() {
        assert_eq!(infer_hemisphere("af-ZA"), "southern");
    }

    #[test]
    fn test_infer_hemisphere_usa() {
        assert_eq!(infer_hemisphere("en-US"), "northern");
    }

    #[test]
    fn test_infer_hemisphere_japan() {
        assert_eq!(infer_hemisphere("ja-JP"), "northern");
    }
}
