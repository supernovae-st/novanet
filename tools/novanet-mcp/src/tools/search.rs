//! novanet_search tool
//!
//! Fulltext and property search across the NovaNet knowledge graph.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Search mode
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    /// Fulltext search using Neo4j fulltext indexes
    Fulltext,
    /// Property-based search with exact or partial matching
    Property,
    /// Combined fulltext + property search
    #[default]
    Hybrid,
}

/// Parameters for novanet_search tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Search query string
    pub query: String,
    /// Search mode (fulltext, property, hybrid)
    #[serde(default)]
    pub mode: SearchMode,
    /// Filter by node kinds (e.g., ["Entity", "Page", "Block"])
    #[serde(default)]
    pub kinds: Option<Vec<String>>,
    /// Filter by realm (shared, org)
    #[serde(default)]
    pub realm: Option<String>,
    /// Filter by layer
    #[serde(default)]
    pub layer: Option<String>,
    /// Maximum number of results (default: 20)
    #[serde(default)]
    pub limit: Option<usize>,
    /// Properties to search in (for property mode)
    #[serde(default)]
    pub properties: Option<Vec<String>>,
}

/// A single search result
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SearchHit {
    /// Node key
    pub key: String,
    /// Node kind (label)
    pub kind: String,
    /// Match score (0.0 - 1.0)
    pub score: f64,
    /// Matched properties with highlights
    pub matches: Vec<PropertyMatch>,
    /// Node properties (subset)
    pub properties: serde_json::Value,
}

/// A matched property
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PropertyMatch {
    /// Property name
    pub property: String,
    /// Matched value (with highlight markers)
    pub value: String,
}

/// Result from novanet_search tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SearchResult {
    /// Search hits
    pub hits: Vec<SearchHit>,
    /// Total hits found (may be > returned if limited)
    pub total_hits: usize,
    /// Search mode used
    pub mode: String,
    /// Token estimate for the result
    pub token_estimate: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Execute the novanet_search tool
