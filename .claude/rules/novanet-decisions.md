# NovaNet Architecture Decisions (v11.0)

This file documents key architecture decisions for NovaNet. Reference these when making implementation choices.

## ADR-001: Arc Terminology

**Status**: Approved (v9.5)

**Decision**: Use "Arc" instead of "Edge" or "Relation" for directed links.

**Rationale**:
- Graph theory uses "arc" for directed edges
- Avoids confusion with React Flow's "Edge" (implementation detail)
- Avoids confusion with Neo4j's "Relationship" (database detail)
- Single consistent term across all platforms (YAML, Rust, TypeScript, UI)

**Exception**: React Flow components may use "Edge" internally since that's the library's API.

## ADR-002: Symmetric Taxonomy

**Status**: Approved (v9.5)

**Decision**: Use prefixed types for global uniqueness, short properties for local context.

```typescript
// Types with prefix (globally unique)
type NodeRealm = 'global' | 'tenant';  // v10.6: 2 realms
type ArcFamily = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';

// Properties without prefix (context is clear)
interface NodeKind {
  realm: NodeRealm;    // Not "nodeRealm"
  layer: NodeLayer;    // Not "nodeLayer"
}

interface ArcKind {
  family: ArcFamily;   // Not "arcFamily"
  scope: ArcScope;     // Not "arcScope"
}
```

**Rationale**: Avoids stuttering (`arcKind.arcFamily`) while maintaining type safety.

## ADR-003: YAML-First Architecture

**Status**: Approved (v9.0)

**Decision**: YAML files are the single source of truth. All code is generated.

```
taxonomy.yaml           → Colors, display names, facet definitions
node-kinds/**/*.yaml    → NodeKind definitions
arc-kinds/**/*.yaml     → ArcKind definitions
         ↓
    Rust Generator
         ↓
TypeScript + Cypher + Mermaid + Rust structs
```

**Rationale**:
- Single source of truth prevents drift
- Non-developers can edit YAML
- Generators enforce consistency
- CI validates sync

## ADR-004: No Color Duplication

**Status**: Approved (v9.5)

**Decision**: Colors are defined ONLY in `taxonomy.yaml`. Visual encoding references colors, doesn't duplicate them.

```yaml
# taxonomy.yaml - Colors defined here
node_realms:
  - key: global
    color: "#2aa198"

# visual-encoding.yaml - References, no hex values
channel_mapping:
  node:
    fill_color: layer    # Uses taxonomy.node_layers[].color
    border_color: realm  # Uses taxonomy.node_realms[].color
```

**Rationale**: Single source of truth for colors prevents inconsistencies.

## ADR-005: Trait-Based Visual Encoding

**Status**: Approved (v9.0)

**Decision**: Node trait (invariant/localized/knowledge/derived/job) is encoded via border style, not color.

| Trait | Border Style | CSS |
|-------|--------------|-----|
| invariant | solid | `border-2 border-solid` |
| localized | dashed | `border-2 border-dashed` |
| knowledge | double | `border-[3px] border-double` |
| derived | dotted | `border-2 border-dotted` |
| job | solid thin | `border border-solid` |

**Rationale**: Colorblind-safe. Layer already uses fill color.

## ADR-006: Realm Differentiates Scope

**Status**: Approved (v9.0, refined v10)

**Decision**: Same type name can exist in different realms with different scope.

```
Thing (shared)   → Universal definition (Wikidata-linked)
Thing (project)  → Brand-specific definition
```

**Rationale**: Realm is the WHERE axis. Type name is the WHAT axis. They're orthogonal.

## ADR-007: Generation, Not Translation

**Status**: Approved (core principle)

**Decision**: Content is generated natively per locale, NOT translated from a source.

```
WRONG:  Source → Translate → Target
RIGHT:  Entity (invariant) → Generate natively → EntityContent (local)
```

> **Note**: v10.9 renamed `EntityL10n` to `EntityContent`. See ADR-014.

**Rationale**: Translation loses cultural nuance. Native generation preserves it.

