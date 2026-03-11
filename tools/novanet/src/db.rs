//! Neo4j connection pool (neo4rs + Arc).
//!
//! `Graph::new()` creates an internal connection pool. Clone `Db` freely
//! across tasks — it's Arc-based internally.
//!
//! # RowExt Trait
//!
//! Provides ergonomic row field extraction with automatic defaults:
//! ```ignore
//! use crate::db::RowExt;
//! let key = row.str("key");        // String, defaults to ""
//! let count = row.int("count");    // i64, defaults to 0
//! let active = row.bool("active"); // bool, defaults to false
//! let tags = row.vec_str("tags");  // Vec<String>, defaults to []
//! ```

use neo4rs::{Graph, Row, query};
use std::sync::Arc;

// =============================================================================
// ROW EXTENSION TRAIT
// =============================================================================

/// Extension trait for ergonomic Neo4j row field extraction.
///
/// Reduces boilerplate from `row.get::<String>("field").unwrap_or_default()`
/// to simply `row.str("field")`.
pub trait RowExt {
    /// Extract a String field, defaulting to empty string.
    fn str(&self, key: &str) -> String;

    /// Extract an i64 field, defaulting to 0.
    fn int(&self, key: &str) -> i64;

    /// Extract a bool field, defaulting to false.
    fn bool(&self, key: &str) -> bool;

    /// Extract a Vec<String> field, defaulting to empty vec.
    fn vec_str(&self, key: &str) -> Vec<String>;

    /// Extract an Option<String> field (None if missing or empty).
    fn opt_str(&self, key: &str) -> Option<String>;

    /// Extract an Option<i64> field (None if missing).
    fn opt_int(&self, key: &str) -> Option<i64>;
}

impl RowExt for Row {
    fn str(&self, key: &str) -> String {
        self.get::<String>(key).unwrap_or_default()
    }

    fn int(&self, key: &str) -> i64 {
        self.get::<i64>(key).unwrap_or(0)
    }

    fn bool(&self, key: &str) -> bool {
        self.get::<bool>(key).unwrap_or(false)
    }

    fn vec_str(&self, key: &str) -> Vec<String> {
        self.get::<Vec<String>>(key).unwrap_or_default()
    }

    fn opt_str(&self, key: &str) -> Option<String> {
        self.get::<String>(key).ok().filter(|s| !s.is_empty())
    }

    fn opt_int(&self, key: &str) -> Option<i64> {
        self.get::<i64>(key).ok()
    }
}

// =============================================================================
// DATABASE CONNECTION
// =============================================================================

#[derive(Clone)]
pub struct Db {
    graph: Arc<Graph>,
}

impl Db {
    pub async fn connect(uri: &str, user: &str, pass: &str) -> crate::Result<Self> {
        let graph =
            Graph::new(uri, user, pass)
                .await
                .map_err(|e| crate::NovaNetError::Connection {
                    uri: uri.to_string(),
                    source: e,
                })?;
        Ok(Self {
            graph: Arc::new(graph),
        })
    }

    /// Execute a read query, return rows.
    pub async fn execute(&self, cypher: &str) -> crate::Result<Vec<neo4rs::Row>> {
        let q = query(cypher);
        let mut result = self
            .graph
            .execute(q)
            .await
            .map_err(|e| crate::NovaNetError::Query {
                query: cypher.to_string(),
                source: e,
            })?;
        let mut rows = Vec::new();
        while let Some(row) = result
            .next()
            .await
            .map_err(|e| crate::NovaNetError::Query {
                query: cypher.to_string(),
                source: e,
            })?
        {
            rows.push(row);
        }
        Ok(rows)
    }

    /// Execute a read query with parameters.
    pub async fn execute_with_params(
        &self,
        cypher: &str,
        params: impl IntoIterator<Item = (&str, impl Into<neo4rs::BoltType>)>,
    ) -> crate::Result<Vec<neo4rs::Row>> {
        let mut q = query(cypher);
        for (k, v) in params {
            q = q.param(k, v);
        }
        let mut result = self
            .graph
            .execute(q)
            .await
            .map_err(|e| crate::NovaNetError::Query {
                query: cypher.to_string(),
                source: e,
            })?;
        let mut rows = Vec::new();
        while let Some(row) = result
            .next()
            .await
            .map_err(|e| crate::NovaNetError::Query {
                query: cypher.to_string(),
                source: e,
            })?
        {
            rows.push(row);
        }
        Ok(rows)
    }

    /// Execute a `CypherStatement` with mixed param types (StringList, Int).
    pub async fn execute_statement(
        &self,
        stmt: &crate::cypher::CypherStatement,
    ) -> crate::Result<Vec<neo4rs::Row>> {
        let mut q = query(&stmt.cypher);
        for (name, value) in &stmt.params {
            match value {
                crate::cypher::ParamValue::StringList(list) => {
                    let bolt_list: Vec<neo4rs::BoltType> = list
                        .iter()
                        .map(|s| neo4rs::BoltType::from(s.as_str()))
                        .collect();
                    q = q.param(name.as_str(), bolt_list);
                },
                crate::cypher::ParamValue::Int(n) => {
                    q = q.param(name.as_str(), *n);
                },
            }
        }
        let mut result = self
            .graph
            .execute(q)
            .await
            .map_err(|e| crate::NovaNetError::Query {
                query: stmt.cypher.clone(),
                source: e,
            })?;
        let mut rows = Vec::new();
        while let Some(row) = result
            .next()
            .await
            .map_err(|e| crate::NovaNetError::Query {
                query: stmt.cypher.clone(),
                source: e,
            })?
        {
            rows.push(row);
        }
        Ok(rows)
    }

    /// Access the underlying Graph for advanced operations.
    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}
