//! Spreading Activation Configuration
//!
//! Implements the spreading activation algorithm from cognitive science for
//! context assembly in the NovaNet knowledge graph.
//!
//! ## Mathematical Model
//!
//! ```text
//! A_j(t) = δ × A_j(t-1) + Σᵢ [w_ij × A_i(t-1) × decay(t) × semantic_boost(type)]
//! ```
//!
//! Where:
//! - δ = retention_factor (how much previous activation remains)
//! - w_ij = edge weight (SEMANTIC_LINK.temperature)
//! - decay(t) = e^(-ρ × t) = exponential decay over steps
//! - semantic_boost = task-specific multiplier for semantic_type
//!
//! ## Phase Implementation
//!
//! Phase 2.1: Create SpreadingConfig struct (this file)
//! Phase 2.2: Load SpreadingConfig in State
//! Phase 3: Replace hardcoded relevance in assemble.rs
//!
//! See: `schema/models/config/spreading-activation.yaml`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Static fallback DEFAULT task modifier
/// Used when config doesn't contain a DEFAULT entry (prevents panic)
static DEFAULT_TASK_MODIFIER: LazyLock<TaskModifier> = LazyLock::new(|| TaskModifier {
    activation_threshold: Some(0.30),
    propagation_steps: Some(2),
    semantic_boosts: HashMap::new(),
    priority_filter: vec![
        "critical".to_string(),
        "high".to_string(),
        "medium".to_string(),
    ],
});

/// Spreading activation configuration
///
/// Loaded from `spreading-activation.yaml` at startup.
/// Used by `novanet_assemble` and `novanet_generate` for context assembly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadingConfig {
    /// Default parameters for spreading activation
    pub default: SpreadingDefaults,

    /// Task-specific modifiers (CTA, FAQ, HERO, PRICING, TESTIMONIAL, DEFAULT)
    pub task_modifiers: HashMap<String, TaskModifier>,

    /// Default temperature values for semantic link types
    pub semantic_link_defaults: HashMap<String, f64>,
}

/// Default spreading activation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadingDefaults {
    /// ρ - Exponential decay factor over steps (e^(-ρ × distance))
    #[serde(default = "default_decay_factor")]
    pub decay_factor: f64,

    /// δ - Activation retained between propagation steps
    #[serde(default = "default_retention_factor")]
    pub retention_factor: f64,

    /// T - Maximum number of hops (propagation steps)
    #[serde(default = "default_propagation_steps")]
    pub propagation_steps: u8,

    /// A₀ - Initial activation at seed node
    #[serde(default = "default_initial_activation")]
    pub initial_activation: f64,

    /// Minimum activation to continue spreading (edge filter)
    #[serde(default = "default_activation_threshold")]
    pub activation_threshold: f64,

    /// Minimum activation to include in final results
    #[serde(default = "default_output_threshold")]
    pub output_threshold: f64,

    /// Limit outgoing edges per node (fan effect control)
    #[serde(default = "default_max_fan_out")]
    pub max_fan_out: usize,

    /// Reduce activation for high-degree nodes
    #[serde(default = "default_fan_penalty")]
    pub fan_penalty: f64,
}

/// Task-specific spreading activation modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskModifier {
    /// Override activation threshold for this task
    #[serde(default)]
    pub activation_threshold: Option<f64>,

    /// Override propagation steps for this task
    #[serde(default)]
    pub propagation_steps: Option<u8>,

    /// Semantic type boosts (e.g., "urgency": 1.3)
    #[serde(default)]
    pub semantic_boosts: HashMap<String, f64>,

    /// Only include concepts with these priority levels
    #[serde(default)]
    pub priority_filter: Vec<String>,
}

// Default value functions for serde
fn default_decay_factor() -> f64 {
    0.01
}
fn default_retention_factor() -> f64 {
    0.5
}
fn default_propagation_steps() -> u8 {
    2
}
fn default_initial_activation() -> f64 {
    1.0
}
fn default_activation_threshold() -> f64 {
    0.3
}
fn default_output_threshold() -> f64 {
    0.1
}
fn default_max_fan_out() -> usize {
    10
}
fn default_fan_penalty() -> f64 {
    0.1
}

impl Default for SpreadingDefaults {
    fn default() -> Self {
        Self {
            decay_factor: default_decay_factor(),
            retention_factor: default_retention_factor(),
            propagation_steps: default_propagation_steps(),
            initial_activation: default_initial_activation(),
            activation_threshold: default_activation_threshold(),
            output_threshold: default_output_threshold(),
            max_fan_out: default_max_fan_out(),
            fan_penalty: default_fan_penalty(),
        }
    }
}

impl Default for SpreadingConfig {
    fn default() -> Self {
        Self {
            default: SpreadingDefaults::default(),
            task_modifiers: Self::default_task_modifiers(),
            semantic_link_defaults: Self::default_semantic_link_defaults(),
        }
    }
}

