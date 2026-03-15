//! Fuzzy match highlighting for tree panel text.
//!
//! Creates styled spans with yellow highlight on matched character positions.

use ratatui::style::{Color, Style};
use ratatui::text::Span;
use rustc_hash::FxHashSet;

/// Create styled spans with fuzzy match highlighting and optional background.
/// Matched character positions are shown with a yellow highlight.
/// Optional background color is applied to non-matched text segments.
pub fn highlight_matches_with_bg(
    text: &str,
    matches: Option<&[u32]>,
    base_color: Color,
    bg_color: Option<Color>,
) -> Vec<Span<'static>> {
    let base_style = if let Some(bg) = bg_color {
        Style::default().fg(base_color).bg(bg)
    } else {
        Style::default().fg(base_color)
    };

    let Some(positions) = matches else {
        return vec![Span::styled(text.to_string(), base_style)];
    };

    if positions.is_empty() {
        return vec![Span::styled(text.to_string(), base_style)];
    }

    let match_set: FxHashSet<usize> = positions.iter().map(|&p| p as usize).collect();
    let mut spans = Vec::with_capacity(positions.len() * 2 + 1);
    let mut current_text = String::new();
    let mut in_match = false;

    for (i, c) in text.chars().enumerate() {
        let is_match = match_set.contains(&i);

        if is_match != in_match {
            if !current_text.is_empty() {
                let style = if in_match {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    base_style
                };
                spans.push(Span::styled(std::mem::take(&mut current_text), style));
            }
            in_match = is_match;
        }
        current_text.push(c);
    }

    if !current_text.is_empty() {
        let style = if in_match {
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            base_style
        };
        spans.push(Span::styled(current_text, style));
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_matches_no_positions() {
        let spans = highlight_matches_with_bg("hello", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        let span = &spans[0];
        assert_eq!(span.content, "hello");
    }

    #[test]
    fn test_highlight_matches_empty_positions() {
        let spans = highlight_matches_with_bg("hello", Some(&[]), Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hello");
    }

    #[test]
    fn test_highlight_matches_single_char() {
        let spans = highlight_matches_with_bg("hello", Some(&[0]), Color::White, None);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "h");
        assert_eq!(spans[1].content, "ello");
    }

    #[test]
    fn test_highlight_matches_consecutive() {
        let spans = highlight_matches_with_bg("hello", Some(&[0, 1, 2]), Color::White, None);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].content, "hel");
        assert_eq!(spans[1].content, "lo");
    }

    #[test]
    fn test_highlight_matches_scattered() {
        let spans = highlight_matches_with_bg("hello", Some(&[0, 2, 4]), Color::White, None);
        assert!(spans.len() >= 3);
    }

    #[test]
    fn test_highlight_matches_full_match() {
        let spans = highlight_matches_with_bg("hi", Some(&[0, 1]), Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hi");
    }

    #[test]
    fn test_highlight_matches_no_match() {
        let spans = highlight_matches_with_bg("hello", None, Color::White, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content, "hello");
        assert_eq!(spans[0].style.fg, Some(Color::White));
        assert_eq!(spans[0].style.bg, None);
    }

    #[test]
    fn test_highlight_matches_with_positions() {
        let spans = highlight_matches_with_bg("hello", Some(&[0, 2]), Color::White, None);
        assert!(
            spans.len() >= 3,
            "Expected at least 3 spans, got {}",
            spans.len()
        );

        assert_eq!(spans[0].content, "h");
        assert_eq!(spans[0].style.bg, Some(Color::Yellow));
        assert_eq!(spans[0].style.fg, Some(Color::Black));

        assert_eq!(spans[1].content, "e");
        assert_eq!(spans[1].style.fg, Some(Color::White));

        assert_eq!(spans[2].content, "l");
        assert_eq!(spans[2].style.bg, Some(Color::Yellow));
        assert_eq!(spans[2].style.fg, Some(Color::Black));

        assert_eq!(spans[3].content, "lo");
        assert_eq!(spans[3].style.fg, Some(Color::White));
    }
}
