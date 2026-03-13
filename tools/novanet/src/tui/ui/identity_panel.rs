//! Identity & Data Management panel (top center).
//!
//! v0.21.0: Complete redesign — focuses exclusively on data management and graph location.
//! - NO repeated node info (key, name, description) — that's in Properties panel
//! - Dynamic title: "Schema Structure" (schema nodes) vs "Data Management" (data nodes)
//! - Category-colored rounded borders with DataCategory-aware styling
//! - Pipeline visualization showing how node was created (YAML → Cypher → Neo4j)
//! - Lifecycle badges (reseed, backup, editable) with inline explanations
//! - Completeness gauge using Unicode block characters
//! - Scrollable with Scrollbar when focused
//! - Visual distinction between schema and data nodes

use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
  Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use serde_json::Value as JsonValue;

use crate::tui::app::{App, Focus};
use crate::tui::data::TreeItem;
use crate::tui::theme;

use super::info::{DataCategory, ProvenanceMeta};

// =============================================================================
// COLORS
// =============================================================================

/// Dim label color for field names.
const COLOR_DIM: Color = Color::Rgb(100, 100, 110);

/// Muted text color for secondary info.
const COLOR_MUTED: Color = Color::Rgb(130, 130, 140);

/// Breadcrumb separator color.
const COLOR_SEPARATOR: Color = Color::Rgb(70, 70, 80);

/// Flow arrow color.
const COLOR_FLOW: Color = Color::Rgb(80, 80, 100);

/// Seed file color.
const COLOR_FILE: Color = Color::Rgb(180, 180, 200);

/// Command color (actionable).
const COLOR_CMD: Color = Color::Rgb(139, 233, 253); // Cyan

/// Gauge filled color.
const COLOR_GAUGE_FILLED: Color = Color::Rgb(34, 197, 94); // Green-500

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Identity + Data Management panel.
pub fn render_identity_panel(f: &mut Frame, area: Rect, app: &mut App) {
  let is_focused = app.focus == Focus::Identity;

  // Build content lines
  let (lines, panel_title, border_color) = build_panel_content(app);

  // Track total line count for scrollbar
  let total_lines = lines.len();
  app.identity_line_count = total_lines;

  // Clamp scroll position
  let visible_height = area.height.saturating_sub(2) as usize; // minus borders
  if app.identity_scroll > total_lines.saturating_sub(visible_height) {
    app.identity_scroll = total_lines.saturating_sub(visible_height);
  }

  // Build block with rounded borders and dynamic title
  let focus_border = if is_focused {
    theme::ui::ACCENT
  } else {
    border_color.unwrap_or(Color::Rgb(60, 60, 70))
  };

  let mut block = Block::default()
    .title(format!(" {} ", panel_title))
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(Style::default().fg(focus_border));

  // Bottom title: scroll hint when focused and content overflows
  if is_focused && total_lines > visible_height {
    block = block.title_bottom(Line::from(vec![
      Span::styled(" j", Style::default().fg(COLOR_CMD)),
      Span::styled("/", Style::default().fg(COLOR_DIM)),
      Span::styled("k", Style::default().fg(COLOR_CMD)),
      Span::styled(" scroll ", Style::default().fg(COLOR_DIM)),
    ]));
  }

  let paragraph = Paragraph::new(lines)
    .block(block)
    .scroll((app.identity_scroll as u16, 0));

  f.render_widget(paragraph, area);

  // Render scrollbar when focused and content overflows
  if is_focused && total_lines > visible_height {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
      .thumb_style(Style::default().fg(focus_border));
    let mut scrollbar_state = ScrollbarState::new(total_lines)
      .position(app.identity_scroll);
    f.render_stateful_widget(
      scrollbar,
      area.inner(Margin { vertical: 1, horizontal: 0 }),
      &mut scrollbar_state,
    );
  }
}

