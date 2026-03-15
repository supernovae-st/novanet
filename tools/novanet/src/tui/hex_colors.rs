//! Hex color conversion utilities.
//!
//! Provides hex string to RGB/ratatui Color conversion with a pre-computed
//! cache for known color values used frequently in tree rendering.

use ratatui::style::Color;
use rustc_hash::FxHashMap;
use std::sync::LazyLock;

/// Convert hex color string to RGB tuple.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

/// Pre-computed color cache for known hex values.
/// PERF: Avoids repeated hex parsing (20+ calls per frame in tree rendering).
static HEX_COLOR_CACHE: LazyLock<FxHashMap<&'static str, Color>> = LazyLock::new(|| {
    let mut map = FxHashMap::default();
    // Realm colors
    map.insert("#2aa198", Color::Rgb(42, 161, 152)); // SHARED
    map.insert("#6c71c4", Color::Rgb(108, 113, 196)); // ORG
    // Layer colors (shared)
    map.insert("#64748b", Color::Rgb(100, 116, 139)); // CONFIG, LOCALE
    map.insert("#10b981", Color::Rgb(16, 185, 129)); // GEOGRAPHY
    map.insert("#8b5cf6", Color::Rgb(139, 92, 246)); // KNOWLEDGE, GENERATION
    // Layer colors (org)
    map.insert("#3b82f6", Color::Rgb(59, 130, 246)); // FOUNDATION, OWNERSHIP
    map.insert("#06b6d4", Color::Rgb(6, 182, 212)); // STRUCTURE
    map.insert("#f97316", Color::Rgb(249, 115, 22)); // SEMANTIC
    map.insert("#eab308", Color::Rgb(234, 179, 8)); // INSTRUCTION
    map.insert("#22c55e", Color::Rgb(34, 197, 94)); // OUTPUT, LOCALIZATION
    // Arc family colors
    map.insert("#ec4899", Color::Rgb(236, 72, 153)); // MINING
    map.insert("#6366f1", Color::Rgb(99, 102, 241)); // SCHEMA (v0.13.1)
    map
});

