//! Validation functions for novanet_write tool
//!
//! Soft validation (returning Vec<CheckIssue>), hard validation (returning Result),
//! schema metadata fetching, dry run execution, and special validations.

use super::types::*;
use crate::error::{Error, Result};
use crate::schema_cache::{ClassMetadata, ContextBudget};
use crate::server::State;
use serde_json::Value;

// =============================================================================
// Soft Validation Functions (return Vec<CheckIssue>)
// =============================================================================

/// Validate write operation has required params (returns issues, not errors)
pub(super) fn validate_operation_params(params: &WriteParams) -> Vec<CheckIssue> {
    let mut issues = vec![];

    match params.operation {
        WriteOperation::UpsertNode => {
            if params.class.is_none() {
                issues.push(
                    CheckIssue::error("E001", "upsert_node requires 'class'")
                        .with_field("class")
                        .with_hint("Specify the NodeClass name (e.g., 'EntityNative', 'SEOKeyword')"),
                );
            }
            if params.key.is_none() {
                issues.push(
                    CheckIssue::error("E002", "upsert_node requires 'key'")
                        .with_field("key")
                        .with_hint("Provide a unique key (e.g., 'qr-code@fr-FR')"),
                );
            }
        }
        WriteOperation::CreateArc => {
            if params.arc_class.is_none() {
                issues.push(
                    CheckIssue::error("E003", "create_arc requires 'arc_class'")
                        .with_field("arc_class")
                        .with_hint("Specify the arc type (e.g., 'TARGETS', 'HAS_NATIVE')"),
                );
            }
            if params.from_key.is_none() {
                issues.push(
                    CheckIssue::error("E004", "create_arc requires 'from_key'")
                        .with_field("from_key"),
                );
            }
            if params.to_key.is_none() {
                issues.push(
                    CheckIssue::error("E005", "create_arc requires 'to_key'")
                        .with_field("to_key"),
                );
            }
        }
        WriteOperation::UpdateProps => {
            if params.class.is_none() {
                issues.push(
                    CheckIssue::error("E006", "update_props requires 'class'")
                        .with_field("class"),
                );
            }
            if params.key.is_none() {
                issues.push(
                    CheckIssue::error("E007", "update_props requires 'key'")
                        .with_field("key"),
                );
            }
            if params.properties.is_none() {
                issues.push(
                    CheckIssue::error("E008", "update_props requires 'properties'")
                        .with_field("properties")
                        .with_hint("Provide a map of properties to update"),
                );
            }
        }
    }

    issues
}

/// Validate required properties are present in the provided properties map
pub(super) fn validate_required_properties(
    meta: &ClassMetadata,
    props: &serde_json::Map<String, Value>,
) -> Vec<CheckIssue> {
    const SYSTEM_MANAGED: &[&str] = &["key", "created_at", "updated_at"];
    let mut issues = vec![];

    let missing: Vec<&String> = meta
        .required_properties
        .iter()
        .filter(|prop| !SYSTEM_MANAGED.contains(&prop.as_str()) && !props.contains_key(*prop))
        .collect();

    if !missing.is_empty() {
        issues.push(
            CheckIssue::error(
                "E011",
                format!(
                    "Missing required properties for {}: {}",
                    meta.name,
                    missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                ),
            )
            .with_hint(format!(
                "Add the missing properties. Required: {:?}",
                meta.required_properties
            )),
        );
    }

    issues
}

/// Validate denomination_forms for EntityNative (ADR-033)
pub(super) fn validate_denomination_forms(
    class_name: &str,
    props: &serde_json::Map<String, Value>,
) -> Vec<CheckIssue> {
    let mut issues = vec![];

    // Only applies to EntityNative
    if class_name != "EntityNative" {
        return issues;
    }

    match props.get("denomination_forms") {
        None => {
            issues.push(
                CheckIssue::warning(
                    "W001",
                    "EntityNative missing denomination_forms (ADR-033)",
                )
                .with_field("denomination_forms")
                .with_hint(
                    "Add denomination_forms with text, title, abbrev, url forms for LLM references",
                ),
            );
        }
        Some(Value::Array(forms)) => {
            let required_types = ["text", "title", "abbrev", "url"];
            let present_types: Vec<&str> = forms
                .iter()
                .filter_map(|f| f.get("type").and_then(|t| t.as_str()))
                .collect();

            for req_type in &required_types {
                if !present_types.contains(req_type) {
                    issues.push(
                        CheckIssue::warning(
                            "W002",
                            format!(
                                "denomination_forms missing '{}' form (ADR-033)",
                                req_type
                            ),
                        )
                        .with_field("denomination_forms"),
                    );
                }
            }

            // Validate each form has required structure
            for (i, form) in forms.iter().enumerate() {
                if form.get("type").is_none() {
                    issues.push(
                        CheckIssue::error(
                            "E014",
                            format!("denomination_forms[{}] missing 'type' field", i),
                        )
                        .with_field("denomination_forms"),
                    );
                }
                if form.get("value").is_none() {
                    issues.push(
                        CheckIssue::error(
                            "E015",
                            format!("denomination_forms[{}] missing 'value' field", i),
                        )
                        .with_field("denomination_forms"),
                    );
                }
            }
        }
        Some(_) => {
            issues.push(
                CheckIssue::error("E014", "denomination_forms must be an array")
                    .with_field("denomination_forms"),
            );
        }
    }

    issues
}

