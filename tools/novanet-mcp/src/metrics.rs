//! Quality metrics for knowledge graph validation
//!
//! Based on MMKG-RDS framework: https://arxiv.org/html/2602.23632v1
//! Implements Constraint Satisfaction Rate (CSR) and related metrics
//! for measuring knowledge graph quality.
//!
//! v0.17.0: Added for novanet_check/audit tool support.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Constraint Satisfaction Rate (CSR)
///
/// CSR = (triples satisfying constraints) / (total triples checked)
///
/// A CSR of 1.0 means perfect constraint satisfaction.
/// A CSR of 0.85 means 15% of checked triples violate constraints.
///
/// Research basis: MMKG-RDS framework for multidimensional quality scoring.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConstraintSatisfactionRate {
    /// CSR value between 0.0 and 1.0
    pub rate: f64,
    /// Number of triples/constraints that passed validation
    pub satisfied_count: u32,
    /// Number of triples/constraints that failed validation
    pub violated_count: u32,
    /// Total number of constraints checked (satisfied + violated)
    pub total_checked: u32,
    /// Human-readable list of constraints that were evaluated
    pub constraints_checked: Vec<String>,
}

impl ConstraintSatisfactionRate {
    /// Create a new CSR from satisfied and violated counts
    pub fn new(satisfied: u32, violated: u32) -> Self {
        let total = satisfied + violated;
        let rate = if total > 0 {
            satisfied as f64 / total as f64
        } else {
            1.0 // No violations if nothing checked
        };
        Self {
            rate,
            satisfied_count: satisfied,
            violated_count: violated,
            total_checked: total,
            constraints_checked: Vec::new(),
        }
    }

    /// Builder pattern: add constraint descriptions
    pub fn with_constraints(mut self, constraints: Vec<String>) -> Self {
        self.constraints_checked = constraints;
        self
    }

    /// Add a single constraint description
    pub fn add_constraint(&mut self, constraint: impl Into<String>) {
        self.constraints_checked.push(constraint.into());
    }

    /// Combine multiple CSRs into one aggregate CSR
    ///
    /// Useful when auditing multiple layers or realms and combining results.
    pub fn merge(csrs: &[ConstraintSatisfactionRate]) -> Self {
        let satisfied: u32 = csrs.iter().map(|c| c.satisfied_count).sum();
        let violated: u32 = csrs.iter().map(|c| c.violated_count).sum();
        let constraints: Vec<String> = csrs
            .iter()
            .flat_map(|c| c.constraints_checked.clone())
            .collect();
        Self::new(satisfied, violated).with_constraints(constraints)
    }

    /// Check if CSR is above acceptable threshold
    ///
    /// Default threshold of 0.95 means 95% constraint satisfaction is acceptable.
    pub fn is_acceptable(&self, threshold: f64) -> bool {
        self.rate >= threshold
    }

    /// Get a severity level based on CSR value
    ///
    /// - 0.95+ : Healthy
    /// - 0.85-0.95: Warning
    /// - <0.85: Critical
    pub fn severity(&self) -> CsrSeverity {
        if self.rate >= 0.95 {
            CsrSeverity::Healthy
        } else if self.rate >= 0.85 {
            CsrSeverity::Warning
        } else {
            CsrSeverity::Critical
        }
    }
}

impl Default for ConstraintSatisfactionRate {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Severity level based on CSR value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CsrSeverity {
    /// CSR >= 0.95 - Graph is in good shape
    Healthy,
    /// CSR 0.85-0.95 - Some issues need attention
    Warning,
    /// CSR < 0.85 - Significant constraint violations
    Critical,
}

/// Quality metrics for a specific graph layer or realm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerMetrics {
    /// Layer name (e.g., "semantic", "knowledge")
    pub layer: String,
    /// Realm name (e.g., "shared", "org")
    pub realm: String,
    /// CSR for this layer
    pub csr: ConstraintSatisfactionRate,
    /// Number of nodes checked in this layer
    pub nodes_checked: u32,
    /// Number of arcs checked in this layer
    pub arcs_checked: u32,
}

impl LayerMetrics {
    /// Create new layer metrics
    pub fn new(layer: impl Into<String>, realm: impl Into<String>) -> Self {
        Self {
            layer: layer.into(),
            realm: realm.into(),
            csr: ConstraintSatisfactionRate::default(),
            nodes_checked: 0,
            arcs_checked: 0,
        }
    }

    /// Update CSR values
    pub fn with_csr(mut self, satisfied: u32, violated: u32) -> Self {
        self.csr = ConstraintSatisfactionRate::new(satisfied, violated);
        self
    }

    /// Update node/arc counts
    pub fn with_counts(mut self, nodes: u32, arcs: u32) -> Self {
        self.nodes_checked = nodes;
        self.arcs_checked = arcs;
        self
    }
}

/// Overall audit summary with aggregated metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditSummary {
    /// Total issues found
    pub total_issues: u32,
    /// Critical (error) issues count
    pub critical_count: u32,
    /// Warning issues count
    pub warning_count: u32,
    /// Info issues count
    pub info_count: u32,
    /// Total nodes checked across all layers
    pub nodes_checked: u32,
    /// Total arcs checked across all layers
    pub arcs_checked: u32,
    /// Overall CSR across all layers
    pub overall_csr: ConstraintSatisfactionRate,
    /// Per-layer metrics breakdown
    pub layer_metrics: Vec<LayerMetrics>,
}

impl AuditSummary {
    /// Create new empty summary
    pub fn new() -> Self {
        Self::default()
    }

