//! Code generators that replace @novanet/schema-tools.
//!
//! Generator execution order: Organizing → Kind → EdgeSchema → Layer → Mermaid → Autowire → Hierarchy → Colors → Icons
//!
//! Each generator reads parsed YAML and produces either:
//! - Cypher statements (for Neo4j seeding)
//! - TypeScript code (via MiniJinja templates)
//! - Mermaid diagrams (for documentation)

pub mod autowire;
pub mod colors;
pub mod cypher_utils;
pub mod edge_schema;
pub mod hierarchy;
pub mod icons;
pub mod kind;
pub mod layer;
pub mod mermaid;
pub mod organizing;
pub mod view_mermaid;

use std::path::Path;

/// Trait implemented by all generators.
pub trait Generator {
    /// Human-readable name for progress display.
    fn name(&self) -> &'static str;

    /// Generate output. Returns the generated content as a string.
    fn generate(&self, root: &Path) -> crate::Result<String>;
}
