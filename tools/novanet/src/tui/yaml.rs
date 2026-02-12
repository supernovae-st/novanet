//! YAML section parser for contextual view in TUI.
//!
//! Splits a NodeKind YAML file into two sections:
//! - **Kind section**: name, realm, layer, trait, description, icon, llm_context
//! - **Instance section**: standard_properties, properties
//!
//! This enables the TUI to show only the relevant section based on context:
//! - v11.7 Graph mode: Realm/Layer/Kind selected → Kind section (schema)
//! - v11.7 Graph mode: Instance selected → Instance section (data)

use std::ops::Range;

/// Parsed YAML sections with line ranges.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // Fields reserved for future use
pub struct YamlSections {
    /// Line range for Kind metadata (name, realm, layer, trait, etc.)
    pub kind_lines: Range<usize>,
    /// Line range for Instance structure (standard_properties, properties)
    pub instance_lines: Range<usize>,
    /// Total line count
    pub total_lines: usize,
    /// Raw YAML content (owned for lifetime safety)
    raw_content: String,
}

/// Keys that belong to the Instance section (structure of instances).
const INSTANCE_KEYS: &[&str] = &["standard_properties:", "properties:"];

#[allow(dead_code)] // Methods reserved for future use
impl YamlSections {
    /// Parse YAML content and identify Kind vs Instance sections.
    ///
    /// Returns `None` if the content doesn't look like a NodeKind YAML
    /// (i.e., doesn't have both kind and instance sections).
    pub fn parse(content: &str) -> Option<Self> {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        if total_lines == 0 {
            return None;
        }

        // Find the start of standard_properties or properties section
        // This marks the boundary between Kind and Instance sections
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

        // If no instance section found, this isn't a NodeKind YAML
        let instance_start = instance_start?;

        // Find where Kind section actually starts (skip initial comments/node:)
        let mut kind_start = 0;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            // Skip comments and empty lines at the start
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Skip "node:" wrapper if present
            if trimmed == "node:" {
                kind_start = i + 1;
                continue;
            }
            // Found actual content
            if kind_start == 0 {
                kind_start = i;
            }
            break;
        }

        Some(Self {
            kind_lines: kind_start..instance_start,
            instance_lines: instance_start..total_lines,
            total_lines,
            raw_content: content.to_string(),
        })
    }

    /// Get the Kind section content (shown when Realm/Layer/Kind selected).
    pub fn kind_content(&self) -> &str {
        self.get_section_content(&self.kind_lines)
    }

    /// Get the Instance section content (shown when Instance selected).
    pub fn instance_content(&self) -> &str {
        self.get_section_content(&self.instance_lines)
    }

    /// Get lines for a specific section.
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

    /// Get lines iterator for Kind section.
    pub fn kind_lines_iter(&self) -> impl Iterator<Item = &str> {
        self.raw_content
            .lines()
            .skip(self.kind_lines.start)
            .take(self.kind_lines.end - self.kind_lines.start)
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

    /// Number of lines in Kind section.
    pub fn kind_line_count(&self) -> usize {
        self.kind_lines.end - self.kind_lines.start
    }

    /// Number of lines in Instance section.
    pub fn instance_line_count(&self) -> usize {
        self.instance_lines.end - self.instance_lines.start
    }

    /// Check if this is a valid NodeKind YAML (has both sections).
    pub fn is_valid(&self) -> bool {
        self.kind_line_count() > 0 && self.instance_line_count() > 0
    }
}

/// Which section is currently active in the YAML view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YamlViewSection {
    #[default]
    Kind,
    Instance,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_YAML: &str = r#"# Locale - Graph-native locale node
# Used by LLM for culturally-appropriate content generation.

node:
  name: Locale
  realm: shared
  layer: config
  trait: invariant
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
        assert!(sections.kind_line_count() > 0);
        assert!(sections.instance_line_count() > 0);

        // Kind section should contain name, realm, etc.
        let kind = sections.kind_content();
        assert!(kind.contains("name: Locale"));
        assert!(kind.contains("realm: shared"));

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
