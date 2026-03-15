//! Badge widget for consistent `[label]` rendering.
//!
//! v0.20.1: Extracted from 20+ inline badge patterns across the TUI.
//! Provides a composable builder that produces `Span<'static>` for use
//! in `Line::from(vec![...])` patterns.

use ratatui::style::Style;
use ratatui::text::Span;

/// A styled `[label]` badge that renders as a `Span`.
///
/// # Usage
///
/// ```ignore
/// use crate::tui::widgets::Badge;
///
/// // Simple bracketed badge
/// let badge = Badge::new("str").style(STYLE_CYAN).to_span();
/// // → "[str]"
///
/// // Padded badge (fixed width for alignment)
/// let badge = Badge::new("str ").style(STYLE_CYAN).to_span();
/// // → "[str ]"
///
/// // Without brackets
/// let badge = Badge::new("own").style(STYLE_DIM).no_brackets().to_span();
/// // → "own"
/// ```
pub struct Badge<'a> {
    text: &'a str,
    style: Style,
    brackets: bool,
    /// Optional trailing space after the closing bracket.
    trailing_space: bool,
}

impl<'a> Badge<'a> {
    /// Create a new badge with the given label text.
    ///
    /// By default, renders with brackets: `[text]`
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::default(),
            brackets: true,
            trailing_space: false,
        }
    }

    /// Set the style for the badge.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Disable brackets — renders as plain `text` instead of `[text]`.
    pub fn no_brackets(mut self) -> Self {
        self.brackets = false;
        self
    }

    /// Add a trailing space after the badge: `[text] ` instead of `[text]`.
    pub fn spaced(mut self) -> Self {
        self.trailing_space = true;
        self
    }

    /// Render the badge as a `Span<'static>`.
    pub fn to_span(&self) -> Span<'static> {
        let formatted = if self.brackets {
            if self.trailing_space {
                format!("[{}] ", self.text)
            } else {
                format!("[{}]", self.text)
            }
        } else if self.trailing_space {
            format!("{} ", self.text)
        } else {
            self.text.to_string()
        };
        Span::styled(formatted, self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn test_badge_default_has_brackets() {
        let badge = Badge::new("str");
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "[str]");
    }

    #[test]
    fn test_badge_no_brackets() {
        let badge = Badge::new("own").no_brackets();
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "own");
    }

    #[test]
    fn test_badge_with_style() {
        let style = Style::default().fg(Color::Cyan);
        let badge = Badge::new("json").style(style);
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "[json]");
        assert_eq!(span.style, style);
    }

    #[test]
    fn test_badge_spaced() {
        let badge = Badge::new("str ").spaced();
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "[str ] ");
    }

    #[test]
    fn test_badge_no_brackets_spaced() {
        let badge = Badge::new("own").no_brackets().spaced();
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "own ");
    }

    #[test]
    fn test_badge_padded_text() {
        // Property type badges use 4-char padded text for alignment
        let badge = Badge::new("dt  ");
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "[dt  ]");
    }

    #[test]
    fn test_badge_empty_text() {
        let badge = Badge::new("");
        let span = badge.to_span();
        assert_eq!(span.content.as_ref(), "[]");
    }

    #[test]
    fn test_badge_builder_chain() {
        let span = Badge::new("sem")
            .style(Style::default().fg(Color::Blue))
            .spaced()
            .to_span();
        assert_eq!(span.content.as_ref(), "[sem] ");
        assert_eq!(span.style.fg, Some(Color::Blue));
    }
}
