//! Spreading activation module
//!
//! This module contains the spreading activation algorithm implementation
//! for assembling context from the NovaNet knowledge graph.
//!
//! ## Modules
//!
//! - `spreading` - Spreading activation configuration and algorithms

pub mod spreading;

pub use spreading::{SpreadingConfig, TaskModifier};
