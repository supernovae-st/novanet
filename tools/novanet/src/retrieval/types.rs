//! Core types for v10 context assembly.
//!
//! These types define the request/response protocol for context window assembly.

use serde::{Deserialize, Serialize};

/// Request for context assembly.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextRequest {
    /// Target block key
    pub block_key: String,
    /// Target locale key
    pub locale_key: String,
    /// Maximum token budget
    pub token_budget: u32,
    /// Optional temperature for semantic arcs (default: 0.3)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Optional max traversal depth override
    pub max_depth: Option<u8>,
}

fn default_temperature() -> f32 {
    0.3
}

/// Assembled context window.
#[derive(Debug, Clone, Serialize)]
pub struct ContextWindow {
    /// Nodes included in context
    pub nodes: Vec<ContextNode>,
    /// Edges included in context (for provenance)
    pub edges: Vec<ContextArc>,
    /// Total tokens used
    pub tokens_used: u32,
    /// Token budget
    pub token_budget: u32,
    /// Traversal log for debugging
    pub traversal_log: Vec<TraversalStep>,
}

/// Node in context window.
#[derive(Debug, Clone, Serialize)]
pub struct ContextNode {
    pub key: String,
    pub kind: String,
    pub realm: String,
    pub layer: String,
    pub trait_type: String,
    pub properties: serde_json::Value,
    pub token_estimate: u32,
    pub depth: u8,
}

/// Arc in context window (v10.4: renamed from ContextArc).
#[derive(Debug, Clone, Serialize)]
pub struct ContextArc {
    pub from_key: String,
    pub to_key: String,
    pub arc_kind: String,
    pub family: String,
    pub properties: serde_json::Value,
}

/// Traversal step for debugging.
#[derive(Debug, Clone, Serialize)]
pub struct TraversalStep {
    pub depth: u8,
    pub from_key: String,
    pub arc_kind: String,
    pub to_key: String,
    pub decision: TraversalDecision,
    pub reason: String,
}

/// Decision made during traversal.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraversalDecision {
    Include,
    Skip,
    BudgetExceeded,
    DepthExceeded,
    ThresholdNotMet,
}

/// Traversal mode from ArcFamily.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraversalMode {
    Eager,
    #[default]
    Lazy,
    Skip,
}

impl TraversalMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eager => "eager",
            Self::Lazy => "lazy",
            Self::Skip => "skip",
        }
    }
}

impl std::str::FromStr for TraversalMode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eager" => Ok(Self::Eager),
            "lazy" => Ok(Self::Lazy),
            "skip" => Ok(Self::Skip),
            _ => Err(format!("unknown traversal mode: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traversal_mode_from_str() {
        assert_eq!(
            "eager".parse::<TraversalMode>().unwrap(),
            TraversalMode::Eager
        );
        assert_eq!(
            "lazy".parse::<TraversalMode>().unwrap(),
            TraversalMode::Lazy
        );
        assert_eq!(
            "skip".parse::<TraversalMode>().unwrap(),
            TraversalMode::Skip
        );
        assert!("unknown".parse::<TraversalMode>().is_err());
    }

    #[test]
    fn traversal_mode_as_str() {
        assert_eq!(TraversalMode::Eager.as_str(), "eager");
        assert_eq!(TraversalMode::Lazy.as_str(), "lazy");
        assert_eq!(TraversalMode::Skip.as_str(), "skip");
    }

    #[test]
    fn default_temperature_value() {
        assert!((default_temperature() - 0.3).abs() < f32::EPSILON);
    }
}
