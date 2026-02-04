//! UI rendering for TUI v2.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::app::{App, Focus, NavMode};
use super::data::{ArcDirection, TreeItem};
use super::theme::{self, hex_to_color};

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

/// Wide layout: Tree (25%) | Info (25%) | YAML (50%).
fn render_main_wide(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Tree
            Constraint::Percentage(25), // Info
            Constraint::Percentage(50), // YAML
        ])
        .split(area);

    render_tree(f, chunks[0], app);
    render_info_panel(f, chunks[1], app);
    render_yaml_panel(f, chunks[2], app);
}

/// Narrow layout: Tree (40%) | Info+YAML stacked (60%).
fn render_main_narrow(f: &mut Frame, area: Rect, app: &mut App) {
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Tree
            Constraint::Percentage(60), // Detail (stacked)
        ])
        .split(area);

    render_tree(f, h_chunks[0], app);

    // Stack Info (60%) and YAML (40%) vertically
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(55), // Info
            Constraint::Percentage(45), // YAML
        ])
        .split(h_chunks[1]);

    render_info_panel(f, v_chunks[0], app);
    render_yaml_panel(f, v_chunks[1], app);
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
        format!("Kinds ({})", kinds_count),
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
                        for kind in &layer.kinds {
                            let count = if kind.instance_count > 0 {
                                format!(" ({})", kind.instance_count)
                            } else {
                                String::new()
                            };
                            // v10: Show knowledge tier badge for knowledge nodes
                            let tier_badge = match kind.knowledge_tier.as_deref() {
                                Some("technical") => " [T]",
                                Some("style") => " [S]",
                                Some("semantic") => " [M]", // M for meaning
                                _ => "",
                            };
                            all_lines.push(make_line(
                                idx,
                                app.tree_cursor,
                                focused,
                                "      ",
                                "",
                                format!("{}{}{}", kind.display_name, tier_badge, count),
                                Color::White,
                            ));
                            idx += 1;
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

    // Show scroll indicator in title
    let total = app.tree.item_count();
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

/// YAML panel: displays YAML source with independent scroll.
fn render_yaml_panel(f: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Yaml;
    let border_color = if focused {
        Color::Green
    } else {
        Color::Rgb(60, 60, 70)
    };

    // Build YAML lines with syntax highlighting
    let mut lines: Vec<Line> = Vec::new();

    let visible_height = area.height.saturating_sub(2) as usize;
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
    match app.tree.item_at(app.tree_cursor) {
        Some(TreeItem::KindsSection) => "Kinds".to_string(),
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
                // realm
                "global" => Color::Green,
                "project" => Color::Yellow,
                "shared" => Color::Cyan,
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
    match app.tree.item_at(app.tree_cursor) {
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
                    Span::styled("Kind", Style::default().fg(Color::Cyan)),
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

            // v10: Knowledge tier (if present, only for knowledge-trait nodes)
            if let Some(ref tier) = kind.knowledge_tier {
                let tier_color = match tier.as_str() {
                    "technical" => Color::Cyan,    // Blue for technical
                    "style" => Color::Magenta,     // Purple for style
                    "semantic" => Color::Yellow,   // Gold for semantic
                    _ => Color::White,
                };
                lines.push(Line::from(vec![
                    Span::styled("tier      ", Style::default().fg(Color::DarkGray)),
                    Span::styled(tier.clone(), Style::default().fg(tier_color)),
                ]));
            }

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
            format!("{} Kinds", stats.kind_count),
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
            Some(TreeItem::KindsSection) => ("", "Kinds".to_string(), "Section"),
            Some(TreeItem::ArcsSection) => ("", "Arcs".to_string(), "Section"),
            Some(TreeItem::Realm(r)) => (r.emoji, r.display_name.clone(), "Realm"),
            Some(TreeItem::Layer(_, l)) => ("  ", l.display_name.clone(), "Layer"),
            Some(TreeItem::Kind(_, _, k)) => ("    ", k.display_name.clone(), "Kind"),
            Some(TreeItem::ArcFamily(f)) => ("  ", f.display_name.clone(), "ArcFamily"),
            Some(TreeItem::ArcKind(_, ek)) => ("    ", ek.display_name.clone(), "ArcKind"),
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
    let width = 45.min(area.width.saturating_sub(4));
    let height = 26.min(area.height.saturating_sub(4));
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
            Span::styled("Toggle Tree ↔ Detail", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    ↑↓       ", Style::default().fg(Color::White)),
            Span::styled("Move cursor in tree", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Tree (vim-style)",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    h        ", Style::default().fg(Color::White)),
            Span::styled(
                "Collapse current node",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("    l        ", Style::default().fg(Color::White)),
            Span::styled("Expand current node", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    H        ", Style::default().fg(Color::White)),
            Span::styled("Collapse all", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    L        ", Style::default().fg(Color::White)),
            Span::styled("Expand all", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  YAML scroll",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![
            Span::styled("    j/k      ", Style::default().fg(Color::White)),
            Span::styled("Scroll YAML up/down", Style::default().fg(Color::DarkGray)),
        ]),
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
                "Data/Meta/Overlay/Query",
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
