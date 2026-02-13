# NovaNet Architecture Decisions (v0.12.0)

This file documents key architecture decisions for NovaNet. Reference these when making implementation choices.

> **Version note**: v0.12.0 "Class Act" — previously versioned as v11.x (missing leading 0).

---

## Quick Reference (TL;DR)

### Core Principles

| ADR | Principle | Rule |
|-----|-----------|------|
| **007** | Generation NOT Translation | `Entity → Generate natively → EntityContent` (not translate) |
| **003** | YAML-First | YAML = source of truth → generators → TS/Cypher/Mermaid |
| **001** | Arc Terminology | Use "Arc" (not Edge/Relation) for directed links |

### Current Architecture (v0.12.0)

```
SHARED (4 layers, 39 nodes): config, locale, geography, knowledge — READ-ONLY
ORG (6 layers, 21 nodes): config, foundation, structure, semantic, instruction, output
Total: 60 nodes, 10 layers, 5 traits
```

### v0.12.0 "Class Act" Key Changes

| ADR | Change | Summary |
|-----|--------|---------|
| **024** | Trait = Data Origin | `invariant→defined`, `localized→authored`, `knowledge→imported`, `aggregated→retrieved` |
| **023** | Class/Instance | `NodeKind→NodeClass`, `:Meta:Kind→:Schema:Class`, "Meta" eliminated |
| **025** | Instruction Rename | `PageType→PageStructure`, `*Prompt→*Instruction` |

### Trait Quick Reference (ADR-024)

| Trait | Origin | Examples |
|-------|--------|----------|
| `defined` | Human creates ONCE | Page, Block, Entity, Locale, OrgConfig |
| `authored` | Human writes PER locale | EntityContent, ProjectContent |
| `imported` | External data brought in | Term, Expression, SEOKeyword, GEOQuery |
| `generated` | Our LLM produces | PageGenerated, BlockGenerated |
| `retrieved` | Fetched from external APIs | GEOAnswer, SEOKeywordMetrics |

### UX Architecture

| ADR | Feature | Rule |
|-----|---------|------|
| **022** | Unified Tree | 2 modes: `[1]Graph` + `[2]Nexus`. "If it's a node in Neo4j, it's a node everywhere" |
| **021** | Query-First | Cypher = source of truth. YAML views only (no TS hardcoding) |
| **014** | Naming | `*Content` = localized, `*Generated` = LLM output. Keys: `entity:key@locale` |
| **013** | Icons | `visual-encoding.yaml` → `{ web: "lucide", terminal: "◆" }` — NO emoji |

---

## Full ADR Documentation

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
type NodeRealm = 'shared' | 'org';  // v11.2: renamed from global/tenant
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
  - key: shared       # v11.2: renamed from global
    color: "#2aa198"
  - key: org          # v11.2: renamed from tenant
    color: "#0ea5e9"

# visual-encoding.yaml - References, no hex values
channel_mapping:
  node:
    fill_color: layer    # Uses taxonomy.node_layers[].color
    border_color: realm  # Uses taxonomy.node_realms[].color
```

**Rationale**: Single source of truth for colors prevents inconsistencies.

## ADR-005: Trait-Based Visual Encoding

**Status**: Approved (v9.0, updated v11.2)

**Decision**: Node trait (invariant/localized/knowledge/generated/aggregated) is encoded via border style, not color.

| Trait | Border Style | CSS |
|-------|--------------|-----|
| invariant | solid | `border-2 border-solid` |
| localized | dashed | `border-2 border-dashed` |
| knowledge | double | `border-[3px] border-double` |
| generated | dotted | `border-2 border-dotted` |
| aggregated | dotted thin | `border border-dotted` |

> **v11.2 Changes**: `derived` → split into `generated` + `aggregated`, `job` removed.

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

**Status**: Approved (v10.6, updated v11.2)

**Decision**: Consolidate 3 realms into 2 realms: SHARED + ORG.

```
v10.5 (3 realms):  global / organization / project
v10.6 (2 realms):  global / tenant
v11.2 (2 realms):  shared / org  (renamed for clarity)
```

**Architecture** (v11.2):
- **SHARED** (2 layers): config, locale-knowledge — Universal, READ-ONLY (32 nodes)
- **ORG** (7 layers): config, foundation, structure, semantic, instruction, seo, output — Business-specific (30 nodes)

> **v11.2 Changes**:
> - `global` → `shared` (describes WHAT: shared resources)
> - `tenant` → `org` (describes WHO: organization-specific, familiar terminology)
> - 3 job nodes removed (GenerationJob, SEOMiningRun, EvaluationSignal)
> - Total: 62 nodes (was 65)

**Rationale**:
- Organization + Project distinction added unnecessary complexity
- Org is the natural isolation boundary for multi-tenant SaaS
- Single realm for all business content simplifies queries and permissions
- 9 total layers (2 shared + 7 org) provides sufficient granularity
- v11.0: SEO moved to org (business-specific keywords, not universal knowledge)
- v11.2: `shared` describes purpose, `org` is familiar (GitHub/Slack orgs)

**Migration path**:
- `global` -> `shared` (rename)
- `tenant` -> `org` (rename)
- All node types from both organization and project now live under org

## ADR-013: Icons Source of Truth

**Status**: Approved (v10.6, updated v11.2)

**Decision**: Centralize all icons in `visual-encoding.yaml`, providing both web (Lucide) and terminal (Unicode) variants.

**Location**: `packages/core/models/visual-encoding.yaml` → `icons:` section

**Structure**:
```yaml
icons:
  realms:           # shared, org (v11.2: renamed from global, tenant)
  layers:           # config, locale-knowledge, seo, foundation, structure, semantic, instruction, output
  traits:           # invariant, localized, knowledge, generated, aggregated (v11.2: derived split, job removed)
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
| realms | Where node lives | ◉ shared, ◎ org |
| layers | Functional category | ⚙ config, ◆ semantic |
| traits | Locale behavior | ■ invariant, □ localized, ✦ generated, ⋆ aggregated |
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
| `ProjectL10n` | `ProjectContent` | v11.0: Consistent with EntityContent pattern |
| `HAS_L10N` | `HAS_CONTENT` | Content relationship, not localization |
| `HAS_OUTPUT` | `HAS_GENERATED` | Moved to generation family, clarifies purpose |
| `BELONGS_TO_PROJECT_L10N` | `BELONGS_TO_PROJECT_CONTENT` | v11.0: Follows ProjectContent rename |
| `GEOSeedL10n` | `GEOQuery` | v10.7: New GEO schema |
| `GEOSeedMetrics` | `GEOMetrics` | v10.7: New GEO schema |

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

