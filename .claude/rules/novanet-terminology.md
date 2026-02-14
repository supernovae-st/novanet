# NovaNet Terminology (v0.12.5)

This file defines the canonical terminology for NovaNet. All code, documentation, and UI must use these terms consistently.

## Core Vocabulary

### Graph Elements

| Level | Vertex | Edge |
|-------|--------|------|
| **General** | Node | **Arc** |
| **Instance (data)** | NodeData | ArcData |
| **Class (schema)** | NodeClass | ArcClass |

> **v0.12.0 Change**: "Kind" → "Class", "Meta" eliminated. See ADR-023.

> **CRITICAL**: We use "Arc" (not "Edge" or "Relation") for directed links between nodes.
> This aligns with graph theory terminology for directed graphs.
> Exception: React Flow uses "Edge" internally — that's acceptable in React Flow-specific code.

### Node Classification (Faceted)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | WHERE? | `NodeRealm` | `realm` | `shared`, `org` |
| 2 | WHAT? | `NodeLayer` | `layer` | `config`, `locale`, `geography`, `knowledge`, `foundation`, `structure`, `semantic`, `instruction`, `output` |
| 3 | HOW? | `NodeTrait` | `trait` | `defined`, `authored`, `imported`, `generated`, `retrieved` |

> **v0.12.0 Trait Redefinition (ADR-024)**: Trait now answers "WHERE does data come from?" (Data Origin)
> - `defined` = Human-created once (templates, configs) — was `invariant`
> - `authored` = Human-written per locale (editorial content) — was `localized`
> - `imported` = External data brought in (corpora, SEO keywords) — was `knowledge`
> - `generated` = Produced by NovaNet LLM (unchanged)
> - `retrieved` = Fetched from external APIs (GEO snapshots) — was `aggregated`

### v11.5 Realm Architecture

| Realm | Layers | Nodes | Description |
|-------|--------|-------|-------------|
| `shared` | config, locale, geography, knowledge | 40 | Universal knowledge (READ-ONLY) |
| `org` | config, foundation, structure, semantic, instruction, output | 21 | Organization-specific content |

> **v0.12.4 Changes:**
> - Brand Architecture: Brand, BrandDesign, BrandPrinciples, PromptStyle nodes added (ADR-028)
> - Country node added to shared/geography
> - Total: **61 nodes** (40 shared + 21 org), **128 arcs**, **10 layers** (4 shared + 6 org)

### v11.7 Unified Tree Architecture

| Change | Before (v11.6) | After (v11.7) |
|--------|----------------|---------------|
| Nav modes | 5 (Meta/Data/Overlay/Query/Atlas) | 2 (Graph/Nexus) |
| Realm/Layer | Visual groupings | Clickable nodes |
| Instances | Hidden | Under Class, expandable |
| Icons | Mixed emoji | Dual: Lucide (web) + Unicode (terminal) |

> **v11.7 Principle**: "If it's a node in Neo4j, it's a node everywhere"
> - Realm nodes: clickable, show properties in detail panel
> - Layer nodes: clickable, show HAS_LAYER relationships
> - ArcFamily/ArcClass nodes: clickable with `[fam]`/`[arc]` badges

> **v11.3 Changes:**
> - Layer split: `locale-knowledge` → `locale`, `geography`, `knowledge` (3 layers)
> - New layer: `geo` added to org realm for GEO intelligence
> - Node merge: Organization + Tenant → OrgConfig

> **v11.2 Changes:**
> - Realm renames: `global` → `shared`, `tenant` → `org`
> - Trait split: `derived` → `generated` + `aggregated`, `job` removed
> - 3 job nodes removed (GenerationJob, SEOMiningRun, EvaluationSignal)

### Arc Classification (Faceted)

| Axis | Question | Type | Property | Values |
|------|----------|------|----------|--------|
| 1 | SCOPE? | `ArcScope` | `scope` | `intra_realm`, `cross_realm` |
| 2 | FUNCTION? | `ArcFamily` | `family` | `ownership`, `localization`, `semantic`, `generation`, `mining` |
| 3 | MULTIPLICITY? | `ArcCardinality` | `cardinality` | `zero_to_one`, `one_to_one`, `one_to_many`, `many_to_many` |

## YAML Source Files (v0.12.4)

