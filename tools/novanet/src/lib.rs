//! NovaNet context graph library.
//!
//! Public API for the NovaNet CLI and TUI. All core logic lives here;
//! `main.rs` is a thin entry point that parses CLI args and dispatches.

pub mod blueprint;
pub mod commands;
pub mod config;
pub mod core;
pub mod cypher;
pub mod db;
pub mod error;
pub mod facets;
pub mod generators;
pub mod output;
pub mod parsers;
#[cfg(feature = "tui")]
pub mod tui;
pub mod user_config;
pub mod validation;

pub use error::{NovaNetError, Result};
