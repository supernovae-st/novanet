# NovaNet Ontology Roadmap: v9 → v10 → v11 → v12

**Date**: 2026-02-01
**Status**: Draft
**Author**: Thibaut + Claude
**Supersedes**: 2026-01-31-organizing-principles.md (v8.3.0)

## Milestones Overview

| Milestone | Version | TrustGraph Level | Phases | Detail |
|-----------|---------|------------------|--------|--------|
| **1 — Self-Describing Context Graph** | v9.0 | Level 5 | 0–8 | Detailed tasks, file-level |
| **2 — Dynamic Retrieval** | v10.0 | Level 6 | 10–12 | High-level objectives + success criteria |
| **3 — Autonomous Learning** | v11.0 | Level 7 | 13–15 | High-level objectives + success criteria |
| **4 — Content Pipeline** | v12.0 | Level 8 | 16 | Placeholder — CLI-driven generation |

v9 builds the foundation — a self-describing meta-graph that AI agents can discover
autonomously. v10 makes context assembly dynamic. v11 closes the feedback loop so the
meta-graph learns from its own outputs. v12 exposes the full content pipeline through CLI.

## Summary

Refactor the NovaNet meta-graph from a flat tree (Scope > Subcategory > NodeTypeMeta)
to a **self-describing context graph** with 6 meta-node types and dual navigation
(hierarchy + facets).

NovaNet is a **context graph** — a knowledge graph enriched with operational metadata,
self-describing schema, and token-aware context assembly for AI agents. This positions
it beyond traditional KGs: the meta-graph carries enough information for an LLM
orchestrator to discover, query, and reason about the entire schema autonomously.

Key changes:
- Rename all meta-labels: Scope -> Realm, Subcategory -> Layer, NodeTypeMeta -> Kind
- Add 2 new meta-types: Trait (locale behavior), EdgeFamily + EdgeKind (relation ontology)
- Self-describing schema: `schema_hint` on Kind, `cypher_pattern` on EdgeKind, `context_budget` on Kind
- Instance bridge: replace `IN_SUBCATEGORY` with `OF_KIND`
- Dual navigation: top-down hierarchy + Kind-centric facets
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
color:        string  (hex color for Studio fills, e.g., '#3B82F6')
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
schema_hint:          string    (human-readable property summary for LLM context assembly)
context_budget:       string    ('high' | 'medium' | 'low' | 'minimal')
traversal_depth:      integer   (nullable — v10: max hops to follow from this Kind)
generation_count:     integer   (default 0 — v11: incremented on each generation)
created_at:           datetime
updated_at:           datetime
```

`schema_hint` — Auto-generated by KindGenerator from node YAML property definitions.
Provides a one-line summary like `"key, display_name, instructions (req), locale_behavior"`.
Enables an LLM orchestrator to understand what data a Kind carries without reading YAML.

`context_budget` — Token-aware priority for context window assembly. The orchestrator
uses this to decide how much of a Kind's data to include when building prompts:
- `high`: full properties + related nodes (e.g., Concept, Page)
- `medium`: key properties (e.g., BlockType, LocaleVoice)
- `low`: summary only (e.g., SEOKeywordMetrics, GEOSeedMetrics)
- `minimal`: key + display_name only (e.g., SEOMiningRun, GEOMiningRun)

Note: `context_budget` is a soft constraint (validated by generators, not by Neo4j).
No database-level enum constraint — values are enforced in KindGenerator + schema validate.

`traversal_depth` — **v10 preparation**. Nullable in v9 (ignored by orchestrator).
In v10, defines how many relationship hops to follow from instances of this Kind
when assembling context. Example: Concept = 2 (follow SEMANTIC_LINK 2 hops), Locale = 1.

`generation_count` — **v11 preparation**. Defaults to 0 in v9. In v11, incremented
each time an instance of this Kind is generated or regenerated. Enables frequency-based
analysis: "which Kinds are regenerated most often?" → candidates for context tuning.

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
color:        string  (hex, for Studio border accent — NOT fill; fill comes from Layer)
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
color:            string  (hex, for Studio edge colors)
arrow_style:      string  (Mermaid arrow syntax)
default_traversal: string  (nullable — 'always' | 'conditional' | 'on_demand')
created_at:       datetime
updated_at:       datetime
```

`default_traversal` — **v10 preparation**. Nullable in v9 (ignored by orchestrator).
In v10, tells the context assembly engine how to handle edges of this family:
- `'always'`: ownership edges — always follow (structural parents/children)
- `'conditional'`: semantic edges — follow only if temperature >= threshold
- `'on_demand'`: mining edges — follow only when task explicitly requests

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
cypher_pattern:          string    (traversal pattern, e.g. '(Page)-[:HAS_BLOCK]->(Block)')
temperature_threshold:   float     (nullable — v10: min temperature for conditional traversal)
created_at:              datetime
updated_at:              datetime
```

`cypher_pattern` — Auto-generated by EdgeSchemaGenerator from `source`/`target` in
relations.yaml. Provides a ready-to-use Cypher pattern like
`'(Block)-[:HAS_OUTPUT]->(BlockL10n, PageL10n)'`. An LLM orchestrator can use this
to construct traversal queries without reading the schema definition.

`temperature_threshold` — **v10 preparation**. Nullable in v9 (ignored by orchestrator).
In v10, used by the context assembly engine for `conditional` traversal edges
(semantic, mining families). Only follow this edge if its `temperature` property >= threshold.
Example: SEMANTIC_LINK threshold = 0.3, USES_CONCEPT threshold = 0.5.

### Data Node v10/v11 Preparations

Two output nodes receive nullable properties for the autonomous learning pipeline:

**PageL10n** and **BlockL10n** (Trait: `localized`, Layer: `output`):

```
# Added to existing YAML properties (nullable, ignored in v9)
quality_score:       float     (nullable — v11: 0.0–1.0 quality rating from evaluation)
prompt_fingerprint:  string    (nullable — v11: hash of the prompt that produced this output)
```

`quality_score` — **v11 preparation**. Nullable in v9 (not set). In v11, filled by the
evaluation pipeline after content review (human or automated). Score range 0.0–1.0.
Feeds back into `context_budget` tuning: low scores → increase context, high scores →
reduce context. Enables pattern analysis per locale × concept × block type.

`prompt_fingerprint` — **v11 preparation**. Nullable in v9 (not set). In v11, stores a
hash of the assembled prompt (context + instructions) that generated this output. Enables
A/B analysis: "same prompt, different locales → quality delta?" and cache invalidation
when prompts change. Format: SHA-256 truncated to 16 chars.

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
| `KindGenerator` | 35 node YAML files | Kind nodes (+ `schema_hint`, `context_budget`, `traversal_depth`, `generation_count`) + IN_REALM, IN_LAYER, EXHIBITS facets |
| `EdgeSchemaGenerator` | `relations.yaml` | EdgeKind nodes (+ `cypher_pattern`, `temperature_threshold`) + FROM_KIND, TO_KIND, IN_FAMILY, HAS_EDGE_KIND |
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
so the 6 constraints above provide 6 indexes for free.

#### Meta-graph relationship indexes (new)

Facet queries and hierarchy traversals need relationship indexes to avoid full scans.
Without these, combo queries (Mode 4) degrade from ~5ms to ~50ms at production volume.

```cypher
-- Facet filter indexes (Mode 4 queries)
CREATE INDEX kind_in_realm IF NOT EXISTS FOR ()-[r:IN_REALM]-() ON ();
CREATE INDEX kind_in_layer IF NOT EXISTS FOR ()-[r:IN_LAYER]-() ON ();
CREATE INDEX kind_exhibits IF NOT EXISTS FOR ()-[r:EXHIBITS]-() ON ();

