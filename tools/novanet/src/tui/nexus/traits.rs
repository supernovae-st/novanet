//! Traits Tab — Constellation view showing 5 traits connected.
//!
//! Layout:
//! - IMPORTED at top (external knowledge brought in)
//! - DEFINED and AUTHORED as core pair (structure -> output)
//! - GENERATED and RETRIEVED at bottom (LLM output and API snapshots)

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::data::TaxonomyTree;
use crate::tui::theme::{Theme, heatmap_color};
use crate::tui::ui::COLOR_UNFOCUSED_BORDER;

// =============================================================================
// TRAIT STATS
// =============================================================================

/// Statistics for a single trait.
#[derive(Debug, Clone)]
pub struct TraitStats {
    /// Trait key (e.g., "defined", "authored").
    pub key: String,
    /// Display name (e.g., "Defined", "Authored").
    pub display_name: String,
    /// Unicode symbol for the trait.
    pub symbol: &'static str,
    /// Number of classes with this trait.
    pub class_count: usize,
    /// LLM context description.
    pub llm_context: String,
    /// Classes grouped by layer for this trait.
    pub classes_by_layer: Vec<(String, Vec<String>)>,
}

/// Canonical trait order for constellation.
pub const TRAIT_ORDER: [&str; 5] = ["defined", "authored", "imported", "generated", "retrieved"];

/// Get symbol for a trait.
fn trait_symbol(key: &str) -> &'static str {
    match key {
        "defined" => "\u{25a0}",   // ■
        "authored" => "\u{25a1}",  // □
        "imported" => "\u{25ca}",  // ◊
        "generated" => "\u{2605}", // ★
        "retrieved" => "\u{25aa}", // ▪
        _ => "\u{00b7}",           // ·
    }
}

/// Get display name for a trait.
fn trait_display_name(key: &str) -> &str {
    match key {
        "defined" => "DEFINED",
        "authored" => "AUTHORED",
        "imported" => "IMPORTED",
        "generated" => "GENERATED",
        "retrieved" => "RETRIEVED",
        _ => key,
    }
}

/// Get LLM context description for a trait.
fn trait_llm_context(key: &str) -> &str {
    match key {
        "defined" => {
            "Human-created once, universal. Structural definitions, configuration, business logic. Examples: Page, Entity, Block."
        }
        "authored" => {
            "Human-written per locale. Editorial content with invariant parent (e.g., EntityContent -> Entity). Created by humans, not LLM."
        }
        "imported" => {
            "External data brought in. Native locale knowledge loaded INTO the LLM as context. Exists only where needed (fr-FR may have 20K Terms)."
        }
        "generated" => {
            "LLM-generated content output. Pages, blocks, and artifacts produced by the generation pipeline. Examples: PageGenerated, BlockGenerated, OutputArtifact."
        }
        "retrieved" => {
            "Fetched from external APIs. Time-series data, performance snapshots from external sources. Examples: GEOAnswer, SEOKeywordMetrics."
        }
        _ => "Unknown trait.",
    }
}

// =============================================================================
// CODE EXAMPLES
// =============================================================================

/// Code example showing YAML, Neo4j, and Cypher for a concept.
#[derive(Debug, Clone)]
pub struct CodeExample {
    /// Example title (e.g., "Entity definition").
    pub title: &'static str,
    /// YAML snippet from model definition.
    pub yaml: &'static str,
    /// Neo4j node representation.
    pub neo4j: &'static str,
    /// Cypher query example.
    pub cypher: &'static str,
}

/// Get code examples for a trait.
pub fn trait_code_examples(key: &str) -> Vec<CodeExample> {
    match key {
        "defined" => vec![
            CodeExample {
                title: "Entity (defined structure)",
                yaml: r#"node:
  name: Entity
  realm: org
  layer: foundation
  trait: defined
  properties:
    key: { type: string, required: true }
    display_name: { type: string }"#,
                neo4j: r#"(:Entity {
  key: "qr-code-generator",
  display_name: "QR Code Generator"
})"#,
                cypher: r#"MATCH (e:Entity {key: $key})
RETURN e.key, e.display_name"#,
            },
            CodeExample {
                title: "Page (structural template)",
                yaml: r#"node:
  name: Page
  realm: org
  layer: structure
  trait: defined"#,
                neo4j: r#"(:Page {
  key: "homepage",
  route: "/"
})"#,
                cypher: r#"MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)
