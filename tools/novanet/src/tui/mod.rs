//! Terminal UI for NovaNet context graph.
//!
//! Provides an interactive tree-based browser for exploring the meta-graph
//! (Realm > Layer > Kind hierarchy) and querying data nodes.

mod app;
mod events;
mod runtime;
mod tree;
mod ui;

pub use runtime::run;
