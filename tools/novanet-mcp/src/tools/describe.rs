//! novanet_describe tool
//!
//! Self-description of the NovaNet knowledge graph for agent bootstrap.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// What to describe
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum DescribeTarget {
    /// Full schema overview
    Schema,
    /// Specific entity details
    Entity,
    /// EntityCategory members
    Category,
    /// ArcClass definitions (v0.12.0: was ArcKind)
    Relations,
    /// Available locales
    Locales,
    /// Graph statistics
    Stats,
}

/// Parameters for novanet_describe tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DescribeParams {
    /// What to describe
    pub describe: DescribeTarget,
    /// Entity key (required for Entity target)
    #[serde(default)]
    pub entity_key: Option<String>,
    /// Category key (required for Category target)
    #[serde(default)]
    pub category_key: Option<String>,
}

/// Result from novanet_describe tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DescribeResult {
    /// Description type
    pub target: String,
    /// Description data
    pub data: serde_json::Value,
    /// Token estimate
    pub token_estimate: usize,
}

/// Execute the novanet_describe tool
#[instrument(name = "novanet_describe", skip(state), fields(target = ?params.describe))]
pub async fn execute(state: &State, params: DescribeParams) -> Result<DescribeResult> {
    match params.describe {
        DescribeTarget::Schema => describe_schema(state).await,
        DescribeTarget::Entity => describe_entity(state, params.entity_key).await,
        DescribeTarget::Category => describe_category(state, params.category_key).await,
        DescribeTarget::Relations => describe_relations(state).await,
        DescribeTarget::Locales => describe_locales(state).await,
        DescribeTarget::Stats => describe_stats(state).await,
    }
}

/// Describe the full schema
async fn describe_schema(state: &State) -> Result<DescribeResult> {
    // Query for schema overview (v0.12.0: Class, was Kind)
    let classes_query = r#"
        MATCH (c:Class)
        WITH c.realm AS realm, c.layer AS layer, collect(c.name) AS classes
        RETURN realm, layer, classes
        ORDER BY realm, layer
    "#;

    let arc_families_query = r#"
        MATCH (a:ArcClass)
        WITH a.family AS family, count(a) AS arc_count
        RETURN family, arc_count
        ORDER BY family
    "#;

    let stats_query = r#"
        MATCH (c:Class) WITH count(c) AS class_count
        MATCH (a:ArcClass) WITH class_count, count(a) AS arc_class_count
        MATCH (l:Locale) WITH class_count, arc_class_count, count(l) AS locale_count
        RETURN class_count, arc_class_count, locale_count
    "#;

    let classes = state.pool().execute_query(classes_query, None).await?;
    let arc_families = state.pool().execute_query(arc_families_query, None).await?;
    let stats = state.pool().execute_single(stats_query, None).await?;

    let data = serde_json::json!({
        "schema_version": "0.12.0",
        "realms": organize_by_realm(&classes),
        "arc_families": arc_families,
        "statistics": stats.unwrap_or(serde_json::json!({})),
        "traversal_hints": {
            "for_generation": ["USES_ENTITY", "HAS_NATIVE", "HAS_TERMS"],
            "for_exploration": ["INCLUDES", "ENABLES", "SEMANTIC_LINK", "SIMILAR_TO"]
        }
    });

    let json_string = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(DescribeResult {
        target: "schema".to_string(),
        data,
        token_estimate,
    })
}