## ADR-008: Invariant Structure, Localized Content

**Status**: Approved (v9.0)

**Decision**: Relationships are defined at the invariant level. Content is resolved at generation time.

```
STRUCTURE = invariant (defined 1×)
CONTENT = localized (resolved 200×)
```

**Rationale**: Structure changes rarely. Content changes per locale.

## ADR-009: Terminal Color Graceful Degradation

**Status**: Approved (v9.5)

**Decision**: TUI supports three color modes with automatic fallback.

```
truecolor (24-bit RGB)
    ↓ not supported
256-color (xterm palette)
    ↓ not supported
16-color (ANSI)
```

**Rationale**: Works on all terminals from VS Code to minimal SSH.

## ADR-010: Skill-First DX

**Status**: Approved (v9.5)

**Decision**: Update DX tools (skills, CLAUDE.md, rules) BEFORE implementing code changes.

**Rationale**:
- Claude Code reads these files for context
- Outdated docs cause confusion during implementation
- DX is cheap to update, expensive to fix later

## ADR-011: Company Project Pattern (Superseded)

**Status**: Superseded by ADR-012 (v10.6)

**Decision**: Organization realm contains only the Organization node. Entity/EntityContent live in PROJECT realm only.

```
Organization ─[:HAS_COMPANY_PROJECT]→ Project (company project)
                                         └── Entity nodes here
             ─[:HAS_PROJECT]─────────→ Project (product projects)
```

**Rationale**:
- An organization has a "company project" that holds org-wide Entity nodes
- Entity/EntityContent in organization was redundant (same nodes existed in project)
- Simplifies the schema: 43 nodes instead of 45, 9 layers instead of 10
- Organization realm becomes a pure multi-tenant isolation boundary

> **Note**: v10.9 renamed `EntityL10n` to `EntityContent`. See ADR-014.

## ADR-012: 2-Realm Architecture

**Status**: Approved (v10.6)

**Decision**: Consolidate 3 realms into 2 realms: GLOBAL + TENANT.

```
v10.5 (3 realms):  global / organization / project
v10.6 (2 realms):  global / tenant
```

**Architecture** (updated v11.0):
- **GLOBAL** (2 layers): config, locale-knowledge — Universal, READ-ONLY
- **TENANT** (7 layers): config, foundation, structure, semantic, instruction, seo, output — Business-specific

**Rationale**:
- Organization + Project distinction added unnecessary complexity
- Tenant is the natural isolation boundary for multi-tenant SaaS
- Single realm for all business content simplifies queries and permissions
- 9 total layers (2 global + 7 tenant) provides sufficient granularity
- v11.0: SEO moved to tenant (business-specific keywords, not universal knowledge)

**Migration path**:
- `organization` -> `tenant` (rename)
- `project` -> `tenant` (merge into tenant)
- All node types from both organization and project now live under tenant

## ADR-013: Icons Source of Truth

**Status**: Approved (v10.6)

**Decision**: Centralize all icons in `visual-encoding.yaml`, providing both web (Lucide) and terminal (Unicode) variants.

**Location**: `packages/core/models/visual-encoding.yaml` → `icons:` section

**Structure**:
```yaml
icons:
  realms:           # global, tenant
  layers:           # config, locale-knowledge, seo, foundation, structure, semantic, instruction, output
  traits:           # invariant, localized, knowledge, derived, job
  arc_families:     # ownership, localization, semantic, generation, mining
  states:           # no_connection, no_kinds, no_results, no_instances, loading, success, error, warning
  navigation:       # expanded, collapsed, leaf, search, help, back, copy
  quality:          # complete, partial, empty, required, optional, chart
  modes:            # meta, data, overlay, query, atlas, audit

# Each icon has:
  category:
    key:
      web: "lucide-icon-name"    # For Studio/web
      terminal: "◉"              # Unicode for TUI
      description: "..."
```

