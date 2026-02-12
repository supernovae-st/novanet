# NovaNet Studio (v11.7)

@.claude/rules/novanet-terminology.md
@.claude/rules/novanet-decisions.md

Knowledge graph visualization for the NovaNet native content generation engine.

---

## Project Context

**What:** Interactive 2D/3D graph visualization for 60 node types, 200 locales (~19,000 instances projected)
**Stack:** Next.js 16 + React 19 + TypeScript 5.9 + Tailwind CSS
**Graph:** @xyflow/react (2D), @react-three/fiber (3D)
**State:** Zustand 5 with persist/immer
**DB:** Neo4j (bolt://localhost:7687)
**AI:** Claude API for natural language → Cypher
**Version:** v11.7.0 (Unified Tree Architecture)

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

**View (v11.7)**
| Key | Action |
|-----|--------|
| `1` | Graph mode (unified tree: Realm > Layer > Kind > Instance) |
| `2` | Nexus mode (hub: Quiz, Audit, Stats, Help) |
| `/` | Search overlay |
| `G` | Focus mode (Zen) |
| `M` | Toggle minimap |
| `L` | Toggle edge labels |
| `T` | Cycle trait filter (Invariant → Localized → ... → None) |
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

**Quick Views (Presets v11.7)**
| Key | Action |
|-----|--------|
| `⇧1` | Project Structure - Project, Pages, Blocks hierarchy |
| `⇧2` | Generation Chain - Entities with Content/Generated outputs |
| `⇧3` | Locale Knowledge - Locale with all knowledge atoms |
| `⇧4` | Entity Network - Entities and semantic links |
| `⇧5` | Prompts & Instructions - AI instructions and prompt artifacts |
| `⇧6` | SEO Intelligence - Search optimization data |
| `⇧7` | Invariant Types - Nodes that do not change between locales |
| `⇧8` | Localized Content - Nodes generated natively per locale |
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
- `treeStore` - Unified tree state: expand/collapse, lazy loading (v11.7)
- `aiQueryStore` - AI-assisted query state
- `animationStore` - Graph animation controls

---

## Neo4j Schema (v11.7.0)

### Meta vs Data: The Core Distinction

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  META vs DATA - Understanding NovaNet's Graph Structure                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  META NODES = Schema (structure)           DATA NODES = Instances (content)     │
│  ───────────────────────────────           ─────────────────────────────────    │
│  :Meta:Kind      (60 types)                :Locale, :Page, :Entity (200K+)      │
│  :Meta:Realm     (2: shared, org)          Actual content nodes                 │
│  :Meta:Layer     (10 layers)               Business data                        │
│  :Meta:Trait     (5 behaviors)                                                  │
│  :Meta:ArcFamily (5 families)              Link: (data)-[:OF_KIND]->(meta)      │
│  :Meta:ArcKind   (114 relationships)                                            │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  CLASSIFICATION AXES (on NodeKind)         CLASSIFICATION VALUES                │
│  ──────────────────────────────────        ─────────────────────────────────    │
│  WHERE? → Realm                            shared (39 nodes, READ-ONLY)         │
│  WHAT?  → Layer                            org (21 nodes, multi-tenant)         │
│  HOW?   → Trait                                                                 │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  LAYERS (v11.5: 10 total = 4 shared + 6 org)                                    │
│  ───────────────────────────────────────────                                    │
│  SHARED: config → locale → geography → knowledge                                │
│  ORG:    config → foundation → structure → semantic → instruction → output      │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  TRAITS (5 behaviors vis-à-vis locales)                                         │
│  ──────────────────────────────────────                                         │
│  invariant   : Same in all locales (Entity, Page, Block)                        │
│  localized   : Locale-specific content (EntityContent, ProjectContent)          │
│  knowledge   : Locale knowledge atoms (Term, Expression, Pattern)               │
│  generated   : LLM output (PageGenerated, BlockGenerated)                       │
│  aggregated  : Computed metrics (SEOKeywordMetrics, GEOMetrics)                 │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  NATIVE GENERATION (not translation!)                                           │
│  ────────────────────────────────────                                           │
│  Entity (invariant) + Knowledge atoms (fr-FR) → Generate → EntityContent@fr-FR  │
│                                                                                 │
│  ❌ Source → Translate → Target                                                 │
│  ✅ Entity (invariant) → Generate natively → EntityContent (locale-specific)    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Meta-Graph (v11.7 - Unified Tree Architecture)

v11.7 establishes faceted classification where **everything is a clickable node**:

| Meta-Type | Count | Purpose | Clickable |
|-----------|-------|---------|-----------|
| **Realm** | 2 | WHERE? (shared / org) | ✅ Yes |
| **Layer** | 10 | WHAT? (4 shared + 6 org) | ✅ Yes |
| **Trait** | 5 | HOW? (invariant / localized / knowledge / generated / aggregated) | ✅ Yes |
| **ArcFamily** | 5 | Relationship classification | ✅ Yes |
| **ArcKind** | 114 | Individual relationship type | ✅ Yes |
| **Kind** | 60 | Node type definitions (39 shared + 21 org) | ✅ Yes |

All meta-nodes carry `:Meta` double-label. Instances link via `[:OF_KIND]`.

### Realm Architecture (v11.5)

| Realm | Layers | Nodes | Description |
|-------|--------|-------|-------------|
| **Shared** | config, locale, geography, knowledge | 39 | Universal knowledge (READ-ONLY) |
| **Org** | config, foundation, structure, semantic, instruction, output | 21 | Business-specific content |

### Key Relations (grouped by ArcFamily)
- **Ownership:** `HAS_PAGE`, `HAS_BLOCK`, `OF_TYPE`, `SUPPORTS_LOCALE`, `HAS_PROJECT`
- **Localization:** `HAS_CONTENT`, `FOR_LOCALE`
- **Semantic:** `USES_ENTITY`, `SEMANTIC_LINK`, `HAS_ENTITY`
- **Generation:** `HAS_GENERATED`, `HAS_PROMPT`
- **Mining:** `EXPRESSES`, `HAS_SEO_TARGET`

### Navigation Modes (v11.7 - ADR-022)

**v11.7 consolidates 5 modes into 2:**

| Mode | Key | Content | Use Case |
|------|-----|---------|----------|
| **Graph** | `1` | Unified tree: Realm > Layer > Kind > Instance + Arcs | Default exploration |
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

> **v11.7 notes:**
> - `NodeCategory` was replaced by `Layer` in v9.0.0
> - `EntityL10n` → `EntityContent`, `PageL10n` → `PageGenerated`, `BlockL10n` → `BlockGenerated` in v10.9
> - 5 navigation modes consolidated into 2 (Graph/Nexus) in v11.7

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
