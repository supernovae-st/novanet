# TUI Panel Simplification v0.13.1

## Overview

Simplify TUI panel composition and align PROPERTIES display between Class and Instance views.

## Changes

### 1. Remove DIAGRAM and ARCHITECTURE boxes

- Delete `src/tui/ui/architecture.rs`
- Remove DIAGRAM rendering from `src/tui/ui/mod.rs`
- Remove `InfoBox::Diagram` and `InfoBox::Architecture` variants
- Leave empty space in bottom-right (for now)

### 2. Align PROPERTIES display

**Current state:**
- Class: shows `*[str] property_name` with type badges
- Instance: shows `property_name    "value"` table-style

**Target state:**
- Both use YAML-style coloring (cyan keys, colored values)
- Same order as YAML source file
- Aligned colons (`:` aligned vertically)

**Class view:**
```
*description:   string
*llm_context:   string
 status:        string
 tagline:       string
```

**Instance view:**
```
description:   "English localization..."
llm_context:   "USE: native English..."
status:        "active"
tagline:       "Smart QR Codes"
```

### 3. Final Layout

```
┌─────────────┬─────────────────┬──────────────────┐
│             │ HEADER          │ SOURCE           │
│ TREE        │ PROPERTIES      │ (YAML/Data tabs) │
│             │ ARCS            │ (empty)          │
└─────────────┴─────────────────┴──────────────────┘
```

## Files to Modify

1. `src/tui/ui/mod.rs` - Remove DIAGRAM/ARCHITECTURE rendering
2. `src/tui/ui/architecture.rs` - Delete file
3. `src/tui/app.rs` - Remove `InfoBox::Diagram`, `InfoBox::Architecture`
4. `src/tui/ui/properties.rs` - New YAML-style rendering
5. `src/tui/clipboard.rs` - Remove Diagram/Architecture handlers

## Implementation Order

1. Remove DIAGRAM + ARCHITECTURE boxes
2. Refactor PROPERTIES to use YAML-style colors
3. Ensure property order matches YAML source
4. Align colons vertically
