//! YAML parsers for NovaNet model definitions.
//!
//! - `yaml_node`: Parse 35 node YAML files with locale_behavior validation
//! - `relations`: Parse relations.yaml (list format + family)
//! - `organizing`: Parse organizing-principles.yaml (realms, layers, traits, edge families)

pub mod organizing;
pub mod relations;
pub mod views;
pub mod yaml_node;
