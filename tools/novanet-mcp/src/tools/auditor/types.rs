//! Types for novanet_audit tool
//!
//! Provides AuditParams, AuditResult, AuditIssue, and AuditTarget
//! for post-write quality audit with CSR metrics.

use crate::metrics::ConstraintSatisfactionRate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// What to audit
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuditTarget {
    /// Missing EntityNatives for target locales
    Coverage,
    /// Nodes without required arcs
    Orphans,
    /// Arcs pointing to non-existent nodes
    Integrity,
    /// Stale generated content or metrics
    Freshness,
    /// All checks combined
    All,
}

impl std::fmt::Display for AuditTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditTarget::Coverage => write!(f, "coverage"),
            AuditTarget::Orphans => write!(f, "orphans"),
            AuditTarget::Integrity => write!(f, "integrity"),
            AuditTarget::Freshness => write!(f, "freshness"),
            AuditTarget::All => write!(f, "all"),
        }
    }
}

/// Scope of audit - limit to specific class, locale, or project
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AuditScope {
    /// Specific class to audit (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// Specific locale to audit (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Project key (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

/// Parameters for novanet_audit
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AuditParams {
    /// What to audit
    pub target: AuditTarget,
    /// Scope limitation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AuditScope>,
    /// Maximum issues to return (default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Severity levels for audit issues
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditSeverity {
    /// Data integrity problem - must be fixed
    Critical,
    /// Missing expected content - should be addressed
    Warning,
    /// Informational finding - no action required
    Info,
}

impl std::fmt::Display for AuditSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditSeverity::Critical => write!(f, "critical"),
            AuditSeverity::Warning => write!(f, "warning"),
            AuditSeverity::Info => write!(f, "info"),
        }
    }
}

/// A single audit issue
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AuditIssue {
    /// Severity level
    pub severity: AuditSeverity,
    /// Category: "coverage", "orphan", "integrity", "freshness"
    pub category: String,
    /// Human-readable message
    pub message: String,
    /// Node key affected (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_key: Option<String>,
    /// Arc class involved (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_class: Option<String>,
    /// Additional details as JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl AuditIssue {
    /// Create a critical issue
    pub fn critical(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: AuditSeverity::Critical,
            category: category.into(),
            message: message.into(),
            node_key: None,
            arc_class: None,
            details: None,
        }
    }

    /// Create a warning issue
    pub fn warning(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: AuditSeverity::Warning,
            category: category.into(),
            message: message.into(),
            node_key: None,
            arc_class: None,
            details: None,
        }
    }

    /// Create an info issue
    pub fn info(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: AuditSeverity::Info,
            category: category.into(),
            message: message.into(),
            node_key: None,
            arc_class: None,
            details: None,
        }
    }

    /// Builder: add node key
    pub fn with_node_key(mut self, key: impl Into<String>) -> Self {
        self.node_key = Some(key.into());
        self
    }

    /// Builder: add arc class
    pub fn with_arc_class(mut self, arc: impl Into<String>) -> Self {
        self.arc_class = Some(arc.into());
        self
    }

    /// Builder: add details
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Summary statistics for audit
#[derive(Debug, Clone, Serialize, JsonSchema, Default)]
pub struct AuditSummary {
    /// Total issues found
    pub total_issues: u32,
    /// Critical issues count
    pub critical_count: u32,
    /// Warning issues count
    pub warning_count: u32,
    /// Info issues count
    pub info_count: u32,
    /// Nodes checked
    pub nodes_checked: u32,
    /// Arcs checked
    pub arcs_checked: u32,
}

impl AuditSummary {
    /// Create from a list of issues
    pub fn from_issues(issues: &[AuditIssue]) -> Self {
        let mut critical_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;

        for issue in issues {
            match issue.severity {
                AuditSeverity::Critical => critical_count += 1,
                AuditSeverity::Warning => warning_count += 1,
                AuditSeverity::Info => info_count += 1,
            }
        }

        Self {
            total_issues: issues.len() as u32,
            critical_count,
            warning_count,
            info_count,
            ..Default::default()
        }
    }

    /// Set nodes checked
    pub fn with_nodes_checked(mut self, count: u32) -> Self {
        self.nodes_checked = count;
        self
    }

    /// Set arcs checked
    pub fn with_arcs_checked(mut self, count: u32) -> Self {
        self.arcs_checked = count;
        self
    }
}

/// Ontology-driven insights from audit
#[derive(Debug, Clone, Serialize, JsonSchema, Default)]
pub struct OntologyInsights {
    /// Most frequently violated constraint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_violated_constraint: Option<String>,
    /// Layer/realm with highest CSR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthiest_layer: Option<String>,
    /// Layer/realm needing attention
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attention_needed: Option<String>,
    /// Traversal paths with gaps
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub traversal_gaps: Vec<String>,
}

