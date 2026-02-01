# NovaNet Ontology v9: Realm / Layer / Kind / Trait

**Date**: 2026-02-01
**Status**: Draft
**Author**: Thibaut + Claude
**Supersedes**: 2026-01-31-organizing-principles.md (v8.3.0)

## Summary

Refactor the NovaNet meta-graph from a flat tree (Scope > Subcategory > NodeTypeMeta)
to a **faceted ontology** with 6 meta-node types and dual navigation (hierarchy + facettes).

Key changes:
- Rename all meta-labels: Scope -> Realm, Subcategory -> Layer, NodeTypeMeta -> Kind
- Add 2 new meta-types: Trait (locale behavior), EdgeFamily + EdgeKind (relation ontology)
- Instance bridge: replace `IN_SUBCATEGORY` with `OF_KIND`
- Dual navigation: top-down hierarchy + Kind-centric facettes
- Edge ontology: OWL-inspired FROM_KIND / TO_KIND multi-hop pattern
- YAML remains source of truth, Neo4j is generated

## Background

### v8.3.0 (Current)

The meta-graph has 3 levels in a simple tree:

```
:Scope (3)  -[:HAS_SUBCATEGORY]->  :Subcategory (9)  -[:DEFINES_TYPE]->  :NodeTypeMeta (35)
```

Problems:
1. **Generic naming**: "Scope", "Subcategory", "NodeTypeMeta" are implementation terms, not domain concepts
2. **Missing behavior axis**: locale behavior (invariant/localized/etc.) only exists in hardcoded TypeScript
3. **No edge metadata**: relation families (ownership/semantic/etc.) are only comments in YAML
4. **Single axis**: the tree only classifies by WHERE > WHAT, missing HOW (behavior)
5. **Weak instance link**: `IN_SUBCATEGORY` skips the type level

### v9.0.0 (This Design)

A **faceted classification** where each Kind sits at the intersection of multiple axes:

```
Axis 1 — WHERE?  :Realm    (global / project / shared)
Axis 2 — WHAT?   :Layer    (knowledge / structure / semantic / ...)
Axis 3 — HOW?    :Trait    (invariant / localized / knowledge / derived / job)
Axis 4 — LINKS?  :EdgeKind (SEMANTIC_LINK, HAS_OUTPUT, HAS_L10N, ...)
```

## Design

### Meta-Node Types

#### :Realm (3 nodes)

Visibility boundary / data governance zone.

| Key | Display Name | Emoji | Color |
|-----|-------------|-------|-------|
| `global` | Global | `🌍` | `#2aa198` |
| `project` | Project | `📦` | `#6c71c4` |
| `shared` | Shared | `🎯` | `#cb4b16` |

Properties:
```
key:          string  (PK, unique)
display_name: string
emoji:        string
color:        string  (hex, for Studio)
llm_context:  string  (description for LLM)
created_at:   datetime
updated_at:   datetime
```

#### :Layer (9 nodes)

Functional classification / architectural layer.

| Key | Display Name | Emoji | Parent Realm |
|-----|-------------|-------|-------------|
| `config` | Configuration | `⚙️` | global |
| `knowledge` | Locale Knowledge | `📚` | global |
| `foundation` | Foundation | `🏛️` | project |
| `structure` | Structure | `🏗️` | project |
| `semantic` | Semantic Layer | `💡` | project |
| `instruction` | Instructions | `📝` | project |
| `output` | Generated Output | `✨` | project |
| `seo` | SEO Intelligence | `🔍` | shared |
| `geo` | GEO Intelligence | `📍` | shared |

Properties:
```
key:          string  (PK, unique)
display_name: string
emoji:        string
llm_context:  string
created_at:   datetime
updated_at:   datetime
```

#### :Kind (35 nodes)

A node type in the data graph. One Kind per Neo4j label.

Properties:
```
label:                string    (PK, unique — the Neo4j label)
display_name:         string
llm_context:          string
yaml_path:            string    (path to YAML source file)
properties:           string[]  (all schema fields)
required_properties:  string[]  (mandatory fields)
created_at:           datetime
updated_at:           datetime
```

Complete Kind inventory:

| Kind (label) | Realm | Layer | Trait |
|-------------|-------|-------|-------|
| Locale | global | config | invariant |
| LocaleIdentity | global | knowledge | knowledge |
| LocaleVoice | global | knowledge | knowledge |
| LocaleCulture | global | knowledge | knowledge |
| LocaleCultureReferences | global | knowledge | knowledge |
| LocaleLexicon | global | knowledge | knowledge |
| LocaleMarket | global | knowledge | knowledge |
| LocaleRulesAdaptation | global | knowledge | knowledge |
| LocaleRulesFormatting | global | knowledge | knowledge |
| LocaleRulesSlug | global | knowledge | knowledge |
| Expression | global | knowledge | knowledge |
| Metaphor | global | knowledge | knowledge |
| Pattern | global | knowledge | knowledge |
| Reference | global | knowledge | knowledge |
| Constraint | global | knowledge | knowledge |
| Project | project | foundation | invariant |
| BrandIdentity | project | foundation | invariant |
| ProjectL10n | project | foundation | localized |
| Page | project | structure | invariant |
| Block | project | structure | invariant |
| Concept | project | semantic | invariant |
| ConceptL10n | project | semantic | localized |
| PageType | project | instruction | invariant |
| BlockType | project | instruction | invariant |
| PagePrompt | project | instruction | invariant |
| BlockPrompt | project | instruction | invariant |
| BlockRules | project | instruction | invariant |
| PageL10n | project | output | localized |
| BlockL10n | project | output | localized |
| SEOKeywordL10n | shared | seo | localized |
| SEOKeywordMetrics | shared | seo | derived |
| SEOMiningRun | shared | seo | job |
| GEOSeedL10n | shared | geo | localized |
| GEOSeedMetrics | shared | geo | derived |
| GEOMiningRun | shared | geo | job |

#### :Trait (5 nodes)

Locale behavior — how a node type changes across locales.

| Key | Display Name | Color | Description |
|-----|-------------|-------|-------------|
| `invariant` | Invariant | `#3b82f6` | Does not change between locales |
| `localized` | Localized | `#22c55e` | Generated natively per locale |
| `knowledge` | Knowledge | `#8b5cf6` | Cultural/linguistic expertise per locale |
| `derived` | Derived | `#9ca3af` | Computed/aggregated data |
| `job` | Job | `#6b7280` | Background processing tasks |

Properties:
```
key:          string  (PK, unique)
display_name: string
llm_context:  string
color:        string  (hex, for Studio node colors)
created_at:   datetime
updated_at:   datetime
```

#### :EdgeFamily (5 nodes)

Classification of relationship types.

| Key | Display Name | Color | Arrow Style |
|-----|-------------|-------|------------|
| `ownership` | Ownership | `#3b82f6` | `-->` |
| `localization` | Localization | `#22c55e` | `-.->` |
| `semantic` | Semantic | `#f97316` | `-.->` |
| `generation` | Generation | `#8b5cf6` | `==>` |
| `mining` | Mining | `#ec4899` | `--o` |

Properties:
```
key:          string  (PK, unique)
display_name: string
llm_context:  string
color:        string  (hex, for Studio edge colors)
arrow_style:  string  (Mermaid arrow syntax)
created_at:   datetime
updated_at:   datetime
```

#### :EdgeKind (47 nodes)

Individual relationship type in the data graph. One EdgeKind per Neo4j relationship type.

Properties:
```
key:                  string    (PK, unique — the Neo4j rel type)
display_name:         string
llm_context:          string
cardinality:          string    ('one_to_one' | 'one_to_many' | 'many_to_many')
is_self_referential:  boolean
inverse_name:         string    (semantic inverse name, nullable)
edge_properties:      string[]  (properties carried on the relationship)
created_at:           datetime
updated_at:           datetime
```

### Meta-Graph Relations

#### Hierarchy (top-down navigation)

```
Realm  -[:HAS_LAYER]->      Layer      (12 rels: 3 realms × avg 3 layers)
Layer  -[:HAS_KIND]->        Kind       (35 rels: one per Kind)
EdgeFamily -[:HAS_EDGE_KIND]-> EdgeKind (47 rels: one per EdgeKind)
```

#### Facettes (Kind-centric)

