//! Semantic color mapping for YAML panel.
//!
//! Maps schema concepts (realm, layer, arc family, scope, cardinality)
//! to their canonical display colors.

use ratatui::style::Color;

use crate::tui::palette;

// =============================================================================
// INSTANCE PANEL COLORS (rich YAML-style)
// =============================================================================

/// YAML key color (cyan)
pub(super) const COLOR_YAML_KEY: Color = palette::YAML_KEY;
/// YAML string color (yellow/gold)
pub(super) const COLOR_YAML_STRING: Color = palette::YAML_STRING;
/// YAML number color (orange)
pub(super) const COLOR_YAML_NUMBER: Color = palette::YAML_NUMBER;
/// YAML boolean/null color (violet)
pub(super) const COLOR_YAML_BOOL: Color = palette::YAML_BOOL;
/// Section header color (muted)
pub(super) const COLOR_SECTION_HEADER: Color = palette::YAML_SECTION_HEADER;

// =============================================================================
// SEMANTIC COLORS
// =============================================================================

/// Realm colors
const COLOR_REALM_SHARED: Color = palette::REALM_SHARED;
const COLOR_REALM_ORG: Color = palette::REALM_ORG;

/// Layer colors (subset)
const COLOR_LAYER_SEMANTIC: Color = palette::LAYER_SEMANTIC;
const COLOR_LAYER_OUTPUT: Color = palette::LAYER_OUTPUT;
const COLOR_LAYER_KNOWLEDGE: Color = palette::LAYER_KNOWLEDGE;

/// Arc family colors
const COLOR_FAMILY_OWNERSHIP: Color = palette::FAMILY_OWNERSHIP;
const COLOR_FAMILY_SEMANTIC: Color = palette::FAMILY_SEMANTIC;
const COLOR_FAMILY_GENERATION: Color = palette::FAMILY_GENERATION;
const COLOR_FAMILY_LOCALIZATION: Color = palette::FAMILY_LOCALIZATION;
const COLOR_FAMILY_MINING: Color = palette::FAMILY_MINING;

/// Get realm color from key.
pub(super) fn realm_color(key: &str) -> Color {
    match key {
        "shared" => COLOR_REALM_SHARED,
        "org" => COLOR_REALM_ORG,
        _ => Color::White,
    }
}

/// Get arc family color from key.
pub(super) fn arc_family_color(family: &str) -> Color {
    match family {
        "ownership" => COLOR_FAMILY_OWNERSHIP,
        "semantic" => COLOR_FAMILY_SEMANTIC,
        "generation" => COLOR_FAMILY_GENERATION,
        "localization" => COLOR_FAMILY_LOCALIZATION,
        "mining" => COLOR_FAMILY_MINING,
        _ => Color::White,
    }
}

/// Get layer color from key.
pub(super) fn layer_color(layer: &str) -> Color {
    match layer {
        "config" => palette::LAYER_CONFIG,
        "locale" => palette::LAYER_LOCALE,
        "geography" => palette::LAYER_GEOGRAPHY,
        "knowledge" => COLOR_LAYER_KNOWLEDGE,
        "foundation" => palette::LAYER_FOUNDATION,
        "structure" => palette::LAYER_STRUCTURE,
        "semantic" => COLOR_LAYER_SEMANTIC,
        "instruction" => palette::LAYER_INSTRUCTION,
        "output" => COLOR_LAYER_OUTPUT,
        _ => Color::White,
    }
}

/// Get arc scope color.
pub(super) fn scope_color(scope: &str) -> Color {
    match scope {
        "intra_realm" => palette::SOLARIZED_CYAN,
        "cross_realm" => palette::ORANGE_500,
        _ => Color::White,
    }
}

/// Get cardinality color.
pub(super) fn cardinality_color(cardinality: &str) -> Color {
    match cardinality {
        "one_to_one" | "1:1" => palette::GREEN_500,
        "one_to_many" | "1:N" => palette::BLUE_500,
        "many_to_one" | "N:1" => palette::PURPLE_500,
        "many_to_many" | "N:M" => palette::ORANGE_500,
        _ => Color::White,
    }
}

