//! Type definitions for novanet_write tool
//!
//! WriteOperation, CheckIssue, WriteParams, WriteResult, and related types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// =============================================================================
// ENUMS
// =============================================================================

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

// =============================================================================
// VALIDATION TYPES
// =============================================================================

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
    pub(super) fn error(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Error,
            code: code.to_string(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    pub(super) fn warning(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Warning,
            code: code.to_string(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    #[allow(dead_code)]
    pub(super) fn info(code: &str, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Info,
            code: code.to_string(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    pub(super) fn with_field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }

    pub(super) fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

// =============================================================================
// SCHEMA CONTEXT
// =============================================================================

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

// =============================================================================
// PARAMS
// =============================================================================

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

// =============================================================================
// RESULTS
// =============================================================================

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
