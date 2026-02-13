---
name: novanet-architecture
description: Display the complete NovaNet architecture diagram in ASCII. Use when user asks about architecture, system overview, how components connect, meta-graph structure, or wants to understand the codebase structure.
disable-model-invocation: false
user-invocable: true
---

# NovaNet Architecture Overview

Display the complete NovaNet architecture diagram showing:
- Source of truth (YAML models)
- v0.12.0 Schema Graph (faceted classification)
- Generators (Mermaid, Layer, Kind, ArcSchema)
- Neo4j infrastructure
- Rust binary (`tools/novanet/`)
- Studio visualization
- Source of Truth Pipeline

## Instructions

Based on the `$ARGUMENTS` provided, display the appropriate section:

- **"source"** or **"yaml"** - Show Source de Verite section only
- **"meta"** or **"facets"** - Show Schema Graph (v0.12.0 faceted classification)
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
|                              NOVANET - SOURCE DE VERITE (v0.12.0)                                 |
+===================================================================================================+

+---------------------------------------------------------------------------------------------------+
|  SOURCE DE VERITE                                                                                 |
+---------------------------------------------------------------------------------------------------+
|                                                                                                   |
|   packages/core/models/                                                                           |
|   +-- _index.yaml                          <- Index du graphe (structure, changelog)              |
|   +-- taxonomy.yaml                        <- v0.12.0: 2 Realms/10 Layers/5 Traits/5 ArcFamilies  |
|   +-- node-kinds/                          <- 59 fichiers YAML (1 par Class)                      |
|   |   +-- shared/                          <- Realm: shared (39 nodes)                            |
|   |   |   +-- config/                      <-   Layer: config (Locale, EntityCategory, etc.)      |
|   |   |   +-- locale/                      <-   Layer: locale (6 nodes)                           |
|   |   |   +-- geography/                   <-   Layer: geography (6 nodes)                        |
|   |   |   +-- knowledge/                   <-   Layer: knowledge (incl. SEO/GEO nodes)            |
|   |   |       +-- term-set.yaml, term.yaml <- Knowledge Containers + Atoms                        |
|   |   |       +-- expression-set.yaml, ... <- ExpressionSet, PatternSet, CultureSet, etc.         |
|   |   |       +-- seo-*.yaml, geo-*.yaml   <- SEO/GEO nodes (moved from org)                      |
|   |   |                                                                                           |
|   |   +-- org/                             <- Realm: org (20 nodes)                               |
|   |       +-- config/                      <-   Layer: config (OrgConfig)                         |
|   |       +-- foundation/                  <-   Layer: foundation (Project, Brand, ProjectContent)|
|   |       +-- structure/                   <-   Layer: structure (Page, Block, Types)             |
|   |       +-- semantic/                    <-   Layer: semantic (Entity, EntityContent, Persona)  |
|   |       +-- instruction/                 <-   Layer: instruction (Instructions, Rules)          |
|   |       +-- output/                      <-   Layer: output (PageGenerated, BlockGenerated)     |
|   |                                                                                               |
|   +-- arc-kinds/                           <- 114 fichiers YAML (1 par ArcClass)                  |
|   +-- relations.yaml                       <- Legacy format (kept for parser compatibility)       |
|   +-- views/                               <- Definitions de vues YAML                            |
|                                                                                                   |
+---------------------------------------------------------------------------------------------------+
```

---

## Section: SCHEMA-GRAPH (v0.12.0 Faceted Classification)

```
+===================================================================================================+
|                    SCHEMA-GRAPH (v0.12.0) - Self-Describing Context Graph                         |
+===================================================================================================+

  Each Class sits at the intersection of 4 classification axes:

  +------------------------------------------------------------------------------------------------+
  |                                                                                                |
  |   Axis 1 - WHERE?   :Realm       (2)  shared / org                                            |
  |   Axis 2 - WHAT?    :Layer      (10)  SHARED: config, locale, geography, knowledge            |
  |                                        ORG: config, foundation, structure, semantic,           |
  |                                             instruction, output                                |
  |   Axis 3 - HOW?     :Trait       (5)  defined / authored / imported / generated / retrieved   |
  |   Axis 4 - LINKS?   :ArcClass  (114)  grouped into 5 ArcFamilies                              |
  |                                                                                                |
  +------------------------------------------------------------------------------------------------+

  6 Schema-Node Types (all carry :Schema double-label):

  +-------------+    +--------------+    +--------------+
  |  Realm (2)  |--->|  Layer (10)  |--->|  Class (59)  |
  |  WHERE?     |    |  WHAT?       |    |  1:1 label   |
  |  HAS_LAYER  |    |  HAS_CLASS   |    |              |
  +-------------+    +--------------+    +------+-------+
                                                |
                     +--------------------------+-------------------------+
                     |                          |                         |
                     v                          v                         v
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

  +----------------+    FROM_CLASS   +--------------+    TO_CLASS    +----------------+
  | ArcClass (114) |---------------->|  Class (59)  |<---------------| ArcClass (114) |
  |  1:1 rel type  |                 +--------------+                |                |
  +-------+--------+                                                 +----------------+
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

  Instance Bridge (every data node links to its Class):

  +----------------+    OF_CLASS    +----------------+
  |  DataNode      |--------------->|  Class :Schema |
  |  (e.g. Block)  |                |  label:Block   |
  +----------------+                +----------------+

  TUI Modes (v0.12.0 - 2 modes):

  +---------------------------------------------------------------------------------+
  |  Graph  |  Unified tree: Realm > Layer > Class > Instance                      |
  |  Nexus  |  Hub: Quiz, Audit, Stats, Help                                       |
  +---------------------------------------------------------------------------------+
