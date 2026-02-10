//! ASCII box drawing utilities for blueprint output.
//!
//! Provides helpers for rendering rich terminal output:
//! - Box containers with titles
//! - Progress bars
//! - Tree structures
//! - Tables

use std::fmt::Write;

/// Box drawing characters (Unicode).
pub mod chars {
    // Rounded corners (default)
    pub const TOP_LEFT: char = '╭';
    pub const TOP_RIGHT: char = '╮';
    pub const BOTTOM_LEFT: char = '╰';
    pub const BOTTOM_RIGHT: char = '╯';
    pub const HORIZONTAL: char = '─';
    pub const VERTICAL: char = '│';
    pub const T_RIGHT: char = '├';
    pub const T_LEFT: char = '┤';
    pub const T_DOWN: char = '┬';
    pub const T_UP: char = '┴';
    pub const CROSS: char = '┼';

    // Double line (for emphasis)
    pub const DOUBLE_HORIZONTAL: char = '═';
    pub const DOUBLE_VERTICAL: char = '║';
    pub const DOUBLE_TOP_LEFT: char = '╔';
    pub const DOUBLE_TOP_RIGHT: char = '╗';
    pub const DOUBLE_BOTTOM_LEFT: char = '╚';
    pub const DOUBLE_BOTTOM_RIGHT: char = '╝';

    // Tree characters
    pub const TREE_BRANCH: &str = "├──";
    pub const TREE_LAST: &str = "└──";
    pub const TREE_PIPE: &str = "│   ";
    pub const TREE_SPACE: &str = "    ";

    // Progress bar
    pub const PROGRESS_FILLED: char = '█';
    pub const PROGRESS_EMPTY: char = '░';

    // Arrows
    pub const ARROW_RIGHT: &str = "──►";
    pub const ARROW_LEFT: &str = "◄──";
    pub const ARROW_DOUBLE: &str = "◄──►";
}

/// Terminal width (default 80, detect if possible).
pub fn terminal_width() -> usize {
    // Try to get terminal width, default to 80
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
        .min(120) // Cap at 120 for readability
}

// ─────────────────────────────────────────────────────────────────────────────
// Box Builder
// ─────────────────────────────────────────────────────────────────────────────

/// Builds ASCII boxes with optional title and sections.
pub struct BoxBuilder {
    title: Option<String>,
    right_title: Option<String>,
    width: usize,
    sections: Vec<BoxSection>,
}

struct BoxSection {
    name: Option<String>,
    content: String,
}

impl BoxBuilder {
    /// Create a new box builder with default width.
    pub fn new() -> Self {
        Self {
            title: None,
            right_title: None,
            width: terminal_width().saturating_sub(2),
            sections: Vec::new(),
        }
    }

    /// Set the main title (left-aligned).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set a right-aligned title (e.g., version).
    pub fn right_title(mut self, title: impl Into<String>) -> Self {
        self.right_title = Some(title.into());
        self
    }

    /// Set custom width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Add a named section.
    pub fn section(mut self, name: impl Into<String>, content: impl Into<String>) -> Self {
        self.sections.push(BoxSection {
            name: Some(name.into()),
            content: content.into(),
        });
        self
    }

