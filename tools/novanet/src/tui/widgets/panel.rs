//! Reusable panel widgets with focus state management.

use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};

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
            unfocused_color: Color::Rgb(60, 60, 70),
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

        Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
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
}
