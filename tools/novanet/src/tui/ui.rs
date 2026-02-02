//! UI rendering: Galaxy-themed mission control layout.
//!
//! Five-area layout:
//! ```text
//! ┌─ Mode Tabs ───────────────────────────────────┐
//! ├──────────────┬────────────────────────────────┤
//! │  TAXONOMY    │  KIND DETAIL                    │
//! │  (tree)      │                                 │
//! │  35%         ├────────────────────────────────┤
//! │              │  CYPHER PREVIEW                 │
//! │              │  (read-only)                    │
//! ├──────────────┴────────────────────────────────┤
//! │  Status bar                                    │
//! └───────────────────────────────────────────────┘
//! ```

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::tui::app::{ActivePanel, AppState, NavMode};
use crate::tui::boot::{BootStage, BootState};
use crate::tui::dashboard::{self as dash, DashboardStats};
use crate::tui::detail::{self, KindDetail};
use crate::tui::dialogs::{DialogKind, DialogState, FieldKind};
use crate::tui::effects::{self, EffectsState, NebulaPulse};
use crate::tui::logo;
use crate::tui::onboarding::{self, OnboardingState, TourTarget};
use crate::tui::palette::PaletteState;
use crate::tui::search::SearchState;
use crate::tui::theme;
use crate::tui::tree::{TaxonomyTree, TreeNodeType};

/// Render the full UI frame.
pub fn render(frame: &mut Frame, state: &AppState) {
    // Fill entire terminal with deep space background
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(theme::BG_VOID)),
        area,
    );

    match state {
        AppState::Loading { message } => render_loading(frame, message),
        AppState::Booting { boot, .. } => render_booting(frame, boot),
        AppState::Ready {
            mode,
            tree,
            active_panel,
            detail_lines,
            status,
            facets,
            node_count,
            cypher_preview,
            kind_detail,
            search,
            edge_explorer_idx,
            dialog,
            edge_kind_keys: _,
            dashboard_stats,
            show_dashboard,
            palette,
            show_help,
            tick,
            effects,
            onboarding,
        } => {
            render_ready(
                frame,
                *mode,
                tree,
                *active_panel,
                detail_lines,
                status,
                facets.show_popup,
                *node_count,
                cypher_preview,
                kind_detail.as_deref(),
                search.as_ref(),
                *edge_explorer_idx,
                dialog.as_ref(),
                dashboard_stats.as_ref(),
                *show_dashboard,
                palette.as_ref(),
                *show_help,
                effects.pulse.as_ref(),
                onboarding.as_ref(),
            );

            // Post-processing effects
            apply_effects(frame, effects, *tick);
        }
    }
}

fn render_loading(frame: &mut Frame, message: &str) {
    let area = frame.area();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::NEBULA_PURPLE))
        .style(Style::default().bg(theme::BG_VOID));

    // Build content: logo + branding + status
    let mut lines: Vec<Line> = Vec::new();

    // Vertical centering: add blank lines before logo
    let logo_height = logo::FULL_LOGO.len() + 4; // logo + branding + status
    let padding = area.height.saturating_sub(logo_height as u16 + 2) / 2;
    for _ in 0..padding {
        lines.push(Line::from(""));
    }

    // Render colorized logo
    lines.extend(logo::full_logo_lines());

    // Branding
    lines.extend(logo::branding_lines());

    // Status message
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        message.to_string(),
        theme::dim_style(),
    )));

    let paragraph = Paragraph::new(lines).block(block).centered();
    frame.render_widget(paragraph, area);
}

fn render_booting(frame: &mut Frame, boot: &BootState) {
    let area = frame.area();

    // Step 1: Build rain lines (background fill)
    let clear_pct = boot.clear_progress();
    let cx = area.width as f32 / 2.0;
    let cy = area.height as f32 / 2.0;
    let max_r = (cx * cx + cy * cy).sqrt();
    let clear_r = max_r * clear_pct;

    let rain_lines: Vec<Line> = (0..area.height)
        .map(|y| {
            let mut spans: Vec<Span> = Vec::new();
            let mut x = 0u16;
            while x < area.width {
                // Clear zone: circular expanding region at center
                if clear_pct > 0.0 {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    if (dx * dx + dy * dy).sqrt() < clear_r {
                        spans.push(Span::raw(" "));
                        x += 1;
                        continue;
                    }
                }
                // Rain character from MatrixRain simulation
                if let Some((ch, brightness)) = boot.rain.char_at(x, y) {
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(Color::Rgb(brightness / 8, brightness, brightness / 6)),
                    ));
                    x += 2; // rain columns are spaced for double-width chars
                } else {
                    spans.push(Span::raw(" "));
                    x += 1;
                }
            }
            Line::from(spans)
        })
        .collect();

    let rain = Paragraph::new(rain_lines).style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(rain, area);

    // Step 2: Logo overlay (centered, overwrites rain in that region)
    let show_logo = !matches!(boot.stage, BootStage::MatrixRain | BootStage::RainClear);
    if show_logo {
        let all_logo = logo::full_logo_lines();
        let visible_count = match &boot.stage {
            BootStage::LogoReveal { lines_visible } => *lines_visible,
            _ => all_logo.len(),
        };

        let mut lines: Vec<Line> = all_logo.into_iter().take(visible_count).collect();

        match &boot.stage {
            BootStage::BrandingType { typewriter } => {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "SuperNovae Studio",
                    Style::default()
                        .fg(theme::NOVA_WHITE)
                        .add_modifier(Modifier::BOLD),
                )));
                let mut spans = vec![Span::styled(
                    typewriter.visible().to_string(),
                    Style::default().fg(theme::STAR_DIM),
                )];
                if let Some(c) = typewriter.cursor_char() {
                    spans.push(Span::styled(
                        c.to_string(),
                        Style::default().fg(theme::CYBER_CYAN),
                    ));
                }
                lines.push(Line::from(spans));
            }
            BootStage::HoldLogo { .. } | BootStage::FadeOut { .. } => {
                lines.extend(logo::branding_lines());
            }
            _ => {}
        }

        let h = lines.len() as u16;
        let y = area.y + area.height.saturating_sub(h) / 2;
        let logo_area = Rect::new(area.x, y, area.width, h.min(area.height));

        let logo_p = Paragraph::new(lines)
            .centered()
            .style(Style::default().bg(theme::BG_VOID));
        frame.render_widget(logo_p, logo_area);
    }

    // Step 3: Fade dimming during FadeOut stage
    if let BootStage::FadeOut { .. } = &boot.stage {
        let fade = boot.fade_progress();
        let factor = 1.0 - fade;
        let area = frame.area();
        let buf = frame.buffer_mut();
        for y in 0..area.height {
            for x in 0..area.width {
                if let Some(cell) = buf.cell_mut((x + area.x, y + area.y)) {
                    if let Color::Rgb(r, g, b) = cell.fg {
                        cell.set_fg(Color::Rgb(
                            (r as f32 * factor) as u8,
                            (g as f32 * factor) as u8,
                            (b as f32 * factor) as u8,
                        ));
                    }
                }
            }
        }
    }
}

