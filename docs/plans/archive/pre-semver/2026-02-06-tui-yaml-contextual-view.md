# TUI YAML Contextual View

**Date**: 2026-02-06
**Status**: Approved
**Author**: Thibaut + Claude

## Problem

The TUI right panel shows raw YAML without distinguishing between:
- **Kind metadata**: name, realm, layer, trait, description (defines the Kind:Meta node)
- **Instance structure**: standard_properties (defines what instances must have)

Users don't understand that one YAML file defines both aspects.

## Solution

Split the YAML view contextually based on navigation mode:
- `[1]Meta` mode → Show Kind metadata section
- `[2]Data` mode → Show Instance structure section

### Visual Design

```
┌─ YAML ─────────────────────────────────────────────────────────────┐
│ [Kind ●] [Instance ○]              1:switch  locale.yaml           │
├────────────────────────────────────────────────────────────────────┤
│ node:                                                              │
│   name: Locale                                                     │
│   realm: global                                                    │
│   layer: config                                                    │
│   trait: invariant                                                 │
│   description: "First-class locale..."                             │
│                                                                    │
│ ┄┄┄ standard_properties (42 lines) [Enter: peek] ┄┄┄              │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

When switching to `[2]Data`:

```
┌─ YAML ─────────────────────────────────────────────────────────────┐
│ [Kind ○] [Instance ●]              1:switch  locale.yaml           │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│ ┄┄┄ node metadata (8 lines) [Enter: peek] ┄┄┄                     │
│                                                                    │
│   standard_properties:                                             │
│     key:                                                           │
│       type: string                                                 │
│       required: true                                               │
│       description: "BCP 47 locale code"                            │
│     country_code:                                                  │
│       type: string                                                 │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

### Peek Mode

When user presses Enter on the ellipsis, the hidden section expands inline in DIM (gray) color:

```
┌────────────────────────────────────────────────────────────────┐
│ [Kind ●] [Instance ○]                                          │
├────────────────────────────────────────────────────────────────┤
│ node:                                                          │
│   name: Locale                                                 │
│   realm: global                                                │
│ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ │
│   standard_properties:                  ← DIM (gray)           │
│     key:                                ← DIM                  │
│       type: string                      ← DIM                  │
│ ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ [Enter: collapse] ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄ │
└────────────────────────────────────────────────────────────────┘
```

## Behaviors

### Auto-Switch
- Press `1` → `[1]Meta` → YAML shows Kind section
- Press `2` → `[2]Data` → YAML shows Instance section
- Transition is instant, scroll resets to 0

### Peek (temporary expand)
- Cursor on ellipsis + `Enter` → expand inline
- Hidden section displayed in DIM (gray) to differentiate
- `Enter` again or `Esc` → collapse
- Changing mode → auto-collapse

## Implementation

### Files to Modify

```
src/tui/
├── app.rs      → add `yaml_peek: bool` state
├── ui.rs       → conditional render of YAML panel
└── yaml.rs     → (new) parser to split Kind/Instance sections
```

### Split Logic

1. Parse YAML with `serde_yaml`
2. Identify sections:
   - **Kind**: name, realm, layer, trait, description, icon, llm_context
   - **Instance**: standard_properties, properties
3. Calculate line ranges for each section
4. Display based on mode + peek state

### Data Structure

```rust
struct YamlSections {
    kind_lines: Range<usize>,        // e.g., 0..12
    instance_lines: Range<usize>,    // e.g., 12..54
    raw_content: String,
}
```

### Keybindings

| Key | Action |
|-----|--------|
| `1` / `2` | Change mode (existing) + reset peek |
| `Enter` on ellipsis | Toggle peek |
| `Esc` | Close peek |

## Effort

~200-300 lines of Rust
