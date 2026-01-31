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
