# NovaNet Visualizer

@.claude/rules/novanet-terminology.md
@.claude/rules/novanet-decisions.md

Knowledge graph visualization for the NovaNet localization orchestrator.

---

## Project Context

**What:** Interactive 2D graph visualization for node types (2 realms), 200 locales (~19,000 instances projected at full deployment)
**Stack:** Next.js 16 + React 19 + TypeScript 5.9 + Tailwind CSS
**Graph:** @xyflow/react
**State:** Zustand 5 with persist/immer
**DB:** Neo4j (bolt://localhost:7687)
**AI:** Claude API for natural language → Cypher

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

**View**
| Key | Action |
|-----|--------|
| `N` | Cycle navigation mode (Data → Meta → Overlay → Query) |
| `V` | Reserved (layout toggle) |
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

**Quick Views (Presets v10.6.0)**
| Key | Action |
|-----|--------|
| `1` | Project Structure - Project, Pages, Blocks hierarchy |
| `2` | Generation Chain - Entities with L10n outputs |
| `3` | Locale Knowledge - Locale with all knowledge nodes |
| `4` | Entity Network - Entities and semantic links |
| `5` | Prompts & Rules - AI instructions and validation rules |
| `6` | SEO Targeting - Search optimization data |
| `7` | Invariant Types - Nodes that do not change between locales |
| `8` | Localized Content - Nodes generated natively per locale |
| `0` | All Nodes - Show everything |

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
- `/api/graph/taxonomy` - Complete taxonomy with visual encoding (v10.6)
- `/api/graph/query` - Execute Cypher queries
- `/api/graph/schema` - Schema information
- `/api/graph/stats` - Graph statistics
- `/api/views` - Saved views CRUD
- `/api/views/[id]` - Individual view operations

### Zustand Stores (8 stores)
- `graphStore` - Nodes, edges, loading state
- `filterStore` - Node types, locale, presets (persisted)
- `uiStore` - View mode, panels, selection
- `chatStore` - AI chat messages, streaming
- `queryStore` - Cypher query state, history
- `viewStore` - Saved views management
- `aiQueryStore` - AI-assisted query state
- `animationStore` - Graph animation controls

---

## Neo4j Schema (v10.6.0)

### Meta-Graph (v10.6 — 2-Realm Architecture)

v10.6 establishes faceted classification with 6 meta-node types:

| Meta-Type | Count | Purpose |
|-----------|-------|---------|
| **Realm** | 2 | WHERE? (global / tenant) — one-way traversal |
| **Layer** | 9 | WHAT? (functional classification by realm) |
| **Trait** | 5 | HOW? (invariant / localized / knowledge / derived / job) |
| **ArcFamily** | 5 | Relationship classification |
| **ArcKind** | varies | Individual relationship type |

All meta-nodes carry `:Meta` double-label.

### Realm Architecture (v10.6)

| Realm | Layers | Description |
|-------|--------|-------------|
| **Global** | config, locale-knowledge, seo | Universal locale knowledge (READ-ONLY) |
| **Tenant** | config, foundation, structure, semantic, instruction, output | Business-specific content |

### Key Relations (grouped by ArcFamily)
- **Ownership:** `HAS_PAGE`, `HAS_BLOCK`, `OF_TYPE`, `SUPPORTS_LOCALE`, `BELONGS_TO_ORG`, `HAS_PROJECT`
- **Localization:** `HAS_L10N`, `FOR_LOCALE`
- **Semantic:** `USES_ENTITY`, `SEMANTIC_LINK`, `HAS_ENTITY`
- **Generation:** `HAS_OUTPUT`, `HAS_PROMPT`
- **Mining:** `EXPRESSES`

### NavigationMode (replaces DataMode)

| Mode | Content | Use Case |
|------|---------|----------|
| `data` | Real instances only | Default exploration |
| `meta` | Meta-graph only | Schema understanding |
| `overlay` | Data + meta combined | Architecture debugging |
| `query` | Faceted filter results | Targeted exploration |

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

> **v9 note:** `NodeCategory` was replaced by `Layer` in v9.0.0.

---

## Development Rules

1. **DX First** - Every interaction should feel instant and copyable
2. **Keyboard Navigation** - All actions accessible via keyboard
3. **Filter Presets** - Quick views for common workflows
4. **AI-Assisted** - Natural language queries to Cypher
5. **Type Safety** - Strict TypeScript, no `any`
6. **Test Before Commit** - All tests must pass

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
