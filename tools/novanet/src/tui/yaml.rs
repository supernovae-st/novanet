//! YAML section parser for contextual view in TUI.
//!
//! Splits a NodeClass YAML file into two sections:
//! - **Class section**: name, realm, layer, trait, description, icon, llm_context
//! - **Instance section**: standard_properties, properties
//!
//! This enables the TUI to show only the relevant section based on context:
//! - v11.7 Graph mode: Realm/Layer/Class selected → Class section (schema)
//! - v11.7 Graph mode: Instance selected → Instance section (data)

use std::ops::Range;

/// Parsed YAML sections with pre-computed byte ranges.
///
/// PERF: Stores byte offsets computed once at parse time, avoiding
/// repeated `lines().collect()` allocations on each content access.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // Fields reserved for future use
pub struct YamlSections {
    /// Line range for Class metadata (name, realm, layer, trait, etc.)
    pub class_lines: Range<usize>,
    /// Line range for Instance structure (standard_properties, properties)
    pub instance_lines: Range<usize>,
    /// Total line count
    pub total_lines: usize,
    /// Pre-computed byte range for Class section (O(1) access)
    class_byte_range: Range<usize>,
    /// Pre-computed byte range for Instance section (O(1) access)
    instance_byte_range: Range<usize>,
    /// Raw YAML content (owned for lifetime safety)
    raw_content: String,
}

/// Keys that belong to the Instance section (structure of instances).
const INSTANCE_KEYS: &[&str] = &["standard_properties:", "properties:"];

#[allow(dead_code)] // Methods reserved for future use
impl YamlSections {
    /// Parse YAML content and identify Class vs Instance sections.
    ///
    /// Returns `None` if the content doesn't look like a NodeClass YAML
    /// (i.e., doesn't have both class and instance sections).
    ///
    /// PERF: Computes byte offsets once at parse time for O(1) content access.
    pub fn parse(content: &str) -> Option<Self> {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        if total_lines == 0 {
            return None;
        }

        // Find the start of standard_properties or properties section
        // This marks the boundary between Class and Instance sections
        let mut instance_start: Option<usize> = None;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            for key in INSTANCE_KEYS {
                if trimmed.starts_with(key) {
                    instance_start = Some(i);
                    break;
                }
            }
            if instance_start.is_some() {
                break;
            }
        }

        // If no instance section found, this isn't a NodeClass YAML
        let instance_start = instance_start?;

        // Find where Class section actually starts (skip initial comments/node:)
        let mut class_start = 0;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            // Skip comments and empty lines at the start
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Skip "node:" wrapper if present
            if trimmed == "node:" {
                class_start = i + 1;
                continue;
            }
            // Found actual content
            if class_start == 0 {
                class_start = i;
            }
            break;
        }

        // PERF: Pre-compute byte offsets once (avoid repeated lines().collect())
        let class_byte_start: usize = lines[..class_start].iter().map(|l| l.len() + 1).sum();
        let class_byte_end: usize = class_byte_start
            + lines[class_start..instance_start]
                .iter()
                .map(|l| l.len() + 1)
                .sum::<usize>();
        let instance_byte_end: usize = class_byte_end
            + lines[instance_start..]
                .iter()
                .map(|l| l.len() + 1)
                .sum::<usize>();

        // Clamp to content length (handle missing trailing newline)
        let class_byte_end = class_byte_end.min(content.len());
        let instance_byte_end = instance_byte_end.min(content.len());

        Some(Self {
            class_lines: class_start..instance_start,
            instance_lines: instance_start..total_lines,
            total_lines,
            class_byte_range: class_byte_start..class_byte_end,
            instance_byte_range: class_byte_end..instance_byte_end,
            raw_content: content.to_string(),
        })
    }

    /// Get the Class section content (shown when Realm/Layer/Class selected).
    ///
    /// PERF: O(1) slice using pre-computed byte range.
    pub fn class_content(&self) -> &str {
        &self.raw_content[self.class_byte_range.clone()]
    }

    /// Get the Instance section content (shown when Instance selected).
    ///
    /// PERF: O(1) slice using pre-computed byte range.
    pub fn instance_content(&self) -> &str {
        &self.raw_content[self.instance_byte_range.clone()]
    }

    /// Get lines for a specific section (legacy method for line-based access).
    fn get_section_content(&self, range: &Range<usize>) -> &str {
        let lines: Vec<&str> = self.raw_content.lines().collect();
        if range.start >= lines.len() {
            return "";
        }

        let end = range.end.min(lines.len());
        let start_offset: usize = lines[..range.start].iter().map(|l| l.len() + 1).sum();
        let section_len: usize = lines[range.start..end].iter().map(|l| l.len() + 1).sum();

        // Handle edge case at end of file (no trailing newline)
        let actual_end = (start_offset + section_len).min(self.raw_content.len());
        &self.raw_content[start_offset..actual_end]
    }

    /// Get lines iterator for Class section.
    pub fn class_lines_iter(&self) -> impl Iterator<Item = &str> {
        self.raw_content
            .lines()
            .skip(self.class_lines.start)
            .take(self.class_lines.end - self.class_lines.start)
    }

    /// Get lines iterator for Instance section.
    pub fn instance_lines_iter(&self) -> impl Iterator<Item = &str> {
        self.raw_content
            .lines()
            .skip(self.instance_lines.start)
            .take(self.instance_lines.end - self.instance_lines.start)
    }

    /// Get the full raw content.
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    /// Number of lines in Class section.
    pub fn class_line_count(&self) -> usize {
        self.class_lines.end - self.class_lines.start
    }

    /// Number of lines in Instance section.
    pub fn instance_line_count(&self) -> usize {
        self.instance_lines.end - self.instance_lines.start
    }

    /// Check if this is a valid NodeClass YAML (has both sections).
    pub fn is_valid(&self) -> bool {
        self.class_line_count() > 0 && self.instance_line_count() > 0
    }
}

// v0.13.1: YamlViewSection enum removed (collapse/peek eliminated)

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_YAML: &str = r#"# Locale - Graph-native locale node
# Used by LLM for culturally-appropriate content generation.

node:
  name: Locale
  realm: shared
  layer: config
  trait: defined  // v0.12.0 ADR-024: invariant→defined
  icon: "⊕"
  description: "First-class locale node (BCP 47)"
  llm_context: "USE: for locale-specific content"

  standard_properties:
    key:
      type: string
      required: true
      description: "BCP 47 locale code"
      example: "fr-FR"
    country_code:
      type: string
      required: true
    display_name:
      type: string
      required: true
"#;

    #[test]
    fn test_parse_yaml_sections() {
        let sections = YamlSections::parse(SAMPLE_YAML).unwrap();

        assert!(sections.is_valid());
        assert!(sections.class_line_count() > 0);
        assert!(sections.instance_line_count() > 0);

        // Class section should contain name, realm, etc.
        let class = sections.class_content();
        assert!(class.contains("name: Locale"));
        assert!(class.contains("realm: shared"));

        // Instance section should contain standard_properties
        let instance = sections.instance_content();
        assert!(instance.contains("standard_properties:"));
        assert!(instance.contains("key:"));
    }

    #[test]
    fn test_invalid_yaml() {
        // YAML without standard_properties
        let invalid = r#"
name: Test
realm: shared
"#;
        assert!(YamlSections::parse(invalid).is_none());
    }
}