/// Apply post-processing visual effects to the rendered frame.
fn apply_effects(frame: &mut Frame, effects: &EffectsState, tick: u64) {
    // Screen shake: shift entire buffer content by offset
    if let Some(ref shake) = effects.shake {
        let (dx, dy) = shake.current_offset();
        effects::apply_shake(frame.buffer_mut(), dx, dy);
    }
    // CRT scanlines: dim even rows + phosphor flicker
    if effects.crt_enabled {
        effects::apply_crt_scanlines(frame.buffer_mut(), tick);
    }
    // Glitch: corrupt cells with block characters + purple shift
    if let Some(ref glitch) = effects.glitch {
        effects::apply_glitch(frame.buffer_mut(), glitch.current_intensity(), tick);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_ready(
    frame: &mut Frame,
    mode: NavMode,
    tree: &TaxonomyTree,
    active_panel: ActivePanel,
    detail_lines: &[String],
    status: &str,
    show_facet_popup: bool,
    node_count: usize,
    cypher_preview: &[String],
    kind_detail: Option<&KindDetail>,
    search: Option<&SearchState>,
    edge_explorer_idx: Option<usize>,
    dialog: Option<&DialogState>,
    dashboard_stats: Option<&DashboardStats>,
    show_dashboard: bool,
    palette: Option<&PaletteState>,
    show_help: bool,
    pulse: Option<&NebulaPulse>,
    onboarding: Option<&OnboardingState>,
) {
    let area = frame.area();

    // Vertical: [breadcrumb/tabs | main content | status bar]
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // mode tabs + breadcrumb
            Constraint::Min(5),    // main content
            Constraint::Length(1), // status bar
        ])
        .split(area);

    render_mode_tabs(frame, vertical[0], mode, tree);
    render_main_content(
        frame,
        vertical[1],
        tree,
        active_panel,
        detail_lines,
        cypher_preview,
        kind_detail,
        edge_explorer_idx,
        dashboard_stats,
        show_dashboard,
        pulse,
    );
    render_status_bar(frame, vertical[2], mode, status, node_count);

    if show_facet_popup {
        render_facet_popup(frame, area);
    }

    if let Some(s) = search {
        render_search_overlay(frame, area, s);
    }

    if let Some(pal) = palette {
        render_palette_overlay(frame, area, pal);
    }

    if show_help {
        render_help_overlay(frame, area);
    }

    if let Some(dlg) = dialog {
        render_dialog(frame, area, dlg);
    }

    if let Some(ob) = onboarding {
        render_onboarding_overlay(frame, area, ob);
    }
}

fn render_mode_tabs(frame: &mut Frame, area: Rect, current_mode: NavMode, tree: &TaxonomyTree) {
    let modes = [
        NavMode::Data,
        NavMode::Meta,
        NavMode::Overlay,
        NavMode::Query,
    ];
    let mut spans: Vec<Span> = Vec::new();

    // Inline logo
    spans.push(Span::styled(
        " \u{25c9}\u{2550}\u{2573} ",
        Style::default()
            .fg(theme::CYBER_CYAN)
            .add_modifier(Modifier::BOLD),
    ));
    spans.push(Span::styled(
        "\u{2502} ",
        Style::default().fg(theme::STAR_DIM),
    ));

    // Mode tabs
    for (i, &m) in modes.iter().enumerate() {
        let active = m == current_mode;
        let color = if active {
            theme::mode_color(m)
        } else {
            theme::STAR_DIM
        };
        let style = Style::default().fg(color).add_modifier(if active {
            Modifier::BOLD
        } else {
            Modifier::empty()
        });
        spans.push(Span::styled(format!("{} {} ", i + 1, m.label()), style));
        if i < 3 {
            spans.push(Span::styled(
                "\u{2502} ", // │
                Style::default().fg(theme::STAR_DIM),
            ));
        }
    }

    // Breadcrumb: show current navigation path
    if let Some(node) = tree.selected() {
        spans.push(Span::styled(
            "  \u{25b8} ", // ▸
            Style::default().fg(theme::STAR_DIM),
        ));
        // Show realm/layer/kind based on node type
        let crumb = match node.node_type {
            TreeNodeType::Realm => node.display_name.clone(),
            TreeNodeType::Layer => {
                // Find parent realm
                if let Some(parent) = tree.parent_of(&node.key) {
                    format!("{} \u{25b8} {}", parent.display_name, node.display_name)
                } else {
                    node.display_name.clone()
                }
            }
            TreeNodeType::Kind => {
                // Find parent layer and realm
                if let Some(layer) = tree.parent_of(&node.key) {
                    if let Some(realm) = tree.parent_of(&layer.key) {
                        format!(
                            "{} \u{25b8} {} \u{25b8} {}",
                            realm.display_name, layer.display_name, node.display_name
                        )
                    } else {
                        format!("{} \u{25b8} {}", layer.display_name, node.display_name)
                    }
                } else {
                    node.display_name.clone()
                }
            }
        };
        spans.push(Span::styled(crumb, Style::default().fg(theme::NOVA_WHITE)));
    }

    let tabs = Paragraph::new(Line::from(spans)).style(Style::default().bg(theme::BG_PANEL));
    frame.render_widget(tabs, area);
}