**Generated artifacts**:
- `tools/novanet/src/tui/theme.rs` → `Icons` struct (loaded at runtime)
- Future: `packages/core/src/config/icons.generated.ts` (TypeScript constants)

**Rationale**:
- Single source of truth for ALL icons (no duplicates in code)
- Dual format: web (Lucide) + terminal (Unicode) for different contexts
- TUI loads icons from YAML at startup with fallback defaults
- Consistent iconography across Studio and TUI
- Colorblind-safe: icons supplement color, not replace it

**Categories explained**:
| Category | Purpose | Example |
|----------|---------|---------|
| realms | Where node lives | ◉ global, ◎ tenant |
| layers | Functional category | ⚙ config, ◆ semantic |
| traits | Locale behavior | ■ invariant, □ localized |
| states | UI empty states | ◐ loading, ∅ no_kinds |
| navigation | Tree controls | ▼ expanded, ▶ collapsed |
| quality | Data completeness | ● complete, ◐ partial |
| modes | Navigation modes | M meta, D data |

## ADR-014: Naming Convention Refactor (L10n to Content/Generated)

**Status**: Approved (v10.9.0)

**Decision**: Rename `*L10n` nodes and arcs to semantically clearer names that distinguish content storage from generation output.

**Renames**:

| Old Name | New Name | Reason |
|----------|----------|--------|
| `EntityL10n` | `EntityContent` | Stores semantic content, not localization metadata |
| `PageL10n` | `PageGenerated` | Clarifies this is LLM generation output |
| `BlockL10n` | `BlockGenerated` | Parallel naming with PageGenerated |
| `HAS_L10N` | `HAS_CONTENT` | Content relationship, not localization |
| `HAS_OUTPUT` | `HAS_GENERATED` | Moved to generation family, clarifies purpose |

**Composite Key Pattern**:

Content and generated nodes use composite keys to ensure uniqueness across locales:

```
EntityContent key:  entity:{entity_key}@{locale_key}
PageGenerated key:  page:{page_key}@{locale_key}
BlockGenerated key: block:{block_key}@{locale_key}

Examples:
  entity:qr-code-generator@fr-FR
  page:homepage@de-DE
  block:hero-section@ja-JP
```

**Key format**: `{kind}:{invariant_key}@{locale_key}`
- `{kind}`: lowercase node kind prefix (entity, page, block)
- `{invariant_key}`: key of the parent invariant node
- `@`: separator (not valid in keys, unambiguous parsing)
- `{locale_key}`: BCP-47 locale code

**Rationale**:

1. **Semantic clarity**: `L10n` (localization) implies translation, but NovaNet generates natively. `Content` and `Generated` describe what the node actually contains.

2. **Layer distinction**:
   - `EntityContent` lives in **semantic** layer (meaning, knowledge)
   - `PageGenerated`/`BlockGenerated` live in **output** layer (rendered artifacts)

3. **Arc family alignment**:
   - `HAS_CONTENT` stays in **semantic** family (content relationship)
   - `HAS_GENERATED` moves to **generation** family (output relationship)

4. **Composite key benefits**:
   - Unique across all locales without additional index
   - Parseable: extract invariant key or locale with simple split
   - Query-friendly: `STARTS WITH 'entity:qr-code-generator@'` finds all locales
   - Self-documenting: key reveals parent and locale at a glance

**Migration**:

```cypher
// Rename node labels
MATCH (n:EntityL10n) SET n:EntityContent REMOVE n:EntityL10n;
MATCH (n:PageL10n) SET n:PageGenerated REMOVE n:PageL10n;
MATCH (n:BlockL10n) SET n:BlockGenerated REMOVE n:BlockL10n;

// Update relationship types (requires recreation in Neo4j)
// HAS_L10N → HAS_CONTENT
// HAS_OUTPUT → HAS_GENERATED
```

**Code impact**:
- YAML: Update `node-kinds/` and `arc-kinds/` files
- Generators: Names propagate automatically via YAML-first architecture
- Queries: Search-replace in Cypher files and Rust code

## ADR-015: Unidirectional Ownership Arcs

