//! Visual encoding parser — reads visual-encoding.yaml.
//!
//! Parses the visual presentation rules for NovaNet graph elements:
//! - Channel mapping (which visual property encodes which facet)
//! - Node/arc states (opacity, scale, shadow, etc.)
//! - Trait border styles (solid, dashed, dotted, double)
//! - Scope stroke styles (intra/cross realm)
//! - Cardinality arrow heads
//! - Kind icons (Lucide icon mapping)
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
    pub kind_icons: HashMap<String, String>,
    pub animations: HashMap<String, Animation>,
    pub accessibility: AccessibilitySettings,
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
    if doc.kind_icons.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "visual-encoding.yaml has no kind_icons".to_string(),
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

        // Version (v10.3: Entity-Centric Architecture)
        assert_eq!(doc.version, "10.3.0");

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

        // Trait borders (5)
        assert!(doc.trait_borders.contains_key("invariant"));
        assert!(doc.trait_borders.contains_key("localized"));
        assert!(doc.trait_borders.contains_key("knowledge"));
        assert!(doc.trait_borders.contains_key("derived"));
        assert!(doc.trait_borders.contains_key("job"));

        // Kind icons (44+)
        assert!(doc.kind_icons.len() >= 30);
        assert_eq!(doc.kind_icons.get("Locale"), Some(&"globe".to_string()));
        assert_eq!(doc.kind_icons.get("Page"), Some(&"file-text".to_string()));

        // Accessibility
        assert!(doc.accessibility.min_contrast_ratio >= 4.5);
        assert!(doc.accessibility.use_icons);
    }
}