#[allow(clippy::too_many_arguments)]
fn render_main_content(
    frame: &mut Frame,
    area: Rect,
    tree: &TaxonomyTree,
    active_panel: ActivePanel,
    detail_lines: &[String],
    cypher_preview: &[String],
    kind_detail: Option<&KindDetail>,
    edge_explorer_idx: Option<usize>,
    dashboard_stats: Option<&DashboardStats>,
    show_dashboard: bool,
    pulse: Option<&NebulaPulse>,
) {
    // Horizontal: [tree 35% | right pane 65%]
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    let tree_active = active_panel == ActivePanel::Tree;
    let detail_active = active_panel == ActivePanel::Detail;
    let cypher_active = active_panel == ActivePanel::CypherPreview;

    render_tree_panel(
        frame,
        horizontal[0],
        tree,
        tree_active,
        if tree_active { pulse } else { None },
    );

    if let (true, Some(stats)) = (show_dashboard, dashboard_stats) {
        // Right pane: vertical split [detail 40% | dashboard 30% | cypher 30%]
        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ])
            .split(horizontal[1]);

        render_detail_panel(
            frame,
            right[0],
            detail_lines,
            detail_active,
            kind_detail,
            edge_explorer_idx,
            if detail_active { pulse } else { None },
        );
        render_dashboard_panel(frame, right[1], stats);
        render_cypher_preview(
            frame,
            right[2],
            cypher_preview,
            cypher_active,
            if cypher_active { pulse } else { None },
        );
    } else {
        // Right pane: vertical split [detail 65% | cypher preview 35%]
        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(horizontal[1]);

        render_detail_panel(
            frame,
            right[0],
            detail_lines,
            detail_active,
            kind_detail,
            edge_explorer_idx,
            if detail_active { pulse } else { None },
        );
        render_cypher_preview(
            frame,
            right[1],
            cypher_preview,
            cypher_active,
            if cypher_active { pulse } else { None },
        );
    }
}

