//! Neo4j Connection Pool
//!
//! Uses neo4rs with Arc for connection sharing.
//! Pattern from tools/novanet/src/db.rs but simplified for MCP.
//!
//! ## Phase 3 Performance Optimization
//!
//! Added streaming query support with early termination for token budget.
//! This prevents buffering large result sets in memory.

use crate::error::{Error, Result};
use neo4rs::{BoltType, ConfigBuilder, Graph, Query};
use std::sync::Arc;
use std::time::Duration;
use tracing::warn;

use super::cypher_guard::{validate_read_only, validate_write_safe};

/// Neo4j connection pool wrapper
#[derive(Clone)]
pub struct Neo4jPool {
    graph: Arc<Graph>,
    /// Maximum number of retries on transient failures (default: 3)
    max_retries: u32,
    /// Base delay for exponential backoff (default: 100ms)
    retry_base_delay: Duration,
}

/// Result of streaming query execution
#[derive(Debug)]
pub struct StreamingResult {
    /// Rows fetched before termination
    pub rows: Vec<serde_json::Value>,
    /// Whether execution terminated early due to budget
    pub terminated_early: bool,
    /// Total estimated tokens consumed
    pub total_tokens: usize,
}

impl Neo4jPool {
    /// Create a new pool with the given configuration
    ///
    /// # Arguments
    /// * `uri` - Neo4j connection URI (e.g., "bolt://localhost:7687")
    /// * `user` - Neo4j username
    /// * `password` - Neo4j password
    /// * `pool_size` - Maximum number of connections
    pub async fn new(uri: &str, user: &str, password: &str, pool_size: usize) -> Result<Self> {
        // Default fetch_size of 500 for better performance (Phase 3)
        Self::with_fetch_size(uri, user, password, pool_size, 500).await
    }

    /// Create a new pool with custom fetch size (Phase 3)
    ///
    /// # Arguments
    /// * `uri` - Neo4j connection URI
    /// * `user` - Neo4j username
    /// * `password` - Neo4j password
    /// * `pool_size` - Maximum number of connections
    /// * `fetch_size` - Number of rows to fetch per batch
    ///
    /// Note: Neo4rs ConfigBuilder doesn't support connect_timeout directly.
    /// Use Tokio timeout wrapper for operation-level timeouts if needed.
    pub async fn with_fetch_size(
        uri: &str,
        user: &str,
        password: &str,
        pool_size: usize,
        fetch_size: usize,
    ) -> Result<Self> {
        Self::with_retry_config(uri, user, password, pool_size, fetch_size, 3, Duration::from_millis(100)).await
    }

    /// Create a new pool with full configuration including retry settings
    ///
    /// # Arguments
    /// * `uri` - Neo4j connection URI
    /// * `user` - Neo4j username
    /// * `password` - Neo4j password
    /// * `pool_size` - Maximum number of connections
    /// * `fetch_size` - Number of rows to fetch per batch
    /// * `max_retries` - Maximum retry attempts on transient failures (0 = no retry)
    /// * `retry_base_delay` - Base delay for exponential backoff (doubles each attempt)
    pub async fn with_retry_config(
        uri: &str,
        user: &str,
        password: &str,
        pool_size: usize,
        fetch_size: usize,
        max_retries: u32,
        retry_base_delay: Duration,
    ) -> Result<Self> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .max_connections(pool_size)
            .fetch_size(fetch_size)
            .build()
            .map_err(|e: neo4rs::Error| Error::Config(e.to_string()))?;

        let graph = Graph::connect(config)
            .await
            .map_err(|e| Error::connection(uri, e))?;

