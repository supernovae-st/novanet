//! Progress bar widget for consistent bar rendering.
//!
//! v0.20.1: Extracted from 7+ inline progress bar patterns across the TUI.
//! Provides a composable builder that produces `(Span, Span)` for use
//! in `Line::from(vec![...])` patterns.

use ratatui::style::Style;
use ratatui::text::Span;

/// A styled progress bar that renders as two `Span`s (filled + empty).
///
/// # Usage
///
/// ```ignore
/// use crate::tui::widgets::ProgressBar;
///
/// // Default chars: █ (filled) + ░ (empty)
/// let (filled, empty) = ProgressBar::new(7, 10, 12)
///     .filled_style(style_green)
///     .empty_style(style_dim)
///     .to_spans();
/// // → ("███████████░", style) — 7/10 of 12-char width
///
/// // Custom chars: ▰▱
/// let (filled, empty) = ProgressBar::new(5, 10, 10)
///     .chars('▰', '▱')
///     .filled_style(style_green)
///     .empty_style(style_dim)
///     .to_spans();
/// // → ("▰▰▰▰▰▱▱▱▱▱", styles)
/// ```
pub struct ProgressBar {
    filled_count: usize,
    total: usize,
    width: usize,
    filled_char: char,
    empty_char: char,
    filled_style: Style,
    empty_style: Style,
    /// Minimum 1 filled char when value > 0.
    min_one: bool,
}

impl ProgressBar {
    /// Create a new progress bar.
    ///
    /// - `filled`: number of filled units
    /// - `total`: total units (denominator)
    /// - `width`: character width of the bar
    pub fn new(filled: usize, total: usize, width: usize) -> Self {
        Self {
            filled_count: filled,
            total,
            width,
            filled_char: '█',
            empty_char: '░',
            filled_style: Style::default(),
            empty_style: Style::default(),
            min_one: true,
        }
    }

    /// Set custom fill/empty characters (e.g., `'▰', '▱'`).
    pub fn chars(mut self, filled: char, empty: char) -> Self {
        self.filled_char = filled;
        self.empty_char = empty;
        self
    }

    /// Set the style for the filled portion.
    pub fn filled_style(mut self, style: Style) -> Self {
        self.filled_style = style;
        self
    }

    /// Set the style for the empty portion.
    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = style;
        self
    }

    /// Disable minimum-1-filled behavior (show 0 filled when value is 0).
    pub fn no_min_one(mut self) -> Self {
        self.min_one = false;
        self
    }

    /// Calculate the number of filled characters.
    fn filled_width(&self) -> usize {
        if self.total == 0 {
            return 0;
        }
        let raw = (self.filled_count * self.width) / self.total.max(1);
        if self.min_one && self.filled_count > 0 {
            raw.max(1)
        } else {
            raw
        }
    }

    /// Render the progress bar as two `Span<'static>` values: (filled, empty).
    pub fn to_spans(&self) -> (Span<'static>, Span<'static>) {
        let filled = self.filled_width();
        let empty = self.width.saturating_sub(filled);
        (
            Span::styled(
                self.filled_char.to_string().repeat(filled),
                self.filled_style,
            ),
            Span::styled(
                self.empty_char.to_string().repeat(empty),
                self.empty_style,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn test_progress_bar_default_chars() {
        let (filled, empty) = ProgressBar::new(5, 10, 10).to_spans();
        assert_eq!(filled.content.as_ref(), "█████");
        assert_eq!(empty.content.as_ref(), "░░░░░");
    }

    #[test]
    fn test_progress_bar_custom_chars() {
        let (filled, empty) = ProgressBar::new(3, 10, 10)
            .chars('▰', '▱')
            .to_spans();
        assert_eq!(filled.content.as_ref(), "▰▰▰");
        assert_eq!(empty.content.as_ref(), "▱▱▱▱▱▱▱");
    }

    #[test]
    fn test_progress_bar_min_one_filled() {
        // 1 out of 100 with width 10 → raw = 0, but min_one bumps to 1
        let (filled, empty) = ProgressBar::new(1, 100, 10).to_spans();
        assert_eq!(filled.content.as_ref(), "█");
        assert_eq!(empty.content.as_ref(), "░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_no_min_one() {
        let (filled, empty) = ProgressBar::new(1, 100, 10).no_min_one().to_spans();
        assert_eq!(filled.content.as_ref(), "");
        assert_eq!(empty.content.as_ref(), "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_zero_total() {
        let (filled, empty) = ProgressBar::new(0, 0, 10).to_spans();
        assert_eq!(filled.content.as_ref(), "");
        assert_eq!(empty.content.as_ref(), "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_zero_filled() {
        let (filled, empty) = ProgressBar::new(0, 10, 10).to_spans();
        assert_eq!(filled.content.as_ref(), "");
        assert_eq!(empty.content.as_ref(), "░░░░░░░░░░");
    }

    #[test]
    fn test_progress_bar_full() {
        let (filled, empty) = ProgressBar::new(10, 10, 10).to_spans();
        assert_eq!(filled.content.as_ref(), "██████████");
        assert_eq!(empty.content.as_ref(), "");
    }

    #[test]
    fn test_progress_bar_with_styles() {
        let fill_style = Style::default().fg(Color::Green);
        let empty_style = Style::default().fg(Color::DarkGray);
        let (filled, empty) = ProgressBar::new(7, 10, 10)
            .filled_style(fill_style)
            .empty_style(empty_style)
            .to_spans();
        assert_eq!(filled.style, fill_style);
        assert_eq!(empty.style, empty_style);
        assert_eq!(filled.content.as_ref(), "███████");
        assert_eq!(empty.content.as_ref(), "░░░");
    }

    #[test]
    fn test_progress_bar_different_widths() {
        // Width 12 (builders.rs pattern)
        let (filled, _) = ProgressBar::new(8, 20, 12).to_spans();
        assert_eq!(filled.content.len(), "████".len()); // 8*12/20 = 4.8 → 4

        // Width 16 (builders.rs realm pattern)
        let (filled, empty) = ProgressBar::new(36, 59, 16).to_spans();
        let total_width = filled.content.chars().count() + empty.content.chars().count();
        assert_eq!(total_width, 16);

        // Width 20 (graph.rs pattern)
        let (filled, empty) = ProgressBar::new(23, 59, 20).to_spans();
        let total_width = filled.content.chars().count() + empty.content.chars().count();
        assert_eq!(total_width, 20);
    }

    #[test]
    fn test_progress_bar_builder_chain() {
        let (filled, empty) = ProgressBar::new(5, 10, 8)
            .chars('▰', '▱')
            .filled_style(Style::default().fg(Color::Cyan))
            .empty_style(Style::default().fg(Color::DarkGray))
            .no_min_one()
            .to_spans();
        assert_eq!(filled.content.as_ref(), "▰▰▰▰");
        assert_eq!(empty.content.as_ref(), "▱▱▱▱");
    }
}
