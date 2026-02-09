//! Tree panel rendering for TUI v2.
//!
//! Renders the taxonomy hierarchy with:
//! - Box-drawing characters for visual structure
//! - Collapse/expand state management
//! - Fuzzy search match highlighting
//! - Data mode instance display
//! - Filtered instances view

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use rustc_hash::FxHashSet;

use super::{
    COLOR_ACTIVE_KIND_BG, COLOR_ARC_FAMILY, COLOR_CONNECTED, COLOR_DESC_TEXT, COLOR_HIGHLIGHT_BG,
    COLOR_HINT_TEXT, COLOR_MUTED_TEXT, COLOR_UNFOCUSED_BORDER, EmptyStateKind, STYLE_DIM,
    STYLE_HIGHLIGHT, STYLE_PRIMARY, STYLE_UNFOCUSED, render_empty_state, spinner, trait_icon,
    truncate_start,
};
use crate::tui::app::{App, Focus};
use crate::tui::theme::hex_to_color;

/// Create styled spans with fuzzy match highlighting and optional background.
/// Matched character positions are shown with a yellow highlight.
/// Optional background color is applied to non-matched text segments.
fn highlight_matches_with_bg(
    text: &str,
    matches: Option<&[u32]>,
    base_color: Color,
    bg_color: Option<Color>,
) -> Vec<Span<'static>> {
    let base_style = if let Some(bg) = bg_color {
        Style::default().fg(base_color).bg(bg)
    } else {
        Style::default().fg(base_color)
    };

    let Some(positions) = matches else {
        return vec![Span::styled(text.to_string(), base_style)];
    };

    if positions.is_empty() {
        return vec![Span::styled(text.to_string(), base_style)];
    }

    let match_set: FxHashSet<usize> = positions.iter().map(|&p| p as usize).collect();
    let mut spans = Vec::with_capacity(positions.len() * 2 + 1);
    let mut current_text = String::new();
    let mut in_match = false;

    for (i, c) in text.chars().enumerate() {
        let is_match = match_set.contains(&i);

        if is_match != in_match {
            if !current_text.is_empty() {
                let style = if in_match {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    base_style
                };
                spans.push(Span::styled(std::mem::take(&mut current_text), style));
            }
            in_match = is_match;
        }
        current_text.push(c);
    }

    if !current_text.is_empty() {
        let style = if in_match {
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            base_style
        };
        spans.push(Span::styled(current_text, style));
    }

    spans
}

