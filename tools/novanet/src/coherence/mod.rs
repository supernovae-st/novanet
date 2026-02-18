//! YAML ↔ Neo4j property coherence module.
//!
//! This module implements Axes A–J from the YAML↔Neo4j coherence plan:
//!
//! | Axis | What |
//! |------|------|
//! | A | REMOVE props in Neo4j absent from YAML (orphans) |
//! | B | ADD required YAML props absent from Neo4j |
//! | C | FIX type mismatches (float→int, string→datetime) |
//! | D | VALIDATE enum values not listed in YAML enum |
//! | E | VERIFY composite key matches pattern |
//! | F | CONSISTENT heterogeneous prop sets across instances |
//! | G | ENFORCE Neo4j 5 type constraints (IS :: FLOAT, etc.) |
//! | H | ORDER REMOVE+SET canonical BLOC 4 order |
//! | I | CI cargo run schema coherence --check in seed.sh |
//! | J | ARCS on-arc properties drift (169 arc types) |
//!
//! ## Architecture
//!
//! ```
//! coherence/
//!   drift.rs     → DriftKind, DriftReport, NodeDrift, ArcDrift (data types)
//!   type_map.rs  → TYPE_MAP, YamlPropType (YAML→Neo4j type mapping)
//!   generator.rs → CoherenceGenerator (emit idempotent Cypher scripts)
//!   validator.rs → validate_node_coherence() (pure diff function)
//! ```

pub mod drift;
pub mod generator;
pub mod type_map;
pub mod validator;

pub use drift::{ArcDrift, DriftKind, DriftReport, NodeDrift, PropertyDrift};
pub use generator::{CoherenceGenerator, CoherenceScript};
pub use type_map::{YamlPropType, TYPE_MAP};
pub use validator::{Neo4jPropSnapshot, YamlPropDef, validate_node_coherence};
