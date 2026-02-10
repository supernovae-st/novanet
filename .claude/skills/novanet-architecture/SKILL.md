---
name: novanet-architecture
description: Display the complete NovaNet architecture diagram in ASCII. Use when user asks about architecture, system overview, how components connect, meta-graph structure, or wants to understand the codebase structure.
disable-model-invocation: false
user-invocable: true
---

# NovaNet Architecture Overview

Display the complete NovaNet architecture diagram showing:
- Source of truth (YAML models)
- v11.3 Meta-Graph (faceted classification)
- Generators (Mermaid, Layer, Kind, ArcSchema)
- Neo4j infrastructure
- Rust binary (`tools/novanet/`)
- Studio visualization
- Source of Truth Pipeline

## Instructions

Based on the `$ARGUMENTS` provided, display the appropriate section:

- **"source"** or **"yaml"** - Show Source de Verite section only
- **"meta"** or **"facets"** - Show Meta-Graph (v11.3 faceted classification)
- **"infra"** or **"neo4j"** - Show Infrastructure section only
- **"studio"** - Show Studio section only
- **"packages"** or **"deps"** - Show Packages Dependency Graph
- **"flow"** or **"generation"** - Show Data Flow (Generation Pipeline)
- **"pipeline"** or **"sync"** - Show Source of Truth Pipeline
- **"locale"** or **"knowledge"** - Show Locale Knowledge Structure
- **"rust"** or **"cli"** - Show Rust Binary Architecture
- **"all"** or empty - Show the complete architecture (default)

---

## Section: SOURCE DE VERITE

```
+===================================================================================================+
|                              NOVANET - SOURCE DE VERITE (v11.3)                                   |
+===================================================================================================+

+---------------------------------------------------------------------------------------------------+
|  SOURCE DE VERITE                                                                                 |
+---------------------------------------------------------------------------------------------------+
|                                                                                                   |
|   packages/core/models/                                                                           |
|   +-- _index.yaml                          <- Index du graphe (structure, changelog)              |
|   +-- taxonomy.yaml                        <- v11.3: 2 Realms/11 Layers/5 Traits/5 ArcFamilies    |
|   +-- node-kinds/                          <- 61 fichiers YAML (1 par Kind)                       |
|   |   +-- shared/                          <- Realm: shared (32 nodes)                            |
|   |   |   +-- locale/                      <-   Layer: locale (7 nodes)                           |
|   |   |   +-- geography/                   <-   Layer: geography (6 nodes)                        |
|   |   |   +-- knowledge/                   <-   Layer: knowledge (19 nodes)                       |
|   |   |       +-- term-set.yaml, term.yaml <- Knowledge Containers + Atoms                        |
|   |   |       +-- expression-set.yaml, ... <- ExpressionSet, PatternSet, CultureSet, etc.         |
|   |   |       +-- taboo-set.yaml, etc.     <- TabooSet, AudienceSet + their atoms                 |
|   |   |                                                                                           |
|   |   +-- org/                             <- Realm: org (29 nodes)                               |
|   |       +-- config/                      <-   Layer: config (OrgConfig)                         |
|   |       +-- foundation/                  <-   Layer: foundation (Project, Brand, ProjectContent)|
|   |       +-- structure/                   <-   Layer: structure (Page, Block, Types)             |
|   |       +-- semantic/                    <-   Layer: semantic (Entity, EntityContent, Persona)  |
|   |       +-- instruction/                 <-   Layer: instruction (Prompts, Rules)               |
|   |       +-- seo/                         <-   Layer: seo (SEOKeyword, Metrics)                  |
|   |       +-- geo/                         <-   Layer: geo (GEOQuery, GEOAnswer, GEOMetrics)      |
|   |       +-- output/                      <-   Layer: output (PageGenerated, BlockGenerated)     |
|   |                                                                                               |
|   +-- arc-kinds/                           <- 125 fichiers YAML (1 par ArcKind)                   |
|   +-- relations.yaml                       <- Legacy format (kept for parser compatibility)       |
|   +-- views/                               <- Definitions de vues YAML                            |
|                                                                                                   |
+---------------------------------------------------------------------------------------------------+
```

---

## Section: META-GRAPH (v11.3 Faceted Classification)

