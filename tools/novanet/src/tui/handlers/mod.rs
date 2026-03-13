//! Mode-specific key handlers for TUI.
//!
//! Graph mode uses global handlers directly.
//!
//! # Architecture
//!
//! ```text
//! App::handle_key()
//!   ├── Overlays (help, legend, search, filter, recent)
//!   ├── dispatch_mode_handler() ← Mode-specific preprocessing
//!   │   └── Graph mode: falls through to global handlers
//!   └── Global handlers (panel focus, tree nav, etc.)
//! ```

use crossterm::event::KeyEvent;

use super::app::App;

/// Dispatch key event to mode-specific handler.
///
/// Returns `Some(true)` if handled, `None` if should fall through to global handlers.
/// Graph mode always falls through to global handlers.
pub fn dispatch_mode_handler(_app: &mut App, _key: KeyEvent) -> Option<bool> {
    // Graph mode uses global handlers directly
    None
}