/// Convert hex color string to ratatui Color.
/// PERF: Uses cached lookup for known colors (O(1) vs O(n) parsing).
pub fn hex_to_color(hex: &str) -> Color {
    // Fast path: check cache first
    if let Some(&color) = HEX_COLOR_CACHE.get(hex) {
        return color;
    }
    // Slow path: parse unknown hex
    hex_to_rgb(hex).map_or(Color::White, |(r, g, b)| Color::Rgb(r, g, b))
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#2aa198"), Some((42, 161, 152)));
        assert_eq!(hex_to_rgb("6c71c4"), Some((108, 113, 196)));
        assert_eq!(hex_to_rgb("#fff"), None); // Too short
        assert_eq!(hex_to_rgb("invalid"), None);
    }

    // =========================================================================
    // Property-based tests for hex_to_rgb
    // =========================================================================

    proptest! {
        /// Property: Any valid 6-digit hex color with # prefix should round-trip correctly.
        /// Given r, g, b values in 0..=255, formatting as "#rrggbb" and parsing
        /// should return the original (r, g, b) tuple.
        #[test]
        fn test_hex_to_rgb_valid_format(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let result = hex_to_rgb(&hex);
            prop_assert!(result.is_some(), "valid hex '{}' should parse successfully", hex);
            let (rr, gg, bb) = result.unwrap();
            prop_assert_eq!(rr, r, "red component mismatch for '{}'", hex);
            prop_assert_eq!(gg, g, "green component mismatch for '{}'", hex);
            prop_assert_eq!(bb, b, "blue component mismatch for '{}'", hex);
        }

        /// Property: Uppercase hex should also parse correctly.
        #[test]
        fn test_hex_to_rgb_uppercase(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
            let result = hex_to_rgb(&hex);
            prop_assert!(result.is_some(), "uppercase hex '{}' should parse successfully", hex);
            let (rr, gg, bb) = result.unwrap();
            prop_assert_eq!(rr, r, "red component mismatch for '{}'", hex);
            prop_assert_eq!(gg, g, "green component mismatch for '{}'", hex);
            prop_assert_eq!(bb, b, "blue component mismatch for '{}'", hex);
        }

        /// Property: Valid hex without # prefix should also parse correctly.
        #[test]
        fn test_hex_to_rgb_no_prefix(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let hex = format!("{:02x}{:02x}{:02x}", r, g, b);
            let result = hex_to_rgb(&hex);
            prop_assert!(result.is_some(), "hex without prefix '{}' should parse successfully", hex);
            let (rr, gg, bb) = result.unwrap();
            prop_assert_eq!(rr, r, "red component mismatch for '{}'", hex);
            prop_assert_eq!(gg, g, "green component mismatch for '{}'", hex);
            prop_assert_eq!(bb, b, "blue component mismatch for '{}'", hex);
        }
    }

    // =========================================================================
    // Edge case unit tests for hex_to_rgb
    // =========================================================================

    /// Test that hex strings with invalid length return None.
    #[test]
    fn test_hex_to_rgb_invalid_length() {
        // Too short (CSS shorthand not supported)
        assert_eq!(hex_to_rgb("#fff"), None, "3-char hex should be rejected");
        assert_eq!(
            hex_to_rgb("fff"),
            None,
            "3-char hex without # should be rejected"
        );
        assert_eq!(hex_to_rgb("#12"), None, "2-char hex should be rejected");
        assert_eq!(hex_to_rgb("#1"), None, "1-char hex should be rejected");

        // Too long
        assert_eq!(
            hex_to_rgb("#fffffff"),
            None,
            "7-char hex should be rejected"
        );
        assert_eq!(
            hex_to_rgb("#ffffffff"),
            None,
            "8-char hex (with alpha) should be rejected"
        );
        assert_eq!(
            hex_to_rgb("#ffffffffff"),
            None,
            "10-char hex should be rejected"
        );

        // Empty
        assert_eq!(hex_to_rgb(""), None, "empty string should be rejected");
        assert_eq!(hex_to_rgb("#"), None, "only # should be rejected");
    }

    /// Test that hex strings with invalid characters return None.
    #[test]
    fn test_hex_to_rgb_invalid_chars() {
        // Invalid hex characters
        assert_eq!(
            hex_to_rgb("#gggggg"),
            None,
            "'g' is not a valid hex character"
        );
        assert_eq!(
            hex_to_rgb("#zzzzzz"),
            None,
            "'z' is not a valid hex character"
        );
        assert_eq!(
            hex_to_rgb("#ghijkl"),
            None,
            "letters after 'f' are not valid hex"
        );

        // Mixed valid/invalid
        assert_eq!(
            hex_to_rgb("#ff00gg"),
            None,
            "partially invalid hex should be rejected"
        );
        assert_eq!(
            hex_to_rgb("#0g0000"),
            None,
            "single invalid char should cause rejection"
        );

        // Special characters
        assert_eq!(hex_to_rgb("#ff-fff"), None, "dash is not valid in hex");
        assert_eq!(hex_to_rgb("#ff fff"), None, "space is not valid in hex");
        assert_eq!(hex_to_rgb("#ff.fff"), None, "dot is not valid in hex");

        // Non-ASCII
        assert_eq!(
            hex_to_rgb("#ffffff\u{00e9}"),
            None,
            "non-ASCII should be rejected (length check)"
        );
    }

    /// Test boundary values for hex_to_rgb.
    #[test]
    fn test_hex_to_rgb_boundary_values() {
        // Minimum values (all zeros)
        assert_eq!(
            hex_to_rgb("#000000"),
            Some((0, 0, 0)),
            "black should parse correctly"
        );

        // Maximum values (all 255)
        assert_eq!(
            hex_to_rgb("#ffffff"),
            Some((255, 255, 255)),
            "white should parse correctly"
        );

        // Single channel max
        assert_eq!(
            hex_to_rgb("#ff0000"),
            Some((255, 0, 0)),
            "pure red should parse correctly"
        );
        assert_eq!(
            hex_to_rgb("#00ff00"),
            Some((0, 255, 0)),
            "pure green should parse correctly"
        );
        assert_eq!(
            hex_to_rgb("#0000ff"),
            Some((0, 0, 255)),
            "pure blue should parse correctly"
        );

        // Mid values
        assert_eq!(
            hex_to_rgb("#808080"),
            Some((128, 128, 128)),
            "gray should parse correctly"
        );
    }

    #[test]
    fn test_hex_to_color_cached() {
        // Test that cached colors return correct values
        assert_eq!(hex_to_color("#2aa198"), Color::Rgb(42, 161, 152));
        assert_eq!(hex_to_color("#6c71c4"), Color::Rgb(108, 113, 196));
    }

    #[test]
    fn test_hex_to_color_uncached() {
        // Test an uncached hex value
        assert_eq!(hex_to_color("#abcdef"), Color::Rgb(171, 205, 239));
    }

    #[test]
    fn test_hex_to_color_invalid() {
        // Invalid hex should return White
        assert_eq!(hex_to_color("invalid"), Color::White);
    }
}
