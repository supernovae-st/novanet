//! Traits Tab — Constellation view showing 5 traits connected.
//!
//! The constellation shows the relationship between traits:
//! - KNOWLEDGE at top (input to generation)
//! - INVARIANT and LOCALIZED as core pair (structure -> output)
//! - DERIVED and JOB at bottom (computed and async)

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::data::TaxonomyTree;
use crate::tui::theme::Theme;

// =============================================================================
// TRAIT STATS
// =============================================================================

/// Statistics for a single trait.
#[derive(Debug, Clone)]
pub struct TraitStats {
    /// Trait key (e.g., "invariant", "localized").
    pub key: String,
    /// Display name (e.g., "Invariant", "Localized").
    pub display_name: String,
    /// Unicode symbol for the trait.
    pub symbol: &'static str,
    /// Number of kinds with this trait.
    pub kind_count: usize,
    /// LLM context description.
    pub llm_context: String,
    /// Kinds grouped by layer for this trait.
    pub kinds_by_layer: Vec<(String, Vec<String>)>,
}

/// Canonical trait order for constellation.
pub const TRAIT_ORDER: [&str; 5] = ["invariant", "localized", "knowledge", "derived", "job"];

/// Get symbol for a trait.
fn trait_symbol(key: &str) -> &'static str {
    match key {
        "invariant" => "\u{25a0}", // ■
        "localized" => "\u{25a1}", // □
        "knowledge" => "\u{25ca}", // ◊
        "derived" => "\u{2550}",   // ═
        "job" => "\u{25cb}",       // ○
        _ => "\u{00b7}",           // ·
    }
}

/// Get display name for a trait.
fn trait_display_name(key: &str) -> &str {
    match key {
        "invariant" => "INVARIANT",
        "localized" => "LOCALIZED",
        "knowledge" => "KNOWLEDGE",
        "derived" => "DERIVED",
        "job" => "JOB",
        _ => key,
    }
}

/// Get LLM context description for a trait.
fn trait_llm_context(key: &str) -> &str {
    match key {
        "invariant" => {
            "Nodes that do not change between locales. Structural definitions, configuration, and invariant business logic. Examples: Page, Entity, Block."
        }
        "localized" => {
            "OUTPUT - Generated content per locale. Has invariant parent (e.g., PageL10n -> Page). Created by LLM generation, not translation."
        }
        "knowledge" => {
            "INPUT - Native locale knowledge (savoir). Loaded INTO the LLM as context. Exists only where needed (fr-FR may have 20K Terms, sw-KE may have 500)."
        }
        "derived" => {
            "Computed/aggregated data. Metrics, statistics, and cached computations derived from other nodes."
        }
        "job" => {
            "Background tasks and async processes. Generation jobs, queue items, and batch operations."
        }
        _ => "Unknown trait.",
    }
}