/// Check if a YAML key should have semantic coloring for its value.
/// Returns Some(color) if the key is semantic, None otherwise.
pub(super) fn semantic_value_color(key: &str, value: &str) -> Option<Color> {
    let key_trimmed = key.trim().trim_end_matches(':');
    let value_trimmed = value.trim();

    match key_trimmed {
        "realm" => Some(realm_color(value_trimmed)),
        "layer" => Some(layer_color(value_trimmed)),
        "family" => Some(arc_family_color(value_trimmed)),
        "scope" => Some(scope_color(value_trimmed)),
        "cardinality" => Some(cardinality_color(value_trimmed)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_value_color_realm_shared() {
        let color = semantic_value_color("realm", " shared");
        assert_eq!(color, Some(COLOR_REALM_SHARED));
    }

    #[test]
    fn test_semantic_value_color_realm_org() {
        let color = semantic_value_color("realm", " org");
        assert_eq!(color, Some(COLOR_REALM_ORG));
    }

    #[test]
    fn test_semantic_value_color_layer_semantic() {
        let color = semantic_value_color("layer", " semantic");
        assert_eq!(color, Some(COLOR_LAYER_SEMANTIC));
    }

    #[test]
    fn test_semantic_value_color_layer_output() {
        let color = semantic_value_color("layer", " output");
        assert_eq!(color, Some(COLOR_LAYER_OUTPUT));
    }

    #[test]
    fn test_semantic_value_color_family_ownership() {
        let color = semantic_value_color("family", " ownership");
        assert_eq!(color, Some(COLOR_FAMILY_OWNERSHIP));
    }

    #[test]
    fn test_semantic_value_color_family_semantic() {
        let color = semantic_value_color("family", " semantic");
        assert_eq!(color, Some(COLOR_FAMILY_SEMANTIC));
    }

    #[test]
    fn test_semantic_value_color_scope_intra() {
        let color = semantic_value_color("scope", " intra_realm");
        assert!(color.is_some());
    }

    #[test]
    fn test_semantic_value_color_scope_cross() {
        let color = semantic_value_color("scope", " cross_realm");
        assert!(color.is_some());
    }

    #[test]
    fn test_semantic_value_color_cardinality() {
        let color = semantic_value_color("cardinality", " one_to_many");
        assert!(color.is_some());
    }

    #[test]
    fn test_semantic_value_color_non_semantic_key() {
        let color = semantic_value_color("name", " Page");
        assert_eq!(color, None);
    }

    #[test]
    fn test_semantic_value_color_with_colon() {
        let color = semantic_value_color("realm:", " shared");
        assert_eq!(color, Some(COLOR_REALM_SHARED));
    }

    #[test]
    fn test_layer_color_knowledge() {
        assert_eq!(layer_color("knowledge"), COLOR_LAYER_KNOWLEDGE);
    }

    #[test]
    fn test_layer_color_semantic() {
        assert_eq!(layer_color("semantic"), COLOR_LAYER_SEMANTIC);
    }

    #[test]
    fn test_layer_color_output() {
        assert_eq!(layer_color("output"), COLOR_LAYER_OUTPUT);
    }

    #[test]
    fn test_layer_color_unknown() {
        assert_eq!(layer_color("unknown"), Color::White);
    }

    #[test]
    fn test_cardinality_color_one_to_one() {
        let color = cardinality_color("one_to_one");
        assert_eq!(color, Color::Rgb(34, 197, 94));
    }

    #[test]
    fn test_cardinality_color_one_to_many() {
        let color = cardinality_color("one_to_many");
        assert_eq!(color, Color::Rgb(59, 130, 246));
    }

    #[test]
    fn test_cardinality_color_many_to_many() {
        let color = cardinality_color("many_to_many");
        assert_eq!(color, Color::Rgb(249, 115, 22));
    }
}