impl SpreadingConfig {
    /// Load configuration from YAML file
    ///
    /// Looks for the file at the standard path:
    /// `schema/models/config/spreading-activation.yaml`
    pub fn load_from_yaml(path: &std::path::Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path).map_err(ConfigError::IoError)?;
        serde_yaml::from_str(&content).map_err(ConfigError::ParseError)
    }

    /// Calculate relevance score using exponential decay
    ///
    /// Formula: e^(-ρ × distance)
    ///
    /// This replaces the hardcoded linear decay `1.0 / (distance + 1.0)`
    #[inline]
    pub fn calculate_relevance(&self, distance: f64) -> f64 {
        (-self.default.decay_factor * distance).exp()
    }

    /// Calculate relevance with temperature weighting
    ///
    /// Formula: e^(-ρ × distance) × temperature
    #[inline]
    pub fn calculate_relevance_with_temperature(&self, distance: f64, temperature: f64) -> f64 {
        self.calculate_relevance(distance) * temperature
    }

    /// Get task modifier for a block type, falling back to DEFAULT
    ///
    /// Falls back to static DEFAULT_TASK_MODIFIER if neither the requested
    /// block_type nor DEFAULT exists in the config (prevents panic).
    pub fn get_task_modifier(&self, block_type: &str) -> &TaskModifier {
        self.task_modifiers
            .get(block_type.to_uppercase().as_str())
            .or_else(|| self.task_modifiers.get("DEFAULT"))
            .unwrap_or(&DEFAULT_TASK_MODIFIER)
    }

    /// Get effective activation threshold for a task
    pub fn effective_activation_threshold(&self, block_type: &str) -> f64 {
        self.get_task_modifier(block_type)
            .activation_threshold
            .unwrap_or(self.default.activation_threshold)
    }

    /// Get effective propagation steps for a task
    pub fn effective_propagation_steps(&self, block_type: &str) -> u8 {
        self.get_task_modifier(block_type)
            .propagation_steps
            .unwrap_or(self.default.propagation_steps)
    }

    /// Get semantic boost for a semantic type within a task context
    pub fn get_semantic_boost(&self, block_type: &str, semantic_type: &str) -> f64 {
        self.get_task_modifier(block_type)
            .semantic_boosts
            .get(semantic_type)
            .copied()
            .unwrap_or(1.0)
    }

    /// Get default temperature for a semantic link type
    pub fn get_semantic_link_temperature(&self, link_type: &str) -> f64 {
        self.semantic_link_defaults
            .get(link_type)
            .copied()
            .unwrap_or(0.6)
    }

    /// Check if activation is above threshold (should continue spreading)
    #[inline]
    pub fn should_continue_spreading(&self, activation: f64, block_type: &str) -> bool {
        activation >= self.effective_activation_threshold(block_type)
    }

    /// Check if activation is above output threshold (should include in results)
    #[inline]
    pub fn should_include_in_output(&self, activation: f64) -> bool {
        activation >= self.default.output_threshold
    }

    /// Default task modifiers matching the YAML configuration
    fn default_task_modifiers() -> HashMap<String, TaskModifier> {
        let mut modifiers = HashMap::new();

        // CTA modifier
        modifiers.insert(
            "CTA".to_string(),
            TaskModifier {
                activation_threshold: Some(0.25),
                propagation_steps: Some(2),
                semantic_boosts: [
                    ("urgency".to_string(), 1.3),
                    ("value".to_string(), 1.2),
                    ("action".to_string(), 1.15),
                    ("is_action_on".to_string(), 1.1),
                ]
                .into_iter()
                .collect(),
                priority_filter: vec!["critical".to_string(), "high".to_string()],
            },
        );

        // FAQ modifier
        modifiers.insert(
            "FAQ".to_string(),
            TaskModifier {
                activation_threshold: Some(0.40),
                propagation_steps: Some(2),
                semantic_boosts: [
                    ("definition".to_string(), 1.3),
                    ("type_of".to_string(), 1.2),
                    ("example".to_string(), 1.1),
                ]
                .into_iter()
                .collect(),
                priority_filter: vec![
                    "critical".to_string(),
                    "high".to_string(),
                    "medium".to_string(),
                ],
            },
        );

        // HERO modifier
        modifiers.insert(
            "HERO".to_string(),
            TaskModifier {
                activation_threshold: Some(0.30),
                propagation_steps: Some(2),
                semantic_boosts: [
                    ("is_action_on".to_string(), 1.2),
                    ("includes".to_string(), 1.1),
                    ("benefit".to_string(), 1.15),
                ]
                .into_iter()
                .collect(),
                priority_filter: vec!["critical".to_string(), "high".to_string()],
            },
        );

        // PRICING modifier
        modifiers.insert(
            "PRICING".to_string(),
            TaskModifier {
                activation_threshold: Some(0.20),
                propagation_steps: Some(2),
                semantic_boosts: [
                    ("includes".to_string(), 1.3),
                    ("type_of".to_string(), 1.2),
                    ("value".to_string(), 1.1),
                    ("opposite".to_string(), 0.8),
                ]
                .into_iter()
                .collect(),
                priority_filter: vec!["critical".to_string(), "high".to_string()],
            },
        );

        // TESTIMONIAL modifier
        modifiers.insert(
            "TESTIMONIAL".to_string(),
            TaskModifier {
                activation_threshold: Some(0.35),
                propagation_steps: Some(1),
                semantic_boosts: [("related".to_string(), 1.2), ("benefit".to_string(), 1.1)]
                    .into_iter()
                    .collect(),
                priority_filter: vec![
                    "critical".to_string(),
                    "high".to_string(),
                    "medium".to_string(),
                ],
            },
        );

        // DEFAULT modifier
        modifiers.insert(
            "DEFAULT".to_string(),
            TaskModifier {
                activation_threshold: Some(0.30),
                propagation_steps: Some(2),
                semantic_boosts: HashMap::new(),
                priority_filter: vec![
                    "critical".to_string(),
                    "high".to_string(),
                    "medium".to_string(),
                ],
            },
        );

        modifiers
    }

    /// Default semantic link temperatures matching the YAML configuration
    fn default_semantic_link_defaults() -> HashMap<String, f64> {
        [
            ("is_action_on".to_string(), 0.95),
            ("has_action".to_string(), 0.90),
            ("includes".to_string(), 0.85),
            ("included_in".to_string(), 0.80),
            ("type_of".to_string(), 0.90),
            ("has_type".to_string(), 0.85),
            ("requires".to_string(), 0.80),
            ("required_by".to_string(), 0.75),
            ("related".to_string(), 0.60),
            ("opposite".to_string(), 0.40),
        ]
        .into_iter()
        .collect()
    }
}