// =============================================================================
// Schema Context Builders
// =============================================================================

/// Build schema context from class metadata
pub(super) fn build_schema_context(meta: &ClassMetadata) -> SchemaContext {
    SchemaContext {
        class_name: meta.name.clone(),
        realm: meta.realm.clone(),
        layer: meta.layer.clone(),
        content: meta.content.clone(),
        triggers: meta.triggers.clone(),
        required_properties: meta.required_properties.clone(),
        optional_properties: meta.optional_properties.clone(),
    }
}

/// Generate ontology-driven suggestions from schema context
pub(super) fn generate_suggestions(
    meta: &ClassMetadata,
    params: &WriteParams,
    issues: &[CheckIssue],
) -> Vec<OntologySuggestion> {
    let mut suggestions = vec![];

    // Suggest FOR_LOCALE arc if locale provided and class is locale-aware
    if params.locale.is_some() && meta.name.ends_with("Native") {
        suggestions.push(OntologySuggestion {
            action: "FOR_LOCALE arc will be auto-created".to_string(),
            reason: format!(
                "{} is locale-specific; a FOR_LOCALE arc links it to the Locale node",
                meta.name
            ),
            example: Some(format!(
                "({})-[:FOR_LOCALE]->(Locale {{key: \"{}\"}})",
                meta.name,
                params.locale.as_deref().unwrap_or("?")
            )),
        });
    }

    // Suggest HAS_NATIVE auto-arc
    if meta.name.ends_with("Native") {
        if let Some(key) = &params.key {
            if key.contains('@') {
                let base_class = meta.name.trim_end_matches("Native");
                suggestions.push(OntologySuggestion {
                    action: "HAS_NATIVE arc will be auto-created".to_string(),
                    reason: format!(
                        "{} keys with @ trigger automatic HAS_NATIVE arc to parent {}",
                        meta.name, base_class
                    ),
                    example: Some(format!(
                        "({} {{key: \"{}\"}})-[:HAS_NATIVE]->({} {{key: \"{}\"}})",
                        base_class,
                        key.split('@').next().unwrap_or("?"),
                        meta.name,
                        key,
                    )),
                });
            }
        }
    }

    // Suggest fixes for missing required properties
    for issue in issues {
        if issue.code == "E011" {
            suggestions.push(OntologySuggestion {
                action: "Add missing required properties".to_string(),
                reason: format!(
                    "Schema requires these properties for {} nodes",
                    meta.name
                ),
                example: None,
            });
            break; // One suggestion is enough
        }
    }

    // Suggest denomination_forms for EntityNative
    if meta.name == "EntityNative" {
        let has_denom = params
            .properties
            .as_ref()
            .is_some_and(|p| p.contains_key("denomination_forms"));

        if !has_denom {
            suggestions.push(OntologySuggestion {
                action: "Add denomination_forms (ADR-033)".to_string(),
                reason: "EntityNative should have prescriptive canonical forms for LLM references"
                    .to_string(),
                example: Some(
                    r#"[{"type":"text","value":"code QR","priority":1},{"type":"title","value":"Code QR","priority":1}]"#
                        .to_string(),
                ),
            });
        }
    }

    // Provenance info
    suggestions.push(OntologySuggestion {
        action: "Provenance auto-injected".to_string(),
        reason: format!("created_by/updated_by set to '{}'", super::operations::MCP_PROVENANCE),
        example: None,
    });

    suggestions
}

