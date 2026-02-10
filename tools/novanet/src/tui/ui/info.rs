//! Info panel rendering for TUI.
//!
//! This module contains all functions related to rendering the Info panel,
//! which displays details about the currently selected tree item.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Bar, BarChart, BarGroup, Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Sparkline,
};
use std::collections::BTreeMap;

use crate::tui::app::{App, Focus};
use crate::tui::data::{ArcDirection, TreeItem};
use crate::tui::schema::{PropertyStatus, ValidationStatus};
use crate::tui::theme::hex_to_color;
use crate::tui::unicode::{display_width, truncate_to_width};

use serde_json::Value as JsonValue;

use super::{
    COLOR_UNFOCUSED_BORDER, STYLE_ACCENT, STYLE_DESC, STYLE_DIM, STYLE_ERROR, STYLE_HIGHLIGHT,
    STYLE_HINT, STYLE_INFO, STYLE_MUTED, STYLE_PRIMARY, STYLE_SUCCESS, STYLE_WARNING,
    scroll_indicator, trait_icon, wrap_text,
};

// =============================================================================
// CONSTANTS
// =============================================================================

/// Major separator line for sections.
const SEPARATOR_MAJOR: &str = "══════════════════════════";

/// Arc family label style.
const STYLE_ARC_FAMILY: Style = Style::new().fg(Color::Rgb(180, 140, 80));

// =============================================================================
// HELPER FUNCTIONS (local to this module)
// =============================================================================

/// Convert property type to short badge for schema overlay.
/// All badges are exactly 4 characters for consistent column alignment.
fn type_badge(prop_type: &str) -> &'static str {
    match prop_type.to_lowercase().as_str() {
        "string" => "str ",
        "json" => "json",
        "enum" => "enum",
        "datetime" => "dt  ",
        "int" | "integer" => "int ",
        "float" | "number" => "num ",
        "bool" | "boolean" => "bool",
        "array" | "list" => "arr ",
        "object" | "map" => "obj ",
        "url" | "uri" => "url ",
        "?" => "?   ", // unknown type from validation
        _ => "··· ",   // fallback for unknown types
    }
}

/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "..." if truncated. Handles CJK, emoji, and combining characters.
fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}

// =============================================================================
// INFO PANEL RENDERING
// =============================================================================

/// Info panel: displays metadata for selected item with independent scroll.
/// Shows sparklines and charts based on the selected item type.
///
/// v12 Sparkline Stats:
/// - Realm: Kinds/Layer bar chart + Trait distribution + Instance sparkline
/// - Layer: Instance sparkline per Kind
/// - Kind: Arc distribution (incoming vs outgoing)
pub fn render_info_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Info;
    let border_color = if focused {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Check if we should show a chart (Realm, Layer, or Kind item)
    let is_realm = matches!(app.current_item(), Some(TreeItem::Realm(_)));
    let is_layer = matches!(app.current_item(), Some(TreeItem::Layer(_, _)));
    let is_kind = matches!(app.current_item(), Some(TreeItem::Kind(..)));

    if is_realm && area.height > 25 {
        // v12: Full realm stats with all charts
        // Split: text (min 6) | bar chart (8) | trait dist (5) | health (5) | instances (5)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(6),
                Constraint::Length(8),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(area);

        render_info_text(f, chunks[0], app, focused, border_color);
        render_realm_bar_chart(f, chunks[1], app);
        render_realm_trait_distribution(f, chunks[2], app);
        render_realm_health_sparkline(f, chunks[3], app);
        render_realm_instance_sparkline(f, chunks[4], app);
    } else if is_realm && area.height > 20 {
        // v12: Three charts (bar, trait, health)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(6),
                Constraint::Length(8),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(area);

        render_info_text(f, chunks[0], app, focused, border_color);
        render_realm_bar_chart(f, chunks[1], app);
        render_realm_trait_distribution(f, chunks[2], app);
        render_realm_health_sparkline(f, chunks[3], app);
    } else if is_realm && area.height > 15 {
        // Medium height: bar chart + trait distribution
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(8), Constraint::Length(5)])
            .split(area);

        render_info_text(f, chunks[0], app, focused, border_color);
        render_realm_bar_chart(f, chunks[1], app);
        render_realm_trait_distribution(f, chunks[2], app);
    } else if is_realm && area.height > 12 {
        // Original: just bar chart
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(8)])
            .split(area);

        render_info_text(f, chunks[0], app, focused, border_color);
        render_realm_bar_chart(f, chunks[1], app);
    } else if is_layer && area.height > 10 {
        // Layer: Instance sparkline per Kind
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(5)])
            .split(area);

        render_info_text(f, chunks[0], app, focused, border_color);
        render_layer_sparkline(f, chunks[1], app);
    } else if is_kind && area.height > 10 {
        // Kind: Arc distribution chart
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(6), Constraint::Length(5)])
            .split(area);

        render_info_text(f, chunks[0], app, focused, border_color);
        render_kind_arc_chart(f, chunks[1], app);
    } else {
        // Normal text-only info panel (fallback for small height)
        render_info_text(f, area, app, focused, border_color);
    }
}

