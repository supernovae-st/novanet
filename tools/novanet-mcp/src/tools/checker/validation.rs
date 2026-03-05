//! Validation logic for novanet_check
//!
//! Provides pre-write validation without executing actual database mutations.
//! Extracts and reuses validation logic from write.rs.

use super::types::{CheckIssue, CheckParams, CheckResult, OntologySuggestion, SchemaContext};
use crate::error::Result;
use crate::schema_cache::{ClassMetadata, SchemaCache};
use crate::server::State;
use crate::tools::write::WriteOperation;
use crate::validation::{is_valid_arc_class_name, is_valid_class_name};

/// Validate operation parameters without touching Neo4j
///
/// Checks:
/// - Required parameters are present
/// - Class/arc names follow naming conventions
/// - Operation-specific requirements
pub fn validate_operation_params(params: &CheckParams) -> Vec<CheckIssue> {
    let mut issues = Vec::new();

    match params.operation {
        WriteOperation::UpsertNode => {
            if params.class.is_none() {
                issues.push(
                    CheckIssue::error("E001", "upsert_node requires 'class' parameter")
                        .with_field("class")
                        .with_hint("Add class: \"EntityNative\" or similar writable class"),
                );
            }
            if params.key.is_none() {
                issues.push(
                    CheckIssue::error("E002", "upsert_node requires 'key' parameter")
                        .with_field("key")
                        .with_hint("Add key: \"qr-code@fr-FR\" or similar"),
                );
            }
            if let Some(c) = &params.class {
                if !is_valid_class_name(c) {
                    issues.push(
                        CheckIssue::error("E003", format!("Invalid class name: {}", c))
                            .with_field("class")
                            .with_hint("Class names must be PascalCase (e.g., EntityNative)"),
                    );
                }
            }
        }
        WriteOperation::CreateArc => {
            if params.arc_class.is_none() {
                issues.push(
                    CheckIssue::error("E004", "create_arc requires 'arc_class' parameter")
                        .with_field("arc_class")
                        .with_hint("Add arc_class: \"TARGETS\" or \"FOR_LOCALE\""),
                );
            }
            if params.from_key.is_none() {
                issues.push(
                    CheckIssue::error("E005", "create_arc requires 'from_key' parameter")
                        .with_field("from_key")
                        .with_hint("Add from_key: source node key"),
                );
            }
            if params.to_key.is_none() {
                issues.push(
                    CheckIssue::error("E006", "create_arc requires 'to_key' parameter")
                        .with_field("to_key")
                        .with_hint("Add to_key: target node key"),
                );
            }
            if let Some(ac) = &params.arc_class {
                if !is_valid_arc_class_name(ac) {
                    issues.push(
                        CheckIssue::error("E007", format!("Invalid arc class name: {}", ac))
                            .with_field("arc_class")
                            .with_hint("Arc names must be SCREAMING_SNAKE_CASE (e.g., HAS_NATIVE)"),
                    );
                }
            }
        }
        WriteOperation::UpdateProps => {
            if params.class.is_none() {
                issues.push(
                    CheckIssue::error("E008", "update_props requires 'class' parameter")
                        .with_field("class")
                        .with_hint("Add class: target node class name"),
                );
            }
            if params.key.is_none() {
                issues.push(
                    CheckIssue::error("E009", "update_props requires 'key' parameter")
                        .with_field("key")
                        .with_hint("Add key: target node key"),
                );
            }
            if params.properties.is_none() || params.properties.as_ref().map(|p| p.is_empty()).unwrap_or(true) {
                issues.push(
                    CheckIssue::error("E010", "update_props requires non-empty 'properties'")
                        .with_field("properties")
                        .with_hint("Add properties to update"),
                );
            }
        }
    }

    issues
}