/// Build panel content, title, and border color based on current selection.
/// Returns (lines, title, optional border color for category).
fn build_panel_content(app: &App) -> (Vec<Line<'static>>, String, Option<Color>) {
  let item = app.tree.item_at(app.tree_cursor);

  match item {
    // =========================================================================
    // SCHEMA NODES — Realm, Layer, Class, ArcFamily, ArcClass, Sections
    // =========================================================================
    Some(TreeItem::Realm(realm)) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "REALM");
      push_empty(&mut lines);
      push_breadcrumb_realm(&mut lines, &realm.display_name, &realm.color);
      push_empty(&mut lines);
      push_kv(&mut lines, "Layers", &format!("{}", realm.layers.len()));
      push_kv(
        &mut lines,
        "Classes",
        &format!("{}", realm.total_classes()),
      );
      push_empty(&mut lines);
      push_pipeline_schema(&mut lines, "taxonomy.yaml", "00.5-taxonomy.cypher");
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        if realm.key == "shared" {
          "Universal data shared across all organizations. Read-only."
        } else {
          "Organization-specific data. Editable per project."
        },
      );
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    Some(TreeItem::Layer(realm, layer)) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "LAYER");
      push_empty(&mut lines);
      push_breadcrumb_layer(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
      );
      push_empty(&mut lines);
      push_kv(&mut lines, "Classes", &format!("{}", layer.classes.len()));
      push_empty(&mut lines);
      push_pipeline_schema(&mut lines, "taxonomy.yaml", "00.5-taxonomy.cypher");
      push_empty(&mut lines);
      if !layer.content.is_empty() {
        push_explanation(&mut lines, &layer.content);
      }
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    Some(TreeItem::Class(realm, layer, class_info)) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "NODE CLASS");
      push_empty(&mut lines);
      push_breadcrumb_class(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
        &class_info.display_name,
      );
      push_empty(&mut lines);
      push_kv(
        &mut lines,
        "Instances",
        &format!("{}", class_info.instance_count),
      );
      push_kv(
        &mut lines,
        "Properties",
        &format!(
          "{} ({} required)",
          class_info.properties.len(),
          class_info.required_properties.len()
        ),
      );
      push_kv(&mut lines, "Arcs", &format!("{}", class_info.arcs.len()));
      push_empty(&mut lines);
      // Show YAML source → Cypher pipeline
      if !class_info.yaml_path.is_empty() {
        let yaml_file = class_info
          .yaml_path
          .rsplit('/')
          .next()
          .unwrap_or(&class_info.yaml_path);
        push_pipeline_schema(&mut lines, yaml_file, "01-classes.cypher");
      } else {
        push_pipeline_schema(&mut lines, "<class>.yaml", "01-classes.cypher");
      }
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Defines the structure (properties, arcs) for all instances of this type.",
      );
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    Some(TreeItem::ArcFamily(family)) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "ARC FAMILY");
      push_empty(&mut lines);
      push_line(
        &mut lines,
        " ",
        &family.display_name,
        Color::Magenta,
        true,
      );
      push_empty(&mut lines);
      push_pipeline_schema(&mut lines, "taxonomy.yaml", "00.5-taxonomy.cypher");
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Groups related arc types that share the same purpose (ownership, localization, etc.)",
      );
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    Some(TreeItem::ArcClass(family, arc_class)) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "ARC CLASS");
      push_empty(&mut lines);
      push_line(
        &mut lines,
        " ",
        &format!("{} > {}", family.display_name, arc_class.display_name),
        Color::Cyan,
        false,
      );
      push_empty(&mut lines);
      // Show pattern
      lines.push(Line::from(vec![
        Span::styled("  Pattern  ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          arc_class.from_class.clone(),
          Style::default().fg(Color::Green),
        ),
        Span::styled(" ──→ ", Style::default().fg(COLOR_FLOW)),
        Span::styled(
          arc_class.to_class.clone(),
          Style::default().fg(Color::Yellow),
        ),
      ]));
      push_empty(&mut lines);
      push_pipeline_schema(&mut lines, "<arc>.yaml", "02-arc-classes.cypher");
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    Some(TreeItem::ClassesSection) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "NODE CLASSES");
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Browse all 59 node class definitions organized by realm and layer.",
      );
      push_empty(&mut lines);
      push_pipeline_schema(&mut lines, "models/node-classes/", "01-classes.cypher");
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    Some(TreeItem::ArcsSection) => {
      let mut lines = Vec::new();
      push_schema_banner(&mut lines, "ARC CLASSES");
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Browse all 159 arc class definitions organized by family.",
      );
      push_empty(&mut lines);
      push_pipeline_schema(&mut lines, "models/arc-classes/", "02-arc-classes.cypher");
      (lines, "Schema Structure".into(), Some(Color::Rgb(100, 116, 139)))
    }

    // =========================================================================
    // DATA NODES — Instance, EntityNative, EntityCategory, LocaleGroup, EntityGroup
    // =========================================================================
    Some(TreeItem::Instance(realm, layer, class_info, instance)) => {
      let provenance = instance.properties.get("provenance");
      let (category, meta) = parse_provenance(provenance);

      let mut lines = Vec::new();

      // Graph location breadcrumb
      push_breadcrumb_class(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
        &class_info.display_name,
      );
      push_empty(&mut lines);

      // Data source with category badge
      push_category_badge(&mut lines, &category);
      push_empty(&mut lines);

      // Pipeline: how this data was created
      push_pipeline_data(&mut lines, &category, &meta);
      push_empty(&mut lines);

      // Lifecycle badges
      push_lifecycle(&mut lines, &category);
      push_empty(&mut lines);

      // Completeness gauge
      push_completeness(
        &mut lines,
        instance.filled_properties,
        instance.total_properties,
      );

      // Generation/MCP details (if present)
      push_runtime_details(&mut lines, &meta);

      // Timestamps
      push_timestamps(&mut lines, &instance.properties);

      (lines, "Data Management".into(), Some(category.color()))
    }

    Some(TreeItem::EntityNativeItem(realm, layer, class_info, native)) => {
      let provenance = native.properties.get("provenance");
      let (category, meta) = parse_provenance(provenance);

      let mut lines = Vec::new();

      // Graph location breadcrumb
      push_breadcrumb_class(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
        &class_info.display_name,
      );
      push_empty(&mut lines);

      // Data source
      push_category_badge(&mut lines, &category);
      push_empty(&mut lines);

      // Locale context
      lines.push(Line::from(vec![
        Span::styled("  Locale   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          native.locale_code.clone(),
          Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  of  ", Style::default().fg(COLOR_SEPARATOR)),
        Span::styled(
          native.entity_display_name.clone(),
          Style::default().fg(Color::Green),
        ),
      ]));
      push_empty(&mut lines);

      // Pipeline
      push_pipeline_data(&mut lines, &category, &meta);
      push_empty(&mut lines);

      // Lifecycle
      push_lifecycle(&mut lines, &category);
      push_empty(&mut lines);

      // Completeness from relationship_power
      let filled = (native.relationship_power as usize * native.properties.len()) / 100;
      push_completeness(&mut lines, filled, native.properties.len());

      // Runtime details
      push_runtime_details(&mut lines, &meta);

      // Timestamps
      push_timestamps(&mut lines, &native.properties);

      (lines, "Data Management".into(), Some(category.color()))
    }

    Some(TreeItem::EntityCategory(realm, layer, class_info, category_item)) => {
      let mut lines = Vec::new();
      push_breadcrumb_class(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
        &class_info.display_name,
      );
      push_empty(&mut lines);
      lines.push(Line::from(vec![
        Span::styled("  Group    ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          category_item.display_name.clone(),
          Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        ),
      ]));
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Entity category grouping. Instances inside share the same semantic domain.",
      );
      (lines, "Data Management".into(), Some(Color::Rgb(59, 130, 246)))
    }

    Some(TreeItem::LocaleGroup(realm, layer, class_info, group)) => {
      let mut lines = Vec::new();
      push_breadcrumb_class(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
        &class_info.display_name,
      );
      push_empty(&mut lines);
      lines.push(Line::from(vec![
        Span::styled("  Locale   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          format!("{} {}", group.flag, group.locale_name),
          Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
        ),
      ]));
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Locale group — native content for this language/region.",
      );
      (lines, "Data Management".into(), Some(Color::Rgb(6, 182, 212)))
    }

    Some(TreeItem::EntityGroup(realm, layer, class_info, group)) => {
      let mut lines = Vec::new();
      push_breadcrumb_class(
        &mut lines,
        &realm.display_name,
        &realm.color,
        &layer.display_name,
        &layer.color,
        &class_info.display_name,
      );
      push_empty(&mut lines);
      lines.push(Line::from(vec![
        Span::styled("  Entity   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          group.entity_display_name.clone(),
          Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
        ),
      ]));
      push_empty(&mut lines);
      push_explanation(
        &mut lines,
        "Entity group — all locale variants for this entity.",
      );
      (lines, "Data Management".into(), Some(Color::Rgb(34, 197, 94)))
    }

    // =========================================================================
    // NOTHING SELECTED
    // =========================================================================
    _ => {
      let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
          "  Select a node to see data management info",
          Style::default().fg(COLOR_DIM),
        )),
      ];
      (lines, "Identity".into(), None)
    }
  }
}