fn render_tree_panel(
    frame: &mut Frame,
    area: Rect,
    tree: &TaxonomyTree,
    active: bool,
    pulse: Option<&NebulaPulse>,
) {
    let border = theme::panel_border_with_pulse(active, pulse);
    let block = Block::default()
        .title(Span::styled(
            " Taxonomy ",
            theme::accent_bold(theme::CYBER_CYAN),
        ))
        .borders(Borders::ALL)
        .border_style(border)
        .style(Style::default().bg(theme::BG_PANEL));

    let items: Vec<ListItem> = tree
        .visible_items()
        .iter()
        .enumerate()
        .map(|(i, (depth, node))| {
            let indent = "  ".repeat(*depth);
            let (icon, color) = match node.node_type {
                TreeNodeType::Realm => {
                    let arrow = if node.expanded {
                        "\u{25bc}"
                    } else {
                        "\u{25b6}"
                    }; // ▼ / ▶
                    let emoji = theme::realm_emoji(&node.key);
                    let c = theme::realm_color(&node.key);
                    (format!("{emoji}{arrow} "), c)
                }
                TreeNodeType::Layer => {
                    let arrow = if node.expanded {
                        "\u{25bc}"
                    } else {
                        "\u{25b6}"
                    };
                    let c = theme::layer_color(&node.key);
                    (format!("{arrow} "), c)
                }
                TreeNodeType::Kind => ("  ".to_string(), theme::NOVA_WHITE),
            };
            let label = format!("{indent}{icon}{}", node.display_name);

            let style = if i == tree.cursor {
                theme::selected_style(color)
            } else {
                theme::tree_item_style(color)
            };

            ListItem::new(Span::styled(label, style))
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn render_detail_panel(
    frame: &mut Frame,
    area: Rect,
    lines: &[String],
    active: bool,
    kind_detail: Option<&KindDetail>,
    edge_explorer_idx: Option<usize>,
    pulse: Option<&NebulaPulse>,
) {
    let border = theme::panel_border_with_pulse(active, pulse);

    let title = if edge_explorer_idx.is_some() {
        " Edge Explorer "
    } else {
        " Detail "
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            theme::accent_bold(theme::NEBULA_PURPLE),
        ))
        .borders(Borders::ALL)
        .border_style(border)
        .style(Style::default().bg(theme::BG_PANEL));

    // Edge explorer mode: show focused edge view
    // Normal mode: use Galaxy-themed styled lines or plain text fallback
    let text: Vec<Line> = if let (Some(idx), Some(kd)) = (edge_explorer_idx, kind_detail) {
        detail::edge_explorer_lines(kd, idx)
    } else if let Some(kd) = kind_detail {
        detail::styled_lines(kd)
    } else {
        lines
            .iter()
            .map(|l| {
                Line::from(Span::styled(
                    l.as_str(),
                    Style::default().fg(theme::NOVA_WHITE),
                ))
            })
            .collect()
    };

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_cypher_preview(
    frame: &mut Frame,
    area: Rect,
    lines: &[String],
    active: bool,
    pulse: Option<&NebulaPulse>,
) {
    let border = theme::panel_border_with_pulse(active, pulse);
    let block = Block::default()
        .title(Span::styled(
            " Cypher ",
            theme::accent_bold(theme::MATRIX_GREEN),
        ))
        .borders(Borders::ALL)
        .border_style(border)
        .style(Style::default().bg(theme::BG_PANEL));

    let text: Vec<Line> = if lines.is_empty() {
        vec![Line::from(Span::styled(
            "Select a Kind to preview Cypher.",
            theme::dim_style(),
        ))]
    } else {
        lines.iter().map(|l| highlight_cypher_line(l)).collect()
    };

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_dashboard_panel(frame: &mut Frame, area: Rect, stats: &DashboardStats) {
    let block = Block::default()
        .title(Span::styled(
            " Dashboard ",
            theme::accent_bold(theme::SOLAR_AMBER),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::SOLAR_AMBER))
        .style(Style::default().bg(theme::BG_PANEL));

    let inner_width = area.width.saturating_sub(4) as usize;
    let bar_width = inner_width.saturating_sub(20).max(5);
    let mut lines: Vec<Line> = Vec::new();

    // Section: Realms with bar chart
    lines.push(Line::from(Span::styled(
        " Kinds by Realm",
        Style::default()
            .fg(theme::NOVA_WHITE)
            .add_modifier(Modifier::BOLD),
    )));
    let max_realm = stats.max_realm_count();
    for rc in &stats.realm_counts {
        let color = theme::realm_color(&rc.key);
        let bar_str = dash::bar(rc.count, max_realm, bar_width);
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {:>10} ", rc.display_name),
                Style::default().fg(color),
            ),
            Span::styled(bar_str, Style::default().fg(color)),
            Span::styled(
                format!(" {}", rc.count),
                Style::default().fg(theme::STAR_DIM),
            ),
        ]));
    }

    // Section: Edge families
    if !stats.family_counts.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " Edges by Family",
            Style::default()
                .fg(theme::NOVA_WHITE)
                .add_modifier(Modifier::BOLD),
        )));
        let max_fam = stats.max_family_count();
        for fc in &stats.family_counts {
            let color = theme::family_color(&fc.key);
            let bar_str = dash::bar(fc.count, max_fam, bar_width);
            lines.push(Line::from(vec![
                Span::styled(format!(" {:>10} ", fc.key), Style::default().fg(color)),
                Span::styled(bar_str, Style::default().fg(color)),
                Span::styled(
                    format!(" {}", fc.count),
                    Style::default().fg(theme::STAR_DIM),
                ),
            ]));
        }
    }

    // Totals line
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" Totals: ", Style::default().fg(theme::STAR_DIM)),
        Span::styled(
            format!("{}", stats.total_nodes),
            Style::default()
                .fg(theme::MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" nodes  ", Style::default().fg(theme::STAR_DIM)),
        Span::styled(
            format!("{}", stats.total_edges),
            Style::default()
                .fg(theme::CYBER_CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" edges", Style::default().fg(theme::STAR_DIM)),
    ]));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

/// Simple Cypher syntax highlighting for the preview pane.
///
/// Colors: keywords → CYBER_CYAN, labels (:Foo) → NEBULA_PURPLE,
/// strings → MATRIX_GREEN, properties/rest → NOVA_WHITE.
fn highlight_cypher_line(line: &str) -> Line<'static> {
    const KEYWORDS: &[&str] = &[
        "MATCH", "WHERE", "RETURN", "CREATE", "DELETE", "SET", "REMOVE", "MERGE", "WITH", "UNWIND",
        "OPTIONAL", "UNION", "ALL", "ORDER", "BY", "LIMIT", "SKIP", "AS", "AND", "OR", "NOT", "IN",
        "IS", "NULL", "TRUE", "FALSE", "DISTINCT", "CASE", "WHEN", "THEN", "ELSE", "END", "CALL",
        "YIELD", "DETACH",
    ];

    let keyword_style = Style::default()
        .fg(theme::CYBER_CYAN)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(theme::NEBULA_PURPLE);
    let string_style = Style::default().fg(theme::MATRIX_GREEN);
    let default_style = Style::default().fg(theme::NOVA_WHITE);

    let mut spans: Vec<Span<'static>> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // String literals (single or double quoted)
        if chars[i] == '\'' || chars[i] == '"' {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len && chars[i] != quote {
                if chars[i] == '\\' {
                    i += 1; // skip escaped char
                }
                i += 1;
            }
            if i < len {
                i += 1; // closing quote
            }
            let s: String = chars[start..i].iter().collect();
            spans.push(Span::styled(s, string_style));
            continue;
        }

        // Labels (:Word)
        if chars[i] == ':' && i + 1 < len && chars[i + 1].is_alphabetic() {
            let start = i;
            i += 1;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            spans.push(Span::styled(s, label_style));
            continue;
        }

        // Words (check if keyword)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            if KEYWORDS.contains(&word.to_uppercase().as_str()) {
                spans.push(Span::styled(word, keyword_style));
            } else {
                spans.push(Span::styled(word, default_style));
            }
            continue;
        }

        // Everything else (operators, whitespace, brackets)
        let start = i;
        while i < len
            && !chars[i].is_alphabetic()
            && chars[i] != '_'
            && chars[i] != ':'
            && chars[i] != '\''
            && chars[i] != '"'
        {
            i += 1;
        }
        let s: String = chars[start..i].iter().collect();
        spans.push(Span::styled(s, default_style));
    }

    Line::from(spans)
}