| File | Content |
|------|---------|
| `taxonomy.yaml` | Realm/Layer/Trait/ArcFamily/ArcScope definitions (v0.12.4: 2 realms, 10 layers, 5 traits) |
| `node-classes/shared/` | 40 NodeClass definitions in shared realm (config: 3, locale: 6, geography: 7, knowledge: 24) |
| `node-classes/org/` | 21 NodeClass definitions in org realm (config: 1, foundation: 6, structure: 3, semantic: 4, instruction: 4, output: 3) |
| `arc-classes/` | 1 file per ArcClass, organized by ArcFamily |
| `relations.yaml` | Legacy format (deprecated, kept for parser compatibility) |

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| NodeClass YAML | `kebab-case.yaml` | `locale-voice.yaml`, `entity-native.yaml`, `page-native.yaml` |
| ArcClass YAML | `kebab-case.yaml` | `has-page.yaml`, `has-native.yaml`, `uses-entity.yaml` |
| TypeScript types | `PascalCase` | `NodeClass`, `ArcFamily`, `NodeRealm` |
| TypeScript files | `kebab-case.ts` | `arc-classes.ts`, `node-layers.ts` |
| Rust structs | `PascalCase` | `ArcClass`, `NodeRealm` |
| Rust files | `snake_case.rs` | `arc_schema.rs`, `taxonomy.rs` |

## Node Naming Convention (v0.12.5)

> **RULE: Suffix indicates trait and relationship to parent defined node**

| Pattern | Trait | Layer | When to Use | Example |
|---------|-------|-------|-------------|---------|
| `FooNative` | authored/generated | semantic/output | **v0.12.5**: Locale-specific content (written OR generated natively) | `EntityNative`, `PageNative` |
| `FooContent` | authored | semantic | **DEPRECATED v0.12.5**: Use `FooNative` instead | `EntityContent` → `EntityNative` |
| `FooGenerated` | generated | output | **DEPRECATED v0.12.5**: Use `FooNative` instead | `PageGenerated` → `PageNative` |
| `FooCategory` | defined | config | Categorical grouping for defined `Foo` | `EntityCategory` |
| `FooSet` | defined | knowledge | Container grouping related atoms | `TermSet`, `SEOKeywordSet`, `GEOQuerySet` |
| `Foo` | varies | varies | Node is standalone (no parent defined) | `SEOKeyword`, `Term`, `Expression` |

> **v0.12.5 *Native Pattern (ADR-029)**: Unified suffix for ALL locale-specific nodes
> - `*Native` suffix indicates content that exists per-locale (natively written OR generated)
> - Both `authored` and `generated` traits use the same suffix — the trait indicates WHO creates
> - Simplifies the model: one pattern instead of two (`*Content` + `*Generated`)

**v0.12.5 Changes (ADR-029 *Native Pattern + ADR-030 Slug Ownership):**
- `EntityContent` → `EntityNative` (semantic layer, authored trait)
- `PageGenerated` → `PageNative` (output layer, generated trait)
- `BlockGenerated` → `BlockNative` (output layer, generated trait)
- `ProjectContent` → `ProjectNative` (foundation layer, authored trait)
- `HAS_CONTENT` + `HAS_GENERATED` → unified `HAS_NATIVE` (with `locale` property)
- Slug properties (`slug`, `full_path`) moved from EntityNative to PageNative
- Entity.key = semantic identifier, Page.slug = URL segment (can differ)

**v0.12.0 Changes (ADR-024 Data Origin):**
- `invariant` → `defined` (human-created once)
- `localized` → `authored` (human-written per locale)
- `knowledge` → `imported` (external data brought in)
- `generated` → `generated` (unchanged, LLM output)
- `aggregated` → `retrieved` (fetched from external APIs)

**v0.12.4 Changes:**
- Brand Architecture: Brand, BrandDesign, BrandPrinciples, PromptStyle (ADR-028)
- Country node added to shared/geography
- 10 layers total (4 shared + 6 org), 61 nodes (40 shared + 21 org), 128 arcs

**v11.2 Changes:**
- Trait `derived` split into `generated` (LLM output) and `aggregated` (computed metrics)
- `job` trait removed (3 nodes deleted)
- Realms renamed: `global` → `shared`, `tenant` → `org`

**v11.1 Changes:**
- `EntityCategory` added (shared/knowledge layer, defined trait, categorical grouping)
- `BELONGS_TO` arc added (Entity → EntityCategory, ownership family)

**v10.9 Changes:**
- `EntityL10n` renamed to `EntityContent` (semantic layer, authored trait)
- `PageL10n` renamed to `PageGenerated` (output layer, generated trait)
- `BlockL10n` renamed to `BlockGenerated` (output layer, generated trait)