```
Kind -[:IN_REALM]->  Realm   (35 rels)
Kind -[:IN_LAYER]->  Layer   (35 rels)
Kind -[:EXHIBITS]->  Trait   (35 rels)
```

#### Edge Schema (OWL-inspired)

```
EdgeKind -[:FROM_KIND]-> Kind   (N sources per EdgeKind)
EdgeKind -[:TO_KIND]->   Kind   (M targets per EdgeKind)
EdgeKind -[:IN_FAMILY]-> EdgeFamily
```

Multi-source/target example:
```
(:EdgeKind {key:'HAS_OUTPUT'})
  -[:FROM_KIND]-> (:Kind {label:'Block'})
  -[:FROM_KIND]-> (:Kind {label:'Page'})
  -[:TO_KIND]->   (:Kind {label:'BlockL10n'})
  -[:TO_KIND]->   (:Kind {label:'PageL10n'})
  -[:IN_FAMILY]-> (:EdgeFamily {key:'localization'})
```

#### Instance Bridge

```
<any data node> -[:OF_KIND]-> Kind
```

Every data instance (Page, Block, Concept, etc.) links to its Kind.
Replaces the v8.3.0 `IN_SUBCATEGORY` relationship.

### Rename Mapping (v8.3.0 -> v9.0.0)

| v8.3.0 | v9.0.0 | Notes |
|--------|--------|-------|
| `:Scope` | `:Realm` | Visibility boundary |
| `:Subcategory` | `:Layer` | Functional classification |
| `:NodeTypeMeta` | `:Kind` | Node type definition |
| (not present) | `:Trait` | Locale behavior |
| (not present) | `:EdgeFamily` | Relation family |
| (not present) | `:EdgeKind` | Relation type |
| `:HAS_SUBCATEGORY` | `:HAS_LAYER` | Hierarchy |
| `:DEFINES_TYPE` | `:HAS_KIND` | Hierarchy |
| `:IN_SUBCATEGORY` | `:OF_KIND` | Instance bridge |
| (not present) | `:IN_REALM` | Facette |
| (not present) | `:IN_LAYER` | Facette |
| (not present) | `:EXHIBITS` | Facette |
| (not present) | `:IN_FAMILY` | Edge classification |
| (not present) | `:HAS_EDGE_KIND` | Edge hierarchy |
| (not present) | `:FROM_KIND` | Edge source |
| (not present) | `:TO_KIND` | Edge target |

### Trait Naming Change

| v8.3.0 (nodes.ts) | v9.0.0 | Reason |
|-------------------|--------|--------|
| `invariant` | `invariant` | No change |
| `localized` | `localized` | No change |
| `localeKnowledge` | `knowledge` | Simplified, camelCase removed |
| `derived` | `derived` | No change |
| `job` | `job` | No change |

## Source of Truth

**YAML remains the master source.**

```
organizing-principles.yaml  →  generate  →  Neo4j seed (Cypher)
node YAML files (35)         →  generate  →  TypeScript types
relations.yaml               →  generate  →  EdgeKind/EdgeFamily seed
```

The flow:
1. Edit YAML files
2. Run `pnpm schema:generate`
3. Generated Cypher seeds, TypeScript, and Mermaid
4. Run `pnpm infra:seed` to apply to Neo4j

### Generator architecture

The v8.3.0 `OrganizingPrinciplesGenerator.ts` handles 3 meta-node types
(Scope, Subcategory, NodeTypeMeta) in a single file. The v9 schema has 6
meta-node types and significantly more relationships, making a monolithic
generator unwieldy.

Recommended split:

| Generator | Input | Output |
|-----------|-------|--------|
| `OrganizingPrinciplesGenerator` | `organizing-principles.yaml` | Realm, Layer, Trait, EdgeFamily Cypher + HAS_LAYER hierarchy |
| `KindGenerator` | 35 node YAML files | Kind nodes + IN_REALM, IN_LAYER, EXHIBITS facettes |
| `EdgeSchemaGenerator` | `relations.yaml` | EdgeKind nodes + FROM_KIND, TO_KIND, IN_FAMILY, HAS_EDGE_KIND |
| `LayerGenerator` (renamed from `SubcategoryGenerator`) | 35 node YAML files | TypeScript `Record<NodeType, Layer>` mapping |
| `MermaidGenerator` | All YAML | Updated diagram with Realm/Layer/Trait coloring |
| `AutowireGenerator` | 35 node YAML files | `99-autowire-kinds.cypher` (OF_KIND statements) |
| `HierarchyGenerator` | `organizing-principles.yaml` | TypeScript `hierarchy.ts` (realm→layer tree, auto-generated) |

Each generator reads YAML and writes to a specific output file.
`generate-all.ts` orchestrates them in dependency order (7 generators).

### RelationsParser restructuring

The v8.3.0 `RelationsParser.ts` parses a **flat dictionary** format:

```yaml
# v8.3.0 format (dict keyed by type)
HAS_PAGE: { from: Project, to: Page }
SEMANTIC_LINK: { from: Concept, to: Concept, properties: [temperature] }
```

The v9 format uses a **list** with `family` and multi-source/target:

```yaml
# v9.0.0 format (list of objects)
- type: HAS_PAGE
  family: ownership
  source: Project
  target: Page
  cardinality: one_to_many

- type: HAS_OUTPUT
  family: localization
  source: [Block, Page]         # multi-source
  target: [BlockL10n, PageL10n] # multi-target
  cardinality: one_to_many
```

`RelationsParser.ts` must be updated to handle:
1. List format instead of dict
2. `family` field (maps to EdgeFamily)
3. `source`/`target` as string OR string[] (multi-source/target)
4. Wildcard `"*"` expansion (see "Wildcard sources" section)

## Neo4j Constraints & Indexes

### Drop v8.3.0 constraints

```cypher
DROP CONSTRAINT scope_key IF EXISTS;
DROP CONSTRAINT subcategory_key IF EXISTS;
DROP CONSTRAINT nodetypemeta_label IF EXISTS;
```

### Create v9.0.0 constraints

```cypher
CREATE CONSTRAINT realm_key IF NOT EXISTS FOR (r:Realm) REQUIRE r.key IS UNIQUE;
CREATE CONSTRAINT layer_key IF NOT EXISTS FOR (l:Layer) REQUIRE l.key IS UNIQUE;
CREATE CONSTRAINT kind_label IF NOT EXISTS FOR (k:Kind) REQUIRE k.label IS UNIQUE;
CREATE CONSTRAINT trait_key IF NOT EXISTS FOR (t:Trait) REQUIRE t.key IS UNIQUE;
CREATE CONSTRAINT edge_family_key IF NOT EXISTS FOR (ef:EdgeFamily) REQUIRE ef.key IS UNIQUE;
CREATE CONSTRAINT edge_kind_key IF NOT EXISTS FOR (ek:EdgeKind) REQUIRE ek.key IS UNIQUE;
```

### :Meta double-labeling

All 6 meta-node types receive a secondary `:Meta` label:

```cypher
// Every meta-node is double-labeled
CREATE (:Meta:Realm {key:'global', ...})
CREATE (:Meta:Layer {key:'config', ...})
CREATE (:Meta:Kind  {label:'Page', ...})
CREATE (:Meta:Trait  {key:'invariant', ...})
CREATE (:Meta:EdgeFamily {key:'ownership', ...})
CREATE (:Meta:EdgeKind   {key:'HAS_PAGE', ...})
```

Benefits:
- **One-query meta-graph**: `MATCH (n:Meta) RETURN n` returns all ~104 meta-nodes
- **Clear separation**: `MATCH (n) WHERE NOT n:Meta` excludes all meta-nodes
- **Studio toggle**: easy "show/hide meta-graph" filter (`WHERE NOT n:Meta`)
- **Standard OWL pattern**: consistent with n10s/neosemantics conventions

The constraint DDL remains on the specific labels (`:Realm`, `:Kind`, etc.),
not on `:Meta`. The `:Meta` label is purely for grouping/filtering.

### Indexes

Neo4j automatically creates a backing index for each UNIQUE constraint,
so the 6 constraints above provide 6 indexes for free. No additional
indexes are needed for meta-node lookups.

Existing data node indexes (00-constraints.cypher) remain unchanged:
- 7 uniqueness constraints on data nodes (locale_key, project_key, etc.)
- 15+ property indexes (temporal, search, volume)
- 6 relationship indexes (temperature, position, status)

