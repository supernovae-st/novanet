//! Types for novanet_check validation tool
//!
//! Provides CheckParams, CheckResult, CheckIssue, and SchemaContext
//! for pre-write validation with ontology-driven suggestions.

use crate::tools::write::WriteOperation;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Severity levels for check issues
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckSeverity {
    /// Would block write - must be fixed
    Error,
    /// Potential problem - should be reviewed
    Warning,
    /// Informational - no action required
    Info,
}

impl std::fmt::Display for CheckSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckSeverity::Error => write!(f, "error"),
            CheckSeverity::Warning => write!(f, "warning"),
            CheckSeverity::Info => write!(f, "info"),
        }
    }
}

/// A single validation issue
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CheckIssue {
    /// Severity level
    pub severity: CheckSeverity,
    /// Error code (e.g., "E001", "W001", "I001")
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Which field caused the issue (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Actionable suggestion to fix the issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

impl CheckIssue {
    /// Create an error-level issue
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Error,
            code: code.into(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    /// Create a warning-level issue
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Warning,
            code: code.into(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    /// Create an info-level issue
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: CheckSeverity::Info,
            code: code.into(),
            message: message.into(),
            field: None,
            hint: None,
        }
    }

    /// Builder: add field reference
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Builder: add hint
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

/// Parameters for novanet_check
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CheckParams {
    /// Operation to validate (same as WriteParams)
    pub operation: WriteOperation,
    /// Target class name (for upsert_node, update_props)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// Arc class name (for create_arc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_class: Option<String>,
    /// Node key (for upsert_node, update_props)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Source node key (for create_arc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_key: Option<String>,
    /// Target node key (for create_arc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_key: Option<String>,
    /// Properties to set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,
    /// Locale for auto-arc creation (e.g., "fr-FR")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// Result of novanet_check
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CheckResult {
    /// Overall validity - true if no errors (warnings allowed)
    pub valid: bool,
    /// Would this create a new node/arc (vs update existing)?
    pub would_create: bool,
    /// List of issues found during validation
    pub issues: Vec<CheckIssue>,
    /// Preview of Cypher that would be executed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cypher_preview: Option<String>,
    /// Estimated token cost of the response
    pub token_estimate: u32,
    /// Schema context explaining WHY this is valid/invalid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_context: Option<SchemaContext>,
    /// Suggested fixes based on ontology rules
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<OntologySuggestion>,
}

impl CheckResult {
    /// Create a valid result with no issues
    pub fn valid() -> Self {
        Self {
            valid: true,
            would_create: false,
            issues: Vec::new(),
            cypher_preview: None,
            token_estimate: 0,
            schema_context: None,
            suggestions: Vec::new(),
        }
    }

    /// Create an invalid result from issues
    pub fn from_issues(issues: Vec<CheckIssue>) -> Self {
        let has_errors = issues.iter().any(|i| i.severity == CheckSeverity::Error);
        Self {
            valid: !has_errors,
            would_create: false,
            issues,
            cypher_preview: None,
            token_estimate: 0,
            schema_context: None,
            suggestions: Vec::new(),
        }
    }

    /// Builder: set would_create flag
    pub fn with_would_create(mut self, would_create: bool) -> Self {
        self.would_create = would_create;
        self
    }

    /// Builder: set cypher preview
    pub fn with_cypher_preview(mut self, cypher: impl Into<String>) -> Self {
        self.cypher_preview = Some(cypher.into());
        self
    }

    /// Builder: set token estimate
    pub fn with_token_estimate(mut self, tokens: u32) -> Self {
        self.token_estimate = tokens;
        self
    }

    /// Builder: add schema context
    pub fn with_schema_context(mut self, context: SchemaContext) -> Self {
        self.schema_context = Some(context);
        self
    }

    /// Builder: add suggestions
    pub fn with_suggestions(mut self, suggestions: Vec<OntologySuggestion>) -> Self {
        self.suggestions = suggestions;
        self
    }

    /// Add a single issue
    pub fn add_issue(&mut self, issue: CheckIssue) {
        if issue.severity == CheckSeverity::Error {
            self.valid = false;
        }
        self.issues.push(issue);
    }
}

/// Ontology-driven context from :Schema:Class
///
/// Provides AI agents with context about WHY a validation passed/failed
/// based on the NovaNet self-descriptive ontology.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct SchemaContext {
    /// Human-readable description of the class
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_description: Option<String>,
    /// AI-readable context: USE/TRIGGERS/NOT/RELATES pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_context: Option<String>,
    /// Arcs that MUST be created after this node (e.g., FOR_LOCALE)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mandatory_arcs: Vec<String>,
    /// Classes this typically connects to
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related_classes: Vec<String>,
    /// Explanation of the trait (defined/authored/imported/generated/retrieved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trait_explanation: Option<String>,
}

impl SchemaContext {
    /// Create new empty schema context
    pub fn new() -> Self {
        Self {
            class_description: None,
            llm_context: None,
            mandatory_arcs: Vec::new(),
            related_classes: Vec::new(),
            trait_explanation: None,
        }
    }

    /// Builder: set class description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.class_description = Some(desc.into());
        self
    }

    /// Builder: set llm_context
    pub fn with_llm_context(mut self, ctx: impl Into<String>) -> Self {
        self.llm_context = Some(ctx.into());
        self
    }

    /// Builder: set mandatory arcs
    pub fn with_mandatory_arcs(mut self, arcs: Vec<String>) -> Self {
        self.mandatory_arcs = arcs;
        self
    }

    /// Builder: set related classes
    pub fn with_related_classes(mut self, classes: Vec<String>) -> Self {
        self.related_classes = classes;
        self
    }

    /// Builder: set trait explanation
    pub fn with_trait_explanation(mut self, explanation: impl Into<String>) -> Self {
        self.trait_explanation = Some(explanation.into());
        self
    }
}

impl Default for SchemaContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A suggestion based on ontology rules
///
/// Generated from :Schema:Class llm_context and relationship patterns.
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct OntologySuggestion {
    /// Action type: "create_arc", "add_property", "check_exists", etc.
    pub action: String,
    /// Why this is suggested (from ontology rules)
    pub reason: String,
    /// Example of how to implement the suggestion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
}

impl OntologySuggestion {
    /// Create a new suggestion
    pub fn new(action: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            reason: reason.into(),
            example: None,
        }
    }

    /// Builder: add example
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.example = Some(example.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_issue_builders() {
        let issue = CheckIssue::error("E001", "Missing required property")
            .with_field("key")
            .with_hint("Add key: 'qr-code@fr-FR'");

        assert_eq!(issue.severity, CheckSeverity::Error);
        assert_eq!(issue.code, "E001");
        assert_eq!(issue.field, Some("key".to_string()));
        assert!(issue.hint.is_some());
    }

    #[test]
    fn test_check_result_valid() {
        let result = CheckResult::valid()
            .with_would_create(true)
            .with_cypher_preview("MERGE (n:Entity {key: $key})");

        assert!(result.valid);
        assert!(result.would_create);
        assert!(result.cypher_preview.is_some());
    }

    #[test]
    fn test_check_result_from_issues() {
        let issues = vec![
            CheckIssue::error("E001", "Error"),
            CheckIssue::warning("W001", "Warning"),
        ];
        let result = CheckResult::from_issues(issues);

        assert!(!result.valid); // Has errors
        assert_eq!(result.issues.len(), 2);
    }

    #[test]
    fn test_check_result_warnings_only_is_valid() {
        let issues = vec![CheckIssue::warning("W001", "Warning only")];
        let result = CheckResult::from_issues(issues);

        assert!(result.valid); // Warnings don't block
    }

    #[test]
    fn test_schema_context_builders() {
        let ctx = SchemaContext::new()
            .with_description("LLM-generated content")
            .with_llm_context("USE: when loading localized data")
            .with_mandatory_arcs(vec!["FOR_LOCALE".to_string()])
            .with_related_classes(vec!["Entity".to_string(), "Locale".to_string()]);

        assert!(ctx.class_description.is_some());
        assert!(ctx.llm_context.is_some());
        assert_eq!(ctx.mandatory_arcs.len(), 1);
        assert_eq!(ctx.related_classes.len(), 2);
    }

    #[test]
    fn test_ontology_suggestion() {
        let suggestion = OntologySuggestion::new("create_arc", "FOR_LOCALE is required")
            .with_example("create_arc FOR_LOCALE from 'qr-code@fr-FR' to 'fr-FR'");

        assert_eq!(suggestion.action, "create_arc");
        assert!(suggestion.example.is_some());
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(format!("{}", CheckSeverity::Error), "error");
        assert_eq!(format!("{}", CheckSeverity::Warning), "warning");
        assert_eq!(format!("{}", CheckSeverity::Info), "info");
    }
}