```
+===================================================================================================+
|                    META-GRAPH (v11.3) - Self-Describing Context Graph                             |
+===================================================================================================+

  Each Kind sits at the intersection of 4 classification axes:

  +------------------------------------------------------------------------------------------------+
  |                                                                                                |
  |   Axis 1 - WHERE?   :Realm       (2)  shared / org                                            |
  |   Axis 2 - WHAT?    :Layer      (11)  SHARED: locale, geography, knowledge                    |
  |                                        ORG: config, foundation, structure, semantic,           |
  |                                             instruction, seo, geo, output                      |
  |   Axis 3 - HOW?     :Trait       (5)  invariant / localized / knowledge / generated / aggregated |
  |   Axis 4 - LINKS?   :ArcKind   (125)  grouped into 5 ArcFamilies                              |
  |                                                                                                |
  +------------------------------------------------------------------------------------------------+

  6 Meta-Node Types (all carry :Meta double-label):

  +-------------+    +--------------+    +-------------+
  |  Realm (2)  |--->|  Layer (11)  |--->|  Kind (61)  |
  |  WHERE?     |    |  WHAT?       |    |  1:1 label  |
  |  HAS_LAYER  |    |  HAS_KIND    |    |             |
  +-------------+    +--------------+    +------+------+
                                               |
                     +-------------------------+-------------------------+
                     |                         |                         |
                     v                         v                         v
              +-------------+           +-------------+          +--------------+
              |  IN_REALM   |           |  IN_LAYER   |          |  HAS_TRAIT   |
              |  (facet)    |           |  (facet)    |          |  (facet)     |
              +-------------+           +-------------+          +------+-------+
                                                                       |
                                                                       v
                                                                +-------------+
                                                                |  Trait (5)  |
                                                                |  HOW?       |
                                                                +-------------+

  Arc Schema (OWL-inspired):

  +----------------+    FROM_KIND    +-------------+    TO_KIND     +----------------+
  | ArcKind (125)  |---------------->|  Kind (61)  |<---------------| ArcKind (125)  |
  |  1:1 rel type  |                 +-------------+                |                |
  +-------+--------+                                                +----------------+
          |
          | IN_FAMILY
          v
  +----------------+
  | ArcFamily (5)  |
  |  ownership     |
  |  localization  |
  |  semantic      |
  |  generation    |
  |  mining        |
  +----------------+

  Instance Bridge (every data node links to its Kind):

  +----------------+    OF_KIND     +-------------+
  |  DataNode      |--------------->|  Kind :Meta  |
  |  (e.g. Block)  |                |  label:Block |
  +----------------+                +-------------+

  TUI Modes (v11.3 - 3 modes):

  +---------------------------------------------------------------------------------+
  |  Graph  |  Taxonomy (t) or Instances (t) view - unified graph exploration      |
  |  Audit  |  Schema validation and consistency checks                            |
  |  Nexus  |  Gamified learning hub (traits, layers, arcs, pipeline)              |
  +---------------------------------------------------------------------------------+
```

---

## Section: PIPELINE (Source of Truth Sync)

```
+===================================================================================================+
|                    SOURCE OF TRUTH PIPELINE - Schema Propagation (v11.3)                          |
+===================================================================================================+

     +---------------------------------------------------------------------------------------------+
     |                        YAML (Single Source of Truth)                                        |
     |                        packages/core/models/                                                |
     |                        +-- node-kinds/               <- 61 NodeKind files                   |
     |                        +-- arc-kinds/                <- 125 ArcKind files                   |
     |                        +-- taxonomy.yaml             <- 2 Realms, 11 Layers, 5 Traits       |
     +---------------------------------------------+-----------------------------------------------+
                                                   |
         +-----------------------------------------+-----------------------------------------+
         |                    |                    |                    |                    |
         v                    v                    v                    v                    v
  +--------------+   +--------------+   +--------------+   +--------------+   +------------------+
  |  Mermaid     |   |  Layer       |   |  Kind        |   |  Arc         |   |  Manual          |
  |  Generator   |   |  Generator   |   |  Generator   |   |  Schema Gen  |   |  Cypher Seeds    |
  |  tools/novanet|  | tools/novanet|   | tools/novanet|   | tools/novanet|   |  packages/db/    |
  +------+-------+   +------+-------+   +------+-------+   +------+-------+   +--------+---------+
         |                  |                  |                  |                    |
         v                  v                  v                  v                    v
  +--------------+   +--------------+   +--------------+   +--------------+   +------------------+
  | VIEW-COMPLETE|   |  layers.ts   |   | Kind :Meta   |   | ArcKind      |   | 00-constraints   |
  | -GRAPH.md    |   |  src/graph/  |   | nodes w/     |   | :Meta nodes  |   | 00.5-taxonomy    |
  |              |   |              |   | schema_hint  |   | w/ cypher_   |   | 01-kinds         |
  |              |   |              |   |              |   | pattern      |   | 02-arc-kinds     |
  +--------------+   +--------------+   +--------------+   +--------------+   | 99-autowire-kinds|
                                                                              | (all generated)  |
                                                                              +--------+---------+
                                                                                       |
     +----------------------------------------------------------------------------------+
     |
     v
  +------------------------------------------------------------------------------------------------+
  |   Neo4j (Runtime Database)                                                                     |
  |   +-- bolt://localhost:7687                                                                    |
  |   +-- Migrations: 001-inverse-relationships -> 006-formatting-property-alignment               |
  +------------------------------------------------------------------------------------------------+

  ==============================================================================================
   RUST VALIDATE:   novanet schema validate            (YAML <-> Neo4j consistency check)
   RUST GENERATE:   novanet schema generate            (Rebuilds all artifacts from YAML)
   RUST SEED:       novanet db seed                    (Execute seed Cypher files)
  ==============================================================================================
```