// =============================================================================
// CONTENT BUILDERS
// =============================================================================

/// Parse provenance from JSON property value.
fn parse_provenance(provenance: Option<&JsonValue>) -> (DataCategory, ProvenanceMeta) {
  match provenance {
    Some(obj @ JsonValue::Object(_)) => {
      let meta = ProvenanceMeta::from_json(obj);
      let cat = meta
        .source
        .as_deref()
        .map(DataCategory::from_source)
        .unwrap_or(DataCategory::Mcp);
      (cat, meta)
    }
    Some(JsonValue::String(s)) => {
      if let Ok(parsed) = serde_json::from_str::<JsonValue>(s) {
        let meta = ProvenanceMeta::from_json(&parsed);
        let cat = meta
          .source
          .as_deref()
          .map(DataCategory::from_source)
          .unwrap_or(DataCategory::Mcp);
        (cat, meta)
      } else {
        let cat = DataCategory::from_source(s);
        (
          cat,
          ProvenanceMeta {
            source: Some(s.clone()),
            version: None,
            file: None,
            workflow_id: None,
            task_id: None,
            provider: None,
            model: None,
            generated_at: None,
            tool: None,
            user: None,
          },
        )
      }
    }
    _ => (
      DataCategory::Mcp,
      ProvenanceMeta {
        source: None,
        version: None,
        file: None,
        workflow_id: None,
        task_id: None,
        provider: None,
        model: None,
        generated_at: None,
        tool: None,
        user: None,
      },
    ),
  }
}