fn render_dialog(frame: &mut Frame, area: Rect, dlg: &DialogState) {
    // Calculate popup dimensions based on content
    let is_delete = matches!(
        dlg.kind,
        DialogKind::DeleteNode { .. } | DialogKind::DeleteRelation { .. }
    );
    let field_count = dlg.fields.len();
    let cypher_line_count = dlg.cypher_preview.lines().count();

    // Height: title(1) + border(2) + fields + spacing + cypher preview + footer
    let content_height = if is_delete {
        // Compact: warning + field + cypher + footer
        3 + field_count as u16 * 2 + cypher_line_count as u16 + 3
    } else {
        // Full: fields + separator + cypher + footer
        2 + field_count as u16 * 2 + 1 + cypher_line_count as u16 + 3
    };
    let popup_height = content_height.min(area.height.saturating_sub(4));
    let popup_width = 60u16.min(area.width.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Clear area behind popup
    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let border_color = if dlg.submitting {
        theme::SOLAR_AMBER
    } else {
        theme::NEBULA_PURPLE
    };

    let title = if dlg.submitting {
        format!(" {} (submitting...) ", dlg.kind.title())
    } else {
        format!(" {} ", dlg.kind.title())
    };

    let block = Block::default()
        .title(Span::styled(title, theme::accent_bold(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::BG_PANEL));

    let inner_width = popup_width.saturating_sub(2) as usize;
    let mut lines: Vec<Line> = Vec::new();

    // Delete dialogs: show warning first
    if is_delete {
        lines.push(Line::from(""));
        let warning_text = match &dlg.kind {
            DialogKind::DeleteNode { display_name, key } => {
                format!("  \u{26a0} Delete \"{display_name}\" ({key})?")
            }
            DialogKind::DeleteRelation {
                from_key,
                to_key,
                rel_type,
            } => format!("  \u{26a0} Delete ({from_key})-[:{rel_type}]->({to_key})?"),
            _ => String::new(),
        };
        lines.push(Line::from(Span::styled(
            warning_text,
            Style::default()
                .fg(theme::PLASMA_PINK)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        if matches!(dlg.kind, DialogKind::DeleteNode { .. }) {
            lines.push(Line::from(Span::styled(
                "  This will DETACH DELETE the node and ALL",
                Style::default().fg(theme::NOVA_WHITE),
            )));
            lines.push(Line::from(Span::styled(
                "  its relationships. This cannot be undone.",
                Style::default().fg(theme::NOVA_WHITE),
            )));
            lines.push(Line::from(""));
        }
    }

    // Render form fields
    let label_width = dlg.fields.iter().map(|f| f.label.len()).max().unwrap_or(10);

    for (i, field) in dlg.fields.iter().enumerate() {
        let is_focused = i == dlg.focused;
        let label_style = if is_focused {
            Style::default()
                .fg(theme::CYBER_CYAN)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::STAR_DIM)
        };

        let padded_label = format!("  {:>width$}: ", field.label, width = label_width);
        let value_max = inner_width
            .saturating_sub(padded_label.len())
            .saturating_sub(2);

        match field.field_kind {
            FieldKind::Readonly => {
                lines.push(Line::from(vec![
                    Span::styled(padded_label, label_style),
                    Span::styled(
                        truncate_str(&field.value, value_max),
                        Style::default().fg(theme::STAR_DIM),
                    ),
                ]));
            }
            FieldKind::Dropdown => {
                let arrow = if is_focused { "\u{25b8} " } else { "  " }; // ▸ or space
                let suffix = if is_focused { " \u{25be}" } else { "" }; // ▾
                lines.push(Line::from(vec![
                    Span::styled(padded_label, label_style),
                    Span::styled(arrow.to_string(), Style::default().fg(theme::CYBER_CYAN)),
                    Span::styled(
                        truncate_str(&field.value, value_max.saturating_sub(4)),
                        if is_focused {
                            Style::default()
                                .fg(theme::NOVA_WHITE)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(theme::NOVA_WHITE)
                        },
                    ),
                    Span::styled(suffix.to_string(), Style::default().fg(theme::STAR_DIM)),
                ]));
            }
            FieldKind::Text => {
                let display_value = if is_focused && !dlg.submitting {
                    // Show cursor as underscore
                    let (before, after) = field.value.split_at(field.cursor.min(field.value.len()));
                    format!("{before}\u{2502}{after}") // │ cursor
                } else {
                    field.value.clone()
                };
                let value_style = Style::default().fg(theme::NOVA_WHITE);
                lines.push(Line::from(vec![
                    Span::styled(padded_label, label_style),
                    Span::styled(truncate_str(&display_value, value_max), value_style),
                ]));
            }
        }

        // Show field error
        if let Some(err) = &field.error {
            let indent = " ".repeat(label_width + 4);
            lines.push(Line::from(Span::styled(
                format!("{indent}{err}"),
                Style::default().fg(theme::PLASMA_PINK),
            )));
        }
    }

    // Separator + Cypher preview
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(
            "  {}",
            "\u{2504}".repeat(inner_width.saturating_sub(4)) // ┄ dashed separator
        ),
        Style::default().fg(theme::STAR_DIM),
    )));
    for cypher_line in dlg.cypher_preview.lines() {
        let mut highlighted = highlight_cypher_line(cypher_line);
        highlighted.spans.insert(0, Span::raw("  "));
        lines.push(highlighted);
    }
    lines.push(Line::from(Span::styled(
        format!("  {}", "\u{2504}".repeat(inner_width.saturating_sub(4))),
        Style::default().fg(theme::STAR_DIM),
    )));

    // Submission error
    if let Some(err) = &dlg.error {
        lines.push(Line::from(Span::styled(
            format!("  Error: {err}"),
            Style::default()
                .fg(theme::PLASMA_PINK)
                .add_modifier(Modifier::BOLD),
        )));
    }

    // Footer key hints
    let submit_style = if !dlg.submitting {
        Style::default()
            .fg(theme::MATRIX_GREEN)
            .add_modifier(Modifier::BOLD)
    } else {
        theme::dim_style()
    };

    let footer = if is_delete {
        Line::from(vec![
            Span::styled("  Enter", submit_style),
            Span::styled(": confirm  ", theme::dim_style()),
            Span::styled("Esc", Style::default().fg(theme::STAR_DIM)),
            Span::styled(": cancel", theme::dim_style()),
        ])
    } else {
        Line::from(vec![
            Span::styled("  Tab", Style::default().fg(theme::STAR_DIM)),
            Span::styled(": next  ", theme::dim_style()),
            Span::styled("Ctrl+S", submit_style),
            Span::styled(": submit  ", theme::dim_style()),
            Span::styled("Esc", Style::default().fg(theme::STAR_DIM)),
            Span::styled(": cancel", theme::dim_style()),
        ])
    };
    lines.push(footer);

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

/// Truncate a string to fit within max_width characters.
fn truncate_str(s: &str, max_width: usize) -> String {
    if s.len() <= max_width {
        s.to_string()
    } else if max_width > 3 {
        format!("{}...", &s[..max_width - 3])
    } else {
        s[..max_width].to_string()
    }
}

fn render_search_overlay(frame: &mut Frame, area: Rect, search: &SearchState) {
    let popup_width = 60.min(area.width.saturating_sub(4));
    let popup_height = 20.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Clear area behind popup
    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let block = Block::default()
        .title(Span::styled(
            " Search (/) ",
            theme::accent_bold(theme::CYBER_CYAN),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::CYBER_CYAN))
        .style(Style::default().bg(theme::BG_PANEL));

    let mut lines: Vec<Line> = vec![
        // Query input line
        Line::from(vec![
            Span::styled(
                " > ",
                Style::default()
                    .fg(theme::CYBER_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}_", search.query),
                Style::default().fg(theme::NOVA_WHITE),
            ),
        ]),
        Line::from(Span::styled(
            "\u{2500}".repeat(popup_width.saturating_sub(2) as usize), // ─ separator
            Style::default().fg(theme::STAR_DIM),
        )),
    ];

    if search.results.is_empty() && !search.query.is_empty() {
        lines.push(Line::from(Span::styled("  No matches", theme::dim_style())));
    } else {
        for (i, result) in search.results.iter().enumerate() {
            let is_selected = i == search.cursor;
            let indicator = if is_selected { "\u{25b6} " } else { "  " }; // ▶ or space
            let name_style = if is_selected {
                theme::selected_style(theme::NOVA_WHITE)
            } else {
                Style::default().fg(theme::NOVA_WHITE)
            };
            let ctx_style = Style::default().fg(theme::STAR_DIM);

            lines.push(Line::from(vec![
                Span::styled(
                    indicator.to_string(),
                    Style::default().fg(if is_selected {
                        theme::CYBER_CYAN
                    } else {
                        theme::BG_PANEL
                    }),
                ),
                Span::styled(result.display_name.clone(), name_style),
                Span::styled(format!("  {} / {}", result.realm, result.layer), ctx_style),
            ]));
        }
    }

    // Footer: match count
    if !search.query.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!(
                "  {} match{}",
                search.results.len(),
                if search.results.len() == 1 { "" } else { "es" }
            ),
            theme::dim_style(),
        )));
    }

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

