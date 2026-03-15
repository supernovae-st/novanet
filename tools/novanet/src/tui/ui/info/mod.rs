//! Info panel rendering for TUI.
//!
//! v0.20.0: Split into submodules:
//! - `mod.rs`: Types, constants, helpers, render functions
//! - `builders.rs`: Routing dispatcher for tree item → builder
//! - `build_schema.rs`: ClassesSection, ArcsSection, Realm, Layer
//! - `build_class.rs`: Class (NodeClass) content builder
//! - `build_arcs.rs`: ArcFamily, ArcClass content builders
//! - `build_instance.rs`: Instance content builder
//! - `build_groups.rs`: EntityCategory, EntityGroup, empty state
//! - `build_provenance.rs`: Provenance section (ADR-035/042)
//! - `json_formatting.rs`: JSON value formatting + colors
//! - `property_styling.rs`: Property type badges, standard props, render helpers

mod build_arcs;
mod build_class;
mod build_groups;
mod build_instance;
mod build_provenance;
mod build_schema;
mod builders;
mod json_formatting;
mod property_styling;

pub use builders::build_unified_content;

// Re-exports from sub-modules (used by builders and other info sub-modules via `super::`,
// and by identity_panel via `super::super::info::`)
pub(crate) use build_provenance::{build_provenance_section, DataCategory, ProvenanceMeta};
pub(crate) use json_formatting::{
    format_json_value, json_value_color, json_value_to_display, wrap_json_value,
};
pub(crate) use property_styling::{
    is_standard_property, render_property_line, type_badge, type_color, PropType,
};

// Re-export truncate_str from ui/text_utils (used by build_instance.rs via `super::`)
pub(super) use super::truncate_str;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::tui::app::{App, Focus};
use crate::tui::palette;

use super::{BOX_BORDER_FOCUSED, BOX_BORDER_SELECTED, BOX_BORDER_UNFOCUSED, STYLE_DIM, STYLE_MUTED};

// =============================================================================
// YAML-STYLE COLORS FOR PROPERTIES
// =============================================================================

/// YAML key style (cyan) - matches SOURCE panel styling.
pub(super) const STYLE_PROP_KEY: Style = Style::new().fg(palette::PROP_KEY);

/// YAML colon style.
pub(super) const STYLE_PROP_COLON: Style = Style::new().fg(Color::Cyan);

/// JSON value colors - match yaml_panel.rs json_value_color().
pub(super) const COLOR_VALUE_NULL: Color = Color::DarkGray;
pub(super) const COLOR_VALUE_BOOL: Color = palette::VALUE_BOOL;
pub(super) const COLOR_VALUE_NUMBER: Color = palette::VALUE_NUMBER;
pub(super) const COLOR_VALUE_STRING: Color = palette::VALUE_STRING;
pub(super) const COLOR_VALUE_ARRAY: Color = palette::VALUE_ARRAY;
pub(super) const COLOR_VALUE_OBJECT: Color = palette::VALUE_OBJECT;

// =============================================================================
// SECTION HEADER COLORS
// =============================================================================

/// STANDARD section header - teal (same as shared realm color #2aa198).
/// Standard properties are common/boring - stable teal conveys "foundational".
pub(super) const COLOR_HEADER_STANDARD: Color = palette::SOLARIZED_CYAN;

/// SPECIFIC section header - orange (same as semantic layer color #f97316).
/// Specific properties are unique/interesting - vibrant orange conveys "differentiation".
pub(super) const COLOR_HEADER_SPECIFIC: Color = palette::ORANGE_500;

/// PROVENANCE section header - violet (ADR-042 provenance tracking).
/// Provenance shows data origin and lifecycle - violet conveys "authority/trust".
pub(super) const COLOR_HEADER_PROVENANCE: Color = palette::VIOLET_500;

/// Focused property background - subtle highlight for j/k navigation.
/// Dark blue background that works well with all text colors.
pub(super) const COLOR_PROPERTY_FOCUSED_BG: Color = palette::BG_PROPERTY_FOCUSED;

// =============================================================================
// PROPERTY INDICATOR COLORS (Solarized palette)
// =============================================================================

/// Green checkmark (✓) for properties that have values - Solarized Green #859900
pub(super) const COLOR_STATUS_OK: Color = palette::SOLARIZED_GREEN;

/// Red asterisk (*) for required properties - Solarized Red #dc322f
pub(super) const COLOR_REQUIRED_MARKER: Color = palette::SOLARIZED_RED;

/// Blue type badge [str] - Solarized Blue #268bd2
pub(super) const COLOR_TYPE_STRING: Color = palette::SOLARIZED_BLUE;

// =============================================================================
// UNIFIED SECTION TYPES
// =============================================================================

