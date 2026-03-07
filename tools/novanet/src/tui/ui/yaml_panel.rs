//! YAML panel rendering for TUI.
//!
//! This module handles the YAML preview panel with:
//! - Syntax highlighting for keys, values, comments, and punctuation
//! - Scrollbar for long content
//! - v0.13.0: Split into SOURCE and DIAGRAM boxes with visual states
//! - v0.13.1: Simplified - no collapse/peek (PROPERTIES panel shows instance data)
//! - v0.17.3: Context-aware content via ContentPanelMode (Phase 3)

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use super::{COLOR_MUTED_TEXT, STYLE_DIM, scroll_indicator};
use crate::tui::app::{App, ContentPanelMode};

// =============================================================================
// BOX VISUAL STATES v0.13 (enhanced palette)
// =============================================================================

/// Unfocused: Nord Polar Night (dim) - box is NOT selected
const BOX_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82); // #3B4252

/// Selected: Solarized Cyan (bright, active) - this specific box is Tab-selected
const BOX_BORDER_SELECTED: Color = Color::Rgb(42, 161, 152); // #2AA198

// =============================================================================
// v0.13 SEMANTIC COLORS
// =============================================================================

/// Realm colors
const COLOR_REALM_SHARED: Color = Color::Rgb(42, 161, 152); // #2AA198 Solarized Cyan
const COLOR_REALM_ORG: Color = Color::Rgb(108, 113, 196); // #6C71C4 Solarized Violet

/// Layer colors (subset)
const COLOR_LAYER_SEMANTIC: Color = Color::Rgb(249, 115, 22); // #F97316 Orange
const COLOR_LAYER_OUTPUT: Color = Color::Rgb(34, 197, 94); // #22C55E Green
const COLOR_LAYER_KNOWLEDGE: Color = Color::Rgb(139, 92, 246); // #8B5CF6 Violet

/// Trait colors (ADR-024 Data Origin)
const COLOR_TRAIT_DEFINED: Color = Color::Rgb(59, 130, 246); // #3B82F6 Blue
const COLOR_TRAIT_AUTHORED: Color = Color::Rgb(34, 197, 94); // #22C55E Green
const COLOR_TRAIT_IMPORTED: Color = Color::Rgb(139, 92, 246); // #8B5CF6 Violet
const COLOR_TRAIT_GENERATED: Color = Color::Rgb(181, 137, 0); // #B58900 Gold
const COLOR_TRAIT_RETRIEVED: Color = Color::Rgb(108, 113, 196); // #6C71C4 Violet

/// Arc family colors
const COLOR_FAMILY_OWNERSHIP: Color = Color::Rgb(59, 130, 246); // Blue
const COLOR_FAMILY_SEMANTIC: Color = Color::Rgb(249, 115, 22); // Orange
const COLOR_FAMILY_GENERATION: Color = Color::Rgb(181, 137, 0); // Gold
const COLOR_FAMILY_LOCALIZATION: Color = Color::Rgb(34, 197, 94); // Green
const COLOR_FAMILY_MINING: Color = Color::Rgb(139, 92, 246); // Violet

// =============================================================================
// YAML SYNTAX HIGHLIGHTING STYLES
// =============================================================================

/// YAML comment style.
const STYLE_YAML_COMMENT: Style = Style::new().fg(Color::DarkGray);

/// YAML key style.
const STYLE_YAML_KEY: Style = Style::new().fg(Color::Yellow);

/// YAML colon/dash style.
const STYLE_YAML_PUNCT: Style = Style::new().fg(Color::Cyan);

/// YAML string value style.
const STYLE_YAML_STRING: Style = Style::new().fg(Color::Green);

/// YAML boolean/null style.
const STYLE_YAML_LITERAL: Style = Style::new().fg(Color::Magenta);

/// YAML number style.
const STYLE_YAML_NUMBER: Style = Style::new().fg(Color::Cyan);

/// YAML plain text style.
const STYLE_YAML_TEXT: Style = Style::new().fg(Color::White);

// =============================================================================
// PUBLIC API
// =============================================================================

