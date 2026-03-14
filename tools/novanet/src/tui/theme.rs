//! TUI Theme — Visual encoding from individual YAML files + visual-encoding.yaml
//!
//! Provides terminal colors and styles from the NovaNet visual system.
//! Supports 256-color and 16-color fallback palettes.
//!
//! v0.12.5: Source of truth (individual files):
//! - Realm colors: packages/core/models/realms/*.yaml
//! - Layer colors: packages/core/models/layers/*.yaml
//! - Arc family colors: packages/core/models/arc-families/*.yaml
//! - Icons: packages/core/models/visual-encoding.yaml (icons section)

use ratatui::style::{Color, Modifier, Style};
use rustc_hash::FxHashMap;
use std::sync::LazyLock;

// =============================================================================
// COLOR MODE DETECTION
// =============================================================================

/// Terminal color capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Full 24-bit RGB support (modern terminals)
    TrueColor,
    /// 256-color xterm palette
    Color256,
    /// 16-color basic palette
    Color16,
}

impl ColorMode {
    /// Detect terminal color capability from environment.
    pub fn detect() -> Self {
        // Check COLORTERM for true color support
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm == "truecolor" || colorterm == "24bit" {
                return ColorMode::TrueColor;
            }
        }

        // Check TERM for 256-color support
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") || term.contains("256-color") {
                return ColorMode::Color256;
            }
        }

        // Fallback to 16-color
        ColorMode::Color16
    }
}

// =============================================================================
// HEX COLOR CONVERSION
// =============================================================================

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
// REALM COLORS (from realms/*.yaml)
// =============================================================================

/// Realm color definitions (v11.2: 2 realms - shared + org).
pub mod realm {
    use super::*;

    pub const SHARED_HEX: &str = "#2aa198";
    pub const ORG_HEX: &str = "#6c71c4";

    // 256-color palette indices
    pub const SHARED_256: u8 = 37;
    pub const ORG_256: u8 = 141;

    // 16-color palette indices
    pub const SHARED_16: Color = Color::Cyan;
    pub const ORG_16: Color = Color::Magenta;

