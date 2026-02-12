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
fn validate_read_only(cypher: &str) -> Result<()> {
    let upper = cypher.to_uppercase();
    let forbidden = [
        "CREATE", "DELETE", "MERGE", "SET", "REMOVE", "DROP", "DETACH",
    ];

    for keyword in forbidden {
        // Use word boundary check to avoid false positives
        if upper.contains(&format!(" {} ", keyword))
            || upper.starts_with(&format!("{} ", keyword))
            || upper.contains(&format!("\n{}", keyword))
        {
            return Err(Error::write_not_allowed(keyword));
        }
    }

    Ok(())
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
}
