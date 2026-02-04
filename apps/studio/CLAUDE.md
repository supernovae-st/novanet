# NovaNet Visualizer

@.claude/rules/novanet-terminology.md
@.claude/rules/novanet-decisions.md

Knowledge graph visualization for the NovaNet localization orchestrator.

---

## Project Context

**What:** Interactive 2D graph visualization for 42 node types (3 realms), 200 locales (~19,000 instances projected at full deployment)
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

**Quick Views (Presets v10.0.0)**
| Key | Action |
|-----|--------|
| `1` | Project Structure - Project, Pages, Blocks hierarchy |
| `2` | Generation Chain - Concepts with L10n outputs |
| `3` | Locale Knowledge - Locale with all knowledge nodes |
| `4` | Concept Network - Concepts and semantic links |
| `5` | Prompts & Rules - AI instructions and validation rules |
| `6` | SEO & GEO - Search optimization data |
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
- `/api/graph/taxonomy` - Complete taxonomy with visual encoding (v9.5)
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

## Neo4j Schema (v10.0.0)

### Meta-Graph (v9 — Self-Describing Schema)

v9 introduces faceted classification with 6 meta-node types:

| Meta-Type | Count | Purpose |
|-----------|-------|---------|
| **Realm** | 3 | WHERE? (global / project / shared) — replaces "Scope" |
| **Layer** | 9 | WHAT? (functional classification) — replaces "Subcategory" |
| **Kind** | 42 | Node type (1:1 with Neo4j labels) — replaces "NodeTypeMeta" |
| **Trait** | 5 | HOW? (invariant / localized / knowledge / derived / job) |
| **ArcFamily** | 5 | Relationship classification |
| **ArcKind** | 76 | Individual relationship type |

All meta-nodes carry `:Meta` double-label.

### Kind Types (42 across 3 Realms)

| Realm | Nodes | Kinds |
|-------|-------|-------|
| **🌍 Global** | 11 | Locale, Formatting, Slugification, Adaptation, Style, TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet |
| **📦 Project** | 23 | Project, BrandIdentity, ProjectL10n, Page, Block, BlockType, PageType, Concept, ConceptL10n, PagePrompt, BlockPrompt, BlockRules, PageL10n, BlockL10n, GenerationJob, PromptArtifact, OutputArtifact, EvaluationSignal, ContentSlot, TopicCluster, SearchIntent, AudiencePersona, ChannelSurface |
| **🎯 Shared** | 8 | SEOKeyword, SEOKeywordMetrics, SEOMiningRun, GEOSeedL10n, GEOSeedMetrics, GEOMiningRun, Thing, ThingL10n |

### Key Relations (grouped by ArcFamily)
- **Ownership:** `HAS_CONCEPT`, `HAS_PAGE`, `HAS_BLOCK`, `OF_TYPE`, `SUPPORTS_LOCALE`
- **Localization:** `HAS_L10N`, `FOR_LOCALE`
- **Semantic:** `USES_CONCEPT`, `SEMANTIC_LINK`
- **Generation:** `HAS_OUTPUT`, `HAS_PROMPT`
- **Mining:** `HAS_SEO_TARGET`, `HAS_GEO_TARGET`

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
import { Project, Concept, Page, Locale } from '@novanet/core/types';
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
