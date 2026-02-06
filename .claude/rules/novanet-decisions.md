# NovaNet Architecture Decisions (v10.6)

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
RIGHT:  Entity (invariant) → Generate natively → EntityL10n (local)
```

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

**Decision**: Organization realm contains only the Organization node. Entity/EntityL10n live in PROJECT realm only.

```
Organization ─[:HAS_COMPANY_PROJECT]→ Project (company project)
                                         └── Entity nodes here
             ─[:HAS_PROJECT]─────────→ Project (product projects)
```

**Rationale**:
- An organization has a "company project" that holds org-wide Entity nodes
- Entity/EntityL10n in organization was redundant (same nodes existed in project)
- Simplifies the schema: 43 nodes instead of 45, 9 layers instead of 10
- Organization realm becomes a pure multi-tenant isolation boundary

## ADR-012: 2-Realm Architecture

**Status**: Approved (v10.6)

**Decision**: Consolidate 3 realms into 2 realms: GLOBAL + TENANT.

```
v10.5 (3 realms):  global / organization / project
v10.6 (2 realms):  global / tenant
```

**Architecture**:
- **GLOBAL** (3 layers): config, locale-knowledge, seo — Universal, READ-ONLY
- **TENANT** (6 layers): config, foundation, structure, semantic, instruction, output — Business-specific

**Rationale**:
- Organization + Project distinction added unnecessary complexity
- Tenant is the natural isolation boundary for multi-tenant SaaS
- Single realm for all business content simplifies queries and permissions
- 9 total layers (3 global + 6 tenant) provides sufficient granularity

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

## References

- `docs/plans/2026-02-03-nomenclature-v95-design.md` — Full v9.5 design
- `docs/plans/2026-02-03-v10-brainstorm-decisions.md` — v10 roadmap decisions
- `docs/plans/2026-02-01-ontology-v9-design.md` — Original v9 design