---

## Section: RUST (Binary Architecture)

```
+===================================================================================================+
|                    RUST BINARY - tools/novanet/ (v11.3)                                           |
+===================================================================================================+

  +-----------------------------------------------------------------------------------------------+
  |  Single crate: tools/novanet/                                                                 |
  |                                                                                               |
  |  Dependencies:                                                                                |
  |  +-- clap (derive)      <- CLI argument parsing                                               |
  |  +-- neo4rs             <- Neo4j Bolt protocol driver                                         |
  |  +-- ratatui            <- Terminal UI framework                                              |
  |  +-- crossterm          <- Terminal backend                                                   |
  |  +-- tokio              <- Async runtime                                                      |
  |  +-- serde + serde_yaml <- YAML deserialization                                               |
  |  +-- thiserror          <- Library error types                                                |
  |  +-- color-eyre         <- Application error reporting                                        |
  |                                                                                               |
  |  Commands (all implemented, 950 tests):                                                       |
  |  +-- novanet data/meta/overlay/query       <- 4 navigation modes (faceted Cypher)             |
  |  +-- novanet node create/edit/delete       <- Node CRUD (label validation)                    |
  |  +-- novanet arc create/delete             <- Arc CRUD (type validation)                      |
  |  +-- novanet search --query=...            <- Fulltext + property search                      |
  |  +-- novanet locale list/import            <- Locale operations                               |
  |  +-- novanet db seed/migrate/reset         <- Database lifecycle                              |
  |  +-- novanet schema generate/validate      <- YAML -> artifacts (12 generators)               |
  |  +-- novanet doc generate                  <- View-specific Mermaid (11 views)                |
  |  +-- novanet filter build                  <- JSON stdin -> Cypher (Studio subprocess)        |
  |  +-- novanet blueprint                     <- ASCII visualization (10 views)                  |
  |  +-- novanet tui                           <- Galaxy-themed mission control TUI               |
  |                                                                                               |
  |  Architecture:                                                                                |
  |  +-- Rust generates artifacts AND executes at runtime                                         |
  |  +-- TypeScript @novanet/schema-tools eliminated                                              |
  |                                                                                               |
  +-----------------------------------------------------------------------------------------------+
```

---

## Section: LOCALE KNOWLEDGE