/// Render the text portion of the info panel.
fn render_info_text(f: &mut Frame, area: Rect, app: &mut App, focused: bool, border_color: Color) {
    // Build info lines
    let all_lines = build_info_lines(app);

    // Update line count for scroll bounds
    app.info_line_count = all_lines.len();

    // Apply scroll
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.info_scroll)
        .take(visible_height)
        .collect();

    // Get title from current item
    let title = get_detail_title(app);

    // Build scroll indicator with directional arrows
    let scroll_hint = scroll_indicator(app.info_scroll, app.info_line_count, visible_height);

    let block = Block::default()
        .title(Span::styled(format!(" {} ", title), STYLE_PRIMARY))
        .title_bottom(Span::styled(scroll_hint, STYLE_DIM))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if focused { Color::Cyan } else { border_color }));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    if app.info_line_count > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state =
            ScrollbarState::new(app.info_line_count.saturating_sub(visible_height))
                .position(app.info_scroll);

        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(2),
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Render a bar chart showing kinds per layer for the selected Realm.
fn render_realm_bar_chart(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Realm(realm)) = app.current_item() else {
        return;
    };

    // Build bar data from layers
    let bars: Vec<Bar> = realm
        .layers
        .iter()
        .map(|layer| {
            let count = layer.kinds.len() as u64;
            // Use first 4 chars of layer name as label (Unicode-safe)
            let label: String = layer.display_name.chars().take(4).collect();
            Bar::default()
                .value(count)
                .label(Line::from(label))
                .style(Style::default().fg(hex_to_color(&layer.color)))
        })
        .collect();

    if bars.is_empty() {
        return;
    }

    let bar_group = BarGroup::default().bars(&bars);

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(Span::styled(" Kinds/Layer ", STYLE_DIM))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(bar_group)
        .bar_width(5)
        .bar_gap(1)
        .value_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::Gray));

    f.render_widget(chart, area);
}

/// Render a sparkline showing instance counts per kind for the selected Layer.
fn render_layer_sparkline(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Layer(_, layer)) = app.current_item() else {
        return;
    };

    // Collect instance counts from kinds
    let data: Vec<u64> = layer
        .kinds
        .iter()
        .map(|k| k.instance_count.max(0) as u64)
        .collect();

    if data.is_empty() {
        return;
    }

    // Calculate max for label
    let max_val = *data.iter().max().unwrap_or(&0);
    let total: u64 = data.iter().sum();

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Instances ({} total, max {}) ", total, max_val),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(&data)
        .style(Style::default().fg(hex_to_color(&layer.color)));

    f.render_widget(sparkline, area);
}

/// Render a bar chart showing incoming vs outgoing arc distribution for the selected Kind.
fn render_kind_arc_chart(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Kind(_, _, kind)) = app.current_item() else {
        return;
    };

    // Count incoming and outgoing arcs from kind definition
    let incoming: usize = kind
        .arcs
        .iter()
        .filter(|a| a.direction == ArcDirection::Incoming)
        .count();
    let outgoing: usize = kind
        .arcs
        .iter()
        .filter(|a| a.direction == ArcDirection::Outgoing)
        .count();

    if incoming == 0 && outgoing == 0 {
        // No arcs, show placeholder
        let block = Block::default()
            .title(Span::styled(" Arc Distribution ", STYLE_DIM))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));
        let paragraph = Paragraph::new(Span::styled("  No arcs defined", STYLE_MUTED)).block(block);
        f.render_widget(paragraph, area);
        return;
    }

    // Build bar data
    let bars = vec![
        Bar::default()
            .value(incoming as u64)
            .label(Line::from("← In"))
            .style(Style::default().fg(Color::Green)),
        Bar::default()
            .value(outgoing as u64)
            .label(Line::from("Out →"))
            .style(Style::default().fg(Color::Cyan)),
    ];

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Arc Distribution ({} total) ", incoming + outgoing),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(BarGroup::default().bars(&bars))
        .bar_width(8)
        .bar_gap(2)
        .direction(Direction::Vertical);

    f.render_widget(chart, area);
}

/// Render a sparkline showing health percentages across all Kinds in a Realm.
/// Provides a quick visual overview of data quality distribution.
fn render_realm_health_sparkline(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Realm(realm)) = app.current_item() else {
        return;
    };

    // Collect health percentages from all kinds in all layers
    let data: Vec<u64> = realm
        .layers
        .iter()
        .flat_map(|l| l.kinds.iter())
        .map(|k| k.health_percent.unwrap_or(0) as u64)
        .collect();

    if data.is_empty() {
        return;
    }

    // Calculate stats
    let total: u64 = data.iter().sum();
    let count = data.len() as u64;
    let avg = if count > 0 { total / count } else { 0 };
    let min = *data.iter().min().unwrap_or(&0);
    let max = *data.iter().max().unwrap_or(&0);

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Health Distribution (avg {}%, min {}%, max {}%) ", avg, min, max),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(&data)
        .style(Style::default().fg(Color::Green));

    f.render_widget(sparkline, area);
}

/// Render a bar chart showing trait distribution across Kinds in a Realm.
/// Shows how many Kinds belong to each trait category.
fn render_realm_trait_distribution(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Realm(realm)) = app.current_item() else {
        return;
    };

    // Count kinds by trait
    let mut trait_counts: BTreeMap<&str, u64> = BTreeMap::new();
    for layer in &realm.layers {
        for kind in &layer.kinds {
            *trait_counts.entry(kind.trait_name.as_str()).or_insert(0) += 1;
        }
    }

    if trait_counts.is_empty() {
        return;
    }

    // Build bars with trait colors
    let trait_colors: [(&str, Color); 5] = [
        ("invariant", Color::Rgb(38, 139, 210)),   // Blue
        ("localized", Color::Rgb(211, 54, 130)),   // Magenta
        ("knowledge", Color::Rgb(181, 137, 0)),    // Yellow
        ("generated", Color::Rgb(133, 153, 0)),    // Green
        ("aggregated", Color::Rgb(108, 113, 196)), // Violet
    ];

    let bars: Vec<Bar> = trait_colors
        .iter()
        .filter_map(|(trait_name, color)| {
            trait_counts.get(trait_name).map(|&count| {
                // First 3 chars as label
                let label: String = trait_name.chars().take(3).collect();
                Bar::default()
                    .value(count)
                    .label(Line::from(label))
                    .style(Style::default().fg(*color))
            })
        })
        .collect();

    if bars.is_empty() {
        return;
    }

    let total: u64 = trait_counts.values().sum();
    let chart = BarChart::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Trait Distribution ({} Kinds) ", total),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(BarGroup::default().bars(&bars))
        .bar_width(4)
        .bar_gap(1)
        .value_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .label_style(Style::default().fg(Color::Gray));

    f.render_widget(chart, area);
}

