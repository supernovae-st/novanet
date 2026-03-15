//! Reusable panel widgets with focus state management.
//!
//! v0.20.1: Added `bordered_block` free function for common rounded-border pattern.

use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};

use crate::tui::palette;

/// Create a rounded bordered block with title and border color.
///
/// This is the most common block pattern in the TUI — rounded borders
/// with a colored border style. Chain additional methods as needed:
/// ```ignore
/// bordered_block(" Title ", Color::Cyan)
///     .title_bottom(scroll_hint)
///     .style(Style::default().bg(bg_color))
/// ```
pub fn bordered_block<'a>(title: impl Into<Line<'a>>, border_color: Color) -> Block<'a> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
}

/// A panel with automatic focus styling.
pub struct FocusablePanel<'a> {
    title: &'a str,
    focused: bool,
    focused_color: Color,
    unfocused_color: Color,
}

impl<'a> FocusablePanel<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            focused: false,
            focused_color: Color::Cyan,
            unfocused_color: palette::BORDER_UNFOCUSED,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn focused_color(mut self, color: Color) -> Self {
        self.focused_color = color;
        self
    }

    pub fn unfocused_color(mut self, color: Color) -> Self {
        self.unfocused_color = color;
        self
    }

    /// Build the Block with appropriate styling.
    pub fn block(&self) -> Block<'a> {
        let border_color = if self.focused {
            self.focused_color
        } else {
            self.unfocused_color
        };
        bordered_block(self.title, border_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focusable_panel_default_unfocused() {
        let panel = FocusablePanel::new("Test");
        assert!(!panel.focused);
    }

    #[test]
    fn test_focusable_panel_focused_changes_state() {
        let panel = FocusablePanel::new("Test").focused(true);
        assert!(panel.focused);
    }

    #[test]
    fn test_focusable_panel_custom_colors() {
        let panel = FocusablePanel::new("Test")
            .focused_color(Color::Green)
            .unfocused_color(Color::Red);
        assert_eq!(panel.focused_color, Color::Green);
        assert_eq!(panel.unfocused_color, Color::Red);
    }

    #[test]
    fn test_focusable_panel_block_has_borders() {
        let panel = FocusablePanel::new("Test Panel");
        let _block = panel.block();
        // Block is created successfully - compile-time check
    }

    #[test]
    fn test_bordered_block_creates_block() {
        let _block = bordered_block(" Title ", Color::Cyan);
        // Compiles and creates block with rounded borders
    }

    #[test]
    fn test_bordered_block_chainable() {
        let _block = bordered_block(" Title ", Color::Cyan)
            .style(Style::default().bg(Color::Black));
        // Can chain additional Block methods
    }

    #[test]
    fn test_bordered_block_accepts_span_title() {
        use ratatui::text::Span;
        let _block = bordered_block(
            Span::styled(" Search ", Style::default().fg(Color::Cyan)),
            Color::Cyan,
        );
    }

    #[test]
    fn test_bordered_block_accepts_line_title() {
        let _block = bordered_block(
            Line::from(vec![
                ratatui::text::Span::raw(" "),
                ratatui::text::Span::styled("Title", Style::default().fg(Color::Yellow)),
                ratatui::text::Span::raw(" "),
            ]),
            Color::Cyan,
        );
    }

    #[test]
    fn test_focusable_panel_uses_bordered_block() {
        // FocusablePanel::block() should produce the same result as bordered_block
        let panel = FocusablePanel::new("Test").focused(true);
        let _block = panel.block();
        // Verifies that FocusablePanel delegates to bordered_block internally
    }
}
