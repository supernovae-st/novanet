//! Constants for TUI.
//!
//! Scroll amounts, margins, and UI defaults.

/// Number of items to scroll with page up/down (d/u keys).
/// Chosen as ~half a typical terminal screen height (24 lines) for comfortable navigation
/// without losing context when jumping through large lists.
pub const PAGE_SCROLL_AMOUNT: usize = 10;

/// Minimum visible lines to keep above max scroll position in YAML panel.
/// Prevents scrolling past content end, ensuring user always sees meaningful text.
/// 10 lines provides enough context for YAML blocks with nested properties.
pub const YAML_SCROLL_MARGIN: usize = 10;

/// Minimum visible lines to keep above max scroll position in Info panel.
/// Smaller than YAML margin because info panel content is typically shorter.
/// 5 lines balances scroll precision with visual stability.
pub const INFO_SCROLL_MARGIN: usize = 5;

/// Number of lines to scroll per mouse wheel tick.
/// Larger than keyboard (1 line) for comfortable rapid scrolling.
/// 3 lines is the traditional mouse wheel increment in most applications.
pub const MOUSE_SCROLL_LINES: usize = 3;

/// Default tree height (updated by UI on render).
/// Used for initial scroll calculations before first render pass.
/// 20 lines approximates a typical terminal with status bar and borders.
pub const DEFAULT_TREE_HEIGHT: usize = 20;
