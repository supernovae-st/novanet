//! YAML parsers for NovaNet model definitions.
//!
//! ## Core Schema Parsers (v0.12.5)
//!
//! - `realm`: Parse realm YAML files (shared, org)
//! - `layer`: Parse layer YAML files (config, semantic, output, etc.)
//! - `trait_def`: Parse trait YAML files (defined, authored, imported, generated, retrieved)
//! - `arc_family`: Parse arc family YAML files (ownership, localization, semantic, etc.)
//! - `yaml_node`: Parse node-kind YAML files (61 classes with trait validation)
//! - `arcs`: Parse arc-classes/ directory → arc definitions
//!
//! ## Legacy/Transitional Parsers
//!
//! - `organizing`: Parse taxonomy.yaml (realms, layers, traits, arc_families) — being replaced by individual files
//! - `taxonomy`: Parse taxonomy.yaml (v9.5 source of truth) — being replaced by individual files
//!
//! ## Other Parsers
//!
//! - `views`: Parse view YAML files
//! - `visual_encoding`: Parse visual-encoding.yaml (v9.5 visual system)
//! - `slugification`: Parse ATH 2-rules-slug markdown files
//! - `formatting`: Parse ATH 2-rules-formatting markdown files
//! - `expression`: Parse ATH 3-voice-lexicon markdown files
//! - `culture`: Parse ATH 4-culture-norms markdown files
//! - `market`: Parse ATH 5-market markdown files
//! - `utils`: Shared YAML loading utilities
//! - `markdown_utils`: Shared markdown parsing utilities (frontmatter, sections)

// v0.12.5: Individual taxonomy file parsers
pub mod arc_family;
pub mod layer;
pub mod realm;
// v0.19.0: trait_def removed (traits deprecated per ADR-024)

// Core schema parsers
pub mod arcs;
pub mod schema_rules;
pub mod yaml_node;

// Legacy/transitional parsers (to be replaced)
pub mod organizing;
pub mod taxonomy;

// Other parsers
pub mod culture;
pub mod expression;
pub mod formatting;
pub mod markdown_utils;
pub mod market;
pub mod slugification;
pub mod utils;
pub mod views;
pub mod visual_encoding;
