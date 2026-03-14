//! Operation implementations for novanet_write tool
//!
//! MCP_PROVENANCE constant and 3 operation executors:
//! upsert_node, create_arc, update_props.

use super::types::*;
use super::validation::{needs_slug_source_singleton, validate_slug_not_locked};
use crate::error::{Error, Result};
use crate::schema_cache::ClassMetadata;
use crate::server::State;
use serde_json::Value;
use tracing::info;

/// Default provenance value for MCP writes (ADR-042)
pub(super) const MCP_PROVENANCE: &str = "mcp:novanet_write";

/// Execute upsert_node operation
pub(super) async fn execute_upsert_node(
    state: &State,
    params: &WriteParams,
    meta: &ClassMetadata,
) -> Result<ExecutedResult> {
    let start = std::time::Instant::now();
    let key = params
        .key
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("upsert_node requires 'key'".into()))?;
    let class = params
        .class
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("upsert_node requires 'class'".into()))?;
    let mut props = params.properties.clone().unwrap_or_default();

    // ADR-042: Auto-inject created_by provenance if not provided
    if !props.contains_key("created_by") {
        props.insert(
            "created_by".to_string(),
            serde_json::Value::String(MCP_PROVENANCE.to_string()),
        );
    }

    // SECURITY: Validate class name before use in Cypher query
    if !crate::validation::is_valid_class_name(class) {
        return Err(Error::InvalidParams(format!(
            "Invalid class name '{}': must be PascalCase alphanumeric (e.g., Entity, EntityNative)",
            class
        )));
    }

    // Validate required properties are present
    const SYSTEM_MANAGED: &[&str] = &["key", "created_at", "updated_at"];
    let missing: Vec<&String> = meta
        .required_properties
        .iter()
        .filter(|prop| !SYSTEM_MANAGED.contains(&prop.as_str()) && !props.contains_key(*prop))
        .collect();

    if !missing.is_empty() {
        return Err(Error::MissingRequiredProperty {
            class: class.to_string(),
            property: missing
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        });
    }

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
        .execute_write(&query, Some(query_params))
        .await?;

    let created = rows
        .first()
        .and_then(|r| r["created"].as_bool())
        .unwrap_or(false);

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
            .execute_write(&auto_arc_query, Some(arc_params))
            .await?;
        auto_arcs.push("FOR_LOCALE".to_string());
    }

    // Handle HAS_NATIVE auto-arc for EntityNative, PageNative, BlockNative
    if class.ends_with("Native") && key.contains('@') {
        if let Some(entity_key) = key.split('@').next() {
            let base_class = class.trim_end_matches("Native");

            let has_native_query = format!(
                r#"
                OPTIONAL MATCH (base:{base_class} {{key: $entity_key}})
                OPTIONAL MATCH (native:{class} {{key: $native_key}})
                WITH base, native
                WHERE base IS NOT NULL AND native IS NOT NULL
                MERGE (base)-[:HAS_NATIVE]->(native)
                RETURN true AS arc_created
                "#,
                base_class = base_class,
                class = class
            );

            let mut has_native_params = serde_json::Map::new();
            has_native_params.insert(
                "entity_key".to_string(),
                Value::String(entity_key.to_string()),
            );
            has_native_params.insert("native_key".to_string(), Value::String(key.clone()));

            match state
                .pool()
                .execute_write(&has_native_query, Some(has_native_params))
                .await
            {
                Ok(rows) if !rows.is_empty() => {
                    auto_arcs.push("HAS_NATIVE".to_string());
                }
                Ok(_) => {}
                Err(e) => {
                    info!(
                        key = %key,
                        base_class = %base_class,
                        error = %e,
                        "HAS_NATIVE auto-arc creation failed"
                    );
                }
            }
        }
    }

    // Invalidate cache
    let cache_patterns = vec![format!("{}:*", class), key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    info!(key = %key, class = %class, created = created, "upsert_node completed");

    Ok(ExecutedResult {
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
pub(super) async fn execute_create_arc(
    state: &State,
    params: &WriteParams,
) -> Result<ExecutedResult> {
    let start = std::time::Instant::now();
    let arc_class = params
        .arc_class
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("create_arc requires 'arc_class'".into()))?;
    let from_key = params
        .from_key
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("create_arc requires 'from_key'".into()))?;
    let to_key = params
        .to_key
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("create_arc requires 'to_key'".into()))?;
    let props = params.properties.clone().unwrap_or_default();

    // SECURITY: Validate arc class name before use in Cypher query
    if !crate::validation::is_valid_arc_class_name(arc_class) {
        return Err(Error::InvalidParams(format!(
            "Invalid arc class name '{}': must be SCREAMING_SNAKE_CASE (e.g., HAS_NATIVE, TARGETS)",
            arc_class
        )));
    }

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

    // Build MERGE query for arc
    let props_json = serde_json::to_string(&props)
        .map_err(|e| Error::Internal(format!("Props serialization: {}", e)))?;

    // ATOMIC: For TARGETS arc with is_slug_source: true, demote existing + create in ONE query
    let query = if arc_class == "TARGETS" && needs_slug_source_singleton(&props) {
        format!(
            r#"
            // ATOMIC: Demote existing is_slug_source arcs pointing to same target
            CALL {{
                MATCH ()-[existing:TARGETS {{is_slug_source: true}}]->(target {{key: $to_key}})
                SET existing.is_slug_source = false, existing.rank = 'secondary'
                RETURN count(*) AS demoted
            }}
            // Create/update the new arc with is_slug_source
            MATCH (from {{key: $from_key}})
            MATCH (to {{key: $to_key}})
            MERGE (from)-[r:{arc_class}]->(to)
            SET r += $props
            RETURN true AS created
            "#,
            arc_class = arc_class
        )
    } else {
        format!(
            r#"
            MATCH (from {{key: $from_key}})
            MATCH (to {{key: $to_key}})
            MERGE (from)-[r:{arc_class}]->(to)
            SET r += $props
            RETURN true AS created
            "#,
            arc_class = arc_class
        )
    };

    let mut query_params = serde_json::Map::new();
    query_params.insert("from_key".to_string(), Value::String(from_key.clone()));
    query_params.insert("to_key".to_string(), Value::String(to_key.clone()));
    query_params.insert("props".to_string(), serde_json::from_str(&props_json)?);

    state
        .pool()
        .execute_write(&query, Some(query_params))
        .await?;

    // Invalidate cache
    let cache_patterns = vec![from_key.clone(), to_key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    let arc_key = format!("({})--[{}]-->({})", from_key, arc_class, to_key);
    info!(arc = %arc_key, "create_arc completed");

    Ok(ExecutedResult {
        success: true,
        operation: "create_arc".to_string(),
        key: arc_key,
        created: true,
        updated_properties: vec![],
        auto_arcs_created: vec![],
        execution_time_ms: start.elapsed().as_millis() as u64,
        cache_invalidated: cache_patterns,
    })
}

/// Execute update_props operation
pub(super) async fn execute_update_props(
    state: &State,
    params: &WriteParams,
    _meta: &ClassMetadata,
) -> Result<ExecutedResult> {
    let start = std::time::Instant::now();
    let key = params
        .key
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("update_props requires 'key'".into()))?;
    let class = params
        .class
        .as_ref()
        .ok_or_else(|| Error::InvalidParams("update_props requires 'class'".into()))?;
    let mut props = params
        .properties
        .clone()
        .ok_or_else(|| Error::InvalidParams("update_props requires 'properties'".into()))?;

    // ADR-042: Auto-inject updated_by provenance
    props.insert(
        "updated_by".to_string(),
        serde_json::Value::String(MCP_PROVENANCE.to_string()),
    );

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
    validate_slug_not_locked(state, key, &props).await?;

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
        .execute_write(&query, Some(query_params))
        .await?;

    let updated_properties: Vec<String> = props.keys().cloned().collect();

    // Invalidate cache
    let cache_patterns = vec![format!("{}:*", class), key.clone()];
    for pattern in &cache_patterns {
        state.cache().invalidate_pattern(pattern).await;
    }

    info!(key = %key, class = %class, props = ?updated_properties, "update_props completed");

    Ok(ExecutedResult {
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