/// Push the schema banner: ╔══ SCHEMA: <type> ══╗
fn push_schema_banner(lines: &mut Vec<Line<'static>>, node_type: &str) {
  let label = format!("  ╔══ SCHEMA: {} ══╗", node_type);
  lines.push(Line::from(Span::styled(
    label,
    Style::default()
      .fg(Color::Rgb(100, 116, 139)) // Slate-500
      .add_modifier(Modifier::BOLD),
  )));
}

/// Push realm breadcrumb: ● realm
fn push_breadcrumb_realm(lines: &mut Vec<Line<'static>>, realm_name: &str, realm_color: &str) {
  let color = parse_hex_color(realm_color);
  lines.push(Line::from(vec![
    Span::styled("  ● ", Style::default().fg(color)),
    Span::styled(
      realm_name.to_string(),
      Style::default().fg(color).add_modifier(Modifier::BOLD),
    ),
  ]));
}

/// Push realm > layer breadcrumb.
fn push_breadcrumb_layer(
  lines: &mut Vec<Line<'static>>,
  realm_name: &str,
  realm_color: &str,
  layer_name: &str,
  layer_color: &str,
) {
  let r_color = parse_hex_color(realm_color);
  let l_color = parse_hex_color(layer_color);
  lines.push(Line::from(vec![
    Span::styled("  ● ", Style::default().fg(r_color)),
    Span::styled(realm_name.to_string(), Style::default().fg(r_color)),
    Span::styled("  ▸  ", Style::default().fg(COLOR_SEPARATOR)),
    Span::styled(
      layer_name.to_string(),
      Style::default().fg(l_color).add_modifier(Modifier::BOLD),
    ),
  ]));
}