## ADR-017: EntityCategory Classification

**Status**: Approved (v11.1, updated v11.2 for realm renames)

**Decision**: Replace Entity.type enum property with EntityCategory nodes and BELONGS_TO arcs.

**Problem**:
Entity had a `type` enum property with 13 hardcoded values (THING, CONTENT_TYPE, PLACE, PERSON, ORGANIZATION, EVENT, CONCEPT, PRODUCT, SERVICE, RESOURCE, MEDIA, DOCUMENT, ABSTRACT).
Properties are difficult to query and extend. Moving classification to the graph enables queryable, extensible categorization.

**Solution**:
- Create EntityCategory node type in `shared/config` layer (13 nodes, invariant trait)
- Add BELONGS_TO arc from Entity (org/semantic) to EntityCategory (shared/config) — cross_realm, ownership family
- Remove Entity.type enum property

**Structure**:
```
EntityCategory (shared/config, invariant, 13 nodes)
  ├─ category_key: "thing"
  ├─ category_key: "content-type"
  ├─ ... (11 more)

Entity (org/semantic) ─[:BELONGS_TO]→ EntityCategory (shared/config)
```

**Arc Properties**:
- Name: `BELONGS_TO`
- Family: `ownership`
- Scope: `cross_realm` (org/semantic → shared/config)
- Cardinality: `many_to_one` (many Entities can belong to one category)
- Source: Entity
- Target: EntityCategory

**Benefits**:
1. **Queryable**: Find all entities by category with `MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory {category_key: 'person'})`
2. **Extensible**: Add new categories without code changes (just YAML + Neo4j nodes)
3. **Uniform**: Classification follows ADR-006 (realm differentiates scope) — universal categories in shared, instance relationships in org
4. **Graph-native**: Classification is now part of the knowledge graph, not a buried enum property

**Migration**:
1. Create EntityCategory YAML definition in `packages/core/models/node-kinds/shared/config/entity-category.yaml`
2. Create BELONGS_TO arc definition in `packages/core/models/arc-kinds/ownership/belongs-to.yaml`
3. Generate schema artifacts: `cargo run -- schema generate`
4. Create Neo4j migration to insert 13 EntityCategory nodes and create BELONGS_TO relationships from existing Entity nodes
5. Remove Entity.type property from Entity node definition

**No breaking changes** — API clients can still categorize entities, just through graph traversal instead of property lookup.

## ADR-018: Classification System Refinement

**Status**: Approved (v11.2)

**Decision**: Comprehensive refinement of the NovaNet classification system.

### Changes

| Area | Before | After |
|------|--------|-------|
| **Realm names** | `global`, `tenant` | `shared`, `org` |
| **Trait split** | `derived` (8 nodes) | `generated` (4) + `aggregated` (3) |
| **Job trait** | `job` (3 nodes) | Removed |
| **Container traits** | `knowledge` | `invariant` (6 containers) |
| **Node count** | 65 nodes | 62 nodes |

### Realm Renames

```
BEFORE (confusing):
├── global/config      ← "global" sounds like "everywhere"
└── tenant/config      ← "tenant" is SaaS jargon

AFTER (clear):
├── shared/config      ← Describes WHAT (shared resources)
└── org/config         ← Describes WHO (organization owns it)
```

**Benefits:**
- `shared` = describes purpose (shared resources), not scope
- `org` = familiar terminology (GitHub/Slack orgs), avoids SaaS jargon
- Short: `org` (3 chars) vs `tenant` (6 chars)
- No collision: `Organization` node exists in `org/config`

### Trait Changes

| Node | v11.1 Trait | v11.2 Trait |
|------|-------------|-------------|
| PageGenerated | derived | **generated** |
| BlockGenerated | derived | **generated** |
| OutputArtifact | derived | **generated** |
| PromptArtifact | derived | **generated** |
| GEOAnswer | derived | **aggregated** |
| GEOMetrics | derived | **aggregated** |
| SEOKeywordMetrics | derived | **aggregated** |