RETURN p.key, collect(b.key) AS blocks"#,
            },
        ],
        "authored" => vec![
            CodeExample {
                title: "EntityContent (per-locale content)",
                yaml: r#"node:
  name: EntityContent
  realm: org
  layer: semantic
  trait: authored
  # Composite key: entity:{key}@{locale}"#,
                neo4j: r#"(:EntityContent {
  key: "entity:qr-code@fr-FR",
  title: "Générateur de QR Code",
  description: "Créez des QR codes..."
})"#,
                cypher: r#"MATCH (e:Entity {key: $entity_key})
      -[:HAS_CONTENT]->(c:EntityContent)
WHERE c.key ENDS WITH $locale
RETURN c.title, c.description"#,
            },
            CodeExample {
                title: "ProjectContent (editorial)",
                yaml: r#"node:
  name: ProjectContent
  realm: org
  layer: foundation
  trait: authored
  # Human-written per locale"#,
                neo4j: r#"(:ProjectContent {
  key: "project:qrcode-ai@fr-FR",
  tagline: "Créez des QR codes...",
  meta_description: "..."
})"#,
                cypher: r#"MATCH (p:Project)-[:HAS_CONTENT]->(c:ProjectContent)
WHERE c.key ENDS WITH $locale
RETURN c.tagline"#,
            },
        ],
        "imported" => vec![
            CodeExample {
                title: "Term (vocabulary atom)",
                yaml: r#"node:
  name: Term
  realm: shared
  layer: knowledge
  trait: imported
  # Native to locale, not translated"#,
                neo4j: r#"(:Term {
  key: "term:artificial-intelligence@fr-FR",
  term: "intelligence artificielle",
  definition: "Capacité des machines..."
})"#,
                cypher: r#"MATCH (l:Locale {key: $locale})
      -[:HAS_TERMS]->(:TermSet)
      -[:CONTAINS_TERM]->(t:Term)
WHERE t.domain = $domain
RETURN t.term, t.definition"#,
            },
            CodeExample {
                title: "Expression (stylistic pattern)",
                yaml: r#"node:
  name: Expression
  realm: shared
  layer: knowledge
  trait: imported"#,
                neo4j: r#"(:Expression {
  key: "expr:greeting-formal@de-DE",
  pattern: "Sehr geehrte/r {title} {name}",
  register: "formal"
})"#,
                cypher: r#"MATCH (e:Expression)
WHERE e.key STARTS WITH 'expr:'
  AND e.key ENDS WITH $locale
  AND e.register = 'formal'
RETURN e.pattern"#,
            },
        ],
        "generated" => vec![CodeExample {
            title: "PageGenerated (LLM output)",
            yaml: r#"node:
  name: PageGenerated
  realm: org
  layer: output
  trait: generated
  # Generated by LLM from defined Page"#,
            neo4j: r#"(:PageGenerated {
  key: "page:homepage@fr-FR",
  html_content: "<html>...",
  generated_at: datetime()
})"#,
            cypher: r#"MATCH (p:Page)-[:HAS_GENERATED]->(g:PageGenerated)
WHERE g.key ENDS WITH $locale
RETURN g.html_content"#,
        }],
        "retrieved" => vec![CodeExample {
            title: "SEOKeywordMetrics (API snapshot)",
            yaml: r#"node:
  name: SEOKeywordMetrics
  realm: shared
  layer: knowledge
  trait: retrieved
  # Retrieved from external API"#,
            neo4j: r#"(:SEOKeywordMetrics {
  key: "metrics:seo-kw@fr-FR",
  search_volume: 1500,
  difficulty: 0.65,
  retrieved_at: datetime()
})"#,
            cypher: r#"MATCH (k:SEOKeyword)-[:HAS_METRICS]->(m:SEOKeywordMetrics)
