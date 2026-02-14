//! Mode-specific key handlers for TUI.
//!
//! v11.7: Two navigation modes (Graph, Nexus).
//! Graph mode uses global handlers; Nexus mode has its own handler.
//!
//! # Architecture
//!
//! ```text
//! App::handle_key()
//!   ├── Overlays (help, legend, search, filter, recent)
//!   ├── dispatch_mode_handler() ← Mode-specific preprocessing
//!   │   └── NexusModeHandler (delegated to nexus state)
//!   └── Global handlers (mode switch, panel focus, tree nav, etc.)
//! ```

mod nexus;

pub use nexus::handle_nexus_key;

use crossterm::event::KeyEvent;

use super::app::{App, NavMode};

/// Result of mode-specific key handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyResult {
    /// Key was handled, consumed
    Handled,
    /// Key should fall through to global handlers
    FallThrough,
}

impl KeyResult {
    /// Convert to Option<bool> for easy integration with existing code.
    /// - `Some(true)` = Handled (return true from handle_key)
    /// - `None` = Fall through to global handlers
    pub fn as_option(self) -> Option<bool> {
        match self {
            KeyResult::Handled => Some(true),
            KeyResult::FallThrough => None,
        }
    }
}

/// Dispatch key event to mode-specific handler.
///
/// Returns `Some(true)` if handled, `None` if should fall through to global handlers.
pub fn dispatch_mode_handler(app: &mut App, key: KeyEvent) -> Option<bool> {
    match app.mode {
        NavMode::Nexus => handle_nexus_key(app, key).as_option(),
        // Graph and Views modes don't have mode-specific preprocessing
        NavMode::Graph | NavMode::Views => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_result_as_option() {
        assert_eq!(KeyResult::Handled.as_option(), Some(true));
        assert_eq!(KeyResult::FallThrough.as_option(), None);
    }
}