/// Render a sparkline showing instance counts across Kinds in a Realm.
/// Provides a quick visual overview of data distribution.
fn render_realm_instance_sparkline(f: &mut Frame, area: Rect, app: &App) {
    let Some(TreeItem::Realm(realm)) = app.current_item() else {
        return;
    };

    // Collect instance counts from all kinds in all layers
    let data: Vec<u64> = realm
        .layers
        .iter()
        .flat_map(|l| l.kinds.iter())
        .map(|k| k.instance_count.max(0) as u64)
        .collect();

    if data.is_empty() {
        return;
    }

    // Calculate stats
    let total: u64 = data.iter().sum();
    let max_val = *data.iter().max().unwrap_or(&0);

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(Span::styled(
                    format!(" Instance Distribution ({} total, max {}) ", total, max_val),
                    STYLE_DIM,
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER)),
        )
        .data(&data)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(sparkline, area);
}

/// Get title for detail panel based on current selection.
/// Uses [K] badge for Kind and [I] badge for Instance for instant recognition.
fn get_detail_title(app: &App) -> String {
    match app.current_item() {
        Some(TreeItem::KindsSection) => "Node Kinds".to_string(),
        Some(TreeItem::ArcsSection) => "Arcs".to_string(),
        Some(TreeItem::Realm(r)) => format!("{} {}", r.icon, r.display_name),
        Some(TreeItem::Layer(_, l)) => l.display_name.clone(),
        Some(TreeItem::Kind(_, _, k)) => {
            // [K] badge for Kind - instant recognition
            if k.icon.is_empty() {
                format!("[K] {}", k.display_name)
            } else {
                format!("[K] {} {}", k.icon, k.display_name)
            }
        }
        Some(TreeItem::ArcFamily(f)) => f.display_name.clone(),
        Some(TreeItem::ArcKind(_, ek)) => ek.display_name.clone(),
        Some(TreeItem::Instance(_, _, _, inst)) => {
            // [I] badge for Instance - instant recognition
            format!("[I] {} ({})", inst.key, inst.kind_key)
        }
        Some(TreeItem::EntityCategory(_, _, _, cat)) => {
            // [C] badge for Category
            format!("[C] {}", cat.display_name)
        }
        None => "Detail".to_string(),
    }
}

// =============================================================================
// BUILD INFO LINES
// =============================================================================

