# Taxonomy Dashboard - 3-Tier Visual Hierarchy Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Display Scope, Subcategory, and Node Types as 3 visually distinct categories with decreasing size/prominence - both in the sidebar panel and the magnetic graph layout.

**Architecture:** Dashboard cards approach with hero/medium/pill hierarchy. All metadata from Neo4j via `useMagneticData` hook. Dual counts (types + loaded instances) at every level.

**Tech Stack:** React, @xyflow/react, Zustand, Tailwind CSS, Neo4j

---

## Sidebar (ResultsOverview)

### Level 1: Scope Hero Cards (3 cards)

- Full-width, tall (`h-20`), glass background
- Left color accent bar (scope color from Neo4j: `#2aa198`, `#6c71c4`, `#cb4b16`)
- Large emoji (`text-3xl`), bold name, dual count: "15 types · 127 loaded"
- Progress bar: instances loaded ratio
- Glow shadow matching scope color
- Replaces current `ScopeBreakdownRow`

### Level 2: Subcategory Medium Cards (9 cards, grid 2-col)

- Grid 2-col layout, medium height (`h-14`)
- Subtle glass, thinner border
- Medium emoji (`text-xl`), semibold name, dual count
- Mini progress bar (thin)
- Parent scope color as `border-top` accent (no glow)
- Replaces current `SubcategoryBreakdownRow`

### Level 3: Node Type Pills (35 pills, flex-wrap)

- Inline `flex-wrap`, tiny pills (`h-7`)
- Small emoji (`text-sm`) + name + count badge
- Minimal glass, no border accent
- Category color as text color only
- Replaces current `ExpandedNodeBadge`

---

## Graph 2D (Magnetic Mode)

### Level 1: Scope Attractor (3 nodes)

- **150px** circle (up from 120px)
- Thick border (4px), scope color from Neo4j
- Strong glow animation (subtle pulse)
- Dual count: "15 types · 127 loaded"

### Level 2: Subcategory Attractor (9 nodes)

- **90px** circle (up from 80px)
- Thin border (2px), parent scope color at 80% opacity
- Soft glow (dimmer than scope)
- Dual count: "1 type · 3 loaded"

### Level 3: Data Nodes (instances)

- Standard node size (unchanged, ~60px)
- Existing TurboNode / StructuralNode / etc.
- Orbits nearest subcategory attractor

**Size ratio:** 150px > 90px > ~60px
**Glow ratio:** strong > soft > none (unless selected)

---

## Data Flow

```
Neo4j (organizing-principles API)
  │
  ├─→ useMagneticData hook
  │     scopes[]         → 3 ScopeData (key, displayName, emoji, color)
  │     subcategories[]  → 9 SubcategoryData (+ scopeKey)
  │     nodeTypeMapping  → 35 entries (nodeType → subcatKey)
  │
  ├─→ Graph2D (magnetic mode)
  │     ScopeAttractorNode     (150px, dual count, glow)
  │     SubcategoryAttractorNode (90px, dual count, soft glow)
  │     Data nodes (unchanged)
  │
  └─→ ResultsOverview (sidebar)
        ScopeHeroCard      (full-width, tall, glow)
        SubcategoryCard    (grid 2-col, medium)
        NodeTypePill       (inline flex-wrap, tiny)
```

## Dual Count

- **types** = nodeTypeMapping entries for that scope/subcat (static, from schema)
- **loaded** = graphNodes filtered by scope/subcat (dynamic, changes with active filters)

---

## Components to Modify

| File | Change |
|------|--------|
| `ResultsOverview.tsx` | Replace 3 sub-components with ScopeHeroCard, SubcategoryCard, NodeTypePill |
| `ScopeAttractorNode.tsx` | Size 120→150px, add dual count, stronger glow |
| `SubcategoryAttractorNode.tsx` | Size 80→90px, add dual count |
| `Graph2D.tsx` | Pass instance counts (typeCount + loadedCount) to attractor nodes |
