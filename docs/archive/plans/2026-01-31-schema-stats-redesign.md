# Schema Stats Pill Redesign

**Date:** 2026-01-31
**Status:** Approved

## Problem

In schema mode, the stats pill shows "35 nodes | 89 relations" which is confusing:
- 35 = node types (correct)
- 89 = schema edges (source→target pairs)
- But sidebar shows "50 RELATION TYPES" (distinct type names)

Users expect consistency between stats pill and sidebar.

## Solution

Context-aware stats pill with dedicated schema mode display and progress bar breakdowns.

## Design

### Stats Pill Display

**Data Mode** (unchanged):
```
⬡ 523 nodes  │  ⟷ 1,247 relations  │  🔄  │  [badges...]
```

**Schema Mode** (new):
```
🔷 Schema  │  ⬡ 35 types  │  ⟷ 50 relations  │  🔄  │  [badges...]
```

Changes:
- Badge "🔷 Schema" with `bg-blue-500/20` + `text-blue-400`
- "nodes" → "types"
- Relation count = distinct relation types (50), not edges (89)

### Hover Breakdowns with Progress Bars

All breakdowns use progress bars relative to max (most frequent = 100%).

**Schema Mode — Hover "35 types":**
```
┌────────────────────────────────────────┐
│  NODE TYPES BY SCOPE                   │
│                                        │
│  🌍 Global    ████████████████░░  15   │  ← max
│  📦 Project   ███████████████░░░  14   │
│  🎯 Shared    ████████░░░░░░░░░░   6   │
└────────────────────────────────────────┘
```

**Schema Mode — Hover "50 relations":**
```
┌────────────────────────────────────────┐
│  RELATION TYPES                        │
│                                        │
│  → FOR_LOCALE           ██████████  6  │  ← max
│  → HAS_LOCALIZED_CONTENT██████████  6  │
│  → HAS_L10N             ███████░░░  4  │
│  ... (+42 more)                        │
└────────────────────────────────────────┘
```

**Data Mode — Hover nodes (upgraded):**
```
┌────────────────────────────────────────┐
│  NODE TYPES                            │
│                                        │
│  🌐 Locale       ████████████████ 200  │
│  💡 ConceptL10n  ██████████░░░░░░  89  │
│  📦 Project      ██░░░░░░░░░░░░░░  12  │
└────────────────────────────────────────┘
```

## Architecture

### New Components

| Component | Purpose |
|-----------|---------|
| `ProgressBar.tsx` | Reusable bar (value, max, color) |

### Modified Files

| File | Changes |
|------|---------|
| `StatsCounter.tsx` | Add `dataMode` prop, schema badge, wording switch |
| `ExpandedBreakdown.tsx` | Progress bars, scope breakdown for schema |
| `useFilteredGraph.ts` | Expose `distinctRelationTypes`, `scopeCounts` |
| `page.tsx` | Pass `dataMode` to StatsCounter |

### Data Flow

```
useFilteredGraph()
  ├── visibleNodeCount: number
  ├── visibleEdgeCount: number
  ├── distinctRelationTypes: number     ← NEW
  ├── nodeTypeCounts: Map<type, count>
  ├── relationTypeCounts: Map<type, count>
  └── scopeCounts: { global, project, shared }  ← NEW
           ↓
StatsCounter (dataMode prop)
  ├── schema? → distinctRelationTypes : edgeCount
  └── schema? → "types" : "nodes", badge
           ↓
ExpandedBreakdown
  ├── Schema + hover nodes → ScopeBreakdown
  ├── Schema + hover rels  → RelationTypeBreakdown
  └── Data + hover nodes   → NodeTypeBreakdown + progress bars
```

## Implementation Plan

1. Create `ProgressBar` component
2. Extend `useFilteredGraph` with schema stats
3. Update `StatsCounter` with schema mode
4. Update `ExpandedBreakdown` with progress bars
5. Wire up in `page.tsx`
6. Test in browser (both modes)
