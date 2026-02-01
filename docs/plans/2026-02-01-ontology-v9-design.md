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
2. Run `novanet schema generate`
3. Generated Cypher seeds, TypeScript, and Mermaid
4. Run `novanet db seed` to apply to Neo4j

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

### Rust-First Architecture

**Architecture rule**: Single `novanet` Rust binary handles ALL schema and graph
operations. TypeScript is limited to: Studio web app, `core/types` (consumed by
Studio), `core/schemas` (Zod for runtime validation + type inference).

```
novanet (Rust binary) = universal interface for ALL graph + schema operations
├── CLI mode   novanet schema / db / locale / doc / filter / graph ...
├── TUI mode   novanet tui (interactive terminal)
├── AI agent   calls novanet query --format=json programmatically (v10+)
└── v10-v12    context assemble / eval / content generate

pnpm (TS monorepo) = web UI only
├── pnpm dev                Studio web app (Next.js)
├── pnpm build/lint/test    Monorepo dev tools
└── @novanet/core           Types + Zod schemas (consumed by Studio)
```

~5ms startup (vs ~800ms Node.js). An AI orchestrator calls
`novanet query --kind=Page --trait=localized --format=json` to discover what to
generate. This is the "self-describing context graph" in action.

**Eliminated packages**: `@novanet/schema-tools` and `@novanet/cli` are fully
absorbed into the Rust binary. See [TS Elimination Table](#ts-elimination-table)
for the ~7,000 lines of TypeScript replaced by Rust.

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

**Node CRUD Cypher patterns** (`src/commands/node.rs`):

```cypher
-- node create: validate Kind exists, create node, auto-wire OF_KIND
MATCH (k:Kind {label: $kind})
CREATE (n {key: $key})
SET n += $props, n:$kind
CREATE (n)-[:OF_KIND]->(k)
RETURN n.key AS key, labels(n) AS labels

-- node edit: update properties (merge, not replace)
MATCH (n {key: $key})
SET n += $props
RETURN n.key AS key, properties(n) AS props

-- node delete: remove node and all relationships (requires --confirm)
MATCH (n {key: $key})
DETACH DELETE n
RETURN count(*) AS deleted
```

**Relation CRUD Cypher patterns** (`src/commands/relation.rs`):

```cypher
-- relation create: validate EdgeKind exists, create relationship
MATCH (ek:EdgeKind {key: $rel_type})
MATCH (from {key: $from_key}), (to {key: $to_key})
CALL apoc.create.relationship(from, $rel_type, {}, to) YIELD rel
RETURN type(rel) AS type, startNode(rel).key AS from, endNode(rel).key AS to

-- relation delete: remove specific relationship
MATCH (from {key: $from_key})-[r]->(to {key: $to_key})
WHERE type(r) = $rel_type
DELETE r
RETURN count(*) AS deleted
```

> **Note**: Node creation uses dynamic labels via `SET n:$kind` (requires APOC
> `apoc.create.setLabels` in practice). The Rust implementation should validate
> the Kind label against the meta-graph before executing. Relation creation uses
> `apoc.create.relationship` for dynamic relationship types; fallback to string
> interpolation with parameterized keys if APOC is unavailable.

#### Schema & Generation Commands

| Command | Description | Output |
|---------|------------|--------|
| `novanet schema generate` | YAML → all artifacts | `layers.ts`, Mermaid, Cypher seeds, `hierarchy.ts` |
| `novanet schema validate` | YAML ↔ Neo4j consistency check | Pass/fail with diff details |
| `novanet doc generate` | Generate documentation views | Mermaid diagrams, view markdown files |
| `novanet doc validate` | Validate doc sync | Pass/fail |

#### Database Commands

| Command | Description |
|---------|------------|
| `novanet db seed` | Execute seed Cypher files against Neo4j |
| `novanet db migrate` | Run migration scripts |
| `novanet db reset` | Drop all data + reseed (with `--confirm`) |

#### Locale Knowledge Commands

| Command | Description |
|---------|------------|
| `novanet locale parse` | Parse markdown locale knowledge → structured JSON |
| `novanet locale import` | Import parsed locale data into Neo4j |

#### Search & Filter Commands

| Command | Description |
|---------|------------|
| `novanet search --hybrid` | Hybrid vector + graph search |
| `novanet filter build` | Build Cypher from JSON filter spec (for Studio subprocess) |

#### Interactive

| Command | Description |
|---------|------------|
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
    ├── commands/
    │   ├── mod.rs
    │   ├── data.rs         Mode 1: WHERE NOT n:Meta
    │   ├── meta.rs         Mode 2: MATCH (n:Meta)
    │   ├── overlay.rs      Mode 3: MATCH (n)
    │   ├── query.rs        Mode 4: facet-driven
    │   ├── node.rs         node create/edit/delete
    │   ├── relation.rs     relation create/delete
    │   ├── schema.rs       schema generate / validate
    │   ├── db_cmd.rs       db seed / migrate / reset
    │   ├── locale.rs       locale parse / import
    │   ├── doc.rs          doc generate / validate
    │   ├── filter.rs       filter build (for Studio subprocess)
    │   └── search.rs       search --hybrid
    ├── generators/
    │   ├── mod.rs           Generator trait + orchestration
    │   ├── mermaid.rs       YAML → Mermaid flowchart (replaces MermaidGenerator.ts)
    │   ├── layer.rs         YAML → layers.ts (replaces SubcategoryGenerator.ts)
    │   ├── kind.rs          YAML → Kind Cypher (replaces KindGenerator.ts)
    │   ├── edge_schema.rs   YAML → EdgeKind Cypher (replaces EdgeSchemaGenerator.ts)
    │   ├── autowire.rs      YAML → OF_KIND wiring (replaces AutowireGenerator.ts)
    │   ├── hierarchy.rs     YAML → hierarchy.ts (replaces HierarchyGenerator.ts)
    │   └── organizing.rs    YAML → meta-graph Cypher (replaces OrganizingPrinciplesGenerator.ts)
    ├── parsers/
    │   ├── mod.rs           Parser trait + file discovery
    │   ├── yaml_node.rs     Parse YAML node definitions (35 files)
    │   ├── relations.rs     Parse relations.yaml (list format + family)
    │   └── locale_md.rs     Parse markdown locale knowledge (7 parsers: voice, culture, identity, market, lexicon, rules, references)
    ├── search/
    │   ├── mod.rs
    │   ├── hybrid.rs        Hybrid vector + graph search
    │   ├── traversal.rs     Graph traversal service
    │   └── vector.rs        Vector similarity search
    ├── filter/
    │   ├── mod.rs
    │   └── build.rs         JSON filter spec → Cypher (for Studio subprocess)
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

Single `cargo build` produces the `novanet` binary with CLI + TUI.
Split into workspace if crate grows past v10.

**Feature gate**: TUI is behind a feature flag so CI can build without terminal deps:
```toml
[features]
default = ["tui"]
tui = ["dep:ratatui", "dep:crossterm"]
```
Use `cargo build --no-default-features` for headless/CI builds (CLI-only).

#### Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` + `clap_derive` | CLI argument parsing, subcommands |
| `ratatui` | Terminal UI framework |
| `crossterm` | Cross-platform terminal backend |
| `neo4rs` | Neo4j Bolt driver (async) — pin exact minor version |
| `tokio` | Async runtime (`features = ["full"]`) |
| `tokio-util` | `CancellationToken` for graceful TUI shutdown |
| `futures` | `StreamExt` for crossterm `EventStream` integration |
| `serde` + `serde_json` | Neo4j result deserialization, JSON output |
| `serde_yml` | YAML parsing for schema validation + generators (`serde_yaml` is deprecated) |
| `minijinja` | Template engine for TypeScript code generation (1 dep, by Jinja2 creator Armin Ronacher) |
| `tabled` | Table output formatting |
| `nucleo` | Fuzzy search (TUI `/` key) |
| `thiserror` | Structured error enum (`NovaNetError`) for matching |
| `color-eyre` | Error reporting with context (wraps `thiserror` errors) |
| `tracing` + `tracing-subscriber` | Structured logging with env-filter output |
| `indicatif` | Progress bars for long operations (generation, seeding) |
| `petgraph` | In-memory graph for dependency ordering in generators |
| `rayon` | Parallel YAML file processing for faster validation |
| `walkdir` | Recursive YAML file discovery (scan `models/nodes/` tree) |

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

### Rust DX Reference (Implementation Patterns)

This section provides exact code patterns, verified against Context7/docs.rs (2026-02),
for every crate in the dependency table. Implementors should copy-paste these patterns
rather than inventing from scratch.

#### Cargo.toml (exact)

```toml
[package]
name = "novanet"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"  # Minimum supported (edition 2024)

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# TUI (feature-gated, see [features] below)
ratatui = { version = "0.29", optional = true }
crossterm = { version = "0.28", features = ["event-stream"], optional = true }

# Async
tokio = { version = "1.43", features = ["full"] }
tokio-util = "0.7"            # CancellationToken for graceful shutdown
futures = "0.3"               # StreamExt for crossterm EventStream

# Neo4j
neo4rs = "0.8"                # Pin minor: breaking changes between 0.x

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yml = "0.0.12"          # serde_yaml is deprecated — serde_yml is the maintained fork

# Template engine
minijinja = "2"               # 1 dep, better errors than tera (15+ deps), by Armin Ronacher

# Output
tabled = "0.17"
indicatif = "0.17"            # Progress bars for long operations

# Search
nucleo = "0.5"                # Fuzzy search (TUI `/` key)

# Graph
petgraph = "0.7"              # In-memory graph for generator dependency ordering
rayon = "1.10"                # Parallel YAML processing
walkdir = "2"                 # Recursive YAML file discovery

# Error handling
thiserror = "2"               # Library-style errors (NovaNetError enum)
color-eyre = "0.6"            # Rich error reports with context

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[features]
default = ["tui"]
tui = ["dep:ratatui", "dep:crossterm"]

[dev-dependencies]
assert_cmd = "2"              # CLI integration tests (binary invocation)
predicates = "3"              # Output assertion matchers
insta = { version = "1", features = ["yaml"] } # Snapshot testing
tokio-test = "0.4"            # Async test utilities

[profile.release]
strip = true                  # Strip symbols (smaller binary)
lto = "thin"                  # Link-time optimization
codegen-units = 1             # Better optimization
opt-level = 3

[profile.dev.package."*"]
opt-level = 2                 # Optimize deps in dev mode (faster builds)
```

> **Context7 verification (2026-02-01)**: All 288 packages resolved and compiled
> with `cargo check` on rustc 1.92.0. Cargo correctly locks ratatui to 0.29.0
> (0.30.0 requires Rust 1.86.0, incompatible with rust-version = "1.85").
> All crate APIs verified via Context7: neo4rs 0.8, thiserror 2, minijinja 2,
> serde_yml 0.0.12, clap 4 derive. No version conflicts detected.

#### Error Handling Pattern (thiserror + color-eyre)

`src/error.rs` — structured errors for matching + rich context for display:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NovaNetError {
    #[error("Neo4j connection failed: {uri}")]
    Connection { uri: String, #[source] source: neo4rs::Error },

    #[error("query failed: {query}")]
    Query { query: String, #[source] source: neo4rs::Error },

    #[error("no Kind found for label '{0}'")]
    UnknownKind(String),

    #[error("meta-graph integrity: {0}")]
    MetaIntegrity(String),

    #[error("YAML schema error in {path}")]
    Schema { path: String, #[source] source: serde_yml::Error },

    #[error("validation failed: {0}")]
    Validation(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NovaNetError>;
```

Application entry uses `color_eyre::install()` in main.rs for rich backtraces:

```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ... clap parse, dispatch
}
```

#### CLI Structure (clap derive)

`src/main.rs` — thin entry point:

```rust
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "novanet", version, about = "NovaNet context graph CLI")]
struct Cli {
    /// Monorepo root (auto-detected: walks up to find pnpm-workspace.yaml)
    #[arg(long, env = "NOVANET_ROOT")]
    root: Option<std::path::PathBuf>,

    /// Neo4j URI
    #[arg(long, env = "NEO4J_URI", default_value = "bolt://localhost:7687")]
    uri: String,

    /// Neo4j user
    #[arg(long, env = "NEO4J_USER", default_value = "neo4j")]
    user: String,

    /// Neo4j password
    #[arg(long, env = "NEO4J_PASSWORD", default_value = "novanetpassword")]
    password: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mode 1: Data nodes only (WHERE NOT n:Meta)
    Data {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Mode 2: Meta-graph only (MATCH (n:Meta))
    Meta {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Mode 3: Data + Meta overlay
    Overlay {
        #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
        format: OutputFormat,
    },
    /// Mode 4: Facet-driven query
    Query(QueryArgs),
    /// CRUD: node operations
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },
    /// CRUD: relation operations
    Relation {
        #[command(subcommand)]
        action: RelationAction,
    },
    /// Schema validation (YAML ↔ Neo4j)
    Schema {
        #[command(subcommand)]
        action: SchemaAction,
    },
    /// Interactive terminal UI
    Tui,
}

#[derive(clap::Args)]
struct QueryArgs {
    #[arg(long)] realm: Option<String>,
    #[arg(long)] layer: Option<String>,
    #[arg(long, name = "trait")] trait_filter: Option<String>,
    #[arg(long)] edge_family: Option<String>,
    #[arg(long)] kind: Option<String>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Copy)]
enum OutputFormat { Table, Json, Cypher }

#[derive(Subcommand)]
enum NodeAction { Create { /* ... */ }, Edit { /* ... */ }, Delete { /* ... */ } }

#[derive(Subcommand)]
enum RelationAction { Create { /* ... */ }, Delete { /* ... */ } }

#[derive(Subcommand)]
enum SchemaAction {
    /// Validate YAML matches Neo4j state
    Validate {
        #[arg(long)] strict: bool,
    },
}
```

#### Project Root Discovery

`src/config.rs` — resolve the monorepo root for YAML model access:

```rust
use std::path::{Path, PathBuf};

/// Resolve the monorepo root directory.
/// Priority: 1) --root flag  2) NOVANET_ROOT env  3) walk up to pnpm-workspace.yaml
pub fn resolve_root(explicit: Option<&Path>) -> crate::Result<PathBuf> {
    if let Some(root) = explicit {
        return Ok(root.to_path_buf());
    }

    // Walk up from current directory to find pnpm-workspace.yaml
    let mut dir = std::env::current_dir()
        .map_err(|e| crate::NovaNetError::Io(e))?;
    loop {
        if dir.join("pnpm-workspace.yaml").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err(crate::NovaNetError::Validation(
                "Could not find monorepo root (no pnpm-workspace.yaml in parent directories). \
                 Use --root or set NOVANET_ROOT.".to_string()
            ));
        }
    }
}

/// Derived paths from the monorepo root
pub fn models_dir(root: &Path) -> PathBuf { root.join("packages/core/models") }
pub fn nodes_dir(root: &Path) -> PathBuf { root.join("packages/core/models/nodes") }
pub fn relations_path(root: &Path) -> PathBuf { root.join("packages/core/models/relations.yaml") }
pub fn seed_dir(root: &Path) -> PathBuf { root.join("packages/db/seed") }
```

All commands that need YAML access call `resolve_root()` first. Commands that only
query Neo4j (e.g., `data`, `meta`, `overlay`) do not need it.

#### Neo4j Connection (neo4rs + Arc)

`src/db.rs` — shared connection pool:

```rust
use neo4rs::{Graph, query};
use std::sync::Arc;

#[derive(Clone)]
pub struct Db {
    graph: Arc<Graph>,
}

impl Db {
    pub async fn connect(uri: &str, user: &str, pass: &str) -> crate::Result<Self> {
        let graph = Graph::new(uri, user, pass).await
            .map_err(|e| crate::NovaNetError::Connection {
                uri: uri.to_string(), source: e,
            })?;
        Ok(Self { graph: Arc::new(graph) })
    }

    /// Execute a read query, return rows
    pub async fn execute(&self, cypher: &str, params: Vec<(&str, impl Into<neo4rs::BoltType>)>)
        -> crate::Result<Vec<neo4rs::Row>>
    {
        let mut q = query(cypher);
        for (k, v) in params { q = q.param(k, v); }
        let mut result = self.graph.execute(q).await
            .map_err(|e| crate::NovaNetError::Query {
                query: cypher.to_string(), source: e,
            })?;
        let mut rows = Vec::new();
        while let Some(row) = result.next().await
            .map_err(|e| crate::NovaNetError::Query {
                query: cypher.to_string(), source: e,
            })?
        {
            rows.push(row);
        }
        Ok(rows)
    }

    /// Run a write transaction (CRUD)
    pub async fn write_txn(&self, statements: &[&str]) -> crate::Result<()> {
        let mut txn = self.graph.start_txn().await
            .map_err(|e| crate::NovaNetError::Query {
                query: "START TRANSACTION".to_string(), source: e,
            })?;
        txn.run_queries(statements.iter().map(|s| *s)).await
            .map_err(|e| crate::NovaNetError::Query {
                query: "TRANSACTION BODY".to_string(), source: e,
            })?;
        txn.commit().await
            .map_err(|e| crate::NovaNetError::Query {
                query: "COMMIT".to_string(), source: e,
            })?;
        Ok(())
    }

    /// Concurrent query execution (Graph.clone() is cheap — Arc internally)
    pub fn graph(&self) -> &Graph { &self.graph }
}
```

**Key pattern**: `Graph::new()` creates an internal connection pool. Clone `Graph`
(or `Db`) freely across tasks — it's `Arc`-based internally. Never create multiple
`Graph` instances for the same database.

#### TUI Async Architecture (ratatui + tokio)

`src/tui/runtime.rs` — the async bridge between sync TUI and async Neo4j:

The pattern uses **three concurrent streams** merged via `tokio::select!`:
1. **Crossterm events** → keyboard/mouse/resize (via `EventStream`)
2. **Tick interval** → periodic state refresh
3. **Render interval** → frame rate control

```rust
use crossterm::event::{EventStream, KeyEventKind};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use std::time::Duration;

/// Events flowing from input/timer → app
#[derive(Clone, Debug)]
pub enum Event {
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Resize(u16, u16),
    Tick,
    Render,
    /// Async query result arrived
    QueryResult(QueryPayload),
    Error,
}

/// Actions flowing from app → async runtime
#[derive(Clone, Debug)]
pub enum Action {
    RunQuery(String),        // Cypher query to execute
    Navigate(NavigationMode),
    SetFilter(FacetFilter),
    Quit,
}

pub async fn event_loop(
    event_tx: mpsc::UnboundedSender<Event>,
    cancel: CancellationToken,
) {
    let mut events = EventStream::new();
    let mut tick = tokio::time::interval(Duration::from_millis(250));
    let mut render = tokio::time::interval(Duration::from_millis(33)); // ~30fps

    loop {
        let event = tokio::select! {
            _ = cancel.cancelled() => break,
            _ = tick.tick() => Event::Tick,
            _ = render.tick() => Event::Render,
            ev = events.next() => match ev {
                Some(Ok(crossterm::event::Event::Key(k)))
                    if k.kind == KeyEventKind::Press => Event::Key(k),
                Some(Ok(crossterm::event::Event::Mouse(m))) => Event::Mouse(m),
                Some(Ok(crossterm::event::Event::Resize(w, h))) => Event::Resize(w, h),
                Some(Err(_)) => Event::Error,
                None => break,
                _ => continue,
            },
        };
        if event_tx.send(event).is_err() { break; }
    }
    cancel.cancel();
}
```

Main TUI loop in `src/tui/app.rs`:

```rust
pub async fn run_tui(db: Db) -> color_eyre::Result<()> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Event>();
    let (action_tx, mut action_rx) = mpsc::unbounded_channel::<Action>();
    let cancel = CancellationToken::new();

    // Spawn event loop
    let cancel_clone = cancel.clone();
    tokio::spawn(async move { event_loop(event_tx, cancel_clone).await });

    // Spawn async worker (Neo4j queries)
    let db_clone = db.clone();
    let worker_tx = action_tx.clone(); // for sending results back
    tokio::spawn(async move {
        while let Some(action) = action_rx.recv().await {
            match action {
                Action::RunQuery(cypher) => {
                    let rows = db_clone.execute(&cypher, vec![]).await;
                    // Send result back as Event::QueryResult
                },
                Action::Quit => break,
                _ => {}
            }
        }
    });

    // Terminal setup
    crossterm::terminal::enable_raw_mode()?;
    let mut terminal = ratatui::Terminal::new(
        ratatui::backend::CrosstermBackend::new(std::io::stderr())
    )?;

    let mut app = AppState::new(action_tx);
    let mut should_redraw = true; // dirty-flag: only redraw when state changed

    // Main render loop (TEA: Event → Update → View)
    loop {
        // VIEW: only redraw when dirty
        if should_redraw {
            terminal.draw(|f| app.render(f))?;
            should_redraw = false;
        }

        // UPDATE: process next event
        tokio::select! {
            Some(event) = event_rx.recv() => {
                match event {
                    Event::Key(k) => { app.handle_key(k); should_redraw = true; },
                    Event::QueryResult(payload) => { app.apply_result(payload); should_redraw = true; },
                    Event::Tick => { app.tick(); should_redraw = true; },
                    Event::Resize(_, _) => { should_redraw = true; },
                    _ => {}
                };
            }
            else => break,
        }
        if app.should_quit { break; }
    }

    // Cleanup
    cancel.cancel();
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stderr(),
        crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
```

**Key patterns** (from ratatui docs + Context7):
- `CancellationToken` for graceful shutdown (not raw booleans)
- `EventStream` + `tokio::select!` (not `crossterm::event::poll` blocking)
- `UnboundedSender<Action>` stored in App for spawning async work
- Render only on `Event::Render`, not every event (30fps cap)
- `stderr()` for terminal output (stdout reserved for CLI piping)

#### Testing Strategy (Rust)

```
tests/
├── cli_integration.rs     # assert_cmd: invoke binary, check output
├── cypher_builder.rs      # Unit: facet → Cypher string
├── meta_types.rs          # Unit: Kind/EdgeKind deserialization
└── neo4j_integration.rs   # Testcontainers: real Neo4j queries
```

**Unit tests** (no Neo4j needed):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cypher_builder_single_realm() {
        let filter = FacetFilter { realm: Some("global".into()), ..Default::default() };
        let cypher = build_facet_query(&filter);
        assert_eq!(cypher, "MATCH (:Realm {key: 'global'})<-[:IN_REALM]-(k:Kind) RETURN k");
    }

    #[test]
    fn cypher_builder_combo() {
        let filter = FacetFilter {
            realm: Some("project".into()),
            trait_filter: Some("localized".into()),
            ..Default::default()
        };
        let cypher = build_facet_query(&filter);
        insta::assert_snapshot!(cypher); // Snapshot test for complex queries
    }
}
```

**CLI integration tests** (binary invocation):
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn data_command_outputs_table() {
    Command::cargo_bin("novanet").unwrap()
        .args(["data", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\""));
}

#[test]
fn query_unknown_realm_fails() {
    Command::cargo_bin("novanet").unwrap()
        .args(["query", "--realm", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("UnknownKind"));
}
```

**Neo4j integration tests** (testcontainers — Phase 7.2b):
```rust
// Requires running Neo4j (CI uses docker-compose or testcontainers)
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn meta_graph_has_3_realms() {
    let db = Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword").await.unwrap();
    let rows = db.execute("MATCH (r:Realm) RETURN r.key AS key", vec![]).await.unwrap();
    assert_eq!(rows.len(), 3);
}
```

#### Development Workflow

```bash
# Build + run
cargo build                    # Debug build (~2s incremental)
cargo run -- data              # Run CLI
cargo run -- tui               # Run TUI
cargo run -- query --realm=project --format=json

# Quality
cargo clippy -- -D warnings    # Zero warnings policy
cargo fmt --check              # Formatting check
cargo test                     # Unit + integration
cargo test -- --ignored        # Neo4j integration tests

# Watch mode (install: cargo install cargo-watch)
cargo watch -x 'clippy -- -D warnings' -x test -x 'run -- data'

# Pre-commit checklist
cargo fmt && cargo clippy -- -D warnings && cargo test
```

#### CI/CD Integration

The Rust binary is **NOT** managed by Turborepo (it lives in `tools/`, not `packages/`).
CI should run Rust checks separately from the pnpm/turbo pipeline:

```yaml
# .github/workflows/rust.yml (simplified)
name: Rust CI
on:
  push:
    paths: ['tools/novanet/**']
  pull_request:
    paths: ['tools/novanet/**']

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.85
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: tools/novanet
      - run: cargo fmt --check
        working-directory: tools/novanet
      - run: cargo clippy -- -D warnings
        working-directory: tools/novanet
      - run: cargo test
        working-directory: tools/novanet

  integration:
    runs-on: ubuntu-latest
    needs: check
    services:
      neo4j:
        image: neo4j:5-community
        env:
          NEO4J_AUTH: neo4j/novanetpassword
        ports: ['7687:7687']
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.85
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: tools/novanet
      - run: cargo test -- --ignored
        working-directory: tools/novanet
        env:
          NEO4J_URI: bolt://localhost:7687
```

**Monorepo root `package.json`** should include convenience scripts:
```json
{
  "scripts": {
    "rust:check": "cd tools/novanet && cargo check",
    "rust:test": "cd tools/novanet && cargo test",
    "rust:build": "cd tools/novanet && cargo build --release"
  }
}
```

#### Rust-First Architecture Decision

**Architecture rule**: Single `novanet` binary owns ALL schema and graph operations.
Even TypeScript code generation (`layers.ts`) is just string templating — Rust
writes `.ts` files via `writeln!()` / MiniJinja templates trivially.

| Concern | Owner | Rationale |
|---------|-------|-----------|
| YAML → TypeScript types (`layers.ts`) | **Rust** (`novanet schema generate`) | String templating, no TS compiler needed |
| YAML → Mermaid diagrams | **Rust** (`novanet doc generate`) | String templating |
| YAML → Cypher seeds | **Rust** (`novanet schema generate`) | String templating |
| YAML ↔ Neo4j validation | **Rust** (`novanet schema validate`) | Single authoritative validator |
| Locale markdown parsing | **Rust** (`novanet locale parse`) | Performance, streaming |
| Graph traversal + hybrid search | **Rust** (`novanet search --hybrid`) | Performance, neo4rs native driver |
| Graph read queries (4 modes) | **Rust** (`novanet data/meta/overlay/query`) | Runtime performance (~5ms) |
| Graph write (CRUD) | **Rust** (`novanet node/relation`) | Meta-graph validation at write time |
| Database seeding + migration | **Rust** (`novanet db seed/migrate`) | Direct Cypher execution |
| Cypher filter generation | **Rust** (`novanet filter build`) | Studio calls via subprocess |
| Interactive TUI | **Rust** (`novanet tui`) | ~5ms startup, native terminal |
| Web visualization | **TS** (Studio / Next.js) | Separate web concern, neo4j-driver JS |
| TypeScript types | **TS** (`@novanet/core/types`) | Consumed by Studio at build time |
| Zod schemas | **TS** (`@novanet/core/schemas`) | Runtime validation + type inference |

**Eliminated**: `@novanet/schema-tools` (~2,038 lines) and `@novanet/cli` (~5 lines)
are fully absorbed into the Rust binary. See [TS Elimination Table](#ts-elimination-table).

#### Studio Filter Subprocess Pattern

Studio's 2 API routes that need Cypher filter generation (`/api/graph`,
`/api/graph/navigation`) call the Rust binary as a subprocess:

```typescript
// Studio API route (thin TS wrapper)
import { execFile } from 'child_process';

async function buildCypher(filterSpec: FilterSpec): Promise<string> {
  const { stdout } = await execFile('novanet', [
    'filter', 'build', '--format=cypher', '--input=-'
  ], { input: JSON.stringify(filterSpec) });
  return stdout;
}

// Usage in route handler
const cypher = await buildCypher(filterStore.getActiveFilters());
const result = await neo4jDriver.executeQuery(cypher);
```

~5ms subprocess overhead is negligible vs Neo4j roundtrip (~50-200ms).
No code duplication — single Cypher generation logic in Rust.

#### TS Elimination Table

~7,000 lines of TypeScript eliminated by Rust-first architecture:

| Package / Area | Lines | Rust Replacement |
|---------------|-------|-----------------|
| `@novanet/schema-tools` (all generators + parsers) | ~2,038 | `novanet schema generate` + `novanet doc generate` |
| `@novanet/cli` (empty stub) | ~5 | Deleted (was already empty) |
| `core/scripts/` (seed, validate, import scripts) | ~1,154 | `novanet db seed` / `novanet schema validate` |
| `core/src/parsers/` (7 markdown locale parsers) | ~1,550 | `novanet locale parse` |
| `core/src/generators/` (ViewParser, MarkdownGenerator, CypherExporter) | ~823 | `novanet doc generate` |
| `core/src/services/` (graph-traversal, hybrid-retriever, vector-search) | ~1,058 | `novanet search --hybrid` |
| `core/src/db/client.ts` (Neo4j singleton wrapper) | ~115 | `neo4rs` in Rust (`db.rs`) |
| **Total** | **~6,743** | |

**What stays in TypeScript:**

| Package / Area | Lines | Rationale |
|---------------|-------|-----------|
| `core/src/types/` | ~400 | Consumed by Studio at build time |
| `core/src/schemas/` (Zod) | ~1,228 | Runtime validation + type inference in Studio |
| `core/src/filters/` (types only) | ~200 | TypeScript filter type definitions for Studio |
| `apps/studio/` | ~15,000+ | Next.js web application |

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
novanet db down
docker volume rm novanet_neo4j_data

# 2. Update source files (YAML, generators, Studio)
# ... (implementation work)

# 3. Regenerate artifacts
novanet schema generate

# 4. Rebuild and seed
novanet db up
novanet db seed

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

#### @novanet/schema-tools — ELIMINATED

**Absorbed into Rust binary.** All generators and parsers move to `tools/novanet/src/generators/`
and `tools/novanet/src/parsers/`. The entire `packages/schema-tools/` directory is deleted.

| TS File (deleted) | Rust Replacement |
|-------------------|-----------------|
| `OrganizingPrinciplesGenerator.ts` | `generators/organizing.rs` |
| `SubcategoryGenerator.ts` → was `LayerGenerator.ts` | `generators/layer.rs` |
| `MermaidGenerator.ts` | `generators/mermaid.rs` |
| `KindGenerator.ts` | `generators/kind.rs` |
| `EdgeSchemaGenerator.ts` | `generators/edge_schema.rs` |
| `AutowireGenerator.ts` | `generators/autowire.rs` |
| `HierarchyGenerator.ts` | `generators/hierarchy.rs` |
| `RelationsParser.ts` | `parsers/relations.rs` |
| `validate-sync.ts` | `commands/schema.rs` (validate subcommand) |
| `generate-all.ts` | `commands/schema.rs` (generate subcommand) |
| `colors.ts` | Inline in `generators/mermaid.rs` |

#### @novanet/cli — ELIMINATED

Empty stub package (~5 lines). Delete `packages/cli/` entirely.

#### core/scripts/, core/src/parsers/, core/src/services/, core/src/db/client.ts — ELIMINATED

| TS Area (deleted) | Rust Replacement |
|-------------------|-----------------|
| `core/scripts/seed.ts` | `commands/db_cmd.rs` (seed subcommand) |
| `core/scripts/validate.ts` | `commands/schema.rs` (validate subcommand) |
| `core/scripts/import-locale.ts` | `commands/locale.rs` (import subcommand) |
| `core/src/parsers/*.ts` (7 parsers) | `parsers/locale_md.rs` |
| `core/src/services/graph-traversal.ts` | `search/traversal.rs` |
| `core/src/services/hybrid-retriever.ts` | `search/hybrid.rs` |
| `core/src/services/vector-search.ts` | `search/vector.rs` |
| `core/src/generators/*.ts` (3 files) | `generators/` (various) |
| `core/src/db/client.ts` | `db.rs` (neo4rs connection pool) |

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

#### tools/novanet (Rust binary — CLI + TUI + generators)

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Create | Single crate: clap, ratatui, crossterm, neo4rs, tokio, serde, serde_json, serde_yml, minijinja, tabled, nucleo, thiserror, color-eyre, tracing, indicatif, petgraph, rayon |
| **Core modules** | | |
| `src/lib.rs` | Create | Public API: query, facets, meta types, cypher builder (enables integration tests) |
| `src/main.rs` | Create | Thin entry: clap parsing → calls lib, formats output, exits |
| `src/error.rs` | Create | `NovaNetError` enum (thiserror): NotFound, ValidationFailed, Neo4jError, YamlParseError, GeneratorError |
| `src/config.rs` | Create | Connection config struct, env var fallbacks (`NOVANET_URI`, `NOVANET_PASSWORD`) |
| `src/db.rs` | Create | Neo4j connection pool (neo4rs, async) |
| `src/cypher.rs` | Create | Cypher query builder (facet → WHERE clauses) |
| `src/facets.rs` | Create | Realm/Layer/Trait/EdgeFamily filter logic |
| `src/meta.rs` | Create | Meta-graph types (Kind, EdgeKind, etc.) |
| `src/output.rs` | Create | Formatters: table (tabled), json (serde), cypher (raw) |
| **Commands** (~12 subcommands) | | |
| `src/commands/mod.rs` | Create | Command module re-exports |
| `src/commands/data.rs` | Create | Mode 1: WHERE NOT n:Meta |
| `src/commands/meta.rs` | Create | Mode 2: MATCH (n:Meta) |
| `src/commands/overlay.rs` | Create | Mode 3: MATCH (n) |
| `src/commands/query.rs` | Create | Mode 4: facet-driven |
| `src/commands/node.rs` | Create | node create/edit/delete (validate against meta-graph) |
| `src/commands/relation.rs` | Create | relation create/delete |
| `src/commands/schema.rs` | Create | `schema generate` (YAML → all artifacts) + `schema validate` (YAML ↔ Neo4j) |
| `src/commands/db_cmd.rs` | Create | `db seed` / `db migrate` / `db reset` |
| `src/commands/locale.rs` | Create | `locale parse` (markdown → JSON) + `locale import` (JSON → Neo4j) |
| `src/commands/doc.rs` | Create | `doc generate` (Mermaid, views) + `doc validate` |
| `src/commands/filter.rs` | Create | `filter build` (JSON → Cypher, for Studio subprocess) |
| `src/commands/search.rs` | Create | `search --hybrid` (vector + graph) |
| **Generators** (replaces @novanet/schema-tools) | | |
| `src/generators/mod.rs` | Create | `Generator` trait + orchestration (generate-all, validate-sync) |
| `src/generators/mermaid.rs` | Create | YAML → Mermaid flowchart with Realm/Layer coloring |
| `src/generators/layer.rs` | Create | YAML → `layers.ts` (MiniJinja template → TypeScript code) |
| `src/generators/kind.rs` | Create | YAML → Kind Cypher + `schema_hint` + `context_budget` + facet rels |
| `src/generators/edge_schema.rs` | Create | YAML → EdgeKind Cypher + `cypher_pattern` + FROM/TO_KIND |
| `src/generators/autowire.rs` | Create | YAML → OF_KIND wiring Cypher statements |
| `src/generators/hierarchy.rs` | Create | YAML → `hierarchy.ts` (MiniJinja template) |
| `src/generators/organizing.rs` | Create | YAML → meta-graph seed Cypher (Realm, Layer, Trait, EdgeFamily) |
| **Parsers** (replaces core/parsers + schema-tools/parsers) | | |
| `src/parsers/mod.rs` | Create | `Parser` trait + file discovery |
| `src/parsers/yaml_node.rs` | Create | Parse 35 YAML node definitions |
| `src/parsers/relations.rs` | Create | Parse `relations.yaml` (list format + family field) |
| `src/parsers/locale_md.rs` | Create | Parse markdown locale knowledge (7 parsers: voice, culture, identity, market, lexicon, rules, references) |
| **Search** (replaces core/services) | | |
| `src/search/mod.rs` | Create | Search module entry |
| `src/search/hybrid.rs` | Create | Hybrid vector + graph search |
| `src/search/traversal.rs` | Create | Graph traversal service |
| `src/search/vector.rs` | Create | Vector similarity search |
| **Filter** (replaces core/filters CypherGenerator) | | |
| `src/filter/mod.rs` | Create | Filter module entry |
| `src/filter/build.rs` | Create | JSON filter spec → Cypher string (stdin → stdout for subprocess) |
| **TUI** | | |
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

### Expert Review Findings (2026-02-01)

Four specialized reviews were conducted against this plan:

| Domain | Reviewer | Score | Verdict |
|--------|----------|-------|---------|
| Rust Architecture | `spn-rust:rust-architect` | 9/10 | Pipeline validated. walkdir missing, TUI needs feature gate. |
| Neo4j Meta-Graph | `neo4j-architect` | 9/10 | Faceted classification validated. 8 property indexes missing. |
| Architecture Coherence | `feature-dev:code-architect` | 8/10 | Phase dependencies have 3 circular issues. |
| TUI Async | `spn-rust:rust-async-expert` | 8/10 | Channel bridge correct. Dirty-flag render missing. Split TUI to v9.5. |

**7 findings applied to this plan:**

1. Meta-graph design validated (no changes)
2. YAML → Rust → templates → output pipeline validated (no changes)
3. TUI main loop: added dirty-flag `should_redraw` + `tokio::select!` pattern (line ~1719)
4. Phase 7 split into 7A (v9.0: CLI + basic TUI) and 7B (v9.5: advanced TUI + galaxy theme)
5. Added `walkdir` crate + TUI `[features]` gate (`dep:ratatui`, `dep:crossterm`)
6. Added 8 Neo4j facet property indexes in Phase 4.1b (Kind, EdgeKind, L10n quality/fingerprint)
7. Fixed 3 phase dependency issues: Phase 2.12 → dry-run only, Phase 5.0 → version-aware zustand migration, Phase 2 gate → YAML-only validation

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

#### Phase 2: Rust Generator Architecture (~tools/novanet)

Phase 2 now builds the Rust generators that replace `@novanet/schema-tools`.
This is the first Rust code written — scaffold the crate, then implement generators.

| # | Task | Description |
|---|------|-------------|
| 2.1 | Scaffold `tools/novanet/` Rust crate | `Cargo.toml` with all deps. `lib.rs` + thin `main.rs` with clap subcommands. `error.rs` with `NovaNetError` enum. `config.rs` with connection config + root discovery (`resolve_root()`). `db.rs` with neo4rs pool. |
| 2.2 | Implement `parsers/yaml_node.rs` | Parse 35 YAML node definitions with `locale_behavior` validation. **MUST fail-fast** if any YAML is missing `locale_behavior` — no silent defaults, bail with file path. |
| 2.3 | Implement `parsers/relations.rs` | Parse `relations.yaml` (list format + `family` + multi-source/target). **Must complete before generators that consume relations.** |
| 2.4 | Implement `generators/organizing.rs` | v9 Cypher for Realm, Layer, Trait, EdgeFamily |
| 2.5 | Implement `generators/layer.rs` | YAML → `layers.ts` via MiniJinja template |
| 2.6 | Implement `generators/kind.rs` | Kind nodes + `schema_hint`, `context_budget` + facet rels |
| 2.7 | Implement `generators/edge_schema.rs` | EdgeKind nodes + `cypher_pattern` + FROM/TO_KIND (depends on 2.3) |
| 2.8 | Implement `generators/autowire.rs` | OF_KIND wiring Cypher statements |
| 2.9 | Implement `generators/hierarchy.rs` | organizing-principles.yaml → `hierarchy.ts` via MiniJinja template |
| 2.10 | Implement `generators/mermaid.rs` | Mermaid flowchart with Realm/Layer/Trait coloring |
| 2.11 | Implement `commands/schema.rs` | `novanet schema generate` (orchestrates all 7 generators in order) + `novanet schema validate` (YAML ↔ Neo4j) |
| 2.12 | Run `novanet schema generate` | Validate all 7 generators produce correct output. **Dry-run only**: verify Cypher syntax (parse check), TypeScript compiles, Mermaid renders. Actual Neo4j execution deferred to Phase 4 (avoids circular dependency). |
| 2.13 | Delete `packages/schema-tools/` | Remove the entire TS package (absorbed into Rust) |
| 2.14 | Delete `packages/cli/` | Remove the empty stub package |
| 2.15 | Update root `package.json` + `pnpm-workspace.yaml` | Remove schema-tools and cli from workspace, alias `novanet` commands |

**Generator execution order**: Organizing → Kind → EdgeSchema → Layer → Mermaid → Autowire → Hierarchy

**Parallelization**: After 2.1–2.3 (foundation), tasks 2.4–2.10 are independent — use `spn-powers:dispatching-parallel-agents`.

**Gate**: Ralph Wiggum #2 — `novanet schema generate` produces all 7 outputs (dry-run: Cypher parses, TS compiles, Mermaid renders), `novanet schema validate` passes (YAML-only, no Neo4j), EdgeKind count by family matches spec (23+7+7+6+2+2=47), `packages/schema-tools/` deleted, `packages/cli/` deleted

#### Phase 3: TypeScript Types + Core (~core/src)

| # | Task | Description |
|---|------|-------------|
| 3.1 | Generate `KIND_META` + derived maps | Single record replaces 4 separate classification systems |
| 3.2 | Kill `NodeCategory` | Delete from core, update `filters/types.ts` to use Layer directly |
| 3.3 | Update filters | `NovaNetFilter.ts` — kill NodeCategory expansion. CypherGenerator moves to Rust (`filter/build.rs`), keep thin TS types for Studio |
| 3.4 | Update graph module | `layers.ts`, `hierarchy.ts`, `types.ts` — Realm/Layer/Trait types |
| 3.5 | PascalCase→lowercase audit | ~140 string literals: `'Global'`→`'global'`, `'localeKnowledge'`→`'knowledge'` |
| 3.6 | Update tests | Schema sync, hierarchy, generator, convention tests |

| 3.7 | Delete `core/scripts/` | All scripts absorbed into `novanet` subcommands (db seed, schema validate, locale import) |
| 3.8 | Delete `core/src/parsers/` | Absorbed into `parsers/locale_md.rs` in Rust |
| 3.9 | Delete `core/src/services/` | Absorbed into `search/` module in Rust |
| 3.10 | Delete `core/src/generators/` | Absorbed into `generators/` module in Rust |
| 3.11 | Delete `core/src/db/client.ts` | Replaced by `db.rs` (neo4rs) in Rust |

**Gate**: Ralph Wiggum #3 — `pnpm type-check` + `pnpm test --filter=@novanet/core` pass, no `NodeCategory` refs in `packages/core/**` (Studio keeps NodeCategory until Phase 5), eliminated TS areas deleted

#### Phase 4: Neo4j Migration (~db)

| # | Task | Description |
|---|------|-------------|
| 4.1a | Update `00-constraints.cypher` | Drop v8 meta constraints, add v9 (6 types) |
| 4.1b | Add facet property indexes | 8 indexes for faceted query performance (see below) |
| 4.2 | Regenerate seeds | Output of Phase 2 generators |
| 4.3 | Rename autowire | `99-autowire-subcategories.cypher` → `99-autowire-kinds.cypher` |
| 4.4 | Clean rebuild | `novanet db reset` |
| 4.5 | Run integrity tests | Meta-graph integrity queries (all 3 checks) |
| 4.6 | Audit query files | `queries/*.cypher` — update Scope/Subcategory references |

**Task 4.1b — Facet property indexes** (8 indexes for faceted navigation performance):

```cypher
// Kind facet properties (used by novanet query --kind filtering)
CREATE INDEX kind_context_budget IF NOT EXISTS FOR (k:Kind) ON (k.context_budget);
CREATE INDEX kind_traversal_depth IF NOT EXISTS FOR (k:Kind) ON (k.traversal_depth);
CREATE INDEX kind_generation_count IF NOT EXISTS FOR (k:Kind) ON (k.generation_count);

// EdgeKind facet properties (used by edge family filtering)
CREATE INDEX edgekind_temperature IF NOT EXISTS FOR (ek:EdgeKind) ON (ek.temperature_threshold);

// Output quality indexes (used by quality-score sorting/filtering)
CREATE INDEX pagel10n_quality IF NOT EXISTS FOR (n:PageL10n) ON (n.quality_score);
CREATE INDEX blockl10n_quality IF NOT EXISTS FOR (n:BlockL10n) ON (n.quality_score);

// Prompt fingerprint indexes (used by generation deduplication)
CREATE INDEX pagel10n_fingerprint IF NOT EXISTS FOR (n:PageL10n) ON (n.prompt_fingerprint);
CREATE INDEX blockl10n_fingerprint IF NOT EXISTS FOR (n:BlockL10n) ON (n.prompt_fingerprint);
```

**Gate**: Ralph Wiggum #4 — all integrity tests pass, Neo4j schema matches YAML, no v8 labels in DB, all 8 facet indexes exist (`SHOW INDEXES`)

#### Phase 5: Studio Migration (~studio, existing features)

| # | Task | Description |
|---|------|-------------|
| 5.0 | localStorage migration | Version-aware migration: add `schemaVersion: 9` field to both zustand stores (`novanet-ui`, `novanet-filter`). On init, check version — if missing or < 9, clear store and re-initialize with v9 defaults. This replaces v8 dead keys (`collapsedScopes`, `dataMode`) cleanly. Use zustand `version` field in `persist()` config + `migrate()` callback. **Do first** to avoid fighting stale state during development. |
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
| 6.3 | Create `useNavigationMode.ts` | Mode-aware query builder (calls `novanet filter build` subprocess) |
| 6.4 | Create `FacetFilterPanel.tsx` | Sidebar: Realm/Layer/Trait/EdgeFamily checkboxes (populated from `MATCH (n:Meta)`) |
| 6.5 | Create navigation API | `/api/graph/navigation/route.ts` — uses `novanet filter build` subprocess for Cypher generation |
| 6.6 | Create `lib/novanetBridge.ts` | Thin subprocess wrapper: `execFile('novanet', ['filter', 'build', ...])` — shared by graph + navigation routes |
| 6.7 | Context-aware ViewPicker | Data/Query mode → show YAML views as filtered subgraphs; Meta mode → show ontology views (taxonomy tree, edge schema) |
| 6.8 | New keyboard presets | `T` = Trait cycle, `E` = EdgeFamily filter, update `?` help modal |

**Gate**: Ralph Wiggum #6 — all 4 navigation modes work, facet filters are dynamic from meta-graph, ViewPicker context-aware, `novanet filter build` subprocess integration working

#### Phase 7A: Rust Runtime CLI + Basic TUI + Documentation (v9.0)

Crate scaffolding and generators were completed in Phase 2. Phase 7A builds
the runtime CLI commands (read/write/search/filter/locale/db), a basic functional TUI,
and documentation. **Advanced TUI features are deferred to Phase 7B (v9.5).**

| # | Task | Description |
|---|------|-------------|
| **Runtime CLI commands** | | |
| 7.1 | Implement read commands | `novanet data`, `novanet meta`, `novanet overlay`, `novanet query` — 4 navigation modes with `--realm/--layer/--trait/--edge-family/--kind/--format` flags |
| 7.1b | Add unit tests for core modules | Unit tests for `cypher.rs` (query builder → correct Cypher for facet combinations), `facets.rs` (filter intersection logic), `meta.rs` (type mapping). Integration tests with Neo4j testcontainer for end-to-end command output. |
| 7.2 | Implement write commands | `novanet node create/edit/delete`, `novanet relation create/delete` — validate against meta-graph (Kind exists, correct Realm/Layer/Trait, auto-wire OF_KIND) |
| 7.3 | Implement `novanet db seed/migrate/reset` | Execute Cypher files from `packages/db/seed/` and `packages/db/migrations/` |
| 7.4 | Implement `novanet locale parse/import` | Parse markdown locale knowledge files → structured JSON → Neo4j import |
| 7.5 | Implement `novanet search --hybrid` | Hybrid vector + graph search (graph traversal + vector similarity) |
| 7.6 | Implement `novanet filter build` | JSON filter spec → Cypher string (stdin → stdout for Studio subprocess) |
| 7.7 | Implement `novanet doc generate/validate` | Generate Mermaid views, validate doc sync |
| **Basic TUI (v9.0 scope)** | | |
| 7.8a | TUI scaffold | App state machine (`Loading`/`Ready` variants), basic layout (tree + detail + status bar), mode toggle (1/2/3/4), async channel bridge (`runtime.rs`), dirty-flag render loop |
| 7.8b | TUI taxonomy tree | Tree widget with Realm > Layer > Kind hierarchy, collapse/expand, arrow key navigation |
| 7.8c | TUI async + filters | Async Neo4j queries via mpsc channel bridge, loading states, facet filter popup (`f` key) |
| **Documentation** | | |
| 7.9 | Update Claude skills | `novanet-architecture`, `novanet-sync` — v9 terminology |
| 7.10 | Update Claude commands | `novanet-arch`, `novanet-sync` — v9 references |
| 7.11 | Update CLAUDE.md files | Root, core, studio — v9 terminology and version |
| 7.12 | Update docs | NOVANET-PITCH, plan docs, _index.yaml, README |
| 7.13 | Update turbo generators | Scaffold templates with v9 fields |

**Parallelization**: Runtime CLI commands (7.1–7.7) are largely independent — use `spn-powers:dispatching-parallel-agents`. TUI tasks (7.8a–7.8c) are sequential.

**Gate**: Ralph Wiggum #7A — all `novanet` subcommands work (`data/meta/query/node/schema/db/locale/filter/search/doc`), unit tests pass, `novanet tui` launches with taxonomy tree + async queries, all docs reference v9, no stale v8 terminology anywhere

#### Phase 7B: Advanced TUI — SuperNovae Galaxy (v9.5, deferred)

> **Scope**: All advanced TUI features below are a separate project (v9.5).
> They depend on Phase 7A's basic TUI scaffold being stable.
> Add crates `ratatui-macros`, `tui-tree-widget`, `directories` when starting 7B.

| # | Task | Description |
|---|------|-------------|
| 7.8d | TUI search + detail | Nucleo fuzzy search (`search.rs`, `/` key), Kind detail pane, edge explorer (`e` key) |
| 7.8e | TUI CRUD dialogs | Input forms for node create/edit/delete (`dialogs.rs`, `n`/`d` keys), relation CRUD (`r` key), confirmation prompts, Cypher preview (`c` key) |
| 7.8f | TUI visual theme | SuperNovae Galaxy theme: deep space palette, matrix rain canvas, sparklines, gauges, BigText header, animated transitions (see **TUI Visual Design** below) |
| 7.8g | TUI dashboard mode | Mission control dashboard with live Neo4j stats: node/edge counts per Realm, sparkline history, gauge health indicators, edge traffic heatmap |
| 7.8h | TUI animations | Boot sequence (matrix rain → logo fade-in → dashboard), typing effects on status bar, pulsing cursor on active panel, starfield background on idle |
| 7.8i | TUI ASCII logo & branding | Saturn-graph animated logo, SuperNovae Studio + NovaNet branding, BigText variants, compact/full/animated modes, boot integration (see **TUI Logo & Branding** below) |
| 7.8j | TUI onboarding flow | First-run detection (`~/.novanet/init`), welcome screen, Neo4j connection wizard, schema discovery, guided 5-step tour, quick-reference card (see **TUI Onboarding** below) |
| 7.8k | TUI command palette & UX | `:` command palette (fuzzy via nucleo), contextual `?`/`??` help, breadcrumbs, toast notifications, action menus, clipboard yank, bookmarks (see **TUI Enhanced UX** below) |
| 7.8l | TUI wow effects | Particle burst on CRUD, CRT scanlines, screen shake on delete, glitch transitions, nebula pulse, aurora idle, heatmap glow, data stream, warp speed, constellation lines (see **TUI Wow Effects** below) |

**Parallelization**: 7.8d-7.8e are sequential (search before CRUD). 7.8f-7.8l are largely independent visual features.

**Gate**: Ralph Wiggum #7B — all advanced TUI features work, boot animation completes, galaxy theme renders on 80x24+, no regressions on 7A features

---

### TUI Visual Design: SuperNovae Galaxy Theme

The `novanet tui` is NOT a boring data viewer. It's a **mission control cockpit** for
the NovaNet context graph — inspired by NASA flight control screens, 90s terminal
aesthetics (Hackers, War Games, Ghost in the Shell), and the Matrix digital rain.
Every pixel counts. Real data, real animations, zero placeholder energy.

#### Color Palette: Deep Space

All colors use true RGB (`Color::Rgb`). The palette evokes a dark nebula with
electric accents — purple/blue dominant, cyan highlights, supernova bursts of color.

```
BACKGROUND LAYER (deep space)
  bg_void:        Rgb(8, 10, 18)      // #080a12  almost-black deep blue
  bg_panel:       Rgb(13, 17, 30)     // #0d111e  panel background
  bg_active:      Rgb(20, 25, 45)     // #14192d  active panel highlight
  bg_hover:       Rgb(30, 35, 60)     // #1e233c  hover state

ACCENT LAYER (nebula glow)
  nebula_purple:  Rgb(139, 92, 246)   // #8b5cf6  primary purple
  nebula_violet:  Rgb(124, 58, 237)   // #7c3aed  deep violet
  nebula_indigo:  Rgb(99, 102, 241)   // #6366f1  indigo accent
  nebula_blue:    Rgb(59, 130, 246)   // #3b82f6  electric blue

SIGNAL LAYER (data highlights)
  cyber_cyan:     Rgb(34, 211, 238)   // #22d3ee  primary data color
  cyber_teal:     Rgb(20, 184, 166)   // #14b8a6  secondary data
  matrix_green:   Rgb(34, 197, 94)    // #22c55a  matrix rain / success
  plasma_pink:    Rgb(236, 72, 153)   // #ec4899  hot data / alerts
  solar_amber:    Rgb(245, 158, 11)   // #f59e0b  warnings / warm accents
  nova_white:     Rgb(226, 232, 240)  // #e2e8f0  primary text
  star_dim:       Rgb(100, 116, 139)  // #64748b  secondary text

REALM COLORS (spatial zones in taxonomy tree)
  realm_global:   Rgb(42, 161, 152)   // #2aa198  solarized cyan
  realm_project:  Rgb(108, 113, 196)  // #6c71c4  solarized violet
  realm_shared:   Rgb(203, 75, 22)    // #cb4b16  solarized orange

TRAIT BORDERS (locale behavior encoding)
  trait_invariant:  solid line,  cyber_cyan
  trait_localized:  dashed line, matrix_green
  trait_knowledge:  double line, nebula_purple
  trait_derived:    dotted line, star_dim
  trait_job:        thin line,   solar_amber
```

#### Typography

```
HEADER      tui-big-text (PixelSize::Quadrant), nebula_purple, BOLD
PANEL TITLE Block::bordered().title(), cyber_cyan, BOLD
BODY TEXT   nova_white, normal
DIM TEXT    star_dim, DIM modifier
HIGHLIGHT   bg_active background + cyber_cyan foreground + BOLD
SELECTED    plasma_pink background + bg_void foreground + BOLD
SEARCH HIT  solar_amber + UNDERLINED
```

#### Layout: Mission Control

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  ███╗   ██╗ ██████╗ ██╗   ██╗ █████╗ ███╗   ██╗███████╗████████╗         │
│  ████╗  ██║██╔═══██╗██║   ██║██╔══██╗████╗  ██║██╔════╝╚══██╔══╝         │
│  ██╔██╗ ██║██║   ██║██║   ██║███████║██╔██╗ ██║█████╗     ██║            │
│  ██║╚██╗██║██║   ██║╚██╗ ██╔╝██╔══██║██║╚██╗██║██╔══╝     ██║            │
│  ██║ ╚████║╚██████╔╝ ╚████╔╝ ██║  ██║██║ ╚████║███████╗   ██║            │
│  ╚═╝  ╚═══╝ ╚═════╝   ╚═══╝  ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝   ╚═╝            │
│                                                        v9.0.0 | mission ctrl │
├──────────────────────┬──────────────────────────────────────────────────────┤
│  TAXONOMY            │  KIND DETAIL                                         │
│  ─────────           │  ──────────                                          │
│  🌍 global           │  ┌─ Page ──────────────────────────────────────────┐ │
│  ├── ⚙️ config        │  │ Realm:    project         Layer: structure      │ │
│  │   └── Locale      │  │ Trait:    invariant        Edges: 12 in / 8 out │ │
│  ├── 📚 knowledge     │  │ Budget:   2048 tokens     Hint: "Core page..." │ │
│  │   ├── Constraint  │  ├────────────────────────────────────────────────┤ │
│  │   ├── Expression  │  │ EDGES IN        │ EDGES OUT                    │ │
│  │   ├── Locale...   │  │ ──────────      │ ──────────                   │ │
│  │   └── ...         │  │ HAS_PAGE (own)  │ HAS_BLOCK (own)             │ │
│  📦 project           │  │ LINKS_TO (sem)  │ HAS_OUTPUT (l10n)           │ │
│  ├── 🏗️ foundation    │  │ SUBTOPIC (own)  │ HAS_PROMPT (gen)            │ │
│  │   ├── Project     │  │                 │ USES_CONCEPT (sem)          │ │
│  │   ├── BrandId     │  │                 │ OF_TYPE (own)               │ │
│  │   └── ProjectL10n │  └────────────────────────────────────────────────┘ │
│  ├── 📐 structure     │                                                     │
│  │   ├── ► Page      │  CYPHER PREVIEW                                     │
│  │   └── Block       │  ──────────────                                     │
│  ├── 💡 semantic      │  MATCH (k:Kind {key: 'Page'})                       │
│  │   ├── Concept     │  OPTIONAL MATCH (k)-[:IN_REALM]->(r:Realm)          │
│  │   └── ConceptL10n │  OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)          │
│  └── ...             │  RETURN k, r, l                                     │
│  🎯 shared            │                                                     │
│  └── ...             │                                                     │
├──────────────────────┴──────────────────────────────────────────────────────┤
│  DASHBOARD                                                                  │
│  ─────────                                                                  │
│  Nodes by Realm          Edges by Family          Schema Health             │
│  ▁▂▃▅▇█▇▅▃▂▁ global:14  ████░░░ own:23 (34%)     ████████░░ 82%           │
│  ▂▃▅▇█▇▅▃▂▁▂ project:16 █████░░ sem:18 (27%)     ▁▂▃▅▇█▇▅▃ history       │
│  ▁▁▂▃▅▃▂▁▁▁▁ shared:5   ███░░░░ l10n:14 (21%)                             │
│                          ██░░░░░ gen:8 (12%)      Jobs: 2 running          │
│                          █░░░░░░ mine:4 (6%)      Last seed: 3m ago        │
├─────────────────────────────────────────────────────────────────────────────┤
│ [1]Data [2]Meta [3]Overlay [4]Query │ /search │ f:filter │ ?:help │ q:quit │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Widgets Inventory

| Widget | Crate | Use |
|--------|-------|-----|
| `BigText` | `tui-big-text` | NOVANET logo header (PixelSize::Quadrant) |
| `Sparkline` | ratatui built-in | Node counts over time, schema health history |
| `Gauge` / `LineGauge` | ratatui built-in | Schema health %, generation progress |
| `BarChart` | ratatui built-in | Edge family distribution |
| `Chart` (Line) | ratatui built-in | Query latency trends |
| `Table` | ratatui built-in | Kind detail, edge lists |
| `Tree` | custom widget | Realm > Layer > Kind hierarchy |
| `Tabs` | ratatui built-in | Navigation mode selector |
| `Canvas` | ratatui built-in | Matrix rain, starfield, connection graph |
| `Popup` | `tui-popup` | Filter panel, CRUD dialogs, help modal |
| `ScrollView` | `tui-scrollview` | Long Cypher preview, edge lists |
| `Paragraph` | ratatui built-in | Detail panes, status messages |
| Custom `StatusBar` | Widget trait | Bottom bar with mode/search/hints |
| Custom `Chip` | Widget trait | Realm/Layer/Trait tag pills |
| Custom `MatrixRain` | Canvas + Widget | Background animation |
| Custom `StarField` | Canvas + Widget | Idle screen animation |

#### Custom Widgets

##### Chip (Tag Pill)

Small rounded tag showing Realm/Layer/Trait values with semantic coloring:

```
 ┌──────────┐  ┌───────────┐  ┌────────────┐
 │ 🌍 global │  │ 📐 struct  │  │ ● invariant│
 └──────────┘  └───────────┘  └────────────┘
  realm_global   Layer color    Trait + dot
  bg + text      bg + text      border style
```

Implementation: Custom `Widget` trait. Each chip is a 1-row `Block` with colored
background, emoji prefix, and text. Chips are composable in horizontal `Layout`.

```rust
struct Chip {
    label: String,
    emoji: String,
    fg: Color,
    bg: Color,
    border: BorderType,  // encodes Trait
}

impl Widget for Chip {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(self.border)
            .border_style(Style::new().fg(self.fg))
            .style(Style::new().bg(self.bg));
        let inner = block.inner(area);
        block.render(area, buf);
        Line::from(vec![
            Span::raw(&self.emoji),
            Span::raw(" "),
            Span::styled(&self.label, Style::new().fg(self.fg).bold()),
        ]).render(inner, buf);
    }
}
```

##### MatrixRain

Canvas-based digital rain effect for boot screen and idle state:

```rust
struct MatrixRain {
    columns: Vec<RainColumn>,
    tick: u64,
}

struct RainColumn {
    x: f64,
    drops: Vec<RainDrop>,
}

struct RainDrop {
    y: f64,
    speed: f64,
    char: char,
    brightness: u8,  // 255=head, fades to 0
}

impl Widget for MatrixRain {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Canvas with Braille markers for high-res rain
        let canvas = Canvas::default()
            .x_bounds([0.0, area.width as f64])
            .y_bounds([0.0, area.height as f64])
            .paint(|ctx| {
                for col in &self.columns {
                    for drop in &col.drops {
                        let green = (drop.brightness as f64 * 0.77) as u8;
                        ctx.print(
                            col.x, drop.y,
                            Line::styled(
                                drop.char.to_string(),
                                Style::new().fg(Color::Rgb(0, green, 0))
                            )
                        );
                    }
                }
            });
        canvas.render(area, buf);
    }
}
```

Characters: mix of katakana (U+30A0–U+30FF), latin, digits, NovaNet-specific
glyphs (`⊕ ⊗ ◈ ◉ ▣ ⬡`). Head drops are `matrix_green` bright, trail fades
through dim green to `bg_void`. Speed varies per column (0.5–2.0 cells/frame).

##### StarField

Idle animation showing drifting stars with parallax depth:

```rust
struct StarField {
    stars: Vec<Star>,
    tick: u64,
}

struct Star {
    x: f64, y: f64,
    depth: u8,  // 1=near (bright, fast), 3=far (dim, slow)
}
```

Near stars: `nova_white` + BOLD, move 2px/frame.
Mid stars: `star_dim`, move 1px/frame.
Far stars: `Rgb(40, 50, 70)`, move 0.5px/frame.

#### Animations

##### Boot Sequence (2-3 seconds)

```
Frame 0-30:   Matrix rain fills screen, characters cascade
Frame 30-45:  Rain slows, center clears in circle
Frame 45-60:  NOVANET BigText fades in (char by char, purple glow)
Frame 60-75:  Subtitle types in: "Context Graph Mission Control v9.0"
Frame 75-90:  Dashboard panels slide in from edges
Frame 90+:    Normal operation, matrix rain fades to starfield
```

##### Panel Transitions

- **Panel focus**: Border color transitions from `star_dim` to `cyber_cyan` over
  5 frames. Active panel border glows (BOLD modifier toggle).
- **Mode switch**: Current panels fade (DIM), new panels slide in from bottom.
- **Search activation**: Search bar expands from status bar, background dims.
- **Data loading**: Gauge widget with animated fill + throbber character cycle
  (`⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏` braille spinner, `cyber_cyan`).

##### Status Bar Effects

```
┌─────────────────────────────────────────────────────────────────────────┐
│ ◉ CONNECTED  bolt://localhost:7687  │  35 Kinds  66 Edges  3 Realms   │
│ ▂▃▅▇▅▃▂ 4ms │ ⣀⣤⣶⣿⣶⣤⣀ mem: 142MB  │  ⠋ syncing...                    │
└─────────────────────────────────────────────────────────────────────────┘
```

- Connection dot: pulsing green (`●` ↔ `◉` every 30 frames)
- Latency sparkline: last 20 query times, auto-scales
- Memory braille block: current heap usage
- Spinner: braille rotation during async operations

#### Keyboard Map (90s Hacker Feel)

```
NAVIGATION                       ACTIONS                        MODES
───────────                      ───────                        ─────
↑↓  / j k   Tree nav             n   Create node               1  Data mode
←→  / h l   Panel switch         d   Delete (shake + confirm)  2  Meta mode
Enter       Select / action menu  e   Edge explorer             3  Overlay mode
Space       Toggle select         r   Relation CRUD             4  Query mode
Tab         Next panel            c   Cypher preview
Backspace   Go up level           u   Undo last action          DISPLAY
                                                                ───────
SEARCH & FILTER                  CLIPBOARD                      t  Toggle tree
───────────────                  ─────────                      s  Toggle stats
/           Fuzzy search          y   Yank (copy to clip)       b  Toggle borders
:           Command palette       p   Paste Cypher query        m  Toggle matrix bg
f           Facet filters         B   Set bookmark              |  Split view
R           Realm filter          '   Jump to bookmark
L           Layer filter          x   Export view to file       EFFECTS
T           Trait filter                                        ───────
E           Edge filter          SYSTEM                         Ctrl+R  CRT scanlines
                                 ──────                         Ctrl+M  Mouse toggle
HELP                             F5   Refresh (warp effect)
────                             F11  Fullscreen toggle         MOUSE (opt-in)
?           Quick help (1-line)  q    Quit                      ─────
??          Full reference card                                 Click   Focus panel
                                                                Scroll  Scroll list
                                                                Drag    Resize borders
                                                                DblClk  Expand/collapse
```

#### Crate Dependencies (TUI-specific, Phase 7B additions)

These crates extend the v9.0 Cargo.toml (§ Phase 1) for the Phase 7B galaxy-themed TUI.
`ratatui`, `crossterm`, and `nucleo` are already declared in v9.0; the rest are new additions:

> **ratatui 0.30 upgrade note**: ratatui 0.30.0 (requires Rust 1.86+) introduces
> breaking changes: `ratatui::init()`/`restore()` replace manual terminal setup,
> `block::Title` removed (use `Line`), `Marker::Block` renamed to `Bar`,
> terminal module made private, `Table::new()` requires column widths.
> Consider upgrading from 0.29 → 0.30 at Phase 7B start (bump rust-version to 1.86).
> All Phase 7A code patterns remain valid on 0.29.

```toml
# Cargo.toml additions for Phase 7B TUI
[dependencies]
# Already in v9.0 Cargo.toml:
# ratatui = { version = "0.29", optional = true, features = ["all-widgets"] }
# crossterm = { version = "0.28", optional = true, features = ["event-stream"] }
# nucleo = "0.5"

# Phase 7B additions (optional, gated behind `tui` feature)
tui-big-text = { version = "0.7", optional = true }
tui-popup = { version = "0.7", optional = true }
tui-scrollview = { version = "0.6", optional = true }
rand = "0.8"                 # matrix rain / glitch randomness
unicode-width = "0.2"        # correct CJK character widths
dirs = "5"                   # home directory detection (onboarding)
chrono = "0.4"               # onboarding timestamp

[features]
tui = [
    "dep:ratatui", "dep:crossterm",
    "dep:tui-big-text", "dep:tui-popup", "dep:tui-scrollview",
]
```

#### File Structure

```
tools/novanet/src/tui/
├── mod.rs                       # TUI entry point + app state machine
├── theme.rs                     # SuperNovae Galaxy palette + styles
├── layout.rs                    # Mission control layout builder
├── runtime.rs                   # Async channel bridge (mpsc)
├── widgets/
│   ├── mod.rs
│   ├── tree.rs                  # Taxonomy tree (Realm > Layer > Kind)
│   ├── detail.rs                # Kind detail pane
│   ├── chip.rs                  # Realm/Layer/Trait tag pills
│   ├── dashboard.rs             # Stats dashboard (sparklines, gauges)
│   ├── matrix_rain.rs           # Matrix digital rain effect
│   ├── starfield.rs             # Parallax star background
│   ├── status_bar.rs            # Bottom status with sparkline + spinner
│   ├── search.rs                # Nucleo fuzzy search overlay
│   ├── cypher_preview.rs        # Syntax-highlighted Cypher viewer
│   ├── breadcrumb.rs            # Navigation breadcrumb bar
│   ├── toast.rs                 # Auto-dismiss notification toasts
│   └── constellation.rs         # Inline edge lines on tree hover
├── dialogs/
│   ├── mod.rs
│   ├── node_form.rs             # Create/edit node dialog
│   ├── relation_form.rs         # Create/edit relation dialog
│   ├── confirm.rs               # Delete confirmation
│   ├── filter_panel.rs          # Facet filter checkboxes
│   └── help.rs                  # Quick reference card modal
├── animations/
│   ├── mod.rs
│   ├── boot.rs                  # Boot sequence orchestrator
│   ├── transitions.rs           # Panel focus/mode transitions
│   ├── ticker.rs                # Frame-based animation timer
│   └── effects.rs               # Particle burst, screen shake, glitch
├── onboarding/
│   ├── mod.rs
│   ├── welcome.rs               # Welcome screen + schema discovery
│   ├── connection.rs            # Neo4j connection wizard
│   ├── tour.rs                  # 5-step guided panel tour
│   └── quick_ref.rs             # Keyboard reference card
├── branding/
│   ├── mod.rs
│   ├── logo.rs                  # Saturn-graph ASCII logo widget
│   ├── bigtext.rs               # BigText NOVANET header
│   └── splash.rs                # Full splash screen compositor
└── ux/
    ├── mod.rs
    ├── command_palette.rs       # Fuzzy-searchable command palette
    ├── action_menu.rs           # Context-sensitive action menus
    ├── clipboard.rs             # Yank/paste Cypher queries
    ├── bookmarks.rs             # Kind bookmarks + jump
    └── session.rs               # History, undo, export
```

---

#### TUI Logo & Branding

##### The NovaNet Logo: Saturn Context Graph

The logo represents NovaNet's core identity: a **context graph** visualized as a
Saturn-like planet with orbiting, connected nodes. The central planet contains
"NOVANET", the horizontal ring represents edge connections, and 8 orbiting nodes
represent the graph's layers — all connected by visible edges.

**Design concept**:
- Central body = NovaNet core (rounded box, `nebula_purple` border)
- Horizontal ring = edge connections (`═══╳` double lines, `cyber_cyan`)
- 8 orbit nodes = graph layers (`◉` symbols, realm-colored)
- Edges between nodes = graph relationships (`──` `╱` `╲` lines)
- Stars = deep space atmosphere (`· ✦ ★`, `star_dim`)
- Branding = "SuperNovae Studio" below

##### Full Logo (boot splash, help screen, about dialog)

```
                        .    ✦    .
                 ✦                       ✦
             ·        ◉ ─ ─ ─ ◉        ·
           ◉─────── ╱           ╲ ───────◉
          ╱       ╱  ╭─────────╮  ╲       ╲
    ◉────╱──────╱    │ N O V A │    ╲──────╲────◉
  ════════════════╳  │  N E T  │  ╳════════════════
  ════════════════╳  │    ◉    │  ╳════════════════
    ◉────╲──────╲    │         │    ╱──────╱────◉
          ╲       ╲  ╰─────────╯  ╱       ╱
           ◉─────── ╲           ╱ ───────◉
             ·        ◉ ─ ─ ─ ◉        ·
                 ✦                       ✦
                        .    ✦    .

                 SuperNovae Studio
              context graph engine v9.0
```

Colors per element:
```
Stars (· ✦)     : star_dim Rgb(100, 116, 139)
Orbit nodes (◉) : top pair realm_global, left/right realm_project, bottom realm_shared
Edges (── ╱ ╲)  : nebula_indigo Rgb(99, 102, 241), DIM
Ring (═══╳)     : cyber_cyan Rgb(34, 211, 238), BOLD
Planet border    : nebula_purple Rgb(139, 92, 246)
Planet text      : nova_white Rgb(226, 232, 240), BOLD
Center dot (◉)  : plasma_pink Rgb(236, 72, 153), pulsing animation
Branding line 1  : star_dim → nova_white (typewriter)
Branding line 2  : star_dim, DIM
```

##### Compact Logo (status bar corner, 3 lines)

```
◉─╮╭──╮╭─◉
═══╳│◉N│╳═══
◉─╯╰──╯╰─◉
```

Used in status bar left corner when dashboard is minimized. Colors: `cyber_cyan`
ring, `nebula_purple` planet, `plasma_pink` center dot.

##### Inline Logo (tab bar, window title, exports)

```
◉═╳◉╳═◉ NOVANET
```

Single line. `cyber_cyan` symbols, `nova_white` text, BOLD.

##### Boot Animation: Logo Reveal (3 seconds, 90 frames at 30fps)

The logo builds itself piece by piece — each element materializes as if emerging
from deep space. 6 stages:

```
STAGE 1: STARFIELD (frames 0-15)
─────────────────────────────────
Stars blink into existence across the full terminal.
Each starts as star_dim, some pulse to nova_white.

        ·              ✦
   ✦          ·                ·
         ·        ✦       ·
    ·         ·               ✦
              ✦        ·   ·     ✦
              ·

STAGE 2: NODES MATERIALIZE (frames 15-30)
──────────────────────────────────────────
8 orbit nodes (◉) appear one by one with matrix_green flash.
Each starts bright green, settles to its realm color.
Order: top-left → clockwise.

                        ·    ✦    ·
                 ✦                       ✦
             ·        ◉           ◉        ·
           ◉                                 ◉

    ◉                                             ◉

           ◉                                 ◉
             ·        ◉           ◉        ·

STAGE 3: EDGES CONNECT (frames 30-45)
──────────────────────────────────────
Lines draw from node to node — each edge animates source → target.
Color: nebula_indigo, starts bright then settles to normal.

             ·        ◉ ─ ─ ─ ◉        ·
           ◉─────── ╱           ╲ ───────◉
          ╱       ╱                ╲       ╲
    ◉────╱──────╱                    ╲──────╲────◉

    ◉────╲──────╲                    ╱──────╱────◉
          ╲       ╲                ╱       ╱
           ◉─────── ╲           ╱ ───────◉
             ·        ◉ ─ ─ ─ ◉        ·

STAGE 4: PLANET FORMS (frames 45-55)
─────────────────────────────────────
Central box draws itself: top-left corner → clockwise border → fill.
Border: nebula_purple. Interior: bg_panel.

                     ╭─────────╮
                     │         │
                     │    ◉    │
                     │         │
                     ╰─────────╯

STAGE 5: RING SWEEPS (frames 55-70)
────────────────────────────────────
═══╳ characters race outward from center in both directions.
Color: cyber_cyan, bright head with dimming trail.

              ════════╳         ╳════════
              ════════╳         ╳════════

STAGE 6: TEXT + BRANDING (frames 70-90)
────────────────────────────────────────
"N O V A" types char by char, line 1 (nova_white, BOLD).
"N E T" types char by char, line 2 (nova_white, BOLD).
Center ◉ starts pulsing (plasma_pink).
"SuperNovae Studio" typewriter below (star_dim).
"context graph engine v9.0" fades in (star_dim, DIM).

    → Final state = full logo (as shown above)

TRANSITION TO DASHBOARD (frames 90+)
─────────────────────────────────────
Logo shrinks upward into BigText header position.
Dashboard panels slide in from left/right/bottom.
Matrix rain residue clears with a downward sweep.
```

##### Branding Guidelines

```
PRODUCT NAME     : NovaNet (PascalCase, one word)
                   "NOVANET" in all-caps for headers/logos
                   "novanet" lowercase for CLI binary

COMPANY          : SuperNovae Studio (two words, PascalCase each)
                   Never: "Supernovae", "Super Novae", "SuperNova"
                   Never: "SuperNovae Studio™" (no TM symbol)

TAGLINE          : "context graph engine"

VERSION FORMAT   : v9.0.0 (semver with v prefix)

COMBINED MARKS   : "NovaNet v9 — context graph engine"
                   "NovaNet by SuperNovae Studio"
                   "NOVANET | SuperNovae Studio"

CLI BINARY       : novanet (lowercase, no dash)
                   Usage: novanet [command] [flags]

CONFIG DIR       : ~/.novanet/
```

##### Rust Implementation: Logo Widget

```rust
/// Logo display modes
enum LogoMode {
    Full,                // Full Saturn graph (16 lines, boot splash)
    Compact,             // 3-line mini (status bar corner)
    Inline,              // 1-line (tab headers)
    Animated(LogoStage), // Boot animation
}

/// Boot animation stages
enum LogoStage {
    Stars { tick: u16 },
    Nodes { revealed: u8, tick: u16 },
    Edges { progress: f32 },
    Planet { draw_pct: f32 },
    Ring { sweep_pct: f32 },
    Text { chars_shown: u8 },
    Complete,
}

struct Logo {
    mode: LogoMode,
    frame: u64,
}

/// Branding constants
const BRAND_PRODUCT: &str = "NOVANET";
const BRAND_STUDIO: &str = "SuperNovae Studio";
const BRAND_TAGLINE: &str = "context graph engine";

impl Logo {
    fn advance(&mut self) -> bool {
        self.frame += 1;
        match &mut self.mode {
            LogoMode::Animated(stage) => {
                // Transition between stages based on frame count
                let next = match stage {
                    LogoStage::Stars { tick } if *tick >= 15 =>
                        Some(LogoStage::Nodes { revealed: 0, tick: 0 }),
                    LogoStage::Nodes { revealed, .. } if *revealed >= 8 =>
                        Some(LogoStage::Edges { progress: 0.0 }),
                    LogoStage::Edges { progress } if *progress >= 1.0 =>
                        Some(LogoStage::Planet { draw_pct: 0.0 }),
                    LogoStage::Planet { draw_pct } if *draw_pct >= 1.0 =>
                        Some(LogoStage::Ring { sweep_pct: 0.0 }),
                    LogoStage::Ring { sweep_pct } if *sweep_pct >= 1.0 =>
                        Some(LogoStage::Text { chars_shown: 0 }),
                    LogoStage::Text { chars_shown } if *chars_shown as usize
                        >= BRAND_PRODUCT.len() + BRAND_STUDIO.len() =>
                        Some(LogoStage::Complete),
                    LogoStage::Complete => return false, // animation done
                    _ => None,
                };
                if let Some(n) = next { *stage = n; }
                true // still animating
            }
            _ => false,
        }
    }
}

impl Widget for Logo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            LogoMode::Full => render_full_logo(area, buf),
            LogoMode::Compact => render_compact_logo(area, buf),
            LogoMode::Inline => render_inline_logo(area, buf),
            LogoMode::Animated(stage) => render_animated_logo(area, buf, stage),
        }
    }
}
```

---

#### TUI Onboarding Experience

First-time users see a guided welcome flow instead of the raw dashboard.
Onboarding runs once and stores completion state in `~/.novanet/init`.

##### First-Run Detection

```rust
fn is_first_run() -> bool {
    dirs::home_dir()
        .map(|h| !h.join(".novanet/init").exists())
        .unwrap_or(true) // No home dir → treat as first run
}

fn mark_onboarding_complete() -> std::io::Result<()> {
    let home = dirs::home_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "no home directory")
    })?;
    let path = home.join(".novanet/init");
    std::fs::create_dir_all(path.parent().expect("path has parent"))?;
    std::fs::write(&path, chrono::Utc::now().to_rfc3339())
}
```

##### Welcome Screen

After boot animation completes, first-run users see the welcome overlay.
Each status check animates with a braille spinner (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏) before
showing result. Failed checks show `✗` in plasma_pink with a troubleshooting hint.

```
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║      ◉ ─ ─ ◉          ◉ ─ ─ ◉                                        ║
║      ═══╳│◉ NOVANET│╳═══                                              ║
║      ◉ ─ ─ ◉          ◉ ─ ─ ◉                                        ║
║                                                                       ║
║      Welcome to NovaNet — context graph engine                        ║
║      by SuperNovae Studio                                             ║
║                                                                       ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║      ◉ Checking Neo4j connection...                                   ║
║        ├── bolt://localhost:7687 ─────── ● CONNECTED                  ║
║        ├── Database: neo4j ──────────── ● OK                          ║
║        └── APOC plugin ─────────────── ● LOADED                      ║
║                                                                       ║
║      ◉ Discovering schema...                                          ║
║        ├── 35 Kinds across 3 Realms                                   ║
║        ├── 47 EdgeKinds in 5 Families                                 ║
║        ├── 9 Layers                                                   ║
║        └── 5 Traits                                                   ║
║                                                                       ║
║      ◉ Ready!                                                         ║
║                                                                       ║
║      Press [Enter] to start guided tour                               ║
║      Press [Esc] to skip to dashboard                                 ║
║      Press [?] for keyboard shortcuts                                 ║
║                                                                       ║
╚═══════════════════════════════════════════════════════════════════════╝
```

Connection failure state:
```
║      ◉ Checking Neo4j connection...                                   ║
║        ├── bolt://localhost:7687 ─────── ✗ REFUSED                    ║
║        │   └── Hint: run `pnpm infra:up` from monorepo root          ║
║        └── Retrying in 3s... (⠹)                                     ║
```

##### Guided Tour (5 steps)

If user presses [Enter], a highlight overlay walks through each panel.
Each step highlights the target panel (cyber_cyan border glow, rest dimmed to 40%)
and shows a tooltip popup with explanation + [Next]/[Skip] navigation.

```
STEP 1/5: TAXONOMY TREE
────────────────────────
┌─ HIGHLIGHTED ────────────┐  ╭──────────────────────────────────╮
│  🌍 global               │  │ This is the taxonomy tree.        │
│  ├── ⚙ config            │  │ All 35 Kinds organized by         │
│  │   └── Locale          │  │ Realm > Layer hierarchy.          │
│  ├── 📚 knowledge        │  │                                   │
│  │   ├── Constraint      │  │ Navigate: ↑↓ or j/k              │
│  │   └── ...             │  │ Expand:   → or Enter              │
│  📦 project              │  │ Collapse: ← or Backspace          │
│  └── ...                 │  │                                   │
└──────────────────────────┘  │         [Next →]  [Skip tour]     │
                              ╰──────────────────────────────────╯

STEP 2/5: KIND DETAIL
─────────────────────
Properties, edges in/out, Realm/Layer/Trait chips, token budget.
"Select any Kind to see its full profile here."

STEP 3/5: CYPHER PREVIEW
─────────────────────────
Live Cypher query generated for the currently selected Kind.
"Press c to copy, or modify the query in Query mode (4)."

STEP 4/5: DASHBOARD
────────────────────
Sparklines, gauges, bar charts — live Neo4j stats.
"Real data, updating every 5 seconds."

STEP 5/5: STATUS BAR & MODES
─────────────────────────────
Navigation modes (1-4), search (/), command palette (:), help (?).
"Press : to open the command palette — it knows every action."
```

##### Quick Reference Card

At end of tour (or via `?` key → then `?` again for full card):

```
╔═══════════════════════════════════════════════════════════════════════╗
║  NOVANET QUICK REFERENCE                                              ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║  NAVIGATE              │ ACTIONS              │ VIEWS                  ║
║  ↑↓ jk   Tree nav      │ n   Create node      │ 1  Data mode          ║
║  ←→ hl   Panel focus   │ d   Delete            │ 2  Meta mode          ║
║  Enter   Select/menu   │ e   Edge explorer     │ 3  Overlay mode       ║
║  Tab     Next panel    │ r   Relation CRUD     │ 4  Query mode         ║
║  Bksp    Go up level   │ c   Cypher preview    │ t  Toggle tree        ║
║                        │ u   Undo              │ s  Toggle stats       ║
║  SEARCH & FILTER       │ y   Yank (copy)       │ b  Toggle borders     ║
║  /  Fuzzy search       │ B   Bookmark          │ m  Toggle matrix      ║
║  :  Command palette    │ '   Jump bookmark     │ |  Split view         ║
║  f  Facet filters      │ x   Export view       │                       ║
║                        │                       │ EFFECTS               ║
║  HELP                  │ SYSTEM                │ Ctrl+R CRT scanlines  ║
║  ?  Quick help         │ F5  Refresh           │ Ctrl+M Mouse toggle   ║
║  ?? Full reference     │ q   Quit              │                       ║
║                                                                       ║
║  [Esc] Close                                     SuperNovae Studio     ║
╚═══════════════════════════════════════════════════════════════════════╝
```

---

#### TUI Enhanced UX

##### Command Palette (`:` key)

VS Code-style fuzzy-searchable command palette. Triggered by `:` (colon), appears
as a centered overlay with input field + ranked results. Uses `nucleo` for fuzzy
matching (already in TUI dependencies).

```
╭──────────────────────────────────────────────────────╮
│ : seed█                                              │
├──────────────────────────────────────────────────────┤
│   db seed          Run database seed scripts         │
│ → db seed:reset    Reset database and re-seed        │
│   search seeds     Search for GEO/SEO seeds          │
│                                                      │
│ 3 commands · Tab select · Enter run · Esc close      │
╰──────────────────────────────────────────────────────╯
```

Implementation: Commands are registered in a `CommandRegistry` and scored by
nucleo on every keystroke:

```rust
struct Command {
    key: &'static str,           // "db.seed"
    label: &'static str,         // "db seed"
    description: &'static str,   // "Run database seed scripts"
    action: CommandAction,
    shortcut: Option<&'static str>, // "F5"
    category: CommandCategory,   // Navigation, Action, View, Filter, System
}

enum CommandAction {
    SwitchMode(NavigationMode),
    OpenDialog(DialogKind),
    RunQuery(String),
    TogglePanel(PanelId),
    ApplyFilter(FacetFilter),
    SystemCommand(SystemCmd),
}
```

Full command list (30+ commands):

| Command | Description | Category |
|---------|-------------|----------|
| `data mode` | Switch to Data navigation | Navigation |
| `meta mode` | Switch to Meta navigation | Navigation |
| `overlay mode` | Switch to Overlay navigation | Navigation |
| `query mode` | Switch to Query navigation | Navigation |
| `create node` | Open node creation dialog | Action |
| `create relation` | Open relation creation dialog | Action |
| `delete` | Delete selected item (with confirm) | Action |
| `edge explorer` | Open edge explorer for selected Kind | Action |
| `cypher preview` | Show Cypher query for current view | Action |
| `filter realm [name]` | Filter by Realm | Filter |
| `filter layer [name]` | Filter by Layer | Filter |
| `filter trait [name]` | Filter by Trait | Filter |
| `filter edge [family]` | Filter by EdgeFamily | Filter |
| `clear filters` | Remove all active filters | Filter |
| `toggle tree` | Show/hide taxonomy tree | View |
| `toggle stats` | Show/hide dashboard stats | View |
| `toggle borders` | Show/hide panel borders | View |
| `toggle matrix` | Toggle matrix rain background | View |
| `toggle crt` | Toggle CRT scanline effect | View |
| `split view` | Toggle split detail pane | View |
| `db seed` | Run seed scripts | System |
| `db reset` | Reset and re-seed | System |
| `db status` | Show connection info | System |
| `refresh` | Refresh all data (warp effect) | System |
| `export text` | Export view as text file | System |
| `export yaml` | Export schema as YAML | System |
| `export cypher` | Export current Cypher query | System |
| `bookmarks` | Show bookmark list | System |
| `history` | Show command history | System |
| `help` | Show keyboard reference | System |
| `mouse toggle` | Toggle mouse support | System |
| `quit` | Exit application | System |

##### Contextual Help (`?` key)

Pressing `?` shows help relevant to the currently focused panel.
Appears as a 1-line tooltip above the status bar (auto-dismiss after 5s):

```
Focus = Tree      → "Navigate ↑↓, expand →, collapse ←, search /"
Focus = Detail    → "Press e edges, c Cypher, n create, y yank YAML"
Focus = Dashboard → "Sparklines show 24h, gauges = live health"
Focus = Cypher    → "Edit query, Enter to execute, y to copy"
Focus = Search    → "Type to filter, Enter select, Esc cancel"
```

Press `??` (double question mark) for the full Quick Reference Card modal.

##### Breadcrumb Navigation

Top bar shows current navigation path, updating live as user navigates.
Each segment is color-coded by realm:

```
┌────────────────────────────────────────────────────────────────────┐
│ ◉ NOVANET │ Meta Mode │ 📦 project > 📐 structure > Page          │
└────────────────────────────────────────────────────────────────────┘
```

- `Backspace` = go up one level
- Click on any segment = jump to that level (mouse mode)
- Realm emoji + color matches taxonomy tree
- Mode indicator uses navigation mode name + number

```rust
struct Breadcrumb {
    segments: Vec<BreadcrumbSegment>,
    mode: NavigationMode,
}

struct BreadcrumbSegment {
    label: String,
    emoji: String,
    color: Color,      // realm or layer color
    level: BreadcrumbLevel,
}

enum BreadcrumbLevel {
    Root,              // "NOVANET"
    Mode,              // "Meta Mode"
    Realm(String),     // "📦 project"
    Layer(String),     // "📐 structure"
    Kind(String),      // "Page"
}
```

##### Toast Notifications

Auto-dismiss messages at bottom-right corner. Severity levels with colors:

```
SUCCESS (matrix_green):    ┌────────────────────────────┐
                           │ ✓ Node created: Block      │
                           └────────────────────────────┘

WARNING (solar_amber):     ┌────────────────────────────┐
                           │ ⚠ 3 Kinds have no edges    │
                           └────────────────────────────┘

ERROR (plasma_pink):       ┌────────────────────────────┐
                           │ ✗ Neo4j connection lost     │
                           └────────────────────────────┘

INFO (cyber_cyan):         ┌────────────────────────────┐
                           │ ◉ Schema refreshed (4ms)   │
                           └────────────────────────────┘
```

- Stack up to 3 visible, newest on top
- Auto-dismiss: 3s for success/info, 5s for warning, persist until Esc for error
- Animation: slide in from right (5 frames), fade out left (5 frames)
- Sound: terminal bell on error only (configurable)

```rust
struct Toast {
    message: String,
    severity: Severity,
    created_at: Instant,
    ttl: Duration,
    animation: ToastAnimation,
}

enum ToastAnimation {
    SlideIn { frame: u8 },
    Visible,
    FadeOut { frame: u8 },
    Done,
}
```

##### Action Menus

Press `Enter` on any item to open a context-sensitive action menu:

```
On Kind "Page":               On EdgeKind "HAS_BLOCK":
┌───────────────────┐         ┌───────────────────────┐
│ ◉ View detail     │         │ ◉ View source Kind    │
│ ✎ Edit Kind       │         │ ◉ View target Kind    │
│ + Create instance  │         │ ✎ Edit EdgeKind      │
│ 🔗 Edge explorer  │         │ ⌘ Run traversal       │
│ ⌘ Cypher query    │         │ ⌘ Cypher query        │
│ 📋 Copy as YAML   │         │ 📋 Copy as YAML       │
│ 🔖 Bookmark       │         │                       │
│ ✗ Delete          │         │ ✗ Delete              │
└───────────────────┘         └───────────────────────┘
```

Arrow keys navigate, Enter selects, Esc closes. Actions are context-dependent —
different item types surface different menus. Destructive actions (`✗`) are
always last and require confirmation dialog.

##### Clipboard & Bookmarks

**Clipboard (`y` key)**: Yanks current selection to system clipboard.
- On Kind → copies YAML definition
- On Cypher preview → copies the full query
- On edge → copies edge pattern `(a:Kind)-[:EDGE]->(b:Kind)`
- Feedback: toast "Copied to clipboard" + brief flash on source element

**Bookmarks (`B` to set, `'` to jump)**:
- `B` on any Kind → toggles bookmark (max 10, shown in status bar)
- `'` → opens bookmark jump list (fuzzy searchable)
- Bookmarks persist in `~/.novanet/bookmarks.json`

Status bar with bookmarks:
```
│ 🔖 Page · Block · Concept · Locale │ 4 bookmarks │
```

##### Mouse Support (opt-in)

Disabled by default. Toggle with `Ctrl+M` or `:mouse toggle`.

| Mouse Action | Effect |
|-------------|--------|
| Left click | Focus panel, select item |
| Scroll | Scroll list in focused panel |
| Drag border | Resize panel split ratios |
| Right click | Open action menu (same as Enter) |
| Double click | Expand/collapse tree node |

##### Session Management

**Undo (`u` key)**: Undo last CRUD action (1 level deep). Stores previous state
of created/edited/deleted nodes. Shows confirmation toast.

**History**: Last 50 commands stored in `~/.novanet/history.json`. Accessible via
`:history` in command palette. Arrow-up in command palette cycles history.

**Export (`x` key)**: Snapshot current view to file.
- Default: `~/.novanet/exports/novanet-{timestamp}.txt`
- `:export yaml` → schema dump
- `:export cypher` → current Cypher query
- `:export text` → rendered terminal view (stripped of control codes)

---

#### TUI Wow Effects & Animations

Every interaction should feel **alive**. Inspired by 90s sci-fi terminals
(Hackers, WarGames, Ghost in the Shell), cyberpunk dashboards, and the Matrix
digital rain — but applied tastefully, never blocking the user.

##### Particle Burst on Actions

When a CRUD action completes, a brief particle burst radiates from the action
location. Particles are single characters with velocity and fade:

```rust
struct ParticleBurst {
    origin: (u16, u16),       // screen coordinates
    particles: Vec<Particle>,
    lifetime: u8,             // frames remaining (max 15)
}

struct Particle {
    x: f64, y: f64,
    vx: f64, vy: f64,        // velocity (cells/frame)
    char: char,               // ✦ · ★ ◉ ⊕
    color: Color,
    age: u8,                  // increments each frame, dims with age
}
```

| Action | Color | Chars | Pattern |
|--------|-------|-------|---------|
| Create | matrix_green | `✦ ◉ + ·` | Expanding ring outward |
| Delete | plasma_pink | `· ✗ ░ ▒` | Imploding scatter inward |
| Edit | solar_amber | `✎ ◈ ★ ·` | Gentle shimmer in place |
| Connect | cyber_cyan | `─ ═ ╳ ◉` | Line traces to target |

Lifetime: 15 frames (~0.5s at 30fps). Particles stay within panel boundaries.

##### CRT Scanline Mode

Toggle-able retro CRT effect that adds horizontal scanlines across the terminal:

```rust
fn apply_crt_scanlines(buf: &mut Buffer, tick: u64) {
    for y in 0..buf.area.height {
        let scanline_intensity = if y % 2 == 0 { 0.85 } else { 1.0 };
        // Phosphor flicker: subtle brightness variance per frame
        let flicker = 1.0 - (((tick + y as u64) % 7) as f32 * 0.01);
        let factor = scanline_intensity * flicker;

        for x in 0..buf.area.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                if let Color::Rgb(r, g, b) = cell.fg() {
                    cell.set_fg(Color::Rgb(
                        (r as f32 * factor) as u8,
                        (g as f32 * factor) as u8,
                        (b as f32 * factor) as u8,
                    ));
                }
            }
        }
    }
}
```

Toggle with `Ctrl+R` or `:toggle crt`. Adds subtle phosphor persistence where
bright text leaves a faint ghost on the next frame. Performance: O(width*height)
per frame, negligible on modern terminals.

##### Screen Shake on Delete

When a destructive action (delete node/relation) is confirmed, the entire frame
shifts by 1-2 cells for 3 frames, simulating physical impact:

```rust
struct ScreenShake {
    frames_remaining: u8,             // 3
    offsets: [(i16, i16); 3],         // [(-1, 0), (1, 1), (0, -1)]
}

fn apply_shake(area: &mut Rect, shake: &ScreenShake) {
    let idx = (3 - shake.frames_remaining) as usize;
    let (dx, dy) = shake.offsets[idx];
    area.x = (area.x as i16 + dx).max(0) as u16;
    area.y = (area.y as i16 + dy).max(0) as u16;
}
```

Duration: 3 frames (~100ms). Makes destructive actions feel weighty and
deliberate. Combines with particle burst for full delete feedback.

##### Glitch Transition on Mode Switch

When switching navigation modes (1/2/3/4), a brief digital glitch effect
distorts the outgoing view before the incoming view slides in:

```
Frame 0 (current):    Normal display
Frame 1 (glitch):     Random cells replaced with ░▒▓█ blocks
Frame 2 (peak):       30% of cells corrupted, colors shifted to purple
Frame 3 (resolving):  New view starts appearing through static
Frame 4 (clean):      New view fully rendered
```

```rust
use rand::Rng;

fn apply_glitch(buf: &mut Buffer, intensity: f32) {
    let mut rng = rand::thread_rng();
    let glitch_chars = ['░', '▒', '▓', '█', '╳', '┃', '━', '╋'];
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            if rng.gen::<f32>() < intensity {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(glitch_chars[rng.gen_range(0..glitch_chars.len())]);
                    cell.set_fg(Color::Rgb(
                        rng.gen_range(80..200),
                        rng.gen_range(0..60),
                        rng.gen_range(120..255),
                    ));
                }
            }
        }
    }
}
```

Intensity curve over 4 frames: `[0.05, 0.30, 0.15, 0.0]`.

##### Nebula Pulse on Panel Focus

When a panel receives focus, its border cycles through the accent palette
over 8 frames (~0.27s) before settling at the focused color:

```rust
fn border_pulse_color(frame: u8) -> Color {
    const PULSE: [Color; 6] = [
        Color::Rgb(100, 116, 139), // star_dim (unfocused)
        Color::Rgb(99, 102, 241),  // nebula_indigo
        Color::Rgb(124, 58, 237),  // nebula_violet
        Color::Rgb(139, 92, 246),  // nebula_purple
        Color::Rgb(59, 130, 246),  // nebula_blue
        Color::Rgb(34, 211, 238),  // cyber_cyan (focused steady)
    ];
    PULSE[frame.min(5) as usize]
}
```

The border also gains a BOLD modifier at the peak (frame 3) and keeps it
for the active state. Unfocused panels return to `star_dim` over 3 frames.

##### Aurora Borealis Idle

When the TUI is idle for 30+ seconds, slow aurora bands drift across the
background — bands of color moving top-to-bottom through `bg_void` cells only:

```rust
struct Aurora {
    bands: Vec<AuroraBand>,
    tick: u64,
}

struct AuroraBand {
    y_center: f64,              // vertical position
    width: f64,                 // band thickness
    speed: f64,                 // cells per frame (0.05-0.15)
    hue: AuroraHue,            // color family
}

enum AuroraHue {
    Purple,  // nebula_purple → nebula_violet gradient
    Blue,    // nebula_blue → nebula_indigo gradient
    Cyan,    // cyber_cyan → cyber_teal gradient
    Green,   // matrix_green at 20% opacity
}
```

Very subtle — only affects background cells (those showing `bg_void`), never
overwrites text or panel content. Disappears instantly on any keypress.
3-5 bands visible at a time, each moving at different speeds for parallax.

##### Heatmap Glow on Data

In the dashboard and tree, high-traffic/high-count data glows warmer:

```
Low activity:    cyber_cyan      Rgb(34, 211, 238)    (cool)
Medium activity: nebula_blue     Rgb(59, 130, 246)    (warm)
High activity:   nebula_purple   Rgb(139, 92, 246)    (hot)
Peak activity:   plasma_pink     Rgb(236, 72, 153)    (critical)
```

Applied to: sparkline peaks, bar chart bars, gauge fill color, tree node
labels for Kinds with high edge counts. Updates in real-time as Neo4j stats
refresh. Thresholds calculated from min/max of visible data range.

##### Typewriter Effect

All text appearing during onboarding, boot, and status messages uses a
typewriter effect — characters appear one by one:

```rust
struct Typewriter {
    full_text: String,
    chars_visible: usize,
    delay_ms: u16,           // base delay between chars (30-80ms)
    variance: f32,           // 0.0-1.0, randomizes timing
    cursor_char: char,       // '█' while typing, ' ' after done
    sound: bool,             // terminal bell on each char (off by default)
}

impl Typewriter {
    fn tick(&mut self) -> bool {
        if self.chars_visible < self.full_text.len() {
            self.chars_visible += 1;
            true  // still typing
        } else {
            false // done
        }
    }

    fn visible(&self) -> String {
        format!(
            "{}{}",
            &self.full_text[..self.chars_visible],
            if self.chars_visible < self.full_text.len() {
                self.cursor_char.to_string()
            } else {
                String::new()
            }
        )
    }
}
```

Used for: boot branding text, welcome screen messages, toast notifications,
status bar hints, mode switch labels.

##### Data Stream Effect

In the Cypher preview panel, incoming query results animate as a data stream —
characters cascade from noise into the actual result text:

```
Frame 0:  ▓▒░░ ▓▒░ ▒░░▓ ▓░░▒ ░▒▓▒ ░░▓▒  (all noise)
Frame 1:  ▓▒░░ MA░ ▒░░▓ (k:░ ░▒nd▒ ░░▓▒  (25% resolved)
Frame 2:  MATC░ (k:░ ind ░{ke░ ░'Pa▒ e'}░  (60% resolved)
Frame 3:  MATCH (k:Kind {key: 'Page'})       (100% clean)
```

Each character independently transitions from a random block character
(`░▒▓█`) to its final value. Resolution progresses left-to-right with some
randomness. Color: noise chars are `star_dim`, resolved chars are `matrix_green`
then settle to `nova_white`.

##### Warp Speed on Refresh

When `F5` (refresh all) is pressed, all sparklines show a "warp speed" effect —
values stretch into horizontal trails before snapping to new data:

```
Before refresh:  ▁▂▃▅▇█▇▅▃▂▁            (normal data)
During warp:     ▁▂▃▅▇████████████▇▅▃▂▁  (trails extend right)
After refresh:   ▂▃▅▇█▇▅▃▂▁▂            (new data)
```

Duration: 10 frames. Color shifts from `cyber_cyan` to `nova_white` during
warp, then back to normal. Gauge widgets fill rapidly to 100% then snap to
new value.

##### Constellation Lines on Tree Hover

When focusing a Kind in the taxonomy tree, dotted lines connect it to
related Kinds that are also visible in the tree — showing edges inline:

```
  📦 project
  ├── 📐 structure
  │   ├── Page ··············◌  (HAS_BLOCK →)
  │   └── Block ←···········╯
  ├── 💡 semantic
  │   ├── Concept ···········◌  (USES_CONCEPT →)
  │   └── ConceptL10n
  └── 📝 output
      └── PageL10n ←·········╯  (HAS_OUTPUT →)
```

Dotted lines (`·`) connect related Kinds within the visible tree.
Color: `nebula_indigo` with DIM modifier. Lines appear after 300ms focus
delay and disappear when focus moves. Direction arrows (`→` `←`) show edge
direction. Max 5 constellation lines visible at once to avoid clutter.

##### Effect Configuration

All effects are configurable via `~/.novanet/config.toml`:

```toml
[effects]
crt_scanlines = false        # CRT mode off by default
particle_bursts = true       # CRUD particle effects
screen_shake = true          # delete shake
glitch_transitions = true    # mode switch glitch
nebula_pulse = true          # focus border animation
aurora_idle = true           # idle background aurora
aurora_idle_delay_s = 30     # seconds before aurora starts
heatmap_glow = true          # data temperature coloring
typewriter = true            # text typing effect
data_stream = true           # Cypher result cascade
warp_speed = true            # refresh sparkline warp
constellation_lines = true   # tree edge preview

[effects.animation]
fps = 30                     # target frame rate
boot_animation = true        # show boot sequence
boot_skip_key = "Escape"     # key to skip boot
```

Users who prefer a clean, no-animation experience can disable everything:
```toml
[effects]
crt_scanlines = false
particle_bursts = false
screen_shake = false
glitch_transitions = false
nebula_pulse = false
aurora_idle = false
heatmap_glow = false
typewriter = false
data_stream = false
warp_speed = false
constellation_lines = false

[effects.animation]
boot_animation = false
```

---

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

**Ralph Wiggum #8**: All tests pass, all audits clean, PR approved

> **Note**: There is no Phase 9 — v9.0 completes at Phase 8. Phase numbers
> align with milestone versions: Phases 0-8 = v9.0, Phases 10-12 = v10.0, etc.

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

