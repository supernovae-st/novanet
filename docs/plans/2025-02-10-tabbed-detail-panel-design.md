# Tabbed Detail Panel Design

**Version:** v1.0
**Date:** 2025-02-10
**Status:** Approved (Brainstorm Complete)

## Overview

Refonte du panel de détails de nodes dans NovaNet Studio avec:
- Interface à tabs pour densité d'information
- Parité structure avec TUI, design web-native
- Features avancées (Mermaid, Neo4j live, stats)

## Decisions

| Question | Choice |
|----------|--------|
| Objectif | D) Tout: Densité + Parité TUI + Features avancées |
| Structure tabs | B) 4 tabs: [Overview] [Data] [Graph] [Code] |
| Mermaid views | D) 3 switchables: Ego, Arc-type, Layer-flow |
| Code formats | B) 4 formats: JSON, YAML, Cypher, TypeScript |
| Parité TUI | AB) Structure identique + design web-adapté |

## Architecture

```
TabbedDetailPanel (wrapper principal)
├── TabBar (SegmentedTabs)
│   └── [Overview] [Data] [Graph] [Code]
│
├── OverviewTab
│   ├── HeaderCard (type badge, title, key + copy)
│   ├── ClassificationGrid (realm, layer, trait)
│   └── DescriptionBlock (description + llmContext)
│
├── DataTab
│   ├── StatsBar (incoming/outgoing/properties counts)
│   ├── PropertiesTable (key-value avec type badges)
│   └── PropertyCoverage (progress bar style TUI)
│
├── GraphTab
│   ├── ViewSwitcher [Ego] [Arcs] [Flow] [Context]
│   ├── MermaidDiagram (react-x-mermaid)
│   ├── ActionBar [Refresh] [Load More] [Expand] [Copy Query] [Run]
│   ├── QueryPanel (Cypher editor + status)
│   └── RelationsList (navigation cards)
│
└── CodeTab
    ├── FormatSwitcher [JSON] [YAML] [Cypher] [TS]
    └── CodeViewer (Prism syntax highlighting)
```

## Visual Design Language

Inspirations: Context7 + Perplexity + Magic UI

### Context7 Style
- Cards avec subtle border glow on hover
- Compact metadata badges (version, source, tokens)
- Monospace code avec copy-on-click

### Perplexity Style
- Answer cards avec gradient headers
- Source chips cliquables (→ relation cards)
- Streaming text animation (Cypher live)
- Floating action buttons

### Magic UI Style
- Glassmorphism panels (backdrop-blur)
- Gradient borders animés (pulse on selection)
- Bento grid layout pour Stats
- Shimmer loading states

### Design Tokens

```css
--panel-bg: hsl(240, 10%, 4%);      /* near-black */
--card-bg: hsl(240, 8%, 8%);        /* elevated surface */
--border: rgba(255,255,255, 0.06);  /* subtle */
--border-hover: rgba(primary, 0.4); /* glow effect */
--tab-active: gradient(primary → secondary);
--code-bg: hsl(240, 12%, 6%);       /* darker for contrast */
```

## Graph Tab — 3 Mermaid Views

### 1. Ego View (default)
```mermaid
flowchart LR
  subgraph incoming
    Project & Entity
  end
  incoming --> SELECTED
  SELECTED --> outgoing
```
Node au centre, voisins directs groupés par direction.

### 2. Arc-Type View
```mermaid
flowchart TB
  subgraph OWNERSHIP
    Project -->|HAS_PAGE| Page
    Page -->|HAS_BLOCK| Block
  end
  subgraph SEMANTIC
    Entity -->|USES| Page
  end
```
Groupé par ArcFamily (ownership, semantic, generation).

### 3. Layer-Flow View
```mermaid
flowchart LR
  foundation --> structure --> semantic --> output
  structure:::selected
```
Position du node dans le pipeline de layers.

### 4. Context View (NEW - Type-Specific)
Vue contextuelle selon le type de node:

| Node Type | Context View |
|-----------|--------------|
| Page | Construction: Page → Blocks → ContentSlots |
| Entity | Connections: Entity → EntityContent + SEO Keywords |
| Block | Hierarchy: Page ← Block → ContentSlot + Entities |
| Project | Overview: Project → Pages → Entities |
| SEOKeyword | Network: Keywords → Entities → Pages |

## Interactive Features

### Action Bar
```
[🔄 Refresh] [📥 Load More] [🔍 Expand] [📋 Copy Query] [▶️ Run]
```

- **Refresh**: Re-fetch depuis Neo4j
- **Load More**: +1 niveau de profondeur
- **Expand**: Fullscreen modal
- **Copy Query**: Cypher → clipboard
- **Run**: Execute live

### Neo4j Live Sync

```
┌─────────────────────────────────────────────────────────┐
│ ▼ Cypher Query                           [Edit] [Run]  │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ MATCH (n:Page {key: "homepage"})-[r]-(m)            │ │
│ │ RETURN n, r, m LIMIT 25                             │ │
│ └─────────────────────────────────────────────────────┘ │
│ Status: ● Connected │ Last sync: 2s ago │ 4 nodes     │
└─────────────────────────────────────────────────────────┘
```

### View Modes
```
Mode: [Schema ◉] [Data ○] [Overlay ○]     Depth: [1] [2] [3]
```

- **Schema**: KIND relationships (meta-graph)
- **Data**: Vraies instances Neo4j
- **Overlay**: Schema + Data superposés
- **Depth**: 1/2/3 niveaux de neighbors

### Results Panel
```
┌─────────────────────────────────────────────────────────┐
│ Results (4 nodes, 3 relationships)     [Table] [Graph] │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ n.key      │ type(r)     │ m.key        │ m.type   │ │
│ │ homepage   │ HAS_BLOCK   │ hero-section │ Block    │ │
│ │ homepage   │ HAS_CONTENT │ homepage@fr  │ PageGen  │ │
│ └─────────────────────────────────────────────────────┘ │
│ [← Prev] Page 1/1 [Next →]              [Export CSV]   │
└─────────────────────────────────────────────────────────┘
```

## Files to Create

| File | Purpose |
|------|---------|
| `components/sidebar/TabbedDetailPanel.tsx` | Main wrapper |
| `components/sidebar/tabs/OverviewTab.tsx` | Summary view |
| `components/sidebar/tabs/DataTab.tsx` | Properties + Stats |
| `components/sidebar/tabs/GraphTab.tsx` | Mermaid + Relations |
| `components/sidebar/tabs/CodeTab.tsx` | JSON/YAML/Cypher/TS |
| `components/sidebar/tabs/index.ts` | Barrel export |
| `components/graph/MermaidView.tsx` | Mermaid renderer |
| `components/graph/QueryPanel.tsx` | Cypher editor |
| `hooks/useNeo4jQuery.ts` | Neo4j live queries |

## Files to Modify

| File | Changes |
|------|---------|
| `stores/uiStore.ts` | Add `detailPanelTab` state |
| `app/page.tsx` | Replace NodeDetailsPanel with TabbedDetailPanel |

## Dependencies to Add

```bash
pnpm add mermaid react-x-mermaid
```

## Next Steps

1. [ ] Explorer agents pour comprendre les patterns de connexion par type
2. [ ] Implémenter TabbedDetailPanel wrapper
3. [ ] Créer les 4 tabs
4. [ ] Intégrer Mermaid avec dark theme
5. [ ] Ajouter Neo4j live sync
6. [ ] Implémenter Context Views par type de node