/// Errors that can occur when loading spreading configuration
#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(serde_yaml::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error loading spreading config: {}", e),
            ConfigError::ParseError(e) => write!(f, "Parse error in spreading config: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SpreadingConfig::default();
        assert!((config.default.decay_factor - 0.01).abs() < f64::EPSILON);
        assert!((config.default.retention_factor - 0.5).abs() < f64::EPSILON);
        assert_eq!(config.default.propagation_steps, 2);
        assert!((config.default.activation_threshold - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exponential_decay_distance_0() {
        let config = SpreadingConfig::default();
        let relevance = config.calculate_relevance(0.0);
        // e^(-0.01 * 0) = e^0 = 1.0
        assert!((relevance - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exponential_decay_distance_1() {
        let config = SpreadingConfig::default();
        let relevance = config.calculate_relevance(1.0);
        // e^(-0.01 * 1) ≈ 0.99
        assert!(relevance > 0.98 && relevance < 1.0);
    }

    #[test]
    fn test_exponential_decay_distance_100() {
        let config = SpreadingConfig::default();
        let relevance = config.calculate_relevance(100.0);
        // e^(-0.01 * 100) = e^(-1) ≈ 0.368
        assert!(relevance > 0.36 && relevance < 0.38);
    }

    #[test]
    fn test_task_modifier_cta() {
        let config = SpreadingConfig::default();
        let threshold = config.effective_activation_threshold("CTA");
        assert!((threshold - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_task_modifier_faq() {
        let config = SpreadingConfig::default();
        let threshold = config.effective_activation_threshold("FAQ");
        assert!((threshold - 0.40).abs() < f64::EPSILON);
    }

    #[test]
    fn test_task_modifier_fallback_to_default() {
        let config = SpreadingConfig::default();
        let threshold = config.effective_activation_threshold("UNKNOWN_TYPE");
        assert!((threshold - 0.30).abs() < f64::EPSILON);
    }

    #[test]
    fn test_semantic_boost_cta_urgency() {
        let config = SpreadingConfig::default();
        let boost = config.get_semantic_boost("CTA", "urgency");
        assert!((boost - 1.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_semantic_boost_no_match() {
        let config = SpreadingConfig::default();
        let boost = config.get_semantic_boost("CTA", "nonexistent");
        assert!((boost - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_semantic_link_temperature() {
        let config = SpreadingConfig::default();
        let temp = config.get_semantic_link_temperature("is_action_on");
        assert!((temp - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn test_should_continue_spreading() {
        let config = SpreadingConfig::default();
        // CTA threshold is 0.25
        assert!(config.should_continue_spreading(0.30, "CTA"));
        assert!(!config.should_continue_spreading(0.20, "CTA"));
    }

    #[test]
    fn test_should_include_in_output() {
        let config = SpreadingConfig::default();
        // Output threshold is 0.1
        assert!(config.should_include_in_output(0.15));
        assert!(!config.should_include_in_output(0.05));
    }

    #[test]
    fn test_calculate_relevance_with_temperature() {
        let config = SpreadingConfig::default();
        let relevance = config.calculate_relevance_with_temperature(0.0, 0.8);
        // e^0 * 0.8 = 0.8
        assert!((relevance - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_case_insensitive_block_type() {
        let config = SpreadingConfig::default();
        // Should work with lowercase
        let threshold1 = config.effective_activation_threshold("cta");
        let threshold2 = config.effective_activation_threshold("CTA");
        assert!((threshold1 - threshold2).abs() < f64::EPSILON);
    }
}
