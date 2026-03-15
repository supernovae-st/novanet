//! Text processing utilities for TUI rendering.
//!
//! Pure functions for text wrapping, truncation, and spinner animation.
//! Consolidated from `ui/mod.rs` and `info/mod.rs` to avoid duplication.

use super::super::unicode::truncate_to_width;
#[cfg(test)]
use super::super::unicode::truncate_start_to_width;

/// Spinner animation speed divisor (higher = slower animation).
const SPINNER_SPEED_DIVISOR: usize = 2;

/// Animated spinner braille frames for loading states.
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Wrap text to lines of max `width` characters, returning owned Strings.
/// Uses char indices instead of collecting to Vec<char> for efficiency.
pub(crate) fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = text.char_indices().peekable();
    while chars.peek().is_some() {
        let start = chars.peek().map(|(i, _)| *i).unwrap_or(0);
        let mut end = start;
        let mut count = 0;
        while let Some((idx, c)) = chars.peek() {
            if count >= width {
                break;
            }
            end = *idx + c.len_utf8();
            count += 1;
            chars.next();
        }
        if start < end {
            result.push(text[start..end].to_string());
        }
    }
    result
}

/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "..." if truncated. Handles CJK, emoji, and combining characters.
pub(crate) fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}

/// Safely truncate a UTF-8 string from the START, keeping last N columns.
/// Prepends "..." if truncated. Used for breadcrumbs where the end is most relevant.
#[cfg(test)]
fn truncate_start(s: &str, max_width: usize) -> String {
    truncate_start_to_width(s, max_width)
}

/// Get the current spinner frame based on tick counter.
/// Cycles through braille patterns for smooth animation.
pub(crate) fn spinner(tick: u16) -> &'static str {
    SPINNER_FRAMES[(tick as usize / SPINNER_SPEED_DIVISOR) % SPINNER_FRAMES.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // truncate_str tests
    // =========================================================================

    #[test]
    fn test_truncate_str_ascii_under_limit() {
        assert_eq!(truncate_str("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_str_ascii_at_limit() {
        assert_eq!(truncate_str("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_str_ascii_over_limit() {
        // Width-based: "hell" (4 cols) + "..." (1 col) = 5 cols
        assert_eq!(truncate_str("hello world", 5), "hell\u{2026}");
    }

    #[test]
    fn test_truncate_str_utf8_bengali() {
        // Bengali: "..." - this caused the original panic
        let bengali = "\u{09ac}\u{09be}\u{0982}\u{09b2}\u{09be} (\u{09ac}\u{09be}\u{0982}\u{09b2}\u{09be}\u{09a6}\u{09c7}\u{09b6})";
        // Should not panic even when truncating in the middle of multi-byte chars
        let result = truncate_str(bengali, 5);
        // Width-based truncation with "..." suffix
        assert!(result.ends_with('\u{2026}'));
    }

    #[test]
    fn test_truncate_str_utf8_emoji() {
        let emoji = "Hello \u{1f44b}\u{1f3fb} World \u{1f30d}";
        let result = truncate_str(emoji, 8);
        // Width-based truncation uses "..." (single char ellipsis)
        assert!(result.ends_with('\u{2026}'));
    }

    #[test]
    fn test_truncate_str_chinese() {
        let chinese = "\u{4f60}\u{597d}\u{4e16}\u{754c}\u{8fd9}\u{662f}\u{4e2d}\u{6587}\u{6d4b}\u{8bd5}";
        let result = truncate_str(chinese, 3);
        assert_eq!(result, "\u{4f60}\u{2026}"); // 你(2) + ...(1) = 3 cols
    }

    #[test]
    fn test_truncate_str_empty() {
        assert_eq!(truncate_str("", 10), "");
    }

    // =========================================================================
    // truncate_start tests (UTF-8 safe start truncation for breadcrumbs)
    // =========================================================================

    #[test]
    fn test_truncate_start_under_limit() {
        assert_eq!(truncate_start("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_start_at_limit() {
        assert_eq!(truncate_start("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_start_over_limit() {
        // Width-based: "..." (1 col) + "orld" (4 cols) = 5 cols
        assert_eq!(truncate_start("hello world", 5), "\u{2026}orld");
    }

    #[test]
    fn test_truncate_start_utf8_arrows() {
        // This is the actual bug case: "Shared -> Org" with -> being 3 bytes
        let s = "Shared \u{2192} Org Configuration \u{2192} Slugification";
        let result = truncate_start(s, 20);
        // Should keep last 20 chars without panicking
        assert!(result.starts_with('\u{2026}'));
        assert!(result.chars().count() <= 21); // 20 + ellipsis
    }

    #[test]
    fn test_truncate_start_utf8_emoji() {
        // "... Hello ... World" - emojis are 4 bytes each
        let s = "\u{1f389} Hello \u{1f389} World";
        let result = truncate_start(s, 8);
        assert!(result.starts_with('\u{2026}'));
        // Should not panic on multi-byte boundaries
    }

    #[test]
    fn test_truncate_start_empty() {
        assert_eq!(truncate_start("", 10), "");
    }

    // =========================================================================
    // Spinner tests
    // =========================================================================

    #[test]
    fn test_spinner_cycles_through_frames() {
        // Spinner should return different chars for different ticks
        let frames: Vec<&str> = (0..20).map(spinner).collect();
        // Check that we get braille characters
        assert!(frames.iter().all(|f| f.chars().all(|c| !c.is_ascii())));
    }
}