    /// Add content without a section name.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.sections.push(BoxSection {
            name: None,
            content: content.into(),
        });
        self
    }

    /// Render the box to a string.
    pub fn render(&self) -> String {
        let mut out = String::new();
        let w = self.width;

        // Top border with title
        out.push(chars::TOP_LEFT);
        if let Some(ref title) = self.title {
            let right = self.right_title.as_deref().unwrap_or("");
            let available = w.saturating_sub(title.len() + right.len() + 4);
            out.push_str(&format!(
                "  {}{}{}  ",
                title,
                chars::HORIZONTAL.to_string().repeat(available),
                right
            ));
        } else {
            out.push_str(&chars::HORIZONTAL.to_string().repeat(w));
        }
        out.push(chars::TOP_RIGHT);
        out.push('\n');

        // Sections
        for (i, section) in self.sections.iter().enumerate() {
            // Section separator (except for first)
            if i > 0 {
                out.push(chars::T_RIGHT);
                out.push_str(&chars::HORIZONTAL.to_string().repeat(w));
                out.push(chars::T_LEFT);
                out.push('\n');
            }

            // Section name
            if let Some(ref name) = section.name {
                let line = format!("  {}", name);
                out.push(chars::VERTICAL);
                out.push_str(&pad_right(&line, w));
                out.push(chars::VERTICAL);
                out.push('\n');
            }

            // Section content (can be multi-line)
            for line in section.content.lines() {
                let padded = if line.is_empty() {
                    " ".repeat(w)
                } else {
                    pad_right(&format!("  {}", line), w)
                };
                out.push(chars::VERTICAL);
                out.push_str(&padded);
                out.push(chars::VERTICAL);
                out.push('\n');
            }
        }

        // Bottom border
        out.push(chars::BOTTOM_LEFT);
        out.push_str(&chars::HORIZONTAL.to_string().repeat(w));
        out.push(chars::BOTTOM_RIGHT);

        out
    }
}

impl Default for BoxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Progress Bar
// ─────────────────────────────────────────────────────────────────────────────

/// Render a progress bar.
///
/// # Arguments
/// * `value` - Current value
/// * `max` - Maximum value
/// * `width` - Bar width in characters
///
/// # Example
/// ```
/// use novanet::blueprint::ascii::progress_bar;
/// let bar = progress_bar(67, 100, 20);
/// // Returns: "█████████████░░░░░░░ 67%"
/// ```
pub fn progress_bar(value: usize, max: usize, width: usize) -> String {
    let pct = if max == 0 {
        0.0
    } else {
        value as f64 / max as f64
    };
    let filled = (pct * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "{}{} {:>3}%",
        chars::PROGRESS_FILLED.to_string().repeat(filled),
        chars::PROGRESS_EMPTY.to_string().repeat(empty),
        (pct * 100.0).round() as usize
    )
}

/// Render a compact progress bar (no percentage).
pub fn progress_bar_compact(value: usize, max: usize, width: usize) -> String {
    let pct = if max == 0 {
        0.0
    } else {
        value as f64 / max as f64
    };
    let filled = (pct * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    format!(
        "{}{}",
        chars::PROGRESS_FILLED.to_string().repeat(filled),
        chars::PROGRESS_EMPTY.to_string().repeat(empty)
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Tree Rendering
// ─────────────────────────────────────────────────────────────────────────────

/// Tree node for hierarchical rendering.
pub struct TreeNode {
    pub label: String,
    pub annotation: Option<String>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            annotation: None,
            children: Vec::new(),
        }
    }

    pub fn annotated(label: impl Into<String>, annotation: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            annotation: Some(annotation.into()),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, node: TreeNode) -> Self {
        self.children.push(node);
        self
    }

    pub fn children(mut self, nodes: Vec<TreeNode>) -> Self {
        self.children.extend(nodes);
        self
    }

    /// Render tree to string.
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.render_recursive(&mut out, "", true);
        out
    }

    fn render_recursive(&self, out: &mut String, prefix: &str, is_last: bool) {
        // Current node
        let branch = if prefix.is_empty() {
            ""
        } else if is_last {
            chars::TREE_LAST
        } else {
            chars::TREE_BRANCH
        };

        let _ = write!(out, "{}{}{}", prefix, branch, self.label);
        if let Some(ref ann) = self.annotation {
            let _ = write!(out, " {}", ann);
        }
        out.push('\n');

        // Children
        let child_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{}{}", prefix, chars::TREE_SPACE)
        } else {
            format!("{}{}", prefix, chars::TREE_PIPE)
        };

        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == self.children.len() - 1;
            child.render_recursive(out, &child_prefix, is_last_child);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Truncate string to max length, adding ellipsis if needed.
///
/// # Arguments
/// * `s` - String to truncate
/// * `max_len` - Maximum length (including ellipsis)
///
/// # Example
/// ```
/// use novanet::blueprint::ascii::truncate;
/// assert_eq!(truncate("Hello, World!", 8), "Hello...");
/// assert_eq!(truncate("Hi", 10), "Hi");
/// ```
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Pad string to width with spaces on the right.
pub fn pad_right(s: &str, width: usize) -> String {
    let visible_len = strip_ansi_len(s);
    if visible_len >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - visible_len))
    }
}

