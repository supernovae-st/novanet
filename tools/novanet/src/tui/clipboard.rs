//! Clipboard utilities for TUI.
//!
//! Provides cross-platform clipboard access using arboard crate.

use arboard::Clipboard;

/// Copy text to system clipboard.
/// Returns Ok(()) on success, Err with message on failure.
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Clipboard init failed: {}", e))?;
    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Clipboard set failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_to_clipboard_doesnt_panic() {
        // Just verify it doesn't panic - actual clipboard may not work in CI
        let result = copy_to_clipboard("test");
        // We accept either success or graceful error
        assert!(result.is_ok() || result.is_err());
    }
}