**Trait summary (v11.2):**
- `invariant`: 30 nodes (+6 containers)
- `localized`: 2 nodes (EntityContent, ProjectContent)
- `knowledge`: 23 nodes (-6 containers)
- `generated`: 4 nodes (LLM output)
- `aggregated`: 3 nodes (computed metrics)
- ~~`job`~~: REMOVED (0 nodes)

### Removed Nodes (Job Concept)

```
DELETED:
- GenerationJob    (org/output)
- EvaluationSignal (org/output)
- SEOMiningRun     (org/seo)
```

**Rationale**: Job concept deferred until generation pipeline is more mature. Clean architecture now, add workflow nodes in v12+ when needed.

### Container Traits

Containers changed from `knowledge` to `invariant`:

```
TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
```

**Rationale:**
- Containers are universal categories (pricing, legal, technical) → exist in ALL locales
- Atoms remain `knowledge` → locale-specific content

### Architecture Summary (v11.2)

```
REALMS (62 nodes total):
├── shared/              # Universal locale knowledge (READ-ONLY) — 32 nodes
│   ├── config/          # 14 nodes (incl. EntityCategory)
│   └── locale-knowledge/# 18 nodes (6 containers + 12 atoms)
│
└── org/                 # Organization-specific content — 30 nodes
    ├── config/          # 2 nodes (Organization, Tenant)
    ├── foundation/      # 3 nodes
    ├── structure/       # 3 nodes
    ├── semantic/        # 4 nodes
    ├── seo/             # 8 nodes (SEOMiningRun removed)
    ├── instruction/     # 7 nodes
    └── output/          # 3 nodes (job nodes removed)
```

**Migration**:
- Directory renames: `node-kinds/global/` → `shared/`, `tenant/` → `org/`
- YAML realm field updates: 65 files
- Rust code: 250+ occurrences
- TypeScript code: 80+ occurrences
- Test assertions: 20+ updates

## ADR-019: Layer Reorganization

**Status**: Approved (v11.3)

**Problem**: The `locale-knowledge` layer in v11.2 conflated three distinct concerns:
1. BCP-47 locale configuration (Locale, LocaleVoice, LocaleGrammar)
2. Geographic data (Region, Country, GeoFeature)
3. Semantic knowledge atoms (Terms, Expressions, Patterns)

This made queries ambiguous ("knowledge in locale-knowledge" is confusing) and prevented clean layer-based filtering in Studio.

**Decision**: Reorganize the layer structure for better semantic clarity.

### Changes

| Area | Before (v11.2) | After (v11.3) |
|------|----------------|---------------|
| **Shared layers** | 2 (config, locale-knowledge) | 3 (locale, geography, knowledge) |
| **Org layers** | 7 | 8 (+geo) |
| **Total layers** | 9 | 11 |
| **Org config** | Organization + Tenant (2 nodes) | OrgConfig (1 node) |
| **Total nodes** | 62 | 61 |

### Layer Split: locale-knowledge → 3 layers

```
BEFORE (v11.2):
├── shared/
│   ├── config/              # 14 nodes
│   └── locale-knowledge/    # 18 nodes (mixed concerns)

AFTER (v11.3):
├── shared/
│   ├── locale/              # 7 nodes (Locale, LocaleVoice, LocaleGrammar, etc.)
│   ├── geography/           # 6 nodes (Region, Country, GeoFeature, etc.)
│   └── knowledge/           # 19 nodes (TermSet, Term, CultureSet, etc.)
```

**Rationale:**
- `locale-knowledge` mixed locale configuration with geographic data and semantic knowledge
- Split into 3 focused layers with clearer purposes:
  - `locale`: BCP-47 locale configuration (voice, grammar, formatting)
  - `geography`: Geographic entities (regions, countries, features)
  - `knowledge`: Semantic atoms (terms, expressions, patterns, culture)

### New Layer: geo (org realm)

```
BEFORE (v11.2):
└── org/
    ├── seo/                 # 8 nodes (SEO + GEO mixed)

AFTER (v11.3):
└── org/
    ├── seo/                 # 5 nodes (SEO only)
    └── geo/                 # 3 nodes (GEOAnswer, GEOMetrics, GEOQuery)
```

**Rationale:**
- GEO (AI search optimization) and SEO (search engine optimization) are distinct disciplines
- Separate layers enable clearer queries and filtering
- GEO nodes have trait `aggregated`, SEO nodes have mixed traits

### Node Merge: Organization + Tenant → OrgConfig

```
BEFORE (v11.2):
└── org/
    └── config/              # Organization, Tenant (2 nodes)

AFTER (v11.3):
└── org/
    └── config/              # OrgConfig (1 node)
```

**Rationale:**
- Organization and Tenant were redundant in 2-realm architecture
- Single OrgConfig node holds all org-level configuration
- Simplifies config layer to single entry point

### Architecture Summary (v11.3)

