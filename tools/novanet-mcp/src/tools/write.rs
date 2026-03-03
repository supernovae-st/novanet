//! novanet_write tool
//!
//! Intelligent data writes to Neo4j with schema validation.
//! Single tool with 3 operations: upsert_node, create_arc, update_props.

use crate::error::{Error, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Write operation type
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WriteOperation {
    /// Create or update a node (MERGE pattern)
    UpsertNode,
    /// Create an arc between nodes
    CreateArc,
    /// Update specific properties on existing node
    UpdateProps,
}

/// Parameters for novanet_write tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WriteParams {
    /// Operation type
    pub operation: WriteOperation,

    /// Node class name (for upsert_node, update_props)
    #[serde(default)]
    pub class: Option<String>,

    /// Arc class name (for create_arc)
    #[serde(default)]
    pub arc_class: Option<String>,

    /// Node key (for upsert_node, update_props)
    #[serde(default)]
    pub key: Option<String>,

    /// Source node key (for create_arc)
    #[serde(default)]
    pub from_key: Option<String>,

    /// Target node key (for create_arc)
    #[serde(default)]
    pub to_key: Option<String>,

    /// Properties to write
    #[serde(default)]
    pub properties: Option<serde_json::Map<String, Value>>,

    /// Locale for auto-arc creation (optional)
    #[serde(default)]
    pub locale: Option<String>,
}

/// Result from novanet_write tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct WriteResult {
    /// Whether the operation succeeded
    pub success: bool,

    /// Operation performed
    pub operation: String,

    /// Affected node/arc key
    pub key: String,

    /// Whether a new node was created (vs updated)
    pub created: bool,

    /// Properties that were updated (empty if created)
    pub updated_properties: Vec<String>,

    /// Auto-arcs that were created
    pub auto_arcs_created: Vec<String>,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,

    /// Cache keys invalidated
    pub cache_invalidated: Vec<String>,
}

/// Validate write operation has required params
fn validate_params(params: &WriteParams) -> Result<()> {
    match params.operation {
        WriteOperation::UpsertNode => {
            if params.class.is_none() {
                return Err(Error::InvalidParams("upsert_node requires 'class'".into()));
            }
            if params.key.is_none() {
                return Err(Error::InvalidParams("upsert_node requires 'key'".into()));
            }
        }
        WriteOperation::CreateArc => {
            if params.arc_class.is_none() {
                return Err(Error::InvalidParams("create_arc requires 'arc_class'".into()));
            }
            if params.from_key.is_none() {
                return Err(Error::InvalidParams("create_arc requires 'from_key'".into()));
            }
            if params.to_key.is_none() {
                return Err(Error::InvalidParams("create_arc requires 'to_key'".into()));
            }
        }
        WriteOperation::UpdateProps => {
            if params.class.is_none() {
                return Err(Error::InvalidParams("update_props requires 'class'".into()));
            }
            if params.key.is_none() {
                return Err(Error::InvalidParams("update_props requires 'key'".into()));
            }
            if params.properties.is_none() {
                return Err(Error::InvalidParams(
                    "update_props requires 'properties'".into(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_operation_deserialize() {
        let json = r#""upsert_node""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::UpsertNode);

        let json = r#""create_arc""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::CreateArc);

        let json = r#""update_props""#;
        let op: WriteOperation = serde_json::from_str(json).unwrap();
        assert_eq!(op, WriteOperation::UpdateProps);
    }

    #[test]
    fn test_write_params_deserialize() {
        let json = r#"{
            "operation": "upsert_node",
            "class": "SEOKeyword",
            "key": "seo:qr-code@fr-FR",
            "properties": {
                "keyword": "qr code",
                "search_volume": 110000
            }
        }"#;

        let params: WriteParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.operation, WriteOperation::UpsertNode);
        assert_eq!(params.class, Some("SEOKeyword".to_string()));
        assert_eq!(params.key, Some("seo:qr-code@fr-FR".to_string()));
    }

    #[test]
    fn test_write_result_serialize() {
        let result = WriteResult {
            success: true,
            operation: "upsert_node".to_string(),
            key: "seo:qr-code@fr-FR".to_string(),
            created: true,
            updated_properties: vec![],
            auto_arcs_created: vec!["FOR_LOCALE".to_string()],
            execution_time_ms: 45,
            cache_invalidated: vec!["SEOKeyword:*".to_string()],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("seo:qr-code@fr-FR"));
        assert!(json.contains("FOR_LOCALE"));
    }

    #[test]
    fn test_validate_params_upsert_node_ok() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("SEOKeyword".to_string()),
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: Some(serde_json::Map::new()),
            locale: None,
        };
        assert!(validate_params(&params).is_ok());
    }

    #[test]
    fn test_validate_params_upsert_node_missing_class() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: None,
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("class"));
    }

    #[test]
    fn test_validate_params_create_arc_ok() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("seo:qr-code@fr-FR".to_string()),
            to_key: Some("entity-native:qr-code@fr-FR".to_string()),
            properties: Some(serde_json::Map::new()),
            locale: None,
        };
        assert!(validate_params(&params).is_ok());
    }

    #[test]
    fn test_validate_params_create_arc_missing_to_key() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("seo:qr-code@fr-FR".to_string()),
            to_key: None,
            properties: None,
            locale: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("to_key"));
    }
}
