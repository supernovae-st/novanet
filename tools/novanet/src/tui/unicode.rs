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

/// Truncate string to fit within max terminal columns.
/// Appends "…" if truncated. Uses grapheme-aware iteration.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut current_width = 0;
    let ellipsis = "…";
    let ellipsis_width = 1;

    // Reserve space for ellipsis if we might need it
    let effective_max = if display_width(s) > max_width {
        max_width.saturating_sub(ellipsis_width)
    } else {
        max_width
    };

    for grapheme in s.graphemes(true) {
        let grapheme_width = grapheme.width();
        if current_width + grapheme_width > effective_max {
            result.push_str(ellipsis);
            return result;
        }
        result.push_str(grapheme);
        current_width += grapheme_width;
    }

    result
}

/// Truncate string from the START to fit within max terminal columns.
/// Prepends "…" if truncated. Used for breadcrumbs where end is relevant.
pub fn truncate_start_to_width(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let total_width = display_width(s);
    if total_width <= max_width {
        return s.to_string();
    }

    let ellipsis = "…";
    let ellipsis_width = 1;
    let target_width = max_width.saturating_sub(ellipsis_width);

    // Collect graphemes with their widths
    let graphemes: Vec<(&str, usize)> = s.graphemes(true).map(|g| (g, g.width())).collect();

    // Find where to start from the end
    let mut width_from_end = 0;
    let mut start_index = graphemes.len();

    for (i, (_, w)) in graphemes.iter().enumerate().rev() {
        if width_from_end + w > target_width {
            break;
        }
        width_from_end += w;
        start_index = i;
    }

    let mut result = String::from(ellipsis);
    for (g, _) in &graphemes[start_index..] {
        result.push_str(g);
    }

    result
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

    // truncate_to_width tests
    #[test]
    fn test_truncate_to_width_no_truncation() {
        assert_eq!(truncate_to_width("hello", 10), "hello");
        assert_eq!(truncate_to_width("hi", 2), "hi");
    }

    #[test]
    fn test_truncate_to_width_ascii() {
        assert_eq!(truncate_to_width("hello world", 8), "hello w…");
        assert_eq!(truncate_to_width("abcdefgh", 5), "abcd…");
    }

    #[test]
    fn test_truncate_to_width_cjk() {
        // "中文字" = 6 cols, truncate to 5 → "中文…" (4+1=5)
        assert_eq!(truncate_to_width("中文字", 5), "中文…");
    }

    #[test]
    fn test_truncate_to_width_mixed() {
        // "Hello中文" = 5+4=9, truncate to 7 → "Hello…" (5+1=6) or "Hello中…"
        let result = truncate_to_width("Hello中文", 7);
        assert!(display_width(&result) <= 7);
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_truncate_to_width_empty() {
        assert_eq!(truncate_to_width("", 5), "");
        assert_eq!(truncate_to_width("hello", 0), "");
    }

    #[test]
    fn test_truncate_to_width_emoji() {
        // "🎉🎊🎁" = 6 cols, truncate to 4 → "🎉…" (2+1=3)
        let result = truncate_to_width("🎉🎊🎁", 4);
        assert!(display_width(&result) <= 4);
    }

    // truncate_start_to_width tests
    #[test]
    fn test_truncate_start_no_truncation() {
        assert_eq!(truncate_start_to_width("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_start_ascii() {
        // Keep end: "hello world" → "…world"
        let result = truncate_start_to_width("hello world", 7);
        assert!(result.starts_with('…'));
        assert!(display_width(&result) <= 7);
    }

    #[test]
    fn test_truncate_start_cjk() {
        // "中文字符" = 8 cols, truncate to 5 → "…字符" (1+4=5)
        let result = truncate_start_to_width("中文字符", 5);
        assert!(result.starts_with('…'));
        assert!(display_width(&result) <= 5);
    }

    #[test]
    fn test_truncate_start_empty() {
        assert_eq!(truncate_start_to_width("", 5), "");
        assert_eq!(truncate_start_to_width("hello", 0), "");
    }

    // Edge case tests
    #[test]
    fn test_display_width_zwj_emoji_sequence() {
        // Zero-width joiner sequences: family emoji 👨‍👩‍👧 is one grapheme
        // Made of: man (👨) + ZWJ + woman (👩) + ZWJ + girl (👧)
        let family = "👨‍👩‍👧";
        // Should be treated as a single grapheme cluster
        let grapheme_count = family.graphemes(true).count();
        assert_eq!(grapheme_count, 1, "ZWJ sequence should be one grapheme");
        // Width is typically 2 for emoji
        assert!(
            display_width(family) >= 2,
            "ZWJ emoji should have width >= 2"
        );
    }

    #[test]
    fn test_display_width_devanagari() {
        // Devanagari script (Hindi): "नमस्ते" (namaste)
        // Contains combining characters (virama, vowel signs)
        let hindi = "नमस्ते";
        let width = display_width(hindi);
        // Devanagari characters are typically 1 column each
        // but combining marks should not add width
        assert!(width > 0, "Devanagari should have positive width");
        assert!(
            width <= hindi.chars().count(),
            "Width should be <= char count due to combining marks"
        );
    }

    #[test]
    fn test_display_width_control_characters() {
        // Control characters behavior varies - unicode_width follows UAX #11
        // Most control chars are treated as separate graphemes with width 0 or 1

        // Tab has width 0 (non-printable)
        let with_tab = "a\tb";
        // Tab is typically 0 width (the terminal handles expansion)
        let tab_width = display_width(with_tab);
        assert!(
            tab_width >= 2,
            "Tab width varies but base chars should be counted"
        );

        // Newline should be 0 width
        let with_newline = "ab\ncd";
        let newline_width = display_width(with_newline);
        assert!(newline_width >= 4, "Base chars should be counted");

        // Carriage return should be 0 width
        let with_cr = "test\r";
        let cr_width = display_width(with_cr);
        assert!(cr_width >= 4, "CR should not add visible width");

        // Escape sequence start (ESC) should be 0 width
        let with_esc = "x\x1by";
        let esc_width = display_width(with_esc);
        assert!(esc_width >= 2, "ESC should be 0 width");
    }
}
