//! Visual encoding parser — reads visual-encoding.yaml (v0.12.0).
//!
//! Parses the visual presentation rules for NovaNet graph elements:
//! - Channel mapping (which visual property encodes which facet)
//! - Node/arc states (opacity, scale, shadow, etc.)
//! - Trait border styles (solid, dashed, dotted, double)
//! - Scope stroke styles (intra/cross realm)
//! - Cardinality arrow heads
//! - Class icons (Lucide icon mapping) — v0.12.0: kind → class
//! - Icon system (web + terminal icons for all categories)
//! - Animation presets
//! - Accessibility settings

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Top-level document
// ─────────────────────────────────────────────────────────────────────────────

/// Visual encoding document from visual-encoding.yaml.
#[derive(Debug, Deserialize)]
pub struct VisualEncodingDoc {
    pub version: String,
    pub channel_mapping: ChannelMapping,
    pub node_states: HashMap<String, NodeState>,
    pub arc_states: HashMap<String, ArcState>,
    pub trait_borders: HashMap<String, TraitBorder>,
    pub scope_strokes: HashMap<String, ScopeStroke>,
    pub cardinality_arrows: HashMap<String, CardinalityArrow>,
    pub class_icons: HashMap<String, String>,
    pub animations: HashMap<String, Animation>,
    pub accessibility: AccessibilitySettings,
    /// Icon system (v10.6) — single source of truth for all icons.
    #[serde(default)]
    pub icons: Option<Icons>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Channel mapping
// ─────────────────────────────────────────────────────────────────────────────

/// Channel mapping defines which visual property encodes which facet.
#[derive(Debug, Deserialize)]
pub struct ChannelMapping {
    pub node: NodeChannels,
    pub arc: ArcChannels,
}

/// Node visual channels.
#[derive(Debug, Deserialize)]
pub struct NodeChannels {
    pub fill_color: String,
    pub border_style: String,
    pub border_color: String,
    pub icon: String,
    pub spatial_grouping: String,
}

/// Arc visual channels.
#[derive(Debug, Deserialize)]
pub struct ArcChannels {
    pub stroke_color: String,
    pub stroke_style: String,
    pub arrow_head: String,
    pub label_position: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// States
// ─────────────────────────────────────────────────────────────────────────────

/// Node visual state.
#[derive(Debug, Deserialize)]
pub struct NodeState {
    pub opacity: f32,
    pub scale: f32,
    #[serde(default)]
    pub shadow: Option<String>,
    #[serde(default)]
    pub ring: Option<String>,
}

/// Arc visual state.
#[derive(Debug, Deserialize)]
pub struct ArcState {
    pub opacity: f32,
    pub stroke_width: f32,
    #[serde(default)]
    pub animated: Option<bool>,
    #[serde(default)]
    pub label_visible: Option<bool>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Border and stroke styles
// ─────────────────────────────────────────────────────────────────────────────

/// Trait border style for Studio (CSS) and TUI (Unicode).
#[derive(Debug, Deserialize)]
pub struct TraitBorder {
    pub css_style: String,
    pub css_width: String,
    #[serde(default)]
    pub css_dash_array: Option<String>,
    #[serde(default)]
    pub css_corner_radius: Option<String>,
    pub unicode_char: String,
    pub unicode_style: String,
    pub description: String,
}

/// Scope stroke style (intra vs cross realm).
#[derive(Debug, Deserialize)]
pub struct ScopeStroke {
    pub stroke_style: String,
    pub opacity: f32,
    pub description: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Cardinality arrows
// ─────────────────────────────────────────────────────────────────────────────

/// Arrow head configuration for cardinality.
#[derive(Debug, Deserialize)]
pub struct CardinalityArrow {
    pub arrow_head: String,
    pub arrow_end: String,
    pub description: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Animations
// ─────────────────────────────────────────────────────────────────────────────

/// Animation preset for Studio.
#[derive(Debug, Deserialize)]
pub struct Animation {
    pub duration_ms: u32,
    pub easing: String,
    #[serde(default)]
    pub stagger_ms: Option<u32>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Accessibility
// ─────────────────────────────────────────────────────────────────────────────

/// Accessibility settings.
#[derive(Debug, Deserialize)]
pub struct AccessibilitySettings {
    pub min_contrast_ratio: f32,
    pub use_patterns: bool,
    pub use_icons: bool,
    pub use_borders: bool,
    pub focus_ring_width: String,
    pub focus_ring_color: String,
    pub focus_ring_offset: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Icon system (v10.6)
// ─────────────────────────────────────────────────────────────────────────────

/// Single icon with web (Lucide) and terminal (Unicode) variants.
#[derive(Debug, Clone, Deserialize)]
pub struct Icon {
    /// Lucide icon name for web/Studio.
    pub web: String,
    /// Unicode symbol for terminal/TUI.
    pub terminal: String,
    /// Human-readable description.
    pub description: String,
}

/// Complete icon system — single source of truth for all icons.
#[derive(Debug, Default, Deserialize)]
pub struct Icons {
    /// Realm icons (shared, org).
    #[serde(default)]
    pub realms: HashMap<String, Icon>,
    /// Layer icons (locale, geography, knowledge, config, etc.).
    #[serde(default)]
    pub layers: HashMap<String, Icon>,
    /// Trait icons (invariant, localized, etc.).
    #[serde(default)]
    pub traits: HashMap<String, Icon>,
    /// Arc family icons (ownership, semantic, etc.).
    #[serde(default)]
    pub arc_families: HashMap<String, Icon>,
    /// UI state icons (loading, error, etc.).
    #[serde(default)]
    pub states: HashMap<String, Icon>,
    /// Navigation icons (expanded, collapsed, etc.).
    #[serde(default)]
    pub navigation: HashMap<String, Icon>,
    /// Data quality icons (complete, partial, etc.).
    #[serde(default)]
    pub quality: HashMap<String, Icon>,
    /// Navigation mode icons (meta, data, etc.).
    #[serde(default)]
    pub modes: HashMap<String, Icon>,
}

impl Icons {
    /// Get terminal icon for a realm.
    pub fn realm_terminal(&self, key: &str) -> &str {
        self.realms
            .get(key)
            .map(|i| i.terminal.as_str())
            .unwrap_or("○")
    }

