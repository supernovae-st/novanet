//! Blueprint views — different perspectives on the meta-graph.
//!
//! Each view provides a specialized visualization:
//! - `default` — Rich overview (shown when no --view specified)
//! - `tree` — Hierarchical Realm > Layer > Kind
//! - `flow` — 6 data flow diagrams
//! - `content` — Content Model deep-dive
//! - `arcs` — All arcs grouped by family
//! - `cardinality` — 1:1, 1:N, N:M relationships
//! - `glossary` — Concept definitions
//! - `audit` — Health check and drift detection
//! - `deps` — Dependency impact analysis
//! - `coverage` — Locale completion status
//! - `stats` — Raw numbers for CI/scripts

pub mod default;
pub mod tree;
pub mod flow;
pub mod arcs;
pub mod stats;
pub mod glossary;
pub mod cardinality;
// TODO: Add remaining views
// pub mod content;
// pub mod audit;
// pub mod deps;
// pub mod coverage;

use clap::ValueEnum;
use crate::blueprint::BlueprintData;
use crate::output::OutputFormat;

/// Available blueprint views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum BlueprintView {
    /// Hierarchical Realm > Layer > Kind view
    Tree,
    /// 6 data flow diagrams
    Flow,
    /// Content Model deep-dive (Entity/Page/Block/Generated)
    Content,
    /// All arcs grouped by family
    Arcs,
    /// 1:1, 1:N, N:M relationship constraints
    Cardinality,
    /// Concept definitions (Realm, Layer, Trait, etc.)
    Glossary,
    /// Health check and drift detection
    Audit,
    /// Dependency impact analysis
    Deps,
    /// Locale completion status (requires Neo4j)
    Coverage,
    /// Raw numbers for CI/scripts
    Stats,
}

impl std::fmt::Display for BlueprintView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tree => write!(f, "tree"),
            Self::Flow => write!(f, "flow"),
            Self::Content => write!(f, "content"),
            Self::Arcs => write!(f, "arcs"),
            Self::Cardinality => write!(f, "cardinality"),
            Self::Glossary => write!(f, "glossary"),
            Self::Audit => write!(f, "audit"),
            Self::Deps => write!(f, "deps"),
            Self::Coverage => write!(f, "coverage"),
            Self::Stats => write!(f, "stats"),
        }
    }
}

/// Render a specific view.
pub fn render_view(data: &BlueprintData, view: BlueprintView, format: OutputFormat) -> String {
    match view {
        BlueprintView::Tree => tree::render(data),
        BlueprintView::Flow => flow::render(data),
        BlueprintView::Arcs => arcs::render(data),
        BlueprintView::Stats => stats::render(data, format),
        BlueprintView::Glossary => glossary::render(data),
        BlueprintView::Cardinality => cardinality::render(data),
        // TODO: Implement remaining views
        BlueprintView::Content => "Content view not yet implemented".to_string(),
        BlueprintView::Audit => "Audit view not yet implemented".to_string(),
        BlueprintView::Deps => "Deps view not yet implemented".to_string(),
        BlueprintView::Coverage => "Coverage view not yet implemented".to_string(),
    }
}

/// Render the default overview.
pub fn render_default(data: &BlueprintData, validate: bool) -> String {
    default::render(data, validate)
}
