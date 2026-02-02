//! SuperNovae Galaxy theme — deep space palette + style helpers.
//!
//! Color system:
//! - Background: deep space blues (void, panel, active, hover)
//! - Accents: nebula purple, cyber cyan, matrix green
//! - Signals: plasma pink (alerts), solar amber (warnings)
//! - Text: nova white (primary), star dim (secondary)
//! - Realms: solarized-inspired (global=teal, project=violet, shared=orange)

use ratatui::style::{Color, Modifier, Style};

use crate::tui::app::NavMode;

// ─── Background layer ─────────────────────────────────────────────────
pub const BG_VOID: Color = Color::Rgb(8, 10, 18);
pub const BG_PANEL: Color = Color::Rgb(13, 17, 30);
pub const BG_ACTIVE: Color = Color::Rgb(20, 25, 45);
#[allow(dead_code)] // Used in Phase 7B hover states
pub const BG_HOVER: Color = Color::Rgb(30, 35, 60);

// ─── Accent layer (nebula glow) ───────────────────────────────────────
pub const NEBULA_PURPLE: Color = Color::Rgb(139, 92, 246);
#[allow(dead_code)] // Used in Phase 7B animations
pub const NEBULA_VIOLET: Color = Color::Rgb(124, 58, 237);
pub const NEBULA_INDIGO: Color = Color::Rgb(99, 102, 241);
pub const NEBULA_BLUE: Color = Color::Rgb(59, 130, 246);

// ─── Signal layer (data highlights) ───────────────────────────────────
pub const CYBER_CYAN: Color = Color::Rgb(34, 211, 238);
#[allow(dead_code)] // Used in Phase 7B dashboard
pub const CYBER_TEAL: Color = Color::Rgb(20, 184, 166);
pub const MATRIX_GREEN: Color = Color::Rgb(34, 197, 94);
pub const PLASMA_PINK: Color = Color::Rgb(236, 72, 153);
pub const SOLAR_AMBER: Color = Color::Rgb(245, 158, 11);
pub const NOVA_WHITE: Color = Color::Rgb(226, 232, 240);
pub const STAR_DIM: Color = Color::Rgb(100, 116, 139);

// ─── Realm colors (spatial zones) ─────────────────────────────────────
pub const REALM_GLOBAL: Color = Color::Rgb(42, 161, 152);
pub const REALM_PROJECT: Color = Color::Rgb(108, 113, 196);
pub const REALM_SHARED: Color = Color::Rgb(203, 75, 22);

/// Map a realm key to its signature color.
pub fn realm_color(realm: &str) -> Color {
    match realm {
        "global" => REALM_GLOBAL,
        "project" => REALM_PROJECT,
        "shared" => REALM_SHARED,
        _ => STAR_DIM,
    }
}

/// Map a realm key to its emoji prefix.
pub fn realm_emoji(realm: &str) -> &'static str {
    match realm {
        "global" => "\u{1f30d} ",  // 🌍
        "project" => "\u{1f4e6} ", // 📦
        "shared" => "\u{1f3af} ",  // 🎯
        _ => "  ",
    }
}

/// Map a layer key to its color (softer accent tones).
pub fn layer_color(layer: &str) -> Color {
    match layer {
        "config" => CYBER_CYAN,
        "knowledge" => NEBULA_BLUE,
        "foundation" => NEBULA_PURPLE,
        "structure" => NEBULA_INDIGO,
        "semantic" => MATRIX_GREEN,
        "instruction" => SOLAR_AMBER,
        "output" => PLASMA_PINK,
        "seo" => CYBER_CYAN,
        "geo" => NEBULA_BLUE,
        _ => STAR_DIM,
    }
}

/// Map a trait key to its display style (border encoding).
#[allow(dead_code)] // Used in Phase 7B Task 5 (Kind detail pane)
pub fn trait_style(trait_key: &str) -> Style {
    match trait_key {
        "invariant" => Style::default().fg(CYBER_CYAN),
        "localized" => Style::default().fg(MATRIX_GREEN),
        "knowledge" => Style::default().fg(NEBULA_PURPLE),
        "derived" => Style::default().fg(STAR_DIM),
        "job" => Style::default().fg(SOLAR_AMBER),
        _ => Style::default().fg(STAR_DIM),
    }
}

/// Map an edge family key to its color.
#[allow(dead_code)] // Used in Phase 7B Task 8 (edge explorer)
pub fn family_color(family: &str) -> Color {
    match family {
        "ownership" => CYBER_CYAN,
        "localization" => MATRIX_GREEN,
        "semantic" => NEBULA_PURPLE,
        "generation" => SOLAR_AMBER,
        "mining" => PLASMA_PINK,
        _ => STAR_DIM,
    }
}

/// Map a navigation mode to its accent color.
pub fn mode_color(mode: NavMode) -> Color {
    match mode {
        NavMode::Data => CYBER_CYAN,
        NavMode::Meta => NEBULA_PURPLE,
        NavMode::Overlay => SOLAR_AMBER,
        NavMode::Query => MATRIX_GREEN,
    }
}

/// Panel border style (active vs inactive).
pub fn panel_border(active: bool) -> Style {
    if active {
        Style::default().fg(CYBER_CYAN)
    } else {
        Style::default().fg(STAR_DIM)
    }
}

/// Highlighted selection style for tree items.
pub fn selected_style(color: Color) -> Style {
    Style::default()
        .fg(color)
        .add_modifier(Modifier::BOLD | Modifier::REVERSED)
}

/// Normal tree item style.
pub fn tree_item_style(color: Color) -> Style {
    Style::default().fg(color)
}

/// Status bar background style.
pub fn status_bar_style() -> Style {
    Style::default().bg(BG_ACTIVE).fg(NOVA_WHITE)
}

/// Dim text for secondary info.
pub fn dim_style() -> Style {
    Style::default().fg(STAR_DIM)
}

/// Bold accent text.
pub fn accent_bold(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realm_colors_resolve() {
        assert_eq!(realm_color("global"), REALM_GLOBAL);
        assert_eq!(realm_color("project"), REALM_PROJECT);
        assert_eq!(realm_color("shared"), REALM_SHARED);
        assert_eq!(realm_color("unknown"), STAR_DIM);
    }

    #[test]
    fn mode_colors_resolve() {
        assert_eq!(mode_color(NavMode::Data), CYBER_CYAN);
        assert_eq!(mode_color(NavMode::Meta), NEBULA_PURPLE);
        assert_eq!(mode_color(NavMode::Overlay), SOLAR_AMBER);
        assert_eq!(mode_color(NavMode::Query), MATRIX_GREEN);
    }

    #[test]
    fn family_colors_resolve() {
        assert_eq!(family_color("ownership"), CYBER_CYAN);
        assert_eq!(family_color("semantic"), NEBULA_PURPLE);
        assert_eq!(family_color("mining"), PLASMA_PINK);
        assert_eq!(family_color("unknown"), STAR_DIM);
    }

    #[test]
    fn panel_border_active_vs_inactive() {
        let active = panel_border(true);
        let inactive = panel_border(false);
        assert_ne!(active, inactive);
    }
}
