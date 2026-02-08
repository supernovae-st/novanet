//! Test utilities for TUI snapshot testing.
//!
//! Uses ratatui's TestBackend for headless rendering.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

/// Render a widget to a buffer for testing.
pub fn render_widget<W: Widget>(widget: W, width: u16, height: u16) -> Buffer {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            frame.render_widget(widget, Rect::new(0, 0, width, height));
        })
        .unwrap();
    terminal.backend().buffer().clone()
}

/// Convert a Buffer to a string for snapshot testing.
pub fn buffer_to_string(buffer: &Buffer) -> String {
    let mut output = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.cell((x, y)).unwrap();
            output.push_str(cell.symbol());
        }
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_widget_returns_buffer() {
        let widget = ratatui::widgets::Paragraph::new("Hello");
        let buffer = render_widget(widget, 10, 1);
        assert_eq!(buffer.area.width, 10);
        assert_eq!(buffer.area.height, 1);
    }

    #[test]
    fn test_buffer_to_string_simple() {
        let widget = ratatui::widgets::Paragraph::new("AB");
        let buffer = render_widget(widget, 5, 1);
        let output = buffer_to_string(&buffer);
        assert!(output.contains("AB"));
    }
}
