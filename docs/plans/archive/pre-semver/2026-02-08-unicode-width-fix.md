# Unicode Width Fix Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix terminal display issues with CJK, Arabic, Bengali, Thai, and emoji characters by using proper Unicode width calculation instead of char count.

**Architecture:** Create a new `unicode.rs` module with width-aware truncation functions, then replace all `truncate_str`/`truncate_start` calls throughout the TUI. Uses grapheme clusters for proper handling of combining characters across 200+ locales.

**Tech Stack:** `unicode-width` (0.2) + `unicode-segmentation` (1.11)

---

## Task 1: Add Dependencies

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add unicode crates to Cargo.toml**

Add after line 48 (after `rustc-hash`):

```toml
unicode-width = "0.2"           # Display width for CJK, emoji (UAX#11)
unicode-segmentation = "1.11"   # Grapheme cluster iteration
```

**Step 2: Verify dependencies resolve**

Run: `cargo check`
Expected: Compiles with new dependencies downloaded

**Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "deps: add unicode-width + unicode-segmentation for proper display width"
```

---

## Task 2: Create Unicode Module with Core Functions

**Files:**
- Create: `src/tui/unicode.rs`
- Modify: `src/tui/mod.rs` (add module)

**Step 1: Write failing tests for display_width**

Create `src/tui/unicode.rs`:

```rust
//! Unicode display width utilities for terminal rendering.
//!
//! Handles CJK (2 columns), emoji, Arabic, Bengali, Thai, and other
//! scripts that require grapheme-aware width calculation.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Calculate display width in terminal columns.
/// Uses grapheme clusters to handle combining characters correctly.
/// CJK characters = 2 columns, most others = 1 column.
pub fn display_width(s: &str) -> usize {
    s.graphemes(true).map(|g| g.width()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width(" "), 1);
    }

    #[test]
    fn test_display_width_cjk() {
        // Chinese characters are 2 columns each
        assert_eq!(display_width("中文"), 4);
        assert_eq!(display_width("日本語"), 6);
        assert_eq!(display_width("한글"), 4);
    }

    #[test]
    fn test_display_width_emoji() {
        // Most emoji are 2 columns
        assert_eq!(display_width("🎉"), 2);
        assert_eq!(display_width("👍"), 2);
    }

    #[test]
    fn test_display_width_mixed() {
        // "Hello中文🎉" = 5 + 4 + 2 = 11
        assert_eq!(display_width("Hello中文🎉"), 11);
    }

    #[test]
    fn test_display_width_arabic() {
        // Arabic text (RTL doesn't affect width calculation)
        let arabic = "مرحبا";
        assert!(display_width(arabic) > 0);
    }

    #[test]
    fn test_display_width_bengali() {
        // Bengali with combining characters
        let bengali = "বাংলা";
        assert!(display_width(bengali) > 0);
    }

    #[test]
    fn test_display_width_thai() {
        // Thai with tone marks
        let thai = "ไทย";
        assert!(display_width(thai) > 0);
    }
}
```

**Step 2: Add module to tui/mod.rs**

Add after other module declarations:

```rust
pub mod unicode;
```

**Step 3: Run tests to verify they pass**

Run: `cargo test tui::unicode --lib`
Expected: All 8 tests pass

**Step 4: Commit**

```bash
git add src/tui/unicode.rs src/tui/mod.rs
git commit -m "feat(tui): add unicode module with display_width function"
```

---

## Task 3: Add Truncation Functions

**Files:**
- Modify: `src/tui/unicode.rs`

**Step 1: Write failing tests for truncate_to_width**

Add to `src/tui/unicode.rs` after `display_width`:

```rust
/// Truncate string to fit within max terminal columns.
/// Appends "…" if truncated. Uses grapheme-aware iteration.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut current_width = 0;
    let ellipsis = "…";
    let ellipsis_width = 1;

    // Reserve space for ellipsis if we might need it
    let effective_max = if display_width(s) > max_width {
        max_width.saturating_sub(ellipsis_width)
    } else {
        max_width
    };

    for grapheme in s.graphemes(true) {
        let grapheme_width = grapheme.width();
        if current_width + grapheme_width > effective_max {
            result.push_str(ellipsis);
            return result;
        }
        result.push_str(grapheme);
        current_width += grapheme_width;
    }

    result
}

