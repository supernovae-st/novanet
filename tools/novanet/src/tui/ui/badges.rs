//! Icon and badge classification helpers for TUI rendering.
//!
//! Pure functions mapping realm/layer/arc-family/cardinality keys
//! to their terminal icon or short label. Source of truth: `icons.rs`
//! (generated from `visual-encoding.yaml`).

use super::super::icons;

/// Get icon for realm badge (from visual-encoding.yaml via icons.rs).
/// Named `_badge` to avoid collision with expand_icon variables in tree.rs.
/// Uses icons.rs as source of truth.
pub(crate) fn realm_badge_icon(realm_key: &str) -> &'static str {
    match realm_key {
        "shared" => icons::REALMS_SHARED.terminal,
        "org" => icons::REALMS_ORG.terminal,
        _ => "○",
    }
}

/// Get icon for layer badge (from visual-encoding.yaml via icons.rs).
/// All icons are single-width Unicode symbols (no emojis).
/// Uses icons.rs as source of truth.
pub(crate) fn layer_badge_icon(layer_key: &str) -> &'static str {
    match layer_key {
        // 4 shared layers
        "config" => icons::LAYERS_CONFIG.terminal,
        "locale" => icons::LAYERS_LOCALE.terminal,
        "geography" => icons::LAYERS_GEOGRAPHY.terminal,
        "knowledge" => icons::LAYERS_KNOWLEDGE.terminal,
        // 6 org layers
        "foundation" => icons::LAYERS_FOUNDATION.terminal,
        "structure" => icons::LAYERS_STRUCTURE.terminal,
        "semantic" => icons::LAYERS_SEMANTIC.terminal,
        "instruction" => icons::LAYERS_INSTRUCTION.terminal,
        "output" => icons::LAYERS_OUTPUT.terminal,
        _ => "○",
    }
}

/// Get icon for arc family badge (from visual-encoding.yaml via icons.rs).
/// Uses icons.rs as source of truth.
pub(crate) fn arc_family_badge_icon(family_key: &str) -> &'static str {
    match family_key {
        "ownership" => icons::ARC_FAMILIES_OWNERSHIP.terminal,
        "localization" => icons::ARC_FAMILIES_LOCALIZATION.terminal,
        "semantic" => icons::ARC_FAMILIES_SEMANTIC.terminal,
        "generation" => icons::ARC_FAMILIES_GENERATION.terminal,
        "mining" => icons::ARC_FAMILIES_MINING.terminal,
        _ => "?",
    }
}

/// Get short abbreviation for cardinality display.
pub(crate) fn cardinality_abbrev(cardinality: &str) -> &'static str {
    match cardinality {
        "zero_to_one" => "0:1",
        "one_to_one" => "1:1",
        "one_to_many" => "1:N",
        "many_to_one" => "N:1",
        "many_to_many" => "N:M",
        _ => "?:?",
    }
}
