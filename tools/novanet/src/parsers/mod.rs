//! YAML parsers for NovaNet model definitions.
//!
//! - `yaml_node`: Parse node-kind YAML files (44 kinds with trait validation)
//! - `arcs`: Parse relations.yaml → arc definitions (ArcFamily, ArcDef)
//! - `organizing`: Parse taxonomy.yaml (realms, layers, traits, arc_families)
//! - `taxonomy`: Parse taxonomy.yaml (v9.5 source of truth)
//! - `views`: Parse view YAML files
//! - `visual_encoding`: Parse visual-encoding.yaml (v9.5 visual system)
//! - `locale_rules`: Parse locale rules markdown files (formatting, slugification, adaptation)
//! - `utils`: Shared YAML loading utilities

pub mod arcs;
pub mod locale_rules;
pub mod organizing;
pub mod taxonomy;
pub mod utils;
pub mod views;
pub mod visual_encoding;
pub mod yaml_node;
