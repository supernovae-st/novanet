//! Schema node content builders for the identity panel.
//!
//! Handles Realm, Layer, Class, ArcFamily, ArcClass, ClassesSection, ArcsSection.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::data::{ArcClassInfo, ArcFamilyInfo, ClassInfo, LayerInfo, RealmInfo};
use crate::tui::palette;

use super::helpers::{
  push_breadcrumb_class, push_breadcrumb_layer, push_breadcrumb_realm, push_empty,
  push_explanation, push_kv, push_line, push_schema_banner,
};
use super::{COLOR_DIM, COLOR_FILE, COLOR_FLOW, COLOR_CMD, COLOR_MUTED, COLOR_SCHEMA_STRUCTURE};

// =============================================================================
// SCHEMA PIPELINE
// =============================================================================

/// Push schema pipeline: YAML → Cypher → Neo4j.
fn push_pipeline_schema(lines: &mut Vec<Line<'static>>, yaml_source: &str, cypher_file: &str) {
  lines.push(Line::from(vec![
    Span::styled("  ", Style::default()),
    Span::styled("YAML", Style::default().fg(palette::VALUE_NUMBER)),
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
    Span::styled("Cypher", Style::default().fg(palette::VALUE_ARRAY)),
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
        .fg(palette::GREEN_500)
        .add_modifier(Modifier::BOLD),
    ),
    Span::styled("  ", Style::default()),
    Span::styled("bolt://localhost:7687", Style::default().fg(COLOR_MUTED)),
  ]));
}

// =============================================================================
// SCHEMA NODE BUILDERS
// =============================================================================

/// Build content for a Realm node.
pub(super) fn build_realm(realm: &RealmInfo) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}

/// Build content for a Layer node.
pub(super) fn build_layer(
  realm: &RealmInfo,
  layer: &LayerInfo,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}

/// Build content for a Class node.
pub(super) fn build_class(
  realm: &RealmInfo,
  layer: &LayerInfo,
  class_info: &ClassInfo,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}

/// Build content for an ArcFamily node.
pub(super) fn build_arc_family(
  family: &ArcFamilyInfo,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}

/// Build content for an ArcClass node.
pub(super) fn build_arc_class(
  family: &ArcFamilyInfo,
  arc_class: &ArcClassInfo,
) -> (Vec<Line<'static>>, String, Option<Color>) {
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
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}

/// Build content for the ClassesSection header.
pub(super) fn build_classes_section() -> (Vec<Line<'static>>, String, Option<Color>) {
  let mut lines = Vec::new();
  push_schema_banner(&mut lines, "NODE CLASSES");
  push_empty(&mut lines);
  push_explanation(
    &mut lines,
    "Browse all 59 node class definitions organized by realm and layer.",
  );
  push_empty(&mut lines);
  push_pipeline_schema(&mut lines, "models/node-classes/", "01-classes.cypher");
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}

/// Build content for the ArcsSection header.
pub(super) fn build_arcs_section() -> (Vec<Line<'static>>, String, Option<Color>) {
  let mut lines = Vec::new();
  push_schema_banner(&mut lines, "ARC CLASSES");
  push_empty(&mut lines);
  push_explanation(
    &mut lines,
    "Browse all 159 arc class definitions organized by family.",
  );
  push_empty(&mut lines);
  push_pipeline_schema(&mut lines, "models/arc-classes/", "02-arc-classes.cypher");
  (lines, "Schema Structure".into(), Some(COLOR_SCHEMA_STRUCTURE))
}
