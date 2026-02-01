# Architecture Decisions

## ADR-001: 2D Graph Visualization

**Decision:** Focus on 2D visualization with React Flow

**Rationale:**
- 2D provides precise editing and relationship navigation
- Schema layouts (treemap, swimlanes, target, force) cover all visualization needs
- Simplified codebase without 3D dependency overhead

**Implementation:**
- `@xyflow/react` for all graph rendering
- Multiple layout algorithms via ELK.js
- `V` key reserved for future layout features

**Note:** 3D support (react-force-graph-3d) was deprecated in v8.2.1 for reduced bundle size.

---

## ADR-002: Filter Presets with Keyboard Shortcuts

**Decision:** 10 built-in presets accessible via 1-9 and 0 keys

**Rationale:**
- ~19k instances projected at full deployment - too large to show at once
- Common workflows need quick access
- Keyboard-first design philosophy

**Presets (v8.2.0):**
1. Project Overview - Core structure (Project, Pages, Blocks, Concepts)
2. Full Graph - All 35 node types
3. Core + Concepts - Project structure with ConceptL10n
4. All Locales - Locale nodes only
5. Concepts - Concept + ConceptL10n
6. Current Locale - Selected locale with all knowledge
7. Locale + Expressions - Current locale with expressions
8. Locale Knowledge - Identity, Voice, Culture, Market, Lexicon
9. Expressions - Expression + LocaleLexicon
0. Clear Filters - Reset to default view

**v9 Note:** Presets will migrate to faceted filters (Realm/Layer/Trait dimensions).

---

## ADR-003: AI Chat for Natural Language Queries

**Decision:** Claude API translates natural language to Cypher

**Rationale:**
- Cypher syntax steep learning curve
- Natural language more accessible
- AI can explain results

**Implementation:**
- `/api/chat` route with Claude
- System prompt with full schema
- Returns Cypher + explanation
- Execute button to run query

---

## ADR-004: Zustand with Persist + Immer

**Decision:** Zustand 5 with persist and immer middlewares

**Rationale:**
- Lightweight vs Redux
- Persist filters across sessions
- Immer for immutable updates

**Stores (8 total):**
- `graphStore` - No persist (fetched data)
- `filterStore` - Persist (user preferences)
- `uiStore` - Partial persist (layout prefs)
- `chatStore` - No persist (session only)
- `queryStore` - No persist (Cypher query state)
- `viewStore` - Persist (saved views)
- `aiQueryStore` - No persist (AI-assisted query state)
- `animationStore` - No persist (graph animation controls)

---

## ADR-005: DX-First Component Design

**Decision:** Every property is copyable, every action has a shortcut

**Rationale:**
- Developer users need to copy IDs, properties, queries
- Keyboard navigation faster than mouse
- Reduce friction in exploration workflow

**Components:**
- `CopyButton` - JSON/TS/YAML formats (self-contained + controlled modes)
- `CodeViewer` - Syntax highlighting + copy (with Cypher grammar)
- `KeyboardHelpPanel` - Searchable keyboard shortcuts modal

---

## ADR-006: Type Sharing with novanet-core

**Decision:** Import types from novanet-core via path alias

**Rationale:**
- Single source of truth for types
- Avoid type drift between projects
- Easy to update when schema changes

**Implementation:**
```json
{
  "paths": {
    "@novanet/core/*": ["../../packages/core/src/*"]
  }
}
```

---

## ADR-007: Glassmorphism UI Theme

**Decision:** Dark theme with glass effects (blur + transparency)

**Rationale:**
- Graph visualization needs dark background
- Glass effects create depth without distraction
- Consistent with modern data visualization tools

**Implementation:**
- `bg-black` base
- `glass` utility class (blur + border + transparency)
- `novanet-*` brand colors for accents

---

## ADR-008: Faceted Classification (v9)

**Decision:** Replace flat tree (Scope > Subcategory > NodeTypeMeta) with faceted classification (Realm + Layer + Trait + EdgeFamily)

**Rationale:**
- Flat tree only classifies by WHERE > WHAT, missing HOW (locale behavior)
- No edge metadata — relation families only exist as YAML comments
- Generic naming ("Scope", "Subcategory") are implementation terms, not domain concepts
- Single-axis navigation insufficient for AI agent discovery

**Implementation:**
- 6 meta-node types: Realm (3), Layer (9), Kind (35), Trait (5), EdgeFamily (5), EdgeKind (47)
- Dual navigation: top-down hierarchy + Kind-centric facets
- All meta-nodes carry `:Meta` double-label
- YAML remains source of truth, meta-graph is generated

**Rename mapping:** Scope -> Realm, Subcategory -> Layer, NodeTypeMeta -> Kind

---

## ADR-009: Self-Describing Meta-Graph (v9)

**Decision:** Kind nodes carry `schema_hint`, `context_budget`, `cypher_pattern` for autonomous AI discovery