## LLM Context Assembly

The meta-graph enables **schema-driven context discovery**. Instead of hardcoding
which nodes to load for generation, the orchestrator queries the meta-graph.

### Query: "Describe Kind 'Page' with full context"

```cypher
MATCH (k:Kind {label:'Page'})
MATCH (k)-[:IN_REALM]->(r:Realm)
MATCH (k)-[:IN_LAYER]->(l:Layer)
MATCH (k)-[:EXHIBITS]->(t:Trait)
OPTIONAL MATCH (k)<-[:FROM_KIND]-(ek:EdgeKind)-[:IN_FAMILY]->(f:EdgeFamily)
OPTIONAL MATCH (ek)-[:TO_KIND]->(target:Kind)
RETURN k, r, l, t,
       collect(DISTINCT {edge: ek.key, family: f.key, targets: target.label}) AS edges
```

### Query: "All localized Kinds in the project realm"

```cypher
MATCH (k:Kind)-[:IN_REALM]->(:Realm {key:'project'})
MATCH (k)-[:EXHIBITS]->(:Trait {key:'localized'})
RETURN k.label, k.llm_context
```

### Query: "What edges connect to a Block?"

```cypher
MATCH (:Kind {label:'Block'})<-[:FROM_KIND]-(ek:EdgeKind)
      -[:TO_KIND]->(target:Kind)
MATCH (ek)-[:IN_FAMILY]->(f:EdgeFamily)
RETURN ek.key AS edge, f.key AS family, target.label AS target,
       ek.llm_context AS description
```

### Query: "Navigate the full taxonomy top-down"

```cypher
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
MATCH (k)-[:EXHIBITS]->(t:Trait)
RETURN r.key AS realm, l.key AS layer, k.label AS kind, t.key AS trait
ORDER BY r.key, l.key, k.label
```

## Totals

| Meta-node Type | Count |
|---------------|-------|
| Realm | 3 |
| Layer | 9 |
| Kind | 35 |
| Trait | 5 |
| EdgeFamily | 5 |
| EdgeKind | 47 |
| **Total meta-nodes** | **~104** |

| Meta-relation Type | Count |
|-------------------|-------|
| HAS_LAYER | 12 |
| HAS_KIND | 35 |
| IN_REALM | 35 |
| IN_LAYER | 35 |
| EXHIBITS | 35 |
| HAS_EDGE_KIND | 47 |
| FROM_KIND | ~70 |
| TO_KIND | ~70 |
| IN_FAMILY | 47 |
| **Total meta-rels** | **~386** |

Instance bridge (OF_KIND): proportional to data volume.

## Property Naming Conventions

All meta-node properties follow the existing NovaNet conventions documented in `_index.yaml`:

| Element | Convention | Example |
|---------|-----------|---------|
| Node Labels | PascalCase | `Realm`, `EdgeKind`, `Kind` |
| Relationships | UPPER_SNAKE_CASE | `HAS_LAYER`, `FROM_KIND`, `OF_KIND` |
| Properties | snake_case | `display_name`, `llm_context`, `edge_properties` |

### Standard properties (6 fields on every data node)

| Property | Type | Required | Notes |
|----------|------|----------|-------|
| `key` | string | Yes (invariant only) | Absent on localized/knowledge nodes |
| `display_name` | string | Yes | Human-readable UI name |
| `description` | string | Yes | One-line purpose |
| `llm_context` | string | Yes | Format: `"USE: [when]. TRIGGERS: [kw]. NOT: [disambiguation]."` |
| `created_at` | datetime | Yes | Creation timestamp |
| `updated_at` | datetime | Yes | Last update timestamp |

### Meta-node PK convention

- `:Realm`, `:Layer`, `:Trait`, `:EdgeFamily` use `key` as PK (consistent with data nodes)
- `:Kind` uses `label` as PK (matches the Neo4j label it describes)
- `:EdgeKind` uses `key` as PK (matches the Neo4j relationship type)

## YAML Source Structure

### organizing-principles.yaml (v9 format)

```yaml
version: "9.0.0"

realms:
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"
    llm_context: |
      Shared across ALL projects...
    layers:
      - key: config
        display_name: Configuration
        emoji: "⚙️"
        llm_context: |
          Core configuration nodes...
      - key: knowledge
        # ...

  - key: project
    # ...layers: foundation, structure, semantic, instruction, output

  - key: shared
    # ...layers: seo, geo

traits:
  - key: invariant
    display_name: Invariant
    llm_context: "Nodes that do not change between locales."
    color: "#3b82f6"
  - key: localized
    display_name: Localized
    llm_context: "Content generated natively per locale."
    color: "#22c55e"
  - key: knowledge
    display_name: Knowledge
    llm_context: "Cultural/linguistic expertise per locale."
    color: "#8b5cf6"
  - key: derived
    display_name: Derived
    llm_context: "Computed or aggregated data."
    color: "#9ca3af"
  - key: job
    display_name: Job
    llm_context: "Background processing tasks."
    color: "#6b7280"

edge_families:
  - key: ownership
    display_name: Ownership
    llm_context: "Parent-child structural relationships."
    color: "#3b82f6"
    arrow_style: "-->"
  - key: localization
    display_name: Localization
    llm_context: "Links between invariant nodes and locale-specific content."
    color: "#22c55e"
    arrow_style: ".->"
  - key: semantic
    display_name: Semantic
    llm_context: "Meaning and concept connections."
    color: "#f97316"
    arrow_style: ".->"
  - key: generation
    display_name: Generation
    llm_context: "LLM generation pipeline flow."
    color: "#8b5cf6"
    arrow_style: "==>"
  - key: mining
    display_name: Mining
    llm_context: "SEO/GEO data extraction."
    color: "#ec4899"
    arrow_style: "--o"
```

### Node YAML files (v9 field migration)

Each of the 35 node YAML files requires two changes:

1. **Add `locale_behavior`** (REQUIRED — maps to Trait)
2. **Remove `category`** (replaced by folder-based inference)

The generator infers `realm` and `layer` from the folder path
(`models/nodes/{realm}/{layer}/{file}.yaml`), so an explicit `category`
field is redundant and a drift risk. Currently 33/35 files have `category`
with **old visual values** (`content`, `locale`, `generation`) that don't
match the v9 Layer taxonomy.

```yaml
# models/nodes/project/structure/page.yaml
node:
  name: Page
  locale_behavior: invariant    # REQUIRED — maps to Trait
  # category: removed (inferred from folder path → realm: project, layer: structure)
  description: "..."
```

**Current state (33/35 files need migration):**
- Only `page.yaml` and `page-type.yaml` already have `locale_behavior`
- 33 files have only `category` with old visual values
- 33 files are missing `locale_behavior` entirely

**Also fix stale header comments.** Several YAML files have incorrect paths:
- `block.yaml` header says `models/nodes/content/block.yaml` → fix to `models/nodes/project/structure/block.yaml`
- `block-type.yaml` header says `models/nodes/content/block-type.yaml` → fix to `models/nodes/project/instruction/block-type.yaml`
- `brand-identity.yaml` header says `models/nodes/project/brand-identity.yaml` → fix to `models/nodes/project/foundation/brand-identity.yaml`
- (audit all 35 files during migration)

The generator reads `locale_behavior` to create `Kind -[:EXHIBITS]-> Trait` relationships.

### relations.yaml (family required)

Each relation in relations.yaml MUST include `family`:

```yaml
# Example entries in relations.yaml
- type: HAS_PAGE
  family: ownership              # REQUIRED — maps to EdgeFamily
  source: Project
  target: Page
  cardinality: one_to_many
  llm_context: "..."

- type: SEMANTIC_LINK
  family: semantic
  source: Concept
  target: Concept
  cardinality: many_to_many
  is_self_referential: true
  properties:
    - temperature
    - semantic_field
  llm_context: "..."
```

## Studio Impact

### Overview

Studio already queries Neo4j dynamically for taxonomy via the
`/api/graph/organizing-principles` API endpoint. This is NOT hardcoded.
The migration requires updating queries and component names, not architecture.

### Files to update

