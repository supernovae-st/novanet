//! Arc Families section of the tree panel.
//!
//! Builds tree lines for the ArcFamily > ArcClass hierarchy.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::helpers::{branch_char, cont_char, expand_icon, make_line};
use super::highlight::highlight_matches_with_bg;
use super::super::{
    COLOR_ARC_FAMILY, COLOR_DESC_TEXT, COLOR_HIGHLIGHT_BG, COLOR_MUTED_TEXT, cardinality_abbrev,
};
use crate::tui::app::App;
use crate::tui::unicode::display_width;

/// Build all tree lines for the "Arcs" section.
///
/// Returns the next `idx` value after all arc lines have been added.
pub(super) fn build_arc_lines<'a>(
    all_lines: &mut Vec<Line<'a>>,
    idx: &mut usize,
    app: &App,
    area_width: u16,
    focused: bool,
) {
    let branch = branch_char;
    let cont = cont_char;

    // === ARCS HEADER ===
    let arcs_collapsed = app.tree.is_collapsed("arcs");
    let arcs_icon = expand_icon(arcs_collapsed);
    let arcs_count: usize = app
        .tree
        .arc_families
        .iter()
        .map(|f| f.arc_classes.len())
        .sum();
    all_lines.push(make_line(
        *idx,
        app.tree_cursor,
        focused,
        "",
        arcs_icon,
        format!("Arcs ({})", arcs_count),
        Color::Yellow,
        Color::Yellow,
        app.search.matches.get(idx).map(|v| v.as_slice()),
        None,
    ));
    *idx += 1;

    if arcs_collapsed {
        return;
    }

    // === ARC FAMILIES ===
    let family_count = app.tree.arc_families.len();
    for (fi, family) in app.tree.arc_families.iter().enumerate() {
        let family_is_last = fi == family_count - 1;
        let family_key = format!("family:{}", family.key);
        let family_collapsed = app.tree.is_collapsed(&family_key);
        let family_icon = expand_icon(family_collapsed);

        let is_cursor = *idx == app.tree_cursor;
        let cursor_char = if is_cursor { ">" } else { " " };
        let arcs_in_family = family.arc_classes.len();

        let left_content = format!(
            "{}{}{}  {}",
            cursor_char,
            branch(family_is_last),
            family_icon,
            family.display_name
        );

        let stats_str = format!("◇{}", arcs_in_family);

        if is_cursor && focused {
            let full_line = format!("{} {}", left_content, stats_str);
            all_lines.push(Line::from(Span::styled(
                full_line,
                Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
            )));
        } else {
            let base_style = Style::default();
            let mut spans: Vec<Span> = vec![
                Span::styled(cursor_char, base_style),
                Span::styled(
                    branch(family_is_last).to_string(),
                    base_style.fg(Color::Yellow),
                ),
                Span::styled(
                    format!("{}  ", family_icon),
                    base_style.fg(COLOR_ARC_FAMILY),
                ),
            ];
            spans.extend(highlight_matches_with_bg(
                &family.display_name,
                app.search.matches.get(idx).map(|v| v.as_slice()),
                COLOR_ARC_FAMILY,
                None,
            ));
            spans.push(Span::styled(" ", base_style));
            spans.push(Span::styled(stats_str, base_style.fg(COLOR_MUTED_TEXT)));

            all_lines.push(Line::from(spans));
        }
        *idx += 1;

        if family_collapsed {
            continue;
        }

        // === ARC CLASSES ===
        let arc_count = family.arc_classes.len();
        for (ai, arc_class) in family.arc_classes.iter().enumerate() {
            let arc_is_last = ai == arc_count - 1;

            let is_cursor = *idx == app.tree_cursor;
            let cursor_char = if is_cursor { ">" } else { " " };
            let prefix = format!("{}{}", cont(family_is_last), branch(arc_is_last));

            let left_content =
                format!("{}{}  {}", cursor_char, prefix, arc_class.display_name);

            let from_abbrev = arc_class.from_class.chars().take(8).collect::<String>();
            let to_abbrev = arc_class.to_class.chars().take(8).collect::<String>();
            let flow_str = format!("{}→{}", from_abbrev, to_abbrev);

            let card_str = cardinality_abbrev(&arc_class.cardinality);

            let tree_width = area_width.saturating_sub(5) as usize;
            let left_width = display_width(&left_content);
            let flow_width = display_width(&flow_str);
            let card_width = display_width(card_str);
            let right_side = "│";
            let right_width = 1;

            let total_content = left_width + flow_width + card_width + right_width + 3;
            let padding = tree_width.saturating_sub(total_content);

            if is_cursor && focused {
                let full_line =
                    format!("{} {} {} {}", left_content, flow_str, card_str, right_side);
                all_lines.push(Line::from(Span::styled(
                    full_line,
                    Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White),
                )));
            } else {
                let base_style = Style::default();
                let mut spans: Vec<Span> = vec![
                    Span::styled(cursor_char, base_style),
                    Span::styled(prefix.clone(), base_style.fg(COLOR_ARC_FAMILY)),
                    Span::styled("  ", base_style),
                ];
                spans.extend(highlight_matches_with_bg(
                    &arc_class.display_name,
                    app.search.matches.get(idx).map(|v| v.as_slice()),
                    COLOR_DESC_TEXT,
                    None,
                ));
                spans.push(Span::styled(" ", base_style));
                spans.push(Span::styled(flow_str, base_style.fg(COLOR_MUTED_TEXT)));
                spans.push(Span::styled(" ", base_style));
                spans.push(Span::styled(card_str, base_style.fg(Color::Cyan)));
                spans.push(Span::styled(" ".repeat(padding), base_style));
                spans.push(Span::styled("│", base_style.fg(COLOR_ARC_FAMILY)));

                all_lines.push(Line::from(spans));
            }
            *idx += 1;
        }
    }
}