**Arc Changes (v10.9):**
- `HAS_L10N` renamed to `HAS_CONTENT` (Entity → EntityContent)
- `HAS_OUTPUT` renamed to `HAS_GENERATED` (Page/Block → PageGenerated/BlockGenerated)

**Examples (v0.12.5):**

```
✅ Entity (defined) → EntityNative (authored)        # v0.12.5: Semantic layer native content
✅ Page (defined) → PageNative (generated)           # v0.12.5: Output layer native content
✅ Block (defined) → BlockNative (generated)         # v0.12.5: Output layer native content
✅ Project (defined) → ProjectNative (authored)      # v0.12.5: Foundation layer native content
✅ Entity (defined) → EntityCategory (defined)       # shared/config categorization
✅ SEOKeyword (imported, no parent)                  # Correct: no suffix
✅ Term (imported atom, no parent)                   # Correct: no suffix
✅ SEOKeywordMetrics (retrieved)                     # Computed metrics

❌ EntityContent (deprecated v0.12.5)                # Use EntityNative
❌ PageGenerated (deprecated v0.12.5)                # Use PageNative
❌ BlockGenerated (deprecated v0.12.5)               # Use BlockNative
❌ ProjectContent (deprecated v0.12.5)               # Use ProjectNative
❌ EntityL10n (deprecated v10.9)                     # Use EntityNative (via EntityContent)
❌ PageL10n (deprecated v10.9)                       # Use PageNative (via PageGenerated)
❌ BlockL10n (deprecated v10.9)                      # Use BlockNative (via BlockGenerated)
❌ invariant/localized/knowledge/aggregated (deprecated) # Use ADR-024 names
❌ job (removed trait)                               # Concept deferred to v12+
```

**Rationale (v0.12.5):**
- `*Native` suffix indicates locale-specific content (authored OR generated natively)
- Trait distinguishes WHO creates: `authored` = human, `generated` = LLM
- Unified arc `HAS_NATIVE` replaces both `HAS_CONTENT` and `HAS_GENERATED`
- `*Metrics` suffix indicates computed/retrieved data (retrieved trait)
- `*Category` suffix indicates categorical grouping/taxonomy structure (defined trait)
- `*Content` and `*Generated` suffixes are DEPRECATED - use `*Native` instead
- Suffix choice reflects locale-specificity, trait indicates creation method

## Property Naming

Properties use `snake_case` in YAML and TypeScript:

```yaml
# YAML
node:
  name: LocaleVoice
  realm: shared             # v11.2: renamed from global
  layer: knowledge          # v11.5: 10 layers (4 shared + 6 org)
  trait: imported           # v0.12.0: ADR-024 renamed from knowledge
  display_name: "Locale Voice"
  llm_context: "..."
```

```typescript
// TypeScript
interface NodeClass {
  name: string;
  realm: NodeRealm;
  layer: NodeLayer;
  trait: NodeTrait;
  display_name: string;
  llm_context?: string;
}
```

## Deprecated Terms

These terms are deprecated and should NOT be used:

