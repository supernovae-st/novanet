//! novanet_query tool
//!
//! Execute read-only Cypher queries against the NovaNet knowledge graph.

use crate::cache::QueryCache;
use crate::error::{Error, Result};
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

/// Parameters for novanet_query tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct QueryParams {
    /// Cypher query (must be read-only, no CREATE/DELETE/MERGE/SET)
    pub cypher: String,
    /// Query parameters (optional)
    #[serde(default)]
    pub params: Option<serde_json::Map<String, serde_json::Value>>,
    /// Maximum number of results (default: 100)
    #[serde(default)]
    pub limit: Option<usize>,
    /// Query timeout in milliseconds (default: 30000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

/// Result from novanet_query tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct QueryResult {
    /// Query results as JSON array
    pub rows: Vec<serde_json::Value>,
    /// Number of rows returned
    pub row_count: usize,
    /// Token estimate for the result
    pub token_estimate: usize,
    /// Whether result was served from cache
    pub cached: bool,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Execute the novanet_query tool
#[instrument(name = "novanet_query", skip(state), fields(limit = params.limit, cached))]
pub async fn execute(state: &State, params: QueryParams) -> Result<QueryResult> {
    let start = std::time::Instant::now();

    // Apply limit to query
    let cypher = apply_limit(&params.cypher, params.limit.unwrap_or(100));
    debug!(cypher = %cypher, "Executing query");

    // Check cache first
    let cache_key = QueryCache::cache_key(&cypher, &params.params);
    if let Some(cached) = state.cache().get(&cache_key).await {
        state.record_cache_hit();
        let rows: Vec<serde_json::Value> = serde_json::from_value(cached)
            .map_err(|e| Error::Internal(format!("Cache deserialization error: {}", e)))?;
        let row_count = rows.len();
        let token_estimate = estimate_tokens(&rows);

        return Ok(QueryResult {
            rows,
            row_count,
            token_estimate,
            cached: true,
            execution_time_ms: start.elapsed().as_millis() as u64,
        });
    }

    state.record_cache_miss();

    // Execute query
    let rows = state
        .pool()
        .execute_query(&cypher, params.params.clone())
        .await?;
    state.record_query();

    // Cache the result
    let cache_value = serde_json::to_value(&rows)
        .map_err(|e| Error::Internal(format!("Cache serialization error: {}", e)))?;
    state.cache().insert(cache_key, cache_value).await;

    let row_count = rows.len();
    let token_estimate = estimate_tokens(&rows);

    Ok(QueryResult {
        rows,
        row_count,
        token_estimate,
        cached: false,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Apply LIMIT clause if not present
///
/// PERF: Uses case-insensitive search without allocation (avoids to_uppercase()).
fn apply_limit(cypher: &str, limit: usize) -> String {
    // Check for LIMIT without allocating uppercase copy
    let has_limit = cypher
        .as_bytes()
        .windows(5)
        .any(|w| w.eq_ignore_ascii_case(b"LIMIT"));

    if has_limit {
        cypher.to_string()
    } else {
        format!("{} LIMIT {}", cypher.trim_end_matches(';'), limit)
    }
}

/// Estimate tokens for JSON results
fn estimate_tokens(rows: &[serde_json::Value]) -> usize {
    let json_string = serde_json::to_string(rows).unwrap_or_default();
    // Rough estimate: 4 chars per token
    json_string.len().div_ceil(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_limit() {
        // Without LIMIT
        let query = "MATCH (n) RETURN n";
        let limited = apply_limit(query, 50);
        assert!(limited.contains("LIMIT 50"));

        // With existing LIMIT
        let query = "MATCH (n) RETURN n LIMIT 10";
        let limited = apply_limit(query, 50);
        assert!(!limited.contains("LIMIT 50"));
        assert!(limited.contains("LIMIT 10"));
    }

    #[test]
    fn test_estimate_tokens() {
        let rows = vec![
            serde_json::json!({"name": "Test", "value": 123}),
            serde_json::json!({"name": "Another", "value": 456}),
        ];
        let estimate = estimate_tokens(&rows);
        assert!(estimate > 0);
    }
}