```
+===================================================================================================+
|                              LOCALE KNOWLEDGE STRUCTURE (v11.3)                                   |
+===================================================================================================+

     +---------------------------------------------------------------------------------------------+
     |                                    Locale                                                   |
     |                       (en-US, fr-FR, ja-JP, es-ES, de-DE, fr-CA)                            |
     +-------+------------+------------+------------+------------+------------+--------------------+
             |            |            |            |            |            |
     +-------+--+  +------+--+  +------+--+  +------+--+  +------+--+  +------+--------+
     |HAS_IDENTITY| |HAS_VOICE| |HAS_CULTURE| |HAS_MARKET| |HAS_LEXICON| |HAS_RULES_*    |
     +-----+------+ +----+----+ +-----+-----+ +----+-----+ +-----+-----+ +-------+-------+
           |             |            |           |             |               |
           v             v            v           v             v               v
   +---------------+ +-----------+ +-----------+ +------------+ +-----------+ +---------------------+
   |LocaleIdentity | |LocaleVoice| |LocaleCulture| |LocaleMarket| |LocaleLexicon| |LocaleRules*        |
   |               | |           | |             | |            | |             | |                    |
   | - endonym     | |- formality| | - taboos    | |- currency  | |- domain     | | Adaptation:        |
   | - script      | |- directness| |- sensitivities| |- payments | |  terms      | |  - units, dates    |
   | - direction   | |- emotion  | | - Hofstede  | |- trust     | |             | | Formatting:        |
   +---------------+ +-----------+ |             | +------------+ +------+------+ |  - patterns, decimals|
                                   |             |                      |        | Slug:              |
                                   +------+------+                      |        |  - transliteration |
                                          |                             v        +---------------------+
                        +-----------------+---------------------+   +-----------+
                        |      HAS_CULTURE_REFERENCES           |   | Expression  |
                        |              v                        |   | (urgency,   |
                        |   +-----------------------+           |   |  value,     |
                        |   |LocaleCultureReferences|           |   |  action)    |
                        |   +----------+------------+           |   +-----------+
                        |              |                        |
                        |    +---------+--------+               |
                        |    v         v        v               |
                        | Reference Metaphor Constraint         |
                        +---------------------------------------+
```

---

## Section: INFRASTRUCTURE

```
+===================================================================================================+
|                              INFRASTRUCTURE (packages/db)                                         |
+===================================================================================================+

+---------------------------------------------------------------------------------------------------+
|  DOCKER                                                                                           |
+---------------------------------------------------------------------------------------------------+
|                                                                                                   |
|   docker-compose.yml                                                                              |
|   +-------------------------------------------------------------------------------------------+   |
|   |  Neo4j 5.26.0 Community                                                                   |   |
|   |  +-- Browser: http://localhost:7474                                                       |   |
|   |  +-- Bolt: bolt://localhost:7687                                                          |   |
|   |  +-- Auth: neo4j / novanetpassword                                                        |   |
|   +-------------------------------------------------------------------------------------------+   |
|                                                                                                   |
|   seed/ (generated + manual, executes en ordre)                                                   |
|   +-- 00-constraints.cypher        <- Contraintes d'unicite (UNIQUE, EXISTS)                      |
|   +-- 00.5-taxonomy.cypher         <- Taxonomy: Realms, Layers, Traits (GENERATED)                |
|   +-- 01-kinds.cypher              <- NodeKinds meta-nodes (GENERATED)                            |
|   +-- 01-vector-indexes.cypher     <- Index vectoriels pour recherche semantique                  |
|   +-- 02-arc-kinds.cypher          <- ArcKinds meta-nodes (GENERATED)                             |
|   +-- 20-locales.cypher            <- Locales from CSV + MD (GENERATED)                           |
|   +-- 21-locale-knowledge.cypher   <- LocaleIdentity, LocaleVoice, LocaleCulture, etc.            |
|   +-- 31-project-qrcode-ai.cypher  <- Projet QR Code AI avec Pages, Blocks                        |
|   +-- 34-prompts.cypher            <- BlockPrompt, PagePrompt                                     |
|   +-- 99-autowire-kinds.cypher     <- Links data nodes to Kinds (GENERATED)                       |
|                                                                                                   |
|   migrations/ (6 fichiers, idempotents)                                                           |
|   +-- 001-inverse-relationships.cypher                                                            |
|   +-- 002-semantic-link-inverses.cypher                                                           |
|   +-- 003-vector-indexes.cypher                                                                   |
|   +-- 004-remove-deprecated-properties.cypher                                                     |
|   +-- 005-yaml-synchronization.cypher      <- Alignement YAML v7.11.0                             |
|   +-- 006-formatting-property-alignment.cypher <- LocaleRulesFormatting props                     |
|                                                                                                   |
+---------------------------------------------------------------------------------------------------+
```

---

## Section: STUDIO