#[instrument(name = "novanet_search", skip(state), fields(query = %params.query, mode = ?params.mode))]
pub async fn execute(state: &State, params: SearchParams) -> Result<SearchResult> {
    let start = std::time::Instant::now();
    let limit = params.limit.unwrap_or(20).min(100);

    let (hits, mode_str) = match params.mode {
        SearchMode::Fulltext => (fulltext_search(state, &params, limit).await?, "fulltext"),
        SearchMode::Property => (property_search(state, &params, limit).await?, "property"),
        SearchMode::Hybrid => (hybrid_search(state, &params, limit).await?, "hybrid"),
    };

    let total_hits = hits.len();
    let json_string = serde_json::to_string(&hits).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(SearchResult {
        hits,
        total_hits,
        mode: mode_str.to_string(),
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Fulltext search using Neo4j fulltext indexes
async fn fulltext_search(
    state: &State,
    params: &SearchParams,
    limit: usize,
) -> Result<Vec<SearchHit>> {
    // Build fulltext query
    // NovaNet uses fulltext indexes on key, name, description properties
    // Note: fulltext YIELD uses 'node' as variable name
    let kind_filter = build_kind_filter(&params.kinds, "node");
    let realm_filter = build_realm_filter(&params.realm, "node");

    let cypher = format!(
        r#"
        CALL db.index.fulltext.queryNodes('novanet_fulltext', $query)
        YIELD node, score
        WHERE score > 0.1
        {kind_filter}
        {realm_filter}
        RETURN node.key AS key,
               labels(node)[0] AS kind,
               score,
               node.name AS name,
               node.description AS description,
               properties(node) AS props
        ORDER BY score DESC
        LIMIT {limit}
        "#,
        kind_filter = kind_filter,
        realm_filter = realm_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("query".to_string(), serde_json::json!(params.query));

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| SearchHit {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            score: row["score"].as_f64().unwrap_or(0.0),
            matches: extract_matches(&row, &params.query),
            properties: row["props"].clone(),
        })
        .collect())
}

/// Property-based search
async fn property_search(
    state: &State,
    params: &SearchParams,
    limit: usize,
) -> Result<Vec<SearchHit>> {
    let properties = params.properties.clone().unwrap_or_else(|| {
        vec![
            "key".to_string(),
            "name".to_string(),
            "description".to_string(),
        ]
    });

    // Note: MATCH uses 'n' as variable name
    let kind_filter = build_kind_filter(&params.kinds, "n");
    let realm_filter = build_realm_filter(&params.realm, "n");

    // Build property conditions
    let prop_conditions: Vec<String> = properties
        .iter()
        .map(|p| format!("toLower(toString(n.{})) CONTAINS toLower($query)", p))
        .collect();

    let cypher = format!(
        r#"
        MATCH (n)
        WHERE ({conditions})
        {kind_filter}
        {realm_filter}
        WITH n,
             CASE
                WHEN toLower(n.key) = toLower($query) THEN 1.0
                WHEN toLower(n.key) CONTAINS toLower($query) THEN 0.8
                WHEN toLower(n.name) CONTAINS toLower($query) THEN 0.6
                ELSE 0.4
             END AS score
        RETURN n.key AS key,
               labels(n)[0] AS kind,
               score,
               n.name AS name,
               n.description AS description,
               properties(n) AS props
        ORDER BY score DESC
        LIMIT {limit}
        "#,
        conditions = prop_conditions.join(" OR "),
        kind_filter = kind_filter,
        realm_filter = realm_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("query".to_string(), serde_json::json!(params.query));

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| SearchHit {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or_default().to_string(),
            score: row["score"].as_f64().unwrap_or(0.0),
            matches: extract_matches(&row, &params.query),
            properties: row["props"].clone(),
        })
        .collect())
}

/// Hybrid search combining fulltext and property search
async fn hybrid_search(
    state: &State,
    params: &SearchParams,
    limit: usize,
) -> Result<Vec<SearchHit>> {
    // Try fulltext first, fall back to property search if no results
    let fulltext_hits = fulltext_search(state, params, limit).await?;

    if !fulltext_hits.is_empty() {
        return Ok(fulltext_hits);
    }

    // Fallback to property search
    property_search(state, params, limit).await
}

/// Build kind filter clause
///
/// # Arguments
/// * `kinds` - Optional list of node kinds to filter by
/// * `var` - Cypher variable name (e.g., "n" for MATCH, "node" for fulltext YIELD)
fn build_kind_filter(kinds: &Option<Vec<String>>, var: &str) -> String {
    match kinds {
        Some(k) if !k.is_empty() => {
            let labels: Vec<String> = k.iter().map(|l| format!("{}:{}", var, l)).collect();
            format!("AND ({})", labels.join(" OR "))
        }
        _ => String::new(),
    }
}

/// Build realm filter clause
///
/// # Arguments
/// * `realm` - Optional realm to filter by
/// * `var` - Cypher variable name (e.g., "n" for MATCH, "node" for fulltext YIELD)
fn build_realm_filter(realm: &Option<String>, var: &str) -> String {
    match realm {
        Some(r) => format!("AND {}.realm = '{}'", var, r),
        None => String::new(),
    }
}

/// Extract property matches from a result row
fn extract_matches(row: &serde_json::Value, query: &str) -> Vec<PropertyMatch> {
    let mut matches = Vec::new();
    let query_lower = query.to_lowercase();

    for prop in ["key", "name", "description"] {
        if let Some(value) = row.get(prop).and_then(|v| v.as_str()) {
            if value.to_lowercase().contains(&query_lower) {
                matches.push(PropertyMatch {
                    property: prop.to_string(),
                    value: value.to_string(),
                });
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_kind_filter() {
        // Test with 'n' variable (property search)
        assert_eq!(build_kind_filter(&None, "n"), "");
        assert_eq!(build_kind_filter(&Some(vec![]), "n"), "");
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity".to_string()]), "n"),
            "AND (n:Entity)"
        );
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity".to_string(), "Page".to_string()]), "n"),
            "AND (n:Entity OR n:Page)"
        );

        // Test with 'node' variable (fulltext search)
        assert_eq!(
            build_kind_filter(&Some(vec!["Entity".to_string()]), "node"),
            "AND (node:Entity)"
        );
        assert_eq!(
            build_kind_filter(
                &Some(vec!["Entity".to_string(), "Page".to_string()]),
                "node"
            ),
            "AND (node:Entity OR node:Page)"
        );
    }

    #[test]
    fn test_build_realm_filter() {
        // Test with 'n' variable (property search)
        assert_eq!(build_realm_filter(&None, "n"), "");
        assert_eq!(
            build_realm_filter(&Some("shared".to_string()), "n"),
            "AND n.realm = 'shared'"
        );

        // Test with 'node' variable (fulltext search)
        assert_eq!(
            build_realm_filter(&Some("shared".to_string()), "node"),
            "AND node.realm = 'shared'"
        );
    }

    #[test]
    fn test_extract_matches() {
        let row = serde_json::json!({
            "key": "test-entity",
            "name": "Test Entity Name",
            "description": "A test entity for testing"
        });

        let matches = extract_matches(&row, "test");
        assert_eq!(matches.len(), 3);
        assert!(matches.iter().any(|m| m.property == "key"));
        assert!(matches.iter().any(|m| m.property == "name"));
        assert!(matches.iter().any(|m| m.property == "description"));
    }
}