/// Validate required properties are present
pub fn validate_required_properties(
    meta: &ClassMetadata,
    properties: &Option<serde_json::Map<String, serde_json::Value>>,
) -> Vec<CheckIssue> {
    let mut issues = Vec::new();

    let props = properties.as_ref();

    for required_prop in &meta.required_properties {
        let has_prop = props.map(|p| p.contains_key(required_prop)).unwrap_or(false);
        if !has_prop {
            issues.push(
                CheckIssue::error(
                    "E011",
                    format!("Missing required property: '{}'", required_prop),
                )
                .with_field(required_prop)
                .with_hint(format!(
                    "Add {}: <value> to properties",
                    required_prop
                )),
            );
        }
    }

    issues
}

/// Build schema context from class metadata
pub fn build_schema_context(meta: &ClassMetadata) -> SchemaContext {
    let mut ctx = SchemaContext::new();

    if let Some(desc) = &meta.description {
        ctx = ctx.with_description(desc.clone());
    }

    if let Some(llm_ctx) = &meta.llm_context {
        ctx = ctx.with_llm_context(llm_ctx.clone());
    }

    // Add trait explanation
    let trait_explanation = match meta.trait_type.as_str() {
        "defined" => "defined = Human creates ONCE, immutable. Cannot be written via MCP.",
        "authored" => "authored = Human creates PER locale (e.g., EntityNative). Writable via MCP.",
        "imported" => "imported = External data source (e.g., SEOKeyword). Writable via MCP.",
        "generated" => "generated = LLM-generated content (e.g., BlockNative). Writable via MCP.",
        "retrieved" => "retrieved = External API data (e.g., GEOAnswer). Writable via MCP.",
        _ => "Unknown trait type",
    };
    ctx = ctx.with_trait_explanation(trait_explanation);

    ctx
}

/// Generate suggestions based on class metadata and operation
pub fn generate_suggestions(
    meta: &ClassMetadata,
    params: &CheckParams,
    issues: &[CheckIssue],
) -> Vec<OntologySuggestion> {
    let mut suggestions = Vec::new();

    // Suggest FOR_LOCALE arc for *Native classes
    if meta.name.ends_with("Native") && params.locale.is_some() {
        suggestions.push(
            OntologySuggestion::new(
                "create_arc",
                "FOR_LOCALE arc links locale-specific content to its target Locale",
            )
            .with_example(format!(
                "After creating {}, create arc: FOR_LOCALE from '{}' to '{}'",
                meta.name,
                params.key.as_deref().unwrap_or("<key>"),
                params.locale.as_deref().unwrap_or("<locale>")
            )),
        );
    }

    // Suggest fixing missing required properties
    for issue in issues {
        if issue.code == "E011" {
            if let Some(field) = &issue.field {
                suggestions.push(
                    OntologySuggestion::new(
                        "add_property",
                        format!("Property '{}' is required by {} schema", field, meta.name),
                    )
                    .with_example(format!("\"{}\": <value>", field)),
                );
            }
        }
    }

    suggestions
}