/// Generate Cypher preview for a write operation
pub(super) fn generate_cypher_preview(
    params: &WriteParams,
    meta: Option<&ClassMetadata>,
) -> Option<String> {
    match params.operation {
        WriteOperation::UpsertNode => {
            let class = params.class.as_deref().unwrap_or("?");
            let mut cypher = format!(
                "MERGE (n:{} {{key: $key}})\n\
                 ON CREATE SET n += $props, n.created_at = timestamp()\n\
                 ON MATCH SET n += $props, n.updated_at = timestamp()",
                class
            );

            if params.locale.is_some() {
                cypher.push_str(
                    "\n\n// Auto-arc: FOR_LOCALE\n\
                     MATCH (l:Locale {key: $locale})\n\
                     MERGE (n)-[:FOR_LOCALE]->(l)",
                );
            }

            if let Some(m) = meta {
                if m.name.ends_with("Native") {
                    if let Some(key) = &params.key {
                        if key.contains('@') {
                            let base = m.name.trim_end_matches("Native");
                            cypher.push_str(&format!(
                                "\n\n// Auto-arc: HAS_NATIVE\n\
                                 MATCH (base:{} {{key: $entity_key}})\n\
                                 MERGE (base)-[:HAS_NATIVE]->(n)",
                                base
                            ));
                        }
                    }
                }
            }

            Some(cypher)
        }
        WriteOperation::CreateArc => {
            let arc = params.arc_class.as_deref().unwrap_or("?");
            Some(format!(
                "MATCH (from {{key: $from_key}})\n\
                 MATCH (to {{key: $to_key}})\n\
                 MERGE (from)-[r:{}]->(to)\n\
                 SET r += $props",
                arc
            ))
        }
        WriteOperation::UpdateProps => {
            let class = params.class.as_deref().unwrap_or("?");
            Some(format!(
                "MATCH (n:{} {{key: $key}})\n\
                 SET n += $props, n.updated_at = timestamp()",
                class
            ))
        }
    }
}

/// Estimate token cost of a dry run result
pub(super) fn estimate_result_tokens(result: &DryRunResult) -> u32 {
    let json = serde_json::to_string(result).unwrap_or_default();
    (json.len() as f64 / 4.0).ceil() as u32
}

// =============================================================================
// Hard Validation (for actual writes)
// =============================================================================

/// Validate write operation has required params (hard error)
pub(super) fn validate_params(params: &WriteParams) -> Result<()> {
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
                return Err(Error::InvalidParams(
                    "create_arc requires 'arc_class'".into(),
                ));
            }
            if params.from_key.is_none() {
                return Err(Error::InvalidParams(
                    "create_arc requires 'from_key'".into(),
                ));
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

// =============================================================================
// Schema Metadata Fetching
// =============================================================================

/// Fetch and validate class metadata for write permission
///
/// v0.19.0: Trait-based write permissions removed (ADR-024 deprecated).
/// Provenance is now per-instance, not per-class.
pub(super) async fn fetch_and_validate_class(
    state: &State,
    class_name: &str,
) -> Result<ClassMetadata> {
    // Check cache first
    if let Some(meta) = state.schema_cache().get_class(class_name) {
        return Ok(meta);
    }

    // Fetch from Neo4j (including ontology fields for v0.17.0)
    let query = r#"
        MATCH (c:Schema:Class {label: $name})
        RETURN c.label AS name,
               c.realm AS realm,
               c.layer AS layer,
               c.required_properties AS required_properties,
               c.optional_properties AS optional_properties,
               c.content AS content,
               c.triggers AS triggers,
               c.schema_hint AS schema_hint,
               c.context_budget AS context_budget,
               c.visibility AS visibility
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
        // Ontology-driven fields (v0.17.0)
        content: row["content"].as_str().map(String::from),
        triggers: row["triggers"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            }),
        schema_hint: row["schema_hint"].as_str().map(String::from),
        context_budget: row["context_budget"]
            .as_str()
            .map(ContextBudget::from)
            .unwrap_or_default(),
        visibility: row["visibility"].as_str().map(String::from),
    };

    // Cache the metadata
    state
        .schema_cache()
        .insert_class(class_name.to_string(), meta.clone());

    Ok(meta)
}

// =============================================================================
// Dry Run Execution (D6: replaces novanet_check)
// =============================================================================

