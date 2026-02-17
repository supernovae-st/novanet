//! Schema validation engine with auto-fix capabilities.
//!
//! This module provides:
//! - Auto-fix engine for correcting validation violations
//! - Hook system for extensibility
//! - Plugin architecture for custom validation rules

pub mod autofix;

// Re-exports for convenience
pub use autofix::{AutoFix, FixAction, FixStrategy, Change, FixEngine};
