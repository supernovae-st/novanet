//! YAML parsers for NovaNet model definitions.
//!
//! - `yaml_node`: Parse 44 node YAML files with trait validation
//! - `relations`: Parse relations.yaml (list format + family) — legacy v9
//! - `organizing`: Parse organizing-principles.yaml (realms, layers, traits, arc families) — legacy v9
//! - `taxonomy`: Parse taxonomy.yaml (v9.5 replacement for organizing-principles.yaml)

pub mod organizing;
pub mod relations;
pub mod taxonomy;
pub mod views;
pub mod yaml_node;