/// Pad string to width with spaces on the left.
pub fn pad_left(s: &str, width: usize) -> String {
    let visible_len = strip_ansi_len(s);
    if visible_len >= width {
        s.to_string()
    } else {
        format!("{}{}", " ".repeat(width - visible_len), s)
    }
}

/// Center string within width.
pub fn center(s: &str, width: usize) -> String {
    let visible_len = strip_ansi_len(s);
    if visible_len >= width {
        s.to_string()
    } else {
        let padding = width - visible_len;
        let left = padding / 2;
        let right = padding - left;
        format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
    }
}

/// Get visible length (ignoring ANSI escape codes).
fn strip_ansi_len(s: &str) -> usize {
    // Simple approximation: count non-escape characters
    // For proper handling, would use strip-ansi-escapes crate
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

/// Create a horizontal rule.
pub fn horizontal_rule(width: usize) -> String {
    chars::HORIZONTAL.to_string().repeat(width)
}

/// Trait symbols for display (v11.2: 5 traits - derived split, job removed).
pub fn trait_symbol(trait_key: &str) -> &'static str {
    match trait_key {
        "invariant" => "■",
        "localized" => "□",
        "knowledge" => "◊",
        "generated" => "★",
        "aggregated" => "▪",
        _ => "?",
    }
}

/// Arc family arrow for display.
pub fn arc_family_arrow(family_key: &str) -> &'static str {
    match family_key {
        "ownership" => "→",
        "localization" => "⇢",
        "semantic" => "⇄",
        "generation" => "⇉",
        "mining" => "⇶",
        _ => "→",
    }
}

/// Realm icon for display.
pub fn realm_icon(realm_key: &str) -> &'static str {
    match realm_key {
        "shared" => "◉",
        "org" => "◎",
        _ => "○",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(50, 100, 10);
        assert!(bar.contains("█████"));
        assert!(bar.contains("░░░░░"));
        assert!(bar.contains("50%"));
    }

    #[test]
    fn test_progress_bar_empty() {
        let bar = progress_bar(0, 100, 10);
        assert!(bar.contains("░░░░░░░░░░"));
        assert!(bar.contains("0%"));
    }

    #[test]
    fn test_progress_bar_full() {
        let bar = progress_bar(100, 100, 10);
        assert!(bar.contains("██████████"));
        assert!(bar.contains("100%"));
    }

    #[test]
    fn test_box_builder() {
        let boxed = BoxBuilder::new()
            .title("TEST")
            .section("Section 1", "Content here")
            .render();

        assert!(boxed.contains("TEST"));
        assert!(boxed.contains("Section 1"));
        assert!(boxed.contains("Content here"));
        assert!(boxed.contains("╭"));
        assert!(boxed.contains("╰"));
    }

    #[test]
    fn test_tree_node() {
        let tree = TreeNode::new("Root")
            .child(TreeNode::new("Child 1"))
            .child(TreeNode::new("Child 2").child(TreeNode::new("Grandchild")));

        let rendered = tree.render();
        assert!(rendered.contains("Root"));
        assert!(rendered.contains("Child 1"));
        assert!(rendered.contains("Child 2"));
        assert!(rendered.contains("Grandchild"));
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("test", 10), "test      ");
        assert_eq!(pad_right("test", 4), "test");
        assert_eq!(pad_right("test", 2), "test");
    }

    #[test]
    fn test_center() {
        assert_eq!(center("test", 10), "   test   ");
        assert_eq!(center("test", 4), "test");
    }

    #[test]
    fn test_trait_symbol() {
        assert_eq!(trait_symbol("invariant"), "■");
        assert_eq!(trait_symbol("localized"), "□");
        assert_eq!(trait_symbol("knowledge"), "◊");
    }
}
