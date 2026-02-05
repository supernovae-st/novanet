//! Locale rules parsers for NovaNet.
//!
//! Parses markdown files from localization-data sources:
//! - `2-rules-formatting/*.md` → Formatting rules (dates, numbers, currency, phone, address)
//! - `2-rules-slug/*.md` → Slugification rules (stopwords, transliteration)
//! - `2-rules-adaptation/*.md` → Adaptation rules (FACTS vs ILLUSTRATIONS)
//!
//! v10.6.1: Initial implementation

pub mod adaptation;
pub mod formatting;
pub mod markdown;
pub mod slugification;

pub use adaptation::{AdaptationData, parse_adaptation};
pub use formatting::{FormattingData, parse_formatting};
pub use slugification::{SlugificationData, parse_slugification};

use std::path::Path;

/// Frontmatter extracted from locale rules markdown files.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LocaleRulesFrontmatter {
    pub locale: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub template_version: Option<String>,
    pub last_updated: Option<String>,
}

/// Parse YAML frontmatter from markdown content.
///
/// Expects format:
/// ```text
/// ---
/// locale: fr-FR
/// type: rules-formatting
/// ---
/// # Content...
/// ```
pub fn parse_frontmatter(content: &str) -> Option<(LocaleRulesFrontmatter, &str)> {
    if !content.starts_with("---") {
        return None;
    }

    let rest = &content[3..];
    let end_idx = rest.find("\n---")?;
    let frontmatter_str = &rest[..end_idx];
    let body = &rest[end_idx + 4..].trim_start();

    let frontmatter: LocaleRulesFrontmatter = serde_yaml::from_str(frontmatter_str).ok()?;
    Some((frontmatter, body))
}

/// Load all locale rules from a directory.
///
/// Returns a map of locale code -> parsed data.
pub fn load_all_from_dir<T, F>(
    dir: &Path,
    parser: F,
) -> crate::Result<std::collections::HashMap<String, T>>
where
    F: Fn(&str) -> crate::Result<T>,
{
    use std::collections::HashMap;

    let mut results = HashMap::new();

    if !dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "Directory not found: {}",
            dir.display()
        )));
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let content = std::fs::read_to_string(&path)?;
                match parser(&content) {
                    Ok(data) => {
                        results.insert(stem.to_string(), data);
                    }
                    Err(e) => {
                        eprintln!("  Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
locale: fr-FR
type: rules-formatting
template_version: "2.0"
---

# Rules Formatting: fr-FR

Content here...
"#;

        let (fm, body) = parse_frontmatter(content).expect("Should parse frontmatter");
        assert_eq!(fm.locale, "fr-FR");
        assert_eq!(fm.rule_type, "rules-formatting");
        assert_eq!(fm.template_version, Some("2.0".to_string()));
        assert!(body.starts_with("# Rules Formatting"));
    }

    #[test]
    fn test_parse_frontmatter_no_frontmatter() {
        let content = "# Just content\n\nNo frontmatter here.";
        assert!(parse_frontmatter(content).is_none());
    }
}