| Deprecated | Use Instead | Notes |
|------------|-------------|-------|
| `Edge` | `Arc` | Except in React Flow code |
| `EdgeKind` | `ArcKind` | |
| `EdgeFamily` | `ArcFamily` | |
| `Relation` | `Arc` | |
| `RelationType` | Keep | Neo4j rel type string (e.g., `"HAS_PAGE"`) — ArcClass is the schema-node |
| `Scope` (for realm) | `Realm` | v9.0 renamed |
| `Subcategory` | `Layer` | v9.0 renamed |
| `NodeTypeMeta` | `Class` | v9.0 renamed (Kind→Class in v0.12.0) |
| `DataMode` | `NavigationMode` | v9.0 renamed |
| `category` | `trait` | YAML property |
| `global` | `shared` | v11.2 realm rename |
| `tenant` | `org` | v11.2 realm rename |
| `derived` | `generated` / `aggregated` | v11.2 trait split |
| `job` | (removed) | v11.2 trait removed |
| `EntityL10n` | `EntityContent` | v10.9 renamed (semantic layer) |
| `PageL10n` | `PageGenerated` | v10.9 renamed (output layer) |
| `BlockL10n` | `BlockGenerated` | v10.9 renamed (output layer) |
| `ProjectL10n` | `ProjectContent` | v11.0 renamed (foundation layer) |
| `HAS_L10N` | `HAS_CONTENT` | v10.9 renamed (Entity → EntityContent) |
| `HAS_OUTPUT` | `HAS_GENERATED` | v10.9 renamed (Page/Block → *Generated) |
| `BELONGS_TO_PROJECT_L10N` | `BELONGS_TO_PROJECT_CONTENT` | v11.0 renamed |
| `GenerationJob` | (removed) | v11.2 job nodes removed |
| `SEOMiningRun` | (removed) | v11.2 job nodes removed |
| `EvaluationSignal` | (removed) | v11.2 job nodes removed |
| `GEOSeedL10n` | `GEOQuery` | v10.7 new GEO schema |
| `GEOSeedMetrics` | `GEOMetrics` | v10.7 new GEO schema |
| `locale-knowledge` | `locale` / `geography` / `knowledge` | v11.3 layer split |
| `Organization` | `OrgConfig` | v11.3 node merge |
| `Tenant` | `OrgConfig` | v11.3 node merge |
| `org/seo` layer | `shared/knowledge` | v11.5 SEO nodes consolidated |
| `org/geo` layer | `shared/knowledge` | v11.5 GEO nodes consolidated |
| `Locale` in `shared/locale` | `Locale` in `shared/config` | v11.5 definitions layer pattern |
| `data` mode | `graph` | v11.7 unified tree |
| `meta` mode | `graph` | v11.7 unified tree |
| `overlay` mode | `graph` | v11.7 unified tree |
| `query` mode | `graph` + filters | v11.7 unified tree |
| `atlas` mode | `nexus` | v11.7 renamed |
| emoji icons | dual format `{ web, terminal }` | v11.7 icon system |
| **v0.12.0 Kind→Class + Meta elimination** | | |
| `NodeKind` | `NodeClass` | v0.12.0 terminology |
| `ArcKind` | `ArcClass` | v0.12.0 terminology |
| `KindInfo` | `ClassInfo` | v0.12.0 TUI struct |
| `KindMeta` | `Classification` | v0.12.0 (realm/layer/trait axes) |
| `KIND_META` | `CLASS_TAXONOMY` | v0.12.0 TypeScript constant |
| `:Meta:Kind` | `:Schema:Class` | v0.12.0 Neo4j label |
| `:Meta:ArcKind` | `:Schema:ArcClass` | v0.12.0 Neo4j label |
| `[:FROM_KIND]` | `[:FROM_CLASS]` | v0.12.0 Neo4j relationship |
| `[:TO_KIND]` | `[:TO_CLASS]` | v0.12.0 Neo4j relationship |
| `[:HAS_KIND]` | `[:HAS_CLASS]` | v0.12.0 Neo4j relationship |
| "Meta Node" | "Class" | v0.12.0 glossary |
| "Data Node" | "Instance" | v0.12.0 glossary |
| "Meta mode" | "Schema view" | v0.12.0 Studio UI |
| "Data mode" | "Graph view" | v0.12.0 Studio UI |
| **v0.12.0 Trait Redefinition (ADR-024)** | | |
| `invariant` | `defined` | v0.12.0 trait rename (human-created once) |
| `localized` | `authored` | v0.12.0 trait rename (human-written per locale) |
| `knowledge` (trait) | `imported` | v0.12.0 trait rename (external data brought in) |
| `aggregated` | `retrieved` | v0.12.0 trait rename (fetched from external APIs) |
| **v0.12.0 Instruction Layer (ADR-025)** | | |
| `PageType` | `PageStructure` | v0.12.0 (JSON defining block order) |
| `PagePrompt` | `PageInstruction` | v0.12.0 (Markdown with @ refs) |
| `BlockPrompt` | `BlockInstruction` | v0.12.0 (Markdown with @ refs) |
| `[:OF_TYPE]` (Page→PageType) | `[:HAS_STRUCTURE]` | v0.12.0 arc rename |
| `[:HAS_PROMPT]` (Page→PagePrompt) | `[:HAS_INSTRUCTION]` | v0.12.0 arc rename |
| `[:HAS_PROMPT]` (Block→BlockPrompt) | `[:HAS_INSTRUCTION]` | v0.12.0 arc rename |
| **v0.12.5 *Native Pattern (ADR-029)** | | |
| `EntityContent` | `EntityNative` | v0.12.5 *Native pattern (semantic layer) |
| `PageGenerated` | `PageNative` | v0.12.5 *Native pattern (output layer) |
| `BlockGenerated` | `BlockNative` | v0.12.5 *Native pattern (output layer) |
| `ProjectContent` | `ProjectNative` | v0.12.5 *Native pattern (foundation layer) |
| `HAS_CONTENT` | `HAS_NATIVE` | v0.12.5 unified arc (with `locale` property) |
| `HAS_GENERATED` | `HAS_NATIVE` | v0.12.5 unified arc (with `locale` property) |
| `CONTENT_OF` | `NATIVE_OF` | v0.12.5 inverse arc rename |
| `GENERATED_FOR` | `NATIVE_OF` | v0.12.5 inverse arc rename |
| **v0.12.5 Slug Ownership (ADR-030)** | | |
| `EntityContent.slug` | `PageNative.slug` | v0.12.5 URL segment moved to Page layer |
| `EntityContent.full_path` | `PageNative.full_path` | v0.12.5 full URL path moved to Page layer |
| `EntityContent.parent_slug` | (removed) | v0.12.5 calculated from Page.SUBTOPIC_OF |
| `EntityContent.depth` | (removed) | v0.12.5 calculated from Page hierarchy |

