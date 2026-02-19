# PROPERTIES Panel Improvements

## Overview

Enhance the TUI PROPERTIES box with better UX: colored headers, navigation, copy, and value expansion.

## Features

### Feature 1: Colored Section Headers
- **STANDARD** section header: teal color (config/boring properties)
- **SPECIFIC** section header: gold/orange color (unique/interesting properties)
- Currently both use `STYLE_MUTED` (gray)

**Files**: `src/tui/ui/info.rs`
**Lines**: ~1230 (STANDARD header), ~1296 (SPECIFIC header)

### Feature 2: Property Navigation [j/k]
- When PROPERTIES panel is focused, allow [j/k] to select individual properties
- Need: `selected_property_index: Option<usize>` in AppState
- Visual: highlight selected property row
- Keybindings: j=down, k=up within properties

**Files**:
- `src/tui/app.rs` - add `selected_property_index` state
- `src/tui/ui/info.rs` - render selected property with highlight
- `src/tui/mod.rs` - handle j/k when in PROPERTIES focus

### Feature 3: Copy Property Value [c]
- When property is selected, [c] copies its value to clipboard
- Uses `arboard` crate (already in dependencies for copy features)
- Show status message "Copied: key = value"

**Files**:
- `src/tui/mod.rs` - handle 'c' key
- `src/tui/app.rs` - add copy_property_value() method

### Feature 4: Expand Truncated Value [Enter]
- When property is selected, [Enter] shows full value in overlay
- Modal/overlay with full value (scrollable if very long)
- [Esc] to close overlay

**Files**:
- `src/tui/app.rs` - add `expanded_property: Option<(String, String)>` state
- `src/tui/ui/info.rs` or new `src/tui/ui/overlays.rs` - render overlay
- `src/tui/mod.rs` - handle Enter key

## Implementation Order (TDD)

1. **Feature 1** (simplest, no state changes)
2. **Feature 2** (state + rendering)
3. **Feature 3** (builds on Feature 2)
4. **Feature 4** (builds on Feature 2)

## Test Plan

### Feature 1 Tests
- `test_standard_header_uses_teal_color`
- `test_specific_header_uses_gold_color`

### Feature 2 Tests
- `test_selected_property_index_default_none`
- `test_property_nav_j_increments`
- `test_property_nav_k_decrements`
- `test_property_nav_wraps_or_clamps`
- `test_selected_property_rendered_with_highlight`

### Feature 3 Tests
- `test_copy_property_requires_selection`
- `test_copy_property_copies_to_clipboard`
- `test_copy_property_shows_status_message`

### Feature 4 Tests
- `test_expand_property_requires_selection`
- `test_expand_property_shows_overlay`
- `test_expand_overlay_closes_on_esc`