/// Content for a single info section.
/// Each section has a title and content lines.
/// Empty sections display "—" as content.
/// Made public for render optimization.
#[derive(Default)]
pub struct SectionContent<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> SectionContent<'a> {
    pub(super) fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub(super) fn add_line(&mut self, line: Line<'a>) {
        self.lines.push(line);
    }

    pub(super) fn add_kv(&mut self, key: &str, value: Span<'a>) {
        self.lines.push(Line::from(vec![
            Span::styled(format!("{:<10} ", key), STYLE_DIM),
            value,
        ]));
    }

    /// Add a classification entry with explicit key:value format.
    /// Format: `key: icon value` (e.g., `realm: ◎ org`)
    /// Uses narrower 8-char width for compact CLASSIFICATION section.
    pub(super) fn add_classification(&mut self, key: &str, icon: &str, value: &str, color: Color) {
        self.lines.push(Line::from(vec![
            Span::styled(format!("{:<8}", format!("{}:", key)), STYLE_DIM),
            Span::styled(format!("{} ", icon), Style::default().fg(color)),
            Span::styled(value.to_string(), Style::default().fg(color)),
        ]));
    }

    pub(super) fn add_empty(&mut self) {
        self.lines.push(Line::from(Span::styled("—", STYLE_DIM)));
    }
}

/// Unified info content with 7 fixed sections.
/// All sections are always present; empty sections show "—".
/// Made public for render optimization.
/// Added PROVENANCE section (ADR-042).
#[derive(Default)]
pub struct UnifiedContent<'a> {
    /// IDENTITY: type, category, key, class
    pub identity: SectionContent<'a>,
    /// LOCATION: realm, layer
    pub location: SectionContent<'a>,
    /// METRICS: counts, totals, budgets
    pub metrics: SectionContent<'a>,
    /// COVERAGE: property fill rates, health bars
    pub coverage: SectionContent<'a>,
    /// PROVENANCE: data origin, category, lifecycle (ADR-042)
    pub provenance: SectionContent<'a>,
    /// PROPERTIES: property list with values/schema
    pub properties: SectionContent<'a>,
    /// RELATIONSHIPS: arcs, pipeline context
    pub relationships: SectionContent<'a>,
}

// =============================================================================
// UNIFIED INFO PANEL RENDERING
// =============================================================================

/// Visual state for a box in the info panel.
#[derive(Clone, Copy, PartialEq, Eq)]
enum BoxVisualState {
    /// Panel not active
    Unfocused,
    /// Panel active, but this box is not selected (will be used for multi-box focus)
    #[allow(dead_code)]
    Focused,
    /// This box is selected (active for copy/scroll)
    Selected,
}

/// Get border color and title style based on visual state.
fn box_styles(state: BoxVisualState) -> (Color, Style) {
    match state {
        BoxVisualState::Unfocused => (BOX_BORDER_UNFOCUSED, STYLE_DIM),
        BoxVisualState::Focused => (BOX_BORDER_FOCUSED, STYLE_MUTED),
        BoxVisualState::Selected => (
            BOX_BORDER_SELECTED,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    }
}

/// Render a scrollable section box with scroll indicator.
fn render_scrollable_section_box(
    f: &mut Frame,
    area: Rect,
    title: &str,
    content: &SectionContent,
    state: BoxVisualState,
    scroll_offset: usize,
) -> usize {
    let lines: Vec<Line> = if content.is_empty() {
        vec![Line::from(Span::styled("—", STYLE_DIM))]
    } else {
        content.lines.clone()
    };

    let total_lines = lines.len();
    // Inner height = area height - 2 (for borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);
    let clamped_scroll = scroll_offset.min(max_scroll);

    let (border_color, base_title_style) = box_styles(state);

    // Show scroll indicator in title if scrollable
    let scroll_info = if total_lines > visible_height {
        format!(" [{}/{}]", clamped_scroll + 1, total_lines)
    } else {
        String::new()
    };

    // Selected box gets a ▶ indicator
    let title_text = if state == BoxVisualState::Selected {
        format!(" ▶ {}{} ", title, scroll_info)
    } else {
        format!(" {}{} ", title, scroll_info)
    };

    let block = Block::default()
        .title(Span::styled(title_text, base_title_style))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((clamped_scroll as u16, 0));

    f.render_widget(paragraph, area);

    // Render scrollbar if content exceeds visible area
    if total_lines > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(max_scroll).position(clamped_scroll);

        // Scrollbar area is inside the block, right edge
        let scrollbar_area = Rect {
            x: area.x + area.width - 2,
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    // Return total lines for scroll calculation in app
    total_lines
}

/// Compute visual state for a box in the Detail panel.
/// Focus enum is the source of truth for panel selection.
fn detail_box_state(panel_focused: bool) -> BoxVisualState {
    if panel_focused {
        BoxVisualState::Selected
    } else {
        BoxVisualState::Unfocused
    }
}

/// Render the properties panel [3] in the right column.
/// Render the separated Properties panel.
/// Accepts pre-built content to avoid double-building.
pub fn render_props_panel(f: &mut Frame, area: Rect, app: &mut App, content: &UnifiedContent) {
    // Props panel focused when Focus::Props
    let panel_focused = app.focus == Focus::Props;

    // Render the PROPERTIES section as a scrollable panel
    let props_state = detail_box_state(panel_focused);
    let total_lines = render_scrollable_section_box(
        f,
        area,
        "PROPERTIES [3]",
        &content.properties,
        props_state,
        app.props_scroll,
    );
    app.props_line_count = total_lines;
}