    /// Get realm color for a given color mode.
    pub fn color(realm: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match realm {
                "shared" => hex_to_color(SHARED_HEX),
                "org" => hex_to_color(ORG_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match realm {
                "shared" => Color::Indexed(SHARED_256),
                "org" => Color::Indexed(ORG_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match realm {
                "shared" => SHARED_16,
                "org" => ORG_16,
                _ => Color::White,
            },
        }
    }
}

// =============================================================================
// LAYER COLORS (from layers/*.yaml)
// =============================================================================

/// Layer color definitions (v11.5: 10 layers - 4 shared + 6 org).
/// SEO/GEO consolidated to shared/knowledge in v11.5.
pub mod layer {
    use super::*;

    // Shared realm layers (4)
    pub const CONFIG_HEX: &str = "#64748b";
    pub const LOCALE_HEX: &str = "#64748b";
    pub const GEOGRAPHY_HEX: &str = "#10b981";
    pub const KNOWLEDGE_HEX: &str = "#8b5cf6";

    // Org realm layers (6)
    pub const FOUNDATION_HEX: &str = "#3b82f6";
    pub const STRUCTURE_HEX: &str = "#06b6d4";
    pub const SEMANTIC_HEX: &str = "#f97316";
    pub const INSTRUCTION_HEX: &str = "#eab308";
    pub const OUTPUT_HEX: &str = "#22c55e";

    // 256-color palette indices
    pub const CONFIG_256: u8 = 244;
    pub const LOCALE_256: u8 = 244;
    pub const GEOGRAPHY_256: u8 = 43;
    pub const KNOWLEDGE_256: u8 = 141;
    pub const FOUNDATION_256: u8 = 33;
    pub const STRUCTURE_256: u8 = 45;
    pub const SEMANTIC_256: u8 = 208;
    pub const INSTRUCTION_256: u8 = 178;
    pub const OUTPUT_256: u8 = 41;

    // 16-color palette
    pub const CONFIG_16: Color = Color::DarkGray;
    pub const LOCALE_16: Color = Color::DarkGray;
    pub const GEOGRAPHY_16: Color = Color::Green;
    pub const KNOWLEDGE_16: Color = Color::Magenta;
    pub const FOUNDATION_16: Color = Color::Blue;
    pub const STRUCTURE_16: Color = Color::Cyan;
    pub const SEMANTIC_16: Color = Color::Yellow;
    pub const INSTRUCTION_16: Color = Color::LightYellow;
    pub const OUTPUT_16: Color = Color::Green;

    /// Get layer color for a given color mode.
    pub fn color(layer_key: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match layer_key {
                "config" => hex_to_color(CONFIG_HEX),
                "locale" => hex_to_color(LOCALE_HEX),
                "geography" => hex_to_color(GEOGRAPHY_HEX),
                "knowledge" => hex_to_color(KNOWLEDGE_HEX),
                "foundation" => hex_to_color(FOUNDATION_HEX),
                "structure" => hex_to_color(STRUCTURE_HEX),
                "semantic" => hex_to_color(SEMANTIC_HEX),
                "instruction" => hex_to_color(INSTRUCTION_HEX),
                "output" => hex_to_color(OUTPUT_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match layer_key {
                "config" => Color::Indexed(CONFIG_256),
                "locale" => Color::Indexed(LOCALE_256),
                "geography" => Color::Indexed(GEOGRAPHY_256),
                "knowledge" => Color::Indexed(KNOWLEDGE_256),
                "foundation" => Color::Indexed(FOUNDATION_256),
                "structure" => Color::Indexed(STRUCTURE_256),
                "semantic" => Color::Indexed(SEMANTIC_256),
                "instruction" => Color::Indexed(INSTRUCTION_256),
                "output" => Color::Indexed(OUTPUT_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match layer_key {
                "config" => CONFIG_16,
                "locale" => LOCALE_16,
                "geography" => GEOGRAPHY_16,
                "knowledge" => KNOWLEDGE_16,
                "foundation" => FOUNDATION_16,
                "structure" => STRUCTURE_16,
                "semantic" => SEMANTIC_16,
                "instruction" => INSTRUCTION_16,
                "output" => OUTPUT_16,
                _ => Color::White,
            },
        }
    }
}

// =============================================================================
// ARC FAMILY COLORS (from arc-families/*.yaml)
// =============================================================================

/// Arc family color definitions (v0.13.1: 6 families including schema).
pub mod arc_family {
    use super::*;

    pub const OWNERSHIP_HEX: &str = "#3b82f6";
    pub const LOCALIZATION_HEX: &str = "#22c55e";
    pub const SEMANTIC_HEX: &str = "#f97316";
    pub const GENERATION_HEX: &str = "#8b5cf6";
    pub const MINING_HEX: &str = "#ec4899";
    pub const SCHEMA_HEX: &str = "#6366f1"; // v0.13.1: indigo for meta-schema

    // 256-color palette
    pub const OWNERSHIP_256: u8 = 33;
    pub const LOCALIZATION_256: u8 = 41;
    pub const SEMANTIC_256: u8 = 208;
    pub const GENERATION_256: u8 = 141;
    pub const MINING_256: u8 = 205;
    pub const SCHEMA_256: u8 = 99; // v0.13.1: indigo

    // 16-color palette
    pub const OWNERSHIP_16: Color = Color::Blue;
    pub const LOCALIZATION_16: Color = Color::Green;
    pub const SEMANTIC_16: Color = Color::Yellow;
    pub const GENERATION_16: Color = Color::Magenta;
    pub const MINING_16: Color = Color::LightMagenta;
    pub const SCHEMA_16: Color = Color::Blue; // v0.13.1: blue fallback

    /// Get arc family color for a given color mode.
    pub fn color(family: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match family {
                "ownership" => hex_to_color(OWNERSHIP_HEX),
                "localization" => hex_to_color(LOCALIZATION_HEX),
                "semantic" => hex_to_color(SEMANTIC_HEX),
                "generation" => hex_to_color(GENERATION_HEX),
                "mining" => hex_to_color(MINING_HEX),
                "schema" => hex_to_color(SCHEMA_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match family {
                "ownership" => Color::Indexed(OWNERSHIP_256),
                "localization" => Color::Indexed(LOCALIZATION_256),
                "semantic" => Color::Indexed(SEMANTIC_256),
                "generation" => Color::Indexed(GENERATION_256),
                "mining" => Color::Indexed(MINING_256),
                "schema" => Color::Indexed(SCHEMA_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match family {
                "ownership" => OWNERSHIP_16,
                "localization" => LOCALIZATION_16,
                "semantic" => SEMANTIC_16,
                "generation" => GENERATION_16,
                "mining" => MINING_16,
                "schema" => SCHEMA_16,
                _ => Color::White,
            },
        }
    }
}

// =============================================================================
// ICONS — Loaded from visual-encoding.yaml (single source of truth)
// =============================================================================

/// Terminal icons loaded from visual-encoding.yaml.
/// Provides Unicode symbols for TUI with fallback defaults.
/// Uses FxHashMap for ~30% faster string key lookups.
#[derive(Debug, Clone, Default)]
pub struct Icons {
    pub realms: FxHashMap<String, String>,
    pub layers: FxHashMap<String, String>,
    pub arc_families: FxHashMap<String, String>,
    pub states: FxHashMap<String, String>,
    pub navigation: FxHashMap<String, String>,
    pub quality: FxHashMap<String, String>,
    pub modes: FxHashMap<String, String>,
}

impl Icons {
    /// Load icons from visual-encoding.yaml.
    /// Returns default icons if loading fails (graceful degradation).
    pub fn load(root_path: &str) -> Self {
        let path =
            std::path::Path::new(root_path).join("packages/core/models/visual-encoding.yaml");

        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(doc) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                return Self::from_yaml(&doc);
            }
        }
        Self::defaults()
    }

    /// Parse icons from YAML document.
    fn from_yaml(doc: &serde_yaml::Value) -> Self {
        let mut icons = Self::defaults();

        if let Some(icons_section) = doc.get("icons") {
            // Parse each category
            Self::parse_category(icons_section, "realms", &mut icons.realms);
            Self::parse_category(icons_section, "layers", &mut icons.layers);
            Self::parse_category(icons_section, "arc_families", &mut icons.arc_families);
            Self::parse_category(icons_section, "states", &mut icons.states);
            Self::parse_category(icons_section, "navigation", &mut icons.navigation);
            Self::parse_category(icons_section, "quality", &mut icons.quality);
            Self::parse_category(icons_section, "modes", &mut icons.modes);
        }

        icons
    }

    /// Parse a single category from YAML into a FxHashMap.
    fn parse_category(
        icons_section: &serde_yaml::Value,
        category: &str,
        map: &mut FxHashMap<String, String>,
    ) {
        if let Some(cat) = icons_section.get(category) {
            if let Some(obj) = cat.as_mapping() {
                for (key, value) in obj {
                    if let (Some(k), Some(terminal)) =
                        (key.as_str(), value.get("terminal").and_then(|v| v.as_str()))
                    {
                        map.insert(k.to_string(), terminal.to_string());
                    }
                }
            }
        }
    }

    /// Default icons (fallback if YAML loading fails).
    fn defaults() -> Self {
        let mut icons = Self::default();

        // Realms (v11.2: shared + org)
        icons.realms.insert("shared".into(), "◉".into());
        icons.realms.insert("org".into(), "◎".into());

        // Layers (v11.5: 10 layers - 4 shared + 6 org)
        // All icons are single-width Unicode symbols (no emojis)
        // Shared realm (4)
        icons.layers.insert("config".into(), "⚙".into());
        icons.layers.insert("locale".into(), "⊕".into());
        icons.layers.insert("geography".into(), "⊙".into());
        icons.layers.insert("knowledge".into(), "◈".into());
        // Org realm (6)
        icons.layers.insert("foundation".into(), "▣".into());
        icons.layers.insert("structure".into(), "▤".into());
        icons.layers.insert("semantic".into(), "◆".into());
        icons.layers.insert("instruction".into(), "▧".into());
        icons.layers.insert("output".into(), "●".into());

        // Arc families (v0.13.1: 6 families including schema)
        icons.arc_families.insert("ownership".into(), "→".into());
        icons.arc_families.insert("localization".into(), "⇢".into());
        icons.arc_families.insert("semantic".into(), "~".into());
        icons.arc_families.insert("generation".into(), "⇒".into());
        icons.arc_families.insert("mining".into(), "⇝".into());
        icons.arc_families.insert("schema".into(), "≡".into());

        // States
        icons.states.insert("no_connection".into(), "⚠".into());
        icons.states.insert("no_classes".into(), "∅".into());
        icons.states.insert("no_results".into(), "◌".into());
        icons.states.insert("no_instances".into(), "□".into());
        icons.states.insert("loading".into(), "◐".into());
        icons.states.insert("success".into(), "✓".into());
        icons.states.insert("error".into(), "✗".into());
        icons.states.insert("warning".into(), "⚠".into());

        // Navigation
        icons.navigation.insert("expanded".into(), "▼".into());
        icons.navigation.insert("collapsed".into(), "▶".into());
        icons.navigation.insert("leaf".into(), "·".into());
        icons.navigation.insert("search".into(), "/".into());
        icons.navigation.insert("help".into(), "?".into());
        icons.navigation.insert("back".into(), "←".into());
        icons.navigation.insert("copy".into(), "□".into());

        // Quality
        icons.quality.insert("complete".into(), "●".into());
        icons.quality.insert("partial".into(), "◐".into());
        icons.quality.insert("empty".into(), "○".into());
        icons.quality.insert("required".into(), "*".into());
        icons.quality.insert("optional".into(), " ".into());
        icons.quality.insert("chart".into(), "≡".into());

        // Modes (v0.20.0: Graph only)
        icons.modes.insert("graph".into(), "G".into());

        icons
    }

    // Getter methods with fallbacks
    pub fn realm(&self, key: &str) -> &str {
        self.realms.get(key).map(|s| s.as_str()).unwrap_or("○")
    }

    pub fn layer(&self, key: &str) -> &str {
        self.layers.get(key).map(|s| s.as_str()).unwrap_or("·")
    }

    pub fn arc_family(&self, key: &str) -> &str {
        self.arc_families
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or("→")
    }

    pub fn state(&self, key: &str) -> &str {
        self.states.get(key).map(|s| s.as_str()).unwrap_or("·")
    }

    pub fn nav(&self, key: &str) -> &str {
        self.navigation.get(key).map(|s| s.as_str()).unwrap_or("·")
    }

    pub fn quality(&self, key: &str) -> &str {
        self.quality.get(key).map(|s| s.as_str()).unwrap_or("·")
    }

    pub fn mode(&self, key: &str) -> &str {
        self.modes.get(key).map(|s| s.as_str()).unwrap_or("·")
    }
}

// =============================================================================
// THEME STRUCT — Holds detected color mode and provides styled helpers
// =============================================================================

/// Theme instance with detected color mode and icons.
#[derive(Debug, Clone)]
pub struct Theme {
    pub mode: ColorMode,
    pub icons: Icons,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl Theme {
    /// Create a new theme with auto-detected color mode and default icons.
    pub fn new() -> Self {
        Self {
            mode: ColorMode::detect(),
            icons: Icons::defaults(),
        }
    }

    /// Create a theme with icons loaded from a specific root path.
    /// This is the preferred constructor when the monorepo root is known.
    pub fn with_root(root_path: &str) -> Self {
        Self {
            mode: ColorMode::detect(),
            icons: Icons::load(root_path),
        }
    }

    /// Create a theme with explicit color mode (uses default icons).
    pub fn with_mode(mode: ColorMode) -> Self {
        Self {
            mode,
            icons: Icons::defaults(),
        }
    }

    /// Get realm color.
    pub fn realm_color(&self, realm_key: &str) -> Color {
        realm::color(realm_key, self.mode)
    }

    /// Get layer color.
    pub fn layer_color(&self, layer_key: &str) -> Color {
        layer::color(layer_key, self.mode)
    }

    /// Get arc family color.
    pub fn arc_family_color(&self, family: &str) -> Color {
        arc_family::color(family, self.mode)
    }

    /// Get styled text for a realm.
    pub fn realm_style(&self, realm_key: &str) -> Style {
        Style::default().fg(self.realm_color(realm_key))
    }

    /// Get styled text for a layer.
    pub fn layer_style(&self, layer_key: &str) -> Style {
        Style::default().fg(self.layer_color(layer_key))
    }

    /// Get styled text for an arc family.
    pub fn arc_family_style(&self, family: &str) -> Style {
        Style::default().fg(self.arc_family_color(family))
    }

    /// Get nav mode color.
    pub fn nav_mode_color(&self, mode_label: &str) -> Color {
        nav_mode::color(mode_label, self.mode)
    }

    /// Get nav mode icon.
    pub fn nav_mode_icon(&self, mode_label: &str) -> &'static str {
        nav_mode::icon(mode_label)
    }

    /// Get styled text for a nav mode.
    pub fn nav_mode_style(&self, mode_label: &str) -> Style {
        Style::default()
            .fg(self.nav_mode_color(mode_label))
            .add_modifier(Modifier::BOLD)
    }
}

// =============================================================================
// NAV MODE COLORS — Colors for navigation modes
// =============================================================================

/// Navigation mode color definitions (v0.20.0: Graph only).
pub mod nav_mode {
    use super::*;

    // Mode colors (hex) — v0.20.0: Graph only
    pub const GRAPH_HEX: &str = "#06b6d4"; // Cyan - unified tree exploration

    // 256-color palette
    pub const GRAPH_256: u8 = 45;

    // 16-color palette
    pub const GRAPH_16: Color = Color::Cyan;

    /// Get nav mode color for a given color mode.
    /// v0.20.0: Graph only
    pub fn color(nav_mode: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match nav_mode {
                "graph" | "Graph" => hex_to_color(GRAPH_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match nav_mode {
                "graph" | "Graph" => Color::Indexed(GRAPH_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match nav_mode {
                "graph" | "Graph" => GRAPH_16,
                _ => Color::White,
            },
        }
    }

    /// Get icon for nav mode.
    /// v0.20.0: Graph only
    pub fn icon(nav_mode: &str) -> &'static str {
        match nav_mode {
            "graph" | "Graph" => "◈",
            _ => "○",
        }
    }
}

// =============================================================================
// UI COLORS — Common UI element colors
// =============================================================================

/// Common UI colors.
pub mod ui {
    use super::*;

    /// Background color (dark space theme).
    pub const BG: Color = Color::Rgb(15, 15, 20);

    /// Header background.
    pub const HEADER_BG: Color = Color::Rgb(15, 15, 20);

    /// Muted text color.
    pub const MUTED: Color = Color::DarkGray;

    /// Primary accent (cyan).
    pub const ACCENT: Color = Color::Cyan;

    /// Logo color (magenta).
    pub const LOGO: Color = Color::Magenta;

    /// Selected item highlight.
    pub const SELECTED_BG: Color = Color::Rgb(30, 35, 50);

    /// Focused item style.
    pub fn focused_style() -> Style {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    }

    /// Muted text style.
    pub fn muted_style() -> Style {
        Style::default().fg(MUTED)
    }

    /// Logo style.
    pub fn logo_style() -> Style {
        Style::default().fg(LOGO).add_modifier(Modifier::BOLD)
    }
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
    // Task 5.2: Property-based tests for hex_to_rgb
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
    // Task 5.2: Edge case unit tests for hex_to_rgb
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
    fn test_realm_colors_truecolor() {
        let mode = ColorMode::TrueColor;
        assert_eq!(realm::color("shared", mode), Color::Rgb(42, 161, 152));
        assert_eq!(realm::color("org", mode), Color::Rgb(108, 113, 196));
    }

    #[test]
    fn test_realm_colors_256() {
        let mode = ColorMode::Color256;
        assert_eq!(realm::color("shared", mode), Color::Indexed(37));
        assert_eq!(realm::color("org", mode), Color::Indexed(141));
    }

    #[test]
    fn test_realm_colors_16() {
        let mode = ColorMode::Color16;
        assert_eq!(realm::color("shared", mode), Color::Cyan);
        assert_eq!(realm::color("org", mode), Color::Magenta);
    }

    #[test]
    fn test_layer_colors() {
        let mode = ColorMode::TrueColor;
        assert_eq!(layer::color("foundation", mode), Color::Rgb(59, 130, 246));
        assert_eq!(layer::color("semantic", mode), Color::Rgb(249, 115, 22));
    }

    // Traits no longer part of schema - provenance is per-instance at runtime

    #[test]
    fn test_theme_instance() {
        let theme = Theme::with_mode(ColorMode::TrueColor);
        assert_eq!(theme.realm_color("shared"), Color::Rgb(42, 161, 152));
        assert_eq!(theme.layer_color("output"), Color::Rgb(34, 197, 94));
    }

    #[test]
    fn test_icons_defaults() {
        let icons = Icons::defaults();

        // Realms (v11.2: shared + org)
        assert_eq!(icons.realm("shared"), "◉");
        assert_eq!(icons.realm("org"), "◎");
        assert_eq!(icons.realm("unknown"), "○"); // Fallback

        // Layers
        assert_eq!(icons.layer("config"), "⚙");
        assert_eq!(icons.layer("semantic"), "◆");
        assert_eq!(icons.layer("unknown"), "·"); // Fallback

        // States
        assert_eq!(icons.state("loading"), "◐");
        assert_eq!(icons.state("no_classes"), "∅");

        // Navigation
        assert_eq!(icons.nav("expanded"), "▼");
        assert_eq!(icons.nav("collapsed"), "▶");

        // Quality
        assert_eq!(icons.quality("required"), "*");
        assert_eq!(icons.quality("chart"), "≡");

        // Modes (v0.20.0: Graph only)
        assert_eq!(icons.mode("graph"), "G");
    }

    #[test]
    fn test_theme_has_icons() {
        let theme = Theme::new();
        // Icons should be available on theme
        assert_eq!(theme.icons.realm("shared"), "◉");
        assert_eq!(theme.icons.state("loading"), "◐");
    }

    #[test]
    fn test_icons_load_integration() {
        // Test that Icons::load works with real file (integration test)
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return; // Not in monorepo context
        }

        let icons = Icons::load(&root.display().to_string());

        // Should have loaded from visual-encoding.yaml (v11.2: shared + org)
        assert_eq!(icons.realm("shared"), "◉");
        assert_eq!(icons.realm("org"), "◎");
        assert_eq!(icons.layer("config"), "⚙");
        assert_eq!(icons.state("loading"), "◐");
        assert_eq!(icons.nav("expanded"), "▼");
    }

    // =========================================================================
    // Task 2.1: Realm color resolution tests (v11.2: shared + org)
    // =========================================================================

    #[test]
    fn test_realm_color_shared_truecolor() {
        let color = realm::color("shared", ColorMode::TrueColor);
        assert!(
            matches!(color, Color::Rgb(..)),
            "shared realm should return RGB color in TrueColor mode"
        );
    }

    #[test]
    fn test_realm_color_org_truecolor() {
        let color = realm::color("org", ColorMode::TrueColor);
        assert!(
            matches!(color, Color::Rgb(..)),
            "org realm should return RGB color in TrueColor mode"
        );
    }

    #[test]
    fn test_realm_color_unknown_returns_white() {
        let color = realm::color("unknown_realm", ColorMode::TrueColor);
        assert_eq!(
            color,
            Color::White,
            "unknown realm should return White as fallback"
        );
    }

    // =========================================================================
    // Task 2.2: Layer color resolution tests
    // =========================================================================

    #[test]
    fn test_layer_color_all_layers_truecolor() {
        // Test all 10 layers return RGB colors in TrueColor mode.
        // v11.5: 4 shared layers + 6 org layers (SEO/GEO consolidated)
        let layers = [
            // Shared realm (4 layers)
            ("config", layer::CONFIG_HEX),
            ("locale", layer::LOCALE_HEX),
            ("geography", layer::GEOGRAPHY_HEX),
            ("knowledge", layer::KNOWLEDGE_HEX),
            // Org realm (6 layers) - SEO/GEO consolidated to shared/knowledge in v11.5
            ("foundation", layer::FOUNDATION_HEX),
            ("structure", layer::STRUCTURE_HEX),
            ("semantic", layer::SEMANTIC_HEX),
            ("instruction", layer::INSTRUCTION_HEX),
            ("output", layer::OUTPUT_HEX),
        ];

        for (layer_key, _expected_hex) in layers {
            let color = layer::color(layer_key, ColorMode::TrueColor);
            assert!(
                matches!(color, Color::Rgb(..)),
                "Layer '{}' should return RGB color in TrueColor mode, got {:?}",
                layer_key,
                color
            );
        }
    }

    #[test]
    fn test_layer_color_256_mode() {
        // Verify config returns Indexed color in 256 mode.
        let color = layer::color("config", ColorMode::Color256);
        assert!(
            matches!(color, Color::Indexed(_)),
            "config layer should return Indexed color in Color256 mode, got {:?}",
            color
        );
        assert_eq!(
            color,
            Color::Indexed(layer::CONFIG_256),
            "config layer should use CONFIG_256 index"
        );
    }

    #[test]
    fn test_layer_color_16_mode() {
        // Verify config doesn't return RGB in 16-color mode.
        let color = layer::color("config", ColorMode::Color16);
        assert!(
            !matches!(color, Color::Rgb(..)),
            "config layer should NOT return RGB in Color16 mode"
        );
        assert_eq!(
            color,
            layer::CONFIG_16,
            "config layer should return CONFIG_16 in Color16 mode"
        );
    }

    // =========================================================================
    // Task 2.3: Trait border style tests (ADR-005)
    // =========================================================================

    /// Test trait border styles for all 5 traits (ADR-005, updated ADR-024).
    /// Maps CSS border styles to Unicode box-drawing characters:
    // test_trait_border_all_traits, test_trait_border_fallback, test_trait_border_via_theme removed
    // Provenance will be per-instance at runtime (seed/nika/mcp)

    // =========================================================================
    // Task 2.4: Arc family color resolution tests
    // =========================================================================

    #[test]
    fn test_arc_family_color_all_families() {
        // Test all 6 arc families return RGB colors in TrueColor mode (v0.13.1)
        let mode = ColorMode::TrueColor;

        // ownership: #3b82f6 -> RGB(59, 130, 246)
        let ownership = arc_family::color("ownership", mode);
        assert!(
            matches!(ownership, Color::Rgb(..)),
            "ownership should return RGB color"
        );
        assert_eq!(ownership, Color::Rgb(59, 130, 246));

        // localization: #22c55e -> RGB(34, 197, 94)
        let localization = arc_family::color("localization", mode);
        assert!(
            matches!(localization, Color::Rgb(..)),
            "localization should return RGB color"
        );
        assert_eq!(localization, Color::Rgb(34, 197, 94));

        // semantic: #f97316 -> RGB(249, 115, 22)
        let semantic = arc_family::color("semantic", mode);
        assert!(
            matches!(semantic, Color::Rgb(..)),
            "semantic should return RGB color"
        );
        assert_eq!(semantic, Color::Rgb(249, 115, 22));

        // generation: #8b5cf6 -> RGB(139, 92, 246)
        let generation = arc_family::color("generation", mode);
        assert!(
            matches!(generation, Color::Rgb(..)),
            "generation should return RGB color"
        );
        assert_eq!(generation, Color::Rgb(139, 92, 246));

        // mining: #ec4899 -> RGB(236, 72, 153)
        let mining = arc_family::color("mining", mode);
        assert!(
            matches!(mining, Color::Rgb(..)),
            "mining should return RGB color"
        );
        assert_eq!(mining, Color::Rgb(236, 72, 153));

        // schema: #6366f1 -> RGB(99, 102, 241) (v0.13.1)
        let schema = arc_family::color("schema", mode);
        assert!(
            matches!(schema, Color::Rgb(..)),
            "schema should return RGB color"
        );
        assert_eq!(schema, Color::Rgb(99, 102, 241));
    }

    #[test]
    fn test_arc_family_color_unknown() {
        // Unknown arc family should return White fallback in all color modes
        assert_eq!(
            arc_family::color("unknown", ColorMode::TrueColor),
            Color::White,
            "unknown family should return White in TrueColor"
        );
        assert_eq!(
            arc_family::color("invalid", ColorMode::Color256),
            Color::White,
            "invalid family should return White in Color256"
        );
        assert_eq!(
            arc_family::color("nonexistent", ColorMode::Color16),
            Color::White,
            "nonexistent family should return White in Color16"
        );
    }

}
