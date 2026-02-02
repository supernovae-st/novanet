//! Terminal UI for NovaNet context graph.
//!
//! Galaxy-themed mission control layout with five areas: mode tabs,
//! taxonomy tree (Realm > Layer > Kind), Kind detail pane, Cypher preview,
//! and status bar. Features: fuzzy search (`/`), edge explorer (`e`),
//! CRUD dialogs, command palette, boot animation, and visual effects.

mod app;
mod boot;
mod dashboard;
mod detail;
mod dialogs;
mod effects;
mod events;
mod logo;
mod onboarding;
mod palette;
mod runtime;
mod search;
mod theme;
mod tree;
mod ui;

pub use runtime::run;
