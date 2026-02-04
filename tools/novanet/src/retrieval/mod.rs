//! v10 Context Assembly Engine
//!
//! Data-driven context window assembly using meta-graph traversal rules.
//!
//! # Overview
//!
//! The Context Engine uses the meta-graph properties (traversal_depth, context_budget,
//! token_estimate from Kind; default_traversal from ArcFamily; temperature_threshold
//! from ArcKind) to autonomously assemble context windows for content generation.
//!
//! # Usage
//!
//! ```ignore
//! use novanet::retrieval::{ContextEngine, ContextRequest};
//!
//! let engine = ContextEngine::new(graph).await?;
//! let request = ContextRequest {
//!     block_key: "hero-pricing".into(),
//!     locale_key: "fr-FR".into(),
//!     token_budget: 4000,
//!     temperature: 0.3,
//!     max_depth: None,
//! };
//! let window = engine.assemble(&request).await?;
//! ```

pub mod engine;
pub mod meta;
pub mod planner;
pub mod types;

pub use engine::ContextEngine;
pub use meta::MetaGraphReader;
pub use planner::TraversalPlanner;
pub use types::*;
