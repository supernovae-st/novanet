//! Pure helper functions for tree rendering.
//!
//! Box-drawing characters, expand/collapse icons, health badge formatting,
//! and the make_line helper for building styled tree lines.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::highlight::highlight_matches_with_bg;
use super::super::COLOR_HIGHLIGHT_BG;

/// Get the branch character for tree drawing.
/// - `└─` for last item (no more siblings)
/// - `├─` for non-last item (more siblings below)
#[inline]
pub fn branch_char(is_last: bool) -> &'static str {
    if is_last { "└─" } else { "├─" }
}

/// Get the continuation character for tree drawing.
/// - `  ` (two spaces) if parent was last (no vertical line needed)
/// - `│ ` if parent was not last (vertical line continues)
#[inline]
pub fn cont_char(parent_is_last: bool) -> &'static str {
    if parent_is_last { "  " } else { "│ " }
}

/// Get the expand/collapse icon for a tree node.
/// - `▶` when collapsed (pointing right, can expand)
/// - `▼` when expanded (pointing down, can collapse)
#[inline]
pub fn expand_icon(is_collapsed: bool) -> &'static str {
    if is_collapsed { "▶" } else { "▼" }
}

/// Format a health badge for a Class node.
/// Returns empty string if no health data, or a bar like " ━━━░░░░░░░50%"
pub fn format_health_badge(health_percent: Option<u8>, issues_count: Option<usize>) -> String {
    let Some(percent) = health_percent else {
        return String::new();
    };
    let filled = percent / 10;
    let empty = 10 - filled;
    let issues = issues_count.unwrap_or(0);
    if issues > 0 {
        format!(
            " {}{}{}% ⚠{}",
            "━".repeat(filled as usize),
            "░".repeat(empty as usize),
            percent,
            issues
        )
    } else {
        format!(
            " {}{}{}%",
            "━".repeat(filled as usize),
            "░".repeat(empty as usize),
            percent
        )
    }
}

/// Build a styled tree line with cursor indicator, box-drawing prefix, icon, and text.
///
/// Handles cursor highlighting (full-line highlight when focused) and
/// fuzzy search match highlighting for non-cursor lines.
#[allow(clippy::too_many_arguments)]
pub fn make_line<'a>(
    idx: usize,
    cursor: usize,
    focused: bool,
    tree_prefix: &str,
    icon: &str,
    text: String,
    line_color: Color,
    text_color: Color,
    match_positions: Option<&[u32]>,
    bg_color: Option<Color>,
) -> Line<'a> {
    let is_cursor = idx == cursor;
    let cursor_char = if is_cursor { ">" } else { " " };
    let icon_space = if icon.is_empty() { "" } else { " " };

    if is_cursor && focused {
        let style = Style::default().bg(COLOR_HIGHLIGHT_BG).fg(Color::White);
        Line::from(Span::styled(
            format!(
                "{}{}{}{}{}",
                cursor_char, tree_prefix, icon, icon_space, text
            ),
            style,
        ))
    } else {
        let base_style = if let Some(bg) = bg_color {
            Style::default().bg(bg)
        } else {
            Style::default()
        };
        let mut spans = Vec::with_capacity(8);
        spans.push(Span::styled(cursor_char.to_string(), base_style));
        if !tree_prefix.is_empty() {
            spans.push(Span::styled(
                tree_prefix.to_string(),
                base_style.fg(line_color),
            ));
        }
        spans.push(Span::styled(
            format!("{}{}", icon, icon_space),
            base_style.fg(text_color),
        ));
        spans.extend(highlight_matches_with_bg(
            &text,
            match_positions,
            text_color,
            bg_color,
        ));
        Line::from(spans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::data::locale_to_flag;

    // =========================================================================
    // Tree structure helpers tests
    // =========================================================================

    #[test]
    fn test_branch_char_last() {
        assert_eq!(branch_char(true), "└─");
    }

    #[test]
    fn test_branch_char_not_last() {
        assert_eq!(branch_char(false), "├─");
    }

    #[test]
    fn test_cont_char_parent_was_last() {
        assert_eq!(cont_char(true), "  ");
    }

    #[test]
    fn test_cont_char_parent_was_not_last() {
        assert_eq!(cont_char(false), "│ ");
    }

    #[test]
    fn test_expand_icon_collapsed() {
        assert_eq!(expand_icon(true), "▶");
    }

    #[test]
    fn test_expand_icon_expanded() {
        assert_eq!(expand_icon(false), "▼");
    }

    // =========================================================================
    // Locale to flag tests
    // =========================================================================

    #[test]
    fn test_locale_to_flag_france() {
        assert_eq!(locale_to_flag("fr-FR"), "🇫🇷");
    }

    #[test]
    fn test_locale_to_flag_mexico() {
        assert_eq!(locale_to_flag("es-MX"), "🇲🇽");
    }

    #[test]
    fn test_locale_to_flag_usa() {
        assert_eq!(locale_to_flag("en-US"), "🇺🇸");
    }

    #[test]
    fn test_locale_to_flag_germany() {
        assert_eq!(locale_to_flag("de-DE"), "🇩🇪");
    }

    #[test]
    fn test_locale_to_flag_japan() {
        assert_eq!(locale_to_flag("ja-JP"), "🇯🇵");
    }

    #[test]
    fn test_locale_to_flag_fallback_invalid() {
        assert_eq!(locale_to_flag("invalid"), "🏳️");
    }

    #[test]
    fn test_locale_to_flag_single_part() {
        assert_eq!(locale_to_flag("FR"), "🇫🇷");
    }

    // =========================================================================
    // Health badge tests
    // =========================================================================

    #[test]
    fn test_format_health_badge_none() {
        assert_eq!(format_health_badge(None, None), "");
    }

    #[test]
    fn test_format_health_badge_zero_percent() {
        let badge = format_health_badge(Some(0), None);
        assert!(badge.contains("0%"));
        assert!(badge.contains("░░░░░░░░░░"));
    }

    #[test]
    fn test_format_health_badge_fifty_percent() {
        let badge = format_health_badge(Some(50), None);
        assert!(badge.contains("50%"));
        assert!(badge.contains("━━━━━"));
        assert!(badge.contains("░░░░░"));
    }

    #[test]
    fn test_format_health_badge_hundred_percent() {
        let badge = format_health_badge(Some(100), None);
        assert!(badge.contains("100%"));
        assert!(badge.contains("━━━━━━━━━━"));
    }

    #[test]
    fn test_format_health_badge_with_issues() {
        let badge = format_health_badge(Some(70), Some(3));
        assert!(badge.contains("70%"));
        assert!(badge.contains("⚠3"));
    }

    #[test]
    fn test_format_health_badge_with_zero_issues() {
        let badge = format_health_badge(Some(80), Some(0));
        assert!(badge.contains("80%"));
        assert!(!badge.contains("⚠"));
    }
}
