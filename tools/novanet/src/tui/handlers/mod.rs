//! Mode-specific key handlers for TUI.
//!
//! v0.20.0: Two modes -- Graph (global handlers) and Flow (dedicated handler).

mod flow;

pub use flow::handle_flow_key;

use crossterm::event::KeyEvent;

use super::app::{App, NavMode};

/// Result of mode-specific key handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyResult {
    Handled,
    FallThrough,
}

impl KeyResult {
    pub fn as_option(self) -> Option<bool> {
        match self {
            KeyResult::Handled => Some(true),
            KeyResult::FallThrough => None,
        }
    }
}

/// Dispatch key event to mode-specific handler.
pub fn dispatch_mode_handler(app: &mut App, key: KeyEvent) -> Option<bool> {
    match app.mode {
        NavMode::Flow => handle_flow_key(app, key).as_option(),
        NavMode::Graph => None,
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
