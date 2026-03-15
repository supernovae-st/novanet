//! Data node content builders for the identity panel.
//!
//! Handles Instance, EntityNativeItem, EntityCategory, EntityGroup.
//! Includes provenance parsing, data pipelines, lifecycle badges,
//! completeness gauges, runtime details, and timestamps.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use serde_json::Value as JsonValue;

use crate::tui::data::{
  ClassInfo, EntityCategory as EntityCategoryData, EntityNativeGroup, EntityNativeInfo,
  InstanceInfo, LayerInfo, RealmInfo,
};
use crate::tui::palette;
use crate::tui::widgets::ProgressBar;

use super::helpers::{push_breadcrumb_class, push_empty, push_explanation};
use super::super::info::{DataCategory, ProvenanceMeta};
use super::super::COLOR_SEPARATOR;
use super::{COLOR_DIM, COLOR_FILE, COLOR_GAUGE_FILLED, COLOR_MUTED};

// =============================================================================
// DATA NODE BUILDERS
// =============================================================================

/// Build content for an Instance node.
pub(super) fn build_instance(
  realm: &RealmInfo,
  layer: &LayerInfo,
  class_info: &ClassInfo,
  instance: &InstanceInfo,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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

/// Build content for an EntityNativeItem node.
pub(super) fn build_entity_native(
  realm: &RealmInfo,
  layer: &LayerInfo,
  class_info: &ClassInfo,
  native: &EntityNativeInfo,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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

/// Build content for an EntityCategory node.
pub(super) fn build_entity_category(
  realm: &RealmInfo,
  layer: &LayerInfo,
  class_info: &ClassInfo,
  category_item: &EntityCategoryData,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Data Management".into(), Some(palette::BLUE_500))
}

/// Build content for an EntityGroup node.
pub(super) fn build_entity_group(
  realm: &RealmInfo,
  layer: &LayerInfo,
  class_info: &ClassInfo,
  group: &EntityNativeGroup,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Data Management".into(), Some(palette::GREEN_500))
}

// =============================================================================
// PROVENANCE PARSING
// =============================================================================

/// Parse provenance from JSON property value.
pub(super) fn parse_provenance(
  provenance: Option<&JsonValue>,
) -> (DataCategory, ProvenanceMeta) {
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

// =============================================================================
// DATA CONTENT BUILDERS
// =============================================================================

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
        Span::styled("Seed  ", Style::default().fg(palette::VALUE_NUMBER)),
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
          Style::default().fg(palette::ORANGE_500),
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
          Style::default().fg(palette::PURPLE_500),
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
      Style::default().fg(palette::GREEN_500),
    )
  } else {
    Span::styled(
      "⚠ Reseed-LOST",
      Style::default().fg(palette::RED_500),
    )
  };

  let backup = if category.needs_backup() {
    Span::styled(
      "● Backup",
      Style::default().fg(palette::ORANGE_500),
    )
  } else {
    Span::styled("○ No backup", Style::default().fg(COLOR_DIM))
  };

  let edit = if category.is_editable() {
    Span::styled(
      "✎ Editable",
      Style::default().fg(palette::BLUE_500),
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

  let gauge_color = if pct >= 80 {
    COLOR_GAUGE_FILLED
  } else if pct >= 50 {
    palette::VALUE_NUMBER
  } else {
    palette::RED_500
  };

  let (bar, empty) = ProgressBar::new(filled, total, 10)
    .chars('▰', '▱')
    .filled_style(Style::default().fg(gauge_color))
    .empty_style(Style::default().fg(gauge_color))
    .to_spans();

  lines.push(Line::from(vec![
    Span::styled("  Props    ", Style::default().fg(COLOR_DIM)),
    bar,
    empty,
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
        .fg(palette::VIOLET_500)
        .add_modifier(Modifier::DIM),
    )));
    if let Some(ref wf) = meta.workflow_id {
      lines.push(Line::from(vec![
        Span::styled("  Workflow  ", Style::default().fg(COLOR_DIM)),
        Span::styled(wf.to_string(), Style::default().fg(Color::Cyan)),
      ]));
    }
    if let Some(ref task) = meta.task_id {
      lines.push(Line::from(vec![
        Span::styled("  Task      ", Style::default().fg(COLOR_DIM)),
        Span::styled(task.to_string(), Style::default().fg(COLOR_MUTED)),
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
        .fg(palette::VIOLET_500)
        .add_modifier(Modifier::DIM),
    )));
    if let Some(ref tool) = meta.tool {
      lines.push(Line::from(vec![
        Span::styled("  Tool      ", Style::default().fg(COLOR_DIM)),
        Span::styled(tool.to_string(), Style::default().fg(Color::Cyan)),
      ]));
    }
    if let Some(ref user) = meta.user {
      lines.push(Line::from(vec![
        Span::styled("  User      ", Style::default().fg(COLOR_DIM)),
        Span::styled(user.to_string(), Style::default().fg(Color::Yellow)),
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
}

#[cfg(test)]
mod tests {
  use super::*;

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
