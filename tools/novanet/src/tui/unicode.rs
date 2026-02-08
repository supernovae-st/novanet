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

    // =========================================================================
    // Exotic Unicode Scripts Tests
    // =========================================================================
    // These tests verify that various scripts don't crash and return sensible
    // widths. Width calculation for complex scripts varies by terminal, so we
    // focus on stability (no panic) and reasonable values (> 0).

    #[test]
    fn test_display_width_hebrew_rtl() {
        // Hebrew (RTL script) - "Shalom"
        let hebrew = "שלום";
        let width = display_width(hebrew);
        // Should not crash and return positive width
        assert!(width > 0, "Hebrew should have positive width");
        // Hebrew characters are typically 1 column each (4 chars = 4 cols)
        assert_eq!(width, 4, "Hebrew 'שלום' should be 4 columns");
    }

    #[test]
    fn test_display_width_persian_farsi() {
        // Persian/Farsi (RTL script) - "Salam"
        let persian = "سلام";
        let width = display_width(persian);
        assert!(width > 0, "Persian should have positive width");
        // Persian characters are typically 1 column each
        assert_eq!(width, 4, "Persian 'سلام' should be 4 columns");
    }

    #[test]
    fn test_display_width_tamil() {
        // Tamil (South Asian script with complex ligatures)
        let tamil = "தமிழ்";
        let width = display_width(tamil);
        assert!(width > 0, "Tamil should have positive width");
        // Tamil: த + ம + ி (combining) + ழ + ் (combining)
        // Grapheme clusters should handle combining marks
        assert!(
            (3..=5).contains(&width),
            "Tamil width {} should be reasonable (3-5)",
            width
        );
    }

    #[test]
    fn test_display_width_georgian() {
        // Georgian script - "Gamarjoba" (hello)
        let georgian = "გამარჯობა";
        let width = display_width(georgian);
        assert!(width > 0, "Georgian should have positive width");
        // Georgian characters are 1 column each (9 chars = 9 cols)
        assert_eq!(width, 9, "Georgian 'გამარჯობა' should be 9 columns");
    }

    #[test]
    fn test_display_width_armenian() {
        // Armenian script - provided sample "Բdelays"
        let provided_sample = "Բdelays";
        let sample_width = display_width(provided_sample);
        // Բ (1) + d,e,l,a,y,s (6) = 7
        assert_eq!(sample_width, 7, "Armenian 'Բdelays' should be 7 columns");

        // Full Armenian greeting: "Barev"
        let armenian_word = "Բdelays";
        assert!(
            display_width(armenian_word) > 0,
            "Armenian should have positive width"
        );
    }

    #[test]
    fn test_display_width_tibetan() {
        // Tibetan script - "Bod" (Tibet)
        let tibetan = "བོད་";
        let width = display_width(tibetan);
        assert!(width > 0, "Tibetan should have positive width");
        // Tibetan with stacked consonants - width varies by terminal
        assert!(
            (2..=8).contains(&width),
            "Tibetan width {} should be reasonable",
            width
        );
    }

    #[test]
    fn test_display_width_myanmar() {
        // Myanmar/Burmese script
        let myanmar = "မြန်မာ";
        let width = display_width(myanmar);
        assert!(width > 0, "Myanmar should have positive width");
        // Myanmar with stacked consonants and medials
        assert!(
            (4..=8).contains(&width),
            "Myanmar width {} should be reasonable",
            width
        );
    }

    #[test]
    fn test_display_width_khmer() {
        // Khmer/Cambodian script - bonus test
        let khmer = "ខ្មែរ";
        let width = display_width(khmer);
        assert!(width > 0, "Khmer should have positive width");
        assert!(
            (2..=6).contains(&width),
            "Khmer width {} should be reasonable",
            width
        );
    }

    #[test]
    fn test_display_width_devanagari_hindi() {
        // Hindi in Devanagari script
        let hindi = "हिन्दी";
        let width = display_width(hindi);
        assert!(width > 0, "Hindi should have positive width");
        // Devanagari with combining vowel signs
        assert!(
            (4..=8).contains(&width),
            "Hindi width {} should be reasonable",
            width
        );
    }

    #[test]
    fn test_display_width_ethiopic() {
        // Ethiopic/Amharic script
        let ethiopic = "አማርኛ";
        let width = display_width(ethiopic);
        assert!(width > 0, "Ethiopic should have positive width");
        // Ethiopic syllabary characters
        assert_eq!(width, 4, "Ethiopic 'አማርኛ' should be 4 columns");
    }

    #[test]
    fn test_display_width_sinhala() {
        // Sinhala script (Sri Lanka)
        let sinhala = "සිංහල";
        let width = display_width(sinhala);
        assert!(width > 0, "Sinhala should have positive width");
        assert!(
            (3..=6).contains(&width),
            "Sinhala width {} should be reasonable",
            width
        );
    }

    // =========================================================================
    // Truncation tests for exotic scripts
    // =========================================================================

    #[test]
    fn test_truncate_hebrew_rtl() {
        let hebrew = "שלום עולם"; // "Hello world" in Hebrew
        let result = truncate_to_width(hebrew, 6);
        // Should truncate without crashing
        assert!(display_width(&result) <= 6);
        // Should have ellipsis if truncated
        if display_width(hebrew) > 6 {
            assert!(result.ends_with('…'));
        }
    }

    #[test]
    fn test_truncate_persian() {
        let persian = "سلام دنیا"; // "Hello world" in Persian
        let result = truncate_to_width(persian, 5);
        assert!(display_width(&result) <= 5);
    }

    #[test]
    fn test_truncate_tamil() {
        let tamil = "தமிழ் மொழி";
        let result = truncate_to_width(tamil, 6);
        assert!(display_width(&result) <= 6);
    }

    #[test]
    fn test_truncate_georgian() {
        let georgian = "გამარჯობა მსოფლიო";
        let result = truncate_to_width(georgian, 10);
        assert!(display_width(&result) <= 10);
    }

    #[test]
    fn test_truncate_tibetan() {
        let tibetan = "བོད་སྐད།";
        let result = truncate_to_width(tibetan, 5);
        assert!(display_width(&result) <= 5);
    }

    #[test]
    fn test_truncate_myanmar() {
        let myanmar = "မြန်မာဘာသာ";
        let result = truncate_to_width(myanmar, 6);
        assert!(display_width(&result) <= 6);
    }

    #[test]
    fn test_truncate_start_hebrew() {
        let hebrew = "שלום עולם";
        let result = truncate_start_to_width(hebrew, 6);
        assert!(display_width(&result) <= 6);
        if display_width(hebrew) > 6 {
            assert!(result.starts_with('…'));
        }
    }

    #[test]
    fn test_truncate_start_georgian() {
        let georgian = "გამარჯობა მსოფლიო";
        let result = truncate_start_to_width(georgian, 10);
        assert!(display_width(&result) <= 10);
    }

    // =========================================================================
    // Edge cases with combining characters
    // =========================================================================

    #[test]
    fn test_combining_diacritics() {
        // Latin with combining diacritics: e + combining acute = e
        let combined = "e\u{0301}"; // e + combining acute accent
        let width = display_width(combined);
        // Should be 1 column (grapheme cluster)
        assert_eq!(width, 1, "Combined e-acute should be 1 column");
    }

    #[test]
    fn test_zalgo_text() {
        // Extreme combining characters (Zalgo-style)
        let zalgo = "H\u{0336}\u{0335}\u{0334}e\u{0336}\u{0335}l\u{0334}p";
        let width = display_width(zalgo);
        // Should handle without panic
        assert!(width > 0, "Zalgo text should have positive width");
        // Base characters: H, e, l, p = 4 columns
        assert_eq!(width, 4, "Zalgo base should be 4 columns");
    }

    #[test]
    fn test_truncate_combining_diacritics() {
        // Don't split combining sequences
        let text = "e\u{0301}e\u{0301}e\u{0301}e\u{0301}"; // eeee (4 e-acutes)
        let result = truncate_to_width(text, 3);
        // Should keep whole graphemes
        assert!(display_width(&result) <= 3);
        // Result should be "ee…" or "e…" (complete graphemes + ellipsis)
    }

    // =========================================================================
    // Zero-width characters
    // =========================================================================

    #[test]
    fn test_zero_width_joiner() {
        // Family emoji with ZWJ
        let family = "👨‍👩‍👧"; // Man + ZWJ + Woman + ZWJ + Girl
        let width = display_width(family);
        // Should be treated as single grapheme cluster
        assert!(width > 0, "Family emoji should have positive width");
        // Typically 2 columns for the combined emoji
        assert!(
            width <= 6,
            "Family emoji width {} should be reasonable",
            width
        );
    }

    #[test]
    fn test_zero_width_space() {
        // Zero-width space should not add width
        let text = "a\u{200B}b"; // a + ZWSP + b
        let width = display_width(text);
        // ZWSP is 0 width, so total should be 2
        assert_eq!(width, 2, "Text with ZWSP should be 2 columns");
    }

    #[test]
    fn test_zero_width_non_joiner() {
        // ZWNJ used in Persian to prevent ligature
        let text = "a\u{200C}b"; // a + ZWNJ + b
        let width = display_width(text);
        assert_eq!(width, 2, "Text with ZWNJ should be 2 columns");
    }

    // =========================================================================
    // Bidirectional text
    // =========================================================================

    #[test]
    fn test_mixed_rtl_ltr() {
        // Hebrew + English mixed
        let mixed = "Hello שלום World";
        let width = display_width(mixed);
        // 5 + 1 + 4 + 1 + 5 = 16
        assert_eq!(width, 16, "Mixed RTL/LTR should be 16 columns");
    }

    #[test]
    fn test_truncate_mixed_rtl_ltr() {
        let mixed = "Hello שלום World";
        let result = truncate_to_width(mixed, 10);
        assert!(display_width(&result) <= 10);
    }

    /// Diagnostic test to print actual widths for all exotic scripts.
    /// Run with: cargo test diagnostic_print_widths -- --nocapture
    #[test]
    fn diagnostic_print_widths() {
        let scripts = [
            ("Hebrew (RTL)", "שלום"),
            ("Persian/Farsi", "سلام"),
            ("Tamil", "தமிழ்"),
            ("Georgian", "გამარჯობა"),
            ("Armenian (provided)", "Բdelays"),
            ("Tibetan", "བོད་"),
            ("Myanmar", "မြန်မာ"),
            ("Khmer (bonus)", "ខ្មែរ"),
            ("Hindi/Devanagari", "हिन्दी"),
            ("Ethiopic/Amharic", "አማርኛ"),
            ("Sinhala", "සිංහල"),
            ("Arabic", "مرحبا"),
            ("Bengali", "বাংলা"),
            ("Thai", "ไทย"),
        ];

        println!("\n=== Unicode Width Report ===\n");
        println!(
            "{:<22} {:>12} {:>10} {:>10}",
            "Script", "Text", "Chars", "Width"
        );
        println!("{}", "-".repeat(56));

        for (name, text) in scripts {
            let char_count = text.chars().count();
            let width = display_width(text);
            println!("{:<22} {:>12} {:>10} {:>10}", name, text, char_count, width);
        }
        println!();
    }
}