fn render_palette_overlay(frame: &mut Frame, area: Rect, palette: &PaletteState) {
    let popup_width = 64u16.min(area.width.saturating_sub(4));
    let popup_height = 22u16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Clear area behind popup
    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let block = Block::default()
        .title(Span::styled(
            " Command Palette (:) ",
            theme::accent_bold(theme::CYBER_CYAN),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::CYBER_CYAN))
        .style(Style::default().bg(theme::BG_PANEL));

    let inner_width = popup_width.saturating_sub(2) as usize;
    let mut lines: Vec<Line> = vec![
        // Query input line
        Line::from(vec![
            Span::styled(
                " : ",
                Style::default()
                    .fg(theme::CYBER_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}_", palette.query),
                Style::default().fg(theme::NOVA_WHITE),
            ),
        ]),
        Line::from(Span::styled(
            "\u{2500}".repeat(inner_width), // ─ separator
            Style::default().fg(theme::STAR_DIM),
        )),
    ];

    if palette.results.is_empty() && !palette.query.is_empty() {
        lines.push(Line::from(Span::styled("  No matches", theme::dim_style())));
    } else {
        let max_visible = (popup_height.saturating_sub(5)) as usize;
        for (i, _) in palette.results.iter().enumerate().take(max_visible) {
            let is_selected = i == palette.cursor;
            let cmd = match palette.command_at(i) {
                Some(c) => c,
                None => continue,
            };

            let indicator = if is_selected { "\u{25b6} " } else { "  " }; // ▶
            let shortcut_str = cmd.shortcut.map_or(String::new(), |s| format!("[{s}]"));
            let category = cmd.category.label();

            let label_style = if is_selected {
                theme::selected_style(theme::NOVA_WHITE)
            } else {
                Style::default().fg(theme::NOVA_WHITE)
            };

            lines.push(Line::from(vec![
                Span::styled(
                    indicator.to_string(),
                    Style::default().fg(if is_selected {
                        theme::CYBER_CYAN
                    } else {
                        theme::BG_PANEL
                    }),
                ),
                Span::styled(format!("{:<16}", cmd.label), label_style),
                Span::styled(
                    format!("{:<5} ", shortcut_str),
                    Style::default().fg(theme::STAR_DIM),
                ),
                Span::styled(cmd.description.to_string(), theme::dim_style()),
                Span::styled(
                    format!("  {category}"),
                    Style::default().fg(theme::STAR_DIM),
                ),
            ]));
        }
    }

    // Footer: match count
    if !palette.query.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!(
                "  {} match{}",
                palette.results.len(),
                if palette.results.len() == 1 { "" } else { "es" }
            ),
            theme::dim_style(),
        )));
    }

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

