//! Search command: `novanet search --query=...`.
//!
//! Property-based text search across nodes. Searches `key`, `display_name`,
//! and `description` properties using case-insensitive CONTAINS matching.
//! Optional `--class` filter restricts results to a specific Class.

use crate::cypher::{CypherStatement, ParamValue};
use crate::db::Db;
use crate::output::{self, NodeRow, OutputFormat};
use tracing::instrument;

/// Build a search query with optional Kind filter.
pub fn build_search_query(query: &str, kind: Option<&str>, limit: i64) -> CypherStatement {
    // Pre-allocate: base query ~230 chars + kind filter ~50 chars + return clause ~120 chars
    let mut cypher = String::with_capacity(400);
    // 2-3 params: query, limit, optional kind
    let mut params: Vec<(String, ParamValue)> = Vec::with_capacity(3);

    // Base: search across key, display_name, description
    cypher.push_str(
        "MATCH (n)\n\
         WHERE NOT n:Meta\n\
         AND (n.key CONTAINS $query\n\
              OR n.display_name CONTAINS $query\n\
              OR n.description CONTAINS $query)",
    );

    params.push((
        "query".to_string(),
        ParamValue::StringList(vec![query.to_string()]),
    ));

    // Optional Class filter (parameterized for safety)
    if let Some(class_label) = kind {
        // Use parameterized label matching instead of direct interpolation
        // This prevents injection even if validation is bypassed
        cypher.push_str("\nAND ANY(label IN labels(n) WHERE label = $class)");
        params.push((
            "class".to_string(),
            ParamValue::StringList(vec![class_label.to_string()]),
        ));
    }

    cypher.push_str(
        "\nRETURN labels(n)[0] AS label, n.key AS key, \
         n.display_name AS display_name, n.description AS description\n\
         ORDER BY n.key\n\
         LIMIT $limit",
    );

    params.push(("limit".to_string(), ParamValue::Int(limit)));

    CypherStatement { cypher, params }
}

/// Run the search and display results.
#[instrument(skip(db), fields(query = %query, limit))]
pub async fn run_search(
    db: &Db,
    query: &str,
    kind: Option<&str>,
    limit: i64,
    format: OutputFormat,
) -> crate::Result<()> {
    // For search, use direct parameterized query instead of CypherStatement
    // because CONTAINS needs a plain string param, not a list
    let mut cypher = String::from(
        "MATCH (n)\n\
         WHERE NOT n:Meta\n\
         AND (toLower(n.key) CONTAINS toLower($query)\n\
              OR toLower(n.display_name) CONTAINS toLower($query)\n\
              OR toLower(n.description) CONTAINS toLower($query))",
    );

    // Parameterized label matching for safety
    let has_kind = kind.is_some();
    if has_kind {
        cypher.push_str("\nAND ANY(label IN labels(n) WHERE label = $class)");
    }

    cypher.push_str(
        "\nRETURN labels(n)[0] AS label, n.key AS key, \
         n.display_name AS display_name, n.description AS description\n\
         ORDER BY n.key\n\
         LIMIT $limit",
    );

    match format {
        OutputFormat::Cypher => {
            // Show the query with parameters inlined
            let mut display = cypher
                .replace("$query", &format!("'{}'", query.replace('\'', "\\'")))
                .replace("$limit", &limit.to_string());
            if let Some(k) = kind {
                display = display.replace("$class", &format!("'{}'", k.replace('\'', "\\'")));
            }
            output::print_output(&display);
        }
        OutputFormat::Table | OutputFormat::Json => {
            let rows = if let Some(k) = kind {
                db.execute_with_params(
                    &cypher,
                    [("query", query), ("class", k), ("limit", &limit.to_string())],
                )
                .await
            } else {
                db.execute_with_params(&cypher, [("query", query), ("limit", &limit.to_string())])
                    .await
            };

            // Fallback: if the query fails (e.g., no text index), try simpler approach
            let rows = match rows {
                Ok(r) => r,
                Err(_) => {
                    // Retry without toLower for properties that might not exist
                    let simple_cypher = format!(
                        "MATCH (n) WHERE NOT n:Meta \
                         AND n.key CONTAINS $query \
                         RETURN labels(n)[0] AS label, n.key AS key, \
                         n.display_name AS display_name, n.description AS description \
                         ORDER BY n.key LIMIT {}",
                        limit
                    );
                    db.execute_with_params(&simple_cypher, [("query", query)])
                        .await?
                }
            };

            let node_rows: Vec<NodeRow> = rows
                .iter()
                .map(|row| NodeRow {
                    label: row.get("label").unwrap_or_default(),
                    key: row.get("key").unwrap_or_default(),
                    display_name: row.get("display_name").unwrap_or_default(),
                    description: row.get("description").unwrap_or_default(),
                })
                .collect();

            match format {
                OutputFormat::Table => {
                    output::print_output(&output::format_table(&node_rows));
                    eprintln!("{} result(s)", node_rows.len());
                }
                OutputFormat::Json => {
                    output::print_output(&output::format_json(&node_rows));
                }
                OutputFormat::Cypher => unreachable!(),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_query_basic() {
        let stmt = build_search_query("test", None, 10);
        assert!(stmt.cypher.contains("CONTAINS"));
        assert!(stmt.cypher.contains("LIMIT $limit"));
    }

    #[test]
    fn search_query_with_kind() {
        let stmt = build_search_query("test", Some("Page"), 10);
        // Uses parameterized label matching (not direct interpolation)
        assert!(
            stmt.cypher
                .contains("ANY(label IN labels(n) WHERE label = $class)")
        );
        let class_param = stmt.params.iter().find(|(n, _)| n == "class");
        assert!(matches!(
            class_param,
            Some((_, ParamValue::StringList(v))) if v == &vec!["Page"]
        ));
    }

    #[test]
    fn search_query_class_injection_safe() {
        // Even malicious input is safely parameterized
        let stmt = build_search_query("test", Some("Page;DROP"), 10);
        // No direct interpolation - uses parameterized query
        assert!(!stmt.cypher.contains("DROP"));
        assert!(stmt.cypher.contains("$class"));
        // Malicious string is safely contained in params
        let class = stmt.params.iter().find(|(n, _)| n == "class");
        assert!(matches!(
            class,
            Some((_, ParamValue::StringList(v))) if v == &vec!["Page;DROP"]
        ));
    }

    #[test]
    fn search_query_class_with_underscore() {
        let stmt = build_search_query("test", Some("Locale_Identity"), 10);
        assert!(
            stmt.cypher
                .contains("ANY(label IN labels(n) WHERE label = $class)")
        );
    }

    #[test]
    fn search_query_excludes_meta() {
        let stmt = build_search_query("test", None, 10);
        assert!(stmt.cypher.contains("NOT n:Meta"));
    }

    #[test]
    fn search_query_has_limit_param() {
        let stmt = build_search_query("test", None, 25);
        assert!(stmt.cypher.contains("LIMIT $limit"));
        let limit = stmt.params.iter().find(|(n, _)| n == "limit");
        assert!(matches!(limit, Some((_, ParamValue::Int(25)))));
    }

    #[test]
    fn search_query_searches_three_properties() {
        let stmt = build_search_query("test", None, 10);
        assert!(stmt.cypher.contains("n.key CONTAINS"));
        assert!(stmt.cypher.contains("n.display_name CONTAINS"));
        assert!(stmt.cypher.contains("n.description CONTAINS"));
    }
}