/// Truncate string from the START to fit within max terminal columns.
/// Prepends "…" if truncated. Used for breadcrumbs where end is relevant.
pub fn truncate_start_to_width(s: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    let total_width = display_width(s);
    if total_width <= max_width {
        return s.to_string();
    }

    let ellipsis = "…";
    let ellipsis_width = 1;
    let target_width = max_width.saturating_sub(ellipsis_width);

    // Collect graphemes with their widths
    let graphemes: Vec<(&str, usize)> = s
        .graphemes(true)
        .map(|g| (g, g.width()))
        .collect();

    // Find where to start from the end
    let mut width_from_end = 0;
    let mut start_index = graphemes.len();

    for (i, (_, w)) in graphemes.iter().enumerate().rev() {
        if width_from_end + w > target_width {
            break;
        }
        width_from_end += w;
        start_index = i;
    }

    let mut result = String::from(ellipsis);
    for (g, _) in &graphemes[start_index..] {
        result.push_str(g);
    }

    result
}
```

**Step 2: Add tests for truncation**

Add to the tests module:

```rust
    // truncate_to_width tests
    #[test]
    fn test_truncate_to_width_no_truncation() {
        assert_eq!(truncate_to_width("hello", 10), "hello");
        assert_eq!(truncate_to_width("hi", 2), "hi");
    }

    #[test]
    fn test_truncate_to_width_ascii() {
        assert_eq!(truncate_to_width("hello world", 8), "hello w…");
        assert_eq!(truncate_to_width("abcdefgh", 5), "abcd…");
    }

    #[test]
    fn test_truncate_to_width_cjk() {
        // "中文字" = 6 cols, truncate to 5 → "中文…" (4+1=5)
        assert_eq!(truncate_to_width("中文字", 5), "中文…");
    }

    #[test]
    fn test_truncate_to_width_mixed() {
        // "Hello中文" = 5+4=9, truncate to 7 → "Hello…" (5+1=6) or "Hello中…"
        let result = truncate_to_width("Hello中文", 7);
        assert!(display_width(&result) <= 7);
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_truncate_to_width_empty() {
        assert_eq!(truncate_to_width("", 5), "");
        assert_eq!(truncate_to_width("hello", 0), "");
    }

    #[test]
    fn test_truncate_to_width_emoji() {
        // "🎉🎊🎁" = 6 cols, truncate to 4 → "🎉…" (2+1=3)
        let result = truncate_to_width("🎉🎊🎁", 4);
        assert!(display_width(&result) <= 4);
    }

    // truncate_start_to_width tests
    #[test]
    fn test_truncate_start_no_truncation() {
        assert_eq!(truncate_start_to_width("hello", 10), "hello");
    }

    #[test]
    fn test_truncate_start_ascii() {
        // Keep end: "hello world" → "…world"
        let result = truncate_start_to_width("hello world", 7);
        assert!(result.starts_with('…'));
        assert!(display_width(&result) <= 7);
    }

    #[test]
    fn test_truncate_start_cjk() {
        // "中文字符" = 8 cols, truncate to 5 → "…字符" (1+4=5)
        let result = truncate_start_to_width("中文字符", 5);
        assert!(result.starts_with('…'));
        assert!(display_width(&result) <= 5);
    }

    #[test]
    fn test_truncate_start_empty() {
        assert_eq!(truncate_start_to_width("", 5), "");
        assert_eq!(truncate_start_to_width("hello", 0), "");
    }
```

**Step 3: Run all unicode tests**

Run: `cargo test tui::unicode --lib`
Expected: All 19 tests pass

**Step 4: Commit**

```bash
git add src/tui/unicode.rs
git commit -m "feat(tui): add truncate_to_width and truncate_start_to_width"
```

---

## Task 4: Replace truncate_str in ui/mod.rs

**Files:**
- Modify: `src/tui/ui/mod.rs`

**Step 1: Add import at top of file**

Add after other use statements (around line 10):

```rust
use super::unicode::{display_width, truncate_to_width, truncate_start_to_width};
```

**Step 2: Update truncate_str function (lines 443-453)**

Replace the old function with a call to the new one:

```rust
/// Safely truncate a UTF-8 string to N terminal columns (not chars).
/// Appends "…" if truncated. Handles CJK, emoji, and combining characters.
fn truncate_str(s: &str, max_width: usize) -> String {
    truncate_to_width(s, max_width)
}
```

**Step 3: Update truncate_start function (lines 455-466)**

Replace with:

```rust
/// Safely truncate a UTF-8 string from the START, keeping last N columns.
/// Prepends "…" if truncated. Used for breadcrumbs where the end is most relevant.
fn truncate_start(s: &str, max_width: usize) -> String {
    truncate_start_to_width(s, max_width)
}
```

**Step 4: Run existing tests to ensure no regression**

Run: `cargo test tui::ui --lib`
Expected: All existing ui tests pass

**Step 5: Commit**

```bash
git add src/tui/ui/mod.rs
git commit -m "refactor(tui): use unicode-aware truncation in ui/mod.rs"
```

---

## Task 5: Update Guide Module wrap_text

**Files:**
- Modify: `src/tui/guide/arcs.rs`

**Step 1: Check current wrap_text implementation**

The `wrap_text` function in `guide/arcs.rs` likely uses char count. Update it to use display_width.

Add import at top:

```rust
use crate::tui::unicode::display_width;
```

**Step 2: Update wrap_text function**

Find the `wrap_text` function and update width calculations to use `display_width()` instead of `.chars().count()` or `.len()`.

**Step 3: Run guide tests**

Run: `cargo test tui::guide --lib`
Expected: All guide tests pass

**Step 4: Commit**

```bash
git add src/tui/guide/arcs.rs
git commit -m "refactor(tui): use unicode-aware width in guide/arcs.rs"
```

---

## Task 6: Final Verification

**Step 1: Run full test suite**

Run: `cargo nextest run`
Expected: All tests pass (581+)

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Test manually with CJK content**

Run: `cargo run -- tui`
Navigate to a node with CJK/emoji in its name or description.
Verify text truncates cleanly without visual overflow.

**Step 4: Final commit**

```bash
git add -A
git commit -m "test(tui): verify unicode width handling across all modules"
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Add dependencies | Cargo.toml |
| 2 | Create unicode.rs with display_width | src/tui/unicode.rs |
| 3 | Add truncation functions | src/tui/unicode.rs |
| 4 | Replace truncate_str/truncate_start | src/tui/ui/mod.rs |
| 5 | Update guide module | src/tui/guide/arcs.rs |
| 6 | Final verification | - |

**Total:** 6 tasks, ~15 steps, estimated 20-30 minutes