fn render_help_overlay(frame: &mut Frame, area: Rect) {
    let popup_width = 58u16.min(area.width.saturating_sub(4));
    let popup_height = 30u16.min(area.height.saturating_sub(2));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Clear area behind popup
    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let block = Block::default()
        .title(Span::styled(
            " Keyboard Reference ",
            theme::accent_bold(theme::SOLAR_AMBER),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::SOLAR_AMBER))
        .style(Style::default().bg(theme::BG_PANEL));

    let section = |title: &str| -> Line<'static> {
        Line::from(Span::styled(
            format!(" {title}"),
            Style::default()
                .fg(theme::CYBER_CYAN)
                .add_modifier(Modifier::BOLD),
        ))
    };

    let binding = |key: &str, desc: &str| -> Line<'static> {
        Line::from(vec![
            Span::styled(
                format!("   {key:<12}"),
                Style::default()
                    .fg(theme::NOVA_WHITE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(desc.to_string(), Style::default().fg(theme::STAR_DIM)),
        ])
    };

    let lines = vec![
        Line::from(""),
        section("Navigation"),
        binding("1-4", "Switch mode (Data/Meta/Overlay/Query)"),
        binding("Tab", "Cycle to next mode"),
        binding("Left/Right", "Cycle panel (Tree/Detail/Cypher)"),
        binding("Up/Down", "Navigate tree or edges"),
        binding("Enter", "Expand/collapse tree node"),
        Line::from(""),
        section("Search & Commands"),
        binding("/", "Open fuzzy search"),
        binding(":", "Open command palette"),
        binding("Esc", "Close overlay"),
        Line::from(""),
        section("CRUD"),
        binding("n", "Create node"),
        binding("r", "Create relation"),
        binding("E / F2", "Edit node (Phase 7C)"),
        binding("d", "Delete node (Phase 7C)"),
        Line::from(""),
        section("Panels"),
        binding("e", "Toggle edge explorer"),
        binding("s", "Toggle dashboard"),
        binding("f", "Toggle facet filter (Query mode)"),
        Line::from(""),
        section("System"),
        binding("?", "Show/close this help"),
        binding("Ctrl+R", "Toggle CRT scanlines"),
        binding("q", "Quit"),
        binding("Ctrl+C", "Force quit"),
        Line::from(""),
        Line::from(Span::styled(" Press ? or Esc to close", theme::dim_style())),
    ];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    mode: NavMode,
    status: &str,
    node_count: usize,
) {
    let mode_text = format!(" {} ", mode.label());
    let status_text = format!(" {status} ");
    let right_text = format!(" {node_count} nodes | q:quit ?:help ::cmd ");

    let used = mode_text.len() + status_text.len() + right_text.len();
    let padding = (area.width as usize).saturating_sub(used);

    let line = Line::from(vec![
        Span::styled(
            mode_text,
            Style::default()
                .fg(theme::BG_VOID)
                .bg(theme::mode_color(mode))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(status_text, Style::default().fg(theme::NOVA_WHITE)),
        Span::raw(" ".repeat(padding)),
        Span::styled(right_text, Style::default().fg(theme::STAR_DIM)),
    ]);

    let bar = Paragraph::new(line).style(theme::status_bar_style());
    frame.render_widget(bar, area);
}

fn render_facet_popup(frame: &mut Frame, area: Rect) {
    let popup_width = 50.min(area.width.saturating_sub(4));
    let popup_height = 15.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    let block = Block::default()
        .title(Span::styled(
            " Facet Filters (Esc to close) ",
            theme::accent_bold(theme::MATRIX_GREEN),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::MATRIX_GREEN))
        .style(Style::default().bg(theme::BG_PANEL));

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Realms: global, project, shared",
            Style::default().fg(theme::REALM_GLOBAL),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Layers: knowledge, structure, ...",
            Style::default().fg(theme::NEBULA_BLUE),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Traits: invariant, localized, ...",
            Style::default().fg(theme::NEBULA_PURPLE),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  (Interactive selection coming soon)",
            theme::dim_style(),
        )),
    ];

    // Clear area behind popup
    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, popup_area);
}

fn render_onboarding_overlay(frame: &mut Frame, area: Rect, ob: &OnboardingState) {
    match ob {
        OnboardingState::Welcome { checks } => render_welcome_overlay(frame, area, checks),
        OnboardingState::Tour { step } => render_tour_overlay(frame, area, *step),
    }
}

