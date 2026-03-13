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

/// Neo4j connection pool wrapper
#[derive(Clone)]
pub struct Neo4jPool {
    graph: Arc<Graph>,
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
        })
    }

    /// Execute a read-only query and return results as JSON
    pub async fn execute_query(
        &self,
        cypher: &str,
        params: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<Vec<serde_json::Value>> {
        // Validate read-only
        validate_read_only(cypher)?;

        // Build query with parameters
        let mut query = Query::new(cypher.to_string());
        if let Some(params) = params {
            for (key, value) in params {
                query = add_param(query, &key, value);
            }
        }

        // Execute query
        let mut result = self
            .graph
            .execute(query)
            .await
            .map_err(|e| Error::query(cypher, e))?;

        // Collect results - deserialize each row to JSON
        let mut rows = Vec::new();
        while let Some(row) = result.next().await.map_err(|e| Error::query(cypher, e))? {
            // Use row.to() to deserialize entire row to serde_json::Value
            let json_row: serde_json::Value = row
                .to()
                .map_err(|e| Error::Internal(format!("Row deserialization failed: {}", e)))?;
            rows.push(json_row);
        }

        Ok(rows)
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

        // Build query with parameters
        let mut query = Query::new(cypher.to_string());
        if let Some(params) = params {
            for (key, value) in params {
                query = add_param(query, &key, value);
            }
        }

        // Execute query
        let mut result = self
            .graph
            .execute(query)
            .await
            .map_err(|e| Error::query(cypher, e))?;

        // Collect results - deserialize each row to JSON
        let mut rows = Vec::new();
        while let Some(row) = result.next().await.map_err(|e| Error::query(cypher, e))? {
            // Use row.to() to deserialize entire row to serde_json::Value
            let json_row: serde_json::Value = row
                .to()
                .map_err(|e| Error::Internal(format!("Row deserialization failed: {}", e)))?;
            rows.push(json_row);
        }

        Ok(rows)
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

        // Build query with parameters
        let mut query = Query::new(cypher.to_string());
        if let Some(params) = params {
            for (key, value) in params {
                query = add_param(query, &key, value);
            }
        }

        // Execute query
        let mut result = self
            .graph
            .execute(query)
            .await
            .map_err(|e| Error::query(cypher, e))?;

        // Stream results with token budget
        let mut rows = Vec::new();
        let mut total_tokens = 0;
        let mut terminated_early = false;

        while let Some(row) = result.next().await.map_err(|e| Error::query(cypher, e))? {
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

        Ok(StreamingResult {
            rows,
            terminated_early,
            total_tokens,
        })
    }
}

/// Check if keyword appears with valid boundaries (no allocations)
/// Checks for keyword preceded/followed by: space, newline, tab, parenthesis, brace, pipe
#[inline]
fn contains_keyword_with_boundary(query: &str, keyword: &str) -> bool {
    // Pre-boundary characters that can appear before keyword
    const PRE_BOUNDARIES: &[char] = &[' ', '\n', '\t', '(', '{', '|'];
    // Post-boundary characters that can appear after keyword
    const POST_BOUNDARIES: &[char] = &[' ', '\n', '\t', '(', '{'];

    let mut start = 0;
    while let Some(pos) = query[start..].find(keyword) {
        let abs_pos = start + pos;

        // Check pre-boundary (or start of string)
        let has_pre_boundary = abs_pos == 0
            || query[..abs_pos]
                .chars()
                .last()
                .is_some_and(|c| PRE_BOUNDARIES.contains(&c));

        // Check post-boundary (or end of string)
        let end_pos = abs_pos + keyword.len();
        let has_post_boundary = end_pos >= query.len()
            || query[end_pos..]
                .chars()
                .next()
                .is_some_and(|c| POST_BOUNDARIES.contains(&c));

        if has_pre_boundary && has_post_boundary {
            return true;
        }

        // Move past this occurrence
        start = abs_pos + 1;
    }

    false
}

/// Check if query starts with keyword (with proper boundary)
#[inline]
fn starts_with_keyword(query: &str, keyword: &str) -> bool {
    if !query.starts_with(keyword) {
        return false;
    }
    // Check what follows the keyword
    let rest = &query[keyword.len()..];
    rest.is_empty()
        || rest.starts_with(' ')
        || rest.starts_with('(')
        || rest.starts_with('\n')
        || rest.starts_with('\t')
}

/// Check for FOREACH ... | WRITE pattern without heap allocations
/// Matches both "|CREATE" and "| CREATE" patterns
#[inline]
fn contains_foreach_write(query: &str, keyword: &str) -> bool {
    // Pattern 1: |KEYWORD (pipe directly followed by keyword)
    if let Some(pos) = query.find('|') {
        let after_pipe = &query[pos + 1..];
        // Check |KEYWORD
        if let Some(rest) = after_pipe.strip_prefix(keyword) {
            if rest.is_empty()
                || rest.starts_with(' ')
                || rest.starts_with('(')
                || rest.starts_with('\n')
            {
                return true;
            }
        }
        // Check | KEYWORD (with space)
        let trimmed = after_pipe.trim_start();
        if let Some(rest) = trimmed.strip_prefix(keyword) {
            if rest.is_empty()
                || rest.starts_with(' ')
                || rest.starts_with('(')
                || rest.starts_with('\n')
            {
                return true;
            }
        }
    }
    false
}

/// Validate that a Cypher query is read-only
///
/// Security checks:
/// 1. Block write keywords (CREATE, DELETE, MERGE, SET, REMOVE, DROP, DETACH)
/// 2. Block dangerous APOC procedures (apoc.cypher.run, apoc.periodic.*, etc.)
/// 3. Block subquery writes (CALL { CREATE ... })
/// 4. Block FOREACH writes (FOREACH ... | CREATE ...)
/// 5. Block LOAD CSV (potential SSRF)
/// 6. Strip comments before validation to prevent bypass
fn validate_read_only(cypher: &str) -> Result<()> {
    // Step 1: Strip comments to prevent bypass attacks
    let cleaned = strip_cypher_comments(cypher);
    let upper = cleaned.to_uppercase();

    // Step 2: Block write keywords anywhere in the query
    // Use static patterns to avoid format!() allocations in hot path
    static WRITE_KEYWORDS: &[&str] = &[
        "CREATE", "DELETE", "MERGE", "SET", "REMOVE", "DROP", "DETACH",
    ];

    for keyword in WRITE_KEYWORDS {
        // Check if keyword appears with valid boundaries (no allocations)
        if contains_keyword_with_boundary(&upper, keyword) {
            return Err(Error::write_not_allowed(*keyword));
        }

        // Check if query starts with keyword
        let trimmed = upper.trim_start();
        if starts_with_keyword(trimmed, keyword) {
            return Err(Error::write_not_allowed(*keyword));
        }
    }

    // Step 3: Block dangerous APOC procedures
    let dangerous_apoc = [
        // Dynamic Cypher execution
        "APOC.CYPHER.RUN",
        "APOC.CYPHER.DOIT",
        "APOC.CYPHER.RUNMANY",
        "APOC.CYPHER.PARALLEL",
        // Periodic/scheduled execution
        "APOC.PERIODIC.COMMIT",
        "APOC.PERIODIC.ITERATE",
        "APOC.PERIODIC.SUBMIT",
        "APOC.PERIODIC.REPEAT",
        // File system access
        "APOC.EXPORT",
        "APOC.IMPORT",
        "APOC.LOAD.CSV",
        "APOC.LOAD.JSON",
        "APOC.LOAD.XML",
        // Schema modifications
        "APOC.SCHEMA.ASSERT",
        "APOC.TRIGGER",
        // Database operations
        "APOC.SYSTEMDB",
    ];

    for proc in dangerous_apoc {
        if upper.contains(proc) {
            return Err(Error::invalid_cypher(format!(
                "Dangerous APOC procedure not allowed: {}",
                proc
            )));
        }
    }

    // Step 4: Block LOAD CSV (SSRF risk)
    if upper.contains("LOAD CSV") || upper.contains("LOAD CSV FROM") {
        return Err(Error::invalid_cypher("LOAD CSV not allowed"));
    }

    // Step 5: Block FOREACH with write operations (extra safety)
    // Check for FOREACH ... | WRITE pattern without allocations
    if upper.contains("FOREACH") {
        for keyword in WRITE_KEYWORDS {
            // Build patterns for FOREACH | keyword matching
            // Pattern 1: |KEYWORD (no space)
            // Pattern 2: | KEYWORD (with space)
            if contains_foreach_write(&upper, keyword) {
                return Err(Error::write_not_allowed(format!(
                    "FOREACH with {}",
                    keyword
                )));
            }
        }
    }

    Ok(())
}

/// Validate that a Cypher write query is safe
///
/// Security checks (less restrictive than read-only):
/// 1. Block dangerous APOC procedures (apoc.cypher.run, apoc.periodic.*, etc.)
/// 2. Block LOAD CSV (potential SSRF)
/// 3. Block DROP (schema modification)
/// 4. Allow MERGE, SET, CREATE, DELETE for novanet_write operations
fn validate_write_safe(cypher: &str) -> Result<()> {
    // Step 1: Strip comments to prevent bypass attacks
    let cleaned = strip_cypher_comments(cypher);
    let upper = cleaned.to_uppercase();

    // Step 2: Block DROP (schema modification not allowed via MCP)
    if contains_keyword_with_boundary(&upper, "DROP") {
        return Err(Error::write_not_allowed("DROP"));
    }

    // Step 3: Block dangerous APOC procedures
    let dangerous_apoc = [
        // Dynamic Cypher execution
        "APOC.CYPHER.RUN",
        "APOC.CYPHER.DOIT",
        "APOC.CYPHER.RUNMANY",
        "APOC.CYPHER.PARALLEL",
        // Periodic/scheduled execution
        "APOC.PERIODIC.COMMIT",
        "APOC.PERIODIC.ITERATE",
        "APOC.PERIODIC.SUBMIT",
        "APOC.PERIODIC.REPEAT",
        // File system access
        "APOC.EXPORT",
        "APOC.IMPORT",
        "APOC.LOAD.CSV",
        "APOC.LOAD.JSON",
        "APOC.LOAD.XML",
        // Schema modifications
        "APOC.SCHEMA.ASSERT",
        "APOC.TRIGGER",
        // Database operations
        "APOC.SYSTEMDB",
    ];

    for proc in dangerous_apoc {
        if upper.contains(proc) {
            return Err(Error::invalid_cypher(format!(
                "Dangerous APOC procedure not allowed: {}",
                proc
            )));
        }
    }

    // Step 4: Block LOAD CSV (SSRF risk)
    if upper.contains("LOAD CSV") || upper.contains("LOAD CSV FROM") {
        return Err(Error::invalid_cypher("LOAD CSV not allowed"));
    }

    Ok(())
}

/// Strip Cypher comments to prevent bypass attacks
/// Handles both /* block */ and // line comments
fn strip_cypher_comments(cypher: &str) -> String {
    let mut result = String::with_capacity(cypher.len());
    let mut chars = cypher.chars().peekable();
    let mut in_string = false;
    let mut string_char = '"';

    while let Some(c) = chars.next() {
        // Track string literals to avoid stripping "comments" inside strings
        if !in_string && (c == '"' || c == '\'') {
            in_string = true;
            string_char = c;
            result.push(c);
            continue;
        }
        if in_string && c == string_char {
            // Handle Cypher's doubled-quote escape convention ('' or "")
            if chars.peek() == Some(&string_char) {
                // Escaped quote — push both characters and stay in string
                result.push(c);
                result.push(chars.next().unwrap());
                continue;
            }
            // End of string
            in_string = false;
            result.push(c);
            continue;
        }
        if in_string {
            result.push(c);
            continue;
        }

        // Handle block comments /* ... */
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume *
            // Skip until */
            while let Some(c2) = chars.next() {
                if c2 == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume /
                    break;
                }
            }
            result.push(' '); // Replace comment with space
            continue;
        }

        // Handle line comments // ...
        if c == '/' && chars.peek() == Some(&'/') {
            chars.next(); // consume /
            // Skip until newline
            for c2 in chars.by_ref() {
                if c2 == '\n' {
                    result.push('\n');
                    break;
                }
            }
            continue;
        }

        // Handle line comments -- ... (SQL style, sometimes used in Cypher)
        if c == '-' && chars.peek() == Some(&'-') {
            chars.next(); // consume -
            // Skip until newline
            for c2 in chars.by_ref() {
                if c2 == '\n' {
                    result.push('\n');
                    break;
                }
            }
            continue;
        }

        result.push(c);
    }

    result
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
    fn test_validate_read_only() {
        // Valid read queries
        assert!(validate_read_only("MATCH (n) RETURN n").is_ok());
        assert!(validate_read_only("MATCH (n:Entity) WHERE n.key = 'test' RETURN n").is_ok());

        // Invalid write queries
        assert!(validate_read_only("CREATE (n:Entity)").is_err());
        assert!(validate_read_only("MATCH (n) DELETE n").is_err());
        assert!(validate_read_only("MATCH (n) SET n.foo = 'bar'").is_err());
        assert!(validate_read_only("MERGE (n:Entity)").is_err());
    }

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
    // Additional Security Tests (Cypher Validation)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_read_only_case_insensitive() {
        // Case variations should all be blocked
        assert!(validate_read_only("CREATE (n)").is_err());
        assert!(validate_read_only("create (n)").is_err());
        assert!(validate_read_only("CrEaTe (n)").is_err());
        assert!(validate_read_only("DELETE n").is_err());
        assert!(validate_read_only("delete n").is_err());
        assert!(validate_read_only("DeLeTe n").is_err());
        assert!(validate_read_only("MERGE (n)").is_err());
        assert!(validate_read_only("merge (n)").is_err());
        assert!(validate_read_only("SET n.x = 1").is_err());
        assert!(validate_read_only("set n.x = 1").is_err());
    }

    #[test]
    fn test_validate_read_only_embedded_keywords() {
        // Keywords embedded in strings should NOT trigger block
        assert!(validate_read_only("MATCH (n) WHERE n.name CONTAINS 'CREATE' RETURN n").is_ok());
        assert!(validate_read_only("MATCH (n) WHERE n.name = 'DELETE ME' RETURN n").is_ok());
        // But actual keywords outside strings should block
        assert!(validate_read_only("MATCH (n) WHERE n.name = 'test' CREATE (m) RETURN m").is_err());
    }

    #[test]
    fn test_validate_read_only_special_constructs() {
        // REMOVE should be blocked
        assert!(validate_read_only("MATCH (n) REMOVE n.property").is_err());
        // DROP should be blocked
        assert!(validate_read_only("DROP INDEX my_index").is_err());
        // DETACH DELETE should be blocked
        assert!(validate_read_only("MATCH (n) DETACH DELETE n").is_err());
    }

    #[test]
    fn test_validate_read_only_semicolon_injection() {
        // Semicolon followed by write operation should be blocked
        assert!(validate_read_only("MATCH (n) RETURN n; CREATE (m)").is_err());
        assert!(validate_read_only("MATCH (n) RETURN n; DELETE n").is_err());
        // Multiple semicolons
        assert!(validate_read_only("RETURN 1; RETURN 2; CREATE (n)").is_err());
    }

    #[test]
    fn test_validate_read_only_whitespace_variations() {
        // Extra whitespace should not bypass
        assert!(validate_read_only("MATCH (n)  CREATE  (m)").is_err());
        assert!(validate_read_only("MATCH (n)\nCREATE (m)").is_err());
        // Tab might not be treated as word boundary - test actual behavior
        // The important thing is that obvious attacks are blocked
        assert!(validate_read_only("MATCH (n) \t DELETE n").is_err());
    }

    #[test]
    fn test_validate_read_only_valid_queries() {
        // Valid read queries with various clauses
        assert!(validate_read_only("MATCH (n) RETURN n").is_ok());
        assert!(validate_read_only("MATCH (n)-[r]->(m) RETURN n, r, m").is_ok());
        assert!(validate_read_only("MATCH (n) WHERE n.key = 'test' RETURN n").is_ok());
        assert!(validate_read_only("MATCH (n) WITH n ORDER BY n.name RETURN n").is_ok());
        assert!(validate_read_only("MATCH (n) OPTIONAL MATCH (n)-[r]->(m) RETURN n, r, m").is_ok());
        assert!(validate_read_only("MATCH (n) UNWIND [1,2,3] AS x RETURN x").is_ok());
        assert!(validate_read_only("CALL db.schema.visualization()").is_ok());
        assert!(validate_read_only("RETURN 1 AS num, 'hello' AS str").is_ok());
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Critical Security Tests (Expert Agent Findings)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_apoc_dangerous_procedures_blocked() {
        // Dynamic Cypher execution
        assert!(
            validate_read_only(
                "CALL apoc.cypher.run('CREATE (n:Evil)', {}) YIELD value RETURN value"
            )
            .is_err()
        );
        assert!(
            validate_read_only("CALL apoc.cypher.doIt('DELETE (n)', {}) YIELD value RETURN value")
                .is_err()
        );

        // Periodic execution
        assert!(
            validate_read_only("CALL apoc.periodic.commit('CREATE (n)', {}) YIELD value").is_err()
        );
        assert!(
            validate_read_only("CALL apoc.periodic.iterate('MATCH (n)', 'DELETE n', {})").is_err()
        );

        // File system access
        assert!(validate_read_only("CALL apoc.export.csv.all('/tmp/data.csv', {})").is_err());
        assert!(validate_read_only("CALL apoc.load.json('http://evil.com/') YIELD value").is_err());

        // Safe APOC procedures should be allowed
        assert!(validate_read_only("CALL apoc.meta.data() YIELD label RETURN label").is_ok());
        assert!(validate_read_only("CALL apoc.text.capitalize('hello')").is_ok());
    }

    #[test]
    fn test_subquery_write_bypass_blocked() {
        // Subquery with write operations
        assert!(validate_read_only("MATCH (n) CALL { CREATE (m:Evil) } RETURN n").is_err());
        assert!(validate_read_only("MATCH (n) CALL { DELETE n } RETURN n").is_err());
        assert!(validate_read_only("MATCH (n) CALL { MERGE (m) } RETURN n").is_err());
        assert!(validate_read_only("MATCH (n) CALL { WITH n SET n.x = 1 } RETURN n").is_err());

        // Read-only subqueries should be allowed
        assert!(validate_read_only("MATCH (n) CALL { MATCH (m) RETURN m } RETURN n, m").is_ok());
    }

    #[test]
    fn test_foreach_write_bypass_blocked() {
        // FOREACH with write operations
        assert!(validate_read_only("MATCH (n) FOREACH (x IN [1] | CREATE (m)) RETURN n").is_err());
        assert!(validate_read_only("MATCH (n) FOREACH (x IN [1] | DELETE n) RETURN n").is_err());
        assert!(validate_read_only("MATCH (n) FOREACH (x IN [1] | SET n.x = 1) RETURN n").is_err());
        assert!(validate_read_only("MATCH (n) FOREACH (x IN [1]|MERGE (m)) RETURN n").is_err());

        // FOREACH without write is uncommon but valid for reading
        assert!(validate_read_only("MATCH (n) RETURN n").is_ok());
    }

    #[test]
    fn test_comment_bypass_blocked() {
        // Block comments should be stripped
        assert!(validate_read_only("MATCH (n) /* comment */ CREATE (m)").is_err());
        assert!(validate_read_only("/* comment */ CREATE (n)").is_err());
        // Note: "C/**/REATE (n)" becomes "C REATE (n)" after stripping, which is NOT valid Cypher
        // This is correct behavior - Neo4j would reject "C REATE" anyway
        assert!(validate_read_only("C/**/REATE (n)").is_ok()); // "C REATE" is not a keyword
        assert!(validate_read_only("MATCH (n) /*\n*/ DELETE n").is_err());

        // Line comments should be stripped
        assert!(validate_read_only("MATCH (n) // comment\nCREATE (m)").is_err());
        assert!(validate_read_only("MATCH (n) -- comment\nDELETE n").is_err());

        // Comments in strings should be preserved
        assert!(
            validate_read_only("MATCH (n) WHERE n.name = '/* not a comment */' RETURN n").is_ok()
        );
    }

    #[test]
    fn test_load_csv_blocked() {
        // LOAD CSV should be blocked (SSRF risk)
        assert!(
            validate_read_only("LOAD CSV FROM 'http://example.com' AS line RETURN line").is_err()
        );
        assert!(
            validate_read_only("LOAD CSV FROM 'file:///etc/passwd' AS line RETURN line").is_err()
        );
        assert!(
            validate_read_only("LOAD CSV WITH HEADERS FROM 'http://example.com' AS row RETURN row")
                .is_err()
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Write-Safe Validation Tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_write_safe_allows_merge() {
        // MERGE should be allowed for write operations
        assert!(validate_write_safe("MERGE (n:Entity {key: $key})").is_ok());
        assert!(validate_write_safe("MERGE (n)-[:HAS_NATIVE]->(m)").is_ok());
    }

    #[test]
    fn test_validate_write_safe_allows_set() {
        // SET should be allowed for write operations
        assert!(validate_write_safe("MATCH (n) SET n.updated_at = timestamp()").is_ok());
        assert!(validate_write_safe("MERGE (n) ON CREATE SET n.created = true").is_ok());
    }

    #[test]
    fn test_validate_write_safe_allows_create() {
        // CREATE should be allowed for write operations
        assert!(validate_write_safe("CREATE (n:Entity)").is_ok());
        assert!(validate_write_safe("CREATE (n)-[:TARGETS]->(m)").is_ok());
    }

    #[test]
    fn test_validate_write_safe_blocks_drop() {
        // DROP should still be blocked (schema modification)
        assert!(validate_write_safe("DROP INDEX my_index").is_err());
        assert!(validate_write_safe("DROP CONSTRAINT my_constraint").is_err());
    }

    #[test]
    fn test_validate_write_safe_blocks_dangerous_apoc() {
        // Dangerous APOC should still be blocked
        assert!(validate_write_safe("CALL apoc.cypher.run('CREATE (n)', {})").is_err());
        assert!(validate_write_safe("CALL apoc.periodic.iterate('MATCH', 'DELETE', {})").is_err());
        assert!(validate_write_safe("CALL apoc.export.csv.all('/tmp/data.csv', {})").is_err());
    }

    #[test]
    fn test_validate_write_safe_blocks_load_csv() {
        // LOAD CSV should still be blocked (SSRF risk)
        assert!(validate_write_safe("LOAD CSV FROM 'http://example.com' AS row").is_err());
    }

    #[test]
    fn test_strip_cypher_comments() {
        // Block comments
        assert_eq!(
            strip_cypher_comments("SELECT /* comment */ FROM"),
            "SELECT   FROM"
        );

        // Line comments
        assert_eq!(
            strip_cypher_comments("SELECT // comment\nFROM"),
            "SELECT \nFROM"
        );

        // SQL-style comments
        assert_eq!(
            strip_cypher_comments("SELECT -- comment\nFROM"),
            "SELECT \nFROM"
        );

        // Comments in strings should be preserved
        assert_eq!(
            strip_cypher_comments("WHERE n.name = '/* not comment */'"),
            "WHERE n.name = '/* not comment */'"
        );

        // Multiple comments
        assert_eq!(strip_cypher_comments("A /* 1 */ B /* 2 */ C"), "A   B   C");
    }
}
