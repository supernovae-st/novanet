//! Neo4j connection pool (neo4rs + Arc).
//!
//! `Graph::new()` creates an internal connection pool. Clone `Db` freely
//! across tasks — it's Arc-based internally.

use neo4rs::{Graph, query};
use std::sync::Arc;

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
                }
                crate::cypher::ParamValue::Int(n) => {
                    q = q.param(name.as_str(), *n);
                }
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
