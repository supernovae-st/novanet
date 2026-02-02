//! NovaNet context graph library.
//!
//! Public API for the NovaNet CLI and TUI. All core logic lives here;
//! `main.rs` is a thin entry point that parses CLI args and dispatches.

pub mod commands;
pub mod config;
pub mod db;
pub mod error;
pub mod generators;
pub mod parsers;

pub use error::{NovaNetError, Result};