```
REALMS (61 nodes total):
├── shared/              # Universal locale knowledge (READ-ONLY) — 32 nodes
│   ├── locale/          # 7 nodes (Locale, LocaleVoice, LocaleGrammar, LocaleFormats, etc.)
│   ├── geography/       # 6 nodes (Region, Country, GeoFeature, GeoZone, etc.)
│   └── knowledge/       # 19 nodes (CategorySet, EntityCategory, TermSet, Term, etc.)
│
└── org/                 # Organization-specific content — 29 nodes
    ├── config/          # 1 node (OrgConfig)
    ├── foundation/      # 3 nodes (Project, ProjectContent, Brand)
    ├── structure/       # 3 nodes (Page, PageType, Block, BlockType)
    ├── semantic/        # 4 nodes (Entity, EntityContent, Thing, Category)
    ├── instruction/     # 7 nodes (PagePrompt, BlockPrompt, SEOPrompt, etc.)
    ├── seo/             # 5 nodes (SEOKeyword, SEOKeywordMetrics, SEOCluster, etc.)
    ├── geo/             # 3 nodes (GEOQuery, GEOAnswer, GEOMetrics)
    └── output/          # 3 nodes (PageGenerated, BlockGenerated, OutputArtifact)
```

### Migration

1. **Directory restructure**:
   ```bash
   mv shared/locale-knowledge/ → split into locale/, geography/, knowledge/
   ```

2. **YAML layer field updates**: 32 files in shared realm

3. **New geo layer**: Move GEO* nodes from seo/ to geo/

4. **Node merge**:
   - Delete `organization.yaml` and `tenant.yaml`
   - Create `org-config.yaml`
   - Update arcs referencing Organization/Tenant

5. **Regenerate artifacts**: `cargo run -- schema generate`

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
| 012 | v10.6 | 2-Realm Architecture (updated v11.5: 10 layers) |
| 013 | v10.6 | Icons source of truth |
| 014 | v10.9 | Naming convention refactor (L10n to Content/Generated) |
| 015 | v10.9 | Unidirectional ownership arcs |
| 016 | v10.9 | Type-constrained container arcs |
| 017 | v11.1 | EntityCategory classification |
| 018 | v11.2 | Classification system refinement (realm renames, trait split) |
| 019 | v11.3 | Layer reorganization (locale-knowledge split, geo layer, OrgConfig) |
| 020 | v11.5 | Schema refinement (Locale to config, SEO/GEO consolidation) |
| 021 | v11.6 | Query-First Architecture (Cypher as source of truth) |
| 022 | v11.7 | Unified Tree Architecture (everything is a node, 2 modes) |
| 023 | v11.8 | Class/Instance Terminology + Meta Elimination (Kind→Class, Meta→Schema) |
| 024 | v11.8 | Trait Redefinition as "Data Origin" (defined/authored/imported/generated/retrieved) |

## ADR-020: Schema Refinement

**Status**: Approved (v11.5)

**Problem**: Two architectural issues emerged from v11.3/v11.4 usage:
1. **Locale misplacement**: `Locale` was in `shared/locale` layer with settings nodes (Style, Formatting), but Locale is a *definition* (invariant trait), not a *setting* (knowledge trait). This caused trait inconsistency.
2. **SEO/GEO redundancy**: SEO/GEO nodes in `org` realm duplicated knowledge that should be universal across organizations. An SEO keyword like "QR code generator" is the same regardless of organization.

**Decision**: Refine the schema with Locale moved to shared/config and SEO/GEO layers consolidated.

### Changes

| Area | Before (v11.4) | After (v11.5) |
|------|----------------|---------------|
| **Locale location** | shared/locale | shared/config |
| **SEO/GEO layers** | org/seo, org/geo (separate) | Removed from org |
| **SEO/GEO nodes** | In org realm | Moved to shared/knowledge |
| **Layer count** | 11 (3 shared + 8 org) | 10 (4 shared + 6 org) |
| **Node distribution** | 32 shared + 29 org | **39 shared + 21 org = 60 nodes** |

### Locale to Config Layer

```
BEFORE (v11.4):
├── shared/
│   ├── config/          # EntityCategory only
│   └── locale/          # Locale, Style, Formatting, etc.

AFTER (v11.5):
├── shared/
│   ├── config/          # EntityCategory + Locale (definitions)
│   └── locale/          # Style, Formatting, etc. (settings)
```

**Rationale:**
- Locale is a DEFINITION (invariant), not a parameter/setting
- Follows EntityCategory pattern: definitions go in config layer
- shared/locale now contains only locale SETTINGS (Culture, Style, etc.)
- Clean separation: definitions vs settings

### SEO/GEO Consolidation

```
BEFORE (v11.4):
└── org/
    ├── seo/             # 5 nodes (SEOKeyword, etc.)
    └── geo/             # 3 nodes (GEOQuery, etc.)

AFTER (v11.5):
├── shared/
│   └── knowledge/       # Includes SEO + GEO nodes (now 26 nodes)
└── org/
    # No seo/geo layers
```

**Rationale:**
- SEO/GEO data is universal market intelligence, not org-specific
- Moving to shared realm enables cross-org analytics
- Reduces org layers from 8 to 6
- Knowledge layer becomes the home for all market intelligence

### Architecture Summary (v11.5)