```

---

## Section: PIPELINE (Source of Truth Sync)

```
+===================================================================================================+
|                    SOURCE OF TRUTH PIPELINE - Schema Propagation (v0.12.0)                        |
+===================================================================================================+

     +---------------------------------------------------------------------------------------------+
     |                        YAML (Single Source of Truth)                                        |
     |                        packages/core/models/                                                |
     |                        +-- node-kinds/               <- 59 NodeClass files                  |
     |                        +-- arc-kinds/                <- 114 ArcClass files                  |
     |                        +-- taxonomy.yaml             <- 2 Realms, 10 Layers, 5 Traits       |
     +---------------------------------------------+-----------------------------------------------+
                                                   |
         +-----------------------------------------+-----------------------------------------+
         |                    |                    |                    |                    |
         v                    v                    v                    v                    v
  +--------------+   +--------------+   +--------------+   +--------------+   +------------------+
  |  Mermaid     |   |  Layer       |   |  Class       |   |  Arc         |   |  Manual          |
  |  Generator   |   |  Generator   |   |  Generator   |   |  Schema Gen  |   |  Cypher Seeds    |
  |  tools/novanet|  | tools/novanet|   | tools/novanet|   | tools/novanet|   |  packages/db/    |
  +------+-------+   +------+-------+   +------+-------+   +------+-------+   +--------+---------+
         |                  |                  |                  |                    |
         v                  v                  v                  v                    v
  +--------------+   +--------------+   +--------------+   +--------------+   +------------------+
  | VIEW-COMPLETE|   |  layers.ts   |   | Class :Schema|   | ArcClass     |   | 00-constraints   |
  | -GRAPH.md    |   |  src/graph/  |   | nodes w/     |   | :Schema nodes|   | 00.5-taxonomy    |
  |              |   |              |   | schema_hint  |   | w/ cypher_   |   | 01-classes       |
  |              |   |              |   |              |   | pattern      |   | 02-arc-classes   |
  +--------------+   +--------------+   +--------------+   +--------------+   | 99-autowire      |
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
|                    RUST BINARY - tools/novanet/ (v0.12.0)                                         |
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
|                              LOCALE KNOWLEDGE STRUCTURE (v0.12.0)                                 |
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
|   +-- 34-instructions.cypher       <- BlockInstruction, PageInstruction                           |
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
|   |  VISUALIZATION - TUI Modes (v0.12.0 - 2 modes)                                            |   |
|   +-------------------------------------------------------------------------------------------+   |
|   |                                                                                           |   |
|   |  +---------------------------------------+  +---------------------------------------+     |   |
|   |  |  GRAPH MODE                           |  |  NEXUS MODE                          |     |   |
|   |  |  (Unified Tree)                       |  |  (Hub)                               |     |   |
|   |  |                                       |  |                                      |     |   |
|   |  |  Realm > Layer > Class > Instance     |  |  Quiz, Audit, Stats, Help            |     |   |
|   |  |  Real Neo4j data                      |  |  Gamified learning                   |     |   |
|   |  |  Lazy instance loading                |  |  Schema validation                   |     |   |
|   |  +---------------------------------------+  +---------------------------------------+     |   |
|   |                                                                                           |   |
|   |  Visual Encoding (v0.12.0):                                                               |   |
|   |  +-- Fill color   -> Layer (10 colors)                                                    |   |
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
|                    +--------+---------+     v0.12.0: 2 Realms, 10 Layers                          |
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
|  DATA FLOW: Generation Pipeline (v0.12.0)                                                         |
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
|        |     +------------------+                                                                 |
|        +---->| BlockInstruction |                                                                 |
|              +--------+---------+                                                                 |
|                       | GENERATED                                                                 |
|                       v                                                                           |
|              +---------------+                                                                    |
|              | BlockGenerated|  <- Native content (NOT translation)                               |
|              +---------------+                                                                    |
|                                                                                                   |
+===================================================================================================+
```

---

## Key Numbers (v0.12.0)

| Metric | Value |
|--------|-------|
| Class (node types) | 59 |
| ArcClass (arcs) | 114 |
| Realms | 2 (shared, org) |
| Layers | 10 (4 shared + 6 org) |
| Traits | 5 (defined, authored, imported, generated, retrieved) |
| ArcFamilies | 5 |
| Shared nodes | 39 (config + locale + geography + knowledge) |
| Org nodes | 20 (config, foundation, structure, semantic, instruction, output) |
| Seed files | 11 |
| Migrations | 6 |
| Locales supported | 200 |
| API routes (Studio) | 10 |
| Zustand stores | 8 |
| Rust tests | 998 |

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
novanet node create --class=Page --key=my-page
novanet arc create --from=a --to=b --class=USES_ENTITY

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
- `/novanet-arch meta` - Schema Graph (v0.12.0 faceted classification)
- `/novanet-arch pipeline` - Source of Truth Pipeline
- `/novanet-arch locale` - Locale Knowledge Structure
- `/novanet-arch infra` - Infrastructure only
- `/novanet-arch studio` - Studio only
- `/novanet-arch packages` - Package dependencies
- `/novanet-arch flow` - Generation pipeline
- `/novanet-arch rust` - Rust binary architecture
