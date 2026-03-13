//! novanet_write tool (v0.20.0)
//!
//! Intelligent data writes to Neo4j with schema validation.
//! Single tool with 3 operations: upsert_node, create_arc, update_props.
//!
//! v0.20.0 (D6): Absorbs novanet_check via dry_run parameter.
//! When dry_run=true: validates everything, returns Cypher preview + suggestions.
//! When dry_run=false/None: validates then executes.

use crate::error::{Error, Result};
use crate::schema_cache::{ClassMetadata, ContextBudget, SchemaCache};
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

// ═══════════════════════════════════════════════════════════════════════════════
// Types (absorbed from checker/types.rs)
// ═══════════════════════════════════════════════════════════════════════════════

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

/// Severity level for validation issues
#[derive(Debug, Clone, Serialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckSeverity {
    Error,
    Warning,
    Info,
}

/// A single validation issue found during pre-write checks
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CheckIssue {
    pub severity: CheckSeverity,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

impl CheckIssue {
    fn error(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Error,
            code: code.to_string(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    fn warning(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Warning,
            code: code.to_string(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    #[allow(dead_code)]
    fn info(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Info,
            code: code.to_string(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    fn with_field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }

    fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

/// Schema context returned during dry_run validation
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SchemaContext {
    pub class_name: String,
    pub realm: String,
    pub layer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<String>>,
    pub required_properties: Vec<String>,
    pub optional_properties: Vec<String>,
}

/// Ontology-driven suggestion from schema context
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct OntologySuggestion {
    pub action: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
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

    /// Dry run mode: validate and return preview without executing (D6)
    #[serde(default)]
    pub dry_run: Option<bool>,
}

/// Result from a dry_run validation (replaces novanet_check)
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DryRunResult {
    /// Whether the operation would succeed
    pub valid: bool,

    /// Whether a new node would be created (vs updated)
    pub would_create: bool,

    /// Validation issues found
    pub issues: Vec<CheckIssue>,

    /// Cypher query that would be executed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cypher_preview: Option<String>,

    /// Schema context for the target class
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_context: Option<SchemaContext>,

    /// Ontology-driven suggestions
    pub suggestions: Vec<OntologySuggestion>,

    /// Estimated token cost
    pub token_estimate: u32,
}

/// Result from novanet_write tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum WriteResult {
    /// Actual write execution result
    Executed(ExecutedResult),
    /// Dry run validation result
    DryRun(DryRunResult),
}

/// Result from an actual write execution
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ExecutedResult {
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

// ═══════════════════════════════════════════════════════════════════════════════
// Validation Functions (absorbed from checker/validation.rs)
// ═══════════════════════════════════════════════════════════════════════════════

/// Validate write operation has required params (returns issues, not errors)
fn validate_operation_params(params: &WriteParams) -> Vec<CheckIssue> {
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
fn validate_required_properties(
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
fn validate_denomination_forms(
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

/// Build schema context from class metadata
fn build_schema_context(meta: &ClassMetadata) -> SchemaContext {
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
fn generate_suggestions(
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
        reason: format!("created_by/updated_by set to '{}'", MCP_PROVENANCE),
        example: None,
    });

    suggestions
}

/// Generate Cypher preview for a write operation
fn generate_cypher_preview(params: &WriteParams, meta: Option<&ClassMetadata>) -> Option<String> {
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
fn estimate_result_tokens(result: &DryRunResult) -> u32 {
    let json = serde_json::to_string(result).unwrap_or_default();
    (json.len() as f64 / 4.0).ceil() as u32
}

// ═══════════════════════════════════════════════════════════════════════════════
// Legacy validate_params (hard-error version for actual writes)
// ═══════════════════════════════════════════════════════════════════════════════

/// Validate write operation has required params (hard error)
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

/// Fetch and validate class metadata for write permission
async fn fetch_and_validate_class(state: &State, class_name: &str) -> Result<ClassMetadata> {
    // Check cache first
    if let Some(meta) = state.schema_cache().get_class(class_name) {
        if !SchemaCache::is_writable_trait(&meta.trait_type) {
            return Err(Error::trait_not_writable(class_name, &meta.trait_type));
        }
        return Ok(meta);
    }

    // Fetch from Neo4j (including ontology fields for v0.17.0)
    let query = r#"
        MATCH (c:Schema:Class {label: $name})
        RETURN c.label AS name,
               c.realm AS realm,
               c.layer AS layer,
               c.trait AS trait_type,
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
// Dry Run Execution (D6: replaces novanet_check)
// ═══════════════════════════════════════════════════════════════════════════════

/// Execute dry_run validation — validates everything without writing
async fn execute_dry_run(state: &State, params: &WriteParams) -> Result<DryRunResult> {
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
                    Err(Error::TraitNotWritable { class, trait_type }) => {
                        all_issues.push(
                            CheckIssue::error(
                                "E012",
                                format!(
                                    "Class '{}' has trait '{}' which is read-only",
                                    class, trait_type
                                ),
                            )
                            .with_field("class")
                            .with_hint("Only authored/imported/generated/retrieved traits allow writes"),
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

// ═══════════════════════════════════════════════════════════════════════════════
// Special Validations
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
        let current_slug = row["current_slug"].as_str().unwrap_or_default().to_string();

        if locked {
            return Err(Error::slug_locked(key, current_slug));
        }
    }

    Ok(())
}

/// Check if is_slug_source singleton handling is needed
fn needs_slug_source_singleton(props: &serde_json::Map<String, Value>) -> bool {
    props
        .get("is_slug_source")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Operation Implementations
// ═══════════════════════════════════════════════════════════════════════════════

/// Default provenance value for MCP writes (ADR-042)
const MCP_PROVENANCE: &str = "mcp:novanet_write";

/// Execute upsert_node operation
async fn execute_upsert_node(
    state: &State,
    params: &WriteParams,
    meta: &ClassMetadata,
) -> Result<ExecutedResult> {
    let start = std::time::Instant::now();
    let key = params.key.as_ref().expect("key validated");
    let class = params.class.as_ref().expect("class validated");
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
async fn execute_create_arc(state: &State, params: &WriteParams) -> Result<ExecutedResult> {
    let start = std::time::Instant::now();
    let arc_class = params.arc_class.as_ref().expect("arc_class validated");
    let from_key = params.from_key.as_ref().expect("from_key validated");
    let to_key = params.to_key.as_ref().expect("to_key validated");
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
async fn execute_update_props(
    state: &State,
    params: &WriteParams,
    _meta: &ClassMetadata,
) -> Result<ExecutedResult> {
    let start = std::time::Instant::now();
    let key = params.key.as_ref().expect("key validated");
    let class = params.class.as_ref().expect("class validated");
    let mut props = params.properties.clone().expect("properties validated");

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

// ═══════════════════════════════════════════════════════════════════════════════
// Main Execute Function
// ═══════════════════════════════════════════════════════════════════════════════

/// Execute the novanet_write tool
pub async fn execute(state: &State, params: WriteParams) -> Result<WriteResult> {
    // D6: dry_run mode — validate without executing
    if params.dry_run.unwrap_or(false) {
        let result = execute_dry_run(state, &params).await?;
        return Ok(WriteResult::DryRun(result));
    }

    // Normal write: validate then execute
    validate_params(&params)?;

    let result = match params.operation {
        WriteOperation::UpsertNode => {
            let class = params.class.as_ref().expect("validated");
            let meta = fetch_and_validate_class(state, class).await?;
            execute_upsert_node(state, &params, &meta).await?
        }
        WriteOperation::CreateArc => execute_create_arc(state, &params).await?,
        WriteOperation::UpdateProps => {
            let class = params.class.as_ref().expect("validated");
            let meta = fetch_and_validate_class(state, class).await?;
            execute_update_props(state, &params, &meta).await?
        }
    };

    Ok(WriteResult::Executed(result))
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
    fn test_write_params_deserialize_with_dry_run() {
        let json = r#"{
            "operation": "upsert_node",
            "class": "SEOKeyword",
            "key": "seo:qr-code@fr-FR",
            "properties": {
                "keyword": "qr code",
                "search_volume": 110000
            },
            "dry_run": true
        }"#;

        let params: WriteParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.operation, WriteOperation::UpsertNode);
        assert_eq!(params.class, Some("SEOKeyword".to_string()));
        assert_eq!(params.key, Some("seo:qr-code@fr-FR".to_string()));
        assert_eq!(params.dry_run, Some(true));
    }

    #[test]
    fn test_write_params_deserialize_without_dry_run() {
        let json = r#"{
            "operation": "upsert_node",
            "class": "SEOKeyword",
            "key": "seo:qr-code@fr-FR"
        }"#;

        let params: WriteParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.dry_run, None);
    }

    #[test]
    fn test_executed_result_serialize() {
        let result = WriteResult::Executed(ExecutedResult {
            success: true,
            operation: "upsert_node".to_string(),
            key: "seo:qr-code@fr-FR".to_string(),
            created: true,
            updated_properties: vec![],
            auto_arcs_created: vec!["FOR_LOCALE".to_string()],
            execution_time_ms: 45,
            cache_invalidated: vec!["SEOKeyword:*".to_string()],
        });

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("seo:qr-code@fr-FR"));
        assert!(json.contains("FOR_LOCALE"));
    }

    #[test]
    fn test_dry_run_result_serialize() {
        let result = WriteResult::DryRun(DryRunResult {
            valid: true,
            would_create: true,
            issues: vec![],
            cypher_preview: Some("MERGE (n:SEOKeyword {key: $key})".to_string()),
            schema_context: Some(SchemaContext {
                class_name: "SEOKeyword".to_string(),
                realm: "shared".to_string(),
                layer: "knowledge".to_string(),
                content: Some("SEO keyword imported from external tools".to_string()),
                triggers: Some(vec!["seo".to_string(), "keyword".to_string()]),
                required_properties: vec!["keyword".to_string()],
                optional_properties: vec!["search_volume".to_string()],
            }),
            suggestions: vec![OntologySuggestion {
                action: "Provenance auto-injected".to_string(),
                reason: "created_by set to mcp:novanet_write".to_string(),
                example: None,
            }],
            token_estimate: 150,
        });

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("would_create"));
        assert!(json.contains("cypher_preview"));
        assert!(json.contains("schema_context"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Parameter Validation Tests
    // ═══════════════════════════════════════════════════════════════════════════════

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
            dry_run: None,
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
            dry_run: None,
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
            dry_run: None,
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
            dry_run: None,
        };
        let err = validate_params(&params).unwrap_err();
        assert!(err.to_string().contains("to_key"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Validation Issue Tests (absorbed from checker)
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_operation_params_upsert_missing_both() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: None,
            key: None,
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };
        let issues = validate_operation_params(&params);
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].code, "E001");
        assert_eq!(issues[1].code, "E002");
    }

    #[test]
    fn test_validate_operation_params_create_arc_missing_all() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };
        let issues = validate_operation_params(&params);
        assert_eq!(issues.len(), 3);
        assert_eq!(issues[0].code, "E003");
        assert_eq!(issues[1].code, "E004");
        assert_eq!(issues[2].code, "E005");
    }

    #[test]
    fn test_validate_operation_params_update_props_missing_all() {
        let params = WriteParams {
            operation: WriteOperation::UpdateProps,
            class: None,
            key: None,
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };
        let issues = validate_operation_params(&params);
        assert_eq!(issues.len(), 3);
        assert_eq!(issues[0].code, "E006");
        assert_eq!(issues[1].code, "E007");
        assert_eq!(issues[2].code, "E008");
    }

    #[test]
    fn test_validate_denomination_forms_missing() {
        let props = serde_json::Map::new();
        let issues = validate_denomination_forms("EntityNative", &props);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "W001");
    }

    #[test]
    fn test_validate_denomination_forms_incomplete() {
        let mut props = serde_json::Map::new();
        props.insert(
            "denomination_forms".to_string(),
            serde_json::json!([
                {"type": "text", "value": "code QR", "priority": 1},
                {"type": "title", "value": "Code QR", "priority": 1}
            ]),
        );

        let issues = validate_denomination_forms("EntityNative", &props);
        // Missing abbrev and url
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().all(|i| i.code == "W002"));
    }

    #[test]
    fn test_validate_denomination_forms_complete() {
        let mut props = serde_json::Map::new();
        props.insert(
            "denomination_forms".to_string(),
            serde_json::json!([
                {"type": "text", "value": "code QR", "priority": 1},
                {"type": "title", "value": "Code QR", "priority": 1},
                {"type": "abbrev", "value": "QR", "priority": 1},
                {"type": "url", "value": "code-qr", "priority": 1}
            ]),
        );

        let issues = validate_denomination_forms("EntityNative", &props);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_denomination_forms_non_entity_native() {
        let props = serde_json::Map::new();
        let issues = validate_denomination_forms("SEOKeyword", &props);
        assert!(issues.is_empty()); // No check for non-EntityNative
    }

    #[test]
    fn test_validate_denomination_forms_invalid_structure() {
        let mut props = serde_json::Map::new();
        props.insert(
            "denomination_forms".to_string(),
            serde_json::json!([
                {"value": "code QR"},  // missing type
                {"type": "title"}      // missing value
            ]),
        );

        let issues = validate_denomination_forms("EntityNative", &props);
        // E014 (missing type), E015 (missing value), plus W002 for missing types
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == CheckSeverity::Error)
            .collect();
        assert_eq!(errors.len(), 2);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Required Properties Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_validate_required_properties_all_present() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),
            trait_type: "imported".to_string(),
            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec!["search_volume".to_string()],
            ..Default::default()
        };

        let mut props = serde_json::Map::new();
        props.insert(
            "keyword".to_string(),
            serde_json::Value::String("qr code".to_string()),
        );
        props.insert(
            "slug_form".to_string(),
            serde_json::Value::String("qr-code".to_string()),
        );

        let issues = validate_required_properties(&meta, &props);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_required_properties_missing() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),
            trait_type: "imported".to_string(),
            required_properties: vec!["keyword".to_string(), "slug_form".to_string()],
            optional_properties: vec!["search_volume".to_string()],
            ..Default::default()
        };

        let mut props = serde_json::Map::new();
        props.insert(
            "keyword".to_string(),
            serde_json::Value::String("qr code".to_string()),
        );

        let issues = validate_required_properties(&meta, &props);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E011");
    }

    #[test]
    fn test_validate_required_properties_skips_system_managed() {
        let meta = ClassMetadata {
            name: "SEOKeyword".to_string(),
            realm: "shared".to_string(),
            layer: "knowledge".to_string(),
            trait_type: "imported".to_string(),
            required_properties: vec![
                "key".to_string(),
                "keyword".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ],
            ..Default::default()
        };

        let mut props = serde_json::Map::new();
        props.insert(
            "keyword".to_string(),
            serde_json::Value::String("qr code".to_string()),
        );

        // key, created_at, updated_at should be skipped (system-managed)
        let issues = validate_required_properties(&meta, &props);
        assert!(issues.is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Schema Context & Suggestions Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_build_schema_context() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            trait_type: "authored".to_string(),
            required_properties: vec!["name".to_string()],
            optional_properties: vec!["description".to_string()],
            content: Some("Locale-specific entity content".to_string()),
            triggers: Some(vec!["native".to_string(), "locale".to_string()]),
            ..Default::default()
        };

        let ctx = build_schema_context(&meta);
        assert_eq!(ctx.class_name, "EntityNative");
        assert_eq!(ctx.realm, "org");
        assert_eq!(ctx.layer, "semantic");
        assert!(ctx.content.is_some());
        assert!(ctx.triggers.is_some());
    }

    #[test]
    fn test_generate_suggestions_native_class() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            trait_type: "authored".to_string(),
            ..Default::default()
        };

        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("EntityNative".to_string()),
            key: Some("qr-code@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: Some("fr-FR".to_string()),
            dry_run: Some(true),
        };

        let suggestions = generate_suggestions(&meta, &params, &[]);
        // Should have: FOR_LOCALE, HAS_NATIVE, denomination_forms, provenance
        assert!(suggestions.len() >= 3);
        assert!(suggestions
            .iter()
            .any(|s| s.action.contains("FOR_LOCALE")));
        assert!(suggestions
            .iter()
            .any(|s| s.action.contains("HAS_NATIVE")));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Cypher Preview Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_generate_cypher_preview_upsert() {
        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("SEOKeyword".to_string()),
            key: Some("seo:test@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: Some("fr-FR".to_string()),
            dry_run: Some(true),
        };

        let preview = generate_cypher_preview(&params, None).unwrap();
        assert!(preview.contains("MERGE (n:SEOKeyword"));
        assert!(preview.contains("FOR_LOCALE"));
    }

    #[test]
    fn test_generate_cypher_preview_create_arc() {
        let params = WriteParams {
            operation: WriteOperation::CreateArc,
            class: None,
            key: None,
            arc_class: Some("TARGETS".to_string()),
            from_key: Some("a".to_string()),
            to_key: Some("b".to_string()),
            properties: None,
            locale: None,
            dry_run: Some(true),
        };

        let preview = generate_cypher_preview(&params, None).unwrap();
        assert!(preview.contains("TARGETS"));
        assert!(preview.contains("from_key"));
        assert!(preview.contains("to_key"));
    }

    #[test]
    fn test_generate_cypher_preview_update_props() {
        let params = WriteParams {
            operation: WriteOperation::UpdateProps,
            class: Some("EntityNative".to_string()),
            key: Some("qr-code@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };

        let preview = generate_cypher_preview(&params, None).unwrap();
        assert!(preview.contains("MATCH (n:EntityNative"));
        assert!(preview.contains("updated_at"));
    }

    #[test]
    fn test_generate_cypher_preview_native_with_has_native() {
        let meta = ClassMetadata {
            name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            trait_type: "authored".to_string(),
            ..Default::default()
        };

        let params = WriteParams {
            operation: WriteOperation::UpsertNode,
            class: Some("EntityNative".to_string()),
            key: Some("qr-code@fr-FR".to_string()),
            arc_class: None,
            from_key: None,
            to_key: None,
            properties: None,
            locale: None,
            dry_run: Some(true),
        };

        let preview = generate_cypher_preview(&params, Some(&meta)).unwrap();
        assert!(preview.contains("HAS_NATIVE"));
        assert!(preview.contains("Entity"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Check Issue Builder Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_check_issue_builders() {
        let error = CheckIssue::error("E001", "test error")
            .with_field("class")
            .with_hint("try this");

        assert_eq!(error.severity, CheckSeverity::Error);
        assert_eq!(error.code, "E001");
        assert_eq!(error.field, Some("class".to_string()));
        assert_eq!(error.hint, Some("try this".to_string()));

        let warning = CheckIssue::warning("W001", "test warning");
        assert_eq!(warning.severity, CheckSeverity::Warning);

        let info = CheckIssue::info("I001", "test info");
        assert_eq!(info.severity, CheckSeverity::Info);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // HAS_NATIVE Key Extraction Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_has_native_key_extraction_entity_native() {
        let key = "qr-code@fr-FR";
        let class = "EntityNative";

        assert!(class.ends_with("Native"));
        assert!(key.contains('@'));

        let entity_key = key.split('@').next().unwrap();
        assert_eq!(entity_key, "qr-code");

        let base_class = class.trim_end_matches("Native");
        assert_eq!(base_class, "Entity");
    }

    #[test]
    fn test_has_native_key_extraction_page_native() {
        let key = "homepage@es-MX";
        let class = "PageNative";

        assert!(class.ends_with("Native"));
        assert!(key.contains('@'));

        let entity_key = key.split('@').next().unwrap();
        assert_eq!(entity_key, "homepage");

        let base_class = class.trim_end_matches("Native");
        assert_eq!(base_class, "Page");
    }

    #[test]
    fn test_has_native_key_extraction_block_native() {
        let key = "head-seo-meta@ja-JP";
        let class = "BlockNative";

        assert!(class.ends_with("Native"));
        assert!(key.contains('@'));

        let entity_key = key.split('@').next().unwrap();
        assert_eq!(entity_key, "head-seo-meta");

        let base_class = class.trim_end_matches("Native");
        assert_eq!(base_class, "Block");
    }

    #[test]
    fn test_has_native_no_extraction_for_non_native() {
        let _key = "qr-code@fr-FR";
        let class = "Entity";
        assert!(!class.ends_with("Native"));
    }

    #[test]
    fn test_has_native_no_extraction_without_locale() {
        let key = "qr-code";
        let class = "EntityNative";

        assert!(class.ends_with("Native"));
        assert!(!key.contains('@'));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Class Name Validation Integration Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_valid_class_names_accepted() {
        assert!(crate::validation::is_valid_class_name("Entity"));
        assert!(crate::validation::is_valid_class_name("EntityNative"));
        assert!(crate::validation::is_valid_class_name("SEOKeyword"));
        assert!(crate::validation::is_valid_class_name("PageNative"));
        assert!(crate::validation::is_valid_class_name("BlockNative"));
    }

    #[test]
    fn test_cypher_injection_class_names_rejected() {
        assert!(!crate::validation::is_valid_class_name(
            "Entity}DETACH DELETE n"
        ));
        assert!(!crate::validation::is_valid_class_name("Entity]->(x)"));
        assert!(!crate::validation::is_valid_class_name("a:Entity"));
        assert!(!crate::validation::is_valid_class_name("123Entity"));
    }

    #[test]
    fn test_valid_arc_names_accepted() {
        assert!(crate::validation::is_valid_arc_class_name("HAS_NATIVE"));
        assert!(crate::validation::is_valid_arc_class_name("FOR_LOCALE"));
        assert!(crate::validation::is_valid_arc_class_name("BELONGS_TO"));
        assert!(crate::validation::is_valid_arc_class_name("TARGETS"));
    }

    #[test]
    fn test_cypher_injection_arc_names_rejected() {
        assert!(!crate::validation::is_valid_arc_class_name(
            "HAS_NATIVE}RETURN"
        ));
        assert!(!crate::validation::is_valid_arc_class_name("has_native"));
        assert!(!crate::validation::is_valid_arc_class_name("HAS-NATIVE"));
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // ADR-042 Provenance Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_mcp_provenance_constant() {
        assert_eq!(super::MCP_PROVENANCE, "mcp:novanet_write");
    }

    #[test]
    fn test_provenance_format_examples() {
        let valid_formats = [
            "seed:schema",
            "seed:immutable",
            "content:bootstrap",
            "user:manual",
            "user:studio",
            "nika:workflow:abc123",
            "mcp:novanet_write",
        ];

        for format in valid_formats {
            assert!(format.contains(':'), "Format '{}' should contain ':'", format);
            let parts: Vec<&str> = format.split(':').collect();
            assert!(!parts[0].is_empty(), "Source type should not be empty");
        }
    }
}