    /// Get terminal icon for a layer.
    pub fn layer_terminal(&self, key: &str) -> &str {
        self.layers
            .get(key)
            .map(|i| i.terminal.as_str())
            .unwrap_or("·")
    }

    /// Get terminal icon for a trait.
    pub fn trait_terminal(&self, key: &str) -> &str {
        self.traits
            .get(key)
            .map(|i| i.terminal.as_str())
            .unwrap_or("·")
    }

    /// Get terminal icon for a UI state.
    pub fn state_terminal(&self, key: &str) -> &str {
        self.states
            .get(key)
            .map(|i| i.terminal.as_str())
            .unwrap_or("·")
    }

    /// Get terminal icon for navigation.
    pub fn nav_terminal(&self, key: &str) -> &str {
        self.navigation
            .get(key)
            .map(|i| i.terminal.as_str())
            .unwrap_or("·")
    }

    /// Get terminal icon for quality indicator.
    pub fn quality_terminal(&self, key: &str) -> &str {
        self.quality
            .get(key)
            .map(|i| i.terminal.as_str())
            .unwrap_or("·")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load visual encoding from visual-encoding.yaml.
pub fn load_visual_encoding(root: &Path) -> crate::Result<VisualEncodingDoc> {
    let path = root.join("packages/core/models/visual-encoding.yaml");
    let doc: VisualEncodingDoc = super::utils::load_yaml(&path)?;

    // Fail-fast validation
    if doc.node_states.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "visual-encoding.yaml has no node_states".to_string(),
        ));
    }
    if doc.trait_borders.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "visual-encoding.yaml has no trait_borders".to_string(),
        ));
    }
    if doc.class_icons.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "visual-encoding.yaml has no class_icons".to_string(),
        ));
    }