```
REALMS (60 nodes total):
├── shared/              # Universal knowledge (READ-ONLY) — 39 nodes
│   ├── config/          # 3 nodes (EntityCategory, Locale, SEOKeywordFormat)
│   ├── locale/          # 6 nodes (Culture, Style, Formatting, etc.)
│   ├── geography/       # 6 nodes (Continent, Region, etc.)
│   └── knowledge/       # 24 nodes (Terms, Expressions, SEO, GEO, etc.)
│
└── org/                 # Organization-specific — 21 nodes
    ├── config/          # 1 node (OrgConfig)
    ├── foundation/      # 3 nodes (Project, ProjectContent, BrandIdentity)
    ├── structure/       # 3 nodes (Page, Block, ContentSlot)
    ├── semantic/        # 4 nodes (Entity, EntityContent, AudiencePersona, ChannelSurface)
    ├── instruction/     # 7 nodes (PageType, BlockType, prompts, etc.)
    └── output/          # 3 nodes (PageGenerated, BlockGenerated, OutputArtifact)
```

## ADR-021: Query-First Architecture

**Status**: Approved (v11.6)

**Problem**: NovaNet Studio had multiple sources of truth for graph visualization:
1. Hardcoded queries in `viewQueries.ts`
2. YAML view definitions in `packages/core/models/views/`
3. Ad-hoc Cypher queries from QueryPill
4. Mode-specific logic (data/meta/overlay) scattered across components

This caused:
- Inconsistent behavior between 2D and 3D views
- Difficulty understanding "what query produced this graph?"
- Duplicate query definitions (TypeScript + YAML)
- Complex state management across viewStore, queryStore, graphStore

**Decision**: Adopt **Query-First Architecture** where Cypher is the single source of truth.