impl TaxonomyTree {
    /// Build trait statistics from the loaded taxonomy tree.
    pub fn get_trait_stats(&self) -> Vec<TraitStats> {
        let mut stats_map: std::collections::HashMap<String, TraitStats> =
            std::collections::HashMap::new();

        // Initialize all traits in canonical order
        for &trait_key in &TRAIT_ORDER {
            stats_map.insert(
                trait_key.to_string(),
                TraitStats {
                    key: trait_key.to_string(),
                    display_name: trait_display_name(trait_key).to_string(),
                    symbol: trait_symbol(trait_key),
                    kind_count: 0,
                    llm_context: trait_llm_context(trait_key).to_string(),
                    kinds_by_layer: Vec::new(),
                },
            );
        }

        // Collect kinds by trait and layer
        for realm in &self.realms {
            for layer in &realm.layers {
                for kind in &layer.kinds {
                    let trait_key = kind.trait_name.as_str();
                    if let Some(stats) = stats_map.get_mut(trait_key) {
                        stats.kind_count += 1;

                        // Find or create layer group
                        let layer_key = &layer.key;
                        if let Some(layer_group) = stats
                            .kinds_by_layer
                            .iter_mut()
                            .find(|(k, _)| k == layer_key)
                        {
                            layer_group.1.push(kind.key.clone());
                        } else {
                            stats
                                .kinds_by_layer
                                .push((layer_key.clone(), vec![kind.key.clone()]));
                        }
                    }
                }
            }
        }

        // Return in canonical order
        TRAIT_ORDER
            .iter()
            .filter_map(|&key| stats_map.remove(key))
            .collect()
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Traits tab with constellation and detail panel.
/// When drilled down (drill_depth > 0), shows kind list instead of constellation.
pub fn render_traits_tab(f: &mut Frame, app: &App, area: Rect) {
    if app.guide.drill_depth > 0 {
        // Drilled mode: show kind list on left, detail on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        render_kind_list(f, app, chunks[0]);
        render_kind_detail(f, app, chunks[1]);
    } else {
        // Overview mode: constellation and detail panel
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(area);

        render_constellation(f, app, chunks[0]);
        render_detail_panel(f, app, chunks[1]);
    }
}

/// Render the constellation view showing 5 traits connected.
fn render_constellation(f: &mut Frame, app: &App, area: Rect) {
    let trait_stats = app.tree.get_trait_stats();
    let selected_idx = app.guide.trait_cursor;
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " CONSTELLATION ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 70)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Build constellation lines
    let lines = build_constellation_lines(&trait_stats, selected_idx, theme, inner.width as usize);

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Build the ASCII constellation layout.
fn build_constellation_lines(
    stats: &[TraitStats],
    selected_idx: usize,
    theme: &Theme,
    width: usize,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // Get stats by trait key for easier lookup
    let get_stat = |key: &str| -> Option<&TraitStats> { stats.iter().find(|s| s.key == key) };

    // Helper to create styled trait span
    let trait_span = |key: &str, idx: usize| -> Vec<Span<'static>> {
        let stat = get_stat(key);
        let symbol = trait_symbol(key);
        let name = trait_display_name(key);
        let count = stat.map(|s| s.kind_count).unwrap_or(0);

        let is_selected = idx == selected_idx;
        let base_color = theme.trait_color(key);
        let style = if is_selected {
            Style::default()
                .fg(base_color)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(base_color)
        };

        vec![
            Span::styled(format!("{} ", symbol), style),
            Span::styled(name.to_string(), style),
            Span::styled(
                format!(" ({} K)", count),
                Style::default().fg(Color::DarkGray),
            ),
        ]
    };

    // Empty line for spacing
    lines.push(Line::from(""));

    // Row 1: KNOWLEDGE at top center
    let knowledge_spans = trait_span("knowledge", 2);
    let knowledge_line = center_spans(knowledge_spans, width);
    lines.push(knowledge_line);
    lines.push(Line::from(""));

    // Row 2: Connection lines from KNOWLEDGE
    let connector1 = center_text("\u{2571}    \u{2572}", width); // ╱    ╲
    lines.push(Line::from(connector1));
    let connector2 = center_text("\u{2571}      \u{2572}", width); // ╱      ╲
    lines.push(Line::from(connector2));

    // Row 3: INVARIANT ════════════════════ LOCALIZED (core pair)
    let mut core_pair: Vec<Span<'static>> = Vec::new();
    core_pair.extend(trait_span("invariant", 0));
    core_pair.push(Span::styled(
        " \u{2550}\u{2550}\u{2550}\u{2550}\u{21d4}\u{2550}\u{2550}\u{2550}\u{2550} ",
        Style::default().fg(Color::Yellow),
    )); // ════↔════
    core_pair.extend(trait_span("localized", 1));
    let core_line = center_spans(core_pair, width);
    lines.push(core_line);

    // Row 4: Connection lines to DERIVED and JOB
    let connector3 = center_text("\u{2572}      \u{2571}", width); // ╲      ╱
    lines.push(connector3.into());
    let connector4 = center_text("\u{2572}    \u{2571}", width); // ╲    ╱
    lines.push(connector4.into());

    // Row 5: DERIVED and JOB at bottom
    let mut bottom_pair: Vec<Span<'static>> = Vec::new();
    bottom_pair.extend(trait_span("derived", 3));
    bottom_pair.push(Span::raw("     "));
    bottom_pair.extend(trait_span("job", 4));
    let bottom_line = center_spans(bottom_pair, width);
    lines.push(bottom_line);

    lines.push(Line::from(""));

    // Separator
    lines.push(Line::from(Span::styled(
        "\u{2500}".repeat(width.saturating_sub(2)),
        Style::default().fg(Color::Rgb(60, 60, 70)),
    )));
    lines.push(Line::from(""));

    // Selection list below constellation
    for (idx, stat) in stats.iter().enumerate() {
        let is_selected = idx == selected_idx;
        let prefix = if is_selected { "[\u{25cf}]" } else { "[ ]" }; // [●] or [ ]
        let color = theme.trait_color(&stat.key);

        let style = if is_selected {
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD)
                .bg(Color::Rgb(30, 40, 50))
        } else {
            Style::default().fg(color)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", prefix), style),
            Span::styled(stat.symbol.to_string(), style),
            Span::raw(" "),
            Span::styled(stat.display_name.clone(), style),
        ]));
    }

    lines
}

