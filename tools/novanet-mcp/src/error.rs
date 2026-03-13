//! Error types for NovaNet MCP Server
//!
//! Uses thiserror for ergonomic error handling.

use crate::hints;
use rmcp::ErrorData as McpError;
use rmcp::model::ErrorCode;
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

    /// Invalid tool name in batch operation
    #[error("Invalid tool: {0}")]
    InvalidTool(String),

    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Feature not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Trait does not allow writes
    #[error(
        "Class '{class}' has trait '{trait_type}' which is not writable. Only authored/imported/generated/retrieved allow writes."
    )]
    TraitNotWritable { class: String, trait_type: String },

    /// Slug is locked after deployment
    #[error(
        "Slug is locked on '{key}'. Current slug: '{current_slug}'. Create a redirect instead of modifying."
    )]
    SlugLocked { key: String, current_slug: String },

    /// Singleton property violation (e.g., is_slug_source)
    #[error(
        "Singleton violation: Only one arc can have '{property}' = true for target '{target_key}'."
    )]
    SingletonViolation {
        property: String,
        target_key: String,
    },

    /// Schema class not found (generic)
    #[error("Schema class not found: '{class}'. Use novanet_introspect to list available classes.")]
    SchemaNotFound { class: String },

    /// NodeClass not found in schema
    #[error(
        "NodeClass '{name}' not found. Use novanet_introspect(target='classes') to list all 60 NodeClasses. Common classes: Entity, Page, Block, Locale."
    )]
    NodeClassNotFound { name: String },

    /// ArcClass not found in schema
    #[error(
        "ArcClass '{name}' not found. Use novanet_introspect(target='arcs') to list all ArcClasses. Common arcs: HAS_NATIVE, FOR_LOCALE, BELONGS_TO."
    )]
    ArcClassNotFound { name: String },

    /// Missing required property
    #[error("Missing required property '{property}' for class '{class}'.")]
    MissingRequiredProperty { class: String, property: String },

    /// Arc endpoints not found
    #[error("Arc endpoint not found: {endpoint_type} '{key}' does not exist.")]
    ArcEndpointNotFound { endpoint_type: String, key: String },
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

    /// Create an invalid tool error
    pub fn invalid_tool(tool: impl Into<String>) -> Self {
        Self::InvalidTool(tool.into())
    }

    /// Create a trait not writable error
    pub fn trait_not_writable(class: impl Into<String>, trait_type: impl Into<String>) -> Self {
        Self::TraitNotWritable {
            class: class.into(),
            trait_type: trait_type.into(),
        }
    }

    /// Create a slug locked error
    pub fn slug_locked(key: impl Into<String>, current_slug: impl Into<String>) -> Self {
        Self::SlugLocked {
            key: key.into(),
            current_slug: current_slug.into(),
        }
    }

    /// Create a singleton violation error
    pub fn singleton_violation(property: impl Into<String>, target_key: impl Into<String>) -> Self {
        Self::SingletonViolation {
            property: property.into(),
            target_key: target_key.into(),
        }
    }

    /// Create a schema not found error
    pub fn schema_not_found(class: impl Into<String>) -> Self {
        Self::SchemaNotFound {
            class: class.into(),
        }
    }

    /// Create a NodeClass not found error
    pub fn node_class_not_found(name: impl Into<String>) -> Self {
        Self::NodeClassNotFound { name: name.into() }
    }

    /// Create an ArcClass not found error
    pub fn arc_class_not_found(name: impl Into<String>) -> Self {
        Self::ArcClassNotFound { name: name.into() }
    }

    /// Create a missing required property error
    pub fn missing_required_property(
        class: impl Into<String>,
        property: impl Into<String>,
    ) -> Self {
        Self::MissingRequiredProperty {
            class: class.into(),
            property: property.into(),
        }
    }

    /// Create an arc endpoint not found error
    pub fn arc_endpoint_not_found(
        endpoint_type: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        Self::ArcEndpointNotFound {
            endpoint_type: endpoint_type.into(),
            key: key.into(),
        }
    }

    /// Get error message with actionable hint
    pub fn with_hint(&self) -> String {
        hints::with_hint(&self.to_string())
    }
}

// JSON-RPC 2.0 error code for not implemented
const NOT_IMPLEMENTED: i32 = -32000; // Custom code for not implemented

// Convert to MCP error format
impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        let code = match &err {
            Error::NotFound { .. } => RESOURCE_NOT_FOUND,
            Error::InvalidCypher { .. } => INVALID_PARAMS,
            Error::WriteNotAllowed { .. } => INVALID_PARAMS,
            Error::TokenBudgetExceeded { .. } => INVALID_PARAMS,
            Error::InvalidTool(_) => INVALID_PARAMS,
            Error::InvalidParams(_) => INVALID_PARAMS,
            Error::NotImplemented(_) => NOT_IMPLEMENTED,
            // Write-specific error mappings
            Error::TraitNotWritable { .. } => INVALID_PARAMS,
            Error::SlugLocked { .. } => INVALID_PARAMS,
            Error::SingletonViolation { .. } => INVALID_PARAMS,
            Error::SchemaNotFound { .. } => RESOURCE_NOT_FOUND,
            Error::NodeClassNotFound { .. } => RESOURCE_NOT_FOUND,
            Error::ArcClassNotFound { .. } => RESOURCE_NOT_FOUND,
            Error::MissingRequiredProperty { .. } => INVALID_PARAMS,
            Error::ArcEndpointNotFound { .. } => RESOURCE_NOT_FOUND,
            _ => INTERNAL_ERROR,
        };
        // rmcp 1.x: Use new() constructor instead of struct literal
        McpError::new(ErrorCode(code), err.with_hint(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_not_writable_error() {
        let err = Error::trait_not_writable("Entity", "defined");
        assert!(err.to_string().contains("Entity"));
        assert!(err.to_string().contains("defined"));
    }

    #[test]
    fn test_slug_locked_error() {
        let err = Error::slug_locked("block:head-seo-meta@fr-FR", "qr-code");
        assert!(err.to_string().contains("slug_locked") || err.to_string().contains("locked"));
        assert!(err.to_string().contains("qr-code"));
    }

    #[test]
    fn test_singleton_violation_error() {
        let err = Error::singleton_violation("is_slug_source", "entity-native:qr-code@fr-FR");
        assert!(err.to_string().contains("is_slug_source"));
    }

    #[test]
    fn test_schema_not_found_error() {
        let err = Error::schema_not_found("UnknownClass");
        assert!(err.to_string().contains("UnknownClass"));
    }

    #[test]
    fn test_node_class_not_found_error() {
        let err = Error::node_class_not_found("Entity");
        let msg = err.to_string();
        assert!(msg.contains("NodeClass"));
        assert!(msg.contains("Entity"));
        assert!(msg.contains("novanet_introspect"));
    }

    #[test]
    fn test_arc_class_not_found_error() {
        let err = Error::arc_class_not_found("HAS_NATIVE");
        let msg = err.to_string();
        assert!(msg.contains("ArcClass"));
        assert!(msg.contains("HAS_NATIVE"));
        assert!(msg.contains("novanet_introspect"));
    }

    #[test]
    fn test_missing_required_property_error() {
        let err = Error::missing_required_property("SEOKeyword", "keyword");
        assert!(err.to_string().contains("SEOKeyword"));
        assert!(err.to_string().contains("keyword"));
    }
}