/// Build info lines for detail panel.
fn build_info_lines(app: &App) -> Vec<Line<'static>> {
    // Use mode-aware item lookup (shows instances in Data mode)
    match app.current_item() {
        Some(TreeItem::KindsSection) => {
            let theme = &app.theme;
            let kind_count: usize = app
                .tree
                .realms
                .iter()
                .flat_map(|r| r.layers.iter())
                .map(|l| l.kinds.len())
                .sum();

            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Section", STYLE_ACCENT),
                ]),
                Line::from(vec![
                    Span::styled("realms    ", STYLE_DIM),
                    Span::styled(app.tree.realms.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(kind_count.to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
            ];

            // Add realm distribution breakdown
            if kind_count > 0 {
                lines.push(Line::from(Span::styled(
                    "REALM DISTRIBUTION",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let bar_width = 16usize;
                for realm in &app.tree.realms {
                    let realm_kinds: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
                    let percent = (realm_kinds as f64 / kind_count as f64 * 100.0).round() as u8;
                    let filled = (realm_kinds * bar_width) / kind_count.max(1);
                    let bar = "█".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));

                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{:8} ", realm.display_name),
                            Style::default().fg(theme.realm_color(&realm.key)),
                        ),
                        Span::styled(bar, Style::default().fg(theme.realm_color(&realm.key))),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
                        Span::styled(format!("  {} Kinds", realm_kinds), STYLE_DIM),
                    ]));
                }
                lines.push(Line::from(""));
            }

            lines.push(Line::from(Span::styled(
                "h/l to collapse/expand",
                STYLE_DIM,
            )));
            lines
        }
        Some(TreeItem::ArcsSection) => {
            let arc_count: usize = app
                .tree
                .arc_families
                .iter()
                .map(|f| f.arc_kinds.len())
                .sum();
            vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Section", STYLE_HIGHLIGHT),
                ]),
                Line::from(vec![
                    Span::styled("families  ", STYLE_DIM),
                    Span::styled(app.tree.arc_families.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("arcs      ", STYLE_DIM),
                    Span::styled(arc_count.to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", STYLE_DIM)),
            ]
        }
        Some(TreeItem::Realm(realm)) => {
            let theme = &app.theme;
            let kind_count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Realm", STYLE_ACCENT),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(realm.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("layers    ", STYLE_DIM),
                    Span::styled(realm.layers.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(kind_count.to_string(), STYLE_PRIMARY),
                ]),
            ];

            // Add layer breakdown if there are layers with kinds
            if kind_count > 0 {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "LAYER BREAKDOWN",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let bar_width = 12usize;
                for layer in &realm.layers {
                    let count = layer.kinds.len();
                    if count == 0 {
                        continue;
                    }
                    let percent = (count as f64 / kind_count as f64 * 100.0).round() as u8;
                    let filled = (count * bar_width) / kind_count.max(1);
                    let bar = "█".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));
                    let layer_color = theme.layer_color(&layer.key);

                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default().fg(layer_color)),
                        Span::styled(
                            format!("{:16} ", layer.display_name),
                            Style::default().fg(layer_color),
                        ),
                        Span::styled(bar, Style::default().fg(layer_color)),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
                        Span::styled(format!("  {}", count), STYLE_DIM),
                    ]));
                }
            }

            lines
        }
        Some(TreeItem::Layer(realm, layer)) => {
            let theme = &app.theme;
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("Layer", STYLE_SUCCESS),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(layer.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", STYLE_DIM),
                    Span::styled(layer.kinds.len().to_string(), STYLE_PRIMARY),
                ]),
            ];

            // Add trait breakdown if there are kinds
            if !layer.kinds.is_empty() {
                // Count kinds by trait
                let mut trait_counts: std::collections::BTreeMap<String, usize> =
                    std::collections::BTreeMap::new();
                for kind in &layer.kinds {
                    *trait_counts.entry(kind.trait_name.clone()).or_insert(0) += 1;
                }

                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "TRAIT BREAKDOWN",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let total = layer.kinds.len();
                let bar_width = 12usize;
                for (trait_name, count) in &trait_counts {
                    let percent = (*count as f64 / total as f64 * 100.0).round() as u8;
                    let filled = (*count * bar_width) / total.max(1);
                    let bar = "█".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));
                    let icon = trait_icon(trait_name);

                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{} ", icon),
                            Style::default().fg(theme.trait_color(trait_name)),
                        ),
                        Span::styled(
                            format!("{:12} ", trait_name),
                            Style::default().fg(theme.trait_color(trait_name)),
                        ),
                        Span::styled(bar, Style::default().fg(theme.trait_color(trait_name))),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
                        Span::styled(format!("  {}", count), STYLE_DIM),
                    ]));
                }
            }

            lines
        }
        Some(TreeItem::Kind(realm, layer, kind)) => {
            let theme = &app.theme;

            // Unified header: type, key, kind, realm, layer, trait (12-char labels)
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type        ", STYLE_DIM),
                    Span::styled("Node Kind", STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("key         ", STYLE_DIM),
                    Span::styled(kind.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kind        ", STYLE_DIM),
                    Span::styled("—", STYLE_DIM),
                ]),
                Line::from(vec![
                    Span::styled("realm       ", STYLE_DIM),
                    Span::styled(format!("{} ", realm.icon), STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("layer       ", STYLE_DIM),
                    Span::styled(format!("{} ", theme.icons.layer(&layer.key)), STYLE_DIM),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(hex_to_color(&layer.color)),
                    ),
                ]),
            ];

            // Trait with icon (if present)
            if !kind.trait_name.is_empty() {
                let trait_icon = theme.icons.trait_icon(&kind.trait_name);
                lines.push(Line::from(vec![
                    Span::styled("trait       ", STYLE_DIM),
                    Span::styled(format!("{} ", trait_icon), STYLE_DIM),
                    Span::styled(
                        kind.trait_name.clone(),
                        Style::default().fg(theme.trait_color(&kind.trait_name)),
                    ),
                ]));
            }

            // v10.1: knowledge_tier removed from display (node type is sufficient)

            // Instances count (aligned with Instance view)
            let instance_count = kind.instance_count;
            lines.push(Line::from(vec![
                Span::styled("instances   ", STYLE_DIM),
                Span::styled(format!("{} total", instance_count), STYLE_MUTED),
            ]));

            // Blank line before stats section
            lines.push(Line::from(""));

            // Properties line (aligned with Instance view)
            let total_props = kind.properties.len();

            // Format: "properties  8 defined ████░░░░"
            let bar_width = 10usize;
            let log_val = if instance_count > 0 {
                (instance_count as f64).log10().max(0.0)
            } else {
                0.0
            };
            let filled = ((log_val / 4.0) * bar_width as f64).round() as usize;
            let filled = filled.clamp(if instance_count > 0 { 1 } else { 0 }, bar_width);
            let bar = "━".repeat(filled);
            let empty = "░".repeat(bar_width.saturating_sub(filled));

            lines.push(Line::from(vec![
                Span::styled("properties  ", STYLE_DIM),
                Span::styled(format!("{} defined", total_props), STYLE_INFO),
                Span::styled(" ", STYLE_DIM),
                Span::styled(bar, STYLE_SUCCESS),
                Span::styled(empty, STYLE_DIM),
            ]));

            // Context budget (if present)
            if !kind.context_budget.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("budget      ", STYLE_DIM),
                    Span::styled(kind.context_budget.clone(), STYLE_INFO),
                ]));
            }

            // Property coverage summary
            let total_props = kind.properties.len();
            let required_props = kind.required_properties.len();
            let optional_props = total_props.saturating_sub(required_props);

            if total_props > 0 {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "PROPERTY COVERAGE",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                let bar_width = 12usize;
                // Required bar
                let req_percent =
                    (required_props as f64 / total_props as f64 * 100.0).round() as u8;
                let req_filled = (required_props * bar_width) / total_props.max(1);
                let req_bar = "█".repeat(req_filled.max(if required_props > 0 { 1 } else { 0 }));
                let req_empty = "░".repeat(bar_width.saturating_sub(req_filled));

                lines.push(Line::from(vec![
                    Span::styled("* ", Style::default().fg(Color::Red)),
                    Span::styled("required     ", Style::default().fg(Color::Yellow)),
                    Span::styled(req_bar, Style::default().fg(Color::Yellow)),
                    Span::styled(req_empty, STYLE_DIM),
                    Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
                    Span::styled(format!("  {}", required_props), STYLE_DIM),
                ]));

                // Optional bar
                let opt_percent =
                    (optional_props as f64 / total_props as f64 * 100.0).round() as u8;
                let opt_filled = (optional_props * bar_width) / total_props.max(1);
                let opt_bar = "█".repeat(opt_filled.max(if optional_props > 0 { 1 } else { 0 }));
                let opt_empty = "░".repeat(bar_width.saturating_sub(opt_filled));

                lines.push(Line::from(vec![
                    Span::styled("  ", STYLE_DIM),
                    Span::styled("optional     ", Style::default().fg(Color::White)),
                    Span::styled(opt_bar, Style::default().fg(Color::White)),
                    Span::styled(opt_empty, STYLE_DIM),
                    Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
                    Span::styled(format!("  {}", optional_props), STYLE_DIM),
                ]));
            }

            // Properties section with validation (Neo4j ↔ YAML)
            // If validated properties available, show with validation status
            // Otherwise fall back to simple property list
            if let Some(validated) = &app.validated_kind_properties {
                lines.push(Line::from(""));

                // Header with validation stats
                if let Some(stats) = &app.validation_stats {
                    lines.push(Line::from(vec![
                        Span::styled(format!("Properties ({}) ", validated.len()), STYLE_MUTED),
                        Span::styled(format!("✓{}", stats.sync_count), STYLE_SUCCESS),
                        Span::styled(" ", STYLE_DIM),
                        if stats.missing_count > 0 {
                            Span::styled(format!("⚠{}", stats.missing_count), STYLE_WARNING)
                        } else {
                            Span::styled("", STYLE_DIM)
                        },
                        Span::styled(" ", STYLE_DIM),
                        if stats.extra_count > 0 {
                            Span::styled(format!("?{}", stats.extra_count), STYLE_DIM)
                        } else {
                            Span::styled("", STYLE_DIM)
                        },
                    ]));
                } else {
                    lines.push(Line::from(Span::styled(
                        format!("Properties ({})", validated.len()),
                        STYLE_MUTED,
                    )));
                }

                // Render each validated property
                for prop in validated {
                    let (status_icon, status_style) = match prop.status {
                        ValidationStatus::Sync => ("✓", STYLE_SUCCESS),
                        ValidationStatus::Missing => ("⚠", STYLE_WARNING),
                        ValidationStatus::Extra => ("?", STYLE_DIM),
                    };

                    let required_marker = if prop.required { "*" } else { " " };
                    let type_badge = type_badge(&prop.prop_type);

                    // Example value (if available)
                    let example_str = prop
                        .example
                        .as_ref()
                        .map(|e| format!("→ {}", truncate_str(e, 25)))
                        .unwrap_or_default();

                    lines.push(Line::from(vec![
                        Span::styled(status_icon, status_style),
                        Span::styled(
                            required_marker,
                            Style::default().fg(Color::Rgb(255, 100, 100)),
                        ),
                        Span::styled(format!("[{:4}] ", type_badge), STYLE_DIM),
                        Span::styled(format!("{:<15}", prop.name), STYLE_INFO),
                        Span::styled(example_str, STYLE_MUTED),
                    ]));
                }

                // Legend
                lines.push(Line::from(vec![
                    Span::styled("  ✓=sync ⚠=missing ?=extra  ", STYLE_DIM),
                    Span::styled("*=required", STYLE_DIM),
                ]));
            } else if !kind.properties.is_empty() {
                // Fallback: simple property list (no YAML loaded)
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Properties ({})", kind.properties.len()),
                    STYLE_MUTED,
                )));

                for prop in &kind.properties {
                    let is_required = kind.required_properties.contains(prop);
                    let marker = if is_required { "*" } else { " " };
                    let prop_color = if is_required {
                        Color::Yellow
                    } else {
                        Color::White
                    };

                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  {}", marker),
                            Style::default().fg(Color::Rgb(255, 100, 100)),
                        ),
                        Span::styled(prop.clone(), Style::default().fg(prop_color)),
                    ]));
                }

                // Legend
                lines.push(Line::from(Span::styled("  * = required", STYLE_DIM)));
            }

            // Arcs section
            if !kind.arcs.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Arcs ({})", kind.arcs.len()),
                    STYLE_MUTED,
                )));

                for arc in &kind.arcs {
                    let (arrow, arrow_color) = match arc.direction {
                        ArcDirection::Outgoing => ("→", Color::Cyan),
                        ArcDirection::Incoming => ("←", Color::Magenta),
                    };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", arrow), Style::default().fg(arrow_color)),
                        Span::styled(arc.arc_type.clone(), Style::default().fg(arrow_color)),
                        Span::styled(" → ", STYLE_DIM),
                        Span::styled(arc.target_kind.clone(), STYLE_HIGHLIGHT),
                    ]));
                }
            }

            // Description
            if !kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Description", STYLE_MUTED)));
                // Wrap description to multiple lines (no Vec<char> allocation)
                for line in wrap_text(&kind.description, 60) {
                    lines.push(Line::from(Span::styled(format!("  {}", line), STYLE_DESC)));
                }
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Cypher", STYLE_MUTED)));
            lines.push(Line::from(Span::styled(
                format!("  MATCH (n:{}) RETURN n LIMIT 100", kind.key),
                STYLE_HINT,
            )));

            lines
        }
        Some(TreeItem::ArcFamily(family)) => {
            vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("ArcFamily", STYLE_ARC_FAMILY),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(family.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("arcs      ", STYLE_DIM),
                    Span::styled(family.arc_kinds.len().to_string(), STYLE_PRIMARY),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", STYLE_DIM)),
            ]
        }
        Some(TreeItem::ArcKind(family, arc_kind)) => {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", STYLE_DIM),
                    Span::styled("ArcKind", STYLE_HIGHLIGHT),
                ]),
                Line::from(vec![
                    Span::styled("key       ", STYLE_DIM),
                    Span::styled(arc_kind.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("family    ", STYLE_DIM),
                    Span::styled(family.display_name.clone(), STYLE_ARC_FAMILY),
                ]),
                Line::from(vec![
                    Span::styled("from      ", STYLE_DIM),
                    Span::styled(arc_kind.from_kind.clone(), STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("to        ", STYLE_DIM),
                    Span::styled(arc_kind.to_kind.clone(), STYLE_INFO),
                ]),
            ];

            // Cardinality (if present)
            if !arc_kind.cardinality.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("cardin.   ", STYLE_DIM),
                    Span::styled(arc_kind.cardinality.clone(), STYLE_ACCENT),
                ]));
            }

            // Description (if present)
            if !arc_kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Description", STYLE_MUTED)));
                lines.push(Line::from(Span::styled(
                    format!("  {}", &arc_kind.description),
                    STYLE_DESC,
                )));
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Cypher", STYLE_MUTED)));
            lines.push(Line::from(Span::styled(
                format!("  MATCH ()-[r:{}]->() RETURN r LIMIT 100", arc_kind.key),
                STYLE_HINT,
            )));

            lines
        }
        Some(TreeItem::Instance(realm, layer, kind, instance)) => {
            // Instance info for Data view
            // Unified header: type, key, kind, realm, layer, trait (12-char labels + icons)
            let theme = &app.theme;

            // Header - matches Kind view structure for easy comparison
            let mut lines: Vec<Line<'static>> = vec![
                Line::from(vec![
                    Span::styled("type        ", STYLE_DIM),
                    Span::styled("Instance", STYLE_SUCCESS),
                ]),
                Line::from(vec![
                    Span::styled("key         ", STYLE_DIM),
                    Span::styled(instance.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("kind        ", STYLE_DIM),
                    Span::styled(kind.display_name.clone(), STYLE_INFO),
                ]),
                Line::from(vec![
                    Span::styled("realm       ", STYLE_DIM),
                    Span::styled(format!("{} ", realm.icon), STYLE_DIM),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("layer       ", STYLE_DIM),
                    Span::styled(format!("{} ", theme.icons.layer(&layer.key)), STYLE_DIM),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(hex_to_color(&layer.color)),
                    ),
                ]),
            ];
            // Trait with icon
            if !kind.trait_name.is_empty() {
                let trait_icon = theme.icons.trait_icon(&kind.trait_name);
                lines.push(Line::from(vec![
                    Span::styled("trait       ", STYLE_DIM),
                    Span::styled(format!("{} ", trait_icon), STYLE_DIM),
                    Span::styled(
                        kind.trait_name.clone(),
                        Style::default().fg(theme.trait_color(&kind.trait_name)),
                    ),
                ]));
            }

            // Instances count (aligned with Kind's "properties" line context)
            if kind.instance_count > 0 {
                lines.push(Line::from(vec![
                    Span::styled("instances   ", STYLE_DIM),
                    Span::styled(format!("{} total", kind.instance_count), STYLE_MUTED),
                ]));
            }

            // Properties with optional Schema Overlay
            // If schema overlay is enabled and we have matched properties, show schema view
            // Otherwise, fall back to simple property list
            if app.schema_overlay_enabled {
                if let Some(matched) = &app.matched_properties {
                    // Schema overlay: show all schema properties with status
                    let stats = app.coverage_stats.as_ref();
                    let (filled, total) = stats.map(|s| (s.filled, s.total)).unwrap_or((
                        matched
                            .iter()
                            .filter(|p| p.status == PropertyStatus::Filled)
                            .count(),
                        matched.len(),
                    ));
                    let percent = if total > 0 {
                        (filled * 100) / total
                    } else {
                        100
                    };

                    lines.push(Line::from(""));

                    // Properties header (aligned with Kind view)
                    // Format: "properties  14/14 filled ━━━━━━━━━━ 100%"
                    let bar_width = 10usize;
                    let progress_filled = (percent * bar_width) / 100;
                    let progress_empty = bar_width.saturating_sub(progress_filled);
                    lines.push(Line::from(vec![
                        Span::styled("properties  ", STYLE_DIM),
                        Span::styled(format!("{}/{} filled", filled, total), STYLE_INFO),
                        Span::styled(" ", STYLE_DIM),
                        Span::styled("━".repeat(progress_filled), STYLE_SUCCESS),
                        Span::styled("░".repeat(progress_empty), STYLE_DIM),
                        Span::styled(format!(" {}%", percent), STYLE_MUTED),
                    ]));

                    // Status line (aligned with Kind's "budget" line)
                    let missing_required = matched
                        .iter()
                        .filter(|p| p.schema.required && p.status != PropertyStatus::Filled)
                        .count();
                    let (status_text, status_style) = if missing_required > 0 {
                        (
                            format!("missing {} required", missing_required),
                            STYLE_ERROR,
                        )
                    } else if percent == 100 {
                        ("complete".to_string(), STYLE_SUCCESS)
                    } else {
                        ("partial".to_string(), STYLE_INFO)
                    };
                    lines.push(Line::from(vec![
                        Span::styled("status      ", STYLE_DIM),
                        Span::styled(status_text, status_style),
                    ]));

                    // PROPERTY COVERAGE section (aligned with Kind view)
                    let required_count = matched.iter().filter(|p| p.schema.required).count();
                    let optional_count = matched.len().saturating_sub(required_count);
                    let required_filled = matched
                        .iter()
                        .filter(|p| p.schema.required && p.status == PropertyStatus::Filled)
                        .count();
                    let optional_filled = matched
                        .iter()
                        .filter(|p| !p.schema.required && p.status == PropertyStatus::Filled)
                        .count();

                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        "PROPERTY COVERAGE",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )));
                    lines.push(Line::from(Span::styled(SEPARATOR_MAJOR, STYLE_DIM)));

                    // Required bar
                    let req_percent = if required_count > 0 {
                        (required_filled * 100) / required_count
                    } else {
                        100
                    };
                    let req_bar_filled = (req_percent * bar_width) / 100;
                    let req_bar_filled =
                        req_bar_filled.max(if required_filled > 0 { 1 } else { 0 });
                    lines.push(Line::from(vec![
                        Span::styled("* ", Style::default().fg(Color::Red)),
                        Span::styled("required     ", Style::default().fg(Color::Yellow)),
                        Span::styled(
                            "█".repeat(req_bar_filled),
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::styled(
                            "░".repeat(bar_width.saturating_sub(req_bar_filled)),
                            STYLE_DIM,
                        ),
                        Span::styled(format!(" {:>3}%", req_percent), STYLE_MUTED),
                        Span::styled(
                            format!("  {}/{}", required_filled, required_count),
                            STYLE_DIM,
                        ),
                    ]));

                    // Optional bar
                    let opt_percent = if optional_count > 0 {
                        (optional_filled * 100) / optional_count
                    } else {
                        100
                    };
                    let opt_bar_filled = (opt_percent * bar_width) / 100;
                    let opt_bar_filled =
                        opt_bar_filled.max(if optional_filled > 0 { 1 } else { 0 });
                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled("optional     ", Style::default().fg(Color::Gray)),
                        Span::styled("█".repeat(opt_bar_filled), Style::default().fg(Color::Gray)),
                        Span::styled(
                            "░".repeat(bar_width.saturating_sub(opt_bar_filled)),
                            STYLE_DIM,
                        ),
                        Span::styled(format!(" {:>3}%", opt_percent), STYLE_MUTED),
                        Span::styled(
                            format!("  {}/{}", optional_filled, optional_count),
                            STYLE_DIM,
                        ),
                    ]));

                    // Properties list header
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        format!("Properties ({}) ✓{}", matched.len(), filled),
                        STYLE_MUTED,
                    )));

                    // Show each property with status
                    // Feature 3: Track focused property index for intelligent truncation
                    // Feature 6: Type badges [str], [json], [enum], etc.
                    for (prop_idx, prop) in matched.iter().enumerate() {
                        let is_required = prop.schema.required;
                        let prefix = if is_required { "*" } else { " " };
                        let badge = type_badge(&prop.schema.prop_type);
                        // Feature 3: Show full value when property is focused
                        let is_focused = prop_idx == app.focused_property_idx;
                        let truncate_limit = if is_focused { 200 } else { 40 };

                        match prop.status {
                            PropertyStatus::Filled => {
                                // Has value: show normally with type badge
                                let value_str = prop
                                    .value
                                    .as_ref()
                                    .map(|v| {
                                        if app.json_pretty
                                            && (v.starts_with('{') || v.starts_with('['))
                                        {
                                            // Pretty-print JSON
                                            serde_json::from_str::<serde_json::Value>(v)
                                                .ok()
                                                .and_then(|j| serde_json::to_string_pretty(&j).ok())
                                                .unwrap_or_else(|| v.clone())
                                        } else {
                                            v.clone()
                                        }
                                    })
                                    .unwrap_or_default();
                                // Feature 3: Highlight focused property row
                                let name_style = if is_focused {
                                    STYLE_HIGHLIGHT
                                } else {
                                    STYLE_INFO
                                };

                                // Feature 3b: Expand text with Enter toggle
                                if is_focused && app.expanded_property {
                                    // Expanded: show full value with word-wrap
                                    // First line with property name and expand indicator
                                    lines.push(Line::from(vec![
                                        Span::styled(
                                            format!("{}[{:4}] ", prefix, badge),
                                            STYLE_DIM,
                                        ),
                                        Span::styled(
                                            format!("{:<15}", prop.schema.name),
                                            name_style,
                                        ),
                                        Span::styled("▼ ", STYLE_HINT), // Expanded indicator
                                    ]));
                                    // Wrap value text to multiple lines (no Vec<char> allocation)
                                    let full_value = format!("\"{}\"", value_str);
                                    for line in wrap_text(&full_value, 50) {
                                        lines.push(Line::from(vec![
                                            Span::styled("                        ", STYLE_DIM), // Indent
                                            Span::styled(line, STYLE_SUCCESS),
                                        ]));
                                    }
                                } else {
                                    // Collapsed: truncate as before
                                    let truncated =
                                        truncate_str(&format!("\"{}\"", value_str), truncate_limit);
                                    let indicator = if is_focused { "▶ " } else { "" };
                                    lines.push(Line::from(vec![
                                        Span::styled(
                                            format!("{}[{:4}] ", prefix, badge),
                                            STYLE_DIM,
                                        ),
                                        Span::styled(
                                            format!("{:<15}", prop.schema.name),
                                            name_style,
                                        ),
                                        Span::styled(indicator, STYLE_HINT),
                                        Span::styled(truncated, STYLE_SUCCESS),
                                    ]));
                                }
                            }
                            PropertyStatus::EmptyOptional => {
                                // Optional, empty: dim with type badge + example
                                let hint = format!(
                                    "— e.g. {}",
                                    prop.schema.example.as_deref().unwrap_or("...")
                                );
                                lines.push(Line::from(vec![
                                    Span::styled(format!("{}[{:4}] ", prefix, badge), STYLE_DIM),
                                    Span::styled(format!("{:<15}", prop.schema.name), STYLE_DIM),
                                    Span::styled(truncate_str(&hint, 40), STYLE_DIM),
                                ]));
                            }
                            PropertyStatus::MissingRequired => {
                                // Required, missing: red warning with type badge + example
                                let hint = format!(
                                    "⚠ e.g. {}",
                                    prop.schema.example.as_deref().unwrap_or("...")
                                );
                                lines.push(Line::from(vec![
                                    Span::styled(format!("{}[{:4}] ", prefix, badge), STYLE_ERROR),
                                    Span::styled(format!("{:<15}", prop.schema.name), STYLE_ERROR),
                                    Span::styled(truncate_str(&hint, 40), STYLE_ERROR),
                                ]));
                            }
                        }
                    }
                } else {
                    // Schema overlay enabled but no matched properties loaded yet
                    // Fall back to simple display
                    render_simple_properties(&mut lines, &instance.properties);
                }
            } else {
                // Schema overlay disabled: simple property list with fill rate header
                let total_schema_props = kind.properties.len();
                let filled_props = instance.properties.len();

                if total_schema_props > 0 && filled_props > 0 {
                    let fill_percent = ((filled_props as f64 / total_schema_props as f64) * 100.0)
                        .round()
                        .min(100.0) as usize;
                    let bar_width = 10usize;
                    let filled = (fill_percent * bar_width) / 100;
                    let bar = "━".repeat(filled.max(1));
                    let empty = "░".repeat(bar_width.saturating_sub(filled));

                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("Properties ({}/{}) ", filled_props, total_schema_props),
                            STYLE_MUTED,
                        ),
                        Span::styled(bar, STYLE_SUCCESS),
                        Span::styled(empty, STYLE_DIM),
                        Span::styled(format!(" {}%", fill_percent), STYLE_MUTED),
                    ]));

                    // Show properties in YAML definition order (kind.properties preserves order)
                    for prop_name in &kind.properties {
                        if prop_name.starts_with('_')
                            || prop_name == "key"
                            || prop_name == "display_name"
                        {
                            continue;
                        }
                        if let Some(value) = instance.properties.get(prop_name) {
                            let value_str = json_value_to_display(value);
                            let truncated = truncate_str(&value_str, 45);
                            lines.push(Line::from(vec![
                                Span::styled(format!("{:<20}", prop_name), STYLE_INFO),
                                Span::styled(truncated, STYLE_PRIMARY),
                            ]));
                        }
                    }
                } else {
                    render_simple_properties(&mut lines, &instance.properties);
                }
            }

            // Arc comparison diagram: schema arcs vs actual arcs
            // Shows existing (══) and missing (╌╌) connections
            if !kind.arcs.is_empty() {
                let comparisons = instance.compare_arcs(&kind.arcs);
                let existing_count = comparisons.iter().filter(|c| c.exists).count();
                let missing_count = comparisons.len() - existing_count;

                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!(
                        "Arc Diagram ({} exist, {} missing)",
                        existing_count, missing_count
                    ),
                    STYLE_MUTED,
                )));

                // Box drawing for instance node (use display width for CJK/emoji alignment)
                let key_width = display_width(&instance.key);
                lines.push(Line::from(Span::styled(
                    format!("  ┌{}┐", "─".repeat(key_width + 2)),
                    STYLE_INFO,
                )));
                lines.push(Line::from(Span::styled(
                    format!("  │ {} │", instance.key),
                    STYLE_INFO,
                )));
                lines.push(Line::from(Span::styled(
                    format!("  └{}┘", "─".repeat(key_width + 2)),
                    STYLE_INFO,
                )));

                // Arcs with status
                for cmp in &comparisons {
                    if cmp.exists {
                        // Existing arc: solid double line (══)
                        let target_display = cmp
                            .target_key
                            .clone()
                            .unwrap_or_else(|| cmp.target_kind.clone());
                        lines.push(Line::from(vec![
                            Span::styled("    ══", STYLE_SUCCESS),
                            Span::styled(format!("[{}]", cmp.arc_type), STYLE_HIGHLIGHT),
                            Span::styled("══> ", STYLE_SUCCESS),
                            Span::styled(target_display, STYLE_PRIMARY),
                            Span::styled(" ✓", STYLE_SUCCESS),
                        ]));
                    } else {
                        // Missing arc: dashed line (╌╌)
                        lines.push(Line::from(vec![
                            Span::styled("    ╌╌", STYLE_ERROR),
                            Span::styled(format!("[{}]", cmp.arc_type), STYLE_DIM),
                            Span::styled("╌╌> ", STYLE_ERROR),
                            Span::styled(
                                format!("({} - not connected)", cmp.target_kind),
                                STYLE_DIM,
                            ),
                            Span::styled(" ✗", STYLE_ERROR),
                        ]));
                    }
                }
            }

            lines
        }
        Some(TreeItem::EntityCategory(_, _, _, cat)) => {
            // Show category details
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("Category: ", STYLE_HINT),
                    Span::styled(cat.display_name.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("Key: ", STYLE_HINT),
                    Span::styled(cat.key.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("Question: ", STYLE_HINT),
                    Span::styled(cat.question.clone(), STYLE_PRIMARY),
                ]),
                Line::from(vec![
                    Span::styled("Entities: ", STYLE_HINT),
                    Span::styled(cat.instance_count.to_string(), STYLE_PRIMARY),
                ]),
            ];
            if !cat.llm_context.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("LLM Context:", STYLE_HINT)));
                for line in cat.llm_context.lines() {
                    lines.push(Line::from(Span::styled(format!("  {}", line), STYLE_DIM)));
                }
            }
            lines
        }
        None => {
            vec![Line::from(Span::styled("Select an item", STYLE_DIM))]
        }
    }
}

// =============================================================================
// HELPER: Simple Property Rendering
// =============================================================================

/// Render instance properties in simple mode (no schema overlay).
/// Shows each property with key-value format, truncating long values.
fn render_simple_properties(lines: &mut Vec<Line<'_>>, properties: &BTreeMap<String, JsonValue>) {
    if properties.is_empty() {
        return;
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("Properties", STYLE_MUTED)));

    for (key, value) in properties {
        // Skip internal properties (starting with underscore or known meta)
        if key.starts_with('_') || key == "key" || key == "display_name" {
            continue;
        }

        let value_str = json_value_to_display(value);
        let truncated = truncate_str(&value_str, 45);

        lines.push(Line::from(vec![
            Span::styled(format!("{:<20}", key), STYLE_INFO),
            Span::styled(truncated, STYLE_PRIMARY),
        ]));
    }
}

/// Convert a JSON value to a display string.
fn json_value_to_display(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Array(arr) => serde_json::to_string(arr).unwrap_or_else(|_| "[]".to_string()),
        JsonValue::Object(obj) => serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string()),
    }
}
