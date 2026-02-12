//! Error types for NovaNet MCP Server
//!
//! Uses thiserror for ergonomic error handling.

use rmcp::ErrorData as McpError;
use rmcp::model::ErrorCode;
use std::borrow::Cow;
use thiserror::Error;

// JSON-RPC 2.0 error codes
const INVALID_PARAMS: i32 = -32602;
const INTERNAL_ERROR: i32 = -32603;
const RESOURCE_NOT_FOUND: i32 = -32001; // Custom code for not found

/// Result type alias for NovaNet MCP operations
pub type Result<T> = std::result::Result<T, Error>;

/// NovaNet MCP Server errors
#[derive(Error, Debug)]
pub enum Error {
    /// Neo4j connection error
    #[error("Neo4j connection failed to {uri}: {source}")]
    Connection {
        uri: String,
        #[source]
        source: neo4rs::Error,
    },

    /// Neo4j query execution error
    #[error("Query execution failed: {query}\n{source}")]
    Query {
        query: String,
        #[source]
        source: neo4rs::Error,
    },

    /// Entity not found
    #[error("Entity not found: {key}")]
    NotFound { key: String },

    /// Token budget exceeded
    #[error("Token budget exceeded: {used}/{budget}")]
    TokenBudgetExceeded { used: usize, budget: usize },

    /// Write operation attempted on read-only connection
    #[error("Write operations not allowed: {operation}")]
    WriteNotAllowed { operation: String },

    /// Invalid Cypher query (contains forbidden keywords)
    #[error("Invalid Cypher query: {reason}")]
    InvalidCypher { reason: String },

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Connection pool error
    #[error("Connection pool error: {0}")]
    Pool(String),

    /// MCP protocol error
    #[error("MCP protocol error: {0}")]
    Mcp(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Create a connection error
    pub fn connection(uri: impl Into<String>, source: neo4rs::Error) -> Self {
        Self::Connection {
            uri: uri.into(),
            source,
        }
    }

    /// Create a query error
    pub fn query(query: impl Into<String>, source: neo4rs::Error) -> Self {
        Self::Query {
            query: query.into(),
            source,
        }
    }

    /// Create a not found error
    pub fn not_found(key: impl Into<String>) -> Self {
        Self::NotFound { key: key.into() }
    }

    /// Create a token budget exceeded error
    pub fn token_budget_exceeded(used: usize, budget: usize) -> Self {
        Self::TokenBudgetExceeded { used, budget }
    }

    /// Create a write not allowed error
    pub fn write_not_allowed(operation: impl Into<String>) -> Self {
        Self::WriteNotAllowed {
            operation: operation.into(),
        }
    }

    /// Create an invalid cypher error
    pub fn invalid_cypher(reason: impl Into<String>) -> Self {
        Self::InvalidCypher {
            reason: reason.into(),
        }
    }
}

// Convert to MCP error format
impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        let code = match &err {
            Error::NotFound { .. } => RESOURCE_NOT_FOUND,
            Error::InvalidCypher { .. } => INVALID_PARAMS,
            Error::WriteNotAllowed { .. } => INVALID_PARAMS,
            Error::TokenBudgetExceeded { .. } => INVALID_PARAMS,
            _ => INTERNAL_ERROR,
        };
        McpError {
            code: ErrorCode(code),
            message: Cow::Owned(err.to_string()),
            data: None,
        }
    }
}
