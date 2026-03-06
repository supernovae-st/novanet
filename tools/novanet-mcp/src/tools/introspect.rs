//! novanet_introspect tool
//!
//! Schema introspection for agents to query NodeClass and ArcClass metadata.
//! MVP 8 Phase 3: Enables agents to understand the knowledge graph schema.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// What to introspect
#[derive(Debug, Clone, Deserialize, JsonSchema, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IntrospectTarget {
    /// List all NodeClasses (optionally filtered by realm/layer)
    #[default]
    Classes,
    /// Get a specific NodeClass with incoming/outgoing arcs
    Class,
    /// List ArcClasses (optionally filtered by family)
    Arcs,
    /// Get a specific ArcClass
    Arc,
}

/// Parameters for novanet_introspect tool
#[derive(Debug, Clone, Deserialize, JsonSchema, Default)]
pub struct IntrospectParams {
    /// What to introspect
    #[serde(default)]
    pub target: IntrospectTarget,
    /// Name of specific class/arc (required for Class/Arc targets)
    #[serde(default)]
    pub name: Option<String>,
    /// Filter by realm (shared/org)
    #[serde(default)]
    pub realm: Option<String>,
    /// Filter by layer
    #[serde(default)]
    pub layer: Option<String>,
    /// Filter by arc family
    #[serde(default)]
    pub family: Option<String>,
    /// Include arc relationships for Class target
    #[serde(default)]
    pub include_arcs: bool,
}

/// Result from novanet_introspect tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct IntrospectResult {
    /// Introspection target type
    pub target: String,
    /// Introspected data
    pub data: serde_json::Value,
    /// Token estimate for the result
    pub token_estimate: usize,
}

/// Execute the novanet_introspect tool
#[instrument(name = "novanet_introspect", skip(state), fields(target = ?params.target))]
pub async fn execute(state: &State, params: IntrospectParams) -> Result<IntrospectResult> {
    match params.target {
        IntrospectTarget::Classes => {
            get_classes(state, params.realm.as_deref(), params.layer.as_deref()).await
        }
        IntrospectTarget::Class => {
            get_class(state, params.name.as_deref(), params.include_arcs).await
        }
        IntrospectTarget::Arcs => get_arcs(state, params.family.as_deref()).await,
        IntrospectTarget::Arc => get_arc(state, params.name.as_deref()).await,
    }
}

/// Get all classes, optionally filtered by realm/layer
async fn get_classes(
    state: &State,
    realm: Option<&str>,
    layer: Option<&str>,
) -> Result<IntrospectResult> {
    // Build Cypher query with optional filters
    // Note: We use WHERE clause with NULL checks for optional filters
    let query = r#"
        MATCH (c:Class)
        WHERE ($realm IS NULL OR c.realm = $realm)
          AND ($layer IS NULL OR c.layer = $layer)
        RETURN c.name AS name, c.realm AS realm, c.layer AS layer,
               c.trait AS trait_type, c.description AS description
        ORDER BY c.realm, c.layer, c.name
    "#;

    let mut params = serde_json::Map::new();
    params.insert(
        "realm".to_string(),
        realm.map_or(serde_json::Value::Null, |r| {
            serde_json::Value::String(r.to_string())
        }),
    );
    params.insert(
        "layer".to_string(),
        layer.map_or(serde_json::Value::Null, |l| {
            serde_json::Value::String(l.to_string())
        }),
    );

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let data = serde_json::json!({
        "classes": rows,
        "total_count": rows.len(),
        "filters": {
            "realm": realm,
            "layer": layer,
        }
    });

    let json_str = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_str.len().div_ceil(4);

    Ok(IntrospectResult {
        target: "classes".to_string(),
        data,
        token_estimate,
    })
}

/// Get a specific class with optional arc information
async fn get_class(
    state: &State,
    name: Option<&str>,
    include_arcs: bool,
) -> Result<IntrospectResult> {
    let name = name
        .ok_or_else(|| crate::error::Error::invalid_cypher("name is required for Class target"))?;

    let (query, with_arcs) = if include_arcs {
        // Query with incoming/outgoing arcs
        (
            r#"
            MATCH (c:Class {name: $name})
            OPTIONAL MATCH (c)<-[:TO_CLASS]-(incoming:ArcClass)
            OPTIONAL MATCH (c)<-[:FROM_CLASS]-(outgoing:ArcClass)
            RETURN c.name AS name, c.realm AS realm, c.layer AS layer,
                   c.trait AS trait_type, c.description AS description,
                   c.llm_context AS llm_context,
                   collect(DISTINCT incoming.name) AS incoming_arcs,
                   collect(DISTINCT outgoing.name) AS outgoing_arcs
            "#,
            true,
        )
    } else {
        // Simple query without arcs
        (
            r#"
            MATCH (c:Class {name: $name})
            RETURN c.name AS name, c.realm AS realm, c.layer AS layer,
                   c.trait AS trait_type, c.description AS description,
                   c.llm_context AS llm_context
            "#,
            false,
        )
    };

    let mut params = serde_json::Map::new();
    params.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );

    let row = state
        .pool()
        .execute_single(query, Some(params))
        .await?
        .ok_or_else(|| crate::error::Error::schema_not_found(name))?;

    let mut data = row;
    data["include_arcs"] = serde_json::Value::Bool(with_arcs);

    let json_str = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_str.len().div_ceil(4);

    Ok(IntrospectResult {
        target: "class".to_string(),
        data,
        token_estimate,
    })
}