-- Edge schema indexes
CREATE INDEX edge_from_kind IF NOT EXISTS FOR ()-[r:FROM_KIND]-() ON ();
CREATE INDEX edge_to_kind IF NOT EXISTS FOR ()-[r:TO_KIND]-() ON ();

-- Instance bridge (critical — ~5,000-10,000 rels at production)
CREATE INDEX of_kind IF NOT EXISTS FOR ()-[r:OF_KIND]-() ON ();

-- Hierarchy traversal indexes
CREATE INDEX realm_has_layer IF NOT EXISTS FOR ()-[r:HAS_LAYER]-() ON ();
CREATE INDEX layer_has_kind IF NOT EXISTS FOR ()-[r:HAS_KIND]-() ON ();

-- EdgeKind property index
CREATE INDEX edge_kind_self_ref IF NOT EXISTS FOR (ek:EdgeKind) ON (ek.is_self_referential);
```

#### Existing data indexes (unchanged)

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
OPTIONAL MATCH (k)<-[:FROM_KIND]-(ek:EdgeKind)-[:IN_FAMILY]->(f:EdgeFamily),
               (ek)-[:TO_KIND]->(target:Kind)
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

### Studio Visual System v9

v8 has **4 competing color systems** (Scope, Subcategory, NodeCategory, NodeType).
v9 replaces them with a single **3-channel encoding** that uses orthogonal visual
channels — each axis is readable independently.

#### Channel 1: COLOR → Layer (9 distinct colors)

Node fill color comes from its Layer. All 9 Layers have a unique hex color
(defined in `organizing-principles.yaml`). This replaces:
- v8 `SCOPE_COLORS` (3 colors) → subsumed by Realm spatial zones
- v8 `NodeCategory` colors (6 colors) → replaced by Layer colors (9 colors)
- v8 `NodeType` colors (35 individual) → individual accent via border only

#### Channel 2: SHAPE / BORDER → Trait (5 distinct styles)

Node border style encodes locale behavior (Trait). No color collision with Layer:

| Trait | Border Style | Description |
|-------|-------------|-------------|
| `invariant` | Solid 2px | Stable, structural nodes |
| `localized` | Dashed 2px | Generated per locale |
| `knowledge` | Double border | Cultural/linguistic expertise |
| `derived` | Dotted 1px | Computed aggregates |
| `job` | Thin + badge | Background tasks |

#### Channel 3: GROUPING / POSITION → Realm (3 spatial zones)

Realm controls spatial arrangement — not color. The 3 Realms map to visual zones
in the graph layout (top/left/right, or nested subgraphs). This preserves the
v8 scope-based grouping while freeing color for Layer.

#### Migration from v8

| v8 System | v9 Replacement | Where |
|-----------|---------------|-------|
| `SCOPE_COLORS` | Realm spatial zones (no color needed) | `tokens.ts`, layouts |
| `NODE_CATEGORY_COLORS` (6) | 9 Layer colors from meta-graph | `categoryColors.ts` → `layerColors.ts` |
| `NodeType` colors (35) | Layer color + Trait border | `NodeConfig.ts` simplified |
| `Trait.color` in YAML | Border accent only | `organizing-principles.yaml` |

### Studio Views System v9

v8 has 2 modes (`data` | `schema`) and 9 filter presets using dead `NodeCategory`.
v9 **subsumes** data/schema into 4 navigation modes and rewrites presets with facets.

#### DataMode → NavigationMode

```typescript
// v8
type DataMode = 'data' | 'schema';

