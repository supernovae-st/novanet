//! Output formatting for CLI commands.
//!
//! Supports three formats:
//! - **Table**: Human-readable ASCII table (tabled crate)
//! - **Json**: Machine-readable JSON
//! - **Cypher**: Raw Cypher query for copy-paste into Neo4j Browser

use clap::ValueEnum;
use serde::Serialize;
use tabled::Tabled;

use crate::cypher::CypherStatement;

/// Output format flag shared by all read commands.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
    Cypher,
}

/// A row returned by read commands (data/meta/overlay/query).
#[derive(Debug, Clone, Serialize, Tabled)]
pub struct NodeRow {
    pub label: String,
    pub key: String,
    pub display_name: String,
    pub description: String,
}

/// A row returned by overlay mode (includes schema flag).
#[derive(Debug, Clone, Serialize, Tabled)]
pub struct OverlayRow {
    pub label: String,
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub is_schema: bool,
}

/// Format node rows as a table string.
#[must_use]
pub fn format_table(rows: &[NodeRow]) -> String {
    if rows.is_empty() {
        return "(no results)".to_string();
    }
    use tabled::settings::Style;
    tabled::Table::new(rows).with(Style::rounded()).to_string()
}

/// Format overlay rows as a table string.
#[must_use]
pub fn format_overlay_table(rows: &[OverlayRow]) -> String {
    if rows.is_empty() {
        return "(no results)".to_string();
    }
    use tabled::settings::Style;
    tabled::Table::new(rows).with(Style::rounded()).to_string()
}

/// Format node rows as pretty-printed JSON.
#[must_use]
pub fn format_json<T: Serialize>(rows: &[T]) -> String {
    serde_json::to_string_pretty(rows).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}

/// Format a CypherStatement for display (inlined parameters).
#[must_use]
pub fn format_cypher(stmt: &CypherStatement) -> String {
    let mut output = String::new();
    output.push_str("// Parameterized:\n");
    output.push_str(&stmt.cypher);
    if !stmt.params.is_empty() {
        output.push_str("\n\n// Parameters:\n");
        for (name, value) in &stmt.params {
            let display = match value {
                crate::cypher::ParamValue::StringList(list) => format!("{list:?}"),
                crate::cypher::ParamValue::Int(n) => n.to_string(),
            };
            output.push_str(&format!("//   ${name} = {display}\n"));
        }
        output.push_str("\n// Inlined (copy-paste ready):\n");
        output.push_str(&stmt.render_inline());
    }
    output
}

/// Print formatted output to stdout.
pub fn print_output(output: &str) {
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_table_empty() {
        assert_eq!(format_table(&[]), "(no results)");
    }

    #[test]
    fn format_table_rows() {
        let rows = vec![
            NodeRow {
                label: "Class".to_string(),
                key: "locale".to_string(),
                display_name: "Locale".to_string(),
                description: "Language/region".to_string(),
            },
            NodeRow {
                label: "Class".to_string(),
                key: "project".to_string(),
                display_name: "Project".to_string(),
                description: "Business entity".to_string(),
            },
        ];
        let table = format_table(&rows);
        assert!(table.contains("Locale"));
        assert!(table.contains("Project"));
        assert!(table.contains("Class"));
    }

    #[test]
    fn format_json_rows() {
        let rows = vec![NodeRow {
            label: "Realm".to_string(),
            key: "shared".to_string(),
            display_name: "Shared".to_string(),
            description: "Shared scope".to_string(),
        }];
        let json = format_json(&rows);
        assert!(json.contains("\"key\": \"shared\""));
        assert!(json.contains("\"label\": \"Realm\""));
    }

    #[test]
    fn format_cypher_shows_inline() {
        let stmt = CypherStatement {
            cypher: "MATCH (n) WHERE n.key IN $keys LIMIT $limit".to_string(),
            params: vec![
                (
                    "keys".to_string(),
                    crate::cypher::ParamValue::StringList(vec!["a".to_string()]),
                ),
                ("limit".to_string(), crate::cypher::ParamValue::Int(10)),
            ],
        };
        let output = format_cypher(&stmt);
        assert!(output.contains("Parameterized:"));
        assert!(output.contains("$keys"));
        assert!(output.contains("Inlined"));
        assert!(output.contains("['a']"));
        assert!(output.contains("LIMIT 10"));
    }

    #[test]
    fn overlay_table_shows_is_schema() {
        let rows = vec![OverlayRow {
            label: "Class".to_string(),
            key: "locale".to_string(),
            display_name: "Locale".to_string(),
            description: "".to_string(),
            is_schema: true,
        }];
        let table = format_overlay_table(&rows);
        assert!(table.contains("true"));
    }

    #[test]
    fn overlay_table_empty() {
        assert_eq!(format_overlay_table(&[]), "(no results)");
    }

    #[test]
    fn format_json_empty_array() {
        let rows: Vec<NodeRow> = vec![];
        let json = format_json(&rows);
        assert_eq!(json, "[]");
    }

    #[test]
    fn format_json_special_characters() {
        let rows = vec![NodeRow {
            label: "Class".to_string(),
            key: "test\"quotes".to_string(),
            display_name: "Test <html>".to_string(),
            description: "Line1\nLine2".to_string(),
        }];
        let json = format_json(&rows);
        assert!(json.contains(r#"test\"quotes"#));
        assert!(json.contains("Test <html>"));
    }

    #[test]
    fn format_cypher_no_params() {
        let stmt = CypherStatement {
            cypher: "MATCH (n:Schema) RETURN n".to_string(),
            params: vec![],
        };
        let output = format_cypher(&stmt);
        assert!(output.contains("Parameterized:"));
        assert!(output.contains("MATCH (n:Schema) RETURN n"));
        assert!(!output.contains("Inlined"));
    }

    #[test]
    fn format_table_single_row() {
        let rows = vec![NodeRow {
            label: "Locale".to_string(),
            key: "en_US".to_string(),
            display_name: "English (US)".to_string(),
            description: "".to_string(),
        }];
        let table = format_table(&rows);
        assert!(table.contains("en_US"));
        assert!(table.contains("English (US)"));
    }

    #[test]
    fn overlay_row_is_schema_false() {
        let rows = vec![OverlayRow {
            label: "Page".to_string(),
            key: "home".to_string(),
            display_name: "Home".to_string(),
            description: "Landing page".to_string(),
            is_schema: false,
        }];
        let table = format_overlay_table(&rows);
        assert!(table.contains("false"));
        assert!(table.contains("home"));
    }
}