/// Push realm > layer > class breadcrumb.
fn push_breadcrumb_class(
  lines: &mut Vec<Line<'static>>,
  realm_name: &str,
  realm_color: &str,
  layer_name: &str,
  layer_color: &str,
  class_name: &str,
) {
  let r_color = parse_hex_color(realm_color);
  let l_color = parse_hex_color(layer_color);
  lines.push(Line::from(vec![
    Span::styled("  ● ", Style::default().fg(r_color)),
    Span::styled(realm_name.to_string(), Style::default().fg(r_color)),
    Span::styled(" ▸ ", Style::default().fg(COLOR_SEPARATOR)),
    Span::styled(layer_name.to_string(), Style::default().fg(l_color)),
    Span::styled(" ▸ ", Style::default().fg(COLOR_SEPARATOR)),
    Span::styled(
      class_name.to_string(),
      Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD),
    ),
  ]));
}

/// Push category badge: icon + label + description.
fn push_category_badge(lines: &mut Vec<Line<'static>>, category: &DataCategory) {
  lines.push(Line::from(vec![
    Span::styled("  Source    ", Style::default().fg(COLOR_DIM)),
    Span::styled(
      format!("{} ", category.icon()),
      Style::default().fg(category.color()),
    ),
    Span::styled(
      category.label().to_string(),
      Style::default()
        .fg(category.color())
        .add_modifier(Modifier::BOLD),
    ),
    Span::styled(
      format!(" — {}", category.description()),
      Style::default().fg(COLOR_MUTED),
    ),
  ]));
}

/// Push schema pipeline: YAML → Cypher → Neo4j.
fn push_pipeline_schema(lines: &mut Vec<Line<'static>>, yaml_source: &str, cypher_file: &str) {
  lines.push(Line::from(vec![
    Span::styled("  ", Style::default()),
    Span::styled("YAML", Style::default().fg(Color::Rgb(249, 226, 175))), // Yellow
    Span::styled("  ", Style::default().fg(COLOR_FILE)),
    Span::styled(yaml_source.to_string(), Style::default().fg(COLOR_FILE)),
  ]));
  lines.push(Line::from(vec![
    Span::styled("    ↓ ", Style::default().fg(COLOR_FLOW)),
    Span::styled(
      "schema generate",
      Style::default().fg(COLOR_CMD).add_modifier(Modifier::DIM),
    ),
  ]));
  lines.push(Line::from(vec![
    Span::styled("  ", Style::default()),
    Span::styled("Cypher", Style::default().fg(Color::Rgb(137, 180, 250))), // Blue
    Span::styled("  ", Style::default().fg(COLOR_FILE)),
    Span::styled(cypher_file.to_string(), Style::default().fg(COLOR_FILE)),
  ]));
  lines.push(Line::from(vec![
    Span::styled("    ↓ ", Style::default().fg(COLOR_FLOW)),
    Span::styled(
      "db seed",
      Style::default().fg(COLOR_CMD).add_modifier(Modifier::DIM),
    ),
  ]));
  lines.push(Line::from(vec![
    Span::styled("  ", Style::default()),
    Span::styled(
      "Neo4j",
      Style::default()
        .fg(Color::Rgb(34, 197, 94))
        .add_modifier(Modifier::BOLD),
    ), // Green
    Span::styled("  ", Style::default()),
    Span::styled("bolt://localhost:7687", Style::default().fg(COLOR_MUTED)),
  ]));
}

