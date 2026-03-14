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

}
