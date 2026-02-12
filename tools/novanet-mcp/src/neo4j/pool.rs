//! Neo4j Connection Pool
//!
//! Uses neo4rs with Arc for connection sharing.
//! Pattern from tools/novanet/src/db.rs but simplified for MCP.

use crate::error::{Error, Result};
use neo4rs::{BoltType, ConfigBuilder, Graph, Query};
use std::sync::Arc;

/// Neo4j connection pool wrapper
#[derive(Clone)]
pub struct Neo4jPool {
    graph: Arc<Graph>,
}

impl Neo4jPool {
    /// Create a new pool with the given configuration
    pub async fn new(uri: &str, user: &str, password: &str, pool_size: usize) -> Result<Self> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .max_connections(pool_size)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

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

    /// Test connection health
    pub async fn health_check(&self) -> Result<bool> {
        let result = self.execute_query("RETURN 1 AS health", None).await?;
        Ok(!result.is_empty())
    }
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
    let write_keywords = [
        "CREATE", "DELETE", "MERGE", "SET", "REMOVE", "DROP", "DETACH",
    ];

    for keyword in write_keywords {
        // Check for keyword with various boundaries (space, newline, tab, parenthesis, brace)
        let patterns = [
            format!(" {} ", keyword),
            format!(" {}\n", keyword),
            format!(" {}\t", keyword),
            format!(" {}(", keyword),
            format!(" {}{}", keyword, "{"),
            format!("\n{} ", keyword),
            format!("\n{}(", keyword),
            format!("\t{} ", keyword),
            format!("({}", keyword),    // Inside parens (subquery)
            format!("{}{}", "{", keyword), // Inside braces
            format!("|{}", keyword),    // FOREACH ... | CREATE
            format!("| {} ", keyword),  // FOREACH ... | CREATE
        ];

        for pattern in patterns {
            if upper.contains(&pattern) {
                return Err(Error::write_not_allowed(keyword));
            }
        }

        // Also check if query starts with keyword
        if upper.trim_start().starts_with(&format!("{} ", keyword))
            || upper.trim_start().starts_with(&format!("{}(", keyword))
        {
            return Err(Error::write_not_allowed(keyword));
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
    if upper.contains("FOREACH") {
        for keyword in write_keywords {
            // Check for FOREACH ... | WRITE pattern
            if upper.contains("FOREACH") && upper.contains(&format!("|{}", keyword))
                || upper.contains(&format!("| {}", keyword))
            {
                return Err(Error::write_not_allowed(format!(
                    "FOREACH with {}",
                    keyword
                )));
            }
        }
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
            // Check for escaped quote
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

        let negative_float = json_to_bolt_type(serde_json::json!(-3.14159));
        assert!(matches!(negative_float, BoltType::Float(_)));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Critical Security Tests (Expert Agent Findings)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_apoc_dangerous_procedures_blocked() {
        // Dynamic Cypher execution
        assert!(validate_read_only(
            "CALL apoc.cypher.run('CREATE (n:Evil)', {}) YIELD value RETURN value"
        )
        .is_err());
        assert!(validate_read_only(
            "CALL apoc.cypher.doIt('DELETE (n)', {}) YIELD value RETURN value"
        )
        .is_err());

        // Periodic execution
        assert!(validate_read_only(
            "CALL apoc.periodic.commit('CREATE (n)', {}) YIELD value"
        )
        .is_err());
        assert!(validate_read_only(
            "CALL apoc.periodic.iterate('MATCH (n)', 'DELETE n', {})"
        )
        .is_err());

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
        assert!(validate_read_only("LOAD CSV FROM 'http://example.com' AS line RETURN line").is_err());
        assert!(validate_read_only("LOAD CSV FROM 'file:///etc/passwd' AS line RETURN line").is_err());
        assert!(
            validate_read_only("LOAD CSV WITH HEADERS FROM 'http://example.com' AS row RETURN row")
                .is_err()
        );
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
        assert_eq!(
            strip_cypher_comments("A /* 1 */ B /* 2 */ C"),
            "A   B   C"
        );
    }
}
