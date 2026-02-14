//! NovaNet error types (thiserror for structured matching).

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NovaNetError {
    #[error("Neo4j connection failed: {uri}")]
    Connection {
        uri: String,
        #[source]
        source: neo4rs::Error,
    },

    #[error("query failed: {query}")]
    Query {
        query: String,
        #[source]
        source: neo4rs::Error,
    },

    // Removed: Neo4j(#[from] neo4rs::Error) - use Connection or Query instead
    // Removed: Neo4jDe(#[from] neo4rs::DeError) - unused, neo4rs handles internally
    #[error("no Class found for label '{0}'")]
    UnknownClass(String),

    #[error("schema-graph integrity: {0}")]
    SchemaIntegrity(String),

    #[error("YAML schema error in {path}")]
    Schema {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("generator '{generator}' failed: {detail}")]
    Generator { generator: String, detail: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NovaNetError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_class_display() {
        let err = NovaNetError::UnknownClass("FooBar".to_string());
        assert_eq!(err.to_string(), "no Class found for label 'FooBar'");
    }

    #[test]
    fn schema_integrity_display() {
        let err = NovaNetError::SchemaIntegrity("missing Realm node".to_string());
        assert_eq!(
            err.to_string(),
            "schema-graph integrity: missing Realm node"
        );
    }

    #[test]
    fn validation_display() {
        let err = NovaNetError::Validation("empty key".to_string());
        assert_eq!(err.to_string(), "validation failed: empty key");
    }

    #[test]
    fn generator_display() {
        let err = NovaNetError::Generator {
            generator: "mermaid".to_string(),
            detail: "missing input".to_string(),
        };
        assert_eq!(err.to_string(), "generator 'mermaid' failed: missing input");
    }

    #[test]
    fn io_error_transparent() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err: NovaNetError = io_err.into();
        assert!(err.to_string().contains("file missing"));
    }
}