/// Tree panel: taxonomy hierarchy with scroll and collapse.
/// Uses box-drawing characters for visual hierarchy.
pub fn render_tree(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Tree;
    let border_color = if focused {
        Color::Cyan
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Calculate visible height (area minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    app.tree_height = visible_height;

    // === EMPTY STATE: No node kinds loaded ===
    let total_kinds: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.kinds.len())
        .sum();

    if total_kinds == 0 {
        // Render empty tree panel with border
        let block = Block::default()
            .title(" Taxonomy ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));
        f.render_widget(block, area);

        // Overlay empty state
        let inner_area = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );
        render_empty_state(f, inner_area, EmptyStateKind::NoKinds, app.tick);
        return;
    }

    // === FILTERED DATA MODE: Show only instances of selected Kind ===
    if let Some(kind_key) = app.get_filter_kind() {
        render_filtered_instances(
            f,
            area,
            app,
            kind_key,
            visible_height,
            focused,
            border_color,
        );
        return;
    }

    // Build all visible tree lines
    let mut all_lines: Vec<Line> = Vec::new();
    let mut idx = 0;

    // Helper to create a tree line with box-drawing
    // line_color: color for tree prefix (│├└ characters)
    // text_color: color for icon and text
    // match_positions: optional fuzzy match positions for highlighting
    // bg_color: optional background color for the line (e.g., active Kind highlight)
    let make_line = |idx: usize,
                     cursor: usize,
                     focused: bool,
                     tree_prefix: &str,
                     icon: &str,
                     text: String,
                     line_color: Color,
                     text_color: Color,
                     match_positions: Option<&[u32]>,
                     bg_color: Option<Color>|
     -> Line {
        let is_cursor = idx == cursor;
        let cursor_char = if is_cursor { ">" } else { " " };
        let icon_space = if icon.is_empty() { "" } else { " " };

        if is_cursor && focused {
            // When focused/selected, use white on highlight bg for entire line
            let style = Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White);
            Line::from(Span::styled(
                format!(
                    "{}{}{}{}{}",
                    cursor_char, tree_prefix, icon, icon_space, text
                ),
                style,
            ))
        } else {
            // Split into spans: tree_prefix colored, text colored differently
            // Apply optional background color to all spans
            let base_style = if let Some(bg) = bg_color {
                Style::default().bg(bg)
            } else {
                Style::default()
            };
            let mut spans = Vec::with_capacity(6);
            spans.push(Span::styled(cursor_char, base_style));
            if !tree_prefix.is_empty() {
                spans.push(Span::styled(
                    tree_prefix.to_string(),
                    base_style.fg(line_color),
                ));
            }
            spans.push(Span::styled(
                format!("{}{}", icon, icon_space),
                base_style.fg(text_color),
            ));
            // Apply fuzzy match highlighting to text if positions provided
            spans.extend(highlight_matches_with_bg(
                &text,
                match_positions,
                text_color,
                bg_color,
            ));
            Line::from(spans)
        }
    };

    // Box-drawing helpers (using extracted pure functions)
    let branch = branch_char;
    let cont = cont_char;

    // === KINDS SECTION ===
    let kinds_collapsed = app.tree.is_collapsed("kinds");
    let kinds_icon = expand_icon(kinds_collapsed);
    let kinds_count: usize = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.kinds.len())
        .sum();
    all_lines.push(make_line(
        idx,
        app.tree_cursor,
        focused,
        "",
        kinds_icon,
        format!("Node Kinds ({})", kinds_count),
        Color::Magenta, // line_color (not used - no prefix)
        Color::Magenta, // text_color
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None, // bg_color
    ));
    idx += 1;

    let has_arcs = !app.tree.arc_families.is_empty();

    if !kinds_collapsed {
        let realm_count = app.tree.realms.len();
        for (ri, realm) in app.tree.realms.iter().enumerate() {
            let realm_is_last = ri == realm_count - 1 && !has_arcs;
            let realm_key = format!("realm:{}", realm.key);
            let realm_collapsed = app.tree.is_collapsed(&realm_key);
            let realm_icon = expand_icon(realm_collapsed);

            let realm_color = hex_to_color(&realm.color);
            all_lines.push(make_line(
                idx,
                app.tree_cursor,
                focused,
                branch(realm_is_last),
                realm_icon,
                format!("{} {}", realm.icon, realm.display_name),
                Color::Magenta, // line_color: parent section color
                realm_color,    // text_color
                app.search.matches.get(&idx).map(|v| v.as_slice()),
                None, // bg_color
            ));
            idx += 1;

            if !realm_collapsed {
                let is_data_mode = app.is_data_mode();
                let hide_empty = app.hide_empty && is_data_mode;

                // Filter visible layers (hide empty if hide_empty is true)
                let visible_layers: Vec<_> = realm
                    .layers
                    .iter()
                    .filter(|l| {
                        if hide_empty {
                            l.kinds.iter().map(|k| k.instance_count).sum::<i64>() > 0
                        } else {
                            true
                        }
                    })
                    .collect();
                let layer_count = visible_layers.len();

                for (li, layer) in visible_layers.iter().enumerate() {
                    let layer_is_last = li == layer_count - 1;
                    let layer_key = format!("layer:{}:{}", realm.key, layer.key);
                    let layer_collapsed = app.tree.is_collapsed(&layer_key);

                    // Calculate total instances in this layer
                    let layer_instance_count: i64 =
                        layer.kinds.iter().map(|k| k.instance_count).sum();
                    let layer_is_empty = layer_instance_count == 0;

                    // Show expand icon only if layer has content
                    let layer_icon = expand_icon(layer_collapsed);

                    // In Data mode: gray out empty layers, show instance count
                    let layer_color = hex_to_color(&layer.color);
                    let (display_name, text_color) = if is_data_mode {
                        let name = format!("{} ({})", layer.display_name, layer_instance_count);
                        let color = if layer_is_empty {
                            COLOR_MUTED_TEXT // Gray for empty layers
                        } else {
                            layer_color
                        };
                        (name, color)
                    } else {
                        (layer.display_name.clone(), layer_color)
                    };

                    let prefix = format!("{}{}", cont(realm_is_last), branch(layer_is_last));
                    all_lines.push(make_line(
                        idx,
                        app.tree_cursor,
                        focused,
                        &prefix,
                        layer_icon,
                        display_name,
                        realm_color, // line_color: parent realm color
                        text_color,  // text_color (grayed if empty in Data mode)
                        app.search.matches.get(&idx).map(|v| v.as_slice()),
                        None, // bg_color
                    ));
                    idx += 1;

                    if !layer_collapsed {
                        // Filter visible kinds (hide empty if hide_empty is true)
                        let visible_kinds: Vec<_> = layer
                            .kinds
                            .iter()
                            .filter(|k| {
                                if hide_empty {
                                    k.instance_count > 0
                                } else {
                                    true
                                }
                            })
                            .collect();
                        let kind_count = visible_kinds.len();

                        for (ki, kind) in visible_kinds.iter().enumerate() {
                            let kind_is_last = ki == kind_count - 1;
                            let kind_key_str = format!("kind:{}", kind.key);
                            let kind_collapsed = app.tree.is_collapsed(&kind_key_str);

                            // Show collapse icon in Data mode if instances exist
                            let kind_icon = if is_data_mode {
                                if let Some(instances) = app.tree.get_instances(&kind.key) {
                                    if !instances.is_empty() {
                                        expand_icon(kind_collapsed)
                                    } else {
                                        ""
                                    }
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            };

                            // v10.1: Show instance count (always in Data mode)
                            // v10.6: Add trait icon prefix
                            // QW7: Show arc count in Meta mode
                            // Feature 2: Health badges in Data mode
                            let kind_is_empty = kind.instance_count == 0;
                            let icon = trait_icon(&kind.trait_name);
                            let arc_count = kind.arcs.len();
                            let (display_text, kind_text_color) = if is_data_mode {
                                // Build health badge using extracted function
                                let health_badge =
                                    format_health_badge(kind.health_percent, kind.issues_count);
                                let text = format!(
                                    "{} {} ({}){}",
                                    icon, kind.display_name, kind.instance_count, health_badge
                                );
                                let color = if kind_is_empty {
                                    COLOR_MUTED_TEXT // Gray for empty kinds
                                } else {
                                    Color::White
                                };
                                (text, color)
                            } else {
                                // Meta mode: show arc count inline
                                let text = if arc_count > 0 {
                                    format!("{} {} ↔{}", icon, kind.display_name, arc_count)
                                } else {
                                    format!("{} {}", icon, kind.display_name)
                                };
                                (text, Color::White)
                            };

                            let prefix = format!(
                                "{}{}{}",
                                cont(realm_is_last),
                                cont(layer_is_last),
                                branch(kind_is_last)
                            );
                            // Highlight Kind if it has expanded instances (active focus)
                            let kind_has_expanded_instances = is_data_mode
                                && !kind_collapsed
                                && app
                                    .tree
                                    .get_instances(&kind.key)
                                    .is_some_and(|i| !i.is_empty());
                            let kind_bg = if kind_has_expanded_instances {
                                Some(COLOR_ACTIVE_KIND_BG)
                            } else {
                                None
                            };

                            all_lines.push(make_line(
                                idx,
                                app.tree_cursor,
                                focused,
                                &prefix,
                                kind_icon,
                                display_text,
                                layer_color,     // line_color: parent layer color
                                kind_text_color, // text_color (grayed if empty)
                                app.search.matches.get(&idx).map(|v| v.as_slice()),
                                kind_bg, // bg_color: highlight if instances expanded
                            ));
                            idx += 1;

                            // In Data mode, show instances under Kind (if not collapsed)
                            if is_data_mode && !kind_collapsed {
                                // Special case: Entity Kind shows categories instead of flat instances
                                if kind.key == "Entity" && !app.tree.entity_categories.is_empty() {
                                    let cat_count = app.tree.entity_categories.len();
                                    for (ci, category) in app.tree.entity_categories.iter().enumerate() {
                                        let cat_is_last = ci == cat_count - 1;
                                        let cat_key = format!("category:{}", category.key);
                                        let cat_collapsed = app.tree.is_collapsed(&cat_key);
                                        let cat_icon = expand_icon(cat_collapsed);
                                        let _is_cursor = idx == app.tree_cursor;

                                        let cat_prefix = format!(
                                            "{}{}{}{}",
                                            cont(realm_is_last),
                                            cont(layer_is_last),
                                            cont(kind_is_last),
                                            branch(cat_is_last)
                                        );

                                        // Category display: icon, name, and count
                                        let cat_display = format!(
                                            "📁 {} ({})",
                                            category.display_name,
                                            category.instance_count
                                        );

                                        all_lines.push(make_line(
                                            idx,
                                            app.tree_cursor,
                                            focused,
                                            &cat_prefix,
                                            cat_icon,
                                            cat_display,
                                            layer_color,   // line_color
                                            Color::Cyan,   // text_color for categories
                                            app.search.matches.get(&idx).map(|v| v.as_slice()),
                                            None,          // bg_color
                                        ));
                                        idx += 1;

                                        // Show instances under category if not collapsed
                                        if !cat_collapsed {
                                            if let Some(instances) = app.tree.entity_category_instances.get(&category.key) {
                                                let inst_count = instances.len();
                                                for (ii, instance) in instances.iter().enumerate() {
                                                    let inst_is_last = ii == inst_count - 1;
                                                    let is_cursor = idx == app.tree_cursor;

                                                    let style = if is_cursor && focused {
                                                        Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
                                                    } else {
                                                        Style::default().fg(COLOR_CONNECTED)
                                                    };

                                                    let cursor_char = if is_cursor { ">" } else { " " };

                                                    // Badge for missing required properties
                                                    let missing_badge =
                                                        format_missing_badge(instance.missing_required_count);

                                                    // Arc count badge
                                                    let arc_badge = format_arc_badge(
                                                        instance.outgoing_arcs.len(),
                                                        instance.incoming_arcs.len(),
                                                    );

                                                    // Completeness bar
                                                    let completeness_badge = format_completeness_badge(
                                                        instance.filled_properties,
                                                        instance.total_properties,
                                                    );

                                                    let tree_prefix = format!(
                                                        "{}{}{}{}{}",
                                                        cont(realm_is_last),
                                                        cont(layer_is_last),
                                                        cont(kind_is_last),
                                                        cont(cat_is_last),
                                                        branch(inst_is_last)
                                                    );

                                                    if is_cursor && focused {
                                                        all_lines.push(Line::from(Span::styled(
                                                            format!(
                                                                "{}{}○ {}{}{}{}",
                                                                cursor_char,
                                                                tree_prefix,
                                                                instance.display_name,
                                                                completeness_badge,
                                                                arc_badge,
                                                                missing_badge
                                                            ),
                                                            style,
                                                        )));
                                                    } else {
                                                        let mut spans = vec![
                                                            Span::styled(cursor_char, Style::default()),
                                                            Span::styled(
                                                                tree_prefix,
                                                                Style::default().fg(layer_color),
                                                            ),
                                                            Span::styled(
                                                                format!("○ {}", instance.display_name),
                                                                style,
                                                            ),
                                                        ];
                                                        if !completeness_badge.is_empty() {
                                                            let color = if instance.filled_properties
                                                                == instance.total_properties
                                                            {
                                                                Color::Green
                                                            } else if instance.filled_properties
                                                                > instance.total_properties / 2
                                                            {
                                                                Color::Yellow
                                                            } else {
                                                                Color::Red
                                                            };
                                                            spans.push(Span::styled(
                                                                completeness_badge,
                                                                Style::default().fg(color),
                                                            ));
                                                        }
                                                        if !arc_badge.is_empty() {
                                                            spans.push(Span::styled(
                                                                arc_badge,
                                                                Style::default().fg(Color::Cyan),
                                                            ));
                                                        }
                                                        if !missing_badge.is_empty() {
                                                            spans.push(Span::styled(
                                                                missing_badge,
                                                                Style::default().fg(Color::Red),
                                                            ));
                                                        }
                                                        all_lines.push(Line::from(spans));
                                                    }
                                                    idx += 1;
                                                }
                                            }
                                        }
                                    }
                                } else if let Some(instances) = app.tree.get_instances(&kind.key) {
                                    // Regular kinds: show instances directly
                                    let inst_count = instances.len();
                                    for (ii, instance) in instances.iter().enumerate() {
                                        let inst_is_last = ii == inst_count - 1;
                                        let is_cursor = idx == app.tree_cursor;

                                        // Check if primary (for Locale kind)
                                        let is_primary = instance
                                            .properties
                                            .get("is_primary")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(false);

                                        // Count incoming FALLBACK_TO
                                        let fallback_count = if is_primary {
                                            instance
                                                .incoming_arcs
                                                .iter()
                                                .filter(|a| a.arc_type == "FALLBACK_TO")
                                                .count()
                                        } else {
                                            0
                                        };

                                        let (icon, base_color) = if is_primary {
                                            ("●", Color::Yellow)
                                        } else {
                                            ("○", COLOR_CONNECTED)
                                        };

                                        let style = if is_cursor && focused {
                                            Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
                                        } else {
                                            Style::default().fg(base_color)
                                        };

                                        let cursor_char = if is_cursor { ">" } else { " " };
                                        let suffix = if is_primary && fallback_count > 0 {
                                            format!(" [{}↓]", fallback_count)
                                        } else {
                                            String::new()
                                        };

                                        // Badge for missing required properties: (✗N!)
                                        let missing_badge =
                                            format_missing_badge(instance.missing_required_count);

                                        // Arc count badge: [->N <-M] (only if has arcs)
                                        let arc_badge = format_arc_badge(
                                            instance.outgoing_arcs.len(),
                                            instance.incoming_arcs.len(),
                                        );

                                        // Completeness bar: [==--] only shown if incomplete
                                        let completeness_badge = format_completeness_badge(
                                            instance.filled_properties,
                                            instance.total_properties,
                                        );

                                        let tree_prefix = format!(
                                            "{}{}{}{}",
                                            cont(realm_is_last),
                                            cont(layer_is_last),
                                            cont(kind_is_last),
                                            branch(inst_is_last)
                                        );

                                        if is_cursor && focused {
                                            // Selected: single span with highlight bg
                                            all_lines.push(Line::from(Span::styled(
                                                format!(
                                                    "{}{}{} {}{}{}{}{}",
                                                    cursor_char,
                                                    tree_prefix,
                                                    icon,
                                                    instance.display_name,
                                                    suffix,
                                                    completeness_badge,
                                                    arc_badge,
                                                    missing_badge
                                                ),
                                                style,
                                            )));
                                        } else {
                                            // Not selected: split into spans for colored badges
                                            let mut spans = vec![
                                                Span::styled(cursor_char, Style::default()),
                                                Span::styled(
                                                    tree_prefix,
                                                    Style::default().fg(layer_color),
                                                ),
                                                Span::styled(
                                                    format!(
                                                        "{} {}{}",
                                                        icon, instance.display_name, suffix
                                                    ),
                                                    style,
                                                ),
                                            ];
                                            // Completeness bar (green gradient based on fill)
                                            if !completeness_badge.is_empty() {
                                                let color = if instance.filled_properties
                                                    == instance.total_properties
                                                {
                                                    Color::Green
                                                } else if instance.filled_properties
                                                    > instance.total_properties / 2
                                                {
                                                    Color::Yellow
                                                } else {
                                                    Color::Red
                                                };
                                                spans.push(Span::styled(
                                                    completeness_badge,
                                                    Style::default().fg(color),
                                                ));
                                            }
                                            // Arc count (cyan)
                                            if !arc_badge.is_empty() {
                                                spans.push(Span::styled(
                                                    arc_badge,
                                                    Style::default().fg(Color::Cyan),
                                                ));
                                            }
                                            // Missing required (red)
                                            if !missing_badge.is_empty() {
                                                spans.push(Span::styled(
                                                    missing_badge,
                                                    Style::default().fg(Color::Red),
                                                ));
                                            }
                                            all_lines.push(Line::from(spans));
                                        }
                                        idx += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // === RELATIONS SECTION ===
    let arcs_collapsed = app.tree.is_collapsed("arcs");
    let arcs_icon = expand_icon(arcs_collapsed);
    let arcs_count: usize = app
        .tree
        .arc_families
        .iter()
        .map(|f| f.arc_kinds.len())
        .sum();
    all_lines.push(make_line(
        idx,
        app.tree_cursor,
        focused,
        "",
        arcs_icon,
        format!("Arcs ({})", arcs_count),
        Color::Yellow, // line_color (not used - no prefix)
        Color::Yellow, // text_color
        app.search.matches.get(&idx).map(|v| v.as_slice()),
        None, // bg_color
    ));
    idx += 1;

    if !arcs_collapsed {
        let family_count = app.tree.arc_families.len();
        for (fi, family) in app.tree.arc_families.iter().enumerate() {
            let family_is_last = fi == family_count - 1;
            let family_key = format!("family:{}", family.key);
            let family_collapsed = app.tree.is_collapsed(&family_key);
            let family_icon = expand_icon(family_collapsed);

            all_lines.push(make_line(
                idx,
                app.tree_cursor,
                focused,
                branch(family_is_last),
                family_icon,
                format!("{} ({})", family.display_name, family.arc_kinds.len()),
                Color::Yellow,    // line_color: parent section color
                COLOR_ARC_FAMILY, // text_color
                app.search.matches.get(&idx).map(|v| v.as_slice()),
                None, // bg_color
            ));
            idx += 1;

            if !family_collapsed {
                let arc_count = family.arc_kinds.len();
                for (ai, arc_kind) in family.arc_kinds.iter().enumerate() {
                    let arc_is_last = ai == arc_count - 1;
                    let prefix = format!("{}{}", cont(family_is_last), branch(arc_is_last));
                    all_lines.push(make_line(
                        idx,
                        app.tree_cursor,
                        focused,
                        &prefix,
                        "",
                        arc_kind.display_name.clone(),
                        COLOR_ARC_FAMILY, // line_color: parent family color
                        COLOR_DESC_TEXT,  // text_color
                        app.search.matches.get(&idx).map(|v| v.as_slice()),
                        None, // bg_color
                    ));
                    idx += 1;
                }
            }
        }
    }

    // Apply scroll - only show visible lines
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.tree_scroll)
        .take(visible_height)
        .collect();

    // Handle empty tree
    let lines = if lines.is_empty() {
        vec![Line::from(Span::styled("  No data loaded", STYLE_DIM))]
    } else {
        lines
    };

    // Show scroll indicator in title (use mode-aware count for Data view)
    let total = app.current_item_count();
    let title = if total > visible_height {
        format!(
            " Taxonomy [{}-{}/{}] ",
            app.tree_scroll + 1,
            (app.tree_scroll + visible_height).min(total),
            total
        )
    } else {
        " Taxonomy ".to_string()
    };

    // Breadcrumb as bottom title (truncate if too long, UTF-8 safe)
    let breadcrumb = app.current_breadcrumb();
    let max_breadcrumb_len = area.width.saturating_sub(4) as usize;
    let breadcrumb_display = truncate_start(&breadcrumb, max_breadcrumb_len);

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .title_bottom(Line::from(Span::styled(
            format!(" {} ", breadcrumb_display),
            Style::default().fg(COLOR_HINT_TEXT),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);

    // Add scrollbar if content exceeds visible area
    if total > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state =
            ScrollbarState::new(total.saturating_sub(visible_height)).position(app.tree_scroll);

        let scrollbar_area = Rect {
            x: area.x + area.width.saturating_sub(2),
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };
        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Render filtered Data mode: only instances of a specific Kind with breadcrumb.
fn render_filtered_instances(
    f: &mut Frame,
    area: Rect,
    app: &App,
    kind_key: &str,
    visible_height: usize,
    focused: bool,
    border_color: Color,
) {
    // Get Kind info for display with full hierarchy
    let kind_info = app.tree.find_kind(kind_key);
    let (realm_display, realm_color, layer_display, layer_color, kind_display) = kind_info
        .map(|(realm, layer, kind)| {
            (
                realm.display_name.clone(),
                hex_to_color(&realm.color),
                layer.display_name.clone(),
                hex_to_color(&layer.color),
                kind.display_name.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown".to_string(),
                Color::White,
                "Unknown".to_string(),
                Color::White,
                kind_key.to_string(),
            )
        });

    // Build lines: breadcrumb + instances
    let mut all_lines: Vec<Line> = Vec::new();

    // Breadcrumb header with full hierarchy: Realm → Layer → Kind
    all_lines.push(Line::from(vec![
        Span::styled("← ", STYLE_DIM),
        Span::styled("Esc", STYLE_HIGHLIGHT),
        Span::styled(" │ ", STYLE_DIM),
        Span::styled(&realm_display, Style::default().fg(realm_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&layer_display, Style::default().fg(layer_color)),
        Span::styled(" → ", STYLE_DIM),
        Span::styled(&kind_display, STYLE_PRIMARY),
    ]));
    all_lines.push(Line::from(Span::styled(
        "─".repeat(area.width.saturating_sub(2) as usize),
        STYLE_UNFOCUSED,
    )));

    // Get instances and total count
    let instances = app.tree.get_instances(kind_key);
    let instance_count = instances.map(|i| i.len()).unwrap_or(0);
    let total_count = app
        .tree
        .get_instance_total(kind_key)
        .unwrap_or(instance_count);
    let is_truncated = total_count > instance_count;
    let is_loading = app
        .pending_instance_load
        .as_ref()
        .is_some_and(|k| k == kind_key);

    if instance_count == 0 {
        if is_loading {
            // Still loading from Neo4j (animated spinner)
            all_lines.push(Line::from(Span::styled(
                format!("  {} Loading instances...", spinner(app.tick)),
                STYLE_HIGHLIGHT,
            )));
        } else {
            // Loaded but empty
            all_lines.push(Line::from(Span::styled(
                "  No instances exist for this Kind",
                STYLE_DIM,
            )));
        }
    } else if let Some(instances) = instances {
        for (idx, instance) in instances.iter().enumerate() {
            let is_cursor = idx == app.tree_cursor;

            // Check if this is a primary locale (is_primary: true)
            let is_primary = instance
                .properties
                .get("is_primary")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            // Count incoming FALLBACK_TO arcs for primary locales
            let fallback_count = if is_primary {
                instance
                    .incoming_arcs
                    .iter()
                    .filter(|arc| arc.arc_type == "FALLBACK_TO")
                    .count()
            } else {
                0
            };

            // Primary locales: filled circle, yellow
            // Secondary locales: empty circle, green
            let (icon, base_color) = if is_primary {
                ("●", Color::Yellow)
            } else {
                ("○", COLOR_CONNECTED)
            };

            let style = if is_cursor && focused {
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White)
            } else {
                Style::default().fg(base_color)
            };

            let prefix = if is_cursor { "› " } else { "  " };

            // Format: "● Arabic (Saudi Arabia) [13↓]" for primary
            // Format: "○ Arabic (Algeria)" for secondary
            let display = if is_primary && fallback_count > 0 {
                format!(
                    "{}{} {} [{}↓]",
                    prefix, icon, instance.display_name, fallback_count
                )
            } else {
                format!("{}{} {}", prefix, icon, instance.display_name)
            };

            all_lines.push(Line::from(Span::styled(display, style)));
        }
    }

    // Apply scroll
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.tree_scroll)
        .take(visible_height)
        .collect();

    // Title with Kind name and count + position indicator
    // Format: "Locale (3/203)" when all loaded, "Locale (3/500 of 847)" when truncated
    let title = if instance_count > 0 {
        if is_truncated {
            format!(
                " {} ({}/{} of {}) ",
                kind_display,
                app.tree_cursor + 1,
                instance_count,
                total_count
            )
        } else {
            format!(
                " {} ({}/{}) ",
                kind_display,
                app.tree_cursor + 1,
                instance_count
            )
        }
    } else {
        format!(" {} (0) ", kind_display)
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(layer_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

// =============================================================================
// PURE HELPER FUNCTIONS (extracted for testability)
// =============================================================================

/// Get the branch character for tree drawing.
/// - `└─` for last item (no more siblings)
/// - `├─` for non-last item (more siblings below)
#[inline]
pub(super) fn branch_char(is_last: bool) -> &'static str {
    if is_last { "└─" } else { "├─" }
}

/// Get the continuation character for tree drawing.
/// - `  ` (two spaces) if parent was last (no vertical line needed)
/// - `│ ` if parent was not last (vertical line continues)
#[inline]
pub(super) fn cont_char(parent_is_last: bool) -> &'static str {
    if parent_is_last { "  " } else { "│ " }
}

/// Get the expand/collapse icon for a tree node.
/// - `▶` when collapsed (pointing right, can expand)
/// - `▼` when expanded (pointing down, can collapse)
#[inline]
pub(super) fn expand_icon(is_collapsed: bool) -> &'static str {
    if is_collapsed { "▶" } else { "▼" }
}

/// Build a tree prefix string for a given depth and position.
///
/// # Arguments
/// * `parent_is_last` - Slice of booleans indicating whether each ancestor was the last child
/// * `is_last` - Whether this node is the last child at its level
///
/// # Returns
/// A string like "│ │ └─" for drawing tree structure
///
/// NOTE: Currently used only in tests to verify tree prefix logic.
/// Future refactoring could use this in render_tree to replace inline format!() calls.
#[allow(dead_code)]
pub(super) fn build_tree_prefix(parent_is_last: &[bool], is_last: bool) -> String {
    let mut prefix = String::with_capacity(parent_is_last.len() * 3 + 3);
    for &was_last in parent_is_last {
        prefix.push_str(cont_char(was_last));
    }
    prefix.push_str(branch_char(is_last));
    prefix
}

/// Format a health badge for a Kind node.
/// Returns empty string if no health data, or a bar like " ━━━░░░░░░░50%"
pub(super) fn format_health_badge(health_percent: Option<u8>, issues_count: Option<usize>) -> String {
    let Some(percent) = health_percent else {
        return String::new();
    };
    let filled = percent / 10;
    let empty = 10 - filled;
    let issues = issues_count.unwrap_or(0);
    if issues > 0 {
        format!(
            " {}{}{}% ⚠{}",
            "━".repeat(filled as usize),
            "░".repeat(empty as usize),
            percent,
            issues
        )
    } else {
        format!(
            " {}{}{}%",
            "━".repeat(filled as usize),
            "░".repeat(empty as usize),
            percent
        )
    }
}

/// Format a completeness badge for an instance.
/// Returns empty string if 100% complete, or "[==--]" style bar if incomplete.
pub(super) fn format_completeness_badge(filled: usize, total: usize) -> String {
    if total == 0 {
        return String::new();
    }
    if filled >= total {
        // 100% complete - hide badge
        return String::new();
    }
    let ratio = filled as f32 / total as f32;
    let filled_chars = (ratio * 4.0).round() as usize;
    let empty_chars = 4 - filled_chars;
    format!(
        " [{}{}]",
        "=".repeat(filled_chars),
        "-".repeat(empty_chars)
    )
}

/// Format an arc count badge for an instance.
/// Returns empty string if no arcs, or "[->N|<-M]" if has arcs.
pub(super) fn format_arc_badge(outgoing: usize, incoming: usize) -> String {
    if outgoing == 0 && incoming == 0 {
        String::new()
    } else {
        format!(" [->{}|<-{}]", outgoing, incoming)
    }
}

/// Format a missing required properties badge.
/// Returns empty string if none missing, or " (✗N!)" if missing.
pub(super) fn format_missing_badge(missing_count: usize) -> String {
    if missing_count > 0 {
        format!(" (✗{}!)", missing_count)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // Tree structure helpers tests
    // =============================================================================

    #[test]
    fn test_branch_char_last() {
        assert_eq!(branch_char(true), "└─");
    }

    #[test]
    fn test_branch_char_not_last() {
        assert_eq!(branch_char(false), "├─");
    }

    #[test]
    fn test_cont_char_parent_was_last() {
        assert_eq!(cont_char(true), "  ");
    }

    #[test]
    fn test_cont_char_parent_was_not_last() {
        assert_eq!(cont_char(false), "│ ");
    }

    #[test]
    fn test_expand_icon_collapsed() {
        assert_eq!(expand_icon(true), "▶");
    }

    #[test]
    fn test_expand_icon_expanded() {
        assert_eq!(expand_icon(false), "▼");
    }

    // =============================================================================
    // Tree prefix building tests
    // =============================================================================

    #[test]
    fn test_build_tree_prefix_root_level() {
        // First level item (no parents), last child
        assert_eq!(build_tree_prefix(&[], true), "└─");
        // First level item, not last
        assert_eq!(build_tree_prefix(&[], false), "├─");
    }

    #[test]
    fn test_build_tree_prefix_second_level() {
        // Second level, parent was not last, this is last
        assert_eq!(build_tree_prefix(&[false], true), "│ └─");
        // Second level, parent was not last, this is not last
        assert_eq!(build_tree_prefix(&[false], false), "│ ├─");
        // Second level, parent was last, this is last
        assert_eq!(build_tree_prefix(&[true], true), "  └─");
        // Second level, parent was last, this is not last
        assert_eq!(build_tree_prefix(&[true], false), "  ├─");
    }

    #[test]
    fn test_build_tree_prefix_third_level() {
        // Third level: grandparent not last, parent not last, this is last
        assert_eq!(build_tree_prefix(&[false, false], true), "│ │ └─");
        // Third level: grandparent last, parent not last, this is not last
        assert_eq!(build_tree_prefix(&[true, false], false), "  │ ├─");
        // Third level: all were last
        assert_eq!(build_tree_prefix(&[true, true], true), "    └─");
    }

    #[test]
    fn test_build_tree_prefix_deep_nesting() {
        // Deep nesting with mixed last states
        let prefix = build_tree_prefix(&[false, true, false, true], false);
        assert_eq!(prefix, "│   │   ├─");
    }

    // =============================================================================
    // Health badge tests
    // =============================================================================

    #[test]
    fn test_format_health_badge_none() {
        assert_eq!(format_health_badge(None, None), "");
    }

    #[test]
    fn test_format_health_badge_zero_percent() {
        let badge = format_health_badge(Some(0), None);
        assert!(badge.contains("0%"));
        assert!(badge.contains("░░░░░░░░░░")); // 10 empty chars
    }

    #[test]
    fn test_format_health_badge_fifty_percent() {
        let badge = format_health_badge(Some(50), None);
        assert!(badge.contains("50%"));
        assert!(badge.contains("━━━━━")); // 5 filled chars
        assert!(badge.contains("░░░░░")); // 5 empty chars
    }

    #[test]
    fn test_format_health_badge_hundred_percent() {
        let badge = format_health_badge(Some(100), None);
        assert!(badge.contains("100%"));
        assert!(badge.contains("━━━━━━━━━━")); // 10 filled chars
    }

    #[test]
    fn test_format_health_badge_with_issues() {
        let badge = format_health_badge(Some(70), Some(3));
        assert!(badge.contains("70%"));
        assert!(badge.contains("⚠3"));
    }

    #[test]
    fn test_format_health_badge_with_zero_issues() {
        let badge = format_health_badge(Some(80), Some(0));
        assert!(badge.contains("80%"));
        assert!(!badge.contains("⚠"));
    }

    // =============================================================================
    // Completeness badge tests
    // =============================================================================

    #[test]
    fn test_format_completeness_badge_empty() {
        assert_eq!(format_completeness_badge(0, 0), "");
    }

    #[test]
    fn test_format_completeness_badge_complete() {
        // 100% complete - should hide badge
        assert_eq!(format_completeness_badge(10, 10), "");
        assert_eq!(format_completeness_badge(5, 5), "");
    }

    #[test]
    fn test_format_completeness_badge_zero_filled() {
        let badge = format_completeness_badge(0, 8);
        assert_eq!(badge, " [----]"); // 0/8 = 0%, rounds to 0 filled
    }

    #[test]
    fn test_format_completeness_badge_half_filled() {
        let badge = format_completeness_badge(4, 8);
        assert_eq!(badge, " [==--]"); // 50% = 2 filled chars
    }

    #[test]
    fn test_format_completeness_badge_three_quarters() {
        let badge = format_completeness_badge(6, 8);
        // 6/8 = 75% -> 0.75 * 4 = 3 filled chars
        assert_eq!(badge, " [===-]");
    }

    #[test]
    fn test_format_completeness_badge_almost_complete() {
        // 7/8 = 87.5% rounds to 4 filled chars (but badge shown since not 100%)
        let badge = format_completeness_badge(7, 8);
        assert_eq!(badge, " [====]");

        // 9/10 = 90% also rounds to 4 filled chars
        let badge2 = format_completeness_badge(9, 10);
        assert_eq!(badge2, " [====]");
    }

    #[test]
    fn test_format_completeness_badge_low_fill() {
        // 1/8 = 12.5% -> 0.5 rounds to 1
        let badge = format_completeness_badge(1, 8);
        assert_eq!(badge, " [=---]");

        // 2/8 = 25% -> 1 filled char
        let badge2 = format_completeness_badge(2, 8);
        assert_eq!(badge2, " [=---]");
    }

    // =============================================================================
    // Arc badge tests
    // =============================================================================

    #[test]
    fn test_format_arc_badge_no_arcs() {
        assert_eq!(format_arc_badge(0, 0), "");
    }

    #[test]
    fn test_format_arc_badge_outgoing_only() {
        assert_eq!(format_arc_badge(5, 0), " [->5|<-0]");
    }

    #[test]
    fn test_format_arc_badge_incoming_only() {
        assert_eq!(format_arc_badge(0, 3), " [->0|<-3]");
    }

    #[test]
    fn test_format_arc_badge_both() {
        assert_eq!(format_arc_badge(2, 7), " [->2|<-7]");
    }

    // =============================================================================
    // Missing badge tests
    // =============================================================================

    #[test]
    fn test_format_missing_badge_none() {
        assert_eq!(format_missing_badge(0), "");
    }

    #[test]
    fn test_format_missing_badge_one() {
        assert_eq!(format_missing_badge(1), " (✗1!)");
    }

    #[test]
    fn test_format_missing_badge_many() {
        assert_eq!(format_missing_badge(5), " (✗5!)");
    }

    // =============================================================================
    // Highlight matches tests (fuzzy search highlighting)
    // =============================================================================

    #[test]
    fn test_highlight_matches_no_positions() {
        let spans = highlight_matches_with_bg("hello", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        // Check that the text is correct
        let span = &spans[0];
        assert_eq!(span.content, "hello");
    }

    #[test]
    fn test_highlight_matches_empty_positions() {
        let spans = highlight_matches_with_bg("hello", Some(&[]), Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hello");
    }

    #[test]
    fn test_highlight_matches_single_char() {
        // Match positions: 0 (first char 'h')
        let spans = highlight_matches_with_bg("hello", Some(&[0]), Color::White, None);
        // Should be: [highlighted 'h'], [normal 'ello']
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "h");
        assert_eq!(spans[1].content, "ello");
    }

    #[test]
    fn test_highlight_matches_consecutive() {
        // Match positions: 0, 1, 2 ('hel')
        let spans = highlight_matches_with_bg("hello", Some(&[0, 1, 2]), Color::White, None);
        // Should be: [highlighted 'hel'], [normal 'lo']
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "hel");
        assert_eq!(spans[1].content, "lo");
    }

    #[test]
    fn test_highlight_matches_scattered() {
        // Match positions: 0, 2, 4 ('h', 'l', 'o')
        let spans = highlight_matches_with_bg("hello", Some(&[0, 2, 4]), Color::White, None);
        // Should produce alternating highlighted/normal spans
        // [h], [e], [l], [l], [o] with h, l, o highlighted
        // Merged: [h(hl)], [e(norm)], [l(hl)], [l(norm)], [o(hl)]
        assert!(spans.len() >= 3);
    }

    #[test]
    fn test_highlight_matches_full_match() {
        // All chars match
        let spans = highlight_matches_with_bg("hi", Some(&[0, 1]), Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hi");
    }

    // Task 5.1: Additional explicit tests for highlight_matches
    #[test]
    fn test_highlight_matches_no_match() {
        // When matches is None, return single span with base style
        let spans = highlight_matches_with_bg("hello", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hello");
        // Verify base color is applied
        assert_eq!(spans[0].style.fg, Some(Color::White));
        assert_eq!(spans[0].style.bg, None);
    }

    #[test]
    fn test_highlight_matches_with_positions() {
        // Match positions 0 and 2: 'h' and 'l' in "hello"
        let spans = highlight_matches_with_bg("hello", Some(&[0, 2]), Color::White, None);
        // Expected: [h(yellow)], [e(white)], [l(yellow)], [lo(white)]
        assert!(
            spans.len() >= 3,
            "Expected at least 3 spans, got {}",
            spans.len()
        );

        // First span should be 'h' highlighted (yellow bg)
        assert_eq!(spans[0].content, "h");
        assert_eq!(spans[0].style.bg, Some(Color::Yellow));
        assert_eq!(spans[0].style.fg, Some(Color::Black));

        // Second span should be 'e' with base color
        assert_eq!(spans[1].content, "e");
        assert_eq!(spans[1].style.fg, Some(Color::White));

        // Third span should be 'l' highlighted
        assert_eq!(spans[2].content, "l");
        assert_eq!(spans[2].style.bg, Some(Color::Yellow));
        assert_eq!(spans[2].style.fg, Some(Color::Black));

        // Fourth span should be 'lo' with base color
        assert_eq!(spans[3].content, "lo");
        assert_eq!(spans[3].style.fg, Some(Color::White));
    }
}