/// Render the Content panel [2] (SOURCE box only).
/// v0.17.3: Renamed from render_yaml_panel to reflect context-aware content.
///
/// Visual states:
/// - Selected (cyan): This panel is focused (Focus::Content)
/// - Unfocused (dim): This panel is NOT focused
pub fn render_content_panel(f: &mut Frame, area: Rect, app: &App) {
    // v0.17.3: Use Focus::Content for panel focus
    use crate::tui::app::Focus;
    let source_selected = app.focus == Focus::Content;
    render_source_box(f, area, app, source_selected);
}

/// Render the SOURCE box with context-aware content.
/// v0.17.3: Phase 3 - uses ContentPanelMode to determine what to show.
fn render_source_box(f: &mut Frame, area: Rect, app: &App, selected: bool) {
    // Determine border color: selected = cyan, otherwise = dim
    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Get the content mode based on current tree selection
    let mode = app.content_panel_mode();

    match mode {
        ContentPanelMode::Schema { path, name } => {
            render_schema_content(f, area, app, selected, border_color, &path, &name);
        }
        ContentPanelMode::InstanceInfo {
            instance_key,
            class_name,
            realm,
            layer,
        } => {
            render_instance_info(
                f,
                area,
                selected,
                border_color,
                &instance_key,
                &class_name,
                &realm,
                &layer,
            );
        }
        ContentPanelMode::SectionInfo { name, description } => {
            render_section_info(f, area, selected, border_color, &name, &description);
        }
        ContentPanelMode::Empty => {
            render_empty_content(f, area, selected, border_color);
        }
    }
}

/// Abbreviate a YAML path to show only the last 3 segments.
/// Example: "packages/core/models/node-classes/org/semantic/entity-native.yaml"
///       -> "org/semantic/entity-native.yaml"
fn abbreviate_yaml_path(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').collect();
    if segments.len() <= 3 {
        path.to_string()
    } else {
        segments[segments.len() - 3..].join("/")
    }
}

// =============================================================================
// CONTENT PANEL RENDER FUNCTIONS (v0.17.3 Phase 3)
// =============================================================================

/// Render SCHEMA content - shows YAML with syntax highlighting.
/// v0.17.3: Used when a Class or ArcClass is selected in the tree.
fn render_schema_content(
    f: &mut Frame,
    area: Rect,
    app: &App,
    selected: bool,
    border_color: Color,
    yaml_path: &str,
    class_name: &str,
) {
    let visible_height = area.height.saturating_sub(2) as usize;
    let line_count = app.yaml.content.lines().count();

    // Build title: ` ▶ SCHEMA ⊞N │ path/file.yaml `
    let title = build_schema_title(selected, line_count, yaml_path, class_name);

    render_yaml_content_in_box(f, area, app, visible_height, border_color, title);
}

/// Build the SCHEMA panel title.
/// Format: ` ▶ SCHEMA ⊞N │ path/file.yaml ` (when selected)
fn build_schema_title(
    selected: bool,
    line_count: usize,
    yaml_path: &str,
    _class_name: &str,
) -> Line<'static> {
    let mut spans = Vec::new();

    if selected {
        spans.push(Span::styled(
            " ▶ ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            "SCHEMA ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " SCHEMA ",
            Style::default().fg(COLOR_MUTED_TEXT),
        ));
    }

    // Line count badge
    let badge_style = if selected {
        Style::default().fg(Color::Rgb(136, 192, 208)) // Nord Frost
    } else {
        Style::default().fg(Color::DarkGray)
    };
    spans.push(Span::styled(format!("⊞{}", line_count), badge_style));

    // YAML path (abbreviated)
    if !yaml_path.is_empty() {
        let short_path = abbreviate_yaml_path(yaml_path);
        let path_style = if selected {
            Style::default().fg(Color::Rgb(100, 120, 140))
        } else {
            Style::default().fg(Color::Rgb(70, 70, 70))
        };
        spans.push(Span::styled(
            " │ ",
            Style::default().fg(Color::Rgb(60, 60, 60)),
        ));
        spans.push(Span::styled(short_path, path_style));
    }

    spans.push(Span::styled(" ", Style::default()));

    Line::from(spans)
}