/// Main validation function - validates write params without executing
///
/// Returns a CheckResult with:
/// - valid: whether the write would succeed
/// - issues: list of validation problems
/// - schema_context: ontology context for AI understanding
/// - suggestions: actionable fixes
/// - cypher_preview: the Cypher that would be executed
pub async fn validate_write(state: &State, params: &CheckParams) -> Result<CheckResult> {
    let mut result = CheckResult::valid();

    // Step 1: Validate operation parameters (no DB needed)
    let param_issues = validate_operation_params(params);
    for issue in param_issues {
        result.add_issue(issue);
    }

    // If basic params are invalid, return early
    if !result.valid {
        return Ok(result);
    }

    // Step 2: For node operations, fetch and validate class metadata
    if let Some(class_name) = &params.class {
        // Check cache first, then Neo4j
        let meta = match state.schema_cache().get_class(class_name) {
            Some(m) => m,
            None => {
                // Query Neo4j for class metadata
                let query = r#"
                    MATCH (c:Schema:Class {label: $name})
                    RETURN c.label AS name,
                           c.realm AS realm,
                           c.layer AS layer,
                           c.trait AS trait_type,
                           c.required_properties AS required_properties,
                           c.optional_properties AS optional_properties,
                           c.description AS description,
                           c.llm_context AS llm_context
                "#;

                let mut query_params = serde_json::Map::new();
                query_params.insert("name".to_string(), serde_json::Value::String(class_name.clone()));

                let rows = state.pool().execute_query(query, Some(query_params)).await?;

                if rows.is_empty() {
                    result.add_issue(
                        CheckIssue::error("E012", format!("Unknown class: {}", class_name))
                            .with_field("class")
                            .with_hint("Check class name spelling. Use novanet_introspect to list available classes."),
                    );
                    return Ok(result);
                }

                let row = &rows[0];
                ClassMetadata {
                    name: row["name"].as_str().unwrap_or_default().to_string(),
                    realm: row["realm"].as_str().unwrap_or_default().to_string(),
                    layer: row["layer"].as_str().unwrap_or_default().to_string(),
                    trait_type: row["trait_type"].as_str().unwrap_or_default().to_string(),
                    required_properties: row["required_properties"]
                        .as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    optional_properties: row["optional_properties"]
                        .as_array()
                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                        .unwrap_or_default(),
                    description: row["description"].as_str().map(String::from),
                    llm_context: row["llm_context"].as_str().map(String::from),
                    ..Default::default()
                }
            }
        };

        // Validate trait allows writes
        if !SchemaCache::is_writable_trait(&meta.trait_type) {
            result.add_issue(
                CheckIssue::error(
                    "E013",
                    format!("Cannot write to {} - trait '{}' is read-only", class_name, meta.trait_type),
                )
                .with_field("class")
                .with_hint(format!(
                    "Only authored/imported/generated/retrieved traits are writable. {} has trait '{}'.",
                    class_name, meta.trait_type
                )),
            );
        }

        // Validate required properties
        let prop_issues = validate_required_properties(&meta, &params.properties);
        for issue in prop_issues {
            result.add_issue(issue);
        }

        // Build schema context for AI understanding
        let schema_context = build_schema_context(&meta);
        result = result.with_schema_context(schema_context);

        // Generate suggestions
        let suggestions = generate_suggestions(&meta, params, &result.issues);
        result = result.with_suggestions(suggestions);

        // Check if node already exists (to set would_create)
        if let Some(key) = &params.key {
            let exists_query = format!(
                "MATCH (n:{} {{key: $key}}) RETURN count(n) > 0 AS exists",
                class_name
            );
            let mut exists_params = serde_json::Map::new();
            exists_params.insert("key".to_string(), serde_json::Value::String(key.clone()));

            if let Ok(rows) = state.pool().execute_query(&exists_query, Some(exists_params)).await {
                let exists = rows.first()
                    .and_then(|r| r["exists"].as_bool())
                    .unwrap_or(false);
                result = result.with_would_create(!exists);
            }
        }

        // Generate Cypher preview
        let cypher_preview = generate_cypher_preview(params, &meta);
        result = result.with_cypher_preview(cypher_preview);
    }

    // Estimate tokens
    let token_estimate = estimate_result_tokens(&result);
    result = result.with_token_estimate(token_estimate);

    Ok(result)
}

/// Generate a preview of the Cypher that would be executed
fn generate_cypher_preview(params: &CheckParams, meta: &ClassMetadata) -> String {
    match params.operation {
        WriteOperation::UpsertNode => {
            let key = params.key.as_deref().unwrap_or("$key");
            format!(
                "MERGE (n:{} {{key: '{}'}})\nON CREATE SET n += $properties, n.created_at = timestamp()\nON MATCH SET n += $properties, n.updated_at = timestamp()",
                meta.name, key
            )
        }
        WriteOperation::CreateArc => {
            let arc = params.arc_class.as_deref().unwrap_or("$arc_class");
            let from = params.from_key.as_deref().unwrap_or("$from_key");
            let to = params.to_key.as_deref().unwrap_or("$to_key");
            format!(
                "MATCH (from {{key: '{}'}}), (to {{key: '{}'}})\nMERGE (from)-[:{}]->(to)",
                from, to, arc
            )
        }
        WriteOperation::UpdateProps => {
            let key = params.key.as_deref().unwrap_or("$key");
            format!(
                "MATCH (n:{} {{key: '{}'}})\nSET n += $properties, n.updated_at = timestamp()",
                meta.name, key
            )
        }
    }
}

