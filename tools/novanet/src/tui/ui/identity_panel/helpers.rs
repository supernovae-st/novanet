//! Shared helper functions for identity panel rendering.
//!
//! Breadcrumbs, key-value pairs, schema banners, explanations,
//! and hex color parsing used by both schema and data submodules.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use super::super::COLOR_SEPARATOR;
use super::{COLOR_DIM, COLOR_MUTED, COLOR_SCHEMA_STRUCTURE};

// =============================================================================
// PRIMITIVES
// =============================================================================

/// Push an empty line.
pub(super) fn push_empty(lines: &mut Vec<Line<'static>>) {
  lines.push(Line::from(""));
}

/// Push a simple labeled line.
pub(super) fn push_line(
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
pub(super) fn push_kv(lines: &mut Vec<Line<'static>>, key: &str, value: &str) {
  // Pad key to 10 chars for alignment
  let padded = format!("  {:<9}", key);
  lines.push(Line::from(vec![
    Span::styled(padded, Style::default().fg(COLOR_DIM)),
    Span::styled(value.to_string(), Style::default().fg(Color::White)),
  ]));
}

/// Push an explanation line (normie-friendly).
pub(super) fn push_explanation(lines: &mut Vec<Line<'static>>, text: &str) {
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

// =============================================================================
// BREADCRUMBS
// =============================================================================

/// Push the schema banner: ╔══ SCHEMA: <type> ══╗
pub(super) fn push_schema_banner(lines: &mut Vec<Line<'static>>, node_type: &str) {
  let label = format!("  ╔══ SCHEMA: {} ══╗", node_type);
  lines.push(Line::from(Span::styled(
    label,
    Style::default()
      .fg(COLOR_SCHEMA_STRUCTURE)
      .add_modifier(Modifier::BOLD),
  )));
}

/// Push realm breadcrumb: ● realm
pub(super) fn push_breadcrumb_realm(
  lines: &mut Vec<Line<'static>>,
  realm_name: &str,
  realm_color: &str,
) {
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
pub(super) fn push_breadcrumb_layer(
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
pub(super) fn push_breadcrumb_class(
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

// =============================================================================
// COLOR PARSING
// =============================================================================

/// Parse hex color string (#rrggbb) to Color.
pub(super) fn parse_hex_color(hex: &str) -> Color {
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
}