**Rationale:**
- LLM orchestrator should discover schema without reading YAML files
- Token-aware context assembly needs priority hints on each Kind
- Ready-to-use Cypher patterns enable query construction without schema knowledge

**Key properties:**
- `schema_hint` on Kind: "key, display_name, instructions (req), locale_behavior"
- `context_budget` on Kind: high/medium/low/minimal token priority
- `cypher_pattern` on EdgeKind: "(Page)-[:HAS_BLOCK]->(Block)"
- `traversal_depth` on Kind: v10 placeholder (nullable)
- `temperature_threshold` on EdgeKind: v10 placeholder (nullable)

---

## ADR-010: Rust-First Architecture (v9)

**Decision:** Single Rust binary `novanet` handles ALL schema and graph operations (generation, validation, queries, TUI, search, filters, db seeding)

**Rationale:**
- ~5ms startup vs ~800ms for Node.js
- Single binary, zero runtime dependencies
- Eliminates `@novanet/schema-tools` and `@novanet/cli` entirely (~7k lines TS)
- TUI for interactive exploration (ratatui)
- CLI for scripting and CI/CD

**Implementation:**
- Single crate at `tools/novanet/`
- Dependencies: clap, ratatui, crossterm, neo4rs, tokio, serde_yaml, tera
- Modules: commands/, generators/, parsers/, search/, filter/, tui/
- 13 subcommands: schema, db, locale, doc, search, filter, data/meta/overlay/query, node, relation, tui

**Architecture rule:** Single `novanet` Rust binary handles ALL schema and graph operations. TypeScript limited to: Studio web app, core/types, core/schemas (Zod).

---

## ADR-011: Rust-First Architecture Decision (v9)

**Decision:** Single `novanet` Rust binary owns ALL schema and graph operations. `@novanet/schema-tools` and `@novanet/cli` are eliminated entirely.

**Ownership:**

| Concern | Owner | Rationale |
|---------|-------|-----------|
| YAML -> TypeScript types | Rust (novanet) via Tera templates | Generates .ts files from Rust |
| YAML -> Mermaid diagrams | Rust (novanet) | String generation, no TS needed |
| YAML -> Cypher seeds/migrations | Rust (novanet) | String generation, no TS needed |
| YAML -> layers.ts / hierarchy.ts | Rust (novanet) via Tera templates | Replaces schema-tools generators |
| YAML <-> Neo4j validation | Rust (novanet) | Single authoritative validator |
| Graph read queries | Rust (novanet) | Runtime performance |
| Graph write (CRUD) | Rust (novanet) | Meta-graph validation at write time |
| Interactive TUI | Rust (novanet) | Native terminal, ~5ms startup |
| Cypher filter building | Rust (novanet filter build) | Studio calls via subprocess |
| Web visualization | TS (Studio) | Separate web concern |

**Eliminated:** `@novanet/schema-tools` (~2,038 lines), `@novanet/cli` (~5 lines), `core/scripts/`, `core/parsers/`, `core/services/`, `core/generators/`, `core/db/client.ts` (~6,743 lines total)

---

## ADR-012: NavigationMode (v9)

**Decision:** Replace binary DataMode (data/schema) with 4-mode NavigationMode

**Modes:**

| Mode | Content | Use Case |
|------|---------|----------|
| `data` | Real instances only | Default exploration |
| `meta` | Meta-graph only (Realm/Layer/Kind/Trait/EdgeFamily) | Schema understanding |
| `overlay` | Data + meta-graph combined | Architecture debugging |
| `query` | Faceted filter results | Targeted exploration |

**Rationale:**
- Binary data/schema toggle is insufficient for v9's richer meta-graph
- AI agents need programmatic access to all 4 modes
- Query mode enables faceted filter combinations (Realm + Trait + Layer)

---

## ADR-013: OF_KIND Instance Bridge (v9)

**Decision:** Replace IN_SUBCATEGORY with OF_KIND — every data node links to its Kind meta-node

**Rationale:**
- IN_SUBCATEGORY skips the type level (links to subcategory, not type)
- OF_KIND enables direct Kind -> instance traversal
- Autowired during seed: for each data node, create OF_KIND to matching Kind

**Performance:**
- 35 additional relationships per data node instance
- O(1) lookup: `MATCH (n:Block)-[:OF_KIND]->(k:Kind {label: 'Block'})`
- Indexed on Kind.label for fast traversal

---

## ADR-014: Trait-Based Visual Encoding (v9)

**Decision:** Map each facet to a distinct visual channel in Studio

**Visual channels:**

| Channel | Facet | Encoding |
|---------|-------|----------|
| Fill color | Layer (9 colors) | Node background |
| Border style | Trait (5 styles) | Solid/dashed/dotted/double/none |
| Spatial grouping | Realm (3 zones) | Layout position |
| Edge color | EdgeFamily (5 colors) | Relationship stroke |

**Rationale:**
- No overloading: each facet maps to exactly one visual channel
- Colorblind-safe: Trait uses shape/border, not color
- Familiar: color = functional category, position = governance scope