/// Result of novanet_audit
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AuditResult {
    /// What was audited
    pub target: String,
    /// Issues found
    pub issues: Vec<AuditIssue>,
    /// Summary statistics
    pub summary: AuditSummary,
    /// Constraint Satisfaction Rate (from MMKG-RDS research)
    pub csr: ConstraintSatisfactionRate,
    /// Ontology-driven insights
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology_insights: Option<OntologyInsights>,
    /// Actionable recommendations
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recommendations: Vec<String>,
    /// Estimated token cost of the response
    pub token_estimate: u32,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl AuditResult {
    /// Create a new audit result
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            issues: Vec::new(),
            summary: AuditSummary::default(),
            csr: ConstraintSatisfactionRate::new(0, 0),
            ontology_insights: None,
            recommendations: Vec::new(),
            token_estimate: 0,
            execution_time_ms: 0,
        }
    }

    /// Builder: set issues and update summary
    pub fn with_issues(mut self, issues: Vec<AuditIssue>) -> Self {
        self.summary = AuditSummary::from_issues(&issues);
        self.issues = issues;
        self
    }

    /// Builder: set CSR
    pub fn with_csr(mut self, csr: ConstraintSatisfactionRate) -> Self {
        self.csr = csr;
        self
    }

    /// Builder: set ontology insights
    pub fn with_insights(mut self, insights: OntologyInsights) -> Self {
        self.ontology_insights = Some(insights);
        self
    }

    /// Builder: set recommendations
    pub fn with_recommendations(mut self, recs: Vec<String>) -> Self {
        self.recommendations = recs;
        self
    }

    /// Builder: set execution time
    pub fn with_execution_time(mut self, ms: u64) -> Self {
        self.execution_time_ms = ms;
        self
    }

    /// Builder: set nodes checked
    pub fn with_nodes_checked(mut self, count: u32) -> Self {
        self.summary.nodes_checked = count;
        self
    }

    /// Builder: set arcs checked
    pub fn with_arcs_checked(mut self, count: u32) -> Self {
        self.summary.arcs_checked = count;
        self
    }

    /// Builder: set token estimate
    pub fn with_token_estimate(mut self, estimate: u32) -> Self {
        self.token_estimate = estimate;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_issue_builders() {
        let issue = AuditIssue::critical("coverage", "Missing EntityNative")
            .with_node_key("qr-code")
            .with_arc_class("HAS_NATIVE");

        assert_eq!(issue.severity, AuditSeverity::Critical);
        assert_eq!(issue.category, "coverage");
        assert_eq!(issue.node_key, Some("qr-code".to_string()));
        assert_eq!(issue.arc_class, Some("HAS_NATIVE".to_string()));
    }

    #[test]
    fn test_audit_summary_from_issues() {
        let issues = vec![
            AuditIssue::critical("integrity", "Broken arc"),
            AuditIssue::critical("integrity", "Another broken arc"),
            AuditIssue::warning("coverage", "Missing native"),
            AuditIssue::info("freshness", "Stale data"),
        ];

        let summary = AuditSummary::from_issues(&issues);

        assert_eq!(summary.total_issues, 4);
        assert_eq!(summary.critical_count, 2);
        assert_eq!(summary.warning_count, 1);
        assert_eq!(summary.info_count, 1);
    }

    #[test]
    fn test_audit_result_builder() {
        let issues = vec![AuditIssue::warning("coverage", "Test")];
        let csr = ConstraintSatisfactionRate::new(95, 5);

        let result = AuditResult::new("all")
            .with_issues(issues)
            .with_csr(csr)
            .with_nodes_checked(100)
            .with_arcs_checked(200)
            .with_execution_time(50);

        assert_eq!(result.target, "all");
        assert_eq!(result.summary.total_issues, 1);
        assert_eq!(result.summary.warning_count, 1);
        assert_eq!(result.summary.nodes_checked, 100);
        assert_eq!(result.summary.arcs_checked, 200);
        assert_eq!(result.execution_time_ms, 50);
    }

    #[test]
    fn test_audit_target_display() {
        assert_eq!(format!("{}", AuditTarget::Coverage), "coverage");
        assert_eq!(format!("{}", AuditTarget::Orphans), "orphans");
        assert_eq!(format!("{}", AuditTarget::Integrity), "integrity");
        assert_eq!(format!("{}", AuditTarget::Freshness), "freshness");
        assert_eq!(format!("{}", AuditTarget::All), "all");
    }

    #[test]
    fn test_audit_severity_display() {
        assert_eq!(format!("{}", AuditSeverity::Critical), "critical");
        assert_eq!(format!("{}", AuditSeverity::Warning), "warning");
        assert_eq!(format!("{}", AuditSeverity::Info), "info");
    }
}
