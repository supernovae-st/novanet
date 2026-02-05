---
name: novanet-architecture
description: Display the complete NovaNet architecture diagram in ASCII. Use when user asks about architecture, system overview, how components connect, meta-graph structure, or wants to understand the codebase structure.
disable-model-invocation: false
user-invocable: true
---

# NovaNet Architecture Overview

Display the complete NovaNet architecture diagram showing:
- Source of truth (YAML models)
- v9 Meta-Graph (faceted classification)
- Generators (Mermaid, Layer, Kind, ArcSchema)
- Neo4j infrastructure
- Rust binary (`tools/novanet/`)
- Studio visualization
- Source of Truth Pipeline

## Instructions

Based on the `$ARGUMENTS` provided, display the appropriate section:

- **"source"** or **"yaml"** - Show Source de Verite section only
- **"meta"** or **"facets"** - Show Meta-Graph (v9 faceted classification)
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
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              NOVANET - SOURCE DE VERITE                                           ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  📁 SOURCE DE VERITE                                                                                │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│   packages/core/models/                                                                             │
│   ├── _index.yaml                          ← Index du graphe (structure, changelog)                 │
│   ├── taxonomy.yaml                        ← v10.6: 2 Realms/9 Layers/5 Traits/5 ArcFamilies       │
│   ├── node-kinds/                          ← 46 fichiers YAML (1 par Kind)                          │
│   │   ├── global/                          ← Realm: global (23 nodes)                               │
│   │   │   ├── config/                      ←   Layer: config (Locale + utilities)                   │
│   │   │   ├── locale-knowledge/            ←   Layer: locale-knowledge (18 nodes)                   │
│   │   │   │   ├── term-set.yaml, term.yaml ← Knowledge Containers + Atoms                           │
│   │   │   │   ├── expression-set.yaml, ... ← ExpressionSet, PatternSet, CultureSet, etc.            │
│   │   │   │   └── taboo-set.yaml, etc.     ← TabooSet, AudienceSet + their atoms                    │
│   │   │   └── seo/                         ←   Layer: seo (SEOKeyword, Metrics, MiningRun)          │
│   │   │                                                                                             │
│   │   └── tenant/                          ← Realm: tenant (23 nodes, merged org+project)           │
│   │       ├── config/                      ←   Layer: config (Organization)                         │
│   │       ├── foundation/                  ←   Layer: foundation (Project, Brand, ProjectL10n)      │
│   │       ├── structure/                   ←   Layer: structure (Page, Block, Types)                │
│   │       ├── semantic/                    ←   Layer: semantic (Entity, EntityL10n, Persona)        │
│   │       ├── instruction/                 ←   Layer: instruction (Prompts, Rules)                  │
│   │       └── output/                      ←   Layer: output (PageL10n, BlockL10n)                  │
│   │                                                                                                 │
│   ├── arc-kinds/                           ← 51 fichiers YAML (1 par ArcKind)                       │
│   ├── relations.yaml                       ← Legacy format (kept for parser compatibility)          │
│   └── views/                               ← Definitions de vues YAML                               │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Section: META-GRAPH (v9 Faceted Classification)

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                    META-GRAPH (v9) — Self-Describing Context Graph                                ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

  Each Kind sits at the intersection of 4 classification axes:

  ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
  │                                                                                              │
  │   Axis 1 — WHERE?   :Realm       (2)  global / tenant                                       │
  │   Axis 2 — WHAT?    :Layer       (9)  config, locale-knowledge, seo (global)                │
  │                                        config, foundation, structure, semantic,              │
  │                                        instruction, output (tenant)                          │
  │   Axis 3 — HOW?     :Trait       (5)  invariant / localized / knowledge / derived / job      │
  │   Axis 4 — LINKS?   :ArcKind    (51)  grouped into 5 ArcFamilies                            │
  │                                                                                              │
  └──────────────────────────────────────────────────────────────────────────────────────────────┘

  6 Meta-Node Types (all carry :Meta double-label):

  ┌─────────────┐    ┌──────────────┐    ┌─────────────┐
  │  Realm (2)  │───▶│  Layer (9)   │───▶│  Kind (46)  │
  │  WHERE?     │    │  WHAT?       │    │  1:1 label  │
  │  HAS_LAYER  │    │  HAS_KIND    │    │             │
  └─────────────┘    └──────────────┘    └──────┬──────┘
                                               │
                     ┌─────────────────────────┼─────────────────────────┐
                     │                         │                         │
                     ▼                         ▼                         ▼
              ┌─────────────┐           ┌─────────────┐          ┌──────────────┐
              │  IN_REALM   │           │  IN_LAYER   │          │  HAS_TRAIT   │
              │  (facet)    │           │  (facet)    │          │  (facet)     │
              └─────────────┘           └─────────────┘          └──────┬───────┘
                                                                       │
                                                                       ▼
                                                                ┌─────────────┐
                                                                │  Trait (5)  │
                                                                │  HOW?       │
                                                                └─────────────┘

  Arc Schema (OWL-inspired):

  ┌────────────────┐    FROM_KIND    ┌─────────────┐    TO_KIND     ┌────────────────┐
  │  ArcKind (51) │───────────────▶│  Kind (46)  │◀──────────────│  ArcKind (51) │
  │  1:1 rel type  │                └─────────────┘               │                │
  └───────┬────────┘                                              └────────────────┘
          │
          │ IN_FAMILY
          ▼
  ┌────────────────┐
  │ArcFamily (5)  │
  │  ownership     │
  │  localization  │
  │  semantic      │
  │  generation    │
  │  mining        │
  └────────────────┘

  Instance Bridge (every data node links to its Kind):

  ┌────────────────┐    OF_KIND     ┌─────────────┐
  │  DataNode      │──────────────▶│  Kind :Meta  │
  │  (e.g. Block)  │               │  label:Block │
  └────────────────┘               └─────────────┘

  NavigationMode (4 modes):

  ┌─────────────────────────────────────────────────────────────────────────────┐
  │  data     │  Real instances only (default)                                  │
  │  meta     │  Meta-graph only (Realm/Layer/Kind/Trait/ArcFamily)            │
  │  overlay  │  Data + meta combined (debugging)                               │
  │  query    │  Faceted filter results (targeted exploration)                  │
  └─────────────────────────────────────────────────────────────────────────────┘