/// Estimate token count for the result
fn estimate_result_tokens(result: &CheckResult) -> u32 {
    let mut tokens = 50; // Base overhead

    tokens += (result.issues.len() * 30) as u32;

    if let Some(ctx) = &result.schema_context {
        if ctx.class_description.is_some() {
            tokens += 20;
        }
        if ctx.llm_context.is_some() {
            tokens += 50;
        }
        tokens += (ctx.mandatory_arcs.len() * 5) as u32;
        tokens += (ctx.related_classes.len() * 3) as u32;
    }

    tokens += (result.suggestions.len() * 25) as u32;

    if result.cypher_preview.is_some() {
        tokens += 40;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_upsert_node_missing_class() {
        let params = CheckParams {
            operation: WriteOperation::UpsertNode,
            class: None,
            arc_class: None,
            key: Some("test-key".to_string()),
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };

        let issues = validate_operation_params(&params);
        assert!(issues.iter().any(|i| i.code == "E001"));
    }

    #[test]
    fn test_validate_upsert_node_missing_key() {
        let params = CheckParams {
            operation: WriteOperation::UpsertNode,
            class: Some("EntityNative".to_string()),
            arc_class: None,
            key: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };

        let issues = validate_operation_params(&params);
        assert!(issues.iter().any(|i| i.code == "E002"));
    }

    #[test]
    fn test_validate_create_arc_missing_params() {
        let params = CheckParams {
            operation: WriteOperation::CreateArc,
            class: None,
            arc_class: None,
            key: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };

        let issues = validate_operation_params(&params);
        assert!(issues.iter().any(|i| i.code == "E004")); // Missing arc_class
        assert!(issues.iter().any(|i| i.code == "E005")); // Missing from_key
        assert!(issues.iter().any(|i| i.code == "E006")); // Missing to_key
    }

    #[test]
    fn test_validate_invalid_class_name() {
        let params = CheckParams {
            operation: WriteOperation::UpsertNode,
            class: Some("invalid_class".to_string()), // Should be PascalCase
            arc_class: None,
            key: Some("test-key".to_string()),
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };

        let issues = validate_operation_params(&params);
        assert!(issues.iter().any(|i| i.code == "E003"));
    }

    #[test]
    fn test_validate_required_properties() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),
            trait_type: "imported".to_string(),
            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec![],
            ..Default::default()
        };

        // Missing required properties
        let props = Some(serde_json::Map::new());
        let issues = validate_required_properties(&meta, &props);
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().any(|i| i.field == Some("keyword".to_string())));
        assert!(issues.iter().any(|i| i.field == Some("slug_form".to_string())));
    }

    #[test]
    fn test_build_schema_context() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            trait_type: "authored".to_string(),
            required_properties: vec![],
            optional_properties: vec![],
            description: Some("Locale-native content".to_string()),
            llm_context: Some("USE: when loading localized data".to_string()),
            ..Default::default()
        };

        let ctx = build_schema_context(&meta);
        assert!(ctx.class_description.is_some());
        assert!(ctx.llm_context.is_some());
        assert!(ctx.trait_explanation.unwrap().contains("authored"));
    }

    #[test]
    fn test_generate_cypher_preview() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            ..Default::default()
        };

        let params = CheckParams {
            operation: WriteOperation::UpsertNode,
            class: Some("EntityNative".to_string()),
            arc_class: None,
            key: Some("qr-code@fr-FR".to_string()),
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
        };

        let preview = generate_cypher_preview(&params, &meta);
        assert!(preview.contains("MERGE"));
        assert!(preview.contains("EntityNative"));
        assert!(preview.contains("qr-code@fr-FR"));
    }
}
