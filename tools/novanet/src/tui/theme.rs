//! TUI Theme — Visual encoding from taxonomy.yaml + visual-encoding.yaml
//!
//! Provides terminal colors and styles from the NovaNet visual system.
//! Supports 256-color and 16-color fallback palettes.
//!
//! Source of truth:
//! - Colors: packages/core/models/taxonomy.yaml
//! - Icons: packages/core/models/visual-encoding.yaml (icons section)

use ratatui::style::{Color, Modifier, Style};
use rustc_hash::FxHashMap;

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

/// Convert hex color string to ratatui Color.
pub fn hex_to_color(hex: &str) -> Color {
    hex_to_rgb(hex).map_or(Color::White, |(r, g, b)| Color::Rgb(r, g, b))
}

// =============================================================================
// REALM COLORS (from taxonomy.yaml node_realms)
// =============================================================================

/// Realm color definitions (v10.6: 2 realms only - global + tenant).
pub mod realm {
    use super::*;

    pub const GLOBAL_HEX: &str = "#2aa198";
    pub const TENANT_HEX: &str = "#6c71c4";

    // 256-color palette indices
    pub const GLOBAL_256: u8 = 37;
    pub const TENANT_256: u8 = 141;

    // 16-color palette indices
    pub const GLOBAL_16: Color = Color::Cyan;
    pub const TENANT_16: Color = Color::Magenta;