```

---

## Section: PIPELINE (Source of Truth Sync)

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                    SOURCE OF TRUTH PIPELINE - Schema Propagation                                  ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

     ┌─────────────────────────────────────────────────────────────────────────────────────────────┐
     │                        📁 YAML (Single Source of Truth)                                     │
     │                        packages/core/models/                                                │
     │                        ├── node-kinds/               ← 43 NodeKind files                     │
     │                        ├── arc-kinds/                ← 63 ArcKind files                      │
     │                        └── taxonomy.yaml             ← 3 Realms, 9 Layers, 5 Traits         │
     └─────────────────────────────────────────────┬───────────────────────────────────────────────┘
                                                   │
         ┌─────────────────────────────────────────┼─────────────────────────────────────────┐
         │                    │                    │                    │                     │
         ▼                    ▼                    ▼                    ▼                     ▼
  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────────┐
  │  📊 Mermaid  │   │  📝 Layer    │   │  🏷️ Kind    │   │  🔗 Arc      │   │  🗄️ Manual       │
  │  Generator   │   │  Generator   │   │  Generator   │   │  Schema Gen  │   │  Cypher Seeds    │
  │  tools/novanet│   │ tools/novanet│   │ tools/novanet│   │ tools/novanet│   │  packages/db/    │
  └──────┬───────┘   └──────┬───────┘   └──────┬───────┘   └──────┬───────┘   └────────┬─────────┘
         │                  │                  │                  │                     │
         ▼                  ▼                  ▼                  ▼                     ▼
  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────────┐
  │ VIEW-COMPLETE│   │  layers.ts   │   │ Kind :Meta   │   │ ArcKind     │   │ 00-constraints   │
  │ -GRAPH.md    │   │  src/graph/  │   │ nodes w/     │   │ :Meta nodes  │   │ 00.5-taxonomy    │
  │              │   │              │   │ schema_hint  │   │ w/ cypher_   │   │ 01-kinds         │
  │              │   │              │   │              │   │ pattern      │   │ 02-arc-kinds     │
  └──────────────┘   └──────────────┘   └──────────────┘   └──────────────┘   │ 99-autowire-kinds│
                                                                              │ (all generated)  │
                                                                              └────────┬─────────┘
                                                                                       │
     ┌─────────────────────────────────────────────────────────────────────────────────┘
     │
     ▼
  ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
  │   🐳 Neo4j (Runtime Database)                                                                │
  │   ├── bolt://localhost:7687                                                                  │
  │   └── Migrations: 001-inverse-relationships → 006-formatting-property-alignment              │
  └──────────────────────────────────────────────────────────────────────────────────────────────┘

  ═════════════════════════════════════════════════════════════════════════════════════════════════
   RUST VALIDATE:   novanet schema validate            (YAML <-> Neo4j consistency check)
   RUST GENERATE:   novanet schema generate            (Rebuilds all artifacts from YAML)
   RUST SEED:       novanet db seed                    (Execute seed Cypher files)
  ═════════════════════════════════════════════════════════════════════════════════════════════════
```