## Navigation Modes (v11.7)

| Mode | Content | Use Case |
|------|---------|----------|
| `graph` | Unified tree (Realm > Layer > Class > Instance + Arcs) | Default exploration |
| `nexus` | Hub (Quiz, Audit, Stats, Help) | Learning & validation |

> **v11.7 Change**: Consolidated from 5 modes (Meta/Data/Overlay/Query/Atlas) to 2 modes (Graph/Nexus).
> The unified tree in Graph mode shows meta AND data together — no mode switching needed.

**Deprecated modes** (v11.6 and earlier):

| Deprecated | Replacement | Notes |
|------------|-------------|-------|
| `data` | `graph` | Instances now under Class nodes |
| `meta` | `graph` | Meta-graph integrated in unified tree |
| `overlay` | `graph` | Unified tree shows both |
| `query` | `graph` + filters | Use tree filters instead |
| `atlas` | `nexus` | Renamed, expanded functionality |

## Visual Encoding

| Visual Channel | Encodes | Source |
|----------------|---------|--------|
| Fill color | Layer | `taxonomy.yaml` node_layers[].color |
| Border style | Trait | `visual-encoding.yaml` trait_borders |
| Border color | Realm | `taxonomy.yaml` node_realms[].color |
| Arc stroke | ArcFamily | `taxonomy.yaml` arc_families[].color |
| Arc dash | ArcScope | solid (intra) / dashed (cross) |

## Icons (v11.7)

Source of truth: `packages/core/models/visual-encoding.yaml` → `icons:` section

### Dual Icon Format (v11.7)

Icons use dual format for different rendering contexts:

| Context | Format | Source |
|---------|--------|--------|
| Studio (web) | Lucide icon name | `icon.web` |
| TUI (terminal) | Unicode symbol | `icon.terminal` |

> **Rule**: NO emoji in code. Use `{ web: "globe", terminal: "◉" }` format.

```typescript
// Correct (v11.7)
const realmIcon = { web: "globe", terminal: "◉" };

// Wrong - NO emoji
const realmIcon = "🌐";  // DEPRECATED
```

### Icon Categories

| Category | Purpose | Examples (terminal) |
|----------|---------|---------------------|
| `realms` | Node ownership | ◉ shared, ◎ org |
| `layers` | Functional layer | ⚙ config, ● locale, ◆ geography, ◊ knowledge, ■ semantic, ▣ output |
| `traits` | Data origin | ■ defined, □ authored, ◊ imported, ✦ generated, ⋆ retrieved |
| `arc_families` | Arc type | → ownership, ⇢ localization |
| `states` | UI empty states | ◐ loading, ∅ no_kinds, ⚠ error |
| `navigation` | Tree controls | ▼ expanded, ▶ collapsed |
| `quality` | Data completeness | ● complete, ◐ partial, * required |
| `modes` | Nav modes | G graph, N nexus |

**TUI loading**: `Theme::with_root()` loads icons from YAML at startup.

**Fallback**: Default icons used if YAML loading fails (graceful degradation).

## Commands

Use Arc terminology in commands:

```bash
# Correct (v9.5)
novanet arc create --from=page1 --to=entity1 --kind=USES_ENTITY
novanet arc delete --id=abc123

# Deprecated (v9.0)
novanet relation create ...  # Still works, but deprecated
```

## Query-First Architecture (v11.6)

