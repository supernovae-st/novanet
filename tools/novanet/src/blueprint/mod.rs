//! Blueprint command — comprehensive schema-graph visualization and validation.
//!
//! `novanet blueprint` replaces the basic `novanet meta` command with:
//! - Rich ASCII visualizations (flows, trees, matrices)
//! - YAML ↔ Neo4j ↔ Cypher coherence validation
//! - 7 specialized views for different needs
//!
//! # Views (v11.7)
//!
//! - `tree` — Hierarchy Realm > Layer > Kind
//! - `flow` — 6 data flow diagrams
//! - `arcs` — All arcs grouped by family
//! - `cardinality` — 1:1, 1:N, N:M constraints
//! - `glossary` — Concept definitions
//! - `stats` — Raw numbers for CI/scripts
//! - (default) — Rich overview with all sections

pub mod ascii;
pub mod sources;
pub mod validation;
pub mod views;

pub use sources::BlueprintData;
pub use validation::{Severity, ValidationIssue, ValidationResult};
pub use views::BlueprintView;
