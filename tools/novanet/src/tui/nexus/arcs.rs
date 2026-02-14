//! Arcs Tab — Arc families and scope visualization.
//!
//! Layout:
//! - Top: Arc families grid (5 families: ownership, localization, semantic, generation, mining)
//! - Bottom: Arc scope section (intra_realm vs cross_realm)
//!
//! Each family card shows:
//! - Icon (from theme)
//! - Family name
//! - Example arc types
//! - Number of arcs in that family

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::data::TaxonomyTree;
use crate::tui::theme::Theme;
use crate::tui::ui::COLOR_UNFOCUSED_BORDER;
use crate::tui::unicode::truncate_to_width;

// =============================================================================
// ARC FAMILY DEFINITIONS
// =============================================================================

/// Canonical order for arc families.
pub const ARC_FAMILY_ORDER: [&str; 5] = [
    "ownership",
    "localization",
    "semantic",
    "generation",
    "mining",
];

/// Arc family info for rendering.
#[derive(Debug, Clone)]
pub struct ArcFamilyCard {
    /// Family key (e.g., "ownership", "localization").
    pub key: String,
    /// Display name (e.g., "Ownership", "Localization").
    pub display_name: String,
    /// Icon character for the family.
    pub icon: &'static str,
    /// Example arc types.
    pub examples: Vec<String>,
    /// Number of arc classes in this family.
    pub arc_count: usize,
    /// LLM context description.
    pub description: &'static str,
}

/// Get symbol for an arc family.
fn arc_family_symbol(key: &str) -> &'static str {
    match key {
        "ownership" => "\u{2192}",    // →
        "localization" => "\u{21e2}", // ⇢
        "semantic" => "\u{007e}",     // ~
        "generation" => "\u{21d2}",   // ⇒
        "mining" => "\u{21dd}",       // ⇝
        _ => "\u{2192}",              // →
    }
}

/// Get display name for an arc family.
fn arc_family_display_name(key: &str) -> &str {
    match key {
        "ownership" => "OWNERSHIP",
        "localization" => "LOCALIZATION",
        "semantic" => "SEMANTIC",
        "generation" => "GENERATION",
        "mining" => "MINING",
        _ => key,
    }
}

/// Get description for an arc family.
fn arc_family_description(key: &str) -> &str {
    match key {
        "ownership" => {
            "Hierarchical containment. Parent-child relationships like Page HAS_BLOCK, Org HAS_PAGE."
        }
        "localization" => {
            "Locale-specific content. Links defined to authored nodes like Entity HAS_CONTENT EntityContent."
        }
        "semantic" => {
            "Semantic relationships. Content uses imported atoms like Block USES_TERM, USES_EXPRESSION."
        }
        "generation" => "LLM generation flow. GENERATES, PRODUCES arcs tracking what creates what.",
        "mining" => "Data extraction and derivation. EXTRACTS, DERIVES arcs for computed content.",
        _ => "Unknown arc family.",
    }
}

/// Get example arc types for a family (fallback if no data).
fn arc_family_examples(key: &str) -> Vec<&'static str> {
    match key {
        "ownership" => vec!["HAS_PAGE", "HAS_BLOCK", "HAS_ENTITY"],
        "localization" => vec!["HAS_CONTENT", "FOR_LOCALE"],
        "semantic" => vec!["USES_TERM", "REFERENCES", "USES_EXPRESSION"],
        "generation" => vec!["GENERATES", "PRODUCES"],
        "mining" => vec!["EXTRACTS", "DERIVES"],
        _ => vec![],
    }
}