**Status**: Approved (v10.9.0)

**Decision**: Ownership family arcs are intentionally unidirectional. Only a subset of ownership arcs have explicit inverse relationships.

**Arcs with inverses**:
- `HAS_BLOCK` ↔ `BLOCK_OF`
- `HAS_CONTENT` ↔ `CONTENT_OF`
- `HAS_GENERATED` ↔ `GENERATED_FOR`
- `HAS_TYPE` ↔ `TYPE_OF`

**Arcs without inverses** (intentional):
- `HAS_PAGE`, `HAS_ENTITY`, `HAS_TERMS`, `HAS_EXPRESSIONS`, etc.

**Rationale**:
- Ownership implies hierarchy: parent owns children
- Traversal is typically parent→child (downward)
- Inverse navigation uses explicit Cypher: `(child)<-[:HAS_*]-(parent)`
- Adding inverses for all 43 ownership arcs would double complexity without proportional benefit
- Content/Generated inverses exist because bidirectional traversal is common for those patterns

**When to add an inverse**:
- Frequent bidirectional traversal in LLM context loading
- Performance-critical paths that benefit from indexed reverse lookup
- NOT just for DX convenience

## ADR-016: Type-Constrained Container Arcs

**Status**: Approved (v10.9.0)

**Decision**: Split the generic `CONTAINS` arc into 6 type-specific arcs for semantic correctness.

**Previous** (v10.7):
```yaml
# Generic CONTAINS allowed 6×6=36 invalid combinations
arc:
  name: CONTAINS
  source: [ExpressionSet, TermSet, CultureSet, TabooSet, PatternSet, AudienceSet]
  target: [Expression, Term, CultureRef, Taboo, Pattern, AudienceTrait]
```

**New** (v10.9.0):
```yaml
# 6 type-specific arcs, 1:1 mapping only
CONTAINS_TERM:           TermSet → Term
CONTAINS_EXPRESSION:     ExpressionSet → Expression
CONTAINS_PATTERN:        PatternSet → Pattern
CONTAINS_CULTURE_REF:    CultureSet → CultureRef
CONTAINS_TABOO:          TabooSet → Taboo
CONTAINS_AUDIENCE_TRAIT: AudienceSet → AudienceTrait
```

**Rationale**:
- Semantic correctness: An ExpressionSet cannot contain a Taboo
- Graph validation: Type constraints prevent invalid data
- Query clarity: Arc name reveals target type
- No runtime overhead: Same performance, better semantics

**Impact**:
- Total arc count increases by 5 (1 generic → 6 specific)
- All existing CONTAINS arcs must be migrated with correct type suffix
- Queries must use specific arc type instead of generic CONTAINS

## Decision Log

| ADR | Version | Summary |
|-----|---------|---------|
| 001 | v9.5 | Arc terminology |
| 002 | v9.5 | Symmetric taxonomy (prefixed types) |
| 003 | v9.0 | YAML-first architecture |
| 004 | v9.5 | No color duplication |
| 005 | v9.0 | Trait-based visual encoding |
| 006 | v9.0 | Realm differentiates scope |
| 007 | core | Generation, not translation |
| 008 | v9.0 | Invariant structure, localized content |
| 009 | v9.5 | Terminal color graceful degradation |
| 010 | v9.5 | Skill-first DX |
| 011 | v10.5 | Company project pattern (superseded by 012) |
| 012 | v10.6 | 2-Realm Architecture |
| 013 | v10.6 | Icons source of truth |
| 014 | v10.9 | Naming convention refactor (L10n to Content/Generated) |
| 015 | v10.9 | Unidirectional ownership arcs |
| 016 | v10.9 | Type-constrained container arcs |

## References

- `docs/plans/2026-02-03-nomenclature-v95-design.md` — Full v9.5 design
- `docs/plans/2026-02-03-v10-brainstorm-decisions.md` — v10 roadmap decisions
- `docs/plans/2026-02-01-ontology-v9-design.md` — Original v9 design