/// Describe a specific entity
async fn describe_entity(state: &State, entity_key: Option<String>) -> Result<DescribeResult> {
    let key = entity_key.ok_or_else(|| crate::error::Error::InvalidCypher {
        reason: "entity_key is required for Entity description".to_string(),
    })?;

    let query = r#"
        MATCH (e:Entity {key: $key})
        OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
        OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)
        OPTIONAL MATCH (e)-[r]-(related)
        WITH e, c, collect(DISTINCT {locale: en.locale, has_content: true}) AS contents,
             collect(DISTINCT {type: type(r), target_key: related.key, target_labels: labels(related)}) AS relations
        RETURN e {.*, category: c.category_key, contents: contents, relations: relations}
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::Value::String(key.clone()));

    let result = state.pool().execute_single(query, Some(params)).await?;

    let data = result.ok_or_else(|| crate::error::Error::not_found(&key))?;

    let json_string = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(DescribeResult {
        target: "entity".to_string(),
        data,
        token_estimate,
    })
}

/// Describe a category and its members
async fn describe_category(state: &State, category_key: Option<String>) -> Result<DescribeResult> {
    let data = if let Some(key) = category_key {
        // Specific category requested - must exist or error
        let mut params = serde_json::Map::new();
        params.insert("key".to_string(), serde_json::Value::String(key.clone()));

        let q = r#"
            MATCH (c:EntityCategory {category_key: $key})
            OPTIONAL MATCH (e:Entity)-[:BELONGS_TO]->(c)
            RETURN c.category_key AS category, c.content AS content,
                   collect(e.key) AS entity_keys, count(e) AS entity_count
        "#;

        state
            .pool()
            .execute_single(q, Some(params))
            .await?
            .ok_or_else(|| crate::error::Error::not_found(&key))?
    } else {
        // List all categories - empty is valid
        let q = r#"
            MATCH (c:EntityCategory)
            OPTIONAL MATCH (e:Entity)-[:BELONGS_TO]->(c)
            WITH c, count(e) AS entity_count
            RETURN collect({
                category: c.category_key,
                content: c.content,
                entity_count: entity_count
            }) AS categories
        "#;

        state
            .pool()
            .execute_single(q, None)
            .await?
            .unwrap_or_else(|| serde_json::json!({"categories": []}))
    };

    let json_string = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(DescribeResult {
        target: "category".to_string(),
        data,
        token_estimate,
    })
}

/// Describe available relations (ArcClasses, v0.12.0: was ArcKinds)
async fn describe_relations(state: &State) -> Result<DescribeResult> {
    let query = r#"
        MATCH (a:ArcClass)
        RETURN a.name AS name, a.family AS family, a.scope AS scope,
               a.cardinality AS cardinality, a.source AS source, a.target AS target,
               a.content AS content
        ORDER BY a.family, a.name
    "#;

    let relations = state.pool().execute_query(query, None).await?;

    let data = serde_json::json!({
        "arc_classes": relations,
        "families": ["ownership", "localization", "semantic", "generation", "mining"],
        "scopes": ["intra_realm", "cross_realm"]
    });

    let json_string = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(DescribeResult {
        target: "relations".to_string(),
        data,
        token_estimate,
    })
}

/// Describe available locales
async fn describe_locales(state: &State) -> Result<DescribeResult> {
    let query = r#"
        MATCH (l:Locale)
        RETURN l.key AS key, l.display_name AS display_name,
               l.language AS language, l.region AS region
        ORDER BY l.key
    "#;

    let locales = state.pool().execute_query(query, None).await?;

    let data = serde_json::json!({
        "locales": locales,
        "count": locales.len()
    });

    let json_string = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(DescribeResult {
        target: "locales".to_string(),
        data,
        token_estimate,
    })
}