### Core Principles

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  QUERY-FIRST ARCHITECTURE                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. CYPHER QUERY = SOURCE OF TRUTH                                          │
│     └─ Graph always displays the result of the executed Cypher query        │
│     └─ No hidden state or mode-specific filtering                           │
│     └─ QueryPill shows the exact query that produced visible graph          │
│                                                                             │
│  2. YAML VIEWS = SINGLE DEFINITION SOURCE                                   │
│     └─ All views defined in packages/core/models/views/*.yaml               │
│     └─ No hardcoded queries in TypeScript                                   │
│     └─ Views are parameterized Cypher templates                             │
│                                                                             │
│  3. AUTO-EXECUTE WITH EDIT OPTION                                           │
│     └─ Click view → execute immediately → update graph                      │
│     └─ Ctrl+click → load query into QueryPill without executing             │
│     └─ Edit QueryPill → click ▶️ to run modified query                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ ViewPicker  │───▶│  viewStore  │───▶│ /api/views  │───▶│   Neo4j     │
│ (Select)    │    │ executeView │    │ /:id/query  │    │  (Cypher)   │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                  │                  │                  │
       ▼                  ▼                  ▼                  ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  QueryPill  │◀───│ queryStore  │◀───│    YAML     │◀───│   Results   │
│ (Display)   │    │ setQuery()  │    │   cypher    │    │ nodes/edges │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### META Mode: KINDS_QUERY + ARCS_QUERY

META mode uses two foundational queries to build the meta-graph:

```cypher
// KINDS_QUERY: Fetch all NodeKind instances
MATCH (k:Kind)
RETURN k.name AS name, k.realm AS realm, k.layer AS layer,
       k.trait AS trait, k.display_name AS display_name

// ARCS_QUERY: Fetch all ArcKind instances
MATCH (a:ArcKind)
RETURN a.name AS name, a.family AS family, a.scope AS scope,
       a.cardinality AS cardinality, a.source AS source, a.target AS target
```

These queries are executed by `cargo run -- meta` and populate the meta-graph for schema exploration.

### View Categories

| Category | Purpose | Example Views |
|----------|---------|---------------|
| `global` | Full graph exploration | complete-graph, shared-layer, project-layer |
| `contextual` | Node-specific subgraph | composition, knowledge, geographic |
| `generation` | AI agent context | block-generation, page-generation-context |
| `mining` | SEO/GEO intelligence | seo-intel, geo-intel |

### YAML View Schema

```yaml
id: composition
description: Page/Block composition hierarchy
category: contextual
contextual: true
applicable_types: [Page, Block]
modes: [data, meta, overlay, query]
cypher: |
  MATCH (root {key: $nodeKey})
  WHERE root:Page OR root:Block
  OPTIONAL MATCH path = (root)-[:HAS_BLOCK*1..3]->(block:Block)
  WITH root, collect(DISTINCT block) AS blocks
  UNWIND ([root] + blocks) AS n
  WITH collect(DISTINCT n) AS nodes
  UNWIND nodes AS n
  OPTIONAL MATCH (n)-[r:HAS_BLOCK]->(m)
  WHERE m IN nodes
  RETURN nodes, collect(DISTINCT r) AS relationships
```

### Benefits

1. **Debuggability**: QueryPill shows exact query → easy to understand/modify
2. **Consistency**: 2D and 3D views show identical data (same query results)
3. **Extensibility**: Add views by creating YAML files, no code changes
4. **Transparency**: No hidden mode logic, query is the complete specification
5. **Testability**: Views are pure Cypher, testable independently

### Impact

- `viewQueries.ts` deprecated (moved to YAML)
- ViewPicker loads from `_registry.yaml` on mount
- QueryPill displays active view badge
- All navigation modes (data/meta/overlay/query) use same view system

**Reference**: `docs/plans/2026-02-10-query-first-architecture-design.md`

## ADR-022: Unified Tree Architecture

**Status**: Approved (v11.7)

**Problem**: NovaNet had inconsistent behavior between Neo4j and UI:
1. Realm, Layer, Trait ARE nodes in Neo4j (`:Meta:Realm`, `:Meta:Layer`)
2. But TUI/Studio treated them as visual groupings, not clickable nodes
3. 5 separate modes (Meta/Data/Overlay/Query/Atlas) created confusion
4. Emoji icons in code instead of proper icon system

**Decision**: Unify into single tree where everything is a clickable node.

**Changes**:
- 5 modes → 2 modes: `[1]Graph` (unified tree) + `[2]Nexus` (hub)
- Realm, Layer, ArcFamily, ArcKind are clickable nodes with detail panels
- Kind nodes expand to show instances (lazy loading, 10 + "load more")
- Dual icons: `{ web: "lucide-name", terminal: "◆" }` - no emoji
- Atlas removed, Audit moved to Nexus hub

**Principle**: "If it's a node in Neo4j, it's a node everywhere"

**Consequences**:
- Neo4j migration needed (HAS_LAYER, HAS_KIND, BELONGS_TO_FAMILY arcs)
- Types defined before generators
- Backward compatibility shim for old nav modes
- Performance optimization for large instance counts (200K+)

**Files affected**:
- TUI: `tools/novanet/src/tui/{app,data,ui,theme}.rs`
- Studio: `apps/studio/src/components/graph/`, stores
- YAML: `visual-encoding.yaml`, `views/_registry.yaml`

### Header Simplification

```
BEFORE (v11.6): [1]Meta [2]Data [3]Overlay [4]Query [5]Atlas
AFTER (v11.7):  [1]Graph [2]Nexus
```

### Changes Table

| Aspect | Before | After |
|--------|--------|-------|
| Modes | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Realm/Layer | Visual groupings (folders) | Clickable nodes |
| Instances | Hidden or separate Data mode | Expandable under Kind |
| Search | Separate Query mode | `[/]` overlay in Graph |
| Atlas | Separate mode | Removed |
| Audit | In Atlas | In Nexus hub |
| Icons | Mixed emoji | Unicode only (no emoji) |

### Unified Tree Structure

```
▼ Nodes (60)
  ▼ ◉ Realm:shared           ← Clickable :Meta:Realm node
    ▼ ⚙ Layer:config         ← Clickable :Meta:Layer node
      ▼ ◆ Kind:Locale [200]  ← Clickable :Meta:Kind node
        ● Locale:fr-FR       ← Clickable :Locale instance
        ● Locale:en-US
▼ Arcs (114)
  ▼ → ArcFamily:ownership
    → ArcKind:HAS_PROJECT
```

### Nexus Hub

```
[2]Nexus
├── Quiz    — Test NovaNet knowledge
├── Audit   — Validate schema consistency
├── Stats   — Dashboard with graph metrics
└── Help    — Keybindings and documentation
```

### Color Architecture

Single source of truth for colors:
- `taxonomy.yaml` = DEFINES colors (hex values)
- `visual-encoding.yaml` = USES colors (no hex, references taxonomy)
- TUI + Studio = same colors from taxonomy.yaml

### UI Elements Preserved

From current TUI (keep these):
- Trait icons: `■(inv)` `□(loc)` `◇(kno)` `★(gen)` `⋆(agg)`
- Arc counts: `→N` (outgoing) `←N` (incoming)
- Property counts: `⊞required/total`
- Instance counts: `Kind (N)`, `Layer (N)`
- Colored badges: `●org` `◎shd` `◆sem` etc.
- Layer headers with kind count: `◇3`

**Reference**: `docs/plans/2026-02-11-unified-tree-design.md`

## ADR-023: Class/Instance Terminology

**Status**: Approved (v11.8)

**Problem**: Two terminology issues caused confusion:

1. **"Kind" was non-standard**: Not graph theory or ontology terminology. LLMs have less training data on "Kind" vs "Class". French "Genre" was awkward.

2. **"Meta" was ambiguous**: Facebook collision (Meta company), Spanish "meta" means "goal", too abstract for humans, mixed usage across "Meta Node", "KindMeta", Neo4j `:Meta:` labels.

**Decision**:
- Rename schema-level terminology from "Kind" to "Class"
- Data-level stays "Instance"
- **ELIMINATE "Meta" prefix/suffix entirely** - use semantic names instead

**Changes**:

| Before | After | Context |
|--------|-------|---------|
| **Kind → Class** | | |
| NodeKind | NodeClass | Rust/TypeScript struct |
| ArcKind | ArcClass | Rust/TypeScript struct |
| KindInfo | ClassInfo | TUI struct |
| TreeItem::Kind | TreeItem::Class | Rust enum variant |
| [:FROM_KIND] | [:FROM_CLASS] | Neo4j relationship |
| [:TO_KIND] | [:TO_CLASS] | Neo4j relationship |
| [:HAS_KIND] | [:HAS_CLASS] | Neo4j relationship |
| "Node Kinds" | "Classes" | UI label |
| **Meta → Semantic Names** | | |
| KindMeta | Classification | TypeScript interface (realm/layer/trait axes) |
| KIND_META | CLASS_TAXONOMY | TypeScript constant |
| :Meta:Kind | :Schema:Class | Neo4j label (Meta→Schema) |
| :Meta:ArcKind | :Schema:ArcClass | Neo4j label |
| "Meta Node" | "Class" | Glossary term |
| "Data Node" | "Instance" | Glossary term |
| "Meta mode" | "Schema view" | Studio ViewPicker |
| "Data mode" | "Graph view" | Studio ViewPicker |

**Rationale**:

**Class/Instance:**
1. **LLM Semantic Clarity**: `rdfs:Class`, `owl:Class` are in LLM training data millions of times. "Class/Instance" is THE canonical OOP and ontology pairing.
2. **Ontology Standard**: RDF Schema and OWL use "Class" for schema-level definitions. NovaNet is a knowledge graph - aligning with semantic web standards improves interoperability.
3. **Universal Understanding**: Every programmer knows Class/Instance from OOP. Non-programmers understand "a class of things" from everyday English.
4. **Internationalization**: "Classe/Instance" (French), "Clase/Instancia" (Spanish), "Klasse/Instanz" (German), クラス/インスタンス (Japanese) - perfect cognates.

**Meta Elimination:**
5. **Semantic names > abstract labels**: `Classification` describes WHAT it contains (realm/layer/trait axes). `Schema` describes WHAT it is (the schema, not data). "Meta" described NOTHING.
6. **No collisions**: Avoids Facebook "Meta" confusion in searches and discussions.
7. **International clarity**: "Schema" and "Classification" are universal technical terms. "Meta" has different meanings (Spanish "meta" = goal, Greek μετά = after).
8. **Consistency**: Single terminology change instead of half-measures. No more `:Meta:` labels in Neo4j, no more `*Meta` suffixes in code.

**Impact**:

| Zone | Files | Changes | Effort |
|------|-------|---------|--------|
| Rust | 43 | 721 | 4-8h |
| TypeScript | 19 | 93+ | 2-4h |
| TUI/Nexus | 20+ | 1,299 | 3-5h |
| Documentation | 14 | ~50 | 1-2h |
| Studio | 8 | ~30 | 2-3h |
| Neo4j Migration | - | Schema | 1h |

**Migration**: Requires coordinated update across Rust, TypeScript, Neo4j, and documentation. Neo4j schema migration must happen first or synchronously with code changes.

**Reference**: Brainstorming session 2026-02-12, devil's advocate analysis comparing 15 terminology options.

## ADR-024: Trait Redefinition as "Data Origin"

**Status**: Approved (v11.8)

**Problem**: Current trait system is NOT orthogonal to Layer:

1. **60% redundancy**: Most layers have single trait (instruction=invariant, output=generated)
2. **Name collision**: "knowledge" trait vs "knowledge" layer
3. **Catch-all**: 31 nodes are "invariant" but serve very different purposes
4. **Mixed semantics**: Traits mix "locale behavior" with "data origin"

Analysis by 5 brainstorming agents revealed Layer already answers "WHAT functional category?" - Trait should answer a DIFFERENT question.

**Decision**: Redefine Trait as "Data Origin" (WHERE does data come from?):

| Before | After | Definition |
|--------|-------|------------|
| invariant | **defined** | Defined by human, created ONCE. Structure/template. |
| localized | **authored** | Written by human, PER locale. Editorial content. |
| knowledge | **imported** | External data brought in. APIs, databases, corpora. |
| generated | **generated** | Produced by OUR LLM. NovaNet generates this. |
| aggregated | **retrieved** | Retrieved from EXTERNAL APIs. Snapshots of third-party data. |

**True Orthogonality**:

```
LAYER answers:  "WHAT functional category?"
                config, structure, semantic, instruction, output, knowledge...

TRAIT answers:  "WHERE does the data come from?"
                defined, authored, imported, generated, retrieved
```

**Node Distribution**:

| Trait | Count | Examples |
|-------|-------|----------|
| defined | 31 | Page, Block, Entity, PageType, PagePrompt, Locale, OrgConfig |
| imported | 22 | Term, Expression, Pattern, Culture, SEOKeyword, GEOQuery |
| authored | 2 | EntityContent, ProjectContent |
| generated | 5 | PageGenerated, BlockGenerated, OutputArtifact, PromptArtifact |
| retrieved | 3 | GEOAnswer, SEOKeywordMetrics, GEOMetrics |

**Key Clarification - GEOAnswer**:
- GEOAnswer is `retrieved`, NOT `generated`
- It's a SNAPSHOT of what Claude/GPT/Perplexity returned
- We RETRIEVED it from their API, we didn't generate it
- It's evidence of how AI engines see our content

**Rationale**:

1. **defined**: Human creates once, doesn't vary by locale
2. **authored**: Human writes content, per locale (editorial)
3. **imported**: Data brought in from external sources
4. **generated**: Our LLM produces output
5. **retrieved**: Fetched from third-party APIs (we capture, not create)

**Research**: Drupal Config/Content Entity, Sanity Document/Object, OWL TBox/ABox, Neo4j labeling patterns

**Reference**: 5-agent analysis, `docs/plans/2026-02-13-nomenclature-v118-design.md`

## ADR-025: Instruction Layer Renaming

**Status**: Approved (v11.8)

**Problem**: Current instruction layer names don't reflect their function:

- `PageType` → actually defines page STRUCTURE (JSON with headers, sections)
- `BlockType` → defines block JSON schema (this one is OK)
- `PagePrompt` → actually contains page INSTRUCTIONS (markdown with @ refs)
- `BlockPrompt` → contains block INSTRUCTIONS (markdown with @ refs)

The existing Studio UI already uses the correct names: "Page Structures" and "Page Instructions".

**Decision**: Rename to match function and existing UI:

| Before | After | Function |
|--------|-------|----------|
| PageType | **PageStructure** | JSON defining which BlockTypes in what order |
| BlockType | **BlockType** | (keep) JSON schema for a block |
| PagePrompt | **PageInstruction** | Markdown with LLM directives and @ references |
| BlockPrompt | **BlockInstruction** | Markdown with LLM directives and @ references |

**Arc Changes**:

| Before | After |
|--------|-------|
| `[:OF_TYPE]` (Page→PageType) | `[:HAS_STRUCTURE]` (Page→PageStructure) |
| `[:HAS_PROMPT]` (Page→PagePrompt) | `[:HAS_INSTRUCTION]` (Page→PageInstruction) |
| `[:OF_TYPE]` (Block→BlockType) | `[:OF_TYPE]` (keep - BlockType unchanged) |
| `[:HAS_PROMPT]` (Block→BlockPrompt) | `[:HAS_INSTRUCTION]` (Block→BlockInstruction) |

**@ Reference System**:

Instructions support @ references that resolve at generation time:

```markdown
# PageInstruction example
Generate pricing page comparing @entity:tier-pro vs @entity:tier-basic
See @page:features for product consistency

# BlockInstruction example
[TRANSLATE] title: Highlight benefits of @entity:tier-pro
[FIXED] cta_url: /signup
```

At generation time:
- `@entity:tier-pro` → loads `EntityContent(tier-pro@{locale})`
- `@page:features` → loads `Page(features)` context
- `[TRANSLATE]` → field needs locale-native generation
- `[FIXED]` → field is invariant (URLs, technical values)

**Pipeline**:

```
Page
├── [:HAS_STRUCTURE] → PageStructure
│   └── JSON compilé depuis l'ordre des blocks:
│       { "blocks": ["hero", "features", "pricing", "cta"] }
│
├── [:HAS_INSTRUCTION] → PageInstruction
│   └── Markdown compilé depuis BlockInstructions (dans l'ordre):
│       BlockInstruction₁ + BlockInstruction₂ + ... = PageInstruction
│
└── [:HAS_BLOCK {order: N}] → Block (l'ordre est sur l'arc!)
    │
    ├── [:OF_TYPE] → BlockType
    │   └── JSON Schema: { slug, title, description, cta_url, ... }
    │
    └── [:HAS_INSTRUCTION] → BlockInstruction
        └── Markdown avec @ références
```

**L'ordre des blocs** (propriété `order` sur [:HAS_BLOCK]) détermine:
1. **PageStructure JSON** — L'ordre des BlockTypes
2. **PageInstruction** — La compilation séquentielle des BlockInstructions
3. **PageGenerated** — L'ordre final du contenu généré

**Rationale**:

1. **PageStructure**: Describes WHAT it is (the structure combining blocks)
2. **PageInstruction**: Describes WHAT it is (instructions for LLM)
3. **BlockType**: Already correct (defines the type/schema of a block)
4. **BlockInstruction**: Consistent with PageInstruction
5. **Aligned with UI**: The existing Studio UI uses these exact names

**Reference**: `docs/plans/2026-02-13-nomenclature-v118-design.md`

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
| 012 | v10.6 | 2-Realm Architecture (updated v11.5: 10 layers) |
| 013 | v10.6 | Icons source of truth |
| 014 | v10.9 | Naming convention refactor (L10n to Content/Generated) |
| 015 | v10.9 | Unidirectional ownership arcs |
| 016 | v10.9 | Type-constrained container arcs |
| 017 | v11.1 | EntityCategory classification |
| 018 | v11.2 | Classification system refinement (realm renames, trait split) |
| 019 | v11.3 | Layer reorganization (locale-knowledge split, geo layer, OrgConfig) |
| 020 | v11.5 | Schema refinement (Locale to config, SEO/GEO consolidation) |
| 021 | v11.6 | Query-First Architecture (Cypher as source of truth) |
| 022 | v11.7 | Unified Tree Architecture (everything is a node, 2 modes) |
| 023 | v11.8 | Class/Instance Terminology + Meta Elimination (Kind→Class, Meta→Schema) |
| 024 | v11.8 | Trait Redefinition as "Data Origin" (defined/authored/imported/generated/retrieved) |
| 025 | v11.8 | Instruction Layer Renaming (PageType→PageStructure, PagePrompt→PageInstruction) |

## References

- `docs/plans/2026-02-03-nomenclature-v95-design.md` — Full v9.5 design
- `docs/plans/2026-02-03-v10-brainstorm-decisions.md` — v10 roadmap decisions
- `docs/plans/2026-02-01-ontology-v9-design.md` — Original v9 design
- `docs/plans/2026-02-10-query-first-architecture-design.md` — Query-First Architecture design
- `docs/plans/2026-02-11-unified-tree-design.md` — Unified Tree Architecture design
- `docs/plans/2026-02-13-nomenclature-v118-design.md` — Nomenclature v11.8 (Class/Instance, Meta elimination, Trait renaming)