/// Center spans within a given width.
fn center_spans(spans: Vec<Span<'static>>, width: usize) -> Line<'static> {
    let content_len: usize = spans.iter().map(|s| s.content.chars().count()).sum();
    let padding = width.saturating_sub(content_len) / 2;
    let mut centered: Vec<Span<'static>> = vec![Span::raw(" ".repeat(padding))];
    centered.extend(spans);
    Line::from(centered)
}

/// Center text within a given width.
fn center_text(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    let padding = width.saturating_sub(text_len) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

/// Render the detail panel for the selected trait.
fn render_detail_panel(f: &mut Frame, app: &App, area: Rect) {
    let trait_stats = app.tree.get_trait_stats();
    let selected_idx = app.guide.trait_cursor;
    let theme = &app.theme;

    let selected = trait_stats.get(selected_idx);

    let block = Block::default()
        .title(Span::styled(
            " DETAIL ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 70)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let Some(stat) = selected else {
        let empty = Paragraph::new("No trait selected");
        f.render_widget(empty, inner);
        return;
    };

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Trait name with symbol
    let color = theme.trait_color(&stat.key);
    lines.push(Line::from(vec![Span::styled(
        format!("{} {}", stat.symbol, stat.display_name),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )]));

    // Separator
    lines.push(Line::from(Span::styled(
        "\u{2550}".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(color),
    )));
    lines.push(Line::from(""));

    // LLM context description (wrapped)
    let desc_lines = wrap_text(&stat.llm_context, inner.width.saturating_sub(2) as usize);
    for line in desc_lines {
        lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(Color::Rgb(180, 180, 180)),
        )));
    }
    lines.push(Line::from(""));

    // Kinds by layer section
    lines.push(Line::from(Span::styled(
        "\u{250c}\u{2500} BY LAYER \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));

    for (layer_key, kinds) in &stat.kinds_by_layer {
        let layer_color = theme.layer_color(layer_key);
        let kinds_str = kinds.join(", ");
        lines.push(Line::from(vec![
            Span::styled(
                format!("\u{2502} {:<12} ", layer_key),
                Style::default().fg(layer_color),
            ),
            Span::styled(kinds_str, Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(Span::styled(
        "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));
    lines.push(Line::from(""));

    // Pattern section for INVARIANT
    if stat.key == "invariant" {
        lines.push(Line::from(Span::styled(
            "PATTERN:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));

        let invariant_color = theme.trait_color("invariant");
        let localized_color = theme.trait_color("localized");

        // Show invariant -> localized patterns
        // v10.9: Renamed L10n → Generated/Content
        let patterns = [
            ("Page", "PageGenerated"),
            ("Entity", "EntityContent"),
            ("Block", "BlockGenerated"),
        ];

        for (inv, loc) in patterns {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("\u{25a0} {} ", inv),
                    Style::default().fg(invariant_color),
                ),
                Span::styled(
                    "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2192} ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("\u{25a1} {}", loc),
                    Style::default().fg(localized_color),
                ),
            ]));
        }
    }

    // Pattern section for LOCALIZED
    if stat.key == "localized" {
        lines.push(Line::from(Span::styled(
            "RELATIONSHIP:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "Every *L10n node has an invariant parent.",
            Style::default().fg(Color::Rgb(150, 150, 150)),
        )));
        lines.push(Line::from(Span::styled(
            "Generated by LLM, NOT translated.",
            Style::default()
                .fg(Color::Rgb(34, 197, 94))
                .add_modifier(Modifier::ITALIC),
        )));
    }

    // Pattern section for KNOWLEDGE
    if stat.key == "knowledge" {
        lines.push(Line::from(Span::styled(
            "KEY INSIGHT:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "Knowledge nodes are INPUT to generation.",
            Style::default().fg(Color::Rgb(150, 150, 150)),
        )));
        lines.push(Line::from(Span::styled(
            "They exist ONLY where needed (native, not translated).",
            Style::default()
                .fg(Color::Rgb(139, 92, 246))
                .add_modifier(Modifier::ITALIC),
        )));
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

// =============================================================================
// DRILL-DOWN: KIND LIST
// =============================================================================

/// Render the list of kinds for the selected trait (drill-down view).
fn render_kind_list(f: &mut Frame, app: &App, area: Rect) {
    let trait_stats = app.tree.get_trait_stats();
    let selected_trait_idx = app.guide.trait_cursor;
    let theme = &app.theme;

    let trait_name = TRAIT_ORDER.get(selected_trait_idx).unwrap_or(&"");
    let trait_color = theme.trait_color(trait_name);

    let block = Block::default()
        .title(Span::styled(
            format!(" {} KINDS ", trait_display_name(trait_name)),
            Style::default()
                .fg(trait_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 70)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Get flattened kinds list
    let kinds = app.guide.get_trait_kinds(&trait_stats);
    let visible_height = inner.height as usize;

    if kinds.is_empty() {
        let empty_line = Line::from(Span::styled(
            "No kinds with this trait",
            Style::default().fg(Color::DarkGray),
        ));
        let paragraph = Paragraph::new(vec![empty_line]);
        f.render_widget(paragraph, inner);
        return;
    }

    // Calculate scroll window
    let cursor = app.guide.drill_cursor.min(kinds.len().saturating_sub(1));
    let scroll_offset = if cursor < visible_height / 2 {
        0
    } else if cursor >= kinds.len().saturating_sub(visible_height / 2) {
        kinds.len().saturating_sub(visible_height)
    } else {
        cursor.saturating_sub(visible_height / 2)
    };

    let mut lines: Vec<Line> = Vec::new();

    // Header with count
    lines.push(Line::from(vec![
        Span::styled(
            format!("{} kinds with trait ", kinds.len()),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!(
                "{} {}",
                trait_symbol(trait_name),
                trait_display_name(trait_name)
            ),
            Style::default().fg(trait_color),
        ),
    ]));
    lines.push(Line::from(""));

    // Kind list with scroll
    for (idx, (layer_key, kind_key)) in kinds.iter().enumerate().skip(scroll_offset) {
        if lines.len() >= visible_height {
            break;
        }

        let is_selected = idx == cursor;
        let layer_color = theme.layer_color(layer_key);

        let prefix = if is_selected { "▸ " } else { "  " };
        let style = if is_selected {
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(30, 50, 70))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(kind_key.clone(), style),
            Span::styled("  ", Style::default()),
            Span::styled(format!("[{}]", layer_key), Style::default().fg(layer_color)),
        ]));
    }

    // Scroll indicators
    if scroll_offset > 0 {
        lines.insert(
            2,
            Line::from(Span::styled(
                "  ↑ more above...",
                Style::default().fg(Color::DarkGray),
            )),
        );
    }
    if scroll_offset + visible_height < kinds.len() {
        lines.push(Line::from(Span::styled(
            "  ↓ more below...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

/// Render detail panel for the selected kind in drill-down view.
fn render_kind_detail(f: &mut Frame, app: &App, area: Rect) {
    let trait_stats = app.tree.get_trait_stats();
    let kinds = app.guide.get_trait_kinds(&trait_stats);
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " KIND DETAIL ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 70)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let cursor = app.guide.drill_cursor.min(kinds.len().saturating_sub(1));

    let Some((layer_key, kind_key)) = kinds.get(cursor) else {
        let empty = Paragraph::new("Select a kind to see details");
        f.render_widget(empty, inner);
        return;
    };

    // Find the kind in the tree to get full details
    let kind_data = app.tree.realms.iter().find_map(|realm| {
        realm.layers.iter().find_map(|layer| {
            if &layer.key == layer_key {
                layer.kinds.iter().find(|k| &k.key == kind_key).map(|k| {
                    (
                        realm.key.clone(),
                        realm.display_name.clone(),
                        layer.display_name.clone(),
                        k.clone(),
                    )
                })
            } else {
                None
            }
        })
    });

    let Some((realm_key, realm_name, layer_name, kind)) = kind_data else {
        let not_found = Paragraph::new(format!("Kind '{}' not found", kind_key));
        f.render_widget(not_found, inner);
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // Kind name with trait symbol
    let trait_name = TRAIT_ORDER.get(app.guide.trait_cursor).unwrap_or(&"");
    let trait_color = theme.trait_color(trait_name);

    lines.push(Line::from(vec![
        Span::styled(
            format!("{} ", trait_symbol(trait_name)),
            Style::default().fg(trait_color),
        ),
        Span::styled(
            kind.display_name.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    // Separator
    let layer_color = theme.layer_color(layer_key);
    lines.push(Line::from(Span::styled(
        "\u{2550}".repeat(inner.width.saturating_sub(2) as usize),
        Style::default().fg(layer_color),
    )));
    lines.push(Line::from(""));

    // Location
    let realm_color = theme.realm_color(&realm_key);
    lines.push(Line::from(vec![
        Span::styled("Realm:  ", Style::default().fg(Color::DarkGray)),
        Span::styled(realm_name, Style::default().fg(realm_color)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Layer:  ", Style::default().fg(Color::DarkGray)),
        Span::styled(layer_name, Style::default().fg(layer_color)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Trait:  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!(
                "{} {}",
                trait_symbol(trait_name),
                trait_display_name(trait_name)
            ),
            Style::default().fg(trait_color),
        ),
    ]));
    lines.push(Line::from(""));

    // Description (if not empty)
    if !kind.description.is_empty() {
        lines.push(Line::from(Span::styled(
            "Description:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        let wrapped = wrap_text(&kind.description, inner.width.saturating_sub(2) as usize);
        for line in wrapped {
            lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
        }
        lines.push(Line::from(""));
    }

    // Properties summary
    if !kind.properties.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("Properties ({}):", kind.properties.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for prop in kind.properties.iter().take(5) {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(Color::DarkGray)),
                Span::styled(prop.clone(), Style::default().fg(Color::White)),
            ]));
        }
        if kind.properties.len() > 5 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", kind.properties.len() - 5),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Simple text wrapping helper.
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_len = 0;

    for word in text.split_whitespace() {
        let word_len = word.chars().count();

        if current_len + word_len + 1 > width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
            current_len = word_len;
        } else if current_line.is_empty() {
            current_line = word.to_string();
            current_len = word_len;
        } else {
            current_line.push(' ');
            current_line.push_str(word);
            current_len += word_len + 1;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_symbols() {
        assert_eq!(trait_symbol("invariant"), "\u{25a0}");
        assert_eq!(trait_symbol("localized"), "\u{25a1}");
        assert_eq!(trait_symbol("knowledge"), "\u{25ca}");
        assert_eq!(trait_symbol("derived"), "\u{2550}");
        assert_eq!(trait_symbol("job"), "\u{25cb}");
        assert_eq!(trait_symbol("unknown"), "\u{00b7}");
    }

    #[test]
    fn test_trait_display_names() {
        assert_eq!(trait_display_name("invariant"), "INVARIANT");
        assert_eq!(trait_display_name("localized"), "LOCALIZED");
        assert_eq!(trait_display_name("knowledge"), "KNOWLEDGE");
    }

    #[test]
    fn test_trait_order() {
        assert_eq!(TRAIT_ORDER.len(), 5);
        assert_eq!(TRAIT_ORDER[0], "invariant");
        assert_eq!(TRAIT_ORDER[1], "localized");
        assert_eq!(TRAIT_ORDER[2], "knowledge");
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a test string for wrapping";
        let wrapped = wrap_text(text, 15);
        assert!(wrapped.len() > 1);
        for line in &wrapped {
            assert!(line.chars().count() <= 15 || !line.contains(' '));
        }
    }

    #[test]
    fn test_wrap_text_empty() {
        let wrapped = wrap_text("", 20);
        assert!(wrapped.is_empty() || wrapped[0].is_empty());
    }

    #[test]
    fn test_center_text() {
        let centered = center_text("test", 10);
        assert_eq!(centered.len(), 7); // 3 spaces + 4 chars
        assert!(centered.starts_with("   "));
    }
}