```
+===================================================================================================+
|                              STUDIO (apps/studio) - Next.js 16 + React 19                         |
+===================================================================================================+

+---------------------------------------------------------------------------------------------------+
|   +-------------------------------------------------------------------------------------------+   |
|   |  API ROUTES (10)                                  |  ZUSTAND STORES (8)                  |   |
|   +---------------------------------------------------+--------------------------------------+   |
|   |                                                   |                                      |   |
|   |  /api/chat          -> Claude AI                  |  graphStore      -> nodes, edges     |   |
|   |  /api/graph         -> main data                  |  filterStore     -> types, presets   |   |
|   |  /api/graph/expand  -> neighbors                  |  uiStore         -> panels, selection|   |
|   |  /api/graph/ontology-> metadata                   |  chatStore       -> AI messages      |   |
|   |  /api/graph/organizing-principles                 |  queryStore      -> Cypher state     |   |
|   |  /api/graph/query   -> Cypher exec                |  viewStore       -> saved views      |   |
|   |  /api/graph/schema  -> schema info                |  aiQueryStore    -> AI query state   |   |
|   |  /api/graph/stats   -> statistics                 |  animationStore  -> animations       |   |
|   |  /api/views         -> CRUD views                 |                                      |   |
|   |  /api/views/[id]    -> single view                |                                      |   |
|   |                                                   |                                      |   |
|   +---------------------------------------------------+--------------------------------------+   |
|                                                                                                   |
|   +-------------------------------------------------------------------------------------------+   |
|   |  VISUALIZATION - TUI Modes (v11.3)                                                        |   |
|   +-------------------------------------------------------------------------------------------+   |
|   |                                                                                           |   |
|   |  +---------------------+  +---------------------+  +---------------------+                |   |
|   |  |  GRAPH MODE        |  |  AUDIT MODE         |  |  NEXUS MODE        |                |   |
|   |  |  (Taxonomy/Instances)|  |  (validation)      |  |  (learning hub)    |                |   |
|   |  |                    |  |                     |  |                    |                |   |
|   |  |  Real Neo4j data   |  |  Schema checks      |  |  Gamified learning |                |   |
|   |  |  Force-directed    |  |  Consistency        |  |  Traits, Layers    |                |   |
|   |  |  Grouped by Realm  |  |  validation         |  |  Arcs, Pipeline    |                |   |
|   |  +---------------------+  +---------------------+  +---------------------+                |   |
|   |                                                                                           |   |
|   |  Visual Encoding (v11.3):                                                                 |   |
|   |  +-- Fill color   -> Layer (11 colors)                                                    |   |
|   |  +-- Border style  -> Trait (5 styles: solid/dashed/dotted/double/thin-dotted)            |   |
|   |  +-- Spatial group -> Realm (2 zones: shared, org)                                        |   |
|   |  +-- Arc stroke    -> ArcFamily (5 colors)                                                |   |
|   |                                                                                           |   |
|   +-------------------------------------------------------------------------------------------+   |
|                                                                                                   |
+---------------------------------------------------------------------------------------------------+
```

---

## Section: PACKAGES

```
+===================================================================================================+
|  PACKAGES DEPENDENCY GRAPH                                                                        |
+===================================================================================================+
|                                                                                                   |
|                    +------------------+                                                           |
|                    |  @novanet/core   |  <- Types, schemas, generators, filters                   |
|                    |  (source truth)  |     node-kinds/ + arc-kinds/ + taxonomy.yaml              |
|                    +--------+---------+     v11.3: 2 Realms, 11 Layers                            |
|                             |                                                                     |
|              +--------------+--------------+                                                      |
|              |              |              |                                                      |
|              v              v              v                                                      |
|              +------------------+  +------------------+                                           |
|              | @novanet/studio  |  |  @novanet/db     |                                           |
|              | (visualization)  |  |  (infrastructure)|  <- Standalone (Cypher files only)        |
|              +------------------+  +------------------+                                           |
|                                                                                                   |
|   +--------------------------------------------------------------------------------------------+  |
|   |  tools/novanet/  <- Rust binary (CLI + TUI + generators)                                   |  |
|   |  +-- generators/  MermaidGen, LayerGen, KindGen, ArcSchemaGen, AutowireGen, ...            |  |
|   |  +-- parsers/     YAML nodes, relations, locale markdown                                   |  |
|   |  +-- commands/    schema, db, locale, search, filter, data/meta/overlay/query              |  |
|   |  +-- search/      Hybrid vector + graph search                                             |  |
|   |  +-- filter/      Cypher filter builder (Studio subprocess)                                |  |
|   |  +-- tui/         Galaxy-themed mission control (ratatui)                                  |  |
|   +--------------------------------------------------------------------------------------------+  |
|                                                                                                   |
|   !! ELIMINATED: @novanet/schema-tools, @novanet/cli (absorbed into Rust binary)                  |
|                                                                                                   |
+===================================================================================================+
```