---

## Section: RUST (Binary Architecture)

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                    RUST BINARY — tools/novanet/ (v9)                                              ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

  ┌─────────────────────────────────────────────────────────────────────────────────────────────┐
  │  Single crate: tools/novanet/                                                               │
  │                                                                                             │
  │  Dependencies:                                                                              │
  │  ├── clap (derive)      ← CLI argument parsing                                             │
  │  ├── neo4rs             ← Neo4j Bolt protocol driver                                        │
  │  ├── ratatui            ← Terminal UI framework                                             │
  │  ├── crossterm          ← Terminal backend                                                  │
  │  ├── tokio              ← Async runtime                                                     │
  │  ├── serde + serde_yaml ← YAML deserialization                                              │
  │  ├── thiserror          ← Library error types                                               │
  │  └── color-eyre         ← Application error reporting                                       │
  │                                                                                             │
  │  Commands (all implemented, 246 tests):                                                     │
  │  ├── novanet data/meta/overlay/query       ← 4 navigation modes (faceted Cypher)            │
  │  ├── novanet node create/edit/delete       ← Node CRUD (label validation)                   │
  │  ├── novanet arc create/delete             ← Arc CRUD (type validation)                     │
  │  ├── novanet search --query=...            ← Fulltext + property search                     │
  │  ├── novanet locale list/import            ← Locale operations                              │
  │  ├── novanet db seed/migrate/reset         ← Database lifecycle                             │
  │  ├── novanet schema generate/validate      ← YAML → artifacts (7 generators)                │
  │  ├── novanet doc generate                  ← View-specific Mermaid (12 views)               │
  │  ├── novanet filter build                  ← JSON stdin → Cypher (Studio subprocess)        │
  │  └── novanet tui                           ← Interactive terminal (ratatui)                  │
  │                                                                                             │
  │  Architecture:                                                                              │
  │  ├── Rust generates artifacts AND executes at runtime                                       │
  │  └── TypeScript @novanet/schema-tools eliminated                                             │
  │                                                                                             │
  └─────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Section: LOCALE KNOWLEDGE

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              LOCALE KNOWLEDGE STRUCTURE                                           ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

     ┌─────────────────────────────────────────────────────────────────────────────────────────────┐
     │                                    Locale                                                   │
     │                       (en-US, fr-FR, ja-JP, es-ES, de-DE, fr-CA)                            │
     └───────────┬────────────┬────────────┬────────────┬────────────┬────────────┬────────────────┘
                 │            │            │            │            │            │
     ┌───────────┴──┐ ┌───────┴──┐ ┌───────┴──┐ ┌───────┴──┐ ┌───────┴──┐ ┌───────┴──────────┐
     │ HAS_IDENTITY │ │HAS_VOICE │ │HAS_CULTURE│ │HAS_MARKET│ │HAS_LEXICON│ │HAS_RULES_*       │
     └──────┬───────┘ └────┬─────┘ └─────┬─────┘ └────┬─────┘ └─────┬─────┘ └────────┬─────────┘
            │              │             │            │             │                │
            ▼              ▼             ▼            ▼             ▼                ▼
   ┌────────────────┐ ┌─────────┐ ┌───────────┐ ┌──────────┐ ┌───────────┐ ┌─────────────────────┐
   │LocaleIdentity  │ │LocaleVoice│ │LocaleCulture│ │LocaleMarket│ │LocaleLexicon│ │LocaleRules*        │
   │                │ │           │ │             │ │            │ │             │ │                    │
   │ • endonym      │ │• formality│ │ • taboos    │ │• currency  │ │• domain     │ │ Adaptation:        │
   │ • script       │ │• directness│ │• sensitivities│ │• payments │ │  terms      │ │  • units, dates    │
   │ • direction    │ │• emotion  │ │ • Hofstede  │ │• trust     │ │             │ │ Formatting:        │
   └────────────────┘ └───────────┘ │             │ └────────────┘ └──────┬──────┘ │  • patterns, decimals│
                                    │             │                      │        │ Slug:              │
                                    └──────┬──────┘                      │        │  • transliteration │
                                           │                             ▼        └─────────────────────┘
                         ┌─────────────────┴─────────────────┐   ┌─────────────┐
                         │      HAS_CULTURE_REFERENCES       │   │ Expression  │
                         │              ▼                    │   │ (urgency,   │
                         │   ┌───────────────────────┐       │   │  value,     │
                         │   │LocaleCultureReferences│       │   │  action)    │
                         │   └─────────┬─────────────┘       │   └─────────────┘
                         │             │                     │
                         │    ┌────────┼────────┐            │
                         │    ▼        ▼        ▼            │
                         │ Reference Metaphor Constraint     │
                         └───────────────────────────────────┘