NovaNet Studio uses **Query-First Architecture** where Cypher queries are the single source of truth for graph visualization.

### Core Concepts

| Term | Definition |
|------|------------|
| **Query-First** | Architecture pattern where graph display is determined solely by the executed Cypher query |
| **Schema-Graph** | The schema graph showing NodeClass and ArcClass nodes (61 nodes, 128 arcs) |
| **CLASS_QUERY** | Foundational query that fetches all NodeClass instances for schema view |
| **ARCS_QUERY** | Foundational query that fetches all ArcClass instances for schema view |
| **View** | Parameterized Cypher template defined in YAML, executable with context parameters |
| **ViewPicker** | UI component for selecting and executing views |
| **QueryPill** | UI component displaying the current query, with edit capability |

### View Categories

| Category | Purpose | Contextual |
|----------|---------|------------|
| `global` | Full graph exploration (complete-graph, shared-layer) | No |
| `contextual` | Node-specific subgraph (composition, knowledge) | Yes |
| `generation` | AI agent context (block-generation) | Yes |
| `mining` | SEO/GEO intelligence (seo-intel, geo-intel) | Yes |

### Foundational Queries

```cypher
-- CLASS_QUERY: Fetch all NodeClass instances (SCHEMA mode)
MATCH (k:Class)
RETURN k.name AS name, k.realm AS realm, k.layer AS layer,
       k.trait AS trait, k.display_name AS display_name

-- ARCS_QUERY: Fetch all ArcClass instances (SCHEMA mode)
MATCH (a:ArcClass)
RETURN a.name AS name, a.family AS family, a.scope AS scope,
       a.cardinality AS cardinality, a.source AS source, a.target AS target
```

### View Execution Flow

```
User clicks ViewPicker
    ↓
viewStore.executeView(viewId, params)
    ↓
/api/views/:id/query (fetch YAML + substitute params)
    ↓
Neo4j executes Cypher
    ↓
queryStore.setQuery(cypher)   # QueryPill displays
graphStore.setNodes(results)  # Graph renders
```

### Interactions

| Action | Behavior |
|--------|----------|
| Click view | `executeView()` → auto-run query → update graph |
| Ctrl+Click view | `loadQueryOnly()` → load query without executing |
| Edit QueryPill | Manual changes → click ▶️ to run |
| Context view card | `executeView()` with `nodeKey` param |

### YAML View Schema

```yaml
id: composition
description: Page/Block composition hierarchy
category: contextual         # global | contextual | generation | mining
contextual: true             # appears in node sidebar
applicable_types: [Page, Block]  # compatible node types
modes: [data, meta, overlay, query]
cypher: |
  MATCH (root {key: $nodeKey})
  ...
```

> **Reference**: See ADR-021 in `novanet-decisions.md` for full architecture rationale.

## Summary

1. **Arc** = directed link (not Edge, not Relation)
2. **NodeClass** = node type definition (v0.12.0: was NodeKind)
3. **ArcClass** = arc type definition (v0.12.0: was ArcKind)
4. **Realm/Layer/Trait** = node classification axes
5. **ArcFamily/ArcScope/ArcCardinality** = arc classification axes
6. **taxonomy.yaml** = source of truth for facet definitions
7. **Query-First** = Cypher query determines graph display (v11.6)
8. **Schema-Graph** = schema graph of NodeClass + ArcClass nodes (v0.12.0: was Meta-Graph)
9. **CLASS_QUERY / ARCS_QUERY** = foundational queries for schema view (v0.12.0)
10. **Unified Tree** = single tree showing Realm > Layer > Class > Instance + Arcs (v11.7/v0.12.0)
11. **Graph/Nexus** = two navigation modes replacing 5 previous modes (v11.7)
12. **Dual Icons** = `{ web: "lucide-name", terminal: "◉" }` format, NO emoji (v11.7)
13. **Trait = Data Origin** = WHERE does data come from? (defined/authored/imported/generated/retrieved) (v0.12.0)
14. **PageStructure/PageInstruction** = replaced PageType/PagePrompt (v0.12.0)
15. **\*Native Pattern** = unified suffix for locale-specific nodes (EntityNative, PageNative, BlockNative) (v0.12.5)
16. **HAS_NATIVE** = unified arc replacing HAS_CONTENT + HAS_GENERATED, with `locale` property (v0.12.5)
17. **Slug Ownership** = Page owns URL (slug, full_path), Entity owns semantics (key) (v0.12.5)
