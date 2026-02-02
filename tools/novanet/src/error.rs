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

    #[error("no Kind found for label '{0}'")]
    UnknownKind(String),

    #[error("meta-graph integrity: {0}")]
    MetaIntegrity(String),

    #[error("YAML schema error in {path}")]
    Schema {
        path: String,
        #[source]
        source: serde_yml::Error,
    },

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("generator failed: {generator}")]
    Generator { generator: String, detail: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NovaNetError>;
