# NovaNet Visualizer

@.claude/rules/novanet-terminology.md
@.claude/rules/novanet-decisions.md

Knowledge graph visualization for the NovaNet localization orchestrator.

---

## Project Context

**What:** Interactive 2D/3D graph visualization for 35 node types (3 scopes), 200 locales (~19,000 instances projected at full deployment)
**Stack:** Next.js 16 + React 19 + TypeScript 5.9 + Tailwind CSS
**Graph:** @xyflow/react (2D) + react-force-graph-3d (3D)
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
│   ├── graph/        # React Flow + force-graph components
│   ├── panels/       # Control panels
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
- `CopyButton` - Copy with JSON/TypeScript/YAML formats
- `CodeViewer` - Syntax highlighting (Prism)
- `PropertyInspector` - Expandable property tree

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
| `V` | Toggle 2D/3D view |
| `G` | Focus mode (Zen) |
| `M` | Toggle minimap |
| `L` | Toggle edge labels |
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

**Quick Views (Presets v8.2.0)**
| Key | Action |
|-----|--------|
| `1` | Project Overview - Core structure (Project, Pages, Blocks, Concepts) |
| `2` | Full Graph - All 35 node types |
| `3` | Core + Concepts - Project structure with ConceptL10n |
| `4` | All Locales - Locale nodes only |
| `5` | Concepts - Concept + ConceptL10n |
| `6` | Current Locale - Selected locale with all knowledge |
| `7` | Locale + Expressions - Current locale with expressions |
| `8` | Locale Knowledge - Identity, Voice, Culture, Market, Lexicon |
| `9` | Expressions - Expression + LocaleLexicon |
| `0` | Clear Filters - Reset to default view |

**Graph Interaction**
| Action | Description |
|--------|-------------|
| Click | Select node |
| Double-click | Expand node neighbors |
| Drag | Move node |
| Tab | Next connected node |
| ⇧Tab | Previous connected node |
| Delete | Hide node from view |

### API Routes (9 routes)
- `/api/chat` - Claude AI endpoint
- `/api/graph` - Main graph data
- `/api/graph/expand` - Expand node neighbors
- `/api/graph/ontology` - Ontology metadata
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

## Neo4j Schema (v8.2.0)

### Node Types (35 across 3 scopes)

| Scope | Nodes | Types |
|-------|-------|-------|
| **🌍 Global** | 15 | Locale, LocaleIdentity, LocaleVoice, LocaleCulture, LocaleCultureReferences, LocaleMarket, LocaleLexicon, LocaleRulesAdaptation, LocaleRulesFormatting, LocaleRulesSlug, Expression, Reference, Metaphor, Constraint, Pattern |
| **📦 Project** | 14 | Project, BrandIdentity, ProjectL10n, Page, Block, BlockType, PageType, Concept, ConceptL10n, PagePrompt, BlockPrompt, BlockRules, PageL10n, BlockL10n |
| **🎯 Shared** | 6 | SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun, GEOSeedL10n, GEOSeedMetrics, GEOMiningRun |

### Key Relations
- `HAS_CONCEPT` / `HAS_PAGE` - Project structure
- `SUPPORTS_LOCALE` / `DEFAULT_LOCALE` - Project → Locale
- `HAS_L10N` / `HAS_OUTPUT` - Invariant → L10n nodes (curated vs generated)
- `HAS_BLOCK` - Page → Block (with `position`)
- `USES_CONCEPT` - Page/Block → Concept (with `purpose`, `temperature`)
- `SEMANTIC_LINK` - Concept → Concept (spreading activation)
- `HAS_SEO_TARGET` / `HAS_GEO_TARGET` - ConceptL10n → Keywords/Seeds
- `FOR_LOCALE` - L10n nodes → Locale

---

## Integration with novanet-core

Types are imported via path alias:
```typescript
import { Project, Concept, Page, Locale } from '@novanet/core/types';
import type { NodeType, NodeCategory, RelationType } from '@novanet/core/types';
```

Path configured in tsconfig.json:
```json
"@novanet/core/*": ["../../packages/core/src/*"]
```

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
