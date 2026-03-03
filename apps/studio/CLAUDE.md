# NovaNet Studio (v0.15.1)

@.claude/rules/novanet-terminology.md
@.claude/rules/novanet-decisions.md

Knowledge graph visualization for the NovaNet native content generation engine.

---

## Project Context

**What:** Interactive 2D/3D graph visualization for 61 node classes, 200 locales (~19,000 instances projected)
**Stack:** Next.js 16 + React 19 + TypeScript 5.9 + Tailwind CSS
**Graph:** @xyflow/react (2D), @react-three/fiber (3D)
**State:** Zustand 5 with persist/immer
**DB:** Neo4j (bolt://localhost:7687)
**AI:** Claude API for natural language → Cypher
**Version:** v0.15.1 "Version Alignment" (all packages aligned)

---

## Architecture

```
src/
├── app/              # Next.js App Router
│   ├── api/chat/     # Claude AI endpoint
│   └── page.tsx      # Main visualization page
├── components/
│   ├── chat/         # AI chat (⌘J)
│   ├── dx/           # DX tools (copy, code viewer, inspector)
│   ├── graph/        # React Flow components
│   ├── query/        # Query builder components
│   ├── sidebar/      # Node details, filters
│   └── ui/           # Base UI components
├── config/           # Presets, shortcuts, node types
├── hooks/            # Custom React hooks
├── lib/              # Utilities (clipboard, keyboard)
├── stores/           # Zustand stores (graph, filter, ui, chat)
└── types/            # TypeScript types (re-exports novanet-core)
```

---

## Commands

```bash
pnpm dev             # Dev server (localhost:3000)
pnpm build           # Production build
pnpm lint            # ESLint
pnpm type-check      # TypeScript
pnpm test            # Tests
```

---

## Key Patterns

### DX Components
- `CopyButton` - Copy with JSON/TypeScript/YAML formats (self-contained + controlled modes)
- `CodeViewer` - Syntax highlighting (Prism) with Cypher grammar
- `KeyboardHelpPanel` - Searchable keyboard shortcuts modal (`?`)

### Keyboard Shortcuts

**Navigation**
| Key | Action |
|-----|--------|
| `⌘K` | Command palette |
| `⌘J` | AI Chat |
| `F` | Fit view to content |
| `=` / `-` | Zoom in / out |
| `?` | Show shortcuts modal |
| `Esc` | Close dialog / clear selection |

**View (v0.13.0)**
| Key | Action |
|-----|--------|
| `1` | Graph mode (unified tree: Realm > Layer > Class > Instance) |
| `2` | Nexus mode (hub: Quiz, Audit, Stats, Help) |
| `/` | Search overlay |
| `G` | Focus mode (Zen) |
| `M` | Toggle minimap |
| `L` | Toggle edge labels |
| `T` | Cycle trait filter (Defined → Authored → ... → None) |
| `E` | Cycle arc family filter (Ownership → ... → None) |
| `⇧L` | Cycle locale filter |
| `[` | Toggle left sidebar |
| `]` | Toggle right panel |

**Layout (⇧ = Shift)**
| Key | Action |
|-----|--------|
| `⇧H` | Horizontal layout (LR) |
| `⇧V` | Vertical layout (TB) |
| `⇧D` | Dagre layout (hierarchical + force) |
| `⇧R` | Radial layout (circular) |
| `⇧F` | Force-directed layout |

**Quick Views (Presets)**
| Key | Action |
|-----|--------|
| `⇧1` | Project Structure - Project, Pages, Blocks hierarchy |
| `⇧2` | Generation Chain - Entities with Content/Generated outputs |
| `⇧3` | Locale Knowledge - Locale with all knowledge atoms |
| `⇧4` | Entity Network - Entities and semantic links |
| `⇧5` | Prompts & Instructions - AI instructions and prompt artifacts |
| `⇧6` | SEO Intelligence - Search optimization data |
| `⇧7` | Defined Types - Nodes that do not change between locales |
| `⇧8` | Authored Content - Nodes written natively per locale |
| `⇧0` | All Nodes - Show everything |

**Graph Interaction**
| Action | Description |
|--------|-------------|
| Click | Select node |
| Double-click | Expand node neighbors |
| Drag | Move node |
| Tab | Next connected node |
| ⇧Tab | Previous connected node |
| Delete | Hide node from view |

### API Routes (12 routes)
- `/api/chat` - Claude AI endpoint
- `/api/graph` - Main graph data
- `/api/graph/expand` - Expand node neighbors
- `/api/graph/navigation` - Faceted navigation (realm/layer/trait filters)
- `/api/graph/ontology` - Ontology metadata
- `/api/graph/organizing-principles` - Organizing principles (realms, layers)
- `/api/graph/taxonomy` - Complete taxonomy with visual encoding (v10.9)
- `/api/graph/query` - Execute Cypher queries
- `/api/graph/schema` - Schema information
- `/api/graph/stats` - Graph statistics
- `/api/views` - Saved views CRUD
- `/api/views/[id]` - Individual view operations

### Zustand Stores (9 stores)
- `graphStore` - Nodes, edges, loading state
- `filterStore` - Node types, locale, presets (persisted)
- `uiStore` - Navigation mode (Graph/Nexus), panels, selection
- `chatStore` - AI chat messages, streaming
- `queryStore` - Cypher query state, history (Query-First)
- `viewStore` - Saved views management (29 views)
- `treeStore` - Unified tree state: expand/collapse, lazy loading
- `aiQueryStore` - AI-assisted query state
- `animationStore` - Graph animation controls

---

## Neo4j Schema (v0.13.0)

### Schema vs Data: The Core Distinction

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SCHEMA vs DATA - Understanding NovaNet's Graph Structure                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  SCHEMA NODES = Classes (structure)        DATA NODES = Instances (content)     │
│  ───────────────────────────────────       ─────────────────────────────────    │
│  :Schema:Class     (61 types)              :Locale, :Page, :Entity (200K+)      │
│  :Schema:Realm     (2: shared, org)        Actual content nodes                 │
│  :Schema:Layer     (10 layers)             Business data                        │
│  :Schema:Trait     (5 data origins)                                             │
│  :Schema:ArcFamily (5 families)            Link: (data)-[:OF_CLASS]->(schema)   │
│  :Schema:ArcClass  (169 relationships)                                          │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  CLASSIFICATION AXES (on NodeClass)        CLASSIFICATION VALUES                │
│  ──────────────────────────────────        ─────────────────────────────────    │
│  WHERE? → Realm                            shared (40 nodes, READ-ONLY)         │
│  WHAT?  → Layer                            org (21 nodes, multi-tenant)         │
│  HOW?   → Trait (Data Origin)                                                   │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  LAYERS (v0.13: 10 total = 4 shared + 6 org)                                    │
│  ────────────────────────────────────────────                                   │
│  SHARED: config → locale → geography → knowledge                                │
│  ORG:    config → foundation → structure → semantic → instruction → output      │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  TRAITS (5 data origins - WHERE does data come from?)                           │
│  ─────────────────────────────────────────────────────                          │
│  defined    : Human-created once (Entity, Page, Block)                          │
│  authored   : Human-written per locale (EntityNative, ProjectNative)          │
│  imported   : External data brought in (Term, Expression, Pattern)              │
│  generated  : LLM output (PageNative, BlockNative)                        │
│  retrieved  : Fetched from external APIs (SEOKeywordMetrics, GEOMetrics)        │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  NATIVE GENERATION (not translation!)                                           │
│  ────────────────────────────────────                                           │
│  Entity (defined) + Knowledge atoms (fr-FR) → Generate → EntityNative@fr-FR    │
│                                                                                 │
│  X Source → Translate → Target                                                  │
│  V Entity (defined) → Generate natively → EntityNative (locale-specific)       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Schema Graph (v0.13.0 - Unified Tree Architecture)

v0.13.0 establishes faceted classification where **everything is a clickable node**:

| Schema-Type | Count | Purpose | Clickable |
|-------------|-------|---------|-----------|
| **Realm** | 2 | WHERE? (shared / org) | Yes |
| **Layer** | 10 | WHAT? (4 shared + 6 org) | Yes |
| **Trait** | 5 | Data Origin (defined / authored / imported / generated / retrieved) | Yes |
| **ArcFamily** | 5 | Relationship classification | Yes |
| **ArcClass** | 169 | Individual relationship type | Yes |
| **Class** | 61 | Node type definitions (40 shared + 21 org) | Yes |

All schema nodes carry `:Schema` double-label. Instances link via `[:OF_CLASS]`.

### Realm Architecture (v0.13.0)

| Realm | Layers | Nodes | Description |
|-------|--------|-------|-------------|
| **Shared** | config, locale, geography, knowledge | 40 | Universal knowledge (READ-ONLY) |
| **Org** | config, foundation, structure, semantic, instruction, output | 21 | Business-specific content |

### Key Relations (grouped by ArcFamily)
- **Ownership:** `HAS_PAGE`, `HAS_BLOCK`, `OF_TYPE`, `SUPPORTS_LOCALE`, `HAS_PROJECT`
- **Localization:** `HAS_NATIVE`, `FOR_LOCALE`
- **Semantic:** `USES_ENTITY`, `SEMANTIC_LINK`, `HAS_ENTITY`
- **Generation:** `HAS_NATIVE`, `HAS_PROMPT`
- **Mining:** `EXPRESSES`, `HAS_SEO_TARGET`

### Navigation Modes (v0.13.0 - ADR-022)

**Consolidated from 5 modes to 2:**

| Mode | Key | Content | Use Case |
|------|-----|---------|----------|
| **Graph** | `1` | Unified tree: Realm > Layer > Class > Instance + Arcs | Default exploration |
| **Nexus** | `2` | Hub: Quiz, Audit, Stats, Help | Learning & validation |

**Deprecated modes** (v11.6 and earlier):
- `data`, `meta`, `overlay`, `query` → replaced by unified `Graph` mode
- `atlas` → renamed to `Nexus`

**Principle:** "If it's a node in Neo4j, it's a node everywhere"

---

## Integration with novanet-core

Types are imported via path alias:
```typescript
import { Project, Entity, Page, Locale } from '@novanet/core/types';
import type { NodeType, Layer, RelationType } from '@novanet/core/types';
```

Path configured in tsconfig.json:
```json
"@novanet/core/*": ["../../packages/core/src/*"]
```

> **v0.13.0 notes:**
> - `NodeCategory` was replaced by `Layer` in v9.0.0
> - `EntityContent` → `EntityNative`, `ProjectContent` → `ProjectNative` in v0.13.0 (ADR-029)
> - `PageGenerated` → `PageNative`, `BlockGenerated` → `BlockNative` in v0.13.0 (ADR-029)
> - `HAS_CONTENT`/`HAS_GENERATED` → `HAS_NATIVE`, `CONTENT_OF`/`GENERATED_FOR` → `NATIVE_OF` (ADR-029)
> - 5 navigation modes consolidated into 2 (Graph/Nexus) in v11.7
> - `Kind` → `Class`, `ArcKind` → `ArcClass`, `:Meta:` → `:Schema:` in v0.12.0
> - Traits renamed: invariant→defined, localized→authored, knowledge→imported, aggregated→retrieved

---

## Development Rules

1. **DX First** - Every interaction should feel instant and copyable
2. **Keyboard Navigation** - All actions accessible via keyboard (v11.7: `1`/`2` for modes)
3. **Query-First** - Cypher is source of truth for graph display (ADR-021)
4. **Unified Tree** - Everything is a clickable node (ADR-022)
5. **AI-Assisted** - Natural language queries to Cypher
6. **Type Safety** - Strict TypeScript, no `any`
7. **Test Before Commit** - All tests must pass

---

## AI Chat Guidelines

When generating Cypher queries:
1. Use parameterized queries when possible
2. Limit results (LIMIT 100 default)
3. Return relevant properties, not entire nodes
4. Explain the query in natural language

---

## Related Projects

- `../../packages/core` - Core types, schemas, filters (@novanet/core)
- `../../packages/db` - Neo4j infrastructure (@novanet/db)