fn render_welcome_overlay(frame: &mut Frame, area: Rect, checks: &[onboarding::CheckResult]) {
    let popup_width = 56u16.min(area.width.saturating_sub(4));
    let popup_height = 18u16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let block = Block::default()
        .title(Span::styled(
            " Welcome to NovaNet ",
            theme::accent_bold(theme::NEBULA_PURPLE),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::NEBULA_PURPLE))
        .style(Style::default().bg(theme::BG_PANEL));

    // Compact logo
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    for logo_line in logo::COMPACT_LOGO {
        lines.push(Line::from(vec![Span::styled(
            format!("  {logo_line}"),
            Style::default().fg(theme::NEBULA_PURPLE),
        )]));
    }
    lines.push(Line::from(""));

    // Health checks
    lines.push(Line::from(Span::styled(
        "  System Checks:",
        Style::default()
            .fg(theme::CYBER_CYAN)
            .add_modifier(Modifier::BOLD),
    )));

    for check in checks {
        let (icon, color) = match &check.status {
            onboarding::CheckStatus::Checking => ("\u{25cb}", theme::STAR_DIM), // ○
            onboarding::CheckStatus::Ok(_) => ("\u{25cf}", theme::MATRIX_GREEN), // ●
            onboarding::CheckStatus::Failed(_) => ("\u{25cf}", theme::PLASMA_PINK), // ●
        };
        let detail = match &check.status {
            onboarding::CheckStatus::Checking => "checking...".to_string(),
            onboarding::CheckStatus::Ok(msg) => msg.clone(),
            onboarding::CheckStatus::Failed(msg) => msg.clone(),
        };
        lines.push(Line::from(vec![
            Span::styled(format!("    {icon} "), Style::default().fg(color)),
            Span::styled(
                format!("{:<20}", check.label),
                Style::default().fg(theme::NOVA_WHITE),
            ),
            Span::styled(detail, Style::default().fg(color)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Enter", theme::accent_bold(theme::MATRIX_GREEN)),
        Span::styled(": Start tour   ", theme::dim_style()),
        Span::styled("Esc", Style::default().fg(theme::STAR_DIM)),
        Span::styled(": Skip to dashboard", theme::dim_style()),
    ]));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

fn render_tour_overlay(frame: &mut Frame, area: Rect, step: usize) {
    let tour_step = match onboarding::TOUR_STEPS.get(step) {
        Some(s) => s,
        None => return,
    };

    // Tooltip popup positioned near the target panel
    let popup_width = 52u16.min(area.width.saturating_sub(4));
    let popup_height = (tour_step.body.len() as u16 + 6).min(area.height.saturating_sub(4));

    // Position based on target
    let (x, y) = match tour_step.target {
        TourTarget::TreePanel => (2, area.height / 4),
        TourTarget::DetailPanel => (area.width.saturating_sub(popup_width + 2), area.height / 6),
        TourTarget::CypherPanel => (area.width.saturating_sub(popup_width + 2), area.height / 2),
        TourTarget::Dashboard => (area.width.saturating_sub(popup_width + 2), area.height / 3),
        TourTarget::StatusBar => (
            (area.width.saturating_sub(popup_width)) / 2,
            area.height.saturating_sub(popup_height + 2),
        ),
    };

    let popup_area = Rect::new(
        x.min(area.width.saturating_sub(popup_width)),
        y.min(area.height.saturating_sub(popup_height)),
        popup_width,
        popup_height,
    );

    let clear = Block::default().style(Style::default().bg(theme::BG_VOID));
    frame.render_widget(clear, popup_area);

    let step_indicator = format!(" Tour ({}/{}) ", step + 1, onboarding::TOUR_STEP_COUNT);
    let block = Block::default()
        .title(Span::styled(
            step_indicator,
            theme::accent_bold(theme::CYBER_CYAN),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::CYBER_CYAN))
        .style(Style::default().bg(theme::BG_PANEL));

    let mut lines: Vec<Line> = Vec::new();

    // Step title
    lines.push(Line::from(Span::styled(
        format!("  {}", tour_step.title),
        Style::default()
            .fg(theme::NEBULA_PURPLE)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Body text
    for body_line in tour_step.body {
        lines.push(Line::from(Span::styled(
            format!("  {body_line}"),
            Style::default().fg(theme::NOVA_WHITE),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {}", tour_step.hint),
            Style::default().fg(theme::STAR_DIM),
        ),
        Span::styled("  Esc: Skip", theme::dim_style()),
    ]));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_text(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    fn span_texts(line: &Line) -> Vec<(String, ratatui::style::Color)> {
        line.spans
            .iter()
            .map(|s| {
                (
                    s.content.to_string(),
                    s.style.fg.unwrap_or(ratatui::style::Color::Reset),
                )
            })
            .collect()
    }

    #[test]
    fn highlight_keyword_bold_cyan() {
        let line = highlight_cypher_line("MATCH (n)");
        let parts = span_texts(&line);
        assert_eq!(parts[0].0, "MATCH");
        assert_eq!(parts[0].1, theme::CYBER_CYAN);
    }

    #[test]
    fn highlight_label_purple() {
        let line = highlight_cypher_line("(n:Page)");
        let parts = span_texts(&line);
        let label = parts.iter().find(|(t, _)| t == ":Page");
        assert!(label.is_some());
        assert_eq!(label.unwrap().1, theme::NEBULA_PURPLE);
    }

    #[test]
    fn highlight_string_green() {
        let line = highlight_cypher_line("WHERE n.key = 'hello'");
        let parts = span_texts(&line);
        let string = parts.iter().find(|(t, _)| t == "'hello'");
        assert!(string.is_some());
        assert_eq!(string.unwrap().1, theme::MATRIX_GREEN);
    }

    #[test]
    fn highlight_case_insensitive_keywords() {
        let line = highlight_cypher_line("match return");
        let parts = span_texts(&line);
        assert_eq!(parts[0].0, "match");
        assert_eq!(parts[0].1, theme::CYBER_CYAN);
    }

    #[test]
    fn highlight_empty_line() {
        let line = highlight_cypher_line("");
        assert!(line.spans.is_empty());
    }

    #[test]
    fn highlight_preserves_full_text() {
        let input = "MATCH (n:Kind)-[:IN_REALM]->(r:Realm)";
        let line = highlight_cypher_line(input);
        let reconstructed = span_text(&line);
        assert_eq!(reconstructed, input);
    }

    #[test]
    fn highlight_double_quoted_string() {
        let line = highlight_cypher_line("n.name = \"test\"");
        let parts = span_texts(&line);
        let string = parts.iter().find(|(t, _)| t == "\"test\"");
        assert!(string.is_some());
        assert_eq!(string.unwrap().1, theme::MATRIX_GREEN);
    }
}