WHERE m.key ENDS WITH $locale
RETURN m.search_volume, m.difficulty"#,
        }],
        _ => vec![],
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
                    class_count: 0,
                    llm_context: trait_llm_context(trait_key).to_string(),
                    classes_by_layer: Vec::new(),
                },
            );
        }

        // Collect classes by trait and layer
        for realm in &self.realms {
            for layer in &realm.layers {
                for class_info in &layer.classes {
                    let trait_key = class_info.trait_name.as_str();
                    if let Some(stats) = stats_map.get_mut(trait_key) {
                        stats.class_count += 1;

                        // Find or create layer group
                        let layer_key = &layer.key;
                        if let Some(layer_group) = stats
                            .classes_by_layer
                            .iter_mut()
                            .find(|(k, _)| k == layer_key)
                        {
                            layer_group.1.push(class_info.key.clone());
                        } else {
                            stats
                                .classes_by_layer
                                .push((layer_key.clone(), vec![class_info.key.clone()]));
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
/// When drilled down (drill_depth > 0), shows class list instead of constellation.
pub fn render_traits_tab(f: &mut Frame, app: &App, area: Rect) {
    if app.nexus.drill_depth > 0 {
        // Drilled mode: show class list on left, detail on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        render_class_list(f, app, chunks[0]);
        render_class_detail(f, app, chunks[1]);
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
    let selected_idx = app.nexus.trait_cursor;
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " CONSTELLATION ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Build constellation lines
    let lines = build_constellation_lines(&trait_stats, selected_idx, theme, inner.width as usize);

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Build the ASCII constellation layout with data flow visualization.
/// Shows data origin flow with directional arrows and role labels.
fn build_constellation_lines(
    stats: &[TraitStats],
    selected_idx: usize,
    theme: &Theme,
    width: usize,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // Calculate max count for heatmap scaling
    let max_count = stats.iter().map(|s| s.class_count).max().unwrap_or(1);

    // Get stats by trait key for easier lookup
    let get_stat = |key: &str| -> Option<&TraitStats> { stats.iter().find(|s| s.key == key) };

    // Helper to create styled trait span
    let trait_span = |key: &str, idx: usize| -> Vec<Span<'static>> {
        let stat = get_stat(key);
        let symbol = trait_symbol(key);
        let name = trait_display_name(key);
        let count = stat.map(|s| s.class_count).unwrap_or(0);

        let is_selected = idx == selected_idx;
        let base_color = theme.trait_color(key);
        let style = if is_selected {
            Style::default()
                .fg(base_color)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(base_color)
        };

        // Use heatmap color for count (bright = many classes, dim = few)
        let count_color = heatmap_color(count, max_count);

        vec![
            Span::styled(format!("{} ", symbol), style),
            Span::styled(name.to_string(), style),
            Span::styled(format!(" ({} K)", count), Style::default().fg(count_color)),
        ]
    };

    // Header: DATA FLOW title
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            "\u{2193} DATA FLOW \u{2193}",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  External data enters at top, flows down to outputs",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(Line::from(""));

    // Row 1: IMPORTED at top center with INPUT label
    let imported_spans = trait_span("imported", 2);
    let imported_line = center_spans(imported_spans, width);
    lines.push(imported_line);

    // Role label for IMPORTED
    let input_label = center_text("\u{250c}\u{2500} INPUT \u{2500}\u{2510}", width); // ┌─ INPUT ─┐
    lines.push(Line::from(Span::styled(
        input_label,
        Style::default().fg(Color::Rgb(139, 92, 246)), // Purple
    )));

    // Row 2: Flow arrows from IMPORTED (down arrows)
    let flow_down = center_text("\u{2502}            \u{2502}", width); // │            │
    lines.push(Line::from(Span::styled(
        flow_down,
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));
    let arrow_down = center_text("\u{25bc}            \u{25bc}", width); // ▼            ▼
    lines.push(Line::from(Span::styled(
        arrow_down,
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));

    // Row 3: DEFINED ═══ ↔ ═══ AUTHORED (core pair)
    let mut core_pair: Vec<Span<'static>> = Vec::new();
    core_pair.extend(trait_span("defined", 0));
    core_pair.push(Span::styled(
        " \u{2550}\u{2550}\u{21d4}\u{2550}\u{2550} ",
        Style::default().fg(Color::Yellow),
    )); // ══↔══
    core_pair.extend(trait_span("authored", 1));
    let core_line = center_spans(core_pair, width);
    lines.push(core_line);

    // Role labels for DEFINED and AUTHORED
    let structure_label = "STRUCTURE";
    let editorial_label = "EDITORIAL";
    let role_line = center_text(
        &format!("({})         ({})", structure_label, editorial_label),
        width,
    );
    lines.push(Line::from(Span::styled(
        role_line,
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));

    // Row 4: Flow arrows down to outputs
    let flow_down2 = center_text("\u{2502}            \u{2502}", width);
    lines.push(Line::from(Span::styled(
        flow_down2,
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));
    let arrow_down2 = center_text("\u{25bc}            \u{25bc}", width);
    lines.push(Line::from(Span::styled(
        arrow_down2,
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));

    // Row 5: GENERATED and RETRIEVED at bottom
    let mut bottom_pair: Vec<Span<'static>> = Vec::new();
    bottom_pair.extend(trait_span("generated", 3));
    bottom_pair.push(Span::styled("    ", Style::default())); // spacer
    bottom_pair.extend(trait_span("retrieved", 4));
    let bottom_line = center_spans(bottom_pair, width);
    lines.push(bottom_line);

    // Role label for outputs
    let output_label = center_text("\u{2514}\u{2500} OUTPUT \u{2500}\u{2518}", width); // └─ OUTPUT ─┘
    lines.push(Line::from(Span::styled(
        output_label,
        Style::default().fg(Color::Rgb(34, 197, 94)), // Green
    )));

    lines.push(Line::from(""));

    // Separator
    lines.push(Line::from(Span::styled(
        "\u{2500}".repeat(width.saturating_sub(2)),
        Style::default().fg(COLOR_UNFOCUSED_BORDER),
    )));

    // Legend: Quick navigation hints
    lines.push(Line::from(vec![
        Span::styled("  Quick jump: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("d=defined  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("a=authored  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("i=imported  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("g=generated  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("r=retrieved", Style::default().fg(Color::DarkGray)),
    ]));
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

        // Add role indicator
        let role = match stat.key.as_str() {
            "imported" => " \u{2190} INPUT",
            "defined" | "authored" => " \u{2194} CORE",
            "generated" | "retrieved" => " \u{2192} OUTPUT",
            _ => "",
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", prefix), style),
            Span::styled(stat.symbol.to_string(), style),
            Span::raw(" "),
            Span::styled(stat.display_name.clone(), style),
            Span::styled(role, Style::default().fg(Color::DarkGray)),
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
    let selected_idx = app.nexus.trait_cursor;
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
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

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

    // Classs by layer section
    lines.push(Line::from(Span::styled(
        "\u{250c}\u{2500} BY LAYER \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));

    for (layer_key, classes) in &stat.classes_by_layer {
        let layer_color = theme.layer_color(layer_key);
        let classes_str = classes.join(", ");
        lines.push(Line::from(vec![
            Span::styled(
                format!("\u{2502} {:<12} ", layer_key),
                Style::default().fg(layer_color),
            ),
            Span::styled(classes_str, Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(Span::styled(
        "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
        Style::default().fg(Color::Rgb(100, 100, 120)),
    )));
    lines.push(Line::from(""));

    // Pattern section for DEFINED
    if stat.key == "defined" {
        lines.push(Line::from(Span::styled(
            "PATTERN:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));

        let defined_color = theme.trait_color("defined");
        let authored_color = theme.trait_color("authored");

        // Show defined -> authored/generated patterns
        let patterns = [
            ("Page", "PageGenerated"),
            ("Entity", "EntityContent"),
            ("Block", "BlockGenerated"),
        ];

        for (def, out) in patterns {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("\u{25a0} {} ", def),
                    Style::default().fg(defined_color),
                ),
                Span::styled(
                    "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2192} ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("\u{25a1} {}", out),
                    Style::default().fg(authored_color),
                ),
            ]));
        }
    }

    // Pattern section for AUTHORED
    if stat.key == "authored" {
        lines.push(Line::from(Span::styled(
            "RELATIONSHIP:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "Content nodes have defined parents.",
            Style::default().fg(Color::Rgb(150, 150, 150)),
        )));
        lines.push(Line::from(Span::styled(
            "Written by humans per locale, NOT translated.",
            Style::default()
                .fg(Color::Rgb(34, 197, 94))
                .add_modifier(Modifier::ITALIC),
        )));
    }

    // Pattern section for IMPORTED
    if stat.key == "imported" {
        lines.push(Line::from(Span::styled(
            "KEY INSIGHT:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "Imported nodes are INPUT to generation.",
            Style::default().fg(Color::Rgb(150, 150, 150)),
        )));
        lines.push(Line::from(Span::styled(
            "External data brought in (native, not translated).",
            Style::default()
                .fg(Color::Rgb(139, 92, 246))
                .add_modifier(Modifier::ITALIC),
        )));
    }

    lines.push(Line::from(""));

    // Code examples section
    let examples = trait_code_examples(&stat.key);
    if !examples.is_empty() {
        lines.push(Line::from(Span::styled(
            "\u{250c}\u{2500} CODE EXAMPLES \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
            Style::default().fg(Color::Rgb(100, 100, 120)),
        )));

        // Show first example (others available on drill-down)
        if let Some(example) = examples.first() {
            lines.push(Line::from(vec![
                Span::styled("\u{2502} ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled(
                    example.title,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(Span::styled(
                "\u{2502}",
                Style::default().fg(Color::Rgb(100, 100, 120)),
            )));

            // YAML snippet (first 3 lines)
            lines.push(Line::from(vec![
                Span::styled("\u{2502} ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled("YAML: ", Style::default().fg(Color::Green)),
            ]));
            for yaml_line in example.yaml.lines().take(3) {
                lines.push(Line::from(vec![
                    Span::styled(
                        "\u{2502}   ",
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                    Span::styled(
                        yaml_line.to_string(),
                        Style::default().fg(Color::Rgb(150, 200, 150)),
                    ),
                ]));
            }
            if example.yaml.lines().count() > 3 {
                lines.push(Line::from(vec![
                    Span::styled(
                        "\u{2502}   ",
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                    Span::styled("...", Style::default().fg(Color::DarkGray)),
                ]));
            }

            // Neo4j node (first 2 lines)
            lines.push(Line::from(vec![
                Span::styled("\u{2502} ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled("Neo4j: ", Style::default().fg(Color::Rgb(0, 150, 255))),
            ]));
            for neo_line in example.neo4j.lines().take(2) {
                lines.push(Line::from(vec![
                    Span::styled(
                        "\u{2502}   ",
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                    Span::styled(
                        neo_line.to_string(),
                        Style::default().fg(Color::Rgb(150, 180, 220)),
                    ),
                ]));
            }
            if example.neo4j.lines().count() > 2 {
                lines.push(Line::from(vec![
                    Span::styled(
                        "\u{2502}   ",
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                    Span::styled("...", Style::default().fg(Color::DarkGray)),
                ]));
            }

            // Cypher query (first 2 lines)
            lines.push(Line::from(vec![
                Span::styled("\u{2502} ", Style::default().fg(Color::Rgb(100, 100, 120))),
                Span::styled("Cypher: ", Style::default().fg(Color::Yellow)),
            ]));
            for cypher_line in example.cypher.lines().take(2) {
                lines.push(Line::from(vec![
                    Span::styled(
                        "\u{2502}   ",
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                    Span::styled(
                        cypher_line.to_string(),
                        Style::default().fg(Color::Rgb(220, 200, 120)),
                    ),
                ]));
            }
            if example.cypher.lines().count() > 2 {
                lines.push(Line::from(vec![
                    Span::styled(
                        "\u{2502}   ",
                        Style::default().fg(Color::Rgb(100, 100, 120)),
                    ),
                    Span::styled("...", Style::default().fg(Color::DarkGray)),
                ]));
            }
        }

        lines.push(Line::from(Span::styled(
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
            Style::default().fg(Color::Rgb(100, 100, 120)),
        )));

        if examples.len() > 1 {
            lines.push(Line::from(Span::styled(
                format!("  [Enter] for {} more examples", examples.len() - 1),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

// =============================================================================
// DRILL-DOWN: CLASS LIST
// =============================================================================

/// Render the list of classes for the selected trait (drill-down view).
fn render_class_list(f: &mut Frame, app: &App, area: Rect) {
    let trait_stats = app.tree.get_trait_stats();
    let selected_trait_idx = app.nexus.trait_cursor;
    let theme = &app.theme;

    let trait_name = TRAIT_ORDER.get(selected_trait_idx).unwrap_or(&"");
    let trait_color = theme.trait_color(trait_name);

    let block = Block::default()
        .title(Span::styled(
            format!(" {} CLASSES ", trait_display_name(trait_name)),
            Style::default()
                .fg(trait_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Get flattened classes list
    let classes = app.nexus.get_trait_classes(&trait_stats);
    let visible_height = inner.height as usize;

    if classes.is_empty() {
        let empty_line = Line::from(Span::styled(
            "No classes with this trait",
            Style::default().fg(Color::DarkGray),
        ));
        let paragraph = Paragraph::new(vec![empty_line]);
        f.render_widget(paragraph, inner);
        return;
    }

    // Calculate scroll window
    let cursor = app.nexus.drill_cursor.min(classes.len().saturating_sub(1));
    let scroll_offset = if cursor < visible_height / 2 {
        0
    } else if cursor >= classes.len().saturating_sub(visible_height / 2) {
        classes.len().saturating_sub(visible_height)
    } else {
        cursor.saturating_sub(visible_height / 2)
    };

    let mut lines: Vec<Line> = Vec::new();

    // Header with count
    lines.push(Line::from(vec![
        Span::styled(
            format!("{} classes with trait ", classes.len()),
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

    // Class list with scroll
    for (idx, (layer_key, class_key)) in classes.iter().enumerate().skip(scroll_offset) {
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
            Span::styled(class_key.clone(), style),
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
    if scroll_offset + visible_height < classes.len() {
        lines.push(Line::from(Span::styled(
            "  ↓ more below...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

/// Render detail panel for the selected class in drill-down view.
fn render_class_detail(f: &mut Frame, app: &App, area: Rect) {
    let trait_stats = app.tree.get_trait_stats();
    let classes = app.nexus.get_trait_classes(&trait_stats);
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " CLASS DETAIL ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let cursor = app.nexus.drill_cursor.min(classes.len().saturating_sub(1));

    let Some((layer_key, class_key)) = classes.get(cursor) else {
        let empty = Paragraph::new("Select a class to see details");
        f.render_widget(empty, inner);
        return;
    };

    // Find the class in the tree to get full details
    let class_data = app.tree.realms.iter().find_map(|realm| {
        realm.layers.iter().find_map(|layer| {
            if &layer.key == layer_key {
                layer.classes.iter().find(|k| &k.key == class_key).map(|k| {
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

    let Some((realm_key, realm_name, layer_name, class)) = class_data else {
        let not_found = Paragraph::new(format!("Class '{}' not found", class_key));
        f.render_widget(not_found, inner);
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // Class name with trait symbol
    let trait_name = TRAIT_ORDER.get(app.nexus.trait_cursor).unwrap_or(&"");
    let trait_color = theme.trait_color(trait_name);

    lines.push(Line::from(vec![
        Span::styled(
            format!("{} ", trait_symbol(trait_name)),
            Style::default().fg(trait_color),
        ),
        Span::styled(
            class.display_name.clone(),
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
    if !class.description.is_empty() {
        lines.push(Line::from(Span::styled(
            "Description:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        let wrapped = wrap_text(&class.description, inner.width.saturating_sub(2) as usize);
        for line in wrapped {
            lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Rgb(180, 180, 180)),
            )));
        }
        lines.push(Line::from(""));
    }

    // Properties summary
    if !class.properties.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("Properties ({}):", class.properties.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        for prop in class.properties.iter().take(5) {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(Color::DarkGray)),
                Span::styled(prop.clone(), Style::default().fg(Color::White)),
            ]));
        }
        if class.properties.len() > 5 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", class.properties.len() - 5),
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
        assert_eq!(trait_symbol("defined"), "\u{25a0}");
        assert_eq!(trait_symbol("authored"), "\u{25a1}");
        assert_eq!(trait_symbol("imported"), "\u{25ca}");
        assert_eq!(trait_symbol("generated"), "\u{2605}"); // ★
        assert_eq!(trait_symbol("retrieved"), "\u{25aa}"); // ▪
        assert_eq!(trait_symbol("unknown"), "\u{00b7}");
    }

    #[test]
    fn test_trait_display_names() {
        assert_eq!(trait_display_name("defined"), "DEFINED");
        assert_eq!(trait_display_name("authored"), "AUTHORED");
        assert_eq!(trait_display_name("imported"), "IMPORTED");
        assert_eq!(trait_display_name("generated"), "GENERATED");
        assert_eq!(trait_display_name("retrieved"), "RETRIEVED");
    }

    #[test]
    fn test_trait_order() {
        assert_eq!(TRAIT_ORDER.len(), 5);
        assert_eq!(TRAIT_ORDER[0], "defined");
        assert_eq!(TRAIT_ORDER[1], "authored");
        assert_eq!(TRAIT_ORDER[2], "imported");
        assert_eq!(TRAIT_ORDER[3], "generated");
        assert_eq!(TRAIT_ORDER[4], "retrieved");
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

    // ==========================================================================
    // CODE EXAMPLES TESTS
    // ==========================================================================

    #[test]
    fn test_code_examples_defined() {
        let examples = trait_code_examples("defined");
        assert_eq!(examples.len(), 2);
        assert!(examples[0].title.contains("Entity"));
        assert!(examples[0].yaml.contains("trait: defined"));
        assert!(!examples[0].neo4j.is_empty());
        assert!(!examples[0].cypher.is_empty());
    }

    #[test]
    fn test_code_examples_authored() {
        let examples = trait_code_examples("authored");
        assert_eq!(examples.len(), 2);
        assert!(examples[0].title.contains("EntityContent"));
        assert!(examples[0].yaml.contains("trait: authored"));
    }

    #[test]
    fn test_code_examples_imported() {
        let examples = trait_code_examples("imported");
        assert_eq!(examples.len(), 2);
        assert!(examples[0].title.contains("Term"));
        assert!(examples[0].yaml.contains("trait: imported"));
    }

    #[test]
    fn test_code_examples_generated() {
        let examples = trait_code_examples("generated");
        assert_eq!(examples.len(), 1);
        assert!(examples[0].title.contains("PageGenerated"));
        assert!(examples[0].yaml.contains("trait: generated"));
    }

    #[test]
    fn test_code_examples_retrieved() {
        let examples = trait_code_examples("retrieved");
        assert_eq!(examples.len(), 1);
        assert!(examples[0].title.contains("Metrics"));
        assert!(examples[0].yaml.contains("trait: retrieved"));
    }

    #[test]
    fn test_code_examples_unknown() {
        let examples = trait_code_examples("unknown");
        assert!(examples.is_empty());
    }

    #[test]
    fn test_code_example_contains_all_fields() {
        for trait_key in TRAIT_ORDER {
            let examples = trait_code_examples(trait_key);
            for example in examples {
                assert!(!example.title.is_empty(), "Empty title for {}", trait_key);
                assert!(!example.yaml.is_empty(), "Empty YAML for {}", trait_key);
                assert!(!example.neo4j.is_empty(), "Empty Neo4j for {}", trait_key);
                assert!(!example.cypher.is_empty(), "Empty Cypher for {}", trait_key);
            }
        }
    }
}
