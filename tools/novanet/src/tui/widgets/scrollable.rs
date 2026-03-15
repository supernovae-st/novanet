//! Scrollable content widget with scroll indicators.

use ratatui::text::Line;

/// Clamp a scroll offset so it never exceeds `total_lines - visible_height`.
///
/// `visible_height` is typically `area.height.saturating_sub(2) as usize`
/// (area minus top/bottom borders).
#[inline]
pub fn clamp_scroll(scroll: &mut usize, total_lines: usize, visible_height: usize) {
    let max = total_lines.saturating_sub(visible_height);
    if *scroll > max {
        *scroll = max;
    }
}

/// Manages scroll state for content that may exceed visible area.
#[derive(Debug, Default, Clone)]
pub struct ScrollState {
    /// Current scroll position (line offset).
    pub offset: usize,
    /// Total number of content lines.
    pub total_lines: usize,
    /// Visible height in lines.
    pub visible_height: usize,
}

impl ScrollState {
    pub fn new(total_lines: usize, visible_height: usize) -> Self {
        Self {
            offset: 0,
            total_lines,
            visible_height,
        }
    }

    /// Whether content is scrollable (total > visible).
    pub fn is_scrollable(&self) -> bool {
        self.total_lines > self.visible_height
    }

    /// Maximum scroll offset.
    pub fn max_offset(&self) -> usize {
        self.total_lines.saturating_sub(self.visible_height)
    }

    /// Scroll down by N lines.
    pub fn scroll_down(&mut self, amount: usize) {
        self.offset = (self.offset + amount).min(self.max_offset());
    }

    /// Scroll up by N lines.
    pub fn scroll_up(&mut self, amount: usize) {
        self.offset = self.offset.saturating_sub(amount);
    }

    /// Get scroll indicator string.
    /// Returns "" if not scrollable, otherwise "↓ [1/N]", "↑ [N/N]", or "↕ [M/N]"
    pub fn indicator(&self) -> String {
        if !self.is_scrollable() {
            return String::new();
        }

        let current_page = self.offset + 1;
        let total_pages = self.max_offset() + 1;

        let arrow = if self.offset == 0 {
            "↓"
        } else if self.offset >= self.max_offset() {
            "↑"
        } else {
            "↕"
        };

        format!(" {} [{}/{}] ", arrow, current_page, total_pages)
    }

    /// Get visible slice of content.
    pub fn visible_lines<'a>(&self, lines: &'a [Line<'a>]) -> &'a [Line<'a>] {
        let start = self.offset.min(lines.len());
        let end = (start + self.visible_height).min(lines.len());
        &lines[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_state_not_scrollable() {
        let state = ScrollState::new(5, 10);
        assert!(!state.is_scrollable());
        assert_eq!(state.indicator(), "");
    }

    #[test]
    fn test_scroll_state_scrollable() {
        let state = ScrollState::new(20, 10);
        assert!(state.is_scrollable());
        assert_eq!(state.max_offset(), 10);
    }

    #[test]
    fn test_scroll_down() {
        let mut state = ScrollState::new(20, 10);
        state.scroll_down(5);
        assert_eq!(state.offset, 5);
    }

    #[test]
    fn test_scroll_down_clamps_to_max() {
        let mut state = ScrollState::new(20, 10);
        state.scroll_down(100);
        assert_eq!(state.offset, 10); // max_offset
    }

    #[test]
    fn test_scroll_up() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 5;
        state.scroll_up(3);
        assert_eq!(state.offset, 2);
    }

    #[test]
    fn test_scroll_up_clamps_to_zero() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 2;
        state.scroll_up(10);
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn test_indicator_at_top() {
        let state = ScrollState::new(20, 10);
        assert!(state.indicator().contains("↓"));
        assert!(state.indicator().contains("[1/"));
    }

    #[test]
    fn test_indicator_at_bottom() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 10;
        assert!(state.indicator().contains("↑"));
    }

    #[test]
    fn test_indicator_in_middle() {
        let mut state = ScrollState::new(20, 10);
        state.offset = 5;
        assert!(state.indicator().contains("↕"));
    }

    #[test]
    fn test_default_state() {
        let state = ScrollState::default();
        assert_eq!(state.offset, 0);
        assert_eq!(state.total_lines, 0);
        assert_eq!(state.visible_height, 0);
    }

    #[test]
    fn test_clamp_scroll_within_bounds() {
        let mut scroll = 3;
        clamp_scroll(&mut scroll, 20, 10);
        assert_eq!(scroll, 3);
    }

    #[test]
    fn test_clamp_scroll_exceeds_max() {
        let mut scroll = 15;
        clamp_scroll(&mut scroll, 20, 10);
        assert_eq!(scroll, 10);
    }

    #[test]
    fn test_clamp_scroll_content_smaller_than_viewport() {
        let mut scroll = 5;
        clamp_scroll(&mut scroll, 5, 10);
        assert_eq!(scroll, 0);
    }
}