| File | Change | Effort |
|------|--------|--------|
| `api/graph/organizing-principles/route.ts` | Rename Scope→Realm, Subcategory→Layer in Cypher queries. Add Trait query. | Small |
| `hooks/useMagneticData.ts` | Rename scope_key→realm_key, subcategory→layer in response handling | Small |
| `components/graph/schema/ScopeGroupNode.tsx` | Rename to RealmGroupNode.tsx. Get colors from Realm.color instead of hardcoded | Small |
| `components/graph/schema/SubcategoryGroupNode.tsx` | Rename to LayerGroupNode.tsx | Small |
| `stores/filterStore.ts` | Rename collapsedScopes→collapsedRealms, collapsedSubcategories→collapsedLayers | Small |
| `config/nodeTypes.ts` | Align 6 visual categories → 9 Layers from meta-graph | Medium |
| `lib/schemaGenerator.ts` | Update imports if nodes.ts is regenerated | Small |
| `components/graph/nodes/NodeConfig.ts` | Keep as-is (individual node colors are visual, not taxonomy) | None |

### Category alignment

Studio's 6 visual categories (project, content, locale, generation, seo, geo)
will be replaced by the 9 Layers from the meta-graph. This means:

| Old Studio category | New Layer(s) |
|--------------------|----|
| project | foundation |
| content | structure + semantic |
| locale | config + knowledge |
| generation | instruction + output |
| seo | seo |
| geo | geo |

Colors and grouping will come from `Layer.emoji` and `Realm.color` in Neo4j.
Individual node colors in NodeConfig.ts remain Studio-specific visual config.

### New feature: Trait-based filtering

The meta-graph enables a new filter dimension: "show only localized nodes",
"show only invariant nodes", etc. This can be added to filterStore as a
Trait-based filter.

## Navigation Modes (CLI + Studio)

The `:Meta` double-labeling enables **4 distinct navigation modes** for exploring the graph.
These modes should be accessible from both the CLI and Studio.

### Mode 1: Data Only

Show only business data — no meta-graph noise.

**Cypher**: `MATCH (n) WHERE NOT n:Meta RETURN n`

| What you see | What's hidden |
|-------------|--------------|
| Pages, Blocks, Concepts, Locales, L10n content, SEO/GEO data | Realm, Layer, Kind, Trait, EdgeFamily, EdgeKind |
| HAS_PAGE, SEMANTIC_LINK, FOR_LOCALE, etc. | HAS_LAYER, IN_REALM, EXHIBITS, FROM_KIND, etc. |

**CLI**: `pnpm graph:data`
**Studio**: Toggle button "Data Only" (default view)

### Mode 2: Meta Only

Show only the ontology schema — the "schema of the schema".

**Cypher**: `MATCH (n:Meta) RETURN n`

| What you see | What's hidden |
|-------------|--------------|
| 3 Realms, 9 Layers, 35 Kinds, 5 Traits, 5 EdgeFamilies, 47 EdgeKinds | All data instances |
| HAS_LAYER, HAS_KIND, IN_REALM, EXHIBITS, FROM_KIND, TO_KIND, IN_FAMILY | All data relationships |

**CLI**: `pnpm graph:meta`
**Studio**: Toggle button "Meta Only"

### Mode 3: Data + Meta Overlay

Show everything — data graph with meta-graph overlaid. The `OF_KIND` bridge
relationships connect data instances to their Kind nodes.

**Cypher**: `MATCH (n) RETURN n` (no filter)

| What you see |
|-------------|
| All data nodes + all meta-nodes |
| All data relationships + all meta-relationships + OF_KIND bridges |
| ~104 meta-nodes + ~386 meta-rels + data + OF_KIND |

**CLI**: `pnpm graph:overlay`
**Studio**: Toggle button "Show Meta" (overlays meta-graph on current data view)

### Mode 4: Query-Driven Views

Combine facets (Realm, Layer, Trait) to create custom subgraph views.
This is the key power feature — no hardcoding, fully dynamic.

#### Filter by Realm

```cypher
-- All nodes in the "project" realm
MATCH (k:Kind)-[:IN_REALM]->(:Realm {key: 'project'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN instance
```

**CLI**: `pnpm graph:query --realm=project`

#### Filter by Trait

```cypher
-- All localized nodes (generated content)
MATCH (k:Kind)-[:EXHIBITS]->(:Trait {key: 'localized'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN instance
```

**CLI**: `pnpm graph:query --trait=localized`

#### Filter by Layer

```cypher
-- All nodes in the "semantic" layer
MATCH (k:Kind)-[:IN_LAYER]->(:Layer {key: 'semantic'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN instance
```

**CLI**: `pnpm graph:query --layer=semantic`

#### Filter by EdgeFamily

```cypher
-- All edges in the "generation" family
MATCH (ek:EdgeKind)-[:IN_FAMILY]->(:EdgeFamily {key: 'generation'})
RETURN ek.key AS edge, ek.llm_context AS description
```

**CLI**: `pnpm graph:query --edge-family=generation`

#### Combo Filters

```cypher
-- Knowledge nodes in the global realm (Realm + Trait + Layer combo)
MATCH (k:Kind)-[:IN_REALM]->(:Realm {key: 'global'})
MATCH (k)-[:EXHIBITS]->(:Trait {key: 'knowledge'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN labels(instance)[0] AS type, count(*) AS count
```

**CLI**: `pnpm graph:query --realm=global --trait=knowledge`

### TS CLI (`@novanet/cli`) — scripts & CI

Simple pnpm commands for automation, CI pipelines, and quick checks.
Lives in the monorepo, shares types with `@novanet/core`.

| Command | Description | Output |
|---------|------------|--------|
| `pnpm graph:data` | Mode 1: Data only | Node/edge counts by type |
| `pnpm graph:meta` | Mode 2: Meta only | Taxonomy tree (Realm > Layer > Kind) |
| `pnpm graph:overlay` | Mode 3: Data + Meta | Combined stats |
| `pnpm graph:query` | Mode 4: Facet filters | Filtered subgraph |

**`pnpm graph:query` options:**

| Flag | Values | Description |
|------|--------|-------------|
| `--realm` | `global`, `project`, `shared` | Filter by Realm |
| `--layer` | `config`, `knowledge`, ...9 values | Filter by Layer |
| `--trait` | `invariant`, `localized`, `knowledge`, `derived`, `job` | Filter by Trait |
| `--edge-family` | `ownership`, `localization`, `semantic`, `generation`, `mining` | Filter by EdgeFamily |
| `--kind` | Any of the 35 Kind labels | Filter by specific Kind |
| `--format` | `table`, `json`, `cypher` | Output format (default: `table`) |

All flags are composable: `--realm=project --trait=localized` returns the intersection.

### Rust TUI (`novanet-tui`) — interactive exploration