---

## Section: FLOW (Generation Pipeline)

```
+===================================================================================================+
|  DATA FLOW: Generation Pipeline (v11.3)                                                           |
+===================================================================================================+
|                                                                                                   |
|   1. INVARIANT NODES (no locale)                                                                  |
|   +---------+     +---------+     +---------+                                                     |
|   | Project |---->|  Page   |---->|  Block  |                                                     |
|   +----+----+     +----+----+     +----+----+                                                     |
|        |              |              |                                                            |
|        |              |              |                                                            |
|   2. SEMANTIC LAYER   |              |                                                            |
|        |         +----+----+     +---+----+                                                       |
|        |         | Entity  |<----|USES_ENTITY                                                     |
|        |         +----+----+     +--------+                                                       |
|        |              |                                                                           |
|   3. LOCALIZATION     |                                                                           |
|        |         +----+-----+                                                                     |
|        |         |HAS_CONTENT|                                                                    |
|        |         v          |                                                                     |
|   +----+------+  +----------+----+                                                                |
|   |ProjectContent| |EntityContent |------+                                                        |
|   +-----------+  +---------------+      |                                                         |
|                                          |                                                        |
|   4. GENERATION (LLM)                    |                                                        |
|        +----------------------------------+                                                        |
|        |     +-------------+                                                                      |
|        +---->| BlockPrompt |                                                                      |
|              +------+------+                                                                      |
|                     | GENERATED                                                                   |
|                     v                                                                             |
|              +---------------+                                                                    |
|              | BlockGenerated|  <- Native content (NOT translation)                               |
|              +---------------+                                                                    |
|                                                                                                   |
+===================================================================================================+
```

---

## Key Numbers (v11.3.0)

| Metric | Value |
|--------|-------|
| Kind (node types) | 61 |
| ArcKind (arcs) | 125 |
| Realms | 2 (shared, org) |
| Layers | 11 (3 shared + 8 org) |
| Traits | 5 (invariant, localized, knowledge, generated, aggregated) |
| ArcFamilies | 5 |
| Shared nodes | 32 (locale: 7, geography: 6, knowledge: 19) |
| Org nodes | 29 (config: 1, foundation: 3, structure: 3, semantic: 4, instruction: 7, seo: 5, geo: 3, output: 3) |
| Seed files | 11 |
| Migrations | 6 |
| Locales supported | 200 |
| API routes (Studio) | 10 |
| Zustand stores | 8 |
| Rust tests | 950 |

---

## Commands

```bash
# Schema & docs (YAML, no Neo4j)
novanet schema generate            # Regenerate all artifacts from YAML
novanet schema validate            # Validate YAML coherence
novanet doc generate               # Generate 11 view Mermaid diagrams
novanet blueprint                  # ASCII visualization (10 views)

# Read modes (Neo4j)
novanet data                       # Mode 1: Data nodes
novanet meta                       # Mode 2: Meta-graph
novanet overlay                    # Mode 3: Data + Meta
novanet query --realm=org          # Mode 4: Faceted query

# Write (Neo4j)
novanet node create --kind=Page --key=my-page
novanet arc create --from=a --to=b --kind=USES_ENTITY

# Database lifecycle
novanet db seed                    # Execute seed files
novanet db migrate                 # Run migrations
novanet db reset                   # Drop + seed

# Search & locale
novanet search --query="page"      # Fulltext search
novanet locale list                # Locale operations

# Interactive
novanet tui                        # Galaxy-themed mission control TUI
```

---

## Usage

User can invoke with:
- `/novanet-arch` or `/novanet-architecture`
- `/novanet-arch source` - YAML source only
- `/novanet-arch meta` - Meta-Graph (v11.3 faceted classification)
- `/novanet-arch pipeline` - Source of Truth Pipeline
- `/novanet-arch locale` - Locale Knowledge Structure
- `/novanet-arch infra` - Infrastructure only
- `/novanet-arch studio` - Studio only
- `/novanet-arch packages` - Package dependencies
- `/novanet-arch flow` - Generation pipeline
- `/novanet-arch rust` - Rust binary architecture