        Ok(Self {
            graph: Arc::new(graph),
            max_retries,
            retry_base_delay,
        })
    }

    /// Execute a query with exponential backoff retry on transient failures,
    /// then collect all rows as JSON.
    ///
    /// Only retries the `graph.execute()` call, NOT row iteration.
    /// Backoff: base_delay * 2^(attempt-1) — e.g. 100ms, 200ms, 400ms
    async fn execute_with_retry(
        &self,
        cypher: &str,
        params: &Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<Vec<serde_json::Value>> {
        let mut last_err = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let delay = self.retry_base_delay * 2u32.pow(attempt - 1);
                tokio::time::sleep(delay).await;
                warn!(
                    attempt,
                    max_retries = self.max_retries,
                    delay_ms = delay.as_millis() as u64,
                    cypher = &cypher[..cypher.len().min(80)],
                    "Retrying Neo4j query after transient error"
                );
            }

            let query = build_query(cypher, params);
            match self.graph.execute(query).await {
                Ok(mut stream) => {
                    // Collect rows from the stream
                    let mut rows = Vec::new();
                    while let Some(row) = stream.next().await.map_err(|e| Error::query(cypher, e))? {
                        let json_row: serde_json::Value = row
                            .to()
                            .map_err(|e| Error::Internal(format!("Row deserialization failed: {}", e)))?;
                        rows.push(json_row);
                    }
                    return Ok(rows);
                }
                Err(e) => {
                    last_err = Some(e);
                }
            }
        }

        Err(Error::query(
            cypher,
            last_err.expect("retry loop executed at least once"),
        ))
    }

    /// Execute a read-only query and return results as JSON
    pub async fn execute_query(
        &self,
        cypher: &str,
        params: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<Vec<serde_json::Value>> {
        // Validate read-only
        validate_read_only(cypher)?;

        // Execute with retry and collect rows
        self.execute_with_retry(cypher, &params).await
    }

    /// Execute a query and return the first result
    pub async fn execute_single(
        &self,
        cypher: &str,
        params: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<Option<serde_json::Value>> {
        let results = self.execute_query(cypher, params).await?;
        Ok(results.into_iter().next())
    }

    /// Execute a write query (MERGE, SET, CREATE) and return results as JSON
    ///
    /// This method allows write operations (MERGE, SET) that are blocked by execute_query.
    /// Use this ONLY for novanet_write tool operations.
    ///
    /// Security: Still validates against dangerous APOC procedures and LOAD CSV.
    pub async fn execute_write(
        &self,
        cypher: &str,
        params: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<Vec<serde_json::Value>> {
        // Validate against dangerous operations (APOC, LOAD CSV) but allow MERGE/SET
        validate_write_safe(cypher)?;

        // Execute with retry and collect rows
        self.execute_with_retry(cypher, &params).await
    }

    /// Test connection health
    pub async fn health_check(&self) -> Result<bool> {
        let result = self.execute_query("RETURN 1 AS health", None).await?;
        Ok(!result.is_empty())
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Streaming Queries (Phase 3 Performance Optimization)
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Execute a read-only query with streaming and token budget termination
    ///
    /// Unlike `execute_query`, this method terminates early when the token budget
    /// is exceeded. This prevents large queries from consuming excessive memory
    /// by buffering all results.
    ///
    /// # Arguments
    /// * `cypher` - The Cypher query to execute
    /// * `params` - Optional query parameters
    /// * `token_budget` - Maximum tokens to fetch before terminating
    ///
    /// # Returns
    /// A `StreamingResult` containing:
    /// - `rows`: The fetched rows (may be partial)
    /// - `terminated_early`: True if budget was exceeded
    /// - `total_tokens`: Estimated tokens consumed
    ///
    /// # Token Estimation
    /// Uses a simple heuristic: `json_chars / 4` for token estimation.
    /// This is fast but approximate.
    pub async fn execute_streaming(
        &self,
        cypher: &str,
        params: Option<serde_json::Map<String, serde_json::Value>>,
        token_budget: usize,
    ) -> Result<StreamingResult> {
        // Validate read-only
        validate_read_only(cypher)?;

        // Execute with retry (only retries graph.execute(), not row iteration)
        let mut last_err = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let delay = self.retry_base_delay * 2u32.pow(attempt - 1);
                tokio::time::sleep(delay).await;
                warn!(
                    attempt,
                    max_retries = self.max_retries,
                    delay_ms = delay.as_millis() as u64,
                    cypher = &cypher[..cypher.len().min(80)],
                    "Retrying Neo4j streaming query after transient error"
                );
            }

            let query = build_query(cypher, &params);
            match self.graph.execute(query).await {
                Ok(mut stream) => {
                    // Stream results with token budget
                    let mut rows = Vec::new();
                    let mut total_tokens = 0;
                    let mut terminated_early = false;

                    while let Some(row) = stream.next().await.map_err(|e| Error::query(cypher, e))? {
                        let json_row: serde_json::Value = row
                            .to()
                            .map_err(|e| Error::Internal(format!("Row deserialization failed: {}", e)))?;

                        // Estimate tokens for this row (chars / 4 heuristic)
                        let row_tokens = match serde_json::to_string(&json_row) {
                            Ok(s) => s.len() / 4,
                            Err(_) => 25, // Default estimate for failed serialization
                        };

                        // Check budget BEFORE adding row
                        if total_tokens + row_tokens > token_budget {
                            terminated_early = true;
                            break;
                        }

                        total_tokens += row_tokens;
                        rows.push(json_row);
                    }

                    return Ok(StreamingResult {
                        rows,
                        terminated_early,
                        total_tokens,
                    });
                }
                Err(e) => {
                    last_err = Some(e);
                }
            }
        }

        Err(Error::query(
            cypher,
            last_err.expect("retry loop executed at least once"),
        ))
    }
}

/// Build a neo4rs Query from cypher and optional params, cloning values for retry safety
fn build_query(cypher: &str, params: &Option<serde_json::Map<String, serde_json::Value>>) -> Query {
    let mut query = Query::new(cypher.to_string());
    if let Some(params) = params {
        for (key, value) in params {
            query = add_param(query, key, value.clone());
        }
    }
    query
}

/// Add a JSON parameter to a Query, converting to appropriate BoltType
fn add_param(query: Query, key: &str, value: serde_json::Value) -> Query {
    match value {
        serde_json::Value::Null => query.param(key, BoltType::Null(neo4rs::BoltNull)),
        serde_json::Value::Bool(b) => query.param(key, b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                query.param(key, i)
            } else if let Some(f) = n.as_f64() {
                query.param(key, f)
            } else {
                // Fallback for very large numbers - convert to string
                query.param(key, n.to_string())
            }
        }
        serde_json::Value::String(s) => query.param(key, s),
        serde_json::Value::Array(arr) => {
            // Convert array to Vec of BoltType
            let bolt_list: Vec<BoltType> = arr.into_iter().map(json_to_bolt_type).collect();
            query.param(key, bolt_list)
        }
        serde_json::Value::Object(obj) => {
            // Convert object to BoltMap
            let bolt_map: std::collections::HashMap<String, BoltType> = obj
                .into_iter()
                .map(|(k, v)| (k, json_to_bolt_type(v)))
                .collect();
            query.param(key, bolt_map)
        }
    }
}