```

---

## Section: INFRASTRUCTURE

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              INFRASTRUCTURE (packages/db)                                         ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  🐳 DOCKER                                                                                          │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│   docker-compose.yml                                                                                │
│   ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│   │  Neo4j 5.26.0 Community                                                                     │   │
│   │  ├── Browser: http://localhost:7474                                                         │   │
│   │  ├── Bolt: bolt://localhost:7687                                                            │   │
│   │  └── Auth: neo4j / novanetpassword                                                          │   │
│   └─────────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                     │
│   seed/ (generated + manual, executes en ordre)                                                     │
│   ├── 00-constraints.cypher        ← Contraintes d'unicite (UNIQUE, EXISTS)                         │
│   ├── 00.5-taxonomy.cypher         ← Taxonomy: Realms, Layers, Traits (GENERATED)                   │
│   ├── 01-kinds.cypher              ← NodeKinds meta-nodes (GENERATED)                               │
│   ├── 01-vector-indexes.cypher     ← Index vectoriels pour recherche semantique                     │
│   ├── 02-arc-kinds.cypher          ← ArcKinds meta-nodes (GENERATED)                                │
│   ├── 20-locales.cypher            ← Locales from CSV + MD (GENERATED)                              │
│   ├── 21-locale-knowledge.cypher   ← LocaleIdentity, LocaleVoice, LocaleCulture, etc.               │
│   ├── 31-project-qrcode-ai.cypher  ← Projet QR Code AI avec Pages, Blocks                           │
│   ├── 34-prompts.cypher            ← BlockPrompt, PagePrompt                                        │
│   └── 99-autowire-kinds.cypher     ← Links data nodes to Kinds (GENERATED)                          │
│                                                                                                     │
│   migrations/ (6 fichiers, idempotents)                                                             │
│   ├── 001-inverse-relationships.cypher                                                              │
│   ├── 002-semantic-link-inverses.cypher                                                             │
│   ├── 003-vector-indexes.cypher                                                                     │
│   ├── 004-remove-deprecated-properties.cypher                                                       │
│   ├── 005-yaml-synchronization.cypher      ← Alignement YAML v7.11.0                                │
│   └── 006-formatting-property-alignment.cypher ← LocaleRulesFormatting props                        │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Section: STUDIO

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              STUDIO (apps/studio) - Next.js 16 + React 19                         ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│   ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│   │  API ROUTES (10)                                  │  ZUSTAND STORES (8)                    │   │
│   ├────────────────────────────────────────────────────┼────────────────────────────────────────┤   │
│   │                                                    │                                        │   │
│   │  /api/chat          → Claude AI                    │  graphStore      → nodes, edges       │   │
│   │  /api/graph         → main data                    │  filterStore     → types, presets     │   │
│   │  /api/graph/expand  → neighbors                    │  uiStore         → panels, selection  │   │
│   │  /api/graph/ontology→ metadata                     │  chatStore       → AI messages        │   │
│   │  /api/graph/organizing-principles                  │  queryStore      → Cypher state       │   │
│   │  /api/graph/query   → Cypher exec                  │  viewStore       → saved views        │   │
│   │  /api/graph/schema  → schema info                  │  aiQueryStore    → AI query state     │   │
│   │  /api/graph/stats   → statistics                   │  animationStore  → animations         │   │
│   │  /api/views         → CRUD views                   │                                        │   │
│   │  /api/views/[id]    → single view                  │                                        │   │
│   │                                                    │                                        │   │
│   └────────────────────────────────────────────────────┴────────────────────────────────────────┘   │
│                                                                                                     │
│   ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│   │  VISUALIZATION — NavigationMode (v9)                                                       │   │
│   ├─────────────────────────────────────────────────────────────────────────────────────────────┤   │
│   │                                                                                             │   │
│   │  ┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐                 │   │
│   │  │  DATA MODE         │  │  META MODE          │  │  OVERLAY MODE      │                 │   │
│   │  │  (~19k instances)  │  │  (105 meta-nodes)   │  │  Data + Meta       │                 │   │
│   │  │                    │  │                     │  │                    │                 │   │
│   │  │  Real Neo4j data   │  │  Realm/Layer/Kind   │  │  Architecture      │                 │   │
│   │  │  Force-directed    │  │  Trait/ArcFamily    │  │  debugging         │                 │   │
│   │  │  Grouped by Realm  │  │  Hierarchical       │  │                    │                 │   │
│   │  └─────────────────────┘  └─────────────────────┘  └─────────────────────┘                 │   │
│   │                                                                                             │   │
│   │  ┌─────────────────────┐                                                                    │   │
│   │  │  QUERY MODE        │  Visual Encoding (v10.5):                                          │   │
│   │  │  Faceted filters   │  ├── Fill color   → Layer (10 colors)                              │   │
│   │  │                    │  ├── Border style  → Trait (5 styles)                               │   │
│   │  │  Realm + Layer +   │  ├── Spatial group → Realm (3 zones)                               │   │
│   │  │  Trait combos      │  └── Arc stroke    → ArcFamily (5 colors)                         │   │
│   │  └─────────────────────┘                                                                    │   │
│   │                                                                                             │   │
│   └─────────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Section: PACKAGES

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║  📦 PACKAGES DEPENDENCY GRAPH                                                                     ║
╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                   ║
║                    ┌──────────────────┐                                                           ║
║                    │  @novanet/core   │  ← Types, schemas, generators, filters                    ║
║                    │  (source truth)  │     node-kinds/ + arc-kinds/ + taxonomy.yaml             ║
║                    └────────┬─────────┘     v10.5: 3 Realms, 9 Layers                           ║
║                             │                                                                     ║
║              ┌──────────────┼──────────────┐                                                      ║
║              │              │              │                                                      ║
║              ▼              ▼              ▼                                                      ║
║              ┌──────────────────┐  ┌──────────────────┐                                          ║
║              │ @novanet/studio  │  │  @novanet/db     │                                          ║
║              │ (visualization)  │  │  (infrastructure)│  ← Standalone (Cypher files only)         ║
║              └──────────────────┘  └──────────────────┘                                          ║
║                                                                                                   ║
║   ┌──────────────────────────────────────────────────────────────────────────────────────┐        ║
║   │  tools/novanet/  ← Rust binary (CLI + TUI + generators)                              │        ║
║   │  ├─ generators/  MermaidGen, LayerGen, KindGen, ArcSchemaGen, AutowireGen, ...      │        ║
║   │  ├─ parsers/     YAML nodes, relations, locale markdown                              │        ║
║   │  ├─ commands/    schema, db, locale, search, filter, data/meta/overlay/query         │        ║
║   │  ├─ search/      Hybrid vector + graph search                                        │        ║
║   │  ├─ filter/      Cypher filter builder (Studio subprocess)                           │        ║
║   │  └─ tui/         Interactive terminal (ratatui)                                      │        ║
║   └──────────────────────────────────────────────────────────────────────────────────────┘        ║
║                                                                                                   ║
║   ⚠ ELIMINATED: @novanet/schema-tools, @novanet/cli (absorbed into Rust binary)                   ║
║                                                                                                   ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

---

## Section: FLOW (Generation Pipeline)

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║  🔄 DATA FLOW: Generation Pipeline                                                                ║
╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                   ║
║   1. INVARIANT NODES (no locale)                                                                  ║
║   ┌─────────┐     ┌─────────┐     ┌─────────┐                                                     ║
║   │ Project │────▶│  Page   │────▶│  Block  │                                                     ║
║   └────┬────┘     └────┬────┘     └────┬────┘                                                     ║
║        │               │               │                                                          ║
║        │               │               │                                                          ║
║   2. SEMANTIC LAYER    │               │                                                          ║
║        │          ┌────┴────┐     ┌────┴────┐                                                     ║
║        │          │ Entity  │◀────│USES_ENTITY                                                    ║
║        │          └────┬────┘     └─────────┘                                                     ║
║        │               │                                                                          ║
║   3. LOCALIZATION      │                                                                          ║
║        │          ┌────┴─────┐                                                                    ║
║        │          │HAS_L10N  │                                                                    ║
║        │          ▼          │                                                                    ║
║   ┌────┴─────┐  ┌────────────┴───┐                                                                ║
║   │ProjectL10n│  │   EntityL10n   │──────┐                                                        ║
║   └──────────┘  └────────────────┘      │                                                         ║
║                                          │                                                        ║
║   4. GENERATION (LLM)                    │                                                        ║
║        ┌─────────────────────────────────┘                                                        ║
║        │     ┌─────────────┐                                                                      ║
║        └────▶│ BlockPrompt │                                                                      ║
║              └──────┬──────┘                                                                      ║
║                     │ GENERATED                                                                   ║
║                     ▼                                                                             ║
║              ┌─────────────┐                                                                      ║
║              │  BlockL10n  │  ← Native content (NOT translation)                                  ║
║              └─────────────┘                                                                      ║
║                                                                                                   ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

---

## Key Numbers (v10.6.0)

| Metric | Value |
|--------|-------|
| Kind (node types) | 46 |
| ArcKind (arcs) | 51 |
| Realms | 2 (global, tenant) |
| Layers | 9 (3 global, 6 tenant) |
| Traits | 5 |
| ArcFamilies | 5 |
| Locale Knowledge nodes | 18 |
| Seed files | 11 |
| Migrations | 6 |
| Locales supported | 200 |
| API routes (Studio) | 10 |
| Zustand stores | 8 |

---

## Commands

```bash
# Schema & docs (YAML, no Neo4j)
novanet schema generate            # Regenerate all artifacts from YAML
novanet schema validate            # Validate YAML coherence
novanet doc generate               # Generate 12 view Mermaid diagrams

# Read modes (Neo4j)
novanet data                       # Mode 1: Data nodes
novanet meta                       # Mode 2: Meta-graph
novanet overlay                    # Mode 3: Data + Meta
novanet query --realm=project      # Mode 4: Faceted query

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
novanet tui                        # Terminal UI (ratatui)
```

---

## Usage

User can invoke with:
- `/novanet-arch` or `/novanet-architecture`
- `/novanet-arch source` - YAML source only
- `/novanet-arch meta` - Meta-Graph (v9 faceted classification)
- `/novanet-arch pipeline` - Source of Truth Pipeline
- `/novanet-arch locale` - Locale Knowledge Structure
- `/novanet-arch infra` - Infrastructure only
- `/novanet-arch studio` - Studio only
- `/novanet-arch packages` - Package dependencies
- `/novanet-arch flow` - Generation pipeline
- `/novanet-arch rust` - Rust binary architecture
