//! Blueprint command — comprehensive meta-graph visualization and validation.
//!
//! `novanet blueprint` replaces the basic `novanet meta` command with:
//! - Rich ASCII visualizations (flows, trees, matrices)
//! - YAML ↔ Neo4j ↔ Cypher coherence validation
//! - 10 specialized views for different needs
//!
//! # Views
//!
//! Understanding the model:
//! - `tree` — Hierarchy Realm > Layer > Kind
//! - `flow` — 6 data flow diagrams
//! - `content` — Content Model deep-dive
//! - `arcs` — All arcs grouped by family
//! - `cardinality` — 1:1, 1:N, N:M constraints
//! - `glossary` — Concept definitions
//!
//! Analyzing state:
//! - `audit` — Health check, drift detection
//! - `deps` — Dependency impact analysis
//! - `coverage` — Locale completion status
//! - `stats` — Raw numbers for CI/scripts

pub mod ascii;
pub mod sources;
pub mod validation;
pub mod views;

pub use sources::BlueprintData;
pub use validation::{ValidationIssue, ValidationResult, Severity};
pub use views::BlueprintView;