Standalone binary for interactive graph exploration. Built with
[ratatui](https://ratatui.rs/) + [neo4rs](https://docs.rs/neo4rs) (Bolt driver).

Lives in `tools/novanet-tui/` outside the pnpm monorepo.

#### Layout

```
╔══════════════════════════════════════════════════════════════════════════════╗
║  NovaNet TUI            [ Data ] [ Meta ] [ Overlay ] [ Query ]    ?:help  ║
╠═══════════════════╦══════════════════════════════════════════════════════════╣
║  📁 TAXONOMY TREE ║  MAIN VIEW                                             ║
║  ───────────────  ║  ──────────────────────────────────────────────────     ║
║  🌍 global        ║  Kind: Page                                            ║
║  ├─ ⚙️ config     ║  Realm: project │ Layer: structure │ Trait: invariant   ║
║  │  └─ Locale     ║  ──────────────────────────────────────────────────     ║
║  └─ 📚 knowledge  ║  Outgoing edges:                                       ║
║     ├─ LocaleId.. ║  ├─ HAS_BLOCK → Block (ownership, 1:N)                ║
║     ├─ LocaleVo.. ║  ├─ HAS_OUTPUT → BlockL10n, PageL10n (localization)   ║
║     └─ ...        ║  ├─ HAS_PROMPT → BlockPrompt, PagePrompt (generation) ║
║  📦 project       ║  └─ OF_TYPE → BlockType, PageType (ownership)         ║
║  ├─ 🏛️ foundation ║  ──────────────────────────────────────────────────     ║
║  │  ├─ Project    ║  Instances: 4 Page nodes                               ║
║  │  ├─ BrandId.. ║  ├─ pricing (4 blocks, 3 concepts)                     ║
║  │  └─ Project.. ║  ├─ features (3 blocks, 2 concepts)                    ║
║  ├─ 🏗️ structure  ║  ├─ home (4 blocks, 5 concepts)                       ║
║  │  ├─ Page  ◄── ║  └─ api-docs (2 blocks, 3 concepts)                   ║
║  │  └─ Block     ║                                                         ║
║  └─ ...          ║                                                         ║
╠═══════════════════╩══════════════════════════════════════════════════════════╣
║  Realm: project (3) │ Layer: structure (2) │ Trait: invariant (11)          ║
║  ↑↓: navigate  Enter: select  Tab: switch pane  q: quit  /: search        ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

#### Features

| Feature | Description |
|---------|-------------|
| **Taxonomy tree** | Left pane: Realm > Layer > Kind hierarchy, collapsible |
| **Mode toggle** | `1`/`2`/`3`/`4` keys switch Data/Meta/Overlay/Query modes |
| **Kind detail** | Right pane: selected Kind with facets, edges, instance count |
| **Facet filters** | `f` key opens filter popup (Realm/Layer/Trait checkboxes) |
| **Search** | `/` key: fuzzy search across Kind labels and EdgeKind keys |
| **Instance drill-down** | `Enter` on a Kind shows its data instances |
| **Edge explorer** | `e` key: browse EdgeKind → FROM_KIND/TO_KIND schema |
| **Cypher preview** | `c` key: show the Cypher query behind the current view |

#### Tech Stack

| Crate | Purpose |
|-------|---------|
| `ratatui` | Terminal UI framework |
| `crossterm` | Cross-platform terminal backend |
| `neo4rs` | Neo4j Bolt driver (async) |
| `tokio` | Async runtime |
| `clap` | CLI argument parsing (--uri, --user, --password) |
| `serde` / `serde_json` | Neo4j result deserialization |

#### Connection

```bash
# Default (same as Docker dev)
novanet-tui

# Custom connection
novanet-tui --uri bolt://localhost:7687 --user neo4j --password novanetpassword
```

### Studio Implementation

Studio gains a **navigation panel** with mode toggles and facet filters.

#### Mode Toggle (toolbar)

```
[ Data Only ] [ Meta Only ] [ Overlay ] [ Query ]
     ●
```

One toggle active at a time. "Data Only" is the default (current behavior).

#### Facet Filter Panel (sidebar, active in Query mode)

```
┌─ Realm ──────────────┐
│ ☑ global  ☑ project  │
│ ☑ shared             │
└──────────────────────┘
┌─ Layer ──────────────┐
│ ☑ config  ☑ knowledge│
│ ☑ foundation ...     │
└──────────────────────┘
┌─ Trait ──────────────┐
│ ☑ invariant          │
│ ☑ localized          │
│ ☑ knowledge          │
│ ☑ derived  ☑ job     │
└──────────────────────┘
┌─ EdgeFamily ─────────┐
│ ☑ ownership          │
│ ☑ localization       │
│ ☑ semantic           │
│ ☑ generation         │
│ ☑ mining             │
└──────────────────────┘
```

Checkboxes are populated from the meta-graph (`MATCH (n:Meta) ...`).
Unchecking a facet filters out matching nodes/edges dynamically.

#### Files to Create (Studio)

| File | Description |
|------|-------------|
| `src/components/toolbar/NavigationModeToggle.tsx` | Mode 1-4 toggle buttons |
| `src/components/sidebar/FacetFilterPanel.tsx` | Realm/Layer/Trait/EdgeFamily checkboxes |
| `src/stores/navigationStore.ts` | Active mode + selected facets state |
| `src/hooks/useNavigationMode.ts` | Mode-aware Cypher query builder |
| `src/app/api/graph/navigation/route.ts` | API endpoint for filtered subgraph |

## Instance Bridge: OF_KIND

### Performance

Each data instance gets one `OF_KIND` relationship to its Kind node.
Estimated production volume: ~5,000-10,000 OF_KIND relationships
(200 locales x ~20 knowledge nodes + project/shared instances).
This is negligible for Neo4j.

### Autowiring strategy

Same pattern as v8.3.0 `99-autowire-subcategories.cypher`, renamed:

```cypher
// 35 statements, one per Kind
MATCH (n:Page)
MATCH (k:Kind {label: 'Page'})
MERGE (n)-[:OF_KIND]->(k);

MATCH (n:Block)
MATCH (k:Kind {label: 'Block'})
MERGE (n)-[:OF_KIND]->(k);
// ... for all 35 node types
```

Runs after all other seeds. Idempotent via MERGE. ~50ms total.

### Why OF_KIND instead of just labels

Without OF_KIND, finding "all instances in the semantic layer" requires
hardcoding label names:
```cypher
-- BAD: hardcoded, fragile
MATCH (n) WHERE labels(n)[0] IN ['Concept', 'ConceptL10n'] RETURN n
```

With OF_KIND, the query is schema-driven:
```cypher
-- GOOD: self-describing
MATCH (k:Kind)-[:IN_LAYER]->(:Layer {key:'semantic'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN instance
```

## Edge Cases

### Self-referential edges

Several relationship types connect a Kind to itself:

| EdgeKind | Kind |
|----------|------|
| SEMANTIC_LINK | Concept → Concept |
| FALLBACK_TO | Locale → Locale |
| VARIANT_OF | Locale → Locale |
| LINKS_TO | Page → Page |
| SUBTOPIC_OF | Page → Page |
| PREVIOUS_VERSION | BlockL10n → BlockL10n |
| PREVIOUS_VERSION | PageL10n → PageL10n |

For these, `FROM_KIND` and `TO_KIND` point to the same Kind node.
`EdgeKind.is_self_referential = true`.

### Polymorphic edges (multiple sources/targets)

Several relationship types have multiple valid source or target Kinds:

| EdgeKind | Sources | Targets |
|----------|---------|---------|
| HAS_OUTPUT | Block, Page | BlockL10n, PageL10n |
| HAS_PROMPT | Block, Page | BlockPrompt, PagePrompt |
| GENERATED | BlockPrompt, PagePrompt | BlockL10n, PageL10n |
| OF_TYPE | Block, Page | BlockType, PageType |
| HAS_METRICS | SEOKeywordL10n, GEOSeedL10n | SEOKeywordMetrics, GEOSeedMetrics |
| HAS_L10N | Concept, Project | ConceptL10n, ProjectL10n |
| PREVIOUS_VERSION | BlockL10n, PageL10n | BlockL10n, PageL10n |

These are handled naturally by the multi-hop FROM_KIND/TO_KIND pattern:
each EdgeKind has N `FROM_KIND` and M `TO_KIND` relationships.

### Relationship properties

Some Neo4j relationships carry properties (not just connect nodes):

| EdgeKind | Properties |
|----------|-----------|
| SEMANTIC_LINK | `temperature`, `semantic_field` |
| USES_CONCEPT | `temperature` |
| HAS_BLOCK | `position` |
| TARGETS_SEO | `status` |
| TARGETS_GEO | `status` |
| GENERATED | `generated_at` |

These are captured in `EdgeKind.edge_properties: string[]`.

### Wildcard sources in relations.yaml

The v8.3.0 `relations.yaml` uses `from: "*"` for `FOR_LOCALE`:

```yaml
FOR_LOCALE: { from: "*", to: Locale }
# Used by: ConceptL10n, ProjectL10n, BlockL10n, PageL10n, SEOKeywordL10n, GEOSeedL10n
```

The wildcard has no representation in the EdgeKind schema — `FROM_KIND` must
point to a concrete Kind node. The generator MUST expand `"*"` into explicit
`FROM_KIND` relationships using the comment or the trait-based heuristic:

```
(:EdgeKind {key:'FOR_LOCALE'})
  -[:FROM_KIND]-> (:Kind {label:'ConceptL10n'})
  -[:FROM_KIND]-> (:Kind {label:'ProjectL10n'})
  -[:FROM_KIND]-> (:Kind {label:'BlockL10n'})
  -[:FROM_KIND]-> (:Kind {label:'PageL10n'})
  -[:FROM_KIND]-> (:Kind {label:'SEOKeywordL10n'})
  -[:FROM_KIND]-> (:Kind {label:'GEOSeedL10n'})
  -[:TO_KIND]->   (:Kind {label:'Locale'})
  -[:IN_FAMILY]-> (:EdgeFamily {key:'localization'})
```

**Implementation**: Replace the wildcard in `relations.yaml` with an explicit
source list during the v9 migration:

```yaml
- type: FOR_LOCALE
  family: localization
  source:
    - ConceptL10n
    - ProjectL10n
    - BlockL10n
    - PageL10n
    - SEOKeywordL10n
    - GEOSeedL10n
  target: Locale
  cardinality: many_to_one
  llm_context: "Links localized content to its target Locale."
```

The multi-source list is the same pattern as HAS_OUTPUT, HAS_PROMPT, etc.

### Vector embeddings

Some Kinds have vector embedding properties (Concept: 1536d OpenAI,
Expression: 768d LaBSE). This metadata stays in the individual YAML files
and is not replicated in the meta-graph. YAGNI.

## Testing Strategy

### 1. Schema sync tests (adapt existing)

- YAML ↔ TypeScript sync (all 35 node types)
- YAML ↔ Neo4j sync (meta-graph matches YAML)
- Every node YAML has `locale_behavior` field
- Every relation in relations.yaml has `family` field

### 2. Meta-graph integrity tests (new)

```cypher
-- Every Kind has exactly 1 IN_REALM, 1 IN_LAYER, 1 EXHIBITS
MATCH (k:Kind)
OPTIONAL MATCH (k)-[:IN_REALM]->(r:Realm)
OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (k)-[:EXHIBITS]->(t:Trait)
WITH k, count(DISTINCT r) AS realms, count(DISTINCT l) AS layers, count(DISTINCT t) AS traits
WHERE realms <> 1 OR layers <> 1 OR traits <> 1
RETURN k.label AS broken_kind, realms, layers, traits
```

```cypher
-- Every EdgeKind has >= 1 FROM_KIND, >= 1 TO_KIND, exactly 1 IN_FAMILY
MATCH (ek:EdgeKind)
OPTIONAL MATCH (ek)-[:FROM_KIND]->(src:Kind)
OPTIONAL MATCH (ek)-[:TO_KIND]->(tgt:Kind)
OPTIONAL MATCH (ek)-[:IN_FAMILY]->(f:EdgeFamily)
WITH ek, count(DISTINCT src) AS sources, count(DISTINCT tgt) AS targets, count(DISTINCT f) AS families
WHERE sources < 1 OR targets < 1 OR families <> 1
RETURN ek.key AS broken_edge, sources, targets, families
```

```cypher
-- Full hierarchy path exists for all 35 Kinds
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
WITH count(k) AS connected
MATCH (k2:Kind) WITH connected, count(k2) AS total
WHERE connected <> total
RETURN 'HIERARCHY BROKEN' AS status, connected, total
```

### 3. Generator tests (adapt existing)

- OrganizingPrinciplesGenerator produces valid v9 Cypher
- Generated Cypher uses MERGE (idempotent)
- All 6 meta-node types are created
- All meta-relationships are created

### 4. Studio API tests (adapt existing)

- `/api/graph/organizing-principles` returns v9 format
- Response includes realms, layers, kinds, traits
- Correct Realm→Layer→Kind hierarchy

### 5. Performance benchmarks

Target latencies for the 4 navigation modes (on local Neo4j with seed data):

| Query | Target | Description |
|-------|--------|-------------|
| Mode 1: Data Only (`WHERE NOT n:Meta`) | < 100ms | 10,000 data nodes |
| Mode 2: Meta Only (`MATCH (n:Meta)`) | < 10ms | ~104 meta-nodes |
| Mode 3: Overlay (`MATCH (n)`) | < 200ms | All data + meta combined |
| Mode 4: Facet query (single facet) | < 100ms | OF_KIND traversal |
| Mode 4: Combo query (2+ facets) | < 200ms | Multi-facet intersection |
| OF_KIND autowiring (35 MERGE statements) | < 100ms | Idempotent bulk wiring |

### 6. Test coverage targets

| Area | Coverage Target |
|------|----------------|
| Meta-graph integrity tests | 100% (all 6 meta-node types) |
| Generator tests | 100% (all generated artifacts match YAML) |
| Schema sync tests | 100% (YAML ↔ TS ↔ Neo4j roundtrip) |
| Studio navigation modes | 100% (all 4 modes) |
| TS CLI commands | 100% (all 4 graph commands) |

## Migration Strategy

### Approach: Clean slate

This is a **breaking change** (v8.3 → v9.0). No incremental migration.
All dev data lives in seed files, so a clean rebuild is safe and simple.

```bash
# 1. Stop and destroy
pnpm infra:down
docker volume rm novanet_neo4j_data

# 2. Update source files (YAML, generators, Studio)
# ... (implementation work)

# 3. Regenerate artifacts
pnpm schema:generate

# 4. Rebuild and seed
pnpm infra:up
pnpm infra:seed

# 5. Verify
# Run meta-graph integrity queries above
```

### TypeScript migration

#### Problem: Triple duplication

v8.3.0 has **4 separate classification systems** for the same 35 node types:

- `NODE_TYPES` (35 entries, list)
- `NODE_CATEGORIES` (6 visual categories) — **duplicated in 3 files** (core, Studio filterAdapter, Studio nodeTypes)
- `NODE_BEHAVIORS` (35 mappings to LocaleBehavior)
- `NODE_SCOPES` (35 mappings to Scope)
- `NODE_SUBCATEGORIES` (35 mappings to Subcategory, generated)

Plus `SCOPE_HIERARCHY` in hierarchy.ts is hardcoded instead of generated.

#### v9 approach: Unified KIND_META + kill NodeCategory

**Kill NodeCategory entirely.** Replace with 9 Layers everywhere. The 6 old categories
(project, content, locale, generation, seo, geo) merged 2 Layers each — the 9 Layers
are the true classification and there's no reason to maintain a secondary grouping.

**Unify 3 separate records** into a single `KIND_META`:

```typescript
// GENERATED from models/ — do not edit manually
export type Realm = 'global' | 'project' | 'shared';
export type Layer = 'config' | 'knowledge' | 'foundation' | 'structure'
                  | 'semantic' | 'instruction' | 'output' | 'seo' | 'geo';
export type Trait = 'invariant' | 'localized' | 'knowledge' | 'derived' | 'job';

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

// Single source of truth — all 35 node facets in one record
export const KIND_META: Record<NodeType, KindMeta> = {
  Page:       { realm: 'project', layer: 'structure',  trait: 'invariant' },
  Block:      { realm: 'project', layer: 'structure',  trait: 'invariant' },
  ConceptL10n:{ realm: 'project', layer: 'semantic',   trait: 'localized' },
  // ... 35 entries, all generated from YAML
};

// Derived lookups (computed, not maintained separately)
function deriveMap<K extends keyof KindMeta>(field: K): Record<NodeType, KindMeta[K]> {
  return Object.fromEntries(
    Object.entries(KIND_META).map(([k, v]) => [k, v[field]])
  ) as Record<NodeType, KindMeta[K]>;
}
export const NODE_REALMS = deriveMap('realm');   // replaces NODE_SCOPES
export const NODE_TRAITS = deriveMap('trait');    // replaces NODE_BEHAVIORS
export const NODE_LAYERS = deriveMap('layer');    // replaces NODE_SUBCATEGORIES

// NODE_CATEGORIES is DELETED — use Layer directly
// NODE_TYPES remains for convenience
export const NODE_TYPES = Object.keys(KIND_META) as NodeType[];
```

**Benefits:**
- Adding a node = add 1 line in YAML → generator creates 1 entry in KIND_META
- Impossible to have inconsistent realm/layer/trait across 3 separate records
- Studio imports `Layer` directly instead of maintaining its own copy of NodeCategory

#### Breaking: PascalCase → lowercase casing change

The v8.3.0 `Scope` type uses **PascalCase** values (`'Global' | 'Shared' | 'Project'`).
The v9 `Realm` type uses **lowercase** values (`'global' | 'project' | 'shared'`).

This is a breaking change that ripples through:

| Area | v8.3.0 (PascalCase) | v9.0.0 (lowercase) |
|------|---------------------|---------------------|
| TypeScript type | `type Scope = 'Global' \| 'Shared' \| 'Project'` | `type Realm = 'global' \| 'project' \| 'shared'` |
| TypeScript `LocaleBehavior` | `'localeKnowledge'` (camelCase) | `'knowledge'` (flat) |
| Studio `SCOPE_COLORS` | `Record<Scope, {...}>` with PascalCase keys | `Record<Realm, {...}>` with lowercase keys |
| Neo4j Scope.key | `'Global'`, `'Project'`, `'Shared'` | `'global'`, `'project'`, `'shared'` |
| Cypher seed queries | `(:Scope {key: 'Global'})` | `(:Realm {key: 'global'})` |

**Impact**: All Studio components using `SCOPE_COLORS`, all Cypher queries filtering on
scope values, and all TypeScript code switching on `Scope` or `LocaleBehavior` values.
Since this is a clean-slate migration, the casing change can be applied atomically.

## Implementation Scope

### Files to Create / Modify

Total: ~125 files across 5 packages + 1 Rust tool + docs + Claude config. Grouped by package.

#### @novanet/core (YAML + types + filters)

| File | Action | Description |
|------|--------|-------------|
| `models/organizing-principles.yaml` | Rewrite | v9 format with realms/layers/traits/edge_families |
| `models/relations.yaml` | **Full rewrite** | Dict→list format + `family` + multi-source/target |
| `models/nodes/**/*.yaml` (35 files) | Migrate | Add `locale_behavior`, remove `category`, fix stale headers |
| `models/_index.yaml` | Update | Bump version, update scope→realm terminology |
| `models/relations/in-subcategory.yaml` | Delete | Replaced by OF_KIND (generated, not hand-written) |
| `src/types/nodes.ts` | Generate from YAML | Replace hardcoded Scope/LocaleBehavior with Realm/Trait |
| `src/types/index.ts` | Update exports | Realm/Layer/Trait instead of Scope/Subcategory |
| `src/graph/subcategories.ts` | Rename → `layers.ts` | Generated mapping: Kind → Layer |
| `src/graph/types.ts` | Rewrite | Subcategory → Layer, Scope → Realm in GraphNode interface |
| `src/graph/index.ts` | Update exports | Updated module exports |
| `src/graph/hierarchy.ts` | Generate | Auto-generated by HierarchyGenerator (was hardcoded in v8) |
| `src/graph/generator.ts` | Update | Update for new meta-node types |
| `src/filters/types.ts` | Update | Kill `NodeCategory` (use Layer directly), rename ViewCategory `'scope'`→`'overview'` |
| `src/filters/CypherGenerator.ts` | Update | Kill `NodeCategory` expansion logic → use Layer directly |
| `src/filters/NovaNetFilter.ts` | Update | Kill `NodeCategory` filtering → use Layer directly |
| `src/graph/__tests__/subcategories.test.ts` | Rename → `layers.test.ts` | Update assertions for Layer mapping |
| `src/graph/__tests__/hierarchy.test.ts` | Update | Realm/Layer assertions |
| `src/graph/__tests__/types.test.ts` | Update | New type definitions |
| `src/graph/__tests__/generator.test.ts` | Update | v9 generator output |
| `src/__tests__/schema-sync.test.ts` | Update | Sync tests for v9 fields |
| `src/__tests__/v710-conventions.test.ts` | Update | Convention tests for new terms |

#### @novanet/schema-tools (generators + parsers)

| File | Action | Description |
|------|--------|-------------|
| `src/generators/OrganizingPrinciplesGenerator.ts` | Rewrite | Generate v9 Cypher for all 6 meta-node types |
| `src/generators/SubcategoryGenerator.ts` | Rename → `LayerGenerator.ts` | Generate Layer mapping TypeScript |
| `src/generators/MermaidGenerator.ts` | Update | scope→realm, category→layer in node coloring |
| `src/parsers/RelationsParser.ts` | Restructure | Parse new list format + `family` field |
| `src/config/colors.ts` | Update | `localeKnowledge`→`knowledge` in all color maps |
| `src/generators/KindGenerator.ts` | Create | New generator for Kind nodes + facette rels |
| `src/generators/EdgeSchemaGenerator.ts` | Create | New generator for EdgeKind + FROM_KIND/TO_KIND |
| `src/generators/AutowireGenerator.ts` | Create | New generator for OF_KIND statements |
| `src/generators/HierarchyGenerator.ts` | Create | New generator: organizing-principles.yaml → hierarchy.ts (realm→layer tree) |
| `src/index.ts` | Update exports | New generator/parser names |
| `src/__tests__/organizing-principles.test.ts` | Rewrite | v9 generator output assertions |
| `src/__tests__/sync.test.ts` | Update | v9 schema sync checks |

#### @novanet/db (seeds + queries)

| File | Action | Description |
|------|--------|-------------|
| `seed/00-constraints.cypher` | Update | Drop v8 meta constraints, add v9 (6 types) |
| `seed/00.5-organizing-principles.cypher` | Regenerated | v9 meta-graph seed (output of generator) |
| `seed/99-autowire-subcategories.cypher` | Rename → `99-autowire-kinds.cypher` | `IN_SUBCATEGORY` → `OF_KIND` wiring |
| `queries/complete-graph.cypher` | Audit | May reference Scope/Subcategory labels |
| `queries/global-layer.cypher` | Audit | May reference Scope/Subcategory labels |
| `queries/project-layer.cypher` | Audit | May reference Scope/Subcategory labels |
| `queries/shared-layer.cypher` | Audit | May reference Scope/Subcategory labels |
| `migrations/001–006` | Archive | v8 migrations superseded by clean-slate rebuild |

#### @novanet/studio (UI + API)

| File | Action | Description |
|------|--------|-------------|
| `src/app/api/graph/organizing-principles/route.ts` | Rewrite queries | Realm/Layer/Kind/Trait Cypher |
| `src/hooks/useMagneticData.ts` | Rename fields | scope_key→realm_key, subcategory→layer |
| `src/hooks/useMagneticSimulation.ts` | Update | Scope→Realm references in simulation logic |
| `src/hooks/useFilteredGraph.ts` | Update | Scope/Subcategory → Realm/Layer in filter logic |
| `src/components/graph/schema/ScopeGroupNode.tsx` | Rename → `RealmGroupNode.tsx` | Get colors from Realm.color |
| `src/components/graph/schema/SubcategoryGroupNode.tsx` | Rename → `LayerGroupNode.tsx` | |
| `src/components/graph/nodes/ScopeAttractorNode.tsx` | Rename → `RealmAttractorNode.tsx` | |
| `src/components/graph/nodes/SubcategoryAttractorNode.tsx` | Rename → `LayerAttractorNode.tsx` | |
| `src/components/graph/nodes/LocaleKnowledgeNode.tsx` | Update | localeKnowledge → knowledge |
| `src/components/graph/nodes/index.ts` | Update exports | New node component names |
| `src/components/graph/Graph2D.tsx` | Update | Scope→Realm references |
| `src/components/graph/schema/SchemaNode.tsx` | Update | `subcategory` → `layer` in SchemaNodeData interface |
| `src/design/tokens.ts` | Update | `scopeAccents` → `realmAccents` object rename |
| `src/hooks/useGraphInteractions.ts` | Update | Scope z-index constants + `scope-{Scope}` ID parsing |
| `src/components/graph/edges/MagneticEdge.tsx` | Audit | May reference scope/subcategory |
| `src/components/sidebar/SchemaFilterPanel.tsx` | Update | Scope/Subcategory → Realm/Layer filters |
| `src/components/sidebar/ViewPicker.tsx` | Audit | May reference old terms |
| `src/components/query/ResultsOverview.tsx` | Update | Scope→Realm in results display |
| `src/stores/filterStore.ts` | Rename fields | collapsedScopes→collapsedRealms, etc. |
| `src/config/nodeTypes.ts` | Rewrite | Kill local `NodeCategory` copy → import Layer from core |
| `src/lib/filterAdapter.ts` | Rewrite | Kill local `NodeCategory` copy → import Layer from core, update groupings |
| `src/lib/schemaLayouts/magnetic.ts` | Update | Scope→Realm in layout calculations |
| `src/lib/schemaLayouts/stacked.ts` | Update | Hardcoded `Scope[]` array → Realm |
| `src/lib/schemaLayouts/elkLayered.ts` | Update | `Scope[]`, `ScopeLayout` interface, ID parsing |
| `src/lib/schemaLayouts/forceClusters.ts` | Update | `Record<Scope>` cluster centers → Realm |
| `src/lib/schemaLayouts/target.ts` | Update | `SubcategoryMeta` import, `Scope[]` → Realm |
| `src/lib/schemaLayouts/swimlanes.ts` | Update | Hardcoded `Scope[]` array → Realm |
| `src/lib/schemaLayouts/types.ts` | Update | Scope→Realm type references |
| `src/lib/schemaGenerator.ts` | Update | `SCOPE_DESCRIPTIONS` → `REALM_DESCRIPTIONS` |
| `src/config/viewCategories.ts` | Update | Scope→Realm icon mapping, rename ViewCategory `'scope'`→`'overview'` |
| `src/config/categoryColors.ts` | Rewrite | Kill 6 `NodeCategory` colors → 9 `Layer` colors |
| `src/stores/viewStore.ts` | Update | ViewCategoryGroup: `'scope'`→`'overview'` |
| `src/app/api/views/route.ts` | Update | ViewCategoryGroup: `'scope'`→`'overview'` |
| `src/schemas/view.schema.ts` | Update | ViewCategorySchema: `'scope'`→`'overview'` enum value |
| `tailwind.config.ts` | Update | `localeKnowledge` → `knowledge` color token |
| `src/app/page.tsx` | Update | scope-{Scope} container references |
| `src/components/graph/schema/__tests__/ScopeGroupNode.test.tsx` | Rename + update | |
| `src/components/sidebar/__tests__/SchemaFilterPanel.test.tsx` | Update | |
| `src/stores/__tests__/filterStore.test.ts` | Update | |
| `src/lib/__tests__/schemaLayoutELK.test.ts` | Update | Subcategory, localeKnowledge, PascalCase scope fixtures |
| `e2e/schema-mode.spec.ts` | Update | Scope/Subcategory assertions (3 Scopes, GLOBAL, etc.) |

##### Navigation Modes (new files)

| File | Action | Description |
|------|--------|-------------|
| `src/components/toolbar/NavigationModeToggle.tsx` | Create | Mode 1-4 toggle buttons (Data/Meta/Overlay/Query) |
| `src/components/sidebar/FacetFilterPanel.tsx` | Create | Realm/Layer/Trait/EdgeFamily checkboxes |
| `src/stores/navigationStore.ts` | Create | Active mode + selected facets state |
| `src/hooks/useNavigationMode.ts` | Create | Mode-aware Cypher query builder |
| `src/app/api/graph/navigation/route.ts` | Create | API endpoint for filtered subgraph queries |

#### @novanet/cli (TS graph commands)

| File | Action | Description |
|------|--------|-------------|
| `src/commands/graph-data.ts` | Create | Mode 1: Data-only node/edge counts |
| `src/commands/graph-meta.ts` | Create | Mode 2: Meta-only taxonomy tree |
| `src/commands/graph-overlay.ts` | Create | Mode 3: Combined data+meta stats |
| `src/commands/graph-query.ts` | Create | Mode 4: Facet filter with --realm/--layer/--trait/--edge-family/--kind/--format |

#### tools/novanet-tui (Rust interactive TUI)

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Create | Dependencies: ratatui, crossterm, neo4rs, tokio, clap, serde |
| `src/main.rs` | Create | Entry point, Bolt connection, clap args |
| `src/app.rs` | Create | App state: active mode, selected Kind, facet filters |
| `src/ui/mod.rs` | Create | Layout: left tree + right detail + status bar |
| `src/ui/taxonomy_tree.rs` | Create | Realm > Layer > Kind collapsible tree widget |
| `src/ui/kind_detail.rs` | Create | Kind facets, edges, instance list panel |
| `src/ui/facet_filter.rs` | Create | Popup with Realm/Layer/Trait/EdgeFamily checkboxes |
| `src/ui/mode_bar.rs` | Create | Top bar with Data/Meta/Overlay/Query toggle |
| `src/db/mod.rs` | Create | Neo4j queries: meta-graph, data counts, filtered views |
| `src/db/queries.rs` | Create | Cypher query builder for the 4 modes + facet combos |

#### Turbo Generators (scaffolding templates)

| File | Action | Description |
|------|--------|-------------|
| `turbo/generators/config.ts` | Update | `localeKnowledge`→`knowledge` in scaffold options |
| `turbo/generators/templates/node.yaml.hbs` | Update | Add `locale_behavior`, remove `category` |
| `turbo/generators/templates/view.yaml.hbs` | Update | `localeKnowledge`→`knowledge` in template |

#### Documentation + Claude Config

| File | Action | Description |
|------|--------|-------------|
| `core/models/docs/views/VIEW-COMPLETE-GRAPH.md` | Regenerated | New Mermaid with v9 labels |
| `core/models/docs/views/VIEW-*.md` (12 files) | Regenerated | Updated by MermaidGenerator |
| `core/CLAUDE.md` | Update | v8.2→v9, Scope→Realm terminology |
| `CLAUDE.md` (root) | Update | Version bump |
| `apps/studio/CLAUDE.md` | Update | New component names |
| `packages/core/models/_index.yaml` | Update | v8→v9 terminology in index doc |
| `packages/core/models/README.md` | Update | LocaleKnowledge → knowledge Layer reference |
| `packages/core/docs/plans/2026-01-30-unified-types-edge-styles.md` | Update | NodeCategory type, PascalCase scopes, localeKnowledge |
| `docs/NOVANET-PITCH.md` | Update | Subcategory→Layer, LocaleKnowledge→knowledge in architecture |
| `apps/studio/docs/plans/2026-01-31-schema-layouts.md` | Update | Scope→Realm, Subcategory→Layer in layout docs |
| `apps/studio/docs/plans/2026-01-31-schema-view-interactive.md` | Update | Subcategory→Layer terminology |
| `apps/studio/.claude/rules/novanet-terminology.md` | Rewrite | v8.2.0 terminology → v9.0.0 (Realm/Layer/Kind/Trait) |
| `.claude/README.md` | Update | "3 Scopes" → "3 Realms" + v9 terminology |
| `.claude/skills/novanet-architecture/SKILL.md` | Update | Scope→Realm, Subcategory→Layer, SubcategoryGenerator→LayerGenerator |
| `.claude/skills/novanet-sync/SKILL.md` | Update | SubcategoryGenerator→LayerGenerator references |
| `.claude/commands/novanet-arch.md` | Audit | May reference v8 meta-node names |
| `.claude/commands/novanet-sync.md` | Audit | May reference v8 generator names |

### Rollback Strategy

Before starting implementation:
- Tag current codebase: `git tag v8.3.0-stable`
- Create feature branch: `git checkout -b feat/ontology-v9`
- All work happens on the feature branch — `main` stays on v8.3.0
- If migration fails mid-implementation: `git checkout main` restores v8.3.0

### Implementation Steps

0. **Rollback prep**: `git tag v8.3.0-stable && git checkout -b feat/ontology-v9`
1. Add `locale_behavior` to all 35 node YAML files (fix inconsistency)
2. **Full rewrite** of `relations.yaml` (dict→list format + `family` + multi-source/target)
3. Rewrite `organizing-principles.yaml` to v9 format
4. Rewrite `OrganizingPrinciplesGenerator.ts` for v9
5. Run `pnpm schema:generate` to regenerate Cypher + TypeScript
6. Update `00-constraints.cypher` (drop old, add new)
7. Rename `99-autowire-subcategories.cypher` → `99-autowire-kinds.cypher`
8. Clean rebuild: `pnpm infra:down && docker volume rm ... && pnpm infra:up && pnpm infra:seed`
9. Run meta-graph integrity tests
10. **PascalCase→lowercase audit**: find-and-replace all `'Global'`→`'global'`, `'Project'`→`'project'`, `'Shared'`→`'shared'`, `'localeKnowledge'`→`'knowledge'` across TS, Cypher, Studio (~140 string literals)
11. Update Studio API + components (rename ScopeGroup → RealmGroup, etc.)
12. Align Studio visual categories to 9 Layers
13. Implement navigation mode toggle in Studio (NavigationModeToggle, navigationStore)
14. Implement facet filter panel in Studio (FacetFilterPanel, useNavigationMode)
15. Add navigation API endpoint (`/api/graph/navigation/route.ts`)
16. Implement TS CLI graph commands (`graph:data`, `graph:meta`, `graph:overlay`, `graph:query`)
17. Build Rust TUI (`tools/novanet-tui`) — **STRETCH GOAL**: ratatui + neo4rs, taxonomy tree, mode toggle, facet filters
18. Run full test suite