    /// Add layer metrics and update totals
    pub fn add_layer(&mut self, metrics: LayerMetrics) {
        self.nodes_checked += metrics.nodes_checked;
        self.arcs_checked += metrics.arcs_checked;
        self.layer_metrics.push(metrics);
        // Recalculate overall CSR
        self.overall_csr = ConstraintSatisfactionRate::merge(
            &self
                .layer_metrics
                .iter()
                .map(|m| m.csr.clone())
                .collect::<Vec<_>>(),
        );
    }

    /// Increment issue counts by severity
    pub fn add_issue(&mut self, severity: &str) {
        self.total_issues += 1;
        match severity.to_lowercase().as_str() {
            "error" | "critical" => self.critical_count += 1,
            "warning" | "warn" => self.warning_count += 1,
            _ => self.info_count += 1,
        }
    }

    /// Get the layer with lowest CSR (needs most attention)
    pub fn weakest_layer(&self) -> Option<&LayerMetrics> {
        self.layer_metrics
            .iter()
            // Use total_cmp for safe f64 comparison (handles NaN consistently)
            .min_by(|a, b| a.csr.rate.total_cmp(&b.csr.rate))
    }

    /// Get the layer with highest CSR (healthiest)
    pub fn healthiest_layer(&self) -> Option<&LayerMetrics> {
        self.layer_metrics
            .iter()
            // Use total_cmp for safe f64 comparison (handles NaN consistently)
            .max_by(|a, b| a.csr.rate.total_cmp(&b.csr.rate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csr_perfect() {
        let csr = ConstraintSatisfactionRate::new(100, 0);
        assert_eq!(csr.rate, 1.0);
        assert!(csr.is_acceptable(0.95));
        assert_eq!(csr.severity(), CsrSeverity::Healthy);
    }

    #[test]
    fn test_csr_with_violations() {
        let csr = ConstraintSatisfactionRate::new(85, 15);
        assert!((csr.rate - 0.85).abs() < 0.001);
        assert!(!csr.is_acceptable(0.95));
        assert_eq!(csr.severity(), CsrSeverity::Warning);
    }

    #[test]
    fn test_csr_critical() {
        let csr = ConstraintSatisfactionRate::new(70, 30);
        assert!((csr.rate - 0.70).abs() < 0.001);
        assert_eq!(csr.severity(), CsrSeverity::Critical);
    }

    #[test]
    fn test_csr_empty() {
        let csr = ConstraintSatisfactionRate::new(0, 0);
        assert_eq!(csr.rate, 1.0); // No violations if nothing checked
        assert_eq!(csr.total_checked, 0);
    }

    #[test]
    fn test_csr_merge() {
        let csr1 =
            ConstraintSatisfactionRate::new(50, 5).with_constraints(vec!["FOR_LOCALE".to_string()]);
        let csr2 =
            ConstraintSatisfactionRate::new(40, 5).with_constraints(vec!["HAS_NATIVE".to_string()]);
        let merged = ConstraintSatisfactionRate::merge(&[csr1, csr2]);
        assert_eq!(merged.satisfied_count, 90);
        assert_eq!(merged.violated_count, 10);
        assert_eq!(merged.rate, 0.9);
        assert_eq!(merged.constraints_checked.len(), 2);
    }

    #[test]
    fn test_csr_with_constraints() {
        let csr = ConstraintSatisfactionRate::new(95, 5).with_constraints(vec![
            "EntityNative:FOR_LOCALE (mandatory)".to_string(),
            "BlockNative:FOR_LOCALE (mandatory)".to_string(),
        ]);
        assert_eq!(csr.constraints_checked.len(), 2);
        assert!(csr.constraints_checked[0].contains("EntityNative"));
    }

    #[test]
    fn test_layer_metrics() {
        let metrics = LayerMetrics::new("semantic", "org")
            .with_csr(90, 10)
            .with_counts(50, 100);

        assert_eq!(metrics.layer, "semantic");
        assert_eq!(metrics.realm, "org");
        assert_eq!(metrics.csr.rate, 0.9);
        assert_eq!(metrics.nodes_checked, 50);
        assert_eq!(metrics.arcs_checked, 100);
    }

    #[test]
    fn test_audit_summary() {
        let mut summary = AuditSummary::new();

        summary.add_layer(
            LayerMetrics::new("semantic", "org")
                .with_csr(90, 10)
                .with_counts(50, 100),
        );
        summary.add_layer(
            LayerMetrics::new("knowledge", "shared")
                .with_csr(100, 0)
                .with_counts(30, 50),
        );

        assert_eq!(summary.nodes_checked, 80);
        assert_eq!(summary.arcs_checked, 150);
        assert_eq!(summary.layer_metrics.len(), 2);
        assert!(summary.overall_csr.rate > 0.9);
    }

    #[test]
    fn test_audit_summary_layers() {
        let mut summary = AuditSummary::new();

        summary.add_layer(
            LayerMetrics::new("semantic", "org")
                .with_csr(80, 20)
                .with_counts(50, 100),
        );
        summary.add_layer(
            LayerMetrics::new("knowledge", "shared")
                .with_csr(100, 0)
                .with_counts(30, 50),
        );

        let weakest = summary.weakest_layer().unwrap();
        assert_eq!(weakest.layer, "semantic");

        let healthiest = summary.healthiest_layer().unwrap();
        assert_eq!(healthiest.layer, "knowledge");
    }

    #[test]
    fn test_audit_summary_issues() {
        let mut summary = AuditSummary::new();
        summary.add_issue("error");
        summary.add_issue("error");
        summary.add_issue("warning");
        summary.add_issue("info");

        assert_eq!(summary.total_issues, 4);
        assert_eq!(summary.critical_count, 2);
        assert_eq!(summary.warning_count, 1);
        assert_eq!(summary.info_count, 1);
    }
}