/// Get all arcs, optionally filtered by family
async fn get_arcs(state: &State, family: Option<&str>) -> Result<IntrospectResult> {
    let query = r#"
        MATCH (a:ArcClass)
        WHERE $family IS NULL OR a.family = $family
        RETURN a.name AS name, a.family AS family,
               a.scope AS scope, a.cardinality AS cardinality,
               a.source AS source, a.target AS target,
               a.description AS description
        ORDER BY a.family, a.name
    "#;

    let mut params = serde_json::Map::new();
    params.insert(
        "family".to_string(),
        family.map_or(serde_json::Value::Null, |f| {
            serde_json::Value::String(f.to_string())
        }),
    );

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let data = serde_json::json!({
        "arcs": rows,
        "total_count": rows.len(),
        "filters": {
            "family": family,
        }
    });

    let json_str = serde_json::to_string(&data).unwrap_or_default();
    let token_estimate = json_str.len().div_ceil(4);

    Ok(IntrospectResult {
        target: "arcs".to_string(),
        data,
        token_estimate,
    })
}

/// Get a specific arc
async fn get_arc(state: &State, name: Option<&str>) -> Result<IntrospectResult> {
    let name =
        name.ok_or_else(|| crate::error::Error::invalid_cypher("name is required for Arc target"))?;

    let query = r#"
        MATCH (a:ArcClass {name: $name})
        RETURN a.name AS name, a.family AS family,
               a.scope AS scope, a.cardinality AS cardinality,
               a.source AS source, a.target AS target,
               a.description AS description, a.llm_context AS llm_context
    "#;

    let mut params = serde_json::Map::new();
    params.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );

    let row = state
        .pool()
        .execute_single(query, Some(params))
        .await?
        .ok_or_else(|| crate::error::Error::schema_not_found(name))?;

    let json_str = serde_json::to_string(&row).unwrap_or_default();
    let token_estimate = json_str.len().div_ceil(4);

    Ok(IntrospectResult {
        target: "arc".to_string(),
        data: row,
        token_estimate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ══════════════════════════════════════════════════════════════════════════
    // PARAM PARSING TESTS (should pass - no state needed)
    // ══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_introspect_params_default_target_is_classes() {
        let params: IntrospectParams = serde_json::from_str("{}").unwrap();
        assert_eq!(params.target, IntrospectTarget::Classes);
    }

    #[test]
    fn test_introspect_params_parses_class_target() {
        let params: IntrospectParams =
            serde_json::from_str(r#"{"target": "class", "name": "Entity"}"#).unwrap();
        assert_eq!(params.target, IntrospectTarget::Class);
        assert_eq!(params.name, Some("Entity".to_string()));
    }

    #[test]
    fn test_introspect_params_parses_arcs_with_family() {
        let params: IntrospectParams =
            serde_json::from_str(r#"{"target": "arcs", "family": "semantic"}"#).unwrap();
        assert_eq!(params.target, IntrospectTarget::Arcs);
        assert_eq!(params.family, Some("semantic".to_string()));
    }

    #[test]
    fn test_introspect_params_parses_realm_filter() {
        let params: IntrospectParams =
            serde_json::from_str(r#"{"target": "classes", "realm": "org"}"#).unwrap();
        assert_eq!(params.target, IntrospectTarget::Classes);
        assert_eq!(params.realm, Some("org".to_string()));
    }

    #[test]
    fn test_introspect_params_parses_include_arcs_flag() {
        let params: IntrospectParams =
            serde_json::from_str(r#"{"target": "class", "name": "Page", "include_arcs": true}"#)
                .unwrap();
        assert!(params.include_arcs);
    }

    #[test]
    fn test_introspect_result_serializes_correctly() {
        let result = IntrospectResult {
            target: "classes".to_string(),
            data: serde_json::json!({"classes": [{"name": "Entity"}]}),
            token_estimate: 50,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"target\":\"classes\""));
        assert!(json.contains("\"token_estimate\":50"));
    }

    #[test]
    fn test_introspect_target_enum_variants() {
        // Verify all 4 targets are parseable
        let classes: IntrospectTarget = serde_json::from_str("\"classes\"").unwrap();
        let class: IntrospectTarget = serde_json::from_str("\"class\"").unwrap();
        let arcs: IntrospectTarget = serde_json::from_str("\"arcs\"").unwrap();
        let arc: IntrospectTarget = serde_json::from_str("\"arc\"").unwrap();

        assert_eq!(classes, IntrospectTarget::Classes);
        assert_eq!(class, IntrospectTarget::Class);
        assert_eq!(arcs, IntrospectTarget::Arcs);
        assert_eq!(arc, IntrospectTarget::Arc);
    }

    // ══════════════════════════════════════════════════════════════════════════
    // INTEGRATION TESTS (require Neo4j connection)
    // Run with: NOVANET_MCP_NEO4J_PASSWORD=novanetpassword cargo test introspect
    // ══════════════════════════════════════════════════════════════════════════

    use crate::server::{Config, State};

    /// Check if Neo4j is configured via env vars
    fn neo4j_configured() -> bool {
        std::env::var("NOVANET_MCP_NEO4J_PASSWORD").is_ok()
    }

    /// Create test state with Neo4j connection
    async fn test_state() -> State {
        let config = Config::from_env().expect("Config should load from env");
        State::new(config)
            .await
            .expect("State should connect to Neo4j")
    }

    /// Skip test if Neo4j is not configured
    macro_rules! require_neo4j {
        () => {
            if !neo4j_configured() {
                eprintln!("Skipping test: NOVANET_MCP_NEO4J_PASSWORD not set");
                return;
            }
        };
    }

    #[tokio::test]
    async fn test_introspect_all_classes_returns_61_plus_classes() {
        require_neo4j!();
        let state = test_state().await;

        let params = IntrospectParams {
            target: IntrospectTarget::Classes,
            ..Default::default()
        };

        let result = execute(&state, params).await.unwrap();

        // NovaNet has 61 classes (40 shared + 21 org)
        let classes = result.data["classes"].as_array().unwrap();
        assert!(
            classes.len() >= 61,
            "Expected at least 61 classes, got {}",
            classes.len()
        );
        assert!(result.token_estimate > 0);
    }

    #[tokio::test]
    async fn test_introspect_filtered_by_realm_org() {
        require_neo4j!();
        let state = test_state().await;

        let params = IntrospectParams {
            target: IntrospectTarget::Classes,
            realm: Some("org".into()),
            ..Default::default()
        };

        let result = execute(&state, params).await.unwrap();
        let classes = result.data["classes"].as_array().unwrap();

        // All classes should be from org realm
        assert!(classes.iter().all(|c| c["realm"] == "org"));
        // NovaNet has 21 org classes
        assert_eq!(classes.len(), 21);
    }

    #[tokio::test]
    async fn test_introspect_specific_class_entity() {
        require_neo4j!();
        let state = test_state().await;

        let params = IntrospectParams {
            target: IntrospectTarget::Class,
            name: Some("Entity".into()),
            include_arcs: true,
            ..Default::default()
        };

        let result = execute(&state, params).await.unwrap();

        assert_eq!(result.data["name"], "Entity");
        assert_eq!(result.data["realm"], "org");
        assert_eq!(result.data["layer"], "semantic");

        // Entity has incoming and outgoing arcs
        let incoming = result.data["incoming_arcs"].as_array().unwrap();
        let outgoing = result.data["outgoing_arcs"].as_array().unwrap();
        assert!(!incoming.is_empty() || !outgoing.is_empty());
    }

    #[tokio::test]
    async fn test_introspect_arc_families_semantic() {
        require_neo4j!();
        let state = test_state().await;

        let params = IntrospectParams {
            target: IntrospectTarget::Arcs,
            family: Some("semantic".into()),
            ..Default::default()
        };

        let result = execute(&state, params).await.unwrap();
        let arcs = result.data["arcs"].as_array().unwrap();

        // All arcs should be from semantic family
        assert!(arcs.iter().all(|a| a["family"] == "semantic"));
        // Should have at least some semantic arcs
        assert!(!arcs.is_empty(), "Expected semantic arcs");
    }

    #[tokio::test]
    async fn test_introspect_class_not_found_returns_error() {
        require_neo4j!();
        let state = test_state().await;

        let params = IntrospectParams {
            target: IntrospectTarget::Class,
            name: Some("NonExistentClass".into()),
            ..Default::default()
        };

        let result = execute(&state, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::error::Error::NotFound { .. }),
            "Expected NotFound error, got {:?}",
            err
        );
    }

    #[tokio::test]
    async fn test_introspect_class_requires_name() {
        require_neo4j!();
        let state = test_state().await;

        let params = IntrospectParams {
            target: IntrospectTarget::Class,
            name: None, // Missing required name
            ..Default::default()
        };

        let result = execute(&state, params).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::error::Error::InvalidCypher { .. }),
            "Expected InvalidCypher error, got {:?}",
            err
        );
    }
}