/// Execute dry_run validation — validates everything without writing
pub(super) async fn execute_dry_run(
    state: &State,
    params: &WriteParams,
) -> Result<DryRunResult> {
    let mut all_issues = vec![];

    // Phase 1: Parameter validation
    let param_issues = validate_operation_params(params);
    let has_param_errors = param_issues
        .iter()
        .any(|i| i.severity == CheckSeverity::Error);
    all_issues.extend(param_issues);

    // If params are invalid, return early — can't do schema validation
    if has_param_errors {
        let mut result = DryRunResult {
            valid: false,
            would_create: false,
            issues: all_issues,
            cypher_preview: generate_cypher_preview(params, None),
            schema_context: None,
            suggestions: vec![],
            token_estimate: 0,
        };
        result.token_estimate = estimate_result_tokens(&result);
        return Ok(result);
    }

    // Phase 2: Schema validation (for node operations)
    let meta = match params.operation {
        WriteOperation::UpsertNode | WriteOperation::UpdateProps => {
            let class_name = params.class.as_deref().unwrap_or_default();

            // Validate class name format
            if !crate::validation::is_valid_class_name(class_name) {
                all_issues.push(
                    CheckIssue::error(
                        "E009",
                        format!(
                            "Invalid class name '{}': must be PascalCase alphanumeric",
                            class_name
                        ),
                    )
                    .with_field("class"),
                );
                None
            } else {
                // Try to fetch class metadata
                match fetch_and_validate_class(state, class_name).await {
                    Ok(m) => Some(m),
                    Err(Error::SchemaNotFound { .. }) => {
                        all_issues.push(
                            CheckIssue::error(
                                "E010",
                                format!("Class '{}' not found in schema", class_name),
                            )
                            .with_field("class")
                            .with_hint("Use novanet_introspect to list available classes"),
                        );
                        None
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        WriteOperation::CreateArc => {
            // Validate arc class name
            if let Some(arc_name) = &params.arc_class {
                if !crate::validation::is_valid_arc_class_name(arc_name) {
                    all_issues.push(
                        CheckIssue::error(
                            "E013",
                            format!(
                                "Invalid arc class name '{}': must be SCREAMING_SNAKE_CASE",
                                arc_name
                            ),
                        )
                        .with_field("arc_class"),
                    );
                }
            }
            None
        }
    };

    // Phase 3: Property validation (if metadata available)
    if let Some(ref m) = meta {
        if let Some(ref props) = params.properties {
            // Required properties check
            if params.operation == WriteOperation::UpsertNode {
                all_issues.extend(validate_required_properties(m, props));
            }

            // EntityNative denomination_forms check (ADR-033)
            all_issues.extend(validate_denomination_forms(&m.name, props));
        } else if params.operation == WriteOperation::UpsertNode {
            // No properties provided for upsert — check if required exist
            all_issues.extend(validate_required_properties(m, &serde_json::Map::new()));
        }
    }

    // Phase 4: Check if node already exists (for would_create)
    let would_create = if let Some(key) = &params.key {
        if let Some(class) = &params.class {
            let check_query = format!(
                "MATCH (n:{} {{key: $key}}) RETURN n.key AS exists",
                class
            );
            let mut check_params = serde_json::Map::new();
            check_params.insert("key".to_string(), Value::String(key.clone()));

            match state.pool().execute_query(&check_query, Some(check_params)).await {
                Ok(rows) => rows.is_empty(), // Would create if doesn't exist
                Err(_) => true,              // Assume create on error
            }
        } else {
            true
        }
    } else {
        false
    };

    // Phase 5: Build context and suggestions
    let schema_ctx = meta.as_ref().map(build_schema_context);
    let suggestions = meta
        .as_ref()
        .map(|m| generate_suggestions(m, params, &all_issues))
        .unwrap_or_default();

    let valid = !all_issues.iter().any(|i| i.severity == CheckSeverity::Error);

    let mut result = DryRunResult {
        valid,
        would_create,
        issues: all_issues,
        cypher_preview: generate_cypher_preview(params, meta.as_ref()),
        schema_context: schema_ctx,
        suggestions,
        token_estimate: 0,
    };
    result.token_estimate = estimate_result_tokens(&result);

    Ok(result)
}

// =============================================================================
// Special Validations
// =============================================================================

/// Check if slug is locked and reject modification
pub(super) async fn validate_slug_not_locked(
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
        let current_slug = row["current_slug"].as_str().unwrap_or_default().to_string();

        if locked {
            return Err(Error::slug_locked(key, current_slug));
        }
    }

    Ok(())
}

/// Check if is_slug_source singleton handling is needed
pub(super) fn needs_slug_source_singleton(props: &serde_json::Map<String, Value>) -> bool {
    props
        .get("is_slug_source")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}