    Ok(doc)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_visual_encoding_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_visual_encoding(root).expect("should load visual-encoding.yaml");

        // Version (v11.6: Navigation redesign)
        assert_eq!(doc.version, "0.12.0");

        // Channel mapping
        assert_eq!(doc.channel_mapping.node.fill_color, "layer");
        assert_eq!(doc.channel_mapping.node.border_style, "trait");
        assert_eq!(doc.channel_mapping.arc.stroke_color, "family");

        // Node states (5)
        assert!(doc.node_states.contains_key("default"));
        assert!(doc.node_states.contains_key("focused"));
        assert!(doc.node_states.contains_key("hover"));
        assert!(doc.node_states.contains_key("selected"));
        assert!(doc.node_states.contains_key("filtered"));

        // Trait borders (5) — v0.12.0: renamed (ADR-024 Data Origin)
        assert!(doc.trait_borders.contains_key("defined"));
        assert!(doc.trait_borders.contains_key("authored"));
        assert!(doc.trait_borders.contains_key("imported"));
        assert!(doc.trait_borders.contains_key("generated"));
        assert!(doc.trait_borders.contains_key("retrieved"));

        // Kind icons (44+)
        assert!(doc.class_icons.len() >= 30);
        assert_eq!(doc.class_icons.get("Locale"), Some(&"globe".to_string()));
        assert_eq!(doc.class_icons.get("Page"), Some(&"file-text".to_string()));

        // Accessibility
        assert!(doc.accessibility.min_contrast_ratio >= 4.5);
        assert!(doc.accessibility.use_icons);

        // Icon system (v10.6)
        let icons = doc.icons.expect("should have icons section");

        // Realms (2)
        assert!(icons.realms.contains_key("shared"));
        assert!(icons.realms.contains_key("org"));
        assert_eq!(icons.realm_terminal("shared"), "◉");
        assert_eq!(icons.realm_terminal("org"), "◎");

        // Layers (10) — v11.5: 4 shared + 6 org (geo removed, SEO/GEO consolidated to knowledge)
        assert!(icons.layers.contains_key("config"));
        assert!(icons.layers.contains_key("locale"));
        assert!(icons.layers.contains_key("geography"));
        assert!(icons.layers.contains_key("knowledge"));
        assert!(icons.layers.contains_key("output"));
        assert_eq!(icons.layer_terminal("config"), "⚙");

        // Traits (5) — v0.12.0: renamed (ADR-024 Data Origin)
        assert!(icons.traits.contains_key("defined"));
        assert!(icons.traits.contains_key("authored"));
        assert!(icons.traits.contains_key("imported"));
        assert!(icons.traits.contains_key("generated"));
        assert!(icons.traits.contains_key("retrieved"));
        assert_eq!(icons.trait_terminal("defined"), "■");
        assert_eq!(icons.trait_terminal("generated"), "★");

        // States (8)
        assert!(icons.states.contains_key("loading"));
        assert!(icons.states.contains_key("no_kinds"));
        assert_eq!(icons.state_terminal("loading"), "◐");
        assert_eq!(icons.state_terminal("no_kinds"), "∅");

        // Navigation (7)
        assert!(icons.navigation.contains_key("expanded"));
        assert!(icons.navigation.contains_key("collapsed"));
        assert_eq!(icons.nav_terminal("expanded"), "▼");
        assert_eq!(icons.nav_terminal("collapsed"), "▶");

        // Quality (6)
        assert!(icons.quality.contains_key("complete"));
        assert!(icons.quality.contains_key("required"));
        assert_eq!(icons.quality_terminal("required"), "*");

        // Modes (6: 4 Studio + 2 TUI)
        assert!(icons.modes.contains_key("meta"));
        assert!(icons.modes.contains_key("graph"));
        assert!(icons.modes.contains_key("nexus"));
    }
}
