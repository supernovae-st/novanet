//! Unicode display width utilities for terminal rendering.
//!
//! Handles CJK (2 columns), emoji, Arabic, Bengali, Thai, and other
//! scripts that require grapheme-aware width calculation.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Calculate display width in terminal columns.
/// Uses grapheme clusters to handle combining characters correctly.
/// CJK characters = 2 columns, most others = 1 column.
pub fn display_width(s: &str) -> usize {
    s.graphemes(true).map(|g| g.width()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width(" "), 1);
    }

    #[test]
    fn test_display_width_cjk() {
        // Chinese characters are 2 columns each
        assert_eq!(display_width("中文"), 4);
        assert_eq!(display_width("日本語"), 6);
        assert_eq!(display_width("한글"), 4);
    }

    #[test]
    fn test_display_width_emoji() {
        // Most emoji are 2 columns
        assert_eq!(display_width("🎉"), 2);
        assert_eq!(display_width("👍"), 2);
    }

    #[test]
    fn test_display_width_mixed() {
        // "Hello中文🎉" = 5 + 4 + 2 = 11
        assert_eq!(display_width("Hello中文🎉"), 11);
    }

    #[test]
    fn test_display_width_arabic() {
        // Arabic text (RTL doesn't affect width calculation)
        let arabic = "مرحبا";
        assert!(display_width(arabic) > 0);
    }

    #[test]
    fn test_display_width_bengali() {
        // Bengali with combining characters
        let bengali = "বাংলা";
        assert!(display_width(bengali) > 0);
    }

    #[test]
    fn test_display_width_thai() {
        // Thai with tone marks
        let thai = "ไทย";
        assert!(display_width(thai) > 0);
    }
}
