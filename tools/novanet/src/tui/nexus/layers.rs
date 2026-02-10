//! Layers Tab — Split view showing Shared and Org realms.
//!
//! Layout (v11.3):
//! - Left side: SHARED realm (3 layers: locale, geography, knowledge)
//! - Right side: ORG realm (8 layers: config, foundation, structure, semantic, instruction, seo, geo, output)
//!
//! Each layer card shows:
//! - Icon (from theme)
//! - Layer name
//! - Brief description
//! - Number of kinds in that layer

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::data::TaxonomyTree;
use crate::tui::theme::heatmap_color;
use crate::tui::theme::Theme;

// =============================================================================
// LAYER DEFINITIONS
// =============================================================================

/// Layer info for rendering.
#[derive(Debug, Clone)]
pub struct LayerCardInfo {
    /// Layer key (e.g., "config", "foundation").
    pub key: String,
    /// Icon character for the layer.
    pub icon: &'static str,
    /// Brief description of the layer.
    pub description: &'static str,
    /// Number of kinds in this layer.
    pub kind_count: usize,
}

/// Shared realm layers (3 layers) — v11.3: locale, geography, knowledge.
pub const SHARED_LAYERS: [(&str, &str, &str); 3] = [
    ("locale", "\u{1f310}", "Locale definitions"),     // 🌐
    ("geography", "\u{1f5fa}", "Geographic data"),     // 🗺️
    ("knowledge", "\u{1f4da}", "Terms, Patterns"),     // 📚
];

/// Org realm layers (8 layers) — v11.3: +geo for AI visibility.
pub const ORG_LAYERS: [(&str, &str, &str); 8] = [
    ("config", "\u{2699}", "OrgConfig root"),          // ⚙
    ("foundation", "\u{25c7}", "Project, Brand"),      // ◇
    ("structure", "\u{25c6}", "Pages, Blocks"),        // ◆
    ("semantic", "\u{25c6}", "Entities, Personas"),    // ◆
    ("instruction", "\u{270e}", "Prompts, Rules"),     // ✎
    ("seo", "\u{1f50d}", "SEO Keywords"),              // 🔍
    ("geo", "\u{1f916}", "GEO AI Visibility"),         // 🤖
    ("output", "\u{25cf}", "Generated content"),       // ●
];

