//! Schema validation engine with auto-fix capabilities.
//!
//! This module provides:
//! - Auto-fix engine for correcting validation violations
//! - Cypher seed file validator against YAML definitions
//! - Hook system for extensibility
//! - Plugin architecture for custom validation rules

pub mod autofix;
pub mod cypher_validator;

// Re-exports for convenience
pub use autofix::{AutoFix, Change, FixAction, FixEngine, FixStrategy};
pub use cypher_validator::{
    CypherValidationIssue, CypherValidationRule, IssueSeverity, format_summary,
    validate_cypher_files,
};
