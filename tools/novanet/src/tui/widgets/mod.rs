//! Reusable TUI widget components.
//!
//! v0.18.3: Extracted from ui/mod.rs to reduce duplication.
//! v0.20.1: Added Badge widget for consistent `[label]` rendering.

mod badge;
mod panel;
mod scrollable;

pub use badge::Badge;
pub use panel::FocusablePanel;
pub use scrollable::ScrollState;
