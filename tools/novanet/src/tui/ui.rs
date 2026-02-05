//! UI rendering for TUI v2.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::app::{App, Focus, NavMode};
use super::data::{ArcDirection, TreeItem};
use super::theme::{self, hex_to_color};

/// Safely truncate a UTF-8 string to N characters (not bytes).
/// Appends "..." if truncated.
fn truncate_str(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_chars {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}...", truncated)
    } else {
        s.to_string()
    }
}

/// Main render function.
pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_main(f, chunks[1], app);
    render_status(f, chunks[2], app);

    // Overlays on top (order matters: last = topmost)
    if app.search_active {
        render_search(f, app);
    }
    if app.help_active {
        render_help(f);
    }
}

/// Header: Logo + Mode tabs.
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs: Vec<Span> = [
        NavMode::Meta,
        NavMode::Data,
        NavMode::Overlay,
        NavMode::Query,
    ]
    .iter()
    .enumerate()
    .map(|(i, mode)| {
        let num = format!("[{}]", i + 1);
        let label = mode.label();
        let is_active = *mode == app.mode;

        if is_active {
            Span::styled(
                format!(" {}{}\u{2022} ", num, label),
                theme::ui::focused_style(),
            )
        } else {
            Span::styled(
                format!(" {}{} ", num, label),
                Style::default().fg(Color::DarkGray),
            )
        }
    })
    .collect();

    let mut header: Vec<Span> = vec![
        Span::styled(" NovaNet ", theme::ui::logo_style()),
        Span::raw("          "),
    ];
    header.extend(tabs);

    let right_side = vec![Span::styled(
        "  h/l:toggle  jk:scroll  []:yaml  Tab:panel  f:find  /:help  q:quit",
        theme::ui::muted_style(),
    )];

    let mut full_header: Vec<Span<'static>> = header;
    // Calculate padding to right-align
    let header_len: usize = full_header.iter().map(|s| s.content.len()).sum();
    let right_len: usize = right_side.iter().map(|s| s.content.len()).sum();
    let padding = area
        .width
        .saturating_sub(header_len as u16 + right_len as u16);
    full_header.push(Span::raw(" ".repeat(padding as usize)));
    full_header.extend(right_side);

    let paragraph =
        Paragraph::new(Line::from(full_header)).style(Style::default().bg(theme::ui::HEADER_BG));

    f.render_widget(paragraph, area);
}

/// Layout mode based on terminal width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutMode {
    Wide,   // 3 columns: Tree | Info | YAML
    Narrow, // 2 columns: Tree | (Info / YAML stacked)
}

impl LayoutMode {
    fn detect(width: u16) -> Self {
        if width >= 160 {
            LayoutMode::Wide
        } else {
            LayoutMode::Narrow
        }
    }
}

/// Main content: responsive layout based on terminal width.
fn render_main(f: &mut Frame, area: Rect, app: &mut App) {
    let layout_mode = LayoutMode::detect(area.width);

    match layout_mode {
        LayoutMode::Wide => render_main_wide(f, area, app),
        LayoutMode::Narrow => render_main_narrow(f, area, app),
    }
}

/// Wide layout: Tree (15%) | Info+Graph (42.5%) | YAML (42.5%).
fn render_main_wide(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15), // Tree (narrower)
            Constraint::Percentage(43), // Info + Graph (stacked)
            Constraint::Percentage(42), // YAML
        ])
        .split(area);

    render_tree(f, chunks[0], app);

    // Stack Info (60%) and Graph (40%) vertically in the middle panel
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Info
            Constraint::Percentage(40), // Graph
        ])
        .split(chunks[1]);

    render_info_panel(f, middle_chunks[0], app);
    render_graph_panel(f, middle_chunks[1], app);

    render_yaml_panel(f, chunks[2], app);
}

/// Narrow layout: Tree (20%) | Info+Graph+YAML stacked (80%).
fn render_main_narrow(f: &mut Frame, area: Rect, app: &mut App) {
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Tree (narrower)
            Constraint::Percentage(80), // Detail (stacked)
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    // Stack Info (35%), Graph (30%), YAML (35%) vertically
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35), // Info
            Constraint::Percentage(30), // Graph
            Constraint::Percentage(35), // YAML
        ])
        .split(h_chunks[1]);

    render_info_panel(f, v_chunks[0], app);
    render_graph_panel(f, v_chunks[1], app);
    render_yaml_panel(f, v_chunks[2], app);
}

