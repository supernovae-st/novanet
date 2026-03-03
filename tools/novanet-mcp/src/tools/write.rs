//! novanet_write tool
//!
//! Intelligent data writes to Neo4j with schema validation.
//! Single tool with 3 operations: upsert_node, create_arc, update_props.

use crate::error::{Error, Result};
use crate::schema_cache::{ClassMetadata, SchemaCache};
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn};

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

/// Fetch and validate class metadata for write permission
async fn fetch_and_validate_class(state: &State, class_name: &str) -> Result<ClassMetadata> {
    // Check cache first
    if let Some(meta) = state.schema_cache().get_class(class_name) {
        if !SchemaCache::is_writable_trait(&meta.trait_type) {
            return Err(Error::trait_not_writable(class_name, &meta.trait_type));
        }
        return Ok(meta);
    }

    // Fetch from Neo4j
    let query = r#"
        MATCH (c:Schema:Class {name: $name})
        RETURN c.name AS name,
               c.realm AS realm,
               c.layer AS layer,
               c.trait AS trait_type,
               c.required_properties AS required_properties,
               c.optional_properties AS optional_properties
    "#;

    let mut params = serde_json::Map::new();
    params.insert("name".to_string(), Value::String(class_name.to_string()));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if rows.is_empty() {
        return Err(Error::schema_not_found(class_name));
    }

    let row = &rows[0];
    let meta = ClassMetadata {
        name: row["name"].as_str().unwrap_or_default().to_string(),
        realm: row["realm"].as_str().unwrap_or_default().to_string(),
        layer: row["layer"].as_str().unwrap_or_default().to_string(),
        trait_type: row["trait_type"].as_str().unwrap_or_default().to_string(),
        required_properties: row["required_properties"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        optional_properties: row["optional_properties"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
    };

    // Validate trait allows writes
    if !SchemaCache::is_writable_trait(&meta.trait_type) {
        return Err(Error::trait_not_writable(class_name, &meta.trait_type));
    }

    // Cache the metadata
    state
        .schema_cache()
        .insert_class(class_name.to_string(), meta.clone());

    Ok(meta)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Special Validations (Phase 4)
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if slug is locked and reject modification
async fn validate_slug_not_locked(
    state: &State,
    key: &str,
    props: &serde_json::Map<String, Value>,
) -> Result<()> {
    // Only check if "slug" property is being modified
    if !props.contains_key("slug") {
        return Ok(());
    }

    let query = r#"
        MATCH (n {key: $key})
        RETURN n.slug_locked AS locked, n.slug AS current_slug
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), Value::String(key.to_string()));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if let Some(row) = rows.first() {
        let locked = row["locked"].as_bool().unwrap_or(false);
        let current_slug = row["current_slug"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        if locked {
            return Err(Error::slug_locked(key, current_slug));
        }
    }

    Ok(())
}

/// Handle is_slug_source singleton - only one arc can have this property true
async fn handle_slug_source_singleton(
    state: &State,
    to_key: &str,
    props: &serde_json::Map<String, Value>,
) -> Result<()> {
    // Only check if is_slug_source is being set to true
    let is_setting_slug_source = props
        .get("is_slug_source")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !is_setting_slug_source {
        return Ok(());
    }

    // Find existing is_slug_source arc and demote it
    let query = r#"
        MATCH (kw)-[r:TARGETS {is_slug_source: true}]->(en {key: $to_key})
        SET r.is_slug_source = false, r.rank = 'secondary'
        RETURN kw.key AS demoted_key
    "#;

    let mut params = serde_json::Map::new();
    params.insert("to_key".to_string(), Value::String(to_key.to_string()));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if let Some(row) = rows.first() {
        let demoted = row["demoted_key"].as_str().unwrap_or("unknown");
        warn!(
            demoted = %demoted,
            new_source = %to_key,
            "is_slug_source takeover: demoted previous source"
        );
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Operation Implementations
// ═══════════════════════════════════════════════════════════════════════════════

/// Execute upsert_node operation
async fn execute_upsert_node(
    state: &State,
    params: &WriteParams,
    _meta: &ClassMetadata,
) -> Result<WriteResult> {
    let start = std::time::Instant::now();
    let key = params.key.as_ref().expect("key validated");
    let class = params.class.as_ref().expect("class validated");
    let props = params.properties.clone().unwrap_or_default();

    // Build MERGE query
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    let query = format!(
        r#"
        MERGE (n:{class} {{key: $key}})
        ON CREATE SET n += $props, n.created_at = timestamp()
        ON MATCH SET n += $props, n.updated_at = timestamp()
        WITH n,
             CASE WHEN n.created_at = timestamp() THEN true ELSE false END AS created
        RETURN created, keys(n) AS all_keys
        "#,
        class = class
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), Value::String(key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    let rows = state
        .pool()
        .execute_query(&query, Some(query_params))
        .await?;

    let created = rows
        .first()
        .and_then(|r| r["created"].as_bool())
        .unwrap_or(false);

    // Determine updated properties (diff with previous)
    let updated_properties: Vec<String> = if created {
        vec![]
    } else {
        props.keys().cloned().collect()
    };

    // Handle auto-arcs (FOR_LOCALE)
    let mut auto_arcs = vec![];
    if let Some(locale) = &params.locale {
        let auto_arc_query = format!(
            r#"
            MATCH (n:{class} {{key: $key}})
            MATCH (l:Locale {{key: $locale}})
            MERGE (n)-[:FOR_LOCALE]->(l)
            "#,
            class = class
        );

        let mut arc_params = serde_json::Map::new();
        arc_params.insert("key".to_string(), Value::String(key.clone()));
        arc_params.insert("locale".to_string(), Value::String(locale.clone()));

        state
            .pool()
            .execute_query(&auto_arc_query, Some(arc_params))
            .await?;
        auto_arcs.push("FOR_LOCALE".to_string());
    }

    // Invalidate cache
    let cache_patterns = vec![format!("{}:*", class), key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    info!(key = %key, class = %class, created = created, "upsert_node completed");

    Ok(WriteResult {
        success: true,
        operation: "upsert_node".to_string(),
        key: key.clone(),
        created,
        updated_properties,
        auto_arcs_created: auto_arcs,
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}

/// Execute create_arc operation
async fn execute_create_arc(state: &State, params: &WriteParams) -> Result<WriteResult> {
    let start = std::time::Instant::now();
    let arc_class = params.arc_class.as_ref().expect("arc_class validated");
    let from_key = params.from_key.as_ref().expect("from_key validated");
    let to_key = params.to_key.as_ref().expect("to_key validated");
    let props = params.properties.clone().unwrap_or_default();

    // Verify endpoints exist
    let check_query = r#"
        MATCH (from {key: $from_key})
        MATCH (to {key: $to_key})
        RETURN from.key AS from_exists, to.key AS to_exists
    "#;

    let mut check_params = serde_json::Map::new();
    check_params.insert("from_key".to_string(), Value::String(from_key.clone()));
    check_params.insert("to_key".to_string(), Value::String(to_key.clone()));

    let check_rows = state
        .pool()
        .execute_query(check_query, Some(check_params))
        .await?;

    if check_rows.is_empty() {
        // Determine which endpoint is missing
        let from_exists_query = "MATCH (n {key: $key}) RETURN n.key AS exists";
        let mut p = serde_json::Map::new();
        p.insert("key".to_string(), Value::String(from_key.clone()));
        let from_check = state
            .pool()
            .execute_query(from_exists_query, Some(p))
            .await?;

        if from_check.is_empty() {
            return Err(Error::arc_endpoint_not_found("from", from_key));
        }
        return Err(Error::arc_endpoint_not_found("to", to_key));
    }

    // Handle is_slug_source singleton (takeover pattern)
    handle_slug_source_singleton(state, to_key, &props).await?;

    // Build MERGE query for arc
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    let query = format!(
        r#"
        MATCH (from {{key: $from_key}})
        MATCH (to {{key: $to_key}})
        MERGE (from)-[r:{arc_class}]->(to)
        SET r += $props
        RETURN true AS created
        "#,
        arc_class = arc_class
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("from_key".to_string(), Value::String(from_key.clone()));
    query_params.insert("to_key".to_string(), Value::String(to_key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    state
        .pool()
        .execute_query(&query, Some(query_params))
        .await?;

    // Invalidate cache
    let cache_patterns = vec![from_key.clone(), to_key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    let arc_key = format!("({})--[{}]-->({})", from_key, arc_class, to_key);
    info!(arc = %arc_key, "create_arc completed");

    Ok(WriteResult {
        success: true,
        operation: "create_arc".to_string(),
        key: arc_key,
        created: true, // MERGE always reports as created for arcs
        updated_properties: vec![],
        auto_arcs_created: vec![],
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}

/// Execute update_props operation
async fn execute_update_props(
    state: &State,
    params: &WriteParams,
    _meta: &ClassMetadata,
) -> Result<WriteResult> {
    let start = std::time::Instant::now();
    let key = params.key.as_ref().expect("key validated");
    let class = params.class.as_ref().expect("class validated");
    let props = params.properties.as_ref().expect("properties validated");

    // Verify node exists
    let check_query = format!(
        "MATCH (n:{class} {{key: $key}}) RETURN n.key AS exists",
        class = class
    );
    let mut check_params = serde_json::Map::new();
    check_params.insert("key".to_string(), Value::String(key.clone()));

    let check_rows = state
        .pool()
        .execute_query(&check_query, Some(check_params))
        .await?;

    if check_rows.is_empty() {
        return Err(Error::not_found(key));
    }

    // Check for slug_locked before update
    validate_slug_not_locked(state, key, props).await?;

    // Build SET query
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    let query = format!(
        r#"
        MATCH (n:{class} {{key: $key}})
        SET n += $props, n.updated_at = timestamp()
        RETURN keys(n) AS all_keys
        "#,
        class = class
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), Value::String(key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    state
        .pool()
        .execute_query(&query, Some(query_params))
        .await?;

    let updated_properties: Vec<String> = props.keys().cloned().collect();

    // Invalidate cache
    let cache_patterns = vec![format!("{}:*", class), key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    info!(key = %key, class = %class, props = ?updated_properties, "update_props completed");

    Ok(WriteResult {
        success: true,
        operation: "update_props".to_string(),
        key: key.clone(),
        created: false,
        updated_properties,
        auto_arcs_created: vec![],
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Main Execute Function
// ═══════════════════════════════════════════════════════════════════════════════

/// Execute the novanet_write tool
pub async fn execute(state: &State, params: WriteParams) -> Result<WriteResult> {
    // Validate parameters
    validate_params(&params)?;

    match params.operation {
        WriteOperation::UpsertNode => {
            let class = params.class.as_ref().expect("validated");
            let meta = fetch_and_validate_class(state, class).await?;
            execute_upsert_node(state, &params, &meta).await
        }
        WriteOperation::CreateArc => execute_create_arc(state, &params).await,
        WriteOperation::UpdateProps => {
            let class = params.class.as_ref().expect("validated");
            let meta = fetch_and_validate_class(state, class).await?;
            execute_update_props(state, &params, &meta).await
        }
    }
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