// v9
type NavigationMode = 'data' | 'meta' | 'overlay' | 'query';
// data    = v8 'data' mode (WHERE NOT n:Meta)
// meta    = v8 'schema' mode (MATCH n:Meta)
// overlay = NEW: data + meta combined
// query   = NEW: faceted filtering
```

`uiStore.dataMode` → `uiStore.navigationMode`. Matrix Rain transition stays.

#### Filter Presets Migration

v8 presets use `NodeCategory` (dead). v9 presets use `Realm × Layer × Trait`:

| v8 Preset | v9 Replacement |
|-----------|---------------|
| 1: Project Structure | `realm:project, layer:foundation+structure` |
| 2: Generation Chain | `layer:semantic+output, trait:localized` |
| 3: Locale Knowledge | `realm:global, trait:knowledge` |
| 4: Concept Network | `layer:semantic` |
| 5: Prompts & Rules | `layer:instruction` |
| 6: SEO & GEO | `realm:shared` |
| 7: High Priority | **REMOVED** (priority property deleted in v8.2) |
| 8: Realtime | **REMOVED** (freshness property deleted in v8.2) |
| 0: All Nodes | No filter (unchanged) |

New presets enabled by v9:
- `T`: Trait toggle — cycle through invariant/localized/knowledge/derived/job
- `E`: Edge Family filter — show only edges of a given family

#### YAML Views

14 YAML views stay but category `'scope'` → `'overview'`. Views become
preset queries usable in any navigation mode. ViewPicker becomes
context-aware: shows YAML views in Data/Query mode, ontology views in Meta mode.

**Meta mode ontology views (5):**

| # | View Name | Cypher | Description |
|---|-----------|--------|-------------|
| M1 | Full Ontology | `MATCH (n:Meta) RETURN n` | All ~104 meta-nodes — the big picture (default Meta view) |
| M2 | Realms | `MATCH (r:Realm)<-[:IN_REALM]-(k:Kind) RETURN r, k` | 3 Realms + their Kinds, grouped |
| M3 | Layers | `MATCH (l:Layer)<-[:IN_LAYER]-(k:Kind) RETURN l, k` | 9 Layers + their Kinds |
| M4 | Traits | `MATCH (t:Trait)<-[:EXHIBITS]-(k:Kind) RETURN t, k` | 5 Traits + which Kinds exhibit them |
| M5 | Edge Schema | `MATCH (ef:EdgeFamily)<-[:IN_FAMILY]-(ek:EdgeKind) RETURN ef, ek` | 5 Families + 47 EdgeKinds with FROM/TO |

Data/Query mode: shows the 14 YAML data views.
Meta mode: shows M1–M5.
Overlay mode: shows both combined.

### Faceted Filter Strategy

v8 filter system: rigid tree (Scope → Subcategory → NodeCategory) with collapse/expand.
v9 filter system: **independent facets** with AND logic.

#### filterStore Migration

| v8 Field | v9 Replacement |
|----------|---------------|
| `collapsedScopes: Set<Scope>` | `collapsedRealms: Set<Realm>` |
| `collapsedSubcategories: Set<string>` | `collapsedLayers: Set<Layer>` |
| `categoryFilter: Set<NodeCategory>` | **DELETED** → facet checkboxes |
| `typeFilter: Set<NodeType>` | Stays (individual type toggle) |

#### FacetFilterPanel (new component)

Replaces the v8 SchemaFilterPanel tree with independent checkbox groups:
- **Realm** (3): global, project, shared
- **Layer** (9): config, knowledge, foundation, ...
- **Trait** (5): invariant, localized, knowledge, derived, job
- **EdgeFamily** (5): ownership, localization, semantic, generation, mining

All groups are populated dynamically from `MATCH (n:Meta)` — not hardcoded.
Unchecking a facet generates `WHERE NOT` clauses in the Cypher query.
Multiple unchecked facets compose with AND logic (intersection).

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

**CLI**: `novanet data`
**Studio**: Toggle button "Data Only" (default view)

### Mode 2: Meta Only

Show only the ontology schema — the "schema of the schema".

**Cypher**: `MATCH (n:Meta) RETURN n`

| What you see | What's hidden |
|-------------|--------------|
| 3 Realms, 9 Layers, 35 Kinds, 5 Traits, 5 EdgeFamilies, 47 EdgeKinds | All data instances |
| HAS_LAYER, HAS_KIND, IN_REALM, EXHIBITS, FROM_KIND, TO_KIND, IN_FAMILY | All data relationships |

**CLI**: `novanet meta`
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

**CLI**: `novanet overlay`
**Studio**: Toggle button "Show Meta" (overlays meta-graph on current data view)

**Visual distinction**: Meta-nodes render as **ghost style** — semi-transparent
(opacity 0.4), dashed border, 70% size. Data nodes keep full 3-channel encoding
(Layer fill + Trait border + Realm zone). The meta-graph becomes a subtle
"background map" showing where data lives in the taxonomy. Ghost style is
CSS-only (conditional on `node.data.isMeta`), no custom React Flow shapes needed.

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

**CLI**: `novanet query --realm=project`

#### Filter by Trait

```cypher
-- All localized nodes (generated content)
MATCH (k:Kind)-[:EXHIBITS]->(:Trait {key: 'localized'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN instance
```

**CLI**: `novanet query --trait=localized`

#### Filter by Layer

```cypher
-- All nodes in the "semantic" layer
MATCH (k:Kind)-[:IN_LAYER]->(:Layer {key: 'semantic'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN instance
```

**CLI**: `novanet query --layer=semantic`

#### Filter by EdgeFamily

```cypher
-- All edges in the "generation" family
MATCH (ek:EdgeKind)-[:IN_FAMILY]->(:EdgeFamily {key: 'generation'})
RETURN ek.key AS edge, ek.llm_context AS description
```

**CLI**: `novanet query --edge-family=generation`

#### Combo Filters

```cypher
-- Knowledge nodes in the global realm (single path, optimized)
MATCH (:Realm {key: 'global'})<-[:IN_REALM]-(k:Kind)-[:EXHIBITS]->(:Trait {key: 'knowledge'})
MATCH (k)<-[:OF_KIND]-(instance)
RETURN labels(instance)[0] AS type, count(*) AS count
```

**CLI**: `novanet query --realm=global --trait=knowledge`

### CLI-First Architecture

**Boundary rule**: TypeScript generates code artifacts (types, Cypher files, Mermaid).
Rust executes graph operations at runtime (read, write, query, assemble).
Studio keeps its own Neo4j JS driver (web API routes = separate concern).

```
novanet (Rust binary) = universal interface for graph operations
├── CLI mode   novanet data / meta / query / node create ...
├── TUI mode   novanet tui (interactive terminal)
├── AI agent   calls novanet query --format=json programmatically (v10+)
└── v10-v12    context assemble / eval / content generate

pnpm (TS monorepo) = code generation + web UI
├── pnpm schema:generate    YAML → TypeScript types (must be TS)
├── pnpm schema:validate    YAML ↔ TS ↔ Neo4j sync
├── pnpm dev                Studio web app (Next.js)
└── pnpm build/lint/test    Monorepo dev tools
```

An AI orchestrator calls `novanet query --kind=Page --trait=localized --format=json`
to discover what to generate before generating. ~5ms startup (vs ~200ms Node.js).
This is the "self-describing context graph" in action.

**No @novanet/cli package** — the Rust binary IS the CLI. Root package.json can
alias `"graph:data": "novanet data"` for convenience, but no TS wrapper code.

### Rust Binary (`novanet`) — CLI + TUI

Single crate binary for all graph operations. CLI subcommands and interactive TUI
share the same code (Neo4j client, Cypher builder, facet logic, output formatters).
Lives in `tools/novanet/` outside the pnpm monorepo.

#### Read Commands

| Command | Description | Output |
|---------|------------|--------|
| `novanet data` | Mode 1: Data only | Node/edge counts by type |
| `novanet meta` | Mode 2: Meta only | Taxonomy tree (Realm > Layer > Kind) |
| `novanet overlay` | Mode 3: Data + Meta | Combined stats |
| `novanet query` | Mode 4: Facet filters | Filtered subgraph |

**`novanet query` options:**

| Flag | Values | Description |
|------|--------|-------------|
| `--realm` | `global`, `project`, `shared` | Filter by Realm |
| `--layer` | `config`, `knowledge`, ...9 values | Filter by Layer |
| `--trait` | `invariant`, `localized`, `knowledge`, `derived`, `job` | Filter by Trait |
| `--edge-family` | `ownership`, `localization`, `semantic`, `generation`, `mining` | Filter by EdgeFamily |
| `--kind` | Any of the 35 Kind labels | Filter by specific Kind |
| `--format` | `table`, `json`, `cypher` | Output format (default: `table`) |

All flags are composable: `--realm=project --trait=localized` returns the intersection.

#### Write Commands

| Command | Description | Example |
|---------|------------|---------|
| `novanet node create` | Create a data node | `--kind=Page --key=about --props='{"display_name":"About"}'` |
| `novanet node edit` | Edit node properties | `--key=about --set='{"display_name":"About Us"}'` |
| `novanet node delete` | Delete a data node | `--key=about --confirm` |
| `novanet relation create` | Create a relationship | `--from=about --to=pricing --type=SEMANTIC_LINK` |
| `novanet relation delete` | Delete a relationship | `--from=about --to=pricing --type=SEMANTIC_LINK` |

Write commands validate against the meta-graph: `novanet node create --kind=Page`
checks that `Page` is a valid Kind, applies the correct Realm/Layer/Trait, and wires
`OF_KIND` automatically.

#### Utility Commands

| Command | Description |
|---------|------------|
| `novanet schema validate` | Validate Neo4j state matches YAML definitions |
| `novanet tui` | Launch interactive TUI mode |

#### Crate Structure (single crate)

```
tools/novanet/
├── Cargo.toml
└── src/
    ├── lib.rs              Public API: query, facets, meta types, cypher builder
    ├── main.rs             Thin entry: clap parsing → calls lib, formats, exits
    ├── error.rs            NovaNetError enum (thiserror) + color-eyre setup
    ├── config.rs           Connection config (--uri/--user/--password + env fallbacks)
    ├── db.rs               Neo4j connection pool (neo4rs, async)
    ├── cypher.rs           Cypher query builder (facet → WHERE clauses)
    ├── facets.rs           Realm/Layer/Trait/EdgeFamily filter logic
    ├── meta.rs             Meta-graph types (Kind, EdgeKind, etc.)
    ├── output.rs           Formatters: table (tabled), json (serde), cypher (raw)
    ├── validate.rs         Schema validation (Neo4j ↔ YAML)
    ├── commands/
    │   ├── mod.rs
    │   ├── data.rs         Mode 1: WHERE NOT n:Meta
    │   ├── meta.rs         Mode 2: MATCH (n:Meta)
    │   ├── overlay.rs      Mode 3: MATCH (n)
    │   ├── query.rs        Mode 4: facet-driven
    │   ├── node.rs         node create/edit/delete
    │   └── relation.rs     relation create/delete
    └── tui/
        ├── mod.rs
        ├── app.rs          App state machine (mode, selection, filters, loading)
        ├── ui.rs           Layout: left tree + right detail + status bar
        ├── events.rs       Crossterm keyboard/mouse events → Action enum
        ├── runtime.rs      Channel bridge: Action → tokio task → Result → App
        ├── tree.rs         Taxonomy tree widget (Realm > Layer > Kind)
        ├── detail.rs       Kind detail pane (facets, edges, instances)
        ├── search.rs       Nucleo fuzzy search integration (/ key)
        └── dialogs.rs      Input forms for node/relation CRUD (n/d/r keys)
```

No feature flags, no workspace. Single `cargo build` produces the `novanet` binary
with CLI + TUI. Split into workspace if crate grows past v10.

#### Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` + `clap_derive` | CLI argument parsing, subcommands |
| `ratatui` | Terminal UI framework |
| `crossterm` | Cross-platform terminal backend |
| `neo4rs` | Neo4j Bolt driver (async) — pin exact minor version |
| `tokio` | Async runtime (`features = ["full"]`) |
| `serde` + `serde_json` | Neo4j result deserialization, JSON output |
| `serde_yaml` | YAML parsing for schema validation (`validate.rs`) |
| `tabled` | Table output formatting |
| `nucleo` | Fuzzy search (TUI `/` key) |
| `thiserror` | Structured error enum (`NovaNetError`) for matching |
| `color-eyre` | Error reporting with context (wraps `thiserror` errors) |
| `tracing` + `tracing-subscriber` | Structured logging with env-filter output |

#### Connection

```bash
# Default (same as Docker dev)
novanet data
novanet tui

# Custom connection
novanet --uri bolt://localhost:7687 --user neo4j --password novanetpassword data
novanet --uri bolt://remote:7687 --user neo4j --password secret tui
```

#### TUI Layout

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
| **Node CRUD** | `n` key: create node, `d` key: delete node (wraps CLI write commands) |
| **Relation CRUD** | `r` key: create relation between selected nodes |

#### TUI Async Architecture

`neo4rs` is async (requires tokio runtime). `ratatui` runs a synchronous event loop
(blocking on `crossterm::event::poll`). These coexist via **channel bridge** (Pattern B):

```
┌──────────────────┐     mpsc::Sender<Action>     ┌──────────────────┐
│  TUI Event Loop  │ ──────────────────────────>   │  Tokio Runtime   │
│  (main thread)   │                               │  (async tasks)   │
│                  │     mpsc::Receiver<Result>     │                  │
│  crossterm poll  │ <──────────────────────────    │  neo4rs queries  │
│  + try_recv()    │                               │                  │
└──────────────────┘                               └──────────────────┘
```

The event loop uses `crossterm::event::poll(Duration::from_millis(50))` (non-blocking)
combined with `mpsc::try_recv()` on a results channel. This gives:
- **Responsive UI**: 50ms tick rate, never blocked on I/O
- **Async Neo4j**: queries run on tokio tasks in the background
- **Loading state**: `App` state machine has `Loading`/`Ready` variants

Implementation lives in `src/tui/runtime.rs` (channel bridge) and
`src/tui/events.rs` (crossterm events → Action enum).

#### TUI Contract

The TUI **must** support the same interface as the CLI:
- All 4 navigation modes (1/2/3/4 keys)
- All facet flags (`--realm`, `--layer`, `--trait`, `--edge-family`, `--kind`)
- All write operations (`node:create/edit/delete`, `relation:create/delete`)
- Same `--format` options for export (`table`, `json`, `cypher`)

UI details (layout, keybindings, animation) are implementation decisions
for Phase 7. The contract ensures feature parity with CLI.

Tech stack and connection details are defined in the unified Rust binary
section above (Dependencies, Connection).

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

Each data instance gets exactly one `OF_KIND` relationship to its Kind node.
Cardinality is enforced by integrity tests (see Testing Strategy section).
Estimated production volume: ~5,000-10,000 OF_KIND relationships
(200 locales x ~20 knowledge nodes + project/shared instances = ~5,000 rels;
current seed data has 66 FOR_LOCALE instances across 6 localized types).
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

```cypher
-- Every data node has exactly 1 OF_KIND (no orphans, no duplicates)
MATCH (n) WHERE NOT n:Meta AND NOT exists((n)-[:OF_KIND]->(:Kind))
RETURN labels(n)[0] AS orphaned_type, count(*) AS count
```

```cypher
-- No data node has multiple OF_KIND relationships
MATCH (n)-[r:OF_KIND]->()
WITH n, count(r) AS of_kind_count
WHERE of_kind_count > 1
RETURN labels(n)[0] AS type, n.key AS key, of_kind_count
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
| Rust CLI commands | 100% (all 4 navigation modes + node/relation CRUD) |

## Migration Strategy

### Approach: Clean slate (no backward compatibility)

This is a **breaking change** (v8.3 → v9.0). No incremental migration.
All dev data lives in seed files, so a clean rebuild is safe and simple.

**Clean break**: No v8→v9 compatibility layer. No gradual migration. No runtime
detection of old vs new schema. All consumers (Studio, CLI, tests) are updated
atomically on the feature branch. v8 types (`Scope`, `NodeCategory`,
`LocaleBehavior`) are deleted — not deprecated, not aliased. If it compiles, it's v9.

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

Total: ~130 files across 5 packages + 1 Rust tool + docs + Claude config. Grouped by package.

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
| `src/filters/index.ts` | Update | Remove `NodeCategory` re-export, add `Layer`/`Trait`/`Realm` exports |
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
| `src/generators/KindGenerator.ts` | Create | New generator for Kind nodes + facet rels |
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

##### Navigation Modes + Visual System (new files)

| File | Action | Description |
|------|--------|-------------|
| `src/components/toolbar/NavigationModeToggle.tsx` | Create | Mode 1-4 toggle buttons (Data/Meta/Overlay/Query) |
| `src/components/sidebar/FacetFilterPanel.tsx` | Create | Realm/Layer/Trait/EdgeFamily checkboxes |
| `src/stores/navigationStore.ts` | Create | Active mode + selected facets state |
| `src/hooks/useNavigationMode.ts` | Create | Mode-aware Cypher query builder |
| `src/app/api/graph/navigation/route.ts` | Create | API endpoint for filtered subgraph queries |
| `src/config/layerColors.ts` | Create | 9 Layer colors (replaces `categoryColors.ts` with 6) |
| `src/config/traitStyles.ts` | Create | 5 Trait border styles (solid/dashed/double/dotted/thin) |

#### tools/novanet (Rust CLI + TUI binary)

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Create | Single crate: clap, ratatui, crossterm, neo4rs, tokio, serde, serde_json, serde_yaml, tabled, nucleo, thiserror, color-eyre, tracing, tracing-subscriber |
| `src/lib.rs` | Create | Public API: query, facets, meta types, cypher builder (enables integration tests) |
| `src/main.rs` | Create | Thin entry: clap parsing → calls lib, formats output, exits |
| `src/error.rs` | Create | `NovaNetError` enum (thiserror): NotFound, ValidationFailed, Neo4jError, YamlParseError |
| `src/config.rs` | Create | Connection config struct, env var fallbacks (`NOVANET_URI`, `NOVANET_PASSWORD`) |
| `src/db.rs` | Create | Neo4j connection pool (neo4rs, async) |
| `src/cypher.rs` | Create | Cypher query builder (facet → WHERE clauses) |
| `src/facets.rs` | Create | Realm/Layer/Trait/EdgeFamily filter logic |
| `src/meta.rs` | Create | Meta-graph types (Kind, EdgeKind, etc.) |
| `src/output.rs` | Create | Formatters: table (tabled), json (serde), cypher (raw) |
| `src/validate.rs` | Create | Schema validation (Neo4j ↔ YAML via serde_yaml) |
| `src/commands/mod.rs` | Create | Command module re-exports |
| `src/commands/data.rs` | Create | Mode 1: WHERE NOT n:Meta |
| `src/commands/meta.rs` | Create | Mode 2: MATCH (n:Meta) |
| `src/commands/overlay.rs` | Create | Mode 3: MATCH (n) |
| `src/commands/query.rs` | Create | Mode 4: facet-driven |
| `src/commands/node.rs` | Create | node create/edit/delete (validate against meta-graph) |
| `src/commands/relation.rs` | Create | relation create/delete |
| `src/tui/mod.rs` | Create | TUI module entry point |
| `src/tui/app.rs` | Create | App state machine (mode, selection, filters, Loading/Ready) |
| `src/tui/ui.rs` | Create | Layout: left tree + right detail + status bar |
| `src/tui/events.rs` | Create | Crossterm keyboard/mouse events → Action enum |
| `src/tui/runtime.rs` | Create | Channel bridge: Action → tokio task → Result → App update |
| `src/tui/tree.rs` | Create | Taxonomy tree widget (Realm > Layer > Kind) |
| `src/tui/detail.rs` | Create | Kind detail pane (facets, edges, instances) |
| `src/tui/search.rs` | Create | Nucleo fuzzy search integration (/ key) |
| `src/tui/dialogs.rs` | Create | Input forms for node/relation CRUD (n/d/r keys) |

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

### Implementation Phases

#### Milestone 1: v9.0 — Self-Describing Context Graph (TrustGraph Level 5)

The migration is organized into 8 phases (+ 1 stretch goal). Each phase ends with
a **Ralph Wiggum codebase audit** (`/codebase-audit`) to verify nothing was missed,
no dead code remains, and DX is clean.

#### Phase 0: Preparation

| # | Task | Description |
|---|------|-------------|
| 0.1 | Git tag + branch | `git tag v8.3.0-stable && git checkout -b feat/ontology-v9` |
| 0.2 | Baseline audit | `/codebase-audit` — snapshot of current state, identify all v8 references |
| 0.3 | Worktree setup | `spn-powers:using-git-worktrees` — isolated workspace for v9 |

**Gate**: Ralph Wiggum #0 — baseline clean, all v8 references catalogued

#### Phase 1: YAML Foundation (~core/models)

| # | Task | Description |
|---|------|-------------|
| 1.1 | Rewrite `organizing-principles.yaml` | v9 format: realms/layers/traits/edge_families |
| 1.2 | Add `locale_behavior` to 35 node YAMLs | 33/35 missing — add field, remove `category`, fix stale headers |
| 1.3 | Full rewrite `relations.yaml` | Dict→list format + `family` + multi-source/target |
| 1.4 | Update `_index.yaml` | Bump version, update scope→realm terminology |
| 1.5 | Delete `relations/in-subcategory.yaml` | Replaced by OF_KIND (generated) |

**Gate**: Ralph Wiggum #1 — all 35 YAMLs have `locale_behavior`, no `category` field, relations.yaml valid

#### Phase 2: Generator Architecture (~schema-tools)

| # | Task | Description |
|---|------|-------------|
| 2.1 | Rewrite `OrganizingPrinciplesGenerator.ts` | v9 Cypher for Realm, Layer, Trait, EdgeFamily |
| 2.2 | Restructure `RelationsParser.ts` | List format + `family` + multi-source/target. **Must complete before generators that consume relations.yaml** |
| 2.3 | Rename `SubcategoryGenerator` → `LayerGenerator` | Layer mapping TypeScript |
| 2.4 | Create `KindGenerator.ts` | Kind nodes + `schema_hint`, `context_budget` + facet rels. **MUST fail-fast** if any YAML is missing `locale_behavior` — no silent defaults, throw with file path |
| 2.5 | Create `EdgeSchemaGenerator.ts` | EdgeKind nodes + `cypher_pattern` + FROM/TO_KIND (depends on 2.2) |
| 2.6 | Create `AutowireGenerator.ts` | OF_KIND wiring statements |
| 2.7 | Create `HierarchyGenerator.ts` | organizing-principles.yaml → hierarchy.ts |
| 2.8 | Update `MermaidGenerator.ts` | Realm/Layer/Trait coloring |
| 2.9 | Update `generate-all.ts` + `validate-sync.ts` | New generator imports, order: OrganizingPrinciples → Kind → EdgeSchema → Layer → Mermaid → Autowire → Hierarchy |
| 2.10 | Run `pnpm schema:generate` | Validate all 7 generators produce correct output, run generated Cypher against test Neo4j |

**Parallelization**: After 2.1–2.3 (foundation), tasks 2.4–2.8 are independent — use `spn-powers:dispatching-parallel-agents`.

**Gate**: Ralph Wiggum #2 — all generators produce valid output, EdgeKind count by family matches spec (23+7+7+6+2+2=47), no v8 generator logic remains

#### Phase 3: TypeScript Types + Core (~core/src)

| # | Task | Description |
|---|------|-------------|
| 3.1 | Generate `KIND_META` + derived maps | Single record replaces 4 separate classification systems |
| 3.2 | Kill `NodeCategory` | Delete from core, update `filters/types.ts` to use Layer directly |
| 3.3 | Update filters | `CypherGenerator.ts`, `NovaNetFilter.ts` — kill NodeCategory expansion |
| 3.4 | Update graph module | `layers.ts`, `hierarchy.ts`, `types.ts` — Realm/Layer/Trait types |
| 3.5 | PascalCase→lowercase audit | ~140 string literals: `'Global'`→`'global'`, `'localeKnowledge'`→`'knowledge'` |
| 3.6 | Update tests | Schema sync, hierarchy, generator, convention tests |

**Gate**: Ralph Wiggum #3 — `pnpm type-check` + `pnpm test --filter=@novanet/core` pass, no `NodeCategory` refs in `packages/core/**` (Studio keeps NodeCategory until Phase 5)

#### Phase 4: Neo4j Migration (~db)

| # | Task | Description |
|---|------|-------------|
| 4.1 | Update `00-constraints.cypher` | Drop v8 meta constraints, add v9 (6 types) |
| 4.2 | Regenerate seeds | Output of Phase 2 generators |
| 4.3 | Rename autowire | `99-autowire-subcategories.cypher` → `99-autowire-kinds.cypher` |
| 4.4 | Clean rebuild | `pnpm infra:down && docker volume rm ... && pnpm infra:up && pnpm infra:seed` |
| 4.5 | Run integrity tests | Meta-graph integrity queries (all 3 checks) |
| 4.6 | Audit query files | `queries/*.cypher` — update Scope/Subcategory references |

**Gate**: Ralph Wiggum #4 — all integrity tests pass, Neo4j schema matches YAML, no v8 labels in DB

#### Phase 5: Studio Migration (~studio, existing features)

| # | Task | Description |
|---|------|-------------|
| 5.0 | localStorage migration | Clear persisted `novanet-ui` and `novanet-filter` stores on v9 boot — v8 keys (`collapsedScopes`, `dataMode`) contain dead values. Add version check in `partialize` or init logic. **Do first** to avoid fighting stale state during development. |
| 5.1 | Visual system | Define 3-channel encoding: create `layerColors.ts` (9 Layer fill colors), create `traitStyles.ts` (5 Trait border styles: solid/dashed/double/dotted/thin), update layouts for Realm spatial zones. **Defines the replacement before removing the old system.** |
| 5.2 | Kill NodeCategory in Studio | `filterAdapter.ts` (remove NodeCategory type + imports), `nodeTypes.ts` (import Layer from core), delete `categoryColors.ts` (replaced by `layerColors.ts`) |
| 5.3 | View presets migration | Rewrite 9 `VIEW_PRESETS` from `NodeCategory` to v9 facets (Realm×Layer×Trait), drop dead presets 7-8 (priority/freshness deleted in v8.2) |
| 5.4 | Component renames | ScopeGroupNode→Realm, SubcategoryGroupNode→Layer, ScopeAttractor→RealmAttractor, SubcategoryAttractor→LayerAttractor, LocaleKnowledgeNode (`localeKnowledge`→`knowledge`), **Graph2D.tsx** (23 refs — highest density, critical path), **ResultsOverview.tsx** |
| 5.5 | ViewCategory rename | `'scope'`→`'overview'` in viewStore, views/route, view.schema. Audit YAML view files for `category: scope` |
| 5.6 | Update API queries | `organizing-principles/route.ts` — Realm/Layer/Kind/Trait Cypher |
| 5.7 | Update hooks | `useMagneticData`, `useMagneticSimulation`, `useFilteredGraph` |
| 5.8 | Update stores | `filterStore.ts` — collapsedScopes→collapsedRealms, collapsedSubcategories→collapsedLayers |
| 5.9 | Update layouts | 6 layout algorithms — Scope→Realm, Subcategory→Layer, apply Realm spatial zones |
| 5.10 | Update misc | SchemaNode, tokens, page.tsx, tailwind.config, schemaGenerator |
| 5.11 | Update tests | Unit tests + e2e/schema-mode.spec.ts (includes PascalCase→lowercase fixture updates) |

**Parallelization**: After 5.0–5.3 (foundation), tasks 5.4–5.10 are largely independent file updates — use parallel agents for component renames (5.4), layouts (5.9), and hooks (5.7).

**Gate**: Ralph Wiggum #5 — `pnpm test --filter=@novanet/studio` passes, `pnpm dev` renders with 9 Layer colors + 5 Trait borders + 3 Realm zones, no v8 terms, localStorage loads cleanly, no console errors

#### Phase 6: Studio Navigation (new features)

| # | Task | Description |
|---|------|-------------|
| 6.0 | Migrate `DataMode` → `NavigationMode` | In `uiStore.ts`: replace `DataMode = 'data' \| 'schema'` with `NavigationMode = 'data' \| 'meta' \| 'overlay' \| 'query'`, update all consumers (`GraphToolbar`, `ApiRoutes`, `useGraphData`), update persist partialize |
| 6.1 | Create `NavigationModeToggle.tsx` | Toolbar: Data/Meta/Overlay/Query mode buttons |
| 6.2 | Create `navigationStore.ts` | Active mode + selected facets state |
| 6.3 | Create `useNavigationMode.ts` | Mode-aware Cypher query builder |
| 6.4 | Create `FacetFilterPanel.tsx` | Sidebar: Realm/Layer/Trait/EdgeFamily checkboxes (populated from `MATCH (n:Meta)`) |
| 6.5 | Create navigation API | `/api/graph/navigation/route.ts` |
| 6.6 | Context-aware ViewPicker | Data/Query mode → show YAML views as filtered subgraphs; Meta mode → show ontology views (taxonomy tree, edge schema) |
| 6.7 | New keyboard presets | `T` = Trait cycle, `E` = EdgeFamily filter, update `?` help modal |

**Gate**: Ralph Wiggum #6 — all 4 navigation modes work, facet filters are dynamic from meta-graph, ViewPicker context-aware

#### Phase 7: Rust CLI + TUI + Documentation

| # | Task | Description |
|---|------|-------------|
| 7.1 | Scaffold `tools/novanet` Rust crate | `Cargo.toml` with all deps (see Dependencies table). `lib.rs` + thin `main.rs` with clap subcommands. `error.rs` with `NovaNetError` enum. `config.rs` with connection config + env fallbacks. |
| 7.2 | Implement read commands | `novanet data`, `novanet meta`, `novanet overlay`, `novanet query` — 4 navigation modes with `--realm/--layer/--trait/--edge-family/--kind/--format` flags |
| 7.2b | Add unit tests for core modules | Unit tests for `cypher.rs` (query builder → correct Cypher for facet combinations), `facets.rs` (filter intersection logic), `meta.rs` (type mapping). Integration tests with Neo4j testcontainer for end-to-end command output. |
| 7.3 | Implement write commands | `novanet node create/edit/delete`, `novanet relation create/delete` — validate against meta-graph (Kind exists, correct Realm/Layer/Trait, auto-wire OF_KIND) |
| 7.4 | Implement `novanet schema validate` | Validate Neo4j state matches YAML definitions (via `serde_yaml`) |
| 7.5a | TUI scaffold | App state machine (`Loading`/`Ready` variants), basic layout (tree + detail + status bar), mode toggle (1/2/3/4), async channel bridge (`runtime.rs`) |
| 7.5b | TUI taxonomy tree | Tree widget with Realm > Layer > Kind hierarchy, collapse/expand, arrow key navigation |
| 7.5c | TUI async + filters | Async Neo4j queries via mpsc channel bridge, loading states, facet filter popup (`f` key) |
| 7.5d | TUI search + detail | Nucleo fuzzy search (`search.rs`, `/` key), Kind detail pane, edge explorer (`e` key) |
| 7.5e | TUI CRUD dialogs | Input forms for node create/edit/delete (`dialogs.rs`, `n`/`d` keys), relation CRUD (`r` key), confirmation prompts, Cypher preview (`c` key) |
| 7.6 | Update Claude skills | `novanet-architecture`, `novanet-sync` — v9 terminology |
| 7.7 | Update Claude commands | `novanet-arch`, `novanet-sync` — v9 references |
| 7.8 | Update CLAUDE.md files | Root, core, studio — v9 terminology and version |
| 7.9 | Update docs | NOVANET-PITCH, plan docs, _index.yaml, README |
| 7.10 | Update turbo generators | Scaffold templates with v9 fields |

**Gate**: Ralph Wiggum #7 — `novanet data/meta/query/node` commands work, unit tests pass, `novanet tui` launches with taxonomy tree + async queries, all docs reference v9, no stale v8 terminology anywhere

#### Phase 8: Final Verification

| # | Task | Description |
|---|------|-------------|
| 8.1 | Full Ralph Wiggum sweep | `/codebase-audit` — comprehensive dead code + legacy pattern scan |
| 8.2 | Full test suite | `pnpm test` — all packages pass |
| 8.3 | Type check | `pnpm type-check` — zero errors |
| 8.4 | Lint | `pnpm lint` — zero warnings |
| 8.5 | Performance benchmarks | All 4 navigation modes within target latencies |
| 8.6 | Code review | `spn-powers:requesting-code-review` — full implementation review |
| 8.7 | PR creation | `spn-powers:finishing-a-development-branch` — merge to main |

**Gate**: All tests pass, all audits clean, PR approved

---

#### Milestone 2: v10.0 — Dynamic Retrieval (TrustGraph Level 6)

**Prereq**: Milestone 1 (v9.0) complete and stable. Release tag `v9.0.0`.

v9 bakes in 3 nullable properties that v10 populates: `traversal_depth` (Kind),
`default_traversal` (EdgeFamily), `temperature_threshold` (EdgeKind). No schema
migration needed — v10 activates what v9 already carries.

#### Phase 10: Context Assembly Engine

**Objective**: Build an engine that reads the meta-graph to assemble token-aware
context windows autonomously.

- Populate `traversal_depth` on all 35 Kind nodes
- Populate `default_traversal` on all 5 EdgeFamily nodes (`always` / `conditional` / `on_demand`)
- Populate `temperature_threshold` on semantic/mining EdgeKinds
- Build `ContextAssembler` service that traverses the meta-graph to decide what to
  include per Kind + EdgeFamily traversal rules

**Success**: Given a Block + Locale, the engine produces a context window without
hardcoded traversal logic. All traversal decisions come from the meta-graph.

#### Phase 11: Dynamic Budget System

**Objective**: Replace static `context_budget` with a dynamic system that adapts
per prompt type, locale complexity, and concept density.

- Add `retrieval_strategy` to Kind (`'expand'` | `'summary'` | `'key_only'`)
- Build budget calculator: locale complexity x concept count → token budget
- Implement token counting + truncation strategy per budget tier

**Success**: Same graph, different context windows depending on task type.
**Benchmark**: context assembly < 200ms, token waste < 10%.

#### Phase 12: Orchestrator Integration

**Objective**: Wire the context assembly engine into the LLM orchestrator pipeline
(block generation workflow).

- Replace hardcoded Cypher context queries with `ContextAssembler`
- A/B testing: old static context vs new dynamic context
- Benchmark: latency, token efficiency, output quality delta

**Success**: Orchestrator uses meta-graph-driven context for all generation tasks.
No hardcoded traversal patterns remain.

**Gate**: Ralph Wiggum — no static traversal logic, all context assembly driven
by meta-graph properties. Release tag `v10.0.0`.

---

#### Milestone 3: v11.0 — Autonomous Learning (TrustGraph Level 7)

**Prereq**: Milestone 2 (v10.0) complete and stable. Release tag `v10.0.0`.

v9 bakes in 3 nullable properties that v11 populates: `generation_count` (Kind),
`quality_score` (PageL10n/BlockL10n), `prompt_fingerprint` (PageL10n/BlockL10n).
No schema migration needed — v11 activates what v9 already carries.

#### Phase 13: Evaluation Pipeline

**Objective**: Build a system that scores generated content and tracks which
prompts produced which outputs.

- Populate `quality_score` (0.0–1.0) on PageL10n/BlockL10n after generation
- Populate `prompt_fingerprint` (SHA-256/16) on each output
- Increment `generation_count` on Kind after each generation cycle
- Build `EvaluationService`: automated scoring + human review UI

**Success**: Every generated output has a `quality_score` and a
`prompt_fingerprint`. `generation_count` reflects actual usage.

#### Phase 14: Feedback Loops

**Objective**: Close the loop — quality scores feed back into context assembly
to improve future generations.

- `quality_score` → auto-adjust `context_budget` on Kind
  (low scores → increase budget, high scores → reduce budget)
- `prompt_fingerprint` → cache invalidation when prompts change
- Locale x concept x block type analysis dashboards

**Success**: After N generation cycles, `context_budget` values have been
auto-adjusted for at least 5 Kinds. Measurable quality improvement on
re-generation of low-scoring outputs.

#### Phase 15: Self-Tuning Meta-Graph

**Objective**: The meta-graph evolves autonomously based on generation patterns
and quality signals.

- Pattern discovery: best concept combos per locale →
  auto-suggest `SEMANTIC_LINK` temperature adjustments
- Trait reclassification: Kind marked `low` budget but consistently needs more
  context → auto-promote to `medium`
- Meta-graph update loop: orchestrator writes back to meta-graph after
  evaluation, closing the learning cycle

**Success**: Meta-graph has been updated at least once by the autonomous pipeline
without human intervention. Quality trend is measurably positive across 3+
consecutive cycles.

**Gate**: Ralph Wiggum — feedback loop is functional, no manual tuning required
for steady-state operation. Release tag `v11.0.0`.

---

#### Milestone 4: v12.0 — Content Pipeline (placeholder)

**Prereq**: Milestone 3 (v11.0) complete and stable. Release tag `v11.0.0`.

Full content generation pipeline exposed through CLI. The orchestrator dispatches
generation tasks via CLI commands, not internal API calls.

#### Phase 16: CLI Content Pipeline

**Objective**: Make content generation a first-class CLI operation.

- `novanet content generate --page=pricing --locale=fr-FR` — generate a full page
- `novanet content status --locale=ja-JP` — check generation status across all pages
- `novanet content diff --page=pricing --locale=fr-FR` — show what changed since last generation
- `novanet content approve --page=pricing --locale=fr-FR` — mark output as approved
- Orchestrator consumes `novanet` commands programmatically (AI-driven pipeline)
- `novanet tui` wraps all content commands with interactive review UI

**Success**: An AI agent can generate, review, and approve content for any
page/locale combination using only CLI commands. No Studio or direct API required.

**Note**: Detailed design deferred until v11 feedback loop proves the meta-graph
is stable enough for autonomous content operations.

### Quality Gates: Ralph Wiggum Audit Protocol

Each phase ends with a `/codebase-audit` (Ralph Wiggum loop). The audit checks:

1. **Dead code** — no unused imports, functions, types, or files from v8
2. **Stale references** — no `Scope`, `Subcategory`, `NodeTypeMeta`, `NodeCategory`,
   `localeKnowledge`, `IN_SUBCATEGORY` strings anywhere in codebase
3. **Consistency** — YAML ↔ TypeScript ↔ Neo4j ↔ Mermaid all agree
4. **DX quality** — imports are clean, naming is consistent, no TODO/FIXME left behind
5. **Test coverage** — all new/modified code has tests, coverage targets met

**Why 8+ audits instead of 1 final sweep?** Catching drift early is exponentially
cheaper than fixing it at the end. Each phase builds on the previous — if Phase 2
introduces a stale reference, Phase 3 will compound it.

### Skills & Agents Inventory

Tools available for the v9 implementation:

#### Claude Code Skills (slash commands)

| Skill | Use When |
|-------|----------|
| `/codebase-audit` | **Ralph Wiggum** — dead code, stale refs, legacy patterns (after every phase) |
| `/novanet-sync` | Validate YAML ↔ TypeScript ↔ Mermaid sync (Phases 1-3) |
| `/schema:add-node` | Add new node type with Socratic discovery |
| `/schema:edit-node` | Modify existing node type |
| `/schema:add-relation` | Add new relationship type |
| `/novanet-arch` | Display architecture diagram (verification) |
| `/token-audit` | Verify design token adoption (Phase 5) |

#### Superpowers Skills (workflow enforcement)

| Skill | Use When |
|-------|----------|
| `spn-powers:test-driven-development` | Writing generators + tests (Phase 2-3) |
| `spn-powers:systematic-debugging` | Any test failure or unexpected behavior |
| `spn-powers:verification-before-completion` | Before marking any phase as complete |
| `spn-powers:dispatching-parallel-agents` | 3+ independent tasks in a phase |
| `spn-powers:subagent-driven-development` | Executing phase tasks with review between each |
| `spn-powers:requesting-code-review` | After completing a major phase |
| `spn-powers:receiving-code-review` | When acting on review feedback |
| `spn-powers:using-git-worktrees` | Phase 0 workspace isolation |
| `spn-powers:writing-plans` | Detailed implementation plans per phase |
| `spn-powers:executing-plans` | Batch execution with review checkpoints |
| `spn-powers:finishing-a-development-branch` | Phase 8 final merge |
| `spn-powers:brainstorming` | Any design decision within a phase |

#### Specialized Agents

| Agent | Use When |
|-------|----------|
| `neo4j-architect` | Cypher query optimization, schema design (Phases 2, 4) |
| `feature-dev:code-architect` | Component architecture decisions (Phase 5-6) |
| `feature-dev:code-explorer` | Understanding existing code before modifying (all phases) |
| `feature-dev:code-reviewer` | Post-implementation review (all gates) |
| `code-reviewer` | NovaNet-specific code quality (all gates) |

#### Workflow Per Phase

```
1. Read phase tasks → Create TodoWrite items
2. Use spn-powers:writing-plans for detailed subtasks
3. Use spn-powers:test-driven-development for each task
4. Use spn-powers:verification-before-completion before marking done
5. Run /codebase-audit (Ralph Wiggum gate)
6. Use spn-powers:requesting-code-review for phase review
7. Mark phase complete → proceed to next
```