/// Convert a JSON value to BoltType for array/map parameters
fn json_to_bolt_type(value: serde_json::Value) -> BoltType {
    match value {
        serde_json::Value::Null => BoltType::Null(neo4rs::BoltNull),
        serde_json::Value::Bool(b) => BoltType::from(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                BoltType::from(i)
            } else if let Some(f) = n.as_f64() {
                BoltType::from(f)
            } else {
                BoltType::from(n.to_string())
            }
        }
        serde_json::Value::String(s) => BoltType::from(s),
        serde_json::Value::Array(arr) => {
            let bolt_list: Vec<BoltType> = arr.into_iter().map(json_to_bolt_type).collect();
            BoltType::from(bolt_list)
        }
        serde_json::Value::Object(obj) => {
            let bolt_map: std::collections::HashMap<String, BoltType> = obj
                .into_iter()
                .map(|(k, v)| (k, json_to_bolt_type(v)))
                .collect();
            BoltType::from(bolt_map)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_bolt_type() {
        // Test basic types
        let null = json_to_bolt_type(serde_json::Value::Null);
        assert!(matches!(null, BoltType::Null(_)));

        let string = json_to_bolt_type(serde_json::json!("hello"));
        assert!(matches!(string, BoltType::String(_)));

        let int = json_to_bolt_type(serde_json::json!(42));
        assert!(matches!(int, BoltType::Integer(_)));

        let float = json_to_bolt_type(serde_json::json!(2.5));
        assert!(matches!(float, BoltType::Float(_)));

        let bool_val = json_to_bolt_type(serde_json::json!(true));
        assert!(matches!(bool_val, BoltType::Boolean(_)));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // JSON to Bolt Type Edge Cases
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_json_to_bolt_type_arrays() {
        let array = json_to_bolt_type(serde_json::json!([1, 2, 3]));
        assert!(matches!(array, BoltType::List(_)));

        let nested = json_to_bolt_type(serde_json::json!([[1, 2], [3, 4]]));
        assert!(matches!(nested, BoltType::List(_)));

        let empty = json_to_bolt_type(serde_json::json!([]));
        assert!(matches!(empty, BoltType::List(_)));
    }

    #[test]
    fn test_json_to_bolt_type_objects() {
        let obj = json_to_bolt_type(serde_json::json!({"key": "value"}));
        assert!(matches!(obj, BoltType::Map(_)));

        let nested = json_to_bolt_type(serde_json::json!({"outer": {"inner": 1}}));
        assert!(matches!(nested, BoltType::Map(_)));

        let empty = json_to_bolt_type(serde_json::json!({}));
        assert!(matches!(empty, BoltType::Map(_)));
    }

    #[test]
    fn test_json_to_bolt_type_numbers() {
        // Integer boundaries
        let large_int = json_to_bolt_type(serde_json::json!(i64::MAX));
        assert!(matches!(large_int, BoltType::Integer(_)));

        let negative = json_to_bolt_type(serde_json::json!(-42));
        assert!(matches!(negative, BoltType::Integer(_)));

        // Float edge cases
        let small_float = json_to_bolt_type(serde_json::json!(0.0001));
        assert!(matches!(small_float, BoltType::Float(_)));

        let negative_float = json_to_bolt_type(serde_json::json!(-3.5));
        assert!(matches!(negative_float, BoltType::Float(_)));
    }
}
