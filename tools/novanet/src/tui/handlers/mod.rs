//! Mode-specific key handlers for TUI.
//!
//! Each navigation mode (Meta, Data, Audit, Nexus, Atlas) has its own handler
//! that processes mode-specific keys before falling through to global handlers.
//!
//! # Architecture
//!
//! ```text
//! App::handle_key()
//!   ├── Overlays (help, legend, search, filter, recent)
//!   ├── dispatch_mode_handler() ← Mode-specific preprocessing
//!   │   ├── AuditModeHandler (j/k cursor, r refresh)
//!   │   ├── NexusModeHandler (delegated to nexus state)
//!   │   └── AtlasModeHandler (view switching, zoom)
//!   └── Global handlers (mode switch, panel focus, tree nav, etc.)
//! ```

mod audit;
mod atlas;
mod nexus;

pub use audit::handle_audit_key;
pub use atlas::handle_atlas_key;
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
        NavMode::Audit => handle_audit_key(app, key).as_option(),
        NavMode::Nexus => handle_nexus_key(app, key).as_option(),
        NavMode::Atlas => handle_atlas_key(app, key).as_option(),
        // Meta and Data modes don't have mode-specific preprocessing
        NavMode::Meta | NavMode::Data => None,
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
