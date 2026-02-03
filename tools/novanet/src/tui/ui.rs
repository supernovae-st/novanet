//! UI rendering for TUI v2.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::app::{App, Focus, NavMode};
use super::data::{EdgeDirection, TreeItem};

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
    let tabs: Vec<Span> = [NavMode::Data, NavMode::Meta, NavMode::Overlay, NavMode::Query]
        .iter()
        .enumerate()
        .map(|(i, mode)| {
            let num = format!("[{}]", i + 1);
            let label = mode.label();
            let is_active = *mode == app.mode;

            if is_active {
                Span::styled(
                    format!(" {}{}\u{2022} ", num, label),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
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
        Span::styled(" NovaNet ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::raw("          "),
    ];
    header.extend(tabs);

    let right_side = vec![
        Span::styled("  h/l:fold  f:find  /:help  q:quit", Style::default().fg(Color::DarkGray)),
    ];

    let mut full_header: Vec<Span<'static>> = header;
    // Calculate padding to right-align
    let header_len: usize = full_header.iter().map(|s| s.content.len()).sum();
    let right_len: usize = right_side.iter().map(|s| s.content.len()).sum();
    let padding = area.width.saturating_sub(header_len as u16 + right_len as u16);
    full_header.push(Span::raw(" ".repeat(padding as usize)));
    full_header.extend(right_side);

    let paragraph = Paragraph::new(Line::from(full_header))
        .style(Style::default().bg(Color::Rgb(15, 15, 20)));

    f.render_widget(paragraph, area);
}

/// Main content: Tree (left) + Detail (right).
fn render_main(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Tree
            Constraint::Percentage(60), // Detail
        ])
        .split(area);

    render_tree(f, chunks[0], app);
    render_detail(f, chunks[1], app);
}

/// Tree panel: taxonomy hierarchy with scroll and collapse.
fn render_tree(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.focus == Focus::Tree;
    let border_color = if focused { Color::Cyan } else { Color::DarkGray };

    // Calculate visible height (area minus borders)
    let visible_height = area.height.saturating_sub(2) as usize;
    app.tree_height = visible_height;

    // Build all visible tree lines
    let mut all_lines: Vec<Line> = Vec::new();
    let mut idx = 0;

    // Helper to create a line
    let make_line = |idx: usize, cursor: usize, focused: bool, indent: &str, icon: &str, text: String, color: Color| -> Line {
        let is_cursor = idx == cursor;
        let style = if is_cursor && focused {
            Style::default().bg(Color::Rgb(30, 40, 50)).fg(Color::White)
        } else {
            Style::default().fg(color)
        };
        let prefix = if is_cursor { "›" } else { " " };
        Line::from(Span::styled(format!("{}{}{} {}", prefix, indent, icon, text), style))
    };

    // === KINDS SECTION ===
    let kinds_collapsed = app.tree.is_collapsed("kinds");
    let kinds_icon = if kinds_collapsed { "▶" } else { "▼" };
    let kinds_count: usize = app.tree.realms.iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.kinds.len())
        .sum();
    all_lines.push(make_line(idx, app.tree_cursor, focused, "", kinds_icon, format!("Kinds ({})", kinds_count), Color::Magenta));
    idx += 1;

    if !kinds_collapsed {
        for realm in &app.tree.realms {
            let realm_key = format!("realm:{}", realm.key);
            let realm_collapsed = app.tree.is_collapsed(&realm_key);
            let realm_icon = if realm_collapsed { "▶" } else { "▼" };
            all_lines.push(make_line(idx, app.tree_cursor, focused, "  ", realm_icon, format!("{} {}", realm.emoji, realm.display_name), realm_color(&realm.key)));
            idx += 1;

            if !realm_collapsed {
                for layer in &realm.layers {
                    let layer_key = format!("layer:{}", layer.key);
                    let layer_collapsed = app.tree.is_collapsed(&layer_key);
                    let layer_icon = if layer_collapsed { "▶" } else { "▼" };
                    all_lines.push(make_line(idx, app.tree_cursor, focused, "    ", layer_icon, layer.display_name.clone(), Color::Rgb(100, 100, 120)));
                    idx += 1;

                    if !layer_collapsed {
                        for kind in &layer.kinds {
                            let count = if kind.instance_count > 0 {
                                format!(" ({})", kind.instance_count)
                            } else {
                                String::new()
                            };
                            all_lines.push(make_line(idx, app.tree_cursor, focused, "      ", "", format!("{}{}", kind.display_name, count), Color::White));
                            idx += 1;
                        }
                    }
                }
            }
        }
    }

    // === RELATIONS SECTION ===
    let relations_collapsed = app.tree.is_collapsed("relations");
    let relations_icon = if relations_collapsed { "▶" } else { "▼" };
    let relations_count: usize = app.tree.edge_families.iter().map(|f| f.edge_kinds.len()).sum();
    all_lines.push(make_line(idx, app.tree_cursor, focused, "", relations_icon, format!("Relations ({})", relations_count), Color::Yellow));
    idx += 1;

    if !relations_collapsed {
        for family in &app.tree.edge_families {
            let family_key = format!("family:{}", family.key);
            let family_collapsed = app.tree.is_collapsed(&family_key);
            let family_icon = if family_collapsed { "▶" } else { "▼" };
            all_lines.push(make_line(idx, app.tree_cursor, focused, "  ", family_icon, format!("{} ({})", family.display_name, family.edge_kinds.len()), Color::Rgb(180, 140, 80)));
            idx += 1;

            if !family_collapsed {
                for edge_kind in &family.edge_kinds {
                    all_lines.push(make_line(idx, app.tree_cursor, focused, "    ", "", edge_kind.display_name.clone(), Color::Rgb(150, 150, 150)));
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
        format!(" Taxonomy [{}-{}/{}] ", app.tree_scroll + 1, (app.tree_scroll + visible_height).min(total), total)
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

/// Color for realm.
fn realm_color(key: &str) -> Color {
    match key {
        "global" => Color::Green,
        "project" => Color::Yellow,
        "shared" => Color::Cyan,
        _ => Color::White,
    }
}

/// Detail panel: info (1/4) + YAML preview (3/4).
fn render_detail(f: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Detail;
    let border_color = if focused { Color::Cyan } else { Color::DarkGray };

    // Split detail area vertically: 1/4 info, 3/4 YAML
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 4), // Info
            Constraint::Ratio(3, 4), // YAML
        ])
        .split(area);

    render_detail_info(f, chunks[0], app, border_color);
    render_yaml_preview(f, chunks[1], app, border_color);
}

/// Detail info panel (top 1/4).
fn render_detail_info(f: &mut Frame, area: Rect, app: &App, border_color: Color) {
    let lines = match app.tree.item_at(app.tree_cursor) {
        Some(TreeItem::KindsSection) => {
            let kind_count: usize = app.tree.realms.iter()
                .flat_map(|r| r.layers.iter())
                .map(|l| l.kinds.len())
                .sum();
            vec![
                Line::from(Span::styled(
                    "Kinds",
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Section", Style::default().fg(Color::Magenta)),
                ]),
                Line::from(vec![
                    Span::styled("realms    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(app.tree.realms.len().to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(kind_count.to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", Style::default().fg(Color::DarkGray))),
            ]
        }
        Some(TreeItem::RelationsSection) => {
            let edge_count: usize = app.tree.edge_families.iter().map(|f| f.edge_kinds.len()).sum();
            vec![
                Line::from(Span::styled(
                    "Relations",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Section", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("families  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(app.tree.edge_families.len().to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("relations ", Style::default().fg(Color::DarkGray)),
                    Span::styled(edge_count.to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", Style::default().fg(Color::DarkGray))),
            ]
        }
        Some(TreeItem::Realm(realm)) => {
            let kind_count: usize = realm.layers.iter().map(|l| l.kinds.len()).sum();
            vec![
                Line::from(Span::styled(
                    format!("{} {}", realm.emoji, realm.display_name),
                    Style::default().fg(realm_color(&realm.key)).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Realm", Style::default().fg(Color::Magenta)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&realm.key, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("layers    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(realm.layers.len().to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(kind_count.to_string(), Style::default().fg(Color::White)),
                ]),
            ]
        }
        Some(TreeItem::Layer(realm, layer)) => {
            vec![
                Line::from(Span::styled(
                    &layer.display_name,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Layer", Style::default().fg(Color::Green)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&layer.key, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&realm.display_name, Style::default().fg(realm_color(&realm.key))),
                ]),
                Line::from(vec![
                    Span::styled("kinds     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(layer.kinds.len().to_string(), Style::default().fg(Color::White)),
                ]),
            ]
        }
        Some(TreeItem::Kind(realm, layer, kind)) => {
            let title = if kind.icon.is_empty() {
                kind.display_name.clone()
            } else {
                format!("{} {}", kind.icon, kind.display_name)
            };
            let mut lines = vec![
                Line::from(Span::styled(
                    title,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("Kind", Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&kind.key, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("realm     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&realm.display_name, Style::default().fg(realm_color(&realm.key))),
                ]),
                Line::from(vec![
                    Span::styled("layer     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&layer.display_name, Style::default().fg(Color::Green)),
                ]),
            ];

            // Trait (if present)
            if !kind.trait_name.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("trait     ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&kind.trait_name, Style::default().fg(Color::Magenta)),
                ]));
            }

            lines.push(Line::from(vec![
                Span::styled("instances ", Style::default().fg(Color::DarkGray)),
                Span::styled(kind.instance_count.to_string(), Style::default().fg(Color::White)),
            ]));

            // Edges section
            if !kind.edges.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Edges ({})", kind.edges.len()),
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                )));

                for edge in &kind.edges {
                    let (arrow, arrow_color) = match edge.direction {
                        EdgeDirection::Outgoing => ("→", Color::Cyan),
                        EdgeDirection::Incoming => ("←", Color::Magenta),
                    };

                    lines.push(Line::from(vec![
                        Span::styled(format!("  {} ", arrow), Style::default().fg(arrow_color)),
                        Span::styled(&edge.rel_type, Style::default().fg(arrow_color)),
                        Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                        Span::styled(&edge.target_kind, Style::default().fg(Color::Yellow)),
                    ]));
                }
            }

            // Description
            if !kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Description", Style::default().fg(Color::Rgb(100, 100, 120)))));
                lines.push(Line::from(Span::styled(
                    format!("  {}", &kind.description),
                    Style::default().fg(Color::Rgb(150, 150, 150)),
                )));
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Cypher", Style::default().fg(Color::Rgb(100, 100, 120)))));
            lines.push(Line::from(Span::styled(
                format!("  MATCH (n:{}) RETURN n LIMIT 100", kind.key),
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )));

            lines
        }
        Some(TreeItem::EdgeFamily(family)) => {
            vec![
                Line::from(Span::styled(
                    &family.display_name,
                    Style::default().fg(Color::Rgb(180, 140, 80)).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("EdgeFamily", Style::default().fg(Color::Rgb(180, 140, 80))),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&family.key, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("relations ", Style::default().fg(Color::DarkGray)),
                    Span::styled(family.edge_kinds.len().to_string(), Style::default().fg(Color::White)),
                ]),
                Line::from(""),
                Line::from(Span::styled("h/l to collapse/expand", Style::default().fg(Color::DarkGray))),
            ]
        }
        Some(TreeItem::EdgeKind(family, edge_kind)) => {
            let mut lines = vec![
                Line::from(Span::styled(
                    &edge_kind.display_name,
                    Style::default().fg(Color::Rgb(150, 150, 150)).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("type      ", Style::default().fg(Color::DarkGray)),
                    Span::styled("EdgeKind", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("key       ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&edge_kind.key, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("family    ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&family.display_name, Style::default().fg(Color::Rgb(180, 140, 80))),
                ]),
                Line::from(vec![
                    Span::styled("from      ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&edge_kind.from_kind, Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("to        ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&edge_kind.to_kind, Style::default().fg(Color::Cyan)),
                ]),
            ];

            // Cardinality (if present)
            if !edge_kind.cardinality.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("cardin.   ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&edge_kind.cardinality, Style::default().fg(Color::Magenta)),
                ]));
            }

            // Description (if present)
            if !edge_kind.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("Description", Style::default().fg(Color::Rgb(100, 100, 120)))));
                lines.push(Line::from(Span::styled(
                    format!("  {}", &edge_kind.description),
                    Style::default().fg(Color::Rgb(150, 150, 150)),
                )));
            }

            // Cypher
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled("Cypher", Style::default().fg(Color::Rgb(100, 100, 120)))));
            lines.push(Line::from(Span::styled(
                format!("  MATCH ()-[r:{}]->() RETURN r LIMIT 100", edge_kind.key),
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )));

            lines
        }
        None => {
            vec![
                Line::from(Span::styled(
                    "Select an item",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        }
    };

    let block = Block::default()
        .title(Span::styled(" Detail ", Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

/// YAML preview panel (bottom 3/4).
fn render_yaml_preview(f: &mut Frame, area: Rect, app: &App, border_color: Color) {
    // Title with file path
    let title = if app.yaml_path.is_empty() {
        " YAML ".to_string()
    } else {
        format!(" {} ", app.yaml_path)
    };

    // Parse YAML and apply syntax highlighting
    let lines: Vec<Line> = if app.yaml_content.is_empty() {
        vec![Line::from(Span::styled(
            "No YAML file for this item",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        app.yaml_content
            .lines()
            .map(highlight_yaml_line)
            .collect()
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((app.yaml_scroll as u16, 0));
    f.render_widget(paragraph, area);
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
            spans.push(Span::styled(key.to_string(), Style::default().fg(Color::Yellow)));
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(highlight_yaml_value(after_dash));
        }
    } else if let Some(colon_pos) = rest.find(':') {
        // Key-value pair
        let key = &rest[..colon_pos];
        let colon_and_rest = &rest[colon_pos..];

        spans.push(Span::styled(key.to_string(), Style::default().fg(Color::Yellow)));

        if colon_and_rest.len() > 1 {
            spans.push(Span::styled(":", Style::default().fg(Color::White)));
            let value = &colon_and_rest[1..];
            spans.push(highlight_yaml_value(value));
        } else {
            spans.push(Span::styled(":", Style::default().fg(Color::White)));
        }
    } else {
        // Plain text
        spans.push(Span::styled(rest.to_string(), Style::default().fg(Color::White)));
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
    let status = Line::from(vec![
        Span::styled(format!(" {} nodes", stats.node_count), Style::default().fg(Color::Cyan)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{} edges", stats.edge_count), Style::default().fg(Color::Cyan)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{} Kinds", stats.kind_count), Style::default().fg(Color::Cyan)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{} EdgeKinds", stats.edge_kind_count), Style::default().fg(Color::Cyan)),
        Span::raw("                              "),
        Span::styled("↑↓:nav  ←→:panel  N:mode  ", Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(status)
        .style(Style::default().bg(Color::Rgb(15, 15, 20)));

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
            Some(TreeItem::RelationsSection) => ("", "Relations".to_string(), "Section"),
            Some(TreeItem::Realm(r)) => (r.emoji, r.display_name.clone(), "Realm"),
            Some(TreeItem::Layer(_, l)) => ("  ", l.display_name.clone(), "Layer"),
            Some(TreeItem::Kind(_, _, k)) => ("    ", k.display_name.clone(), "Kind"),
            Some(TreeItem::EdgeFamily(f)) => ("  ", f.display_name.clone(), "Family"),
            Some(TreeItem::EdgeKind(_, ek)) => ("    ", ek.display_name.clone(), "EdgeKind"),
            None => ("?", "Unknown".to_string(), ""),
        };

        let style = if is_selected {
            Style::default().bg(Color::Rgb(30, 50, 70)).fg(Color::White)
        } else {
            Style::default().fg(Color::Rgb(150, 150, 150))
        };

        let type_style = if is_selected {
            Style::default().bg(Color::Rgb(30, 50, 70)).fg(Color::DarkGray)
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
    let height = 24.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let help_area = Rect::new(x, y, width, height);
    f.render_widget(Clear, help_area);

    let lines = vec![
        Line::from(Span::styled(
            " NovaNet TUI — Keyboard Shortcuts",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Navigation", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("    ↑↓       ", Style::default().fg(Color::White)),
            Span::styled("Move cursor in tree", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    ←→       ", Style::default().fg(Color::White)),
            Span::styled("Switch panel focus", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Tree (vim-style)", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("    h        ", Style::default().fg(Color::White)),
            Span::styled("Collapse current node", Style::default().fg(Color::DarkGray)),
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
        Line::from(vec![
            Span::styled("  Modes", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("    1-4      ", Style::default().fg(Color::White)),
            Span::styled("Data/Meta/Overlay/Query", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("    N        ", Style::default().fg(Color::White)),
            Span::styled("Cycle through modes", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Actions", Style::default().fg(Color::Yellow)),
        ]),
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