/// Describe graph statistics
async fn describe_stats(state: &State) -> Result<DescribeResult> {
    // Execute all count queries in parallel for ~5x speedup
    let (node_count, rel_count, entity_count, locale_count, expr_count) = tokio::join!(
        state
            .pool()
            .execute_single("MATCH (n) RETURN count(n) AS count", None),
        state
            .pool()
            .execute_single("MATCH ()-[r]->() RETURN count(r) AS count", None),
        state
            .pool()
            .execute_single("MATCH (e:Entity) RETURN count(e) AS count", None),
        state
            .pool()
            .execute_single("MATCH (l:Locale) RETURN count(l) AS count", None),
        state
            .pool()
            .execute_single("MATCH (e:Expression) RETURN count(e) AS count", None),
    );

    let mut stats = serde_json::Map::new();

    // Helper to extract count from query result
    let extract_count = |result: Result<Option<serde_json::Value>>| -> Option<serde_json::Value> {
        result.ok()?.and_then(|r| r.get("count").cloned())
    };

    if let Some(count) = extract_count(node_count) {
        stats.insert("node_count".to_string(), count);
    }
    if let Some(count) = extract_count(rel_count) {
        stats.insert("relationship_count".to_string(), count);
    }
    if let Some(count) = extract_count(entity_count) {
        stats.insert("entity_count".to_string(), count);
    }
    if let Some(count) = extract_count(locale_count) {
        stats.insert("locale_count".to_string(), count);
    }
    if let Some(count) = extract_count(expr_count) {
        stats.insert("expression_count".to_string(), count);
    }

    // Add server stats
    let server_stats = state.stats();
    stats.insert(
        "queries_executed".to_string(),
        serde_json::json!(server_stats.queries_executed),
    );
    stats.insert(
        "cache_hits".to_string(),
        serde_json::json!(server_stats.cache_hits),
    );
    stats.insert(
        "cache_misses".to_string(),
        serde_json::json!(server_stats.cache_misses),
    );

    let data = serde_json::Value::Object(stats);

    let json_string = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(DescribeResult {
        target: "stats".to_string(),
        data,
        token_estimate,
    })
}

