# FilterChip Unified Design

**Date**: 2026-02-03
**Status**: Approved
**Author**: Claude + Thibaut

## Overview

Refactor the badge/chip UI in Studio to create a unified, interactive `<FilterChip />` component
that works across stats bar and sidebar, with YAML as the single source of truth for colors.

## Requirements

1. **No cropping** - Full text visible, wrap to multiple lines if needed
2. **No emojis** - Clean, unified style with colored checkboxes only
3. **Interactive** - Click to filter, Shift+Click to highlight/focus
4. **Single source of truth** - Colors from `organizing-principles.yaml`
5. **TUI + Studio alignment** - Both consume the same YAML colors

## Scope

- **Zone A**: Stats bar (top badges for Realms, Layers, Node Types, Relations)
- **Zone B**: Sidebar (FacetFilterPanel checkboxes)
- **NOT in scope**: MetaBadgeNode in graph canvas (keep current premium style)

## Component Design

### FilterChip Props

```typescript
interface FilterChipProps {
  label: string;        // Full text, never truncated
  count?: number;       // Optional count badge (right-aligned, tabular-nums)
  color: string;        // Hex color from YAML (border + checkbox fill)
  checked: boolean;     // true = visible in graph
  focused: boolean;     // true = highlight mode active
  onToggle: () => void; // Click handler
  onFocus: () => void;  // Shift+Click handler
}
```

### Visual States

| State | Opacity | Border | Checkbox | Background |
|-------|---------|--------|----------|------------|
| DEFAULT (checked) | 100% | solid 1px {color} | filled {color} + white checkmark | transparent |
| FILTERED (unchecked) | 30% | solid 1px {color}@30% | empty border only | transparent |
| FOCUSED | 100% | solid 2px {color} | filled + dot indicator | {color}@10% + glow |
| HOVER | - | brighten if dimmed | - | white/5 |

### Interaction Model

- **Click** = Toggle filter (show/hide nodes in graph)
- **Shift+Click** = Highlight focus (dim all others to 30%)

## Color Architecture

### Source of Truth

```
packages/core/models/organizing-principles.yaml
├── realms[].color          # 3 colors
├── realms[].layers[].color # 9 colors
├── traits[].color          # 5 colors
└── edge_families[].color   # 5 colors
```

### Flow

```
organizing-principles.yaml
         │
         ├──────────────────────────────────┐
         │                                  │
         ▼                                  ▼
    Rust Parser                      Rust Generator
    (parsers/organizing.rs)          (generators/colors.rs)
         │                                  │
         ▼                                  ▼
    TUI (Color::Rgb)               Studio (generated.ts)
```

### YAML Colors (Official)

#### Realms (3)

| Key | Color | Name |
|-----|-------|------|
| global | #2aa198 | teal |
| project | #6c71c4 | violet |
| shared | #cb4b16 | orange |

#### Layers (9)

| Key | Color | Name |
|-----|-------|------|
| config | #64748b | slate |
| knowledge | #8b5cf6 | purple |
| foundation | #3b82f6 | blue |
| structure | #06b6d4 | cyan |
| semantic | #f97316 | orange |
| instruction | #eab308 | yellow |
| output | #22c55e | green |
| seo | #ec4899 | pink |
| geo | #ef4444 | red |

#### Traits (5)

| Key | Color | Name |
|-----|-------|------|
| invariant | #3b82f6 | blue |
| localized | #22c55e | green |
| knowledge | #8b5cf6 | purple |
| derived | #9ca3af | gray |
| job | #6b7280 | gray-dark |

#### Edge Families (5)

| Key | Color | Name |
|-----|-------|------|
| ownership | #3b82f6 | blue |
| localization | #22c55e | green |
| semantic | #f97316 | orange |
| generation | #8b5cf6 | purple |
| mining | #ec4899 | pink |

## Implementation Plan

### Phase 1: Color Generator (Rust)

- Create `tools/novanet/src/generators/colors.rs`
- MiniJinja template → `apps/studio/src/design/colors/generated.ts`
- Export: `REALM_COLORS`, `LAYER_COLORS`, `TRAIT_COLORS`, `EDGE_FAMILY_COLORS`
- Integrate into `schema generate` command

### Phase 2: TUI Dynamic Colors

- Add helper: `hex_to_rgb("#2aa198") → Color::Rgb(42, 161, 152)`
- Replace hardcoded `realm_color()` with parsed `realm.color`
- Apply to layers if displayed

### Phase 3: Studio Color Migration

- Delete: `layerColors.ts`
- Remove: `realmAccents` from `tokens.ts`
- Create: `design/colors/index.ts` (re-export generated.ts)
- Update all imports (~23 files)

### Phase 4: FilterChip Component

- Create: `components/ui/FilterChip.tsx`
- Implement all 4 visual states
- Add click + shift+click handlers
- Ensure accessibility (ARIA, keyboard nav)

### Phase 5: Integration

- Refactor stats bar to use `<FilterChip />`
- Refactor `FacetFilterPanel` to use `<FilterChip />`
- Bidirectional sync via `useFilterStore`
- Add tests

### Phase 6: Cleanup

- Remove dead code (old badge components)
- Validate: `cargo test`, `pnpm test`, `pnpm type-check`
- Update CHANGELOG

## Files

### Created

- `tools/novanet/src/generators/colors.rs`
- `apps/studio/src/design/colors/generated.ts` (auto-generated)
- `apps/studio/src/design/colors/index.ts`
- `apps/studio/src/components/ui/FilterChip.tsx`

### Modified

- `tools/novanet/src/generators/mod.rs`
- `tools/novanet/src/tui/ui.rs`
- `apps/studio/src/design/tokens.ts`
- ~23 files with color imports

### Deleted

- `apps/studio/src/design/layerColors.ts`

## Notes

- This replaces the Solarized palette (was colorblind-safe) with Tailwind colors from YAML
- The TUI uses `Color::Rgb()` for true color support in modern terminals
- Future: Could generate CSS custom properties for theming
