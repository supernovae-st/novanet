//! Terminal UI for NovaNet context graph.
//!
//! Galaxy-themed mission control layout with five areas: mode tabs,
//! taxonomy tree (Realm > Layer > Kind), Kind detail pane, Cypher preview,
//! and status bar. Features: fuzzy search (`/`), edge explorer (`e`),
//! panel cycling, and faceted query filtering.

mod app;
mod dashboard;
mod detail;
mod dialogs;
mod events;
mod logo;
mod palette;
mod runtime;
mod search;
mod theme;
mod tree;
mod ui;

pub use runtime::run;