impl TaxonomyTree {
    /// Get layer stats for a specific realm.
    /// Returns a vector of (layer_key, kind_count) tuples.
    pub fn get_layer_stats(&self, realm_key: &str) -> Vec<(String, usize)> {
        self.realms
            .iter()
            .find(|r| r.key == realm_key)
            .map(|realm| {
                realm
                    .layers
                    .iter()
                    .map(|layer| (layer.key.clone(), layer.kinds.len()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Layers tab with split view (Shared | Org).
pub fn render_layers_tab(f: &mut Frame, app: &App, area: Rect) {
    // Split into two columns: Shared (left) and Org (right)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    render_realm_column(f, app, chunks[0], "shared", "SHARED REALM", 0);
    render_realm_column(f, app, chunks[1], "org", "ORG REALM", 1);
}

/// Render a single realm column with its layers.
fn render_realm_column(
    f: &mut Frame,
    app: &App,
    area: Rect,
    realm_key: &str,
    title: &str,
    realm_idx: usize,
) {
    let theme = &app.theme;
    let is_selected_realm = app.nexus.layer_realm == realm_idx;

    // Get realm color for border
    let realm_color = theme.realm_color(realm_key);
    let border_color = if is_selected_realm {
        realm_color
    } else {
        Color::Rgb(60, 60, 70)
    };

    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(realm_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Get layer stats for this realm
    let layer_stats = app.tree.get_layer_stats(realm_key);

    // Calculate max count for heatmap scaling
    let max_count = layer_stats.iter().map(|(_, c)| *c).max().unwrap_or(1);

    // Build layer cards
    let layers = if realm_key == "shared" {
        &SHARED_LAYERS[..]
    } else {
        &ORG_LAYERS[..]
    };

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Header separator
    lines.push(Line::from(Span::styled(
        "\u{2500}".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(Color::Rgb(60, 60, 70)),
    )));
    lines.push(Line::from(""));

    // Render each layer card
    for (idx, (layer_key, icon, description)) in layers.iter().enumerate() {
        let kind_count = layer_stats
            .iter()
            .find(|(k, _)| k == *layer_key)
            .map(|(_, count)| *count)
            .unwrap_or(0);

        let is_selected = is_selected_realm && app.nexus.layer_cursor == idx;

        let card_lines = build_layer_card(
            layer_key,
            icon,
            description,
            kind_count,
            max_count,
            is_selected,
            theme,
        );

        for line in card_lines {
            lines.push(line);
        }
        lines.push(Line::from(""));
    }

    // Navigation hint at bottom
    if is_selected_realm {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "[j/k] Navigate   [h/l] Switch realm",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Build a single layer card with box drawing.
fn build_layer_card(
    layer_key: &str,
    icon: &str,
    description: &str,
    kind_count: usize,
    max_count: usize,
    is_selected: bool,
    theme: &Theme,
) -> Vec<Line<'static>> {
    let layer_color = theme.layer_color(layer_key);

    // Style based on selection
    let (border_style, content_style) = if is_selected {
        (
            Style::default()
                .fg(layer_color)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            Style::default().fg(Color::Rgb(80, 80, 90)),
            Style::default().fg(Color::Rgb(150, 150, 160)),
        )
    };

    // Card width (fixed for consistency)
    let card_width: usize = 23;

    // Build card lines using box drawing characters
    let mut lines = Vec::new();

    // Top border: ┌─────────────────────┐
    let top_border = format!(
        "\u{250c}{}\u{2510}",
        "\u{2500}".repeat(card_width.saturating_sub(2))
    );
    lines.push(Line::from(Span::styled(top_border, border_style)));

    // Row 1: │ ⚙ config            │
    let icon_name = format!("{} {}", icon, layer_key);
    let padding = card_width.saturating_sub(icon_name.chars().count() + 4);
    lines.push(Line::from(vec![
        Span::styled("\u{2502} ", border_style),
        Span::styled(
            format!("{} {}", icon, layer_key),
            if is_selected {
                Style::default()
                    .fg(layer_color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(layer_color)
            },
        ),
        Span::styled(
            format!("{} \u{2502}", " ".repeat(padding.max(0))),
            border_style,
        ),
    ]));

    // Row 2: │   Description       │
    let desc_padding = card_width.saturating_sub(description.chars().count() + 6);
    lines.push(Line::from(vec![
        Span::styled("\u{2502}   ", border_style),
        Span::styled(description.to_string(), content_style),
        Span::styled(
            format!("{} \u{2502}", " ".repeat(desc_padding.max(0))),
            border_style,
        ),
    ]));

    // Row 3: │   N kinds           │ (heatmap color based on density)
    let count_str = format!("{} kinds", kind_count);
    let count_color = heatmap_color(kind_count, max_count);
    let count_padding = card_width.saturating_sub(count_str.chars().count() + 6);
    lines.push(Line::from(vec![
        Span::styled("\u{2502}   ", border_style),
        Span::styled(count_str, Style::default().fg(count_color)),
        Span::styled(
            format!("{} \u{2502}", " ".repeat(count_padding.max(0))),
            border_style,
        ),
    ]));

    // Bottom border: └─────────────────────┘
    let bottom_border = format!(
        "\u{2514}{}\u{2518}",
        "\u{2500}".repeat(card_width.saturating_sub(2))
    );
    lines.push(Line::from(Span::styled(bottom_border, border_style)));

    lines
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_layers_count() {
        // v11.3: SHARED has 3 layers (locale, geography, knowledge)
        assert_eq!(SHARED_LAYERS.len(), 3);
    }

    #[test]
    fn test_tenant_layers_count() {
        // v11.3: ORG has 8 layers (config, foundation, structure, semantic, instruction, seo, geo, output)
        assert_eq!(ORG_LAYERS.len(), 8);
    }

    #[test]
    fn test_layer_keys_valid() {
        for (key, _, _) in SHARED_LAYERS {
            assert!(!key.is_empty());
        }
        for (key, _, _) in ORG_LAYERS {
            assert!(!key.is_empty());
        }
    }

    #[test]
    fn test_layer_icons_valid() {
        for (_, icon, _) in SHARED_LAYERS {
            assert!(!icon.is_empty());
        }
        for (_, icon, _) in ORG_LAYERS {
            assert!(!icon.is_empty());
        }
    }

    #[test]
    fn test_build_layer_card() {
        let theme = Theme::new();
        // max_count=20 for heatmap scaling
        let lines = build_layer_card("config", "\u{2699}", "System settings", 5, 20, false, &theme);
        assert_eq!(lines.len(), 5); // top border + 3 rows + bottom border
    }

    #[test]
    fn test_build_layer_card_selected() {
        let theme = Theme::new();
        // max_count=20 for heatmap scaling
        let lines = build_layer_card("foundation", "\u{25c7}", "Entities", 10, 20, true, &theme);
        assert_eq!(lines.len(), 5);
    }
}