/// Tree panel: taxonomy hierarchy with scroll and collapse.
fn render_tree(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Tree;
    let border_color = if focused {
        Color::Cyan
    } else {
        Color::Rgb(60, 60, 70)
    };

    // Calculate visible height (area minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    app.tree_height = visible_height;

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

    // Helper to create a line
    let make_line = |idx: usize,
                     cursor: usize,
                     focused: bool,
                     indent: &str,
                     icon: &str,
                     text: String,
                     color: Color|
     -> Line {
        let is_cursor = idx == cursor;
        let style = if is_cursor && focused {
            Style::default().bg(Color::Rgb(30, 40, 50)).fg(Color::White)
        } else {
            Style::default().fg(color)
        };
        let prefix = if is_cursor { "›" } else { " " };
        Line::from(Span::styled(
            format!("{}{}{} {}", prefix, indent, icon, text),
            style,
        ))
    };

    // === KINDS SECTION ===
    let kinds_collapsed = app.tree.is_collapsed("kinds");
    let kinds_icon = if kinds_collapsed { "▶" } else { "▼" };
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
        Color::Magenta,
    ));
    idx += 1;

    if !kinds_collapsed {
        for realm in &app.tree.realms {
            let realm_key = format!("realm:{}", realm.key);
            let realm_collapsed = app.tree.is_collapsed(&realm_key);
            let realm_icon = if realm_collapsed { "▶" } else { "▼" };
            all_lines.push(make_line(
                idx,
                app.tree_cursor,
                focused,
                "  ",
                realm_icon,
                format!("{} {}", realm.emoji, realm.display_name),
                hex_to_color(&realm.color),
            ));
            idx += 1;

            if !realm_collapsed {
                for layer in &realm.layers {
                    let layer_key = format!("layer:{}", layer.key);
                    let layer_collapsed = app.tree.is_collapsed(&layer_key);
                    let layer_icon = if layer_collapsed { "▶" } else { "▼" };
                    all_lines.push(make_line(
                        idx,
                        app.tree_cursor,
                        focused,
                        "    ",
                        layer_icon,
                        layer.display_name.clone(),
                        hex_to_color(&layer.color),
                    ));
                    idx += 1;

                    if !layer_collapsed {
                        let is_data_mode = app.is_data_mode();

                        for kind in &layer.kinds {
                            // In Data mode, Kind can be collapsed to hide instances
                            let kind_key = format!("kind:{}", kind.key);
                            let kind_collapsed = app.tree.is_collapsed(&kind_key);

                            // Show collapse icon in Data mode if instances exist
                            let kind_icon = if is_data_mode {
                                if let Some(instances) = app.tree.get_instances(&kind.key) {
                                    if !instances.is_empty() {
                                        if kind_collapsed { "▶" } else { "▼" }
                                    } else {
                                        ""
                                    }
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            };

                            // v10.1: Show instance count
                            let count = if kind.instance_count > 0 {
                                format!(" ({})", kind.instance_count)
                            } else {
                                String::new()
                            };

                            all_lines.push(make_line(
                                idx,
                                app.tree_cursor,
                                focused,
                                "      ",
                                kind_icon,
                                format!("{}{}", kind.display_name, count),
                                Color::White,
                            ));
                            idx += 1;

                            // In Data mode, show instances under Kind (if not collapsed)
                            if is_data_mode && !kind_collapsed {
                                if let Some(instances) = app.tree.get_instances(&kind.key) {
                                    for instance in instances {
                                        let is_cursor = idx == app.tree_cursor;
                                        let style = if is_cursor && focused {
                                            Style::default()
                                                .bg(Color::Rgb(30, 40, 50))
                                                .fg(Color::Green)
                                        } else {
                                            Style::default().fg(Color::Rgb(100, 180, 100))
                                        };
                                        let prefix = if is_cursor { "›" } else { " " };
                                        all_lines.push(Line::from(Span::styled(
                                            format!(
                                                "{}        • {}",
                                                prefix, instance.display_name
                                            ),
                                            style,
                                        )));
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
    let arcs_icon = if arcs_collapsed { "▶" } else { "▼" };
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
        Color::Yellow,
    ));
    idx += 1;

    if !arcs_collapsed {
        for family in &app.tree.arc_families {
            let family_key = format!("family:{}", family.key);
            let family_collapsed = app.tree.is_collapsed(&family_key);
            let family_icon = if family_collapsed { "▶" } else { "▼" };
            all_lines.push(make_line(
                idx,
                app.tree_cursor,
                focused,
                "  ",
                family_icon,
                format!("{} ({})", family.display_name, family.arc_kinds.len()),
                Color::Rgb(180, 140, 80),
            ));
            idx += 1;

            if !family_collapsed {
                for arc_kind in &family.arc_kinds {
                    all_lines.push(make_line(
                        idx,
                        app.tree_cursor,
                        focused,
                        "    ",
                        "",
                        arc_kind.display_name.clone(),
                        Color::Rgb(150, 150, 150),
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
        vec![Line::from(Span::styled(
            "  No data loaded",
            Style::default().fg(Color::DarkGray),
        ))]
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

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
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
        Span::styled("← ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(&realm_display, Style::default().fg(realm_color)),
        Span::styled(" → ", Style::default().fg(Color::DarkGray)),
        Span::styled(&layer_display, Style::default().fg(layer_color)),
        Span::styled(" → ", Style::default().fg(Color::DarkGray)),
        Span::styled(&kind_display, Style::default().fg(Color::White)),
    ]));
    all_lines.push(Line::from(Span::styled(
        "─".repeat(area.width.saturating_sub(2) as usize),
        Style::default().fg(Color::Rgb(60, 60, 70)),
    )));

    // Get instances
    let instances = app.tree.get_instances(kind_key);
    let instance_count = instances.map(|i| i.len()).unwrap_or(0);
    let is_loading = app
        .pending_instance_load
        .as_ref()
        .is_some_and(|k| k == kind_key);

    if instance_count == 0 {
        if is_loading {
            // Still loading from Neo4j
            all_lines.push(Line::from(Span::styled(
                "  ⏳ Loading instances from Neo4j...",
                Style::default().fg(Color::Yellow),
            )));
        } else {
            // Loaded but empty
            all_lines.push(Line::from(Span::styled(
                "  No instances exist for this Kind",
                Style::default().fg(Color::DarkGray),
            )));
        }
    } else if let Some(instances) = instances {
        for (idx, instance) in instances.iter().enumerate() {
            let is_cursor = idx == app.tree_cursor;
            let style = if is_cursor && focused {
                Style::default().bg(Color::Rgb(30, 40, 50)).fg(Color::Green)
            } else {
                Style::default().fg(Color::Rgb(100, 180, 100))
            };
            let prefix = if is_cursor { "› " } else { "  " };
            all_lines.push(Line::from(Span::styled(
                format!("{}• {}", prefix, instance.display_name),
                style,
            )));
        }
    }

    // Apply scroll
    let lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.tree_scroll)
        .take(visible_height)
        .collect();

    // Title with Kind name and count + position indicator
    let title = if instance_count > 0 {
        format!(
            " {} ({}/{}) [FILTERED] ",
            kind_display,
            app.tree_cursor + 1,
            instance_count
        )
    } else {
        format!(" {} (0) [FILTERED] ", kind_display)
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(layer_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Info panel: displays metadata for selected item with independent scroll.
fn render_info_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Info;
    let border_color = if focused {
        Color::Cyan
    } else {
        Color::Rgb(60, 60, 70)
    };

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

    // Show scroll indicator if scrollable
    let scroll_indicator = if app.info_line_count > visible_height {
        format!(
            " [{}/{}] ",
            app.info_scroll + 1,
            app.info_line_count.saturating_sub(visible_height) + 1
        )
    } else {
        String::new()
    };

    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::White),
        ))
        .title_bottom(Span::styled(
            scroll_indicator,
            Style::default().fg(Color::DarkGray),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Graph panel: Displays Neo4j relationships for the selected Kind or Instance.
///
/// Shows real arc data from Neo4j when a Kind is selected,
/// instance arcs in Data mode, or contextual messages for other selections.
fn render_graph_panel(f: &mut Frame, area: Rect, app: &App) {
    let theme = theme::Theme::new();
    let focused = app.focus == Focus::Graph;
    let border_color = if focused {
        Color::Magenta
    } else {
        Color::Rgb(60, 60, 70)
    };

    // Calculate arc count for title
    let arc_count = if let Some(ref arcs) = app.kind_arcs {
        arcs.incoming.len() + arcs.outgoing.len()
    } else if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
        inst.outgoing_arcs.len() + inst.incoming_arcs.len()
    } else {
        0
    };

    // Build title with count
    let title = if arc_count > 0 {
        format!(" Arc Relationships ({}) ", arc_count)
    } else {
        " Arc Relationships ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    let dim = Style::default().fg(Color::Rgb(100, 100, 100));
    let bright_dim = Style::default().fg(Color::Rgb(140, 140, 140));

    // === LOADING INDICATOR ===
    if app.pending_arcs_load.is_some()
        || app.pending_arc_kind_load.is_some()
        || app.pending_realm_load.is_some()
        || app.pending_layer_load.is_some()
    {
        lines.push(Line::from(Span::styled(
            "  ⏳ Loading from Neo4j...",
            Style::default().fg(Color::Yellow),
        )));
        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === REALM DETAILS VIEW ===
    if let Some(ref details) = app.realm_details {
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(theme.realm_color(&details.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Realm)", Style::default().fg(Color::DarkGray)),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Description (if any)
        if !details.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}", details.description),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
            lines.push(Line::from(Span::raw("")));
        }

        // Stats summary
        lines.push(Line::from(vec![
            Span::styled("  📊 ", dim),
            Span::styled(
                format!("{} Layers", details.layers.len()),
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(" · ", dim),
            Span::styled(
                format!("{} Node Kinds", details.total_kinds),
                Style::default().fg(Color::Green),
            ),
            Span::styled(" · ", dim),
            Span::styled(
                format!("{} Instances", details.total_instances),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Layers with kind counts (horizontal bar chart)
        lines.push(Line::from(Span::styled(
            "  LAYERS",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        let max_kinds = details
            .layers
            .iter()
            .map(|l| l.kind_count)
            .max()
            .unwrap_or(1)
            .max(1);
        let bar_max_width = 20usize;

        for layer in &details.layers {
            let bar_width = (layer.kind_count * bar_max_width) / max_kinds;
            let bar = "█".repeat(bar_width.max(1));

            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{:16}", layer.display_name),
                    Style::default().fg(theme.layer_color(&layer.key)),
                ),
                Span::styled(format!("{:>3} ", layer.kind_count), bright_dim),
                Span::styled(bar, Style::default().fg(theme.layer_color(&layer.key))),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === LAYER DETAILS VIEW ===
    if let Some(ref details) = app.layer_details {
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.realm,
                Style::default().fg(theme.realm_color(&details.realm)),
            ),
            Span::styled(" → ", bright_dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(theme.layer_color(&details.key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Layer)", Style::default().fg(Color::DarkGray)),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Description (if any)
        if !details.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}", details.description),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
            lines.push(Line::from(Span::raw("")));
        }

        // Stats summary
        lines.push(Line::from(vec![
            Span::styled("  📊 ", dim),
            Span::styled(
                format!("{} Node Kinds", details.total_kinds),
                Style::default().fg(Color::Green),
            ),
            Span::styled(" · ", dim),
            Span::styled(
                format!("{} Instances", details.total_instances),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Node Kinds grouped by trait
        lines.push(Line::from(Span::styled(
            "  NODE KINDS BY TRAIT",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        for group in &details.kinds_by_trait {
            // Trait header with color
            let trait_color = theme.trait_color(&group.trait_key);
            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{} ({})", group.trait_key, group.kind_names.len()),
                    Style::default()
                        .fg(trait_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            // Kind names
            for kind_name in &group.kind_names {
                lines.push(Line::from(vec![
                    Span::styled("      • ", dim),
                    Span::styled(
                        kind_name,
                        Style::default().fg(theme.layer_color(&details.key)),
                    ),
                ]));
            }
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === INSTANCE ARCS VIEW (Data mode) ===
    if let Some(TreeItem::Instance(realm, layer, kind, instance)) = app.current_item() {
        // Clone data to avoid lifetime issues
        let realm_key = realm.key.clone();
        let layer_key = layer.key.clone();
        let kind_display = kind.display_name.clone();
        let instance_cloned = instance.clone();

        // Breadcrumb for instance
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                realm_key.clone(),
                Style::default()
                    .fg(theme.realm_color(&realm_key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" → ", bright_dim),
            Span::styled(
                layer_key.clone(),
                Style::default()
                    .fg(theme.layer_color(&layer_key))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" → ", bright_dim),
            Span::styled(kind_display, Style::default().fg(Color::Green)),
            Span::styled(" → ", bright_dim),
            Span::styled(
                instance_cloned.key.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Render instance arcs
        let outgoing_arcs = instance_cloned.outgoing_arcs.clone();
        let incoming_arcs = instance_cloned.incoming_arcs.clone();
        let total = outgoing_arcs.len() + incoming_arcs.len();

        if total == 0 {
            lines.push(Line::from(Span::styled(
                "  No arc connections for this instance",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            // Outgoing arcs
            if !outgoing_arcs.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("  ─▶ OUTGOING ({}) ", outgoing_arcs.len()),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    "  ─────────────────────────────────────────",
                    dim,
                )));

                for arc in &outgoing_arcs {
                    let status_style = if arc.exists {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Yellow)
                    };
                    let status_char = if arc.exists { "✓" } else { "○" };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", status_char), status_style),
                        Span::styled(
                            instance_cloned.key.clone(),
                            Style::default().fg(Color::White),
                        ),
                        Span::styled(" ──[", dim),
                        Span::styled(
                            arc.arc_type.clone(),
                            Style::default()
                                .fg(theme.arc_family_color("semantic"))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("]──▶ ", dim),
                        Span::styled(arc.target_key.clone(), Style::default().fg(Color::Green)),
                        Span::styled(
                            format!(" ({})", arc.target_kind),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]));
                }
                lines.push(Line::from(Span::raw("")));
            }

            // Incoming arcs
            if !incoming_arcs.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("  ◀─ INCOMING ({}) ", incoming_arcs.len()),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(Span::styled(
                    "  ─────────────────────────────────────────",
                    dim,
                )));

                for arc in &incoming_arcs {
                    let status_style = if arc.exists {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Yellow)
                    };
                    let status_char = if arc.exists { "✓" } else { "○" };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", status_char), status_style),
                        Span::styled(arc.target_key.clone(), Style::default().fg(Color::Green)),
                        Span::styled(
                            format!(" ({})", arc.target_kind),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::styled(" ──[", dim),
                        Span::styled(
                            arc.arc_type.clone(),
                            Style::default()
                                .fg(theme.arc_family_color("semantic"))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled("]──▶ ", dim),
                        Span::styled(
                            instance_cloned.key.clone(),
                            Style::default().fg(Color::White),
                        ),
                    ]));
                }
            }
        }

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === KIND ARCS VIEW (from Neo4j) ===
    if let Some(ref arcs) = app.kind_arcs {
        // Hierarchy breadcrumb with theme colors
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &arcs.realm,
                Style::default()
                    .fg(theme.realm_color(&arcs.realm))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Realm)", Style::default().fg(Color::DarkGray)),
            Span::styled(" → ", bright_dim),
            Span::styled(
                &arcs.layer,
                Style::default()
                    .fg(theme.layer_color(&arcs.layer))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Layer)", Style::default().fg(Color::DarkGray)),
            Span::styled(" → ", bright_dim),
            Span::styled(
                &arcs.kind_label,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Node Kind)", Style::default().fg(Color::DarkGray)),
        ]));
        lines.push(Line::from(Span::raw("")));

        // Group all arcs by family
        render_arcs_by_family(&mut lines, arcs, &theme, &dim);

        let paragraph = Paragraph::new(lines);
        f.render_widget(paragraph, inner);
        return;
    }

    // === ARCKIND DETAILS VIEW ===
    if let Some(ref details) = app.arc_kind_details {
        let family_color = theme.arc_family_color(&details.family);

        // Arc name with family
        lines.push(Line::from(vec![
            Span::styled("  ", dim),
            Span::styled(
                &details.display_name,
                Style::default()
                    .fg(family_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  ({})", details.family),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
        lines.push(Line::from(Span::raw("")));

        // === ENDPOINTS ===
        lines.push(Line::from(Span::styled(
            "  ENDPOINTS",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        // FROM endpoint with theme colors
        if let Some(ref from) = details.from_endpoint {
            lines.push(Line::from(vec![
                Span::styled("    FROM: ", Style::default().fg(Color::Magenta)),
                Span::styled(
                    &from.kind_label,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  (", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &from.realm,
                    Style::default().fg(theme.realm_color(&from.realm)),
                ),
                Span::styled("/", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &from.layer,
                    Style::default().fg(theme.layer_color(&from.layer)),
                ),
                Span::styled(")", Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    FROM: ", Style::default().fg(Color::Magenta)),
                Span::styled("(not defined)", Style::default().fg(Color::DarkGray)),
            ]));
        }

        // TO endpoint with theme colors
        if let Some(ref to) = details.to_endpoint {
            lines.push(Line::from(vec![
                Span::styled("    TO:   ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    &to.kind_label,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  (", Style::default().fg(Color::DarkGray)),
                Span::styled(&to.realm, Style::default().fg(theme.realm_color(&to.realm))),
                Span::styled("/", Style::default().fg(Color::DarkGray)),
                Span::styled(&to.layer, Style::default().fg(theme.layer_color(&to.layer))),
                Span::styled(")", Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("    TO:   ", Style::default().fg(Color::Cyan)),
                Span::styled("(not defined)", Style::default().fg(Color::DarkGray)),
            ]));
        }
        lines.push(Line::from(Span::raw("")));

        // === PROPERTIES ===
        lines.push(Line::from(Span::styled(
            "  PROPERTIES",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            dim,
        )));

        // Cardinality
        if !details.cardinality.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("    Cardinality: ", dim),
                Span::styled(&details.cardinality, Style::default().fg(Color::Yellow)),
            ]));
        }

        // Cypher pattern
        if !details.cypher_pattern.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("    Pattern: ", dim),
                Span::styled(
                    &details.cypher_pattern,
                    Style::default().fg(theme.arc_family_color("generation")),
                ),
            ]));
        }

        // Description
        if !details.description.is_empty() {
            lines.push(Line::from(Span::raw("")));
            lines.push(Line::from(Span::styled(
                "  DESCRIPTION",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "  ─────────────────────────────────────────",
                dim,
            )));
            // Wrap description if too long
            for chunk in details.description.chars().collect::<Vec<_>>().chunks(45) {
                let line: String = chunk.iter().collect();
                lines.push(Line::from(Span::styled(
                    format!("    {}", line),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
    } else {
        // No Neo4j data - show contextual message
        let msg = match app.current_item() {
            Some(TreeItem::KindsSection) | Some(TreeItem::ArcsSection) => {
                "▸ Expand a section to explore"
            }
            Some(TreeItem::ArcFamily(_)) => "▸ Select an Arc to see endpoints",
            Some(TreeItem::ArcKind(_, _)) => "▸ Loading arc details...",
            Some(TreeItem::Realm(_)) | Some(TreeItem::Layer(_, _)) => {
                "▸ Select a Node Kind to see arc relationships"
            }
            _ => "▸ Select a Node Kind or Arc to see details",
        };
        lines.push(Line::from(Span::styled(
            msg,
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

/// Render arcs grouped by family (instead of by direction).
fn render_arcs_by_family(
    lines: &mut Vec<Line>,
    arcs: &super::data::KindArcsData,
    theme: &theme::Theme,
    dim: &Style,
) {
    use std::collections::BTreeMap;

    // Collect all arcs grouped by family (clone data to avoid lifetime issues)
    let mut by_family: BTreeMap<String, Vec<(bool, String, String)>> = BTreeMap::new();

    for arc in &arcs.incoming {
        by_family.entry(arc.family.clone()).or_default().push((
            false,
            arc.arc_key.clone(),
            arc.other_kind.clone(),
        )); // false = incoming
    }
    for arc in &arcs.outgoing {
        by_family.entry(arc.family.clone()).or_default().push((
            true,
            arc.arc_key.clone(),
            arc.other_kind.clone(),
        )); // true = outgoing
    }

    if by_family.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No arc relationships defined for this Node Kind",
            Style::default().fg(Color::DarkGray),
        )));
        return;
    }

    let kind_label = arcs.kind_label.clone();

    // Render each family group
    for (family, family_arcs) in &by_family {
        let family_color = theme.arc_family_color(family);
        let incoming_count = family_arcs.iter().filter(|(is_out, _, _)| !is_out).count();
        let outgoing_count = family_arcs.iter().filter(|(is_out, _, _)| *is_out).count();

        // Family header with counts
        lines.push(Line::from(vec![
            Span::styled("  ", *dim),
            Span::styled(
                family.to_uppercase(),
                Style::default()
                    .fg(family_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  (◀{} ▶{})", incoming_count, outgoing_count),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            *dim,
        )));

        // Render arcs in this family
        for (is_outgoing, arc_key, other_kind) in family_arcs {
            if *is_outgoing {
                // Outgoing: Kind ──[ARC]──▶ Target
                lines.push(Line::from(vec![
                    Span::styled("    ", *dim),
                    Span::styled(kind_label.clone(), Style::default().fg(Color::White)),
                    Span::styled(" ──[", *dim),
                    Span::styled(
                        arc_key.clone(),
                        Style::default()
                            .fg(family_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("]──▶ ", *dim),
                    Span::styled(other_kind.clone(), Style::default().fg(Color::Green)),
                ]));
            } else {
                // Incoming: Source ──[ARC]──▶ Kind
                lines.push(Line::from(vec![
                    Span::styled("    ", *dim),
                    Span::styled(other_kind.clone(), Style::default().fg(Color::Green)),
                    Span::styled(" ──[", *dim),
                    Span::styled(
                        arc_key.clone(),
                        Style::default()
                            .fg(family_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("]──▶ ", *dim),
                    Span::styled(kind_label.clone(), Style::default().fg(Color::White)),
                ]));
            }
        }
        lines.push(Line::from(Span::raw("")));
    }
}

/// Render instance arcs (actual connections in the data graph).
#[allow(dead_code)]
fn render_instance_arcs(lines: &mut Vec<Line>, instance: &super::data::InstanceInfo, dim: &Style) {
    let theme = theme::Theme::new();

    let total = instance.outgoing_arcs.len() + instance.incoming_arcs.len();
    if total == 0 {
        lines.push(Line::from(Span::styled(
            "  No arc connections for this instance",
            Style::default().fg(Color::DarkGray),
        )));
        return;
    }

    let instance_key = instance.key.clone();

    // Outgoing arcs
    if !instance.outgoing_arcs.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  ─▶ OUTGOING ({}) ", instance.outgoing_arcs.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            *dim,
        )));

        for arc in &instance.outgoing_arcs {
            let status_style = if arc.exists {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Yellow)
            };
            let status_char = if arc.exists { "✓" } else { "○" };

            lines.push(Line::from(vec![
                Span::styled(format!("  {} ", status_char), status_style),
                Span::styled(instance_key.clone(), Style::default().fg(Color::White)),
                Span::styled(" ──[", *dim),
                Span::styled(
                    arc.arc_type.clone(),
                    Style::default()
                        .fg(theme.arc_family_color("semantic"))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("]──▶ ", *dim),
                Span::styled(arc.target_key.clone(), Style::default().fg(Color::Green)),
                Span::styled(
                    format!(" ({})", arc.target_kind),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
        lines.push(Line::from(Span::raw("")));
    }

    // Incoming arcs
    if !instance.incoming_arcs.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  ◀─ INCOMING ({}) ", instance.incoming_arcs.len()),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────",
            *dim,
        )));

        for arc in &instance.incoming_arcs {
            let status_style = if arc.exists {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Yellow)
            };
            let status_char = if arc.exists { "✓" } else { "○" };

            lines.push(Line::from(vec![
                Span::styled(format!("  {} ", status_char), status_style),
                Span::styled(arc.target_key.clone(), Style::default().fg(Color::Green)),
                Span::styled(
                    format!(" ({})", arc.target_kind),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(" ──[", *dim),
                Span::styled(
                    arc.arc_type.clone(),
                    Style::default()
                        .fg(theme.arc_family_color("semantic"))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("]──▶ ", *dim),
                Span::styled(instance_key.clone(), Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::from(Span::raw("")));
    }
}

/// YAML panel: displays YAML source with independent scroll.
fn render_yaml_panel(f: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Yaml;
    let visible_height = area.height.saturating_sub(2) as usize;

    // Check if we're in Data mode on an Instance → show JSON instead of YAML
    let is_json_mode = app.mode == NavMode::Data
        && matches!(app.current_item(), Some(TreeItem::Instance(_, _, _, _)));

    if is_json_mode {
        render_json_panel(f, area, app, focused, visible_height);
    } else {
        render_yaml_content(f, area, app, focused, visible_height);
    }
}

/// Render JSON panel for Instance data in Data mode.
fn render_json_panel(f: &mut Frame, area: Rect, app: &App, focused: bool, visible_height: usize) {
    // Cyan border for JSON mode
    let border_color = if focused {
        Color::Cyan
    } else {
        Color::Rgb(60, 130, 130) // Dimmed cyan when unfocused
    };

    let (title, json_lines, total_lines) =
        if let Some(TreeItem::Instance(_, _, _, inst)) = app.current_item() {
            let all_lines = inst.to_colored_json();
            let total = all_lines.len();
            let visible: Vec<Line> = all_lines
                .into_iter()
                .skip(app.yaml_scroll)
                .take(visible_height)
                .collect();
            (format!(" {} ", inst.key), visible, total)
        } else {
            (
                " JSON ".to_string(),
                vec![Line::from(Span::styled(
                    "No instance data",
                    Style::default().fg(Color::DarkGray),
                ))],
                1,
            )
        };

    // Scroll indicator
    let scroll_indicator = if total_lines > visible_height {
        format!(
            " [{}/{}] ",
            app.yaml_scroll + 1,
            total_lines.saturating_sub(visible_height) + 1
        )
    } else {
        String::new()
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(Color::Cyan)))
        .title_bottom(Span::styled(
            scroll_indicator,
            Style::default().fg(Color::DarkGray),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(json_lines).block(block);
    f.render_widget(paragraph, area);
}

/// Render YAML panel (original behavior for Meta mode and Kind selection).
fn render_yaml_content(f: &mut Frame, area: Rect, app: &App, focused: bool, visible_height: usize) {
    let border_color = if focused {
        Color::Green
    } else {
        Color::Rgb(60, 60, 70)
    };

    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    if !app.yaml_content.is_empty() {
        for yaml_line in app
            .yaml_content
            .lines()
            .skip(app.yaml_scroll)
            .take(visible_height)
        {
            lines.push(highlight_yaml_line(yaml_line));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No YAML file",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Build title with colored path
    let title_spans = build_yaml_title(&app.yaml_path);

    // Show scroll indicator
    let total_lines = app.yaml_content.lines().count();
    let scroll_indicator = if total_lines > visible_height {
        format!(
            " [{}/{}] ",
            app.yaml_scroll + 1,
            total_lines.saturating_sub(visible_height) + 1
        )
    } else {
        String::new()
    };

    let block = Block::default()
        .title(Line::from(title_spans))
        .title_bottom(Span::styled(
            scroll_indicator,
            Style::default().fg(Color::DarkGray),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// Build YAML panel title with colored path segments.
fn build_yaml_title(path: &str) -> Vec<Span<'static>> {
    if path.is_empty() {
        return vec![Span::styled(
            " YAML ",
            Style::default().fg(Color::Rgb(60, 60, 70)),
        )];
    }

    let mut spans = vec![Span::styled(" ", Style::default())];
    spans.extend(colorize_path_inline(path));
    spans.push(Span::styled(" ", Style::default()));
    spans
}

/// Get title for detail panel based on current selection.
fn get_detail_title(app: &App) -> String {
    match app.current_item() {
        Some(TreeItem::KindsSection) => "Node Kinds".to_string(),
        Some(TreeItem::ArcsSection) => "Arcs".to_string(),
        Some(TreeItem::Realm(r)) => format!("{} {}", r.emoji, r.display_name),
        Some(TreeItem::Layer(_, l)) => l.display_name.clone(),
        Some(TreeItem::Kind(_, _, k)) => {
            if k.icon.is_empty() {
                k.display_name.clone()
            } else {
                format!("{} {}", k.icon, k.display_name)
            }
        }
        Some(TreeItem::ArcFamily(f)) => f.display_name.clone(),
        Some(TreeItem::ArcKind(_, ek)) => ek.display_name.clone(),
        Some(TreeItem::Instance(_, _, _, inst)) => {
            format!("{} ({})", inst.display_name, inst.kind_key)
        }
        None => "Detail".to_string(),
    }
}

/// Colorize path inline for title.
fn colorize_path_inline(path: &str) -> Vec<Span<'static>> {
    let parts: Vec<&str> = path.split('/').collect();
    let mut spans: Vec<Span<'static>> = Vec::new();

    for (i, part) in parts.iter().enumerate() {
        let color = match i {
            0..=2 => Color::Rgb(80, 80, 90), // packages/core/models
            3 => Color::Magenta,             // nodes
            4 => match *part {
                // realm (v10.6: 2 realms - global + tenant)
                "global" => Color::Green,
                "tenant" => Color::Yellow,
                _ => Color::White,
            },
            5 => Color::Rgb(100, 180, 100), // layer
            _ => Color::White,              // filename
        };
        spans.push(Span::styled(part.to_string(), Style::default().fg(color)));
        if i < parts.len() - 1 {
            spans.push(Span::styled(
                "/",
                Style::default().fg(Color::Rgb(50, 50, 60)),
            ));
        }
    }
    spans
}

/// Build info lines for detail panel.
fn build_info_lines(app: &App) -> Vec<Line<'static>> {
    // Use mode-aware item lookup (shows instances in Data mode)
    match app.current_item() {
        Some(TreeItem::KindsSection) => {
            let kind_count: usize = app
                .tree
                .realms
                .iter()
                .flat_map(|r| r.layers.iter())
                .map(|l| l.kinds.len())
                .sum();
            vec![
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Section", Style::default().fg(Color::Magenta)),
                ]),
                Line::from(vec![
                    Span::styled("realms    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        app.tree.realms.len().to_string(),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(kind_count.to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "h/l to collapse/expand",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
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
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Section", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("families  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        app.tree.arc_families.len().to_string(),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("arcs ", Style::default().fg(Color::DarkGray)),
                    Span::styled(arc_count.to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "h/l to collapse/expand",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        }
        Some(TreeItem::Realm(realm)) => {
            let kind_count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
            vec![
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Realm", Style::default().fg(Color::Magenta)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(realm.key.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("layers    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.layers.len().to_string(),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(kind_count.to_string(), Style::default().fg(Color::White)),
                ]),
            ]
        }
        Some(TreeItem::Layer(realm, layer)) => {
            vec![
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Layer", Style::default().fg(Color::Green)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(layer.key.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        layer.kinds.len().to_string(),
                        Style::default().fg(Color::White),
                    ),
                ]),
            ]
        }
        Some(TreeItem::Kind(realm, layer, kind)) => {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Node Kind", Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(kind.key.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        realm.display_name.clone(),
                        Style::default().fg(hex_to_color(&realm.color)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("layer     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        layer.display_name.clone(),
                        Style::default().fg(hex_to_color(&layer.color)),
                    ),
                ]),
            ];

            // Trait (if present)
            if !kind.trait_name.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("trait     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(kind.trait_name.clone(), Style::default().fg(Color::Magenta)),
                ]));
            }

            // v10.1: knowledge_tier removed from display (node type is sufficient)

            lines.push(Line::from(vec![
                Span::styled("instances ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    kind.instance_count.to_string(),
                    Style::default().fg(Color::White),
                ),
            ]));

            // Context budget (if present)
            if !kind.context_budget.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("budget    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        kind.context_budget.clone(),
                        Style::default().fg(Color::Cyan),
                    ),
                ]));
            }

            // Properties section (ALL properties with required markers)
            if !kind.properties.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Properties ({})", kind.properties.len()),
                    Style::default().fg(Color::Rgb(100, 100, 120)),
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
                lines.push(Line::from(Span::styled(
                    "  * = required",
                    Style::default().fg(Color::DarkGray),
                )));
            }

            // Arcs section
            if !kind.arcs.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Arcs ({})", kind.arcs.len()),
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));

                for arc in &kind.arcs {
                    let (arrow, arrow_color) = match arc.direction {
                        ArcDirection::Outgoing => ("→", Color::Cyan),
                        ArcDirection::Incoming => ("←", Color::Magenta),
                    };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", arrow), Style::default().fg(arrow_color)),
                        Span::styled(arc.rel_type.clone(), Style::default().fg(arrow_color)),
                        Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                        Span::styled(arc.target_kind.clone(), Style::default().fg(Color::Yellow)),
                    ]));
                }
            }

            // Description
            if !kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Description",
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));
                // Wrap description to multiple lines if too long
                let desc = &kind.description;
                for chunk in desc.chars().collect::<Vec<_>>().chunks(60) {
                    let line: String = chunk.iter().collect();
                    lines.push(Line::from(Span::styled(
                        format!("  {}", line),
                        Style::default().fg(Color::Rgb(150, 150, 150)),
                    )));
                }
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Cypher",
                Style::default().fg(Color::Rgb(100, 100, 120)),
            )));
            lines.push(Line::from(Span::styled(
                format!("  MATCH (n:{}) RETURN n LIMIT 100", kind.key),
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )));

            lines
        }
        Some(TreeItem::ArcFamily(family)) => {
            vec![
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("ArcFamily", Style::default().fg(Color::Rgb(180, 140, 80))),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(family.key.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("arcs ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        family.arc_kinds.len().to_string(),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "h/l to collapse/expand",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        }
        Some(TreeItem::ArcKind(family, arc_kind)) => {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("ArcKind", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(arc_kind.key.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("family    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        family.display_name.clone(),
                        Style::default().fg(Color::Rgb(180, 140, 80)),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("from      ", Style::default().fg(Color::DarkGray)),
                    Span::styled(arc_kind.from_kind.clone(), Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("to        ", Style::default().fg(Color::DarkGray)),
                    Span::styled(arc_kind.to_kind.clone(), Style::default().fg(Color::Cyan)),
                ]),
            ];

            // Cardinality (if present)
            if !arc_kind.cardinality.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("cardin.   ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        arc_kind.cardinality.clone(),
                        Style::default().fg(Color::Magenta),
                    ),
                ]));
            }

            // Description (if present)
            if !arc_kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Description",
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));
                lines.push(Line::from(Span::styled(
                    format!("  {}", &arc_kind.description),
                    Style::default().fg(Color::Rgb(150, 150, 150)),
                )));
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Cypher",
                Style::default().fg(Color::Rgb(100, 100, 120)),
            )));
            lines.push(Line::from(Span::styled(
                format!("  MATCH ()-[r:{}]->() RETURN r LIMIT 100", arc_kind.key),
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )));

            lines
        }
        Some(TreeItem::Instance(_realm, _layer, kind, instance)) => {
            // Instance info for Data view
            let mut lines: Vec<Line<'static>> = Vec::new();

            // Header
            lines.push(Line::from(vec![
                Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                Span::styled("Instance", Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                Span::styled(instance.key.clone(), Style::default().fg(Color::White)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("kind      ", Style::default().fg(Color::DarkGray)),
                Span::styled(kind.display_name.clone(), Style::default().fg(Color::Cyan)),
            ]));

            // Properties
            if !instance.properties.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Properties ({})", instance.properties.len()),
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));
                for (key, value) in &instance.properties {
                    let truncated = truncate_str(value, 40);
                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", key), Style::default().fg(Color::DarkGray)),
                        Span::styled(truncated, Style::default().fg(Color::White)),
                    ]));
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
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));

                // Box drawing for instance node (use char count for proper alignment)
                let key_width = instance.key.chars().count();
                lines.push(Line::from(Span::styled(
                    format!("  ┌{}┐", "─".repeat(key_width + 2)),
                    Style::default().fg(Color::Cyan),
                )));
                lines.push(Line::from(Span::styled(
                    format!("  │ {} │", instance.key),
                    Style::default().fg(Color::Cyan),
                )));
                lines.push(Line::from(Span::styled(
                    format!("  └{}┘", "─".repeat(key_width + 2)),
                    Style::default().fg(Color::Cyan),
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
                            Span::styled("    ══", Style::default().fg(Color::Green)),
                            Span::styled(
                                format!("[{}]", cmp.arc_type),
                                Style::default().fg(Color::Yellow),
                            ),
                            Span::styled("══> ", Style::default().fg(Color::Green)),
                            Span::styled(target_display, Style::default().fg(Color::White)),
                            Span::styled(" ✓", Style::default().fg(Color::Green)),
                        ]));
                    } else {
                        // Missing arc: dashed line (╌╌)
                        lines.push(Line::from(vec![
                            Span::styled("    ╌╌", Style::default().fg(Color::Red)),
                            Span::styled(
                                format!("[{}]", cmp.arc_type),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled("╌╌> ", Style::default().fg(Color::Red)),
                            Span::styled(
                                format!("({} - not connected)", cmp.target_kind),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled(" ✗", Style::default().fg(Color::Red)),
                        ]));
                    }
                }
            }

            lines
        }
        None => {
            vec![Line::from(Span::styled(
                "Select an item",
                Style::default().fg(Color::DarkGray),
            ))]
        }
    }
}

/// Highlight a YAML line with syntax coloring.
fn highlight_yaml_line(line: &str) -> Line<'static> {
    // Comment line
    if line.trim_start().starts_with('#') {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(Color::DarkGray),
        ));
    }

    // Empty line
    if line.trim().is_empty() {
        return Line::from(Span::raw(line.to_string()));
    }

    // Key-value or list item
    let mut spans: Vec<Span<'static>> = Vec::new();

    // Find leading whitespace
    let indent_len = line.len() - line.trim_start().len();
    let indent = &line[..indent_len];
    let rest = &line[indent_len..];

    spans.push(Span::raw(indent.to_string()));

    // Check for list item
    if rest.starts_with("- ") {
        spans.push(Span::styled("-", Style::default().fg(Color::Cyan)));
        let after_dash = &rest[1..];

        // Check if it's a key-value after dash
        if let Some(colon_pos) = after_dash.find(':') {
            let key = &after_dash[..colon_pos + 1];
            let value = &after_dash[colon_pos + 1..];
            spans.push(Span::styled(
                key.to_string(),
                Style::default().fg(Color::Yellow),
            ));
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(highlight_yaml_value(after_dash));
        }
    } else if let Some(colon_pos) = rest.find(':') {
        // Key-value pair
        let key = &rest[..colon_pos];
        let colon_and_rest = &rest[colon_pos..];

        spans.push(Span::styled(
            key.to_string(),
            Style::default().fg(Color::Yellow),
        ));

        if colon_and_rest.len() > 1 {
            spans.push(Span::styled(":", Style::default().fg(Color::White)));
            let value = &colon_and_rest[1..];
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(Span::styled(":", Style::default().fg(Color::White)));
        }
    } else {
        // Plain text
        spans.push(Span::styled(
            rest.to_string(),
            Style::default().fg(Color::White),
        ));
    }

    Line::from(spans)
}

/// Highlight a YAML value with appropriate color.
fn highlight_yaml_value(value: &str) -> Span<'static> {
    let trimmed = value.trim();

    // Boolean
    if trimmed == "true" || trimmed == "false" {
        return Span::styled(value.to_string(), Style::default().fg(Color::Magenta));
    }

    // Null
    if trimmed == "null" || trimmed == "~" {
        return Span::styled(value.to_string(), Style::default().fg(Color::Magenta));
    }

    // Number
    if trimmed.parse::<f64>().is_ok() {
        return Span::styled(value.to_string(), Style::default().fg(Color::Cyan));
    }

    // String (quoted)
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        return Span::styled(value.to_string(), Style::default().fg(Color::Green));
    }

    // Default string
    Span::styled(value.to_string(), Style::default().fg(Color::Green))
}

/// Status bar: stats + shortcuts.
fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let stats = &app.tree.stats;

    // Focus indicator
    let focus_label = match app.focus {
        Focus::Tree => "Tree",
        Focus::Info => "Info",
        Focus::Graph => "Graph",
        Focus::Yaml => "YAML",
    };

    // Layout indicator
    let layout_mode = LayoutMode::detect(area.width);
    let layout_label = match layout_mode {
        LayoutMode::Wide => "3-col",
        LayoutMode::Narrow => "stack",
    };

    let status = Line::from(vec![
        Span::styled(
            format!(" {} nodes", stats.node_count),
            Style::default().fg(Color::Rgb(100, 100, 120)),
        ),
        Span::styled(" │ ", Style::default().fg(Color::Rgb(50, 50, 60))),
        Span::styled(
            format!("{} arcs", stats.arc_count),
            Style::default().fg(Color::Rgb(100, 100, 120)),
        ),
        Span::styled(" │ ", Style::default().fg(Color::Rgb(50, 50, 60))),
        Span::styled(
            format!("{} Node Kinds", stats.kind_count),
            Style::default().fg(Color::Rgb(100, 100, 120)),
        ),
        Span::styled(" │ ", Style::default().fg(Color::Rgb(50, 50, 60))),
        Span::styled(
            format!("{} Arcs", stats.arc_kind_count),
            Style::default().fg(Color::Rgb(100, 100, 120)),
        ),
        Span::raw("        "),
        Span::styled(
            format!("[{}]", focus_label),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("  "),
        Span::styled(
            format!("[{}]", layout_label),
            Style::default().fg(Color::Rgb(80, 80, 100)),
        ),
        Span::raw("   "),
        Span::styled(
            "↑↓:scroll  Tab:panel  ",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let paragraph = Paragraph::new(status).style(Style::default().bg(Color::Rgb(15, 15, 20)));

    f.render_widget(paragraph, area);
}

/// Search overlay: input + results.
fn render_search(f: &mut Frame, app: &App) {
    // Center the search box
    let area = f.area();
    let width = 50.min(area.width.saturating_sub(4));
    let height = 12.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 3; // Slightly above center

    let search_area = Rect::new(x, y, width, height);

    // Clear the area behind the overlay
    f.render_widget(Clear, search_area);

    // Build content
    let mut lines: Vec<Line> = Vec::new();

    // Input line with cursor
    lines.push(Line::from(vec![
        Span::styled(" > ", Style::default().fg(Color::Cyan)),
        Span::styled(&app.search_query, Style::default().fg(Color::White)),
        Span::styled("█", Style::default().fg(Color::Cyan)), // Cursor
    ]));

    lines.push(Line::from(""));

    // Results count
    let count_text = if app.search_results.is_empty() {
        if app.search_query.is_empty() {
            "Type to search...".to_string()
        } else {
            "No results".to_string()
        }
    } else {
        format!("{} results", app.search_results.len())
    };
    lines.push(Line::from(Span::styled(
        count_text,
        Style::default().fg(Color::DarkGray),
    )));

    lines.push(Line::from(""));

    // Results list with scroll window around cursor
    let max_visible = 8;
    let total_results = app.search_results.len();

    // Calculate scroll window to keep cursor visible
    let start = if total_results <= max_visible || app.search_cursor < max_visible / 2 {
        0
    } else if app.search_cursor > total_results - max_visible / 2 {
        total_results.saturating_sub(max_visible)
    } else {
        app.search_cursor.saturating_sub(max_visible / 2)
    };

    let visible_results = app.search_results.iter().skip(start).take(max_visible);
    for (i, &idx) in visible_results.enumerate() {
        let actual_idx = start + i;
        let is_selected = actual_idx == app.search_cursor;
        let item = app.tree.item_at(idx);

        let (prefix, name, type_label) = match item {
            Some(TreeItem::KindsSection) => ("", "Node Kinds".to_string(), "Section"),
            Some(TreeItem::ArcsSection) => ("", "Arcs".to_string(), "Section"),
            Some(TreeItem::Realm(r)) => (r.emoji, r.display_name.clone(), "Realm"),
            Some(TreeItem::Layer(_, l)) => ("  ", l.display_name.clone(), "Layer"),
            Some(TreeItem::Kind(_, _, k)) => ("    ", k.display_name.clone(), "Node Kind"),
            Some(TreeItem::ArcFamily(f)) => ("  ", f.display_name.clone(), "ArcFamily"),
            Some(TreeItem::ArcKind(_, ek)) => ("    ", ek.display_name.clone(), "Arc Kind"),
            Some(TreeItem::Instance(_, _, _, inst)) => {
                ("      ", inst.display_name.clone(), "Instance")
            }
            None => ("?", "Unknown".to_string(), ""),
        };

        let style = if is_selected {
            Style::default().bg(Color::Rgb(30, 50, 70)).fg(Color::White)
        } else {
            Style::default().fg(Color::Rgb(150, 150, 150))
        };

        let type_style = if is_selected {
            Style::default()
                .bg(Color::Rgb(30, 50, 70))
                .fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" {}{}", prefix, name), style),
            Span::styled(format!("  {}", type_label), type_style),
        ]));
    }

    let block = Block::default()
        .title(Span::styled(" Search ", Style::default().fg(Color::Cyan)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(20, 20, 30)));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, search_area);
}

/// Help overlay: keyboard shortcuts.
fn render_help(f: &mut Frame) {
    let area = f.area();
    let width = 50.min(area.width.saturating_sub(4));
    let height = 32.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let help_area = Rect::new(x, y, width, height);
    f.render_widget(Clear, help_area);

    let lines = vec![
        Line::from(Span::styled(
            " NovaNet TUI — Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Navigation",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    Tab      ", Style::default().fg(Color::White)),
            Span::styled(
                "Cycle: Tree→Info→Graph→YAML",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("    ←→       ", Style::default().fg(Color::White)),
            Span::styled("Quick panel switch", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    j/k ↑↓   ", Style::default().fg(Color::White)),
            Span::styled("Move cursor / scroll", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Tree (vim-style)",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    h/l      ", Style::default().fg(Color::White)),
            Span::styled("Collapse/expand node", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    H/L      ", Style::default().fg(Color::White)),
            Span::styled("Collapse/expand all", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Graph panel",
            Style::default().fg(Color::Magenta),
        )]),
        Line::from(vec![
            Span::styled("    j/k ↑↓   ", Style::default().fg(Color::White)),
            Span::styled("Select neighbor node", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    h/l ←→   ", Style::default().fg(Color::White)),
            Span::styled(
                "Navigate incoming/outgoing",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("    Enter    ", Style::default().fg(Color::White)),
            Span::styled(
                "Jump to selected node",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Scrolling",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    d/u      ", Style::default().fg(Color::White)),
            Span::styled("Page down/up", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Modes",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    1-4      ", Style::default().fg(Color::White)),
            Span::styled(
                "Meta/Data/Overlay/Query",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("    N        ", Style::default().fg(Color::White)),
            Span::styled("Cycle through modes", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Actions",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    f        ", Style::default().fg(Color::White)),
            Span::styled("Find / search", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    /        ", Style::default().fg(Color::White)),
            Span::styled("Show this help", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    q        ", Style::default().fg(Color::White)),
            Span::styled("Quit", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press any key to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let block = Block::default()
        .title(Span::styled(" Help ", Style::default().fg(Color::Magenta)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .style(Style::default().bg(Color::Rgb(20, 20, 30)));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, help_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_str_ascii_under_limit() {
        assert_eq!(truncate_str("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_str_ascii_at_limit() {
        assert_eq!(truncate_str("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_str_ascii_over_limit() {
        assert_eq!(truncate_str("hello world", 5), "hello...");
    }

    #[test]
    fn test_truncate_str_utf8_bengali() {
        // Bengali: "বাংলা (বাংলাদেশ)" - this caused the original panic
        let bengali = "বাংলা (বাংলাদেশ)";
        // Should not panic even when truncating in the middle of multi-byte chars
        let result = truncate_str(bengali, 5);
        assert_eq!(result.chars().count(), 8); // 5 chars + "..."
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_str_utf8_emoji() {
        let emoji = "Hello 👋🏻 World 🌍";
        let result = truncate_str(emoji, 8);
        // "Hello 👋🏻" = 8 chars (emoji with skin tone is 2 chars)
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_truncate_str_chinese() {
        let chinese = "你好世界这是中文测试";
        let result = truncate_str(chinese, 4);
        assert_eq!(result, "你好世界...");
    }

    #[test]
    fn test_truncate_str_empty() {
        assert_eq!(truncate_str("", 10), "");
    }
}
