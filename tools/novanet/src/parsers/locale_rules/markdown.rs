//! Markdown parsing utilities for locale rules.
//!
//! Extracts structured data from markdown tables and lists.

use std::collections::HashMap;

/// A parsed markdown table with headers and rows.
#[derive(Debug, Clone, Default)]
pub struct MarkdownTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl MarkdownTable {
    /// Get a column by header name.
    pub fn get_column(&self, header: &str) -> Vec<&str> {
        let header_lower = header.to_lowercase();
        if let Some(idx) = self
            .headers
            .iter()
            .position(|h| h.to_lowercase() == header_lower)
        {
            self.rows
                .iter()
                .filter_map(|row| row.get(idx).map(|s| s.as_str()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Convert to a key-value map (first column = key, second column = value).
    pub fn to_key_value_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for row in &self.rows {
            if row.len() >= 2 {
                let key = row[0].to_lowercase().replace(' ', "_");
                map.insert(key, row[1].clone());
            }
        }
        map
    }

    /// Get rows as maps (header -> value).
    pub fn rows_as_maps(&self) -> Vec<HashMap<String, String>> {
        self.rows
            .iter()
            .map(|row| {
                self.headers
                    .iter()
                    .zip(row.iter())
                    .map(|(h, v)| (h.to_lowercase().replace(' ', "_"), v.clone()))
                    .collect()
            })
            .collect()
    }
}

/// Parse all markdown tables from content.
pub fn parse_tables(content: &str) -> Vec<MarkdownTable> {
    let mut tables = Vec::new();
    let mut current_table: Option<MarkdownTable> = None;
    let mut in_table = false;
    let mut header_parsed = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Table row starts with |
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            let cells: Vec<String> = trimmed
                .trim_matches('|')
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();

            // Skip separator line (contains only dashes and colons)
            if cells
                .iter()
                .all(|c| c.chars().all(|ch| ch == '-' || ch == ':' || ch == ' '))
            {
                continue;
            }

            if !in_table {
                // Start new table, first row is headers
                current_table = Some(MarkdownTable {
                    headers: cells,
                    rows: Vec::new(),
                });
                in_table = true;
                header_parsed = true;
            } else if header_parsed {
                // Data row
                if let Some(ref mut table) = current_table {
                    table.rows.push(cells);
                }
            }
        } else if in_table && !trimmed.is_empty() && !trimmed.starts_with('|') {
            // End of table (non-empty, non-table line)
            if let Some(table) = current_table.take() {
                if !table.rows.is_empty() {
                    tables.push(table);
                }
            }
            in_table = false;
            header_parsed = false;
        }
    }

    // Don't forget last table
    if let Some(table) = current_table {
        if !table.rows.is_empty() {
            tables.push(table);
        }
    }

    tables
}

/// Extract a specific section from markdown content.
///
/// Returns the content between `## {section_name}` and the next `## ` or end of content.
pub fn extract_section(content: &str, section_name: &str) -> Option<String> {
    let section_marker = format!("## {}", section_name);
    let section_marker_lower = section_marker.to_lowercase();

    let mut in_section = false;
    let mut section_content = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        let trimmed_lower = trimmed.to_lowercase();

        if trimmed_lower.starts_with(&section_marker_lower)
            || (trimmed_lower.starts_with("## ")
                && trimmed_lower.contains(&section_name.to_lowercase()))
        {
            in_section = true;
            continue;
        }

        if in_section {
            // Check if we hit another section
            if trimmed.starts_with("## ") || trimmed.starts_with("---") {
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

/// Extract value from a markdown list item like `- **key**: value` or `- **key**: `value``
pub fn extract_list_value(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim().trim_start_matches('-').trim();

    // Match pattern: **key**: value or **key**: `value`
    if let Some(key_end) = trimmed.find("**:") {
        let key_start = trimmed.find("**")?;
        let key = trimmed[key_start + 2..key_end].trim().to_string();
        let value = trimmed[key_end + 3..].trim().trim_matches('`').to_string();
        return Some((key.to_lowercase().replace(' ', "_"), value));
    }

    // Match pattern: **key**: value (without colon inside **)
    if let Some(start) = trimmed.find("**") {
        if let Some(end) = trimmed[start + 2..].find("**") {
            let key = trimmed[start + 2..start + 2 + end].trim().to_string();
            let rest = trimmed[start + 2 + end + 2..].trim();
            if let Some(stripped) = rest.strip_prefix(':') {
                let value = stripped
                    .trim()
                    .trim_matches('`')
                    .trim_matches('(')
                    .trim_matches(')')
                    .to_string();
                return Some((key.to_lowercase().replace(' ', "_"), value));
            }
        }
    }

    None
}

/// Extract all list values from a section.
pub fn extract_list_values(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = extract_list_value(line) {
            map.insert(key, value);
        }
    }
    map
}

/// Parse a bullet list into a Vec of strings.
pub fn parse_bullet_list(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed.starts_with('-') || trimmed.starts_with('*')
        })
        .map(|line| {
            line.trim()
                .trim_start_matches('-')
                .trim_start_matches('*')
                .trim()
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tables() {
        let content = r#"
## Section

| Field | Value |
|-------|-------|
| decimal_separator | , |
| thousands_separator |   |

Some text.

| Other | Table |
|-------|-------|
| a | b |
"#;

        let tables = parse_tables(content);
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0].headers, vec!["Field", "Value"]);
        assert_eq!(tables[0].rows.len(), 2);
        assert_eq!(tables[0].rows[0], vec!["decimal_separator", ","]);
    }

    #[test]
    fn test_table_to_key_value_map() {
        let table = MarkdownTable {
            headers: vec!["Field".to_string(), "Value".to_string()],
            rows: vec![
                vec!["decimal_separator".to_string(), ",".to_string()],
                vec!["thousands_separator".to_string(), " ".to_string()],
            ],
        };

        let map = table.to_key_value_map();
        assert_eq!(map.get("decimal_separator"), Some(&",".to_string()));
        assert_eq!(map.get("thousands_separator"), Some(&" ".to_string()));
    }

    #[test]
    fn test_extract_section() {
        let content = r#"
# Title

## Section One

Content one.

## Section Two

Content two.

## Section Three
"#;

        let section = extract_section(content, "Section Two").unwrap();
        assert!(section.contains("Content two"));
        assert!(!section.contains("Content one"));
    }

    #[test]
    fn test_extract_list_value() {
        assert_eq!(
            extract_list_value("- **decimal_separator**: `,`"),
            Some(("decimal_separator".to_string(), ",".to_string()))
        );

        assert_eq!(
            extract_list_value("- **pattern**: DD/MM/YYYY (e.g., 15/01/2026)"),
            Some((
                "pattern".to_string(),
                "DD/MM/YYYY (e.g., 15/01/2026)".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_bullet_list() {
        let content = r#"
- item one
- item two
* item three
"#;

        let items = parse_bullet_list(content);
        assert_eq!(items, vec!["item one", "item two", "item three"]);
    }
}