/// Render INFO content for instances - points to PROPERTIES panel.
/// v0.17.3: Used when an Instance node is selected in the tree.
#[allow(clippy::too_many_arguments)]
fn render_instance_info(
    f: &mut Frame,
    area: Rect,
    selected: bool,
    border_color: Color,
    instance_key: &str,
    class_name: &str,
    realm: &str,
    layer: &str,
) {
    let mut lines: Vec<Line> = Vec::new();

    // Add some vertical spacing
    lines.push(Line::from(""));

    // Instance header
    lines.push(Line::from(vec![
        Span::styled("   Instance: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            instance_key.to_string(),
            Style::default()
                .fg(Color::Rgb(136, 192, 208)) // Nord Frost
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    // Class info
    lines.push(Line::from(vec![
        Span::styled("   Class: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} ({}/{})", class_name, realm, layer),
            Style::default().fg(Color::Rgb(163, 190, 140)), // Nord Aurora Green
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "   ─────────────────────────────────────────────",
        Style::default().fg(Color::Rgb(70, 70, 70)),
    )));
    lines.push(Line::from(""));

    // Info message
    lines.push(Line::from(Span::styled(
        "   Instances are stored in Neo4j, not YAML files.",
        Style::default().fg(Color::DarkGray),
    )));

    lines.push(Line::from(vec![
        Span::styled("   → See ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "PROPERTIES",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " panel for instance data",
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    lines.push(Line::from(vec![
        Span::styled("   → Press ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "Tab",
            Style::default()
                .fg(Color::Rgb(181, 137, 0)) // Gold
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " to navigate to PROPERTIES",
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    lines.push(Line::from(""));

    // Hint for viewing parent class schema
    lines.push(Line::from(vec![
        Span::styled("   [", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "s",
            Style::default()
                .fg(Color::Rgb(181, 137, 0))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "] View parent class schema",
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    // Build title
    let title = build_info_title(selected, instance_key);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render INFO content for sections (Realm, Layer, ArcFamily).
/// v0.17.3: Used when a navigation section is selected.
fn render_section_info(
    f: &mut Frame,
    area: Rect,
    selected: bool,
    border_color: Color,
    name: &str,
    description: &str,
) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("   ", Style::default()),
        Span::styled(
            name.to_string(),
            Style::default()
                .fg(Color::Rgb(136, 192, 208)) // Nord Frost
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("   {}", description),
        Style::default().fg(Color::DarkGray),
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "   Select a Class to view its YAML schema.",
        Style::default().fg(Color::Rgb(100, 100, 100)),
    )));

    // Build title
    let title = build_info_title(selected, name);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render empty content state.
/// v0.17.3: Used when nothing is selected.
fn render_empty_content(f: &mut Frame, area: Rect, selected: bool, border_color: Color) {
    let lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "   No selection",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "   Navigate to a node to view its content.",
            Style::default().fg(Color::Rgb(100, 100, 100)),
        )),
    ];

    // Build title
    let title = build_info_title(selected, "INFO");

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Build the INFO panel title.
/// Format: ` ▶ INFO │ name ` (when selected)
fn build_info_title(selected: bool, name: &str) -> Line<'static> {
    let mut spans = Vec::new();

    if selected {
        spans.push(Span::styled(
            " ▶ ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            "INFO ",
            Style::default()
                .fg(BOX_BORDER_SELECTED)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        spans.push(Span::styled(
            " INFO ",
            Style::default().fg(COLOR_MUTED_TEXT),
        ));
    }

    // Name badge
    let name_style = if selected {
        Style::default().fg(Color::Rgb(136, 192, 208)) // Nord Frost
    } else {
        Style::default().fg(Color::DarkGray)
    };

    spans.push(Span::styled(
        "│ ",
        Style::default().fg(Color::Rgb(60, 60, 60)),
    ));
    spans.push(Span::styled(name.to_string(), name_style));
    spans.push(Span::styled(" ", Style::default()));

    Line::from(spans)
}

// v0.13.1: render_diagram_box, get_diagram_type, generate_diagram_content removed (panel simplification)

/// Get realm color from key.
fn realm_color(key: &str) -> Color {
    match key {
        "shared" => COLOR_REALM_SHARED,
        "org" => COLOR_REALM_ORG,
        _ => Color::White,
    }
}

/// Get trait color from name (ADR-024 Data Origin).
fn trait_color(trait_name: &str) -> Color {
    match trait_name {
        "defined" => COLOR_TRAIT_DEFINED,
        "authored" => COLOR_TRAIT_AUTHORED,
        "imported" => COLOR_TRAIT_IMPORTED,
        "generated" => COLOR_TRAIT_GENERATED,
        "retrieved" => COLOR_TRAIT_RETRIEVED,
        _ => Color::White,
    }
}

/// Get trait icon from name.
#[allow(dead_code)] // v0.13.1: May be used for PROPERTIES panel styling
fn trait_icon(trait_name: &str) -> &'static str {
    match trait_name {
        "defined" => "■",
        "authored" => "□",
        "imported" => "◊",
        "generated" => "✦",
        "retrieved" => "⋆",
        _ => "○",
    }
}

/// Get arc family color from key.
fn arc_family_color(family: &str) -> Color {
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
fn layer_color(layer: &str) -> Color {
    match layer {
        "config" => Color::Rgb(59, 130, 246),   // Blue
        "locale" => Color::Rgb(236, 72, 153),   // Pink
        "geography" => Color::Rgb(34, 197, 94), // Green
        "knowledge" => COLOR_LAYER_KNOWLEDGE,
        "foundation" => Color::Rgb(168, 85, 247), // Purple
        "structure" => Color::Rgb(59, 130, 246),  // Blue
        "semantic" => COLOR_LAYER_SEMANTIC,
        "instruction" => Color::Rgb(181, 137, 0), // Gold
        "output" => COLOR_LAYER_OUTPUT,
        _ => Color::White,
    }
}

/// Get arc scope color.
fn scope_color(scope: &str) -> Color {
    match scope {
        "intra_realm" => Color::Rgb(42, 161, 152), // Cyan
        "cross_realm" => Color::Rgb(249, 115, 22), // Orange
        _ => Color::White,
    }
}

/// Get cardinality color.
fn cardinality_color(cardinality: &str) -> Color {
    match cardinality {
        "one_to_one" | "1:1" => Color::Rgb(34, 197, 94), // Green
        "one_to_many" | "1:N" => Color::Rgb(59, 130, 246), // Blue
        "many_to_one" | "N:1" => Color::Rgb(168, 85, 247), // Purple
        "many_to_many" | "N:M" => Color::Rgb(249, 115, 22), // Orange
        _ => Color::White,
    }
}

/// Check if a YAML key should have semantic coloring for its value.
/// Returns Some(color_fn) if the key is semantic, None otherwise.
fn semantic_value_color(key: &str, value: &str) -> Option<Color> {
    let key_trimmed = key.trim().trim_end_matches(':');
    let value_trimmed = value.trim();

    match key_trimmed {
        "realm" => Some(realm_color(value_trimmed)),
        "layer" => Some(layer_color(value_trimmed)),
        "trait" => Some(trait_color(value_trimmed)),
        "family" => Some(arc_family_color(value_trimmed)),
        "scope" => Some(scope_color(value_trimmed)),
        "cardinality" => Some(cardinality_color(value_trimmed)),
        _ => None,
    }
}

// =============================================================================
// INTERNAL FUNCTIONS
// =============================================================================

/// Generate arc badge lines for ArcClass items.
/// v0.13 Option C: Shows source→target relationship with colored badges.
/// Format: ┌ [Source] ──[:ARC_NAME]──► [Target] ┐
fn generate_arc_badge(app: &App) -> Vec<Line<'static>> {
    use crate::tui::data::TreeItem;

    let mut badge_lines = Vec::new();

    if let Some(TreeItem::ArcClass(family, arc)) = app.current_item() {
        let fc = arc_family_color(&family.key);

        // Get source/target class colors (use layer colors if we can resolve them)
        let source_color = Color::Rgb(136, 192, 208); // Nord Frost (default)
        let target_color = Color::Rgb(163, 190, 140); // Nord Aurora Green (default)

        // Line 1: Source ──[:ARC]──► Target
        badge_lines.push(Line::from(vec![
            Span::styled("┌ ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("[{}]", arc.from_class),
                Style::default()
                    .fg(source_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ──[:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                arc.key.clone(),
                Style::default().fg(fc).add_modifier(Modifier::BOLD),
            ),
            Span::styled("]──► ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("[{}]", arc.to_class),
                Style::default()
                    .fg(target_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ┐", Style::default().fg(Color::DarkGray)),
        ]));

        // Line 2: Family + Cardinality badges
        let card_color = cardinality_color(&arc.cardinality);
        badge_lines.push(Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("◇{}", family.key), Style::default().fg(fc)),
            Span::styled(" ", Style::default()),
            Span::styled(
                format!("⊞{}", arc.cardinality),
                Style::default().fg(card_color),
            ),
            Span::styled(" │", Style::default().fg(Color::DarkGray)),
        ]));

        // Line 3: Separator
        badge_lines.push(Line::from(Span::styled(
            "└────────────────────────────────────────┘",
            Style::default().fg(Color::DarkGray),
        )));
    }

    badge_lines
}

/// Render YAML content in a box with given border color and title.
/// v0.13.1: Simplified - shows full YAML with scroll, no collapse/peek.
/// PROPERTIES panel already shows instance properties, so no need for contextual sections.
fn render_yaml_content_in_box(
    f: &mut Frame,
    area: Rect,
    app: &App,
    visible_height: usize,
    border_color: Color,
    title: Line<'static>,
) {
    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    // v0.13 Option C: Add arc badge for ArcClass items
    let arc_badge = generate_arc_badge(app);
    let badge_height = arc_badge.len();
    lines.extend(arc_badge);

    // Adjust visible height for badge
    let content_visible_height = visible_height.saturating_sub(badge_height);

    // v0.17.3: Always show YAML content (Instance tab removed, data is in PROPERTIES panel)
    if !app.yaml.content.is_empty() {
        for yaml_line in app
            .yaml
            .content
            .lines()
            .skip(app.yaml.scroll)
            .take(content_visible_height)
        {
            lines.push(highlight_yaml_line(yaml_line));
        }
    } else {
        lines.push(Line::from(Span::styled("No YAML file", STYLE_DIM)));
    }

    // Total lines for scroll indicator
    let total_lines = app.yaml.content.lines().count();

    // Build scroll indicator with directional arrows
    let scroll_hint = scroll_indicator(app.yaml.scroll, total_lines, visible_height);

    let block = Block::default()
        .title(title)
        .title_bottom(Span::styled(scroll_hint, STYLE_DIM))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    // v0.16.4: Use Unicode symbols for visual consistency with other panels
    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(total_lines.saturating_sub(visible_height))
            .position(app.yaml.scroll);

        // Render scrollbar in the inner area (inside border)
        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(2),
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Highlight a YAML line with syntax coloring.
/// v0.13: Enhanced with semantic coloring for realm, layer, trait, family, scope, cardinality.
fn highlight_yaml_line(line: &str) -> Line<'static> {
    // Comment line
    if line.trim_start().starts_with('#') {
        return Line::from(Span::styled(line.to_string(), STYLE_YAML_COMMENT));
    }

    // Empty line
    if line.trim().is_empty() {
        return Line::from(Span::raw(line.to_string()));
    }

    // Key-value or list item (most lines have 2-4 spans)
    let mut spans: Vec<Span<'static>> = Vec::with_capacity(4);

    // Find leading whitespace
    let indent_len = line.len() - line.trim_start().len();
    let indent = &line[..indent_len];
    let rest = &line[indent_len..];

    spans.push(Span::raw(indent.to_string()));

    // Check for list item
    if rest.starts_with("- ") {
        spans.push(Span::styled("-", STYLE_YAML_PUNCT));
        let after_dash = &rest[1..];

        // Check if it's a key-value after dash
        if let Some(colon_pos) = after_dash.find(':') {
            let key = &after_dash[..colon_pos + 1];
            let value = &after_dash[colon_pos + 1..];
            spans.push(Span::styled(key.to_string(), STYLE_YAML_KEY));
            // v0.13: Semantic coloring for values
            spans.push(highlight_yaml_value_semantic(key, value));
        } else {
            spans.push(highlight_yaml_value(after_dash));
        }
    } else if let Some(colon_pos) = rest.find(':') {
        // Key-value pair
        let key = &rest[..colon_pos];
        let colon_and_rest = &rest[colon_pos..];

        spans.push(Span::styled(key.to_string(), STYLE_YAML_KEY));

        if colon_and_rest.len() > 1 {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
            let value = &colon_and_rest[1..];
            // v0.13: Semantic coloring for values
            spans.push(highlight_yaml_value_semantic(key, value));
        } else {
            spans.push(Span::styled(":", STYLE_YAML_TEXT));
        }
    } else {
        // Plain text
        spans.push(Span::styled(rest.to_string(), STYLE_YAML_TEXT));
    }

    Line::from(spans)
}

/// Highlight a YAML value with semantic coloring if applicable.
/// v0.13: Checks if the key is a semantic key (realm, layer, trait, family, scope, cardinality)
/// and applies the appropriate color from the taxonomy.
fn highlight_yaml_value_semantic(key: &str, value: &str) -> Span<'static> {
    // Check for semantic coloring first
    if let Some(color) = semantic_value_color(key, value) {
        return Span::styled(
            value.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );
    }

    // Fall back to default value highlighting
    highlight_yaml_value(value)
}

/// Highlight a YAML value with appropriate color.
/// Uses const STYLE_YAML_* for efficiency.
fn highlight_yaml_value(value: &str) -> Span<'static> {
    let trimmed = value.trim();

    // Boolean
    if trimmed == "true" || trimmed == "false" {
        return Span::styled(value.to_string(), STYLE_YAML_LITERAL);
    }

    // Null
    if trimmed == "null" || trimmed == "~" {
        return Span::styled(value.to_string(), STYLE_YAML_LITERAL);
    }

    // Number
    if trimmed.parse::<f64>().is_ok() {
        return Span::styled(value.to_string(), STYLE_YAML_NUMBER);
    }

    // String (quoted or unquoted)
    Span::styled(value.to_string(), STYLE_YAML_STRING)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // highlight_yaml_value tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_value_boolean_true() {
        let span = highlight_yaml_value(" true");
        assert_eq!(span.content, " true");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_boolean_false() {
        let span = highlight_yaml_value(" false");
        assert_eq!(span.content, " false");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_null() {
        let span = highlight_yaml_value(" null");
        assert_eq!(span.content, " null");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_tilde_null() {
        let span = highlight_yaml_value(" ~");
        assert_eq!(span.content, " ~");
        assert_eq!(span.style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_value_integer() {
        let span = highlight_yaml_value(" 42");
        assert_eq!(span.content, " 42");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_negative_integer() {
        let span = highlight_yaml_value(" -17");
        assert_eq!(span.content, " -17");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_float() {
        let span = highlight_yaml_value(" 3.14");
        assert_eq!(span.content, " 3.14");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_string() {
        let span = highlight_yaml_value(" hello world");
        assert_eq!(span.content, " hello world");
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_value_quoted_string() {
        let span = highlight_yaml_value(" \"quoted\"");
        assert_eq!(span.content, " \"quoted\"");
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_value_empty() {
        let span = highlight_yaml_value("");
        assert_eq!(span.content, "");
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    // =========================================================================
    // highlight_yaml_line tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_line_comment() {
        let line = highlight_yaml_line("# This is a comment");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "# This is a comment");
        assert_eq!(line.spans[0].style, STYLE_YAML_COMMENT);
    }

    #[test]
    fn test_highlight_yaml_line_comment_with_indent() {
        let line = highlight_yaml_line("  # Indented comment");
        // Indented comments are still treated as full comment lines
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "  # Indented comment");
        assert_eq!(line.spans[0].style, STYLE_YAML_COMMENT);
    }

    #[test]
    fn test_highlight_yaml_line_empty() {
        let line = highlight_yaml_line("");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "");
    }

    #[test]
    fn test_highlight_yaml_line_whitespace_only() {
        let line = highlight_yaml_line("   ");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "   ");
    }

    #[test]
    fn test_highlight_yaml_line_key_value() {
        let line = highlight_yaml_line("name: Page");
        assert_eq!(line.spans.len(), 4);
        // spans[0] = "" (empty indent)
        // spans[1] = "name" (key)
        // spans[2] = ":" (colon)
        // spans[3] = " Page" (value)
        assert_eq!(line.spans[1].content, "name");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[2].content, ":");
        assert_eq!(line.spans[3].content, " Page");
        assert_eq!(line.spans[3].style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_line_key_value_indented() {
        let line = highlight_yaml_line("  realm: shared");
        assert_eq!(line.spans.len(), 4);
        assert_eq!(line.spans[0].content, "  "); // indent
        assert_eq!(line.spans[1].content, "realm"); // key
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[2].content, ":"); // colon
        assert_eq!(line.spans[3].content, " shared"); // value
    }

    #[test]
    fn test_highlight_yaml_line_key_with_boolean_value() {
        let line = highlight_yaml_line("enabled: true");
        assert_eq!(line.spans[3].content, " true");
        assert_eq!(line.spans[3].style, STYLE_YAML_LITERAL);
    }

    #[test]
    fn test_highlight_yaml_line_key_with_number_value() {
        let line = highlight_yaml_line("count: 42");
        assert_eq!(line.spans[3].content, " 42");
        assert_eq!(line.spans[3].style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_line_key_no_value() {
        let line = highlight_yaml_line("properties:");
        assert_eq!(line.spans.len(), 3);
        assert_eq!(line.spans[1].content, "properties");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[2].content, ":");
    }

    #[test]
    fn test_highlight_yaml_line_list_item() {
        let line = highlight_yaml_line("- item");
        assert_eq!(line.spans.len(), 3);
        assert_eq!(line.spans[0].content, ""); // empty indent
        assert_eq!(line.spans[1].content, "-");
        assert_eq!(line.spans[1].style, STYLE_YAML_PUNCT);
        assert_eq!(line.spans[2].content, " item");
        assert_eq!(line.spans[2].style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_line_list_item_indented() {
        let line = highlight_yaml_line("  - indented item");
        assert_eq!(line.spans[0].content, "  "); // indent
        assert_eq!(line.spans[1].content, "-");
        assert_eq!(line.spans[1].style, STYLE_YAML_PUNCT);
    }

    #[test]
    fn test_highlight_yaml_line_list_item_with_key_value() {
        let line = highlight_yaml_line("- name: value");
        assert_eq!(line.spans.len(), 4);
        assert_eq!(line.spans[1].content, "-");
        assert_eq!(line.spans[1].style, STYLE_YAML_PUNCT);
        assert_eq!(line.spans[2].content, " name:");
        assert_eq!(line.spans[2].style, STYLE_YAML_KEY);
        assert_eq!(line.spans[3].content, " value");
        assert_eq!(line.spans[3].style, STYLE_YAML_STRING);
    }

    #[test]
    fn test_highlight_yaml_line_plain_text() {
        let line = highlight_yaml_line("just plain text without colon");
        assert_eq!(line.spans.len(), 2);
        assert_eq!(line.spans[0].content, ""); // empty indent
        assert_eq!(line.spans[1].content, "just plain text without colon");
        assert_eq!(line.spans[1].style, STYLE_YAML_TEXT);
    }

    // =========================================================================
    // Style constant tests
    // =========================================================================

    #[test]
    fn test_style_yaml_comment_is_dark_gray() {
        assert_eq!(STYLE_YAML_COMMENT.fg, Some(Color::DarkGray));
    }

    #[test]
    fn test_style_yaml_key_is_yellow() {
        assert_eq!(STYLE_YAML_KEY.fg, Some(Color::Yellow));
    }

    #[test]
    fn test_style_yaml_punct_is_cyan() {
        assert_eq!(STYLE_YAML_PUNCT.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_style_yaml_string_is_green() {
        assert_eq!(STYLE_YAML_STRING.fg, Some(Color::Green));
    }

    #[test]
    fn test_style_yaml_literal_is_magenta() {
        assert_eq!(STYLE_YAML_LITERAL.fg, Some(Color::Magenta));
    }

    #[test]
    fn test_style_yaml_number_is_cyan() {
        assert_eq!(STYLE_YAML_NUMBER.fg, Some(Color::Cyan));
    }

    #[test]
    fn test_style_yaml_text_is_white() {
        assert_eq!(STYLE_YAML_TEXT.fg, Some(Color::White));
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_line_colon_in_value() {
        // Value containing a colon (URL, time, etc.)
        let line = highlight_yaml_line("url: https://example.com");
        assert_eq!(line.spans[1].content, "url");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
        // The value should include the URL with colons
        assert_eq!(line.spans[3].content, " https://example.com");
    }

    #[test]
    fn test_highlight_yaml_line_multiword_key() {
        let line = highlight_yaml_line("display_name: My Page");
        assert_eq!(line.spans[1].content, "display_name");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
    }

    #[test]
    fn test_highlight_yaml_line_deeply_indented() {
        let line = highlight_yaml_line("        nested: value");
        assert_eq!(line.spans[0].content, "        "); // 8 spaces
        assert_eq!(line.spans[1].content, "nested");
        assert_eq!(line.spans[1].style, STYLE_YAML_KEY);
    }

    #[test]
    fn test_highlight_yaml_value_scientific_notation() {
        let span = highlight_yaml_value(" 1.5e10");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_zero() {
        let span = highlight_yaml_value(" 0");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    #[test]
    fn test_highlight_yaml_value_negative_float() {
        let span = highlight_yaml_value(" -0.5");
        assert_eq!(span.style, STYLE_YAML_NUMBER);
    }

    // =========================================================================
    // v0.13 semantic_value_color tests (Option B)
    // =========================================================================

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
    fn test_semantic_value_color_trait_defined() {
        let color = semantic_value_color("trait", " defined");
        assert_eq!(color, Some(COLOR_TRAIT_DEFINED));
    }

    #[test]
    fn test_semantic_value_color_trait_authored() {
        let color = semantic_value_color("trait", " authored");
        assert_eq!(color, Some(COLOR_TRAIT_AUTHORED));
    }

    #[test]
    fn test_semantic_value_color_trait_imported() {
        let color = semantic_value_color("trait", " imported");
        assert_eq!(color, Some(COLOR_TRAIT_IMPORTED));
    }

    #[test]
    fn test_semantic_value_color_trait_generated() {
        let color = semantic_value_color("trait", " generated");
        assert_eq!(color, Some(COLOR_TRAIT_GENERATED));
    }

    #[test]
    fn test_semantic_value_color_trait_retrieved() {
        let color = semantic_value_color("trait", " retrieved");
        assert_eq!(color, Some(COLOR_TRAIT_RETRIEVED));
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
        assert!(color.is_some()); // Should have a color
    }

    #[test]
    fn test_semantic_value_color_scope_cross() {
        let color = semantic_value_color("scope", " cross_realm");
        assert!(color.is_some()); // Should have a color
    }

    #[test]
    fn test_semantic_value_color_cardinality() {
        let color = semantic_value_color("cardinality", " one_to_many");
        assert!(color.is_some()); // Should have a color
    }

    #[test]
    fn test_semantic_value_color_non_semantic_key() {
        let color = semantic_value_color("name", " Page");
        assert_eq!(color, None); // Not a semantic key
    }

    #[test]
    fn test_semantic_value_color_with_colon() {
        // Key might have trailing colon from parsing
        let color = semantic_value_color("realm:", " shared");
        assert_eq!(color, Some(COLOR_REALM_SHARED));
    }

    // =========================================================================
    // v0.13 highlight_yaml_value_semantic tests
    // =========================================================================

    #[test]
    fn test_highlight_yaml_value_semantic_realm() {
        let span = highlight_yaml_value_semantic("realm", " shared");
        assert_eq!(span.content, " shared");
        // Should be bold with COLOR_REALM_SHARED
        assert!(span.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_highlight_yaml_value_semantic_fallback() {
        // Non-semantic key should fall back to default highlighting
        let span = highlight_yaml_value_semantic("name", " Page");
        assert_eq!(span.content, " Page");
        // Should be string style (green), not bold
        assert_eq!(span.style, STYLE_YAML_STRING);
    }

    // =========================================================================
    // v0.13 layer_color tests
    // =========================================================================

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

    // =========================================================================
    // v0.13 cardinality_color tests
    // =========================================================================

    #[test]
    fn test_cardinality_color_one_to_one() {
        let color = cardinality_color("one_to_one");
        assert_eq!(color, Color::Rgb(34, 197, 94)); // Green
    }

    #[test]
    fn test_cardinality_color_one_to_many() {
        let color = cardinality_color("one_to_many");
        assert_eq!(color, Color::Rgb(59, 130, 246)); // Blue
    }

    #[test]
    fn test_cardinality_color_many_to_many() {
        let color = cardinality_color("many_to_many");
        assert_eq!(color, Color::Rgb(249, 115, 22)); // Orange
    }
}