impl TaxonomyTree {
    /// Build arc family cards from the loaded taxonomy tree.
    pub fn get_arc_family_cards(&self) -> Vec<ArcFamilyCard> {
        // Build a map from loaded arc families for quick lookup
        let family_map: std::collections::HashMap<&str, &crate::tui::data::ArcFamilyInfo> = self
            .arc_families
            .iter()
            .map(|f| (f.key.as_str(), f))
            .collect();

        // Build cards in canonical order
        ARC_FAMILY_ORDER
            .iter()
            .map(|&key| {
                let (arc_count, examples) = if let Some(info) = family_map.get(key) {
                    let count = info.arc_classes.len();
                    let examples: Vec<String> = info
                        .arc_classes
                        .iter()
                        .take(3) // Show up to 3 examples
                        .map(|ak| ak.key.clone())
                        .collect();
                    (count, examples)
                } else {
                    // Fallback to static examples
                    let static_examples: Vec<String> = arc_family_examples(key)
                        .iter()
                        .map(|s| (*s).to_string())
                        .collect();
                    (0, static_examples)
                };

                ArcFamilyCard {
                    key: key.to_string(),
                    display_name: arc_family_display_name(key).to_string(),
                    icon: arc_family_symbol(key),
                    examples,
                    arc_count,
                    description: arc_family_description(key),
                }
            })
            .collect()
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Arcs tab with families grid and scope section.
pub fn render_arcs_tab(f: &mut Frame, app: &App, area: Rect) {
    // Split into families (top 70%) and scope (bottom 30%)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    render_arc_families(f, app, chunks[0]);
    render_arc_scope(f, app, chunks[1]);
}

/// Render the arc families grid.
fn render_arc_families(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let cards = app.tree.get_arc_family_cards();
    let selected_idx = app.nexus.arc_cursor;

    let block = Block::default()
        .title(Span::styled(
            " ARC FAMILIES ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split into 2 rows: top row (3 families), bottom row (2 families)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    // Top row: ownership, localization, semantic
    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(rows[0]);

    for (i, card) in cards.iter().take(3).enumerate() {
        let is_selected = i == selected_idx;
        render_family_card(f, card, top_cols[i], is_selected, theme);
    }

    // Bottom row: generation, mining (centered)
    let bottom_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(17), // left margin
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(17), // right margin
        ])
        .split(rows[1]);

    for (i, card) in cards.iter().skip(3).take(2).enumerate() {
        let is_selected = (i + 3) == selected_idx;
        render_family_card(f, card, bottom_cols[i + 1], is_selected, theme);
    }
}

/// Render a single arc family card.
fn render_family_card(
    f: &mut Frame,
    card: &ArcFamilyCard,
    area: Rect,
    is_selected: bool,
    theme: &Theme,
) {
    let family_color = theme.arc_family_color(&card.key);

    // Style based on selection
    let (border_style, title_style, content_style) = if is_selected {
        (
            Style::default()
                .fg(family_color)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .fg(family_color)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            Style::default().fg(COLOR_UNFOCUSED_BORDER),
            Style::default().fg(family_color),
            Style::default().fg(Color::Rgb(150, 150, 160)),
        )
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Row 1: Icon + Family name
    lines.push(Line::from(vec![
        Span::styled(format!("{} ", card.icon), title_style),
        Span::styled(card.display_name.clone(), title_style),
    ]));

    // Row 2: Example arc types (unicode-aware truncation)
    let examples_str = card.examples.join(", ");
    let max_width = (inner.width as usize).saturating_sub(2);
    let truncated = truncate_to_width(&examples_str, max_width);
    lines.push(Line::from(Span::styled(truncated, content_style)));

    // Row 3: Arc count
    let count_str = format!("{} arcs", card.arc_count);
    lines.push(Line::from(Span::styled(
        count_str,
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(paragraph, inner);
}

/// Render the arc scope section.
fn render_arc_scope(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " ARC SCOPE ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Header
    lines.push(Line::from(Span::styled(
        "How arcs cross realm boundaries:",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Scope explanations with visual representation
    let shared_color = theme.realm_color("shared");
    let org_color = theme.realm_color("org");

    // intra_realm (solid line)
    lines.push(Line::from(vec![
        Span::styled(
            "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::Yellow),
        ), // ──────
        Span::styled(
            "  intra_realm  ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "Within same realm",
            Style::default().fg(Color::Rgb(150, 150, 160)),
        ),
    ]));

    // Example for intra_realm
    lines.push(Line::from(vec![
        Span::raw("        "),
        Span::styled("Page", Style::default().fg(org_color)),
        Span::styled(
            " \u{2500}\u{2500}\u{2500}[\u{2192}]\u{2500}\u{2500}\u{2500} ",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled("Block", Style::default().fg(org_color)),
        Span::styled("  (both in org)", Style::default().fg(Color::DarkGray)),
    ]));

    lines.push(Line::from(""));

    // cross_realm (dashed line)
    lines.push(Line::from(vec![
        Span::styled(
            "\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}\u{254c}",
            Style::default().fg(Color::Magenta),
        ), // ╌╌╌╌╌╌
        Span::styled(
            "  cross_realm  ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "Between shared <-> org",
            Style::default().fg(Color::Rgb(150, 150, 160)),
        ),
    ]));

    // Example for cross_realm
    lines.push(Line::from(vec![
        Span::raw("        "),
        Span::styled("Locale", Style::default().fg(shared_color)),
        Span::styled(
            " \u{254c}\u{254c}\u{254c}[\u{21e2}]\u{254c}\u{254c}\u{254c} ",
            Style::default().fg(Color::Magenta),
        ),
        Span::styled("PageGenerated", Style::default().fg(org_color)),
        Span::styled("  (shared -> org)", Style::default().fg(Color::DarkGray)),
    ]));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_family_order() {
        assert_eq!(ARC_FAMILY_ORDER.len(), 5);
        assert_eq!(ARC_FAMILY_ORDER[0], "ownership");
        assert_eq!(ARC_FAMILY_ORDER[1], "localization");
        assert_eq!(ARC_FAMILY_ORDER[2], "semantic");
        assert_eq!(ARC_FAMILY_ORDER[3], "generation");
        assert_eq!(ARC_FAMILY_ORDER[4], "mining");
    }

    #[test]
    fn test_arc_family_symbols() {
        assert_eq!(arc_family_symbol("ownership"), "\u{2192}");
        assert_eq!(arc_family_symbol("localization"), "\u{21e2}");
        assert_eq!(arc_family_symbol("semantic"), "\u{007e}");
        assert_eq!(arc_family_symbol("generation"), "\u{21d2}");
        assert_eq!(arc_family_symbol("mining"), "\u{21dd}");
        assert_eq!(arc_family_symbol("unknown"), "\u{2192}"); // fallback
    }

    #[test]
    fn test_arc_family_display_names() {
        assert_eq!(arc_family_display_name("ownership"), "OWNERSHIP");
        assert_eq!(arc_family_display_name("localization"), "LOCALIZATION");
        assert_eq!(arc_family_display_name("semantic"), "SEMANTIC");
        assert_eq!(arc_family_display_name("generation"), "GENERATION");
        assert_eq!(arc_family_display_name("mining"), "MINING");
    }

    #[test]
    fn test_arc_family_examples() {
        let examples = arc_family_examples("ownership");
        assert!(examples.contains(&"HAS_PAGE"));
        assert!(examples.contains(&"HAS_BLOCK"));

        let examples = arc_family_examples("semantic");
        assert!(examples.contains(&"USES_TERM"));
    }

    #[test]
    fn test_arc_family_descriptions() {
        let desc = arc_family_description("ownership");
        assert!(desc.contains("Hierarchical"));

        let desc = arc_family_description("localization");
        assert!(desc.contains("Locale"));
    }
}