/// Push data pipeline: shows origin based on category.
fn push_pipeline_data(
  lines: &mut Vec<Line<'static>>,
  category: &DataCategory,
  meta: &ProvenanceMeta,
) {
  match category {
    DataCategory::Schema | DataCategory::Immutable | DataCategory::Locale | DataCategory::Content => {
      // Seed-based: show seed file if available
      let file_name = meta
        .file
        .as_deref()
        .unwrap_or("seed/*.cypher");
      lines.push(Line::from(vec![
        Span::styled("  Origin   ", Style::default().fg(COLOR_DIM)),
        Span::styled("Seed  ", Style::default().fg(Color::Rgb(249, 226, 175))),
        Span::styled(file_name.to_string(), Style::default().fg(COLOR_FILE)),
      ]));
      // Show backup path for content
      if category.needs_backup() {
        lines.push(Line::from(vec![
          Span::styled("  Backup   ", Style::default().fg(COLOR_DIM)),
          Span::styled(
            "private-data/data/*.yaml",
            Style::default().fg(COLOR_FILE),
          ),
        ]));
      }
    }
    DataCategory::Nika => {
      let wf = meta.workflow_id.as_deref().unwrap_or("workflow");
      lines.push(Line::from(vec![
        Span::styled("  Origin   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          "Nika  ",
          Style::default().fg(Color::Rgb(249, 115, 22)), // Orange
        ),
        Span::styled(wf.to_string(), Style::default().fg(COLOR_FILE)),
      ]));
      lines.push(Line::from(vec![
        Span::styled("  Backup   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          "private-data/data/*.yaml",
          Style::default().fg(COLOR_FILE),
        ),
      ]));
    }
    DataCategory::Mcp => {
      let tool = meta.tool.as_deref().unwrap_or("novanet_write");
      lines.push(Line::from(vec![
        Span::styled("  Origin   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          "MCP   ",
          Style::default().fg(Color::Rgb(168, 85, 247)), // Purple
        ),
        Span::styled(tool.to_string(), Style::default().fg(COLOR_FILE)),
      ]));
      lines.push(Line::from(vec![
        Span::styled("  Backup   ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          "private-data/data/*.yaml",
          Style::default().fg(COLOR_FILE),
        ),
      ]));
    }
  }
}

/// Push lifecycle badges line.
fn push_lifecycle(lines: &mut Vec<Line<'static>>, category: &DataCategory) {
  let reseed = if category.reseed_safe() {
    Span::styled(
      "✓ Reseed-safe",
      Style::default().fg(Color::Rgb(34, 197, 94)), // Green
    )
  } else {
    Span::styled(
      "⚠ Reseed-LOST",
      Style::default().fg(Color::Rgb(239, 68, 68)), // Red
    )
  };

  let backup = if category.needs_backup() {
    Span::styled(
      "● Backup",
      Style::default().fg(Color::Rgb(249, 115, 22)), // Orange
    )
  } else {
    Span::styled("○ No backup", Style::default().fg(COLOR_DIM))
  };

  let edit = if category.is_editable() {
    Span::styled(
      "✎ Editable",
      Style::default().fg(Color::Rgb(59, 130, 246)), // Blue
    )
  } else {
    Span::styled("⊘ Read-only", Style::default().fg(COLOR_DIM))
  };

  lines.push(Line::from(vec![
    Span::styled("  ", Style::default()),
    reseed,
    Span::styled("  ", Style::default()),
    backup,
    Span::styled("  ", Style::default()),
    edit,
  ]));
}

/// Push completeness gauge: ▰▰▰▰▰▰▰▱▱▱ 7/10 (70%)
fn push_completeness(lines: &mut Vec<Line<'static>>, filled: usize, total: usize) {
  if total == 0 {
    return;
  }
  let pct = (filled as f64 / total as f64 * 100.0) as usize;
  let gauge_width = 10;
  let filled_bars = (filled * gauge_width) / total.max(1);
  let empty_bars = gauge_width - filled_bars;

  let gauge_str = format!(
    "{}{}",
    "▰".repeat(filled_bars),
    "▱".repeat(empty_bars),
  );

  let gauge_color = if pct >= 80 {
    COLOR_GAUGE_FILLED
  } else if pct >= 50 {
    Color::Rgb(249, 226, 175) // Yellow
  } else {
    Color::Rgb(239, 68, 68) // Red
  };

  lines.push(Line::from(vec![
    Span::styled("  Props    ", Style::default().fg(COLOR_DIM)),
    Span::styled(gauge_str, Style::default().fg(gauge_color)),
    Span::styled(
      format!(" {}/{} ({}%)", filled, total, pct),
      Style::default().fg(COLOR_MUTED),
    ),
  ]));
}

/// Push runtime details (Nika generation, MCP mutation).
fn push_runtime_details(lines: &mut Vec<Line<'static>>, meta: &ProvenanceMeta) {
  // Nika details
  if meta.workflow_id.is_some() || meta.task_id.is_some() {
    push_empty(lines);
    lines.push(Line::from(Span::styled(
      "  ─── Generation ───",
      Style::default()
        .fg(Color::Rgb(139, 92, 246))
        .add_modifier(Modifier::DIM),
    )));
    if let Some(ref wf) = meta.workflow_id {
      lines.push(Line::from(vec![
        Span::styled("  Workflow  ", Style::default().fg(COLOR_DIM)),
        Span::styled(wf.clone(), Style::default().fg(Color::Cyan)),
      ]));
    }
    if let Some(ref task) = meta.task_id {
      lines.push(Line::from(vec![
        Span::styled("  Task      ", Style::default().fg(COLOR_DIM)),
        Span::styled(task.clone(), Style::default().fg(COLOR_MUTED)),
      ]));
    }
    if let (Some(prov), Some(model)) = (&meta.provider, &meta.model) {
      lines.push(Line::from(vec![
        Span::styled("  Model     ", Style::default().fg(COLOR_DIM)),
        Span::styled(
          format!("{}/{}", prov, model),
          Style::default().fg(Color::Yellow),
        ),
      ]));
    }
  }

  // MCP details
  if meta.tool.is_some() || meta.user.is_some() {
    push_empty(lines);
    lines.push(Line::from(Span::styled(
      "  ─── MCP Mutation ───",
      Style::default()
        .fg(Color::Rgb(139, 92, 246))
        .add_modifier(Modifier::DIM),
    )));
    if let Some(ref tool) = meta.tool {
      lines.push(Line::from(vec![
        Span::styled("  Tool      ", Style::default().fg(COLOR_DIM)),
        Span::styled(tool.clone(), Style::default().fg(Color::Cyan)),
      ]));
    }
    if let Some(ref user) = meta.user {
      lines.push(Line::from(vec![
        Span::styled("  User      ", Style::default().fg(COLOR_DIM)),
        Span::styled(user.clone(), Style::default().fg(Color::Yellow)),
      ]));
    }
  }
}

/// Push timestamps (created_at, updated_at).
fn push_timestamps(
  lines: &mut Vec<Line<'static>>,
  properties: &std::collections::BTreeMap<String, JsonValue>,
) {
  let created = properties
    .get("created_at")
    .and_then(|v| v.as_str())
    .map(String::from);
  let updated = properties
    .get("updated_at")
    .and_then(|v| v.as_str())
    .map(String::from);

  if created.is_some() || updated.is_some() {
    push_empty(lines);
    if let Some(ts) = created {
      lines.push(Line::from(vec![
        Span::styled("  Created  ", Style::default().fg(COLOR_DIM)),
        Span::styled(ts, Style::default().fg(COLOR_MUTED)),
      ]));
    }
    if let Some(ts) = updated {
      lines.push(Line::from(vec![
        Span::styled("  Updated  ", Style::default().fg(COLOR_DIM)),
        Span::styled(ts, Style::default().fg(COLOR_MUTED)),
      ]));
    }
  }

  // Version
  // (parsed from provenance, not from properties — but we handle it in push_runtime_details)
}

// =============================================================================
// HELPERS
// =============================================================================

/// Push an empty line.
fn push_empty(lines: &mut Vec<Line<'static>>) {
  lines.push(Line::from(""));
}

/// Push a simple labeled line.
fn push_line(
  lines: &mut Vec<Line<'static>>,
  prefix: &str,
  value: &str,
  color: Color,
  bold: bool,
) {
  let mut style = Style::default().fg(color);
  if bold {
    style = style.add_modifier(Modifier::BOLD);
  }
  lines.push(Line::from(vec![
    Span::styled(prefix.to_string(), Style::default()),
    Span::styled(value.to_string(), style),
  ]));
}

/// Push a key-value pair.
fn push_kv(lines: &mut Vec<Line<'static>>, key: &str, value: &str) {
  // Pad key to 10 chars for alignment
  let padded = format!("  {:<9}", key);
  lines.push(Line::from(vec![
    Span::styled(padded, Style::default().fg(COLOR_DIM)),
    Span::styled(value.to_string(), Style::default().fg(Color::White)),
  ]));
}

/// Push an explanation line (normie-friendly).
fn push_explanation(lines: &mut Vec<Line<'static>>, text: &str) {
  lines.push(Line::from(vec![
    Span::styled("  ", Style::default()),
    Span::styled(
      text.to_string(),
      Style::default()
        .fg(COLOR_MUTED)
        .add_modifier(Modifier::ITALIC),
    ),
  ]));
}

/// Parse hex color string (#rrggbb) to Color.
fn parse_hex_color(hex: &str) -> Color {
  let hex = hex.trim_start_matches('#');
  if hex.len() == 6 {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);
    Color::Rgb(r, g, b)
  } else {
    Color::White
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_hex_color_valid() {
    assert_eq!(parse_hex_color("#ff0000"), Color::Rgb(255, 0, 0));
    assert_eq!(parse_hex_color("#00ff00"), Color::Rgb(0, 255, 0));
    assert_eq!(parse_hex_color("0000ff"), Color::Rgb(0, 0, 255));
  }

  #[test]
  fn test_parse_hex_color_invalid() {
    assert_eq!(parse_hex_color("xyz"), Color::White);
    assert_eq!(parse_hex_color(""), Color::White);
  }

  #[test]
  fn test_parse_provenance_none() {
    let (cat, meta) = parse_provenance(None);
    assert_eq!(cat, DataCategory::Mcp);
    assert!(meta.source.is_none());
  }

  #[test]
  fn test_parse_provenance_seed_schema() {
    let json = serde_json::json!({"source": "seed:schema", "version": "v0.20.0"});
    let (cat, meta) = parse_provenance(Some(&json));
    assert_eq!(cat, DataCategory::Schema);
    assert_eq!(meta.version.as_deref(), Some("v0.20.0"));
  }

  #[test]
  fn test_parse_provenance_runtime_nika() {
    let json = serde_json::json!({
      "source": "runtime:nika",
      "workflow_id": "generate-page",
      "provider": "anthropic",
      "model": "claude-3-5-sonnet"
    });
    let (cat, meta) = parse_provenance(Some(&json));
    assert_eq!(cat, DataCategory::Nika);
    assert_eq!(meta.workflow_id.as_deref(), Some("generate-page"));
  }

  #[test]
  fn test_parse_provenance_string_legacy() {
    let json = JsonValue::String("seed:content".into());
    let (cat, _meta) = parse_provenance(Some(&json));
    assert_eq!(cat, DataCategory::Content);
  }

  #[test]
  fn test_data_category_lifecycle() {
    assert!(DataCategory::Schema.reseed_safe());
    assert!(!DataCategory::Schema.needs_backup());
    assert!(!DataCategory::Schema.is_editable());

    assert!(!DataCategory::Nika.reseed_safe());
    assert!(DataCategory::Nika.needs_backup());
    assert!(DataCategory::Nika.is_editable());
  }
}
