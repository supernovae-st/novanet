//! NovaNet error types (thiserror for structured matching).
//!
//! Includes error hints for actionable suggestions.

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

/// Error hints provide actionable suggestions for fixing errors.
pub trait ErrorHint {
    /// Get a hint for how to fix this error, if available.
    fn hint(&self) -> Option<&'static str>;
}

impl ErrorHint for NovaNetError {
    fn hint(&self) -> Option<&'static str> {
        match self {
            NovaNetError::Connection { source, .. } => {
                let msg = source.to_string().to_lowercase();
                if msg.contains("connection refused") {
                    Some("Is Neo4j running? Try: pnpm infra:up")
                } else if msg.contains("authentication") || msg.contains("unauthorized") {
                    Some("Check NEO4J_PASSWORD env var or run: novanet init")
                } else if msg.contains("timeout") {
                    Some("Neo4j may be starting up. Wait a moment and retry.")
                } else {
                    Some("Check Neo4j status: novanet doctor")
                }
            },
            NovaNetError::Query { query, source } => {
                let msg = source.to_string().to_lowercase();
                if msg.contains("syntax") {
                    Some("Cypher syntax error. Check your query.")
                } else if msg.contains("unknown function") {
                    Some("Unknown function. Is APOC installed?")
                } else if query.contains("DETACH DELETE") {
                    Some("DETACH DELETE requires db:write access.")
                } else {
                    None
                }
            },
            NovaNetError::UnknownClass(label) => {
                if label
                    .chars()
                    .next()
                    .map(|c| c.is_lowercase())
                    .unwrap_or(false)
                {
                    Some("Class names use PascalCase (e.g., 'Page' not 'page').")
                } else {
                    Some("Run 'novanet schema validate' to check class definitions.")
                }
            },
            NovaNetError::SchemaIntegrity(_) => {
                Some("Run 'novanet schema generate' to regenerate artifacts.")
            },
            NovaNetError::Schema { .. } => {
                Some("Check YAML syntax. Run 'novanet schema validate --verbose' for details.")
            },
            NovaNetError::Validation(msg) => {
                if msg.contains("password") {
                    Some("Set NEO4J_PASSWORD env var or run: novanet init")
                } else if msg.contains("config") {
                    Some("Run 'novanet init' to create configuration.")
                } else {
                    None
                }
            },
            NovaNetError::Generator { generator, .. } => Some(match generator.as_str() {
                "mermaid" => "Check view definitions in models/views/.",
                "cypher" => "Validate YAML with 'novanet schema validate'.",
                _ => "Check YAML definitions and try again.",
            }),
            NovaNetError::Io(err) => {
                let kind = err.kind();
                match kind {
                    std::io::ErrorKind::NotFound => {
                        Some("File or directory not found. Check --root path.")
                    },
                    std::io::ErrorKind::PermissionDenied => {
                        Some("Permission denied. Check file permissions.")
                    },
                    _ => None,
                }
            },
        }
    }
}

/// Format an error with its hint for display.
pub fn format_error_with_hint(err: &NovaNetError) -> String {
    let base = err.to_string();
    match err.hint() {
        Some(hint) => format!("{base}\n  Hint: {hint}"),
        None => base,
    }
}

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

    // Error hint tests

    #[test]
    fn hint_unknown_class_lowercase() {
        let err = NovaNetError::UnknownClass("page".to_string());
        assert_eq!(
            err.hint(),
            Some("Class names use PascalCase (e.g., 'Page' not 'page').")
        );
    }

    #[test]
    fn hint_unknown_class_pascalcase() {
        let err = NovaNetError::UnknownClass("FooBar".to_string());
        assert_eq!(
            err.hint(),
            Some("Run 'novanet schema validate' to check class definitions.")
        );
    }

    #[test]
    fn hint_schema_integrity() {
        let err = NovaNetError::SchemaIntegrity("missing node".to_string());
        assert_eq!(
            err.hint(),
            Some("Run 'novanet schema generate' to regenerate artifacts.")
        );
    }

    #[test]
    fn hint_validation_password() {
        let err = NovaNetError::Validation("No password provided".to_string());
        assert_eq!(
            err.hint(),
            Some("Set NEO4J_PASSWORD env var or run: novanet init")
        );
    }

    #[test]
    fn hint_validation_no_match() {
        let err = NovaNetError::Validation("something else".to_string());
        assert_eq!(err.hint(), None);
    }

    #[test]
    fn hint_generator_mermaid() {
        let err = NovaNetError::Generator {
            generator: "mermaid".to_string(),
            detail: "error".to_string(),
        };
        assert_eq!(err.hint(), Some("Check view definitions in models/views/."));
    }

    #[test]
    fn hint_io_not_found() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err: NovaNetError = io_err.into();
        assert_eq!(
            err.hint(),
            Some("File or directory not found. Check --root path.")
        );
    }

    #[test]
    fn hint_io_permission_denied() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err: NovaNetError = io_err.into();
        assert_eq!(
            err.hint(),
            Some("Permission denied. Check file permissions.")
        );
    }

    #[test]
    fn format_error_with_hint_includes_hint() {
        let err = NovaNetError::SchemaIntegrity("missing node".to_string());
        let formatted = format_error_with_hint(&err);
        assert!(formatted.contains("schema-graph integrity"));
        assert!(formatted.contains("Hint:"));
        assert!(formatted.contains("novanet schema generate"));
    }

    #[test]
    fn format_error_without_hint() {
        let err = NovaNetError::Validation("something else".to_string());
        let formatted = format_error_with_hint(&err);
        assert!(!formatted.contains("Hint:"));
    }
}