/// Organize classes by realm and layer (v0.12.0: was organize kinds)
fn organize_by_realm(classes: &[serde_json::Value]) -> serde_json::Value {
    let mut realms: std::collections::HashMap<
        String,
        std::collections::HashMap<String, Vec<String>>,
    > = std::collections::HashMap::new();

    for row in classes {
        if let (Some(realm), Some(layer), Some(classes_arr)) = (
            row.get("realm").and_then(|v| v.as_str()),
            row.get("layer").and_then(|v| v.as_str()),
            row.get("classes").and_then(|v| v.as_array()),
        ) {
            let realm_entry = realms.entry(realm.to_string()).or_default();
            let layer_entry = realm_entry.entry(layer.to_string()).or_default();
            for class in classes_arr {
                if let Some(name) = class.as_str() {
                    layer_entry.push(name.to_string());
                }
            }
        }
    }

    serde_json::to_value(realms).unwrap_or(serde_json::json!({}))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== DescribeTarget Tests ====================

    #[test]
    fn test_describe_target_deserialize_schema() {
        let json = r#""schema""#;
        let target: DescribeTarget = serde_json::from_str(json).unwrap();
        assert!(matches!(target, DescribeTarget::Schema));
    }

    #[test]
    fn test_describe_target_deserialize_entity() {
        let json = r#""entity""#;
        let target: DescribeTarget = serde_json::from_str(json).unwrap();
        assert!(matches!(target, DescribeTarget::Entity));
    }

    #[test]
    fn test_describe_target_deserialize_category() {
        let json = r#""category""#;
        let target: DescribeTarget = serde_json::from_str(json).unwrap();
        assert!(matches!(target, DescribeTarget::Category));
    }

    #[test]
    fn test_describe_target_deserialize_relations() {
        let json = r#""relations""#;
        let target: DescribeTarget = serde_json::from_str(json).unwrap();
        assert!(matches!(target, DescribeTarget::Relations));
    }

    #[test]
    fn test_describe_target_deserialize_locales() {
        let json = r#""locales""#;
        let target: DescribeTarget = serde_json::from_str(json).unwrap();
        assert!(matches!(target, DescribeTarget::Locales));
    }

    #[test]
    fn test_describe_target_deserialize_stats() {
        let json = r#""stats""#;
        let target: DescribeTarget = serde_json::from_str(json).unwrap();
        assert!(matches!(target, DescribeTarget::Stats));
    }

    #[test]
    fn test_describe_target_invalid() {
        let json = r#""invalid""#;
        let result: std::result::Result<DescribeTarget, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ==================== DescribeParams Tests ====================

    #[test]
    fn test_describe_params_minimal() {
        let json = r#"{"describe": "schema"}"#;
        let params: DescribeParams = serde_json::from_str(json).unwrap();
        assert!(matches!(params.describe, DescribeTarget::Schema));
        assert!(params.entity_key.is_none());
        assert!(params.category_key.is_none());
    }

    #[test]
    fn test_describe_params_with_entity_key() {
        let json = r#"{"describe": "entity", "entity_key": "qr-code"}"#;
        let params: DescribeParams = serde_json::from_str(json).unwrap();
        assert!(matches!(params.describe, DescribeTarget::Entity));
        assert_eq!(params.entity_key.as_deref(), Some("qr-code"));
        assert!(params.category_key.is_none());
    }

    #[test]
    fn test_describe_params_with_category_key() {
        let json = r#"{"describe": "category", "category_key": "product"}"#;
        let params: DescribeParams = serde_json::from_str(json).unwrap();
        assert!(matches!(params.describe, DescribeTarget::Category));
        assert!(params.entity_key.is_none());
        assert_eq!(params.category_key.as_deref(), Some("product"));
    }

    #[test]
    fn test_describe_params_all_fields() {
        let json = r#"{
            "describe": "entity",
            "entity_key": "qr-code",
            "category_key": "product"
        }"#;
        let params: DescribeParams = serde_json::from_str(json).unwrap();
        assert!(matches!(params.describe, DescribeTarget::Entity));
        assert_eq!(params.entity_key.as_deref(), Some("qr-code"));
        assert_eq!(params.category_key.as_deref(), Some("product"));
    }

    #[test]
    fn test_describe_params_missing_describe() {
        let json = r#"{"entity_key": "qr-code"}"#;
        let result: std::result::Result<DescribeParams, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ==================== DescribeResult Tests ====================

    #[test]
    fn test_describe_result_serialize() {
        let result = DescribeResult {
            target: "schema".to_string(),
            data: serde_json::json!({"classes": ["Entity", "Page"]}),
            token_estimate: 50,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""target":"schema""#));
        assert!(json.contains(r#""token_estimate":50"#));
        assert!(json.contains(r#""classes""#));
    }

    #[test]
    fn test_describe_result_with_nested_data() {
        let result = DescribeResult {
            target: "entity".to_string(),
            data: serde_json::json!({
                "key": "qr-code",
                "name": "QR Code",
                "contents": [
                    {"locale": "fr-FR", "has_content": true},
                    {"locale": "en-US", "has_content": true}
                ]
            }),
            token_estimate: 120,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("fr-FR"));
        assert!(json.contains("en-US"));
    }

    // ==================== organize_by_realm Tests ====================

    #[test]
    fn test_organize_by_realm() {
        let classes = vec![
            serde_json::json!({"realm": "shared", "layer": "config", "classes": ["Locale", "EntityCategory"]}),
            serde_json::json!({"realm": "org", "layer": "semantic", "classes": ["Entity", "EntityNative"]}),
        ];

        let organized = organize_by_realm(&classes);
        assert!(organized.get("shared").is_some());
        assert!(organized.get("org").is_some());
    }

    #[test]
    fn test_organize_by_realm_multiple_layers() {
        let classes = vec![
            serde_json::json!({"realm": "shared", "layer": "config", "classes": ["Locale"]}),
            serde_json::json!({"realm": "shared", "layer": "knowledge", "classes": ["Term", "Expression"]}),
            serde_json::json!({"realm": "org", "layer": "semantic", "classes": ["Entity"]}),
            serde_json::json!({"realm": "org", "layer": "output", "classes": ["PageNative", "BlockNative"]}),
        ];

        let organized = organize_by_realm(&classes);

        // Check shared realm
        let shared = organized.get("shared").unwrap();
        assert!(shared.get("config").is_some());
        assert!(shared.get("knowledge").is_some());

        // Check org realm
        let org = organized.get("org").unwrap();
        assert!(org.get("semantic").is_some());
        assert!(org.get("output").is_some());
    }

    #[test]
    fn test_organize_by_realm_empty_input() {
        let classes: Vec<serde_json::Value> = vec![];
        let organized = organize_by_realm(&classes);
        assert!(organized.as_object().is_none_or(|m| m.is_empty()));
    }

    #[test]
    fn test_organize_by_realm_missing_fields() {
        // Malformed input should be handled gracefully
        let classes = vec![
            serde_json::json!({"realm": "shared"}), // missing layer and classes
            serde_json::json!({"layer": "config"}), // missing realm
            serde_json::json!({"realm": "org", "layer": "semantic"}), // missing classes
        ];

        let organized = organize_by_realm(&classes);
        // Should not panic, may return empty or partial result
        assert!(organized.is_object());
    }

    #[test]
    fn test_organize_by_realm_duplicate_classes() {
        // Same realm/layer appearing twice should merge
        let classes = vec![
            serde_json::json!({"realm": "shared", "layer": "config", "classes": ["Locale"]}),
            serde_json::json!({"realm": "shared", "layer": "config", "classes": ["EntityCategory"]}),
        ];

        let organized = organize_by_realm(&classes);
        let shared_config = organized
            .get("shared")
            .and_then(|s| s.get("config"))
            .and_then(|c| c.as_array());

        assert!(shared_config.is_some());
        let config_classes = shared_config.unwrap();
        assert_eq!(config_classes.len(), 2);
    }

    // ==================== Token Estimate Tests ====================

    #[test]
    fn test_token_estimate_calculation() {
        // Token estimate is chars / 4 (ceiling division)
        let data = serde_json::json!({"key": "test"});
        let json_string = serde_json::to_string(&data).unwrap();
        let expected = json_string.len().div_ceil(4);

        // Verify our understanding of the calculation
        assert!(expected > 0);
        assert_eq!(14_usize.div_ceil(4), 4); // 14 chars -> 4 tokens
    }

    #[test]
    fn test_token_estimate_empty_data() {
        let data = serde_json::json!({});
        let json_string = serde_json::to_string(&data).unwrap();
        let estimate = json_string.len().div_ceil(4);

        // Empty object "{}" is 2 chars -> 1 token
        assert_eq!(estimate, 1);
    }

    #[test]
    fn test_token_estimate_large_data() {
        let mut classes = vec![];
        for i in 0..100 {
            classes.push(format!("Class{}", i));
        }
        let data = serde_json::json!({ "classes": classes });
        let json_string = serde_json::to_string(&data).unwrap();
        let estimate = json_string.len().div_ceil(4);

        // Large data should have proportionally larger estimate
        assert!(estimate > 100);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_describe_target_case_sensitivity() {
        // Should be lowercase
        let valid = r#""schema""#;
        let invalid = r#""Schema""#;

        assert!(serde_json::from_str::<DescribeTarget>(valid).is_ok());
        assert!(serde_json::from_str::<DescribeTarget>(invalid).is_err());
    }

    #[test]
    fn test_describe_params_null_values() {
        let json = r#"{"describe": "entity", "entity_key": null, "category_key": null}"#;
        let params: DescribeParams = serde_json::from_str(json).unwrap();
        assert!(params.entity_key.is_none());
        assert!(params.category_key.is_none());
    }

    #[test]
    fn test_describe_result_empty_data() {
        let result = DescribeResult {
            target: "stats".to_string(),
            data: serde_json::json!({}),
            token_estimate: 1,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""data":{}"#));
    }
}
