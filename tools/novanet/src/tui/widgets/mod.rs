//! Reusable TUI widget components.
//!
//! v0.18.3: Extracted from ui/mod.rs to reduce duplication.
//! v0.20.1: Added Badge widget for consistent `[label]` rendering.
//! v0.20.1: Added ProgressBar widget for consistent bar rendering.

mod badge;
mod panel;
mod progress_bar;
mod scrollable;

pub use badge::Badge;
pub use panel::FocusablePanel;
pub use progress_bar::ProgressBar;
pub use scrollable::ScrollState;
