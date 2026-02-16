//! Code generators that replace @novanet/schema-tools.
//!
//! Generator execution order: Organizing → NodeClass → ArcClass → Layer → Mermaid → Autowire → Hierarchy → Colors → Icons → VisualEncoding → TuiIcons → TuiColors
//!
//! Each generator reads parsed YAML and produces either:
//! - Cypher statements (for Neo4j seeding)
//! - TypeScript code (via MiniJinja templates)
//! - Rust code (via string building)
//! - Mermaid diagrams (for documentation)

pub mod arc_class;
pub mod autowire;
pub mod colors;
pub mod culture;
pub mod cypher_utils;
pub mod expression;
pub mod formatting;
pub mod hierarchy;
pub mod icons;
pub mod layer;
pub mod market;
pub mod mermaid;
pub mod node_class;
pub mod organizing;
pub mod slugification;
#[cfg(test)]
pub mod test_utils;
pub mod tui_colors;
pub mod tui_icons;
pub mod view_mermaid;
// views module removed in v0.12.5 — views.yaml loaded dynamically by ViewLoader.ts (Studio) and nexus/views.rs (TUI)
pub mod visual_encoding;

use std::path::Path;

/// Trait implemented by all generators.
pub trait Generator {
    /// Human-readable name for progress display.
    fn name(&self) -> &'static str;

    /// Generate output. Returns the generated content as a string.
    fn generate(&self, root: &Path) -> crate::Result<String>;
}