    /// Get realm color for a given color mode.
    pub fn color(realm: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match realm {
                "global" => hex_to_color(GLOBAL_HEX),
                "tenant" => hex_to_color(TENANT_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match realm {
                "global" => Color::Indexed(GLOBAL_256),
                "tenant" => Color::Indexed(TENANT_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match realm {
                "global" => GLOBAL_16,
                "tenant" => TENANT_16,
                _ => Color::White,
            },
        }
    }
}

// =============================================================================
// LAYER COLORS (from taxonomy.yaml node_layers)
// =============================================================================

/// Layer color definitions.
pub mod layer {
    use super::*;

    pub const CONFIG_HEX: &str = "#64748b";
    pub const KNOWLEDGE_HEX: &str = "#8b5cf6";
    pub const FOUNDATION_HEX: &str = "#3b82f6";
    pub const STRUCTURE_HEX: &str = "#06b6d4";
    pub const SEMANTIC_HEX: &str = "#f97316";
    pub const INSTRUCTION_HEX: &str = "#eab308";
    pub const OUTPUT_HEX: &str = "#22c55e";
    pub const SEO_HEX: &str = "#ec4899";

    // 256-color palette indices
    pub const CONFIG_256: u8 = 244;
    pub const KNOWLEDGE_256: u8 = 141;
    pub const FOUNDATION_256: u8 = 33;
    pub const STRUCTURE_256: u8 = 45;
    pub const SEMANTIC_256: u8 = 208;
    pub const INSTRUCTION_256: u8 = 178;
    pub const OUTPUT_256: u8 = 41;
    pub const SEO_256: u8 = 205;

    // 16-color palette
    pub const CONFIG_16: Color = Color::DarkGray;
    pub const KNOWLEDGE_16: Color = Color::Magenta;
    pub const FOUNDATION_16: Color = Color::Blue;
    pub const STRUCTURE_16: Color = Color::Cyan;
    pub const SEMANTIC_16: Color = Color::Yellow;
    pub const INSTRUCTION_16: Color = Color::LightYellow;
    pub const OUTPUT_16: Color = Color::Green;
    pub const SEO_16: Color = Color::LightMagenta;

    /// Get layer color for a given color mode.
    pub fn color(layer_key: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match layer_key {
                "config" => hex_to_color(CONFIG_HEX),
                "knowledge" => hex_to_color(KNOWLEDGE_HEX),
                "foundation" => hex_to_color(FOUNDATION_HEX),
                "structure" => hex_to_color(STRUCTURE_HEX),
                "semantic" => hex_to_color(SEMANTIC_HEX),
                "instruction" => hex_to_color(INSTRUCTION_HEX),
                "output" => hex_to_color(OUTPUT_HEX),
                "seo" => hex_to_color(SEO_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match layer_key {
                "config" => Color::Indexed(CONFIG_256),
                "knowledge" => Color::Indexed(KNOWLEDGE_256),
                "foundation" => Color::Indexed(FOUNDATION_256),
                "structure" => Color::Indexed(STRUCTURE_256),
                "semantic" => Color::Indexed(SEMANTIC_256),
                "instruction" => Color::Indexed(INSTRUCTION_256),
                "output" => Color::Indexed(OUTPUT_256),
                "seo" => Color::Indexed(SEO_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match layer_key {
                "config" => CONFIG_16,
                "knowledge" => KNOWLEDGE_16,
                "foundation" => FOUNDATION_16,
                "structure" => STRUCTURE_16,
                "semantic" => SEMANTIC_16,
                "instruction" => INSTRUCTION_16,
                "output" => OUTPUT_16,
                "seo" => SEO_16,
                _ => Color::White,
            },
        }
    }
}

// =============================================================================
// TRAIT STYLES (from taxonomy.yaml node_traits)
// =============================================================================

/// Trait border styles for visual encoding.
pub mod traits {
    use super::*;

    // Unicode border characters for trait encoding
    pub const INVARIANT_BORDER: &str = "─";
    pub const LOCALIZED_BORDER: &str = "┄";
    pub const KNOWLEDGE_BORDER: &str = "┈";
    pub const DERIVED_BORDER: &str = "═";
    pub const JOB_BORDER: &str = " ";

    // Trait colors (hex)
    pub const INVARIANT_HEX: &str = "#3b82f6";
    pub const LOCALIZED_HEX: &str = "#22c55e";
    pub const KNOWLEDGE_HEX: &str = "#8b5cf6";
    pub const DERIVED_HEX: &str = "#9ca3af";
    pub const JOB_HEX: &str = "#6b7280";

    // 256-color palette
    pub const INVARIANT_256: u8 = 33;
    pub const LOCALIZED_256: u8 = 41;
    pub const KNOWLEDGE_256: u8 = 141;
    pub const DERIVED_256: u8 = 245;
    pub const JOB_256: u8 = 240;

    // 16-color palette
    pub const INVARIANT_16: Color = Color::Blue;
    pub const LOCALIZED_16: Color = Color::Green;
    pub const KNOWLEDGE_16: Color = Color::Magenta;
    pub const DERIVED_16: Color = Color::DarkGray;
    pub const JOB_16: Color = Color::DarkGray;

    /// Get trait border character.
    pub fn border_char(trait_key: &str) -> &'static str {
        match trait_key {
            "invariant" => INVARIANT_BORDER,
            "localized" => LOCALIZED_BORDER,
            "knowledge" => KNOWLEDGE_BORDER,
            "derived" => DERIVED_BORDER,
            "job" => JOB_BORDER,
            _ => INVARIANT_BORDER,
        }
    }

    /// Get trait color for a given color mode.
    pub fn color(trait_key: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match trait_key {
                "invariant" => hex_to_color(INVARIANT_HEX),
                "localized" => hex_to_color(LOCALIZED_HEX),
                "knowledge" => hex_to_color(KNOWLEDGE_HEX),
                "derived" => hex_to_color(DERIVED_HEX),
                "job" => hex_to_color(JOB_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match trait_key {
                "invariant" => Color::Indexed(INVARIANT_256),
                "localized" => Color::Indexed(LOCALIZED_256),
                "knowledge" => Color::Indexed(KNOWLEDGE_256),
                "derived" => Color::Indexed(DERIVED_256),
                "job" => Color::Indexed(JOB_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match trait_key {
                "invariant" => INVARIANT_16,
                "localized" => LOCALIZED_16,
                "knowledge" => KNOWLEDGE_16,
                "derived" => DERIVED_16,
                "job" => JOB_16,
                _ => Color::White,
            },
        }
    }

    /// Get modifier for trait (bold for invariant, dim for job).
    pub fn modifier(trait_key: &str) -> Modifier {
        match trait_key {
            "invariant" => Modifier::BOLD,
            "job" => Modifier::DIM,
            _ => Modifier::empty(),
        }
    }
}

// =============================================================================
// ARC FAMILY COLORS (from taxonomy.yaml arc_families)
// =============================================================================

/// Arc family color definitions.
pub mod arc_family {
    use super::*;

    pub const OWNERSHIP_HEX: &str = "#3b82f6";
    pub const LOCALIZATION_HEX: &str = "#22c55e";
    pub const SEMANTIC_HEX: &str = "#f97316";
    pub const GENERATION_HEX: &str = "#8b5cf6";
    pub const MINING_HEX: &str = "#ec4899";

    // 256-color palette
    pub const OWNERSHIP_256: u8 = 33;
    pub const LOCALIZATION_256: u8 = 41;
    pub const SEMANTIC_256: u8 = 208;
    pub const GENERATION_256: u8 = 141;
    pub const MINING_256: u8 = 205;

    // 16-color palette
    pub const OWNERSHIP_16: Color = Color::Blue;
    pub const LOCALIZATION_16: Color = Color::Green;
    pub const SEMANTIC_16: Color = Color::Yellow;
    pub const GENERATION_16: Color = Color::Magenta;
    pub const MINING_16: Color = Color::LightMagenta;

    /// Get arc family color for a given color mode.
    pub fn color(family: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match family {
                "ownership" => hex_to_color(OWNERSHIP_HEX),
                "localization" => hex_to_color(LOCALIZATION_HEX),
                "semantic" => hex_to_color(SEMANTIC_HEX),
                "generation" => hex_to_color(GENERATION_HEX),
                "mining" => hex_to_color(MINING_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match family {
                "ownership" => Color::Indexed(OWNERSHIP_256),
                "localization" => Color::Indexed(LOCALIZATION_256),
                "semantic" => Color::Indexed(SEMANTIC_256),
                "generation" => Color::Indexed(GENERATION_256),
                "mining" => Color::Indexed(MINING_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match family {
                "ownership" => OWNERSHIP_16,
                "localization" => LOCALIZATION_16,
                "semantic" => SEMANTIC_16,
                "generation" => GENERATION_16,
                "mining" => MINING_16,
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
    pub traits: FxHashMap<String, String>,
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
            Self::parse_category(icons_section, "traits", &mut icons.traits);
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

        // Realms
        icons.realms.insert("global".into(), "◉".into());
        icons.realms.insert("tenant".into(), "◎".into());

        // Layers
        icons.layers.insert("config".into(), "⚙".into());
        icons.layers.insert("locale-knowledge".into(), "◈".into());
        icons.layers.insert("seo".into(), "◇".into());
        icons.layers.insert("foundation".into(), "▣".into());
        icons.layers.insert("structure".into(), "▤".into());
        icons.layers.insert("semantic".into(), "◆".into());
        icons.layers.insert("instruction".into(), "▧".into());
        icons.layers.insert("output".into(), "●".into());

        // Traits
        icons.traits.insert("invariant".into(), "■".into());
        icons.traits.insert("localized".into(), "□".into());
        icons.traits.insert("knowledge".into(), "◊".into());
        icons.traits.insert("derived".into(), "▪".into());
        icons.traits.insert("job".into(), "▫".into());

        // Arc families
        icons.arc_families.insert("ownership".into(), "→".into());
        icons.arc_families.insert("localization".into(), "⇢".into());
        icons.arc_families.insert("semantic".into(), "~".into());
        icons.arc_families.insert("generation".into(), "⇒".into());
        icons.arc_families.insert("mining".into(), "⇝".into());

        // States
        icons.states.insert("no_connection".into(), "⚠".into());
        icons.states.insert("no_kinds".into(), "∅".into());
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

        // Modes
        icons.modes.insert("meta".into(), "M".into());
        icons.modes.insert("data".into(), "D".into());
        icons.modes.insert("overlay".into(), "O".into());
        icons.modes.insert("query".into(), "Q".into());
        icons.modes.insert("atlas".into(), "A".into());
        icons.modes.insert("audit".into(), "U".into());

        icons
    }

    // Getter methods with fallbacks
    pub fn realm(&self, key: &str) -> &str {
        self.realms.get(key).map(|s| s.as_str()).unwrap_or("○")
    }

    pub fn layer(&self, key: &str) -> &str {
        self.layers.get(key).map(|s| s.as_str()).unwrap_or("·")
    }

    pub fn trait_icon(&self, key: &str) -> &str {
        self.traits.get(key).map(|s| s.as_str()).unwrap_or("·")
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

    /// Get trait color.
    pub fn trait_color(&self, trait_key: &str) -> Color {
        traits::color(trait_key, self.mode)
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

    /// Get styled text for a trait (with appropriate modifier).
    pub fn trait_style(&self, trait_key: &str) -> Style {
        Style::default()
            .fg(self.trait_color(trait_key))
            .add_modifier(traits::modifier(trait_key))
    }

    /// Get styled text for an arc family.
    pub fn arc_family_style(&self, family: &str) -> Style {
        Style::default().fg(self.arc_family_color(family))
    }

    /// Get trait border character.
    pub fn trait_border(&self, trait_key: &str) -> &'static str {
        traits::border_char(trait_key)
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

/// Navigation mode color definitions.
pub mod nav_mode {
    use super::*;

    // Mode colors (hex)
    pub const META_HEX: &str = "#06b6d4"; // Cyan - schema exploration
    pub const DATA_HEX: &str = "#22c55e"; // Green - live data
    pub const OVERLAY_HEX: &str = "#f97316"; // Orange - combined view
    pub const QUERY_HEX: &str = "#eab308"; // Yellow - search/filter
    pub const ATLAS_HEX: &str = "#8b5cf6"; // Purple - architecture

    // 256-color palette
    pub const META_256: u8 = 45;
    pub const DATA_256: u8 = 41;
    pub const OVERLAY_256: u8 = 208;
    pub const QUERY_256: u8 = 178;
    pub const ATLAS_256: u8 = 141;

    // 16-color palette
    pub const META_16: Color = Color::Cyan;
    pub const DATA_16: Color = Color::Green;
    pub const OVERLAY_16: Color = Color::Yellow;
    pub const QUERY_16: Color = Color::LightYellow;
    pub const ATLAS_16: Color = Color::Magenta;

    /// Get nav mode color for a given color mode.
    pub fn color(nav_mode: &str, mode: ColorMode) -> Color {
        match mode {
            ColorMode::TrueColor => match nav_mode {
                "meta" | "Meta" => hex_to_color(META_HEX),
                "data" | "Data" => hex_to_color(DATA_HEX),
                "overlay" | "Overlay" => hex_to_color(OVERLAY_HEX),
                "query" | "Query" => hex_to_color(QUERY_HEX),
                "atlas" | "Atlas" => hex_to_color(ATLAS_HEX),
                _ => Color::White,
            },
            ColorMode::Color256 => match nav_mode {
                "meta" | "Meta" => Color::Indexed(META_256),
                "data" | "Data" => Color::Indexed(DATA_256),
                "overlay" | "Overlay" => Color::Indexed(OVERLAY_256),
                "query" | "Query" => Color::Indexed(QUERY_256),
                "atlas" | "Atlas" => Color::Indexed(ATLAS_256),
                _ => Color::White,
            },
            ColorMode::Color16 => match nav_mode {
                "meta" | "Meta" => META_16,
                "data" | "Data" => DATA_16,
                "overlay" | "Overlay" => OVERLAY_16,
                "query" | "Query" => QUERY_16,
                "atlas" | "Atlas" => ATLAS_16,
                _ => Color::White,
            },
        }
    }

    /// Get icon for nav mode.
    pub fn icon(nav_mode: &str) -> &'static str {
        match nav_mode {
            "meta" | "Meta" => "◈",
            "data" | "Data" => "●",
            "overlay" | "Overlay" => "◐",
            "query" | "Query" => "◎",
            "atlas" | "Atlas" => "✦",
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
        assert_eq!(realm::color("global", mode), Color::Rgb(42, 161, 152));
        assert_eq!(realm::color("tenant", mode), Color::Rgb(108, 113, 196));
    }

    #[test]
    fn test_realm_colors_256() {
        let mode = ColorMode::Color256;
        assert_eq!(realm::color("global", mode), Color::Indexed(37));
        assert_eq!(realm::color("tenant", mode), Color::Indexed(141));
    }

    #[test]
    fn test_realm_colors_16() {
        let mode = ColorMode::Color16;
        assert_eq!(realm::color("global", mode), Color::Cyan);
        assert_eq!(realm::color("tenant", mode), Color::Magenta);
    }

    #[test]
    fn test_layer_colors() {
        let mode = ColorMode::TrueColor;
        assert_eq!(layer::color("foundation", mode), Color::Rgb(59, 130, 246));
        assert_eq!(layer::color("semantic", mode), Color::Rgb(249, 115, 22));
    }

    #[test]
    fn test_trait_borders() {
        assert_eq!(traits::border_char("invariant"), "─");
        assert_eq!(traits::border_char("localized"), "┄");
        assert_eq!(traits::border_char("derived"), "═");
    }

    #[test]
    fn test_trait_modifiers() {
        assert_eq!(traits::modifier("invariant"), Modifier::BOLD);
        assert_eq!(traits::modifier("job"), Modifier::DIM);
        assert_eq!(traits::modifier("localized"), Modifier::empty());
    }

    #[test]
    fn test_theme_instance() {
        let theme = Theme::with_mode(ColorMode::TrueColor);
        assert_eq!(theme.realm_color("global"), Color::Rgb(42, 161, 152));
        assert_eq!(theme.layer_color("output"), Color::Rgb(34, 197, 94));
        assert_eq!(theme.trait_border("derived"), "═");
    }

    #[test]
    fn test_icons_defaults() {
        let icons = Icons::defaults();

        // Realms
        assert_eq!(icons.realm("global"), "◉");
        assert_eq!(icons.realm("tenant"), "◎");
        assert_eq!(icons.realm("unknown"), "○"); // Fallback

        // Layers
        assert_eq!(icons.layer("config"), "⚙");
        assert_eq!(icons.layer("semantic"), "◆");
        assert_eq!(icons.layer("unknown"), "·"); // Fallback

        // Traits
        assert_eq!(icons.trait_icon("invariant"), "■");
        assert_eq!(icons.trait_icon("localized"), "□");

        // States
        assert_eq!(icons.state("loading"), "◐");
        assert_eq!(icons.state("no_kinds"), "∅");

        // Navigation
        assert_eq!(icons.nav("expanded"), "▼");
        assert_eq!(icons.nav("collapsed"), "▶");

        // Quality
        assert_eq!(icons.quality("required"), "*");
        assert_eq!(icons.quality("chart"), "≡");

        // Modes
        assert_eq!(icons.mode("meta"), "M");
        assert_eq!(icons.mode("atlas"), "A");
    }

    #[test]
    fn test_theme_has_icons() {
        let theme = Theme::new();
        // Icons should be available on theme
        assert_eq!(theme.icons.realm("global"), "◉");
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

        // Should have loaded from visual-encoding.yaml
        assert_eq!(icons.realm("global"), "◉");
        assert_eq!(icons.realm("tenant"), "◎");
        assert_eq!(icons.layer("config"), "⚙");
        assert_eq!(icons.state("loading"), "◐");
        assert_eq!(icons.nav("expanded"), "▼");
    }

    // =========================================================================
    // Task 2.1: Realm color resolution tests
    // =========================================================================

    #[test]
    fn test_realm_color_global_truecolor() {
        let color = realm::color("global", ColorMode::TrueColor);
        assert!(
            matches!(color, Color::Rgb(..)),
            "global realm should return RGB color in TrueColor mode"
        );
    }

    #[test]
    fn test_realm_color_tenant_truecolor() {
        let color = realm::color("tenant", ColorMode::TrueColor);
        assert!(
            matches!(color, Color::Rgb(..)),
            "tenant realm should return RGB color in TrueColor mode"
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
        // Test all 8 layers return RGB colors in TrueColor mode.
        // Note: "knowledge" is the key for locale-knowledge layer in the color function.
        // The task specifies "locale-knowledge" but the implementation uses "knowledge".
        let layers = [
            ("config", layer::CONFIG_HEX),
            ("knowledge", layer::KNOWLEDGE_HEX), // locale-knowledge
            ("seo", layer::SEO_HEX),
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

    /// Test trait border styles for all 5 traits (ADR-005).
    /// Maps CSS border styles to Unicode box-drawing characters:
    /// - invariant: solid -> "─" (U+2500 BOX DRAWINGS LIGHT HORIZONTAL)
    /// - localized: dashed -> "┄" (U+2504 BOX DRAWINGS LIGHT TRIPLE DASH HORIZONTAL)
    /// - knowledge: double -> "┈" (U+2508 BOX DRAWINGS LIGHT QUADRUPLE DASH HORIZONTAL)
    /// - derived: dotted -> "═" (U+2550 BOX DRAWINGS DOUBLE HORIZONTAL)
    /// - job: none -> " " (space - no border)
    #[test]
    fn test_trait_border_all_traits() {
        assert_eq!(
            traits::border_char("invariant"),
            "─",
            "invariant: solid border"
        );
        assert_eq!(
            traits::border_char("localized"),
            "┄",
            "localized: dashed border"
        );
        assert_eq!(
            traits::border_char("knowledge"),
            "┈",
            "knowledge: double border"
        );
        assert_eq!(
            traits::border_char("derived"),
            "═",
            "derived: dotted border"
        );
        assert_eq!(traits::border_char("job"), " ", "job: no border (thin)");
    }

    /// Test trait border fallback for unknown traits.
    #[test]
    fn test_trait_border_fallback() {
        // Unknown traits should fallback to invariant border (solid)
        assert_eq!(
            traits::border_char("unknown"),
            "─",
            "unknown trait should fallback to invariant border"
        );
        assert_eq!(
            traits::border_char(""),
            "─",
            "empty trait should fallback to invariant border"
        );
    }

    /// Test trait border via Theme instance.
    #[test]
    fn test_trait_border_via_theme() {
        let theme = Theme::with_mode(ColorMode::TrueColor);

        assert_eq!(theme.trait_border("invariant"), "─");
        assert_eq!(theme.trait_border("localized"), "┄");
        assert_eq!(theme.trait_border("knowledge"), "┈");
        assert_eq!(theme.trait_border("derived"), "═");
        assert_eq!(theme.trait_border("job"), " ");
    }

    // =========================================================================
    // Task 2.4: Arc family color resolution tests
    // =========================================================================

    #[test]
    fn test_arc_family_color_all_families() {
        // Test all 5 arc families return RGB colors in TrueColor mode
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
