//! Shared markdown parsing utilities.
//!
//! Common patterns extracted from ATH parsers for DRY code:
//! - YAML frontmatter extraction (--- delimited blocks)
//! - Frontmatter field parsing (template_version, last_updated, etc.)
//! - Section splitting by markdown headers

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

// ============================================================================
// Shared Lazy-compiled Regex Patterns
// ============================================================================

/// YAML frontmatter block: ---\n...\n---
pub static RE_FRONTMATTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)^---\n(.*?)\n---").expect("valid frontmatter regex"));

/// Template version extraction: template_version: <value>
pub static RE_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"template_version:\s*(.+)").expect("valid version regex"));

/// Last updated date extraction: last_updated: <value>
pub static RE_DATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"last_updated:\s*(.+)").expect("valid date regex"));

/// Section header: ## N. Title
pub static RE_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"##\s+\d+\.\s+(.+)").expect("valid section regex"));

// ============================================================================
// Frontmatter Functions
// ============================================================================

/// Extract the raw frontmatter content from a markdown file.
///
/// Returns the text between the opening and closing `---` delimiters,
/// or None if no valid frontmatter block is found.
///
/// # Example
///
/// ```ignore
/// let content = "---\nlocale: fr-FR\nversion: 1.0\n---\n\n# Content";
/// let fm = extract_frontmatter(content);
/// assert_eq!(fm, Some("locale: fr-FR\nversion: 1.0"));
/// ```
pub fn extract_frontmatter(content: &str) -> Option<&str> {
    RE_FRONTMATTER
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
}

/// Extract template_version and last_updated from content.
///
/// Falls back to defaults if not found:
/// - template_version: "2.0"
/// - last_updated: "unknown"
pub fn parse_frontmatter_metadata(content: &str) -> (String, String) {
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

/// Extract a specific field from YAML-style content.
///
/// Searches for `field_name: value` pattern and returns the trimmed value.
pub fn extract_field(content: &str, field_name: &str) -> Option<String> {
    let pattern = format!(r"{}:\s*(.+)", regex::escape(field_name));
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(content))
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}

// ============================================================================
// Section Splitting
// ============================================================================

/// Split markdown content into sections by `## N. Title` headers.
///
/// Returns a HashMap where keys are normalized section names and values
/// are the content under each section (excluding the header line itself).
///
/// Section names are normalized using `normalize_section_name()`.
pub fn split_sections(content: &str) -> HashMap<String, String> {
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

/// Normalize section name to a standard key.
///
/// Maps common section titles to canonical keys:
/// - "number formatting" -> "number"
/// - "date and time" -> "date"
/// - "percentage & temperature" -> "percentage"
pub fn normalize_section_name(name: &str) -> String {
    let name_lower = name.to_lowercase();

    if name_lower.contains("number") {
        "number".to_string()
    } else if name_lower.contains("date") {
        "date".to_string()
    } else if name_lower.contains("time") {
        "time".to_string()
    } else if name_lower.contains("currency") {
        "currency".to_string()
    } else if name_lower.contains("phone") {
        "phone".to_string()
    } else if name_lower.contains("address") {
        "address".to_string()
    } else if name_lower.contains("measurement") {
        "measurement".to_string()
    } else if name_lower.contains("percentage") || name_lower.contains("temperature") {
        "percentage".to_string()
    } else if name_lower.contains("validation") {
        "validation".to_string()
    } else {
        name_lower
    }
}

// ============================================================================
// Value Cleaning
// ============================================================================

/// Clean a value by removing comments and quotes.
///
/// Strips:
/// - Trailing `# comment` or `// comment`
/// - Surrounding backticks, double quotes, single quotes
/// - Leading/trailing whitespace
pub fn clean_value(value: &str) -> String {
    value
        .split('#')
        .next()
        .unwrap_or(value)
        .split("//")
        .next()
        .unwrap_or(value)
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter() {
        let content = "---\nlocale: fr-FR\nversion: 1.0\n---\n\n# Content here";
        let fm = extract_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("locale: fr-FR"));
    }

    #[test]
    fn test_extract_frontmatter_none() {
        let content = "# No frontmatter\n\nJust content.";
        assert!(extract_frontmatter(content).is_none());
    }

    #[test]
    fn test_parse_frontmatter_metadata() {
        let content = "---\ntemplate_version: 3.1\nlast_updated: 2026-01-09\n---";
        let (version, date) = parse_frontmatter_metadata(content);
        assert_eq!(version, "3.1");
        assert_eq!(date, "2026-01-09");
    }

    #[test]
    fn test_parse_frontmatter_metadata_defaults() {
        let content = "# No frontmatter";
        let (version, date) = parse_frontmatter_metadata(content);
        assert_eq!(version, "2.0");
        assert_eq!(date, "unknown");
    }

    #[test]
    fn test_extract_field() {
        let content = "locale: fr-FR\ntype: voice-lexicon";
        assert_eq!(extract_field(content, "locale"), Some("fr-FR".to_string()));
        assert_eq!(
            extract_field(content, "type"),
            Some("voice-lexicon".to_string())
        );
        assert_eq!(extract_field(content, "missing"), None);
    }

    #[test]
    fn test_split_sections() {
        let content = r#"
## 1. Number Formatting

Content for numbers.

## 2. Date Formatting

Content for dates.
"#;
        let sections = split_sections(content);
        assert!(sections.contains_key("number"));
        assert!(sections.contains_key("date"));
        assert!(
            sections
                .get("number")
                .unwrap()
                .contains("Content for numbers")
        );
    }

    #[test]
    fn test_normalize_section_name() {
        assert_eq!(normalize_section_name("number formatting"), "number");
        assert_eq!(normalize_section_name("Date and Time Rules"), "date");
        assert_eq!(
            normalize_section_name("Percentage & Temperature"),
            "percentage"
        );
        assert_eq!(normalize_section_name("custom section"), "custom section");
    }

    #[test]
    fn test_clean_value() {
        assert_eq!(clean_value("`value`"), "value");
        assert_eq!(clean_value("value # comment"), "value");
        assert_eq!(clean_value("value // inline"), "value");
        assert_eq!(clean_value("  spaced  "), "spaced");
        assert_eq!(clean_value("\"quoted\""), "quoted");
        assert_eq!(clean_value("'single'"), "single");
    }
}
