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
│   ├── organizing-principles.yaml           ← v9: Realm/Layer/Trait/ArcFamily [→ taxonomy.yaml]     │
│   ├── nodes/                               ← 44 fichiers YAML (1 par Kind) [→ node-kinds/]        │
│   │   ├── global/                          ← Realm: global                                          │
│   │   │   ├── config/                      ←   Layer: config (Locale)                               │
│   │   │   └── knowledge/                   ←   Layer: knowledge (14 nodes)                          │
│   │   │       ├── locale-identity.yaml     ← Endonym, script, direction                             │
│   │   │       ├── locale-voice.yaml        ← Formality, directness, emotion                         │
│   │   │       ├── locale-culture.yaml      ← Taboos, sensitivities, Hofstede                        │
│   │   │       ├── locale-market.yaml       ← Currency, payments, trust signals                      │
│   │   │       ├── locale-lexicon.yaml      ← Domain vocabulary                                      │
│   │   │       ├── locale-rules-*.yaml      ← Adaptation, Formatting, Slug rules                     │
│   │   │       └── expression.yaml, etc.    ← Reference, Metaphor, Pattern, Constraint               │
│   │   ├── project/                         ← Realm: project                                         │
│   │   │   ├── foundation/                  ←   Layer: foundation (Project, BrandIdentity, ProjectL10n)│
│   │   │   ├── structure/                   ←   Layer: structure (Page, Block, PageType, BlockType)   │
│   │   │   ├── semantic/                    ←   Layer: semantic (Concept, ConceptL10n)                │
│   │   │   ├── instruction/                 ←   Layer: instruction (Prompts, Rules)                   │
│   │   │   └── output/                      ←   Layer: output (PageL10n, BlockL10n)                   │
│   │   └── shared/                          ← Realm: shared                                          │
│   │       ├── seo/                         ←   Layer: seo (Keyword, Metrics, MiningRun)              │
│   │       └── geo/                         ←   Layer: geo (Seed, Metrics, MiningRun)                 │
│   │                                                                                                 │
│   ├── relations.yaml                       ← 83 Arc types Neo4j (with family field) [→ arc-kinds/]  │
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
  │   Axis 1 — WHERE?   :Realm       (3)  global / project / shared                             │
  │   Axis 2 — WHAT?    :Layer       (9)  config, knowledge, foundation, structure, semantic,    │
  │                                        instruction, output, seo, geo                         │
  │   Axis 3 — HOW?     :Trait       (5)  invariant / localized / knowledge / derived / job      │
  │   Axis 4 — LINKS?   :ArcKind    (83)  grouped into 5 ArcFamilies                            │
  │                                                                                              │
  └──────────────────────────────────────────────────────────────────────────────────────────────┘

  6 Meta-Node Types (all carry :Meta double-label):

  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
  │  Realm (3)  │───▶│  Layer (9)  │───▶│  Kind (44)  │
  │  WHERE?     │    │  WHAT?      │    │  1:1 label  │
  │  HAS_LAYER  │    │  HAS_KIND   │    │             │
  └─────────────┘    └─────────────┘    └──────┬──────┘
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
  │  ArcKind (83) │───────────────▶│  Kind (35)  │◀──────────────│  ArcKind (83) │
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
     │                        ├── nodes/                    ← 44 Kinds [→ node-kinds/]             │
     │                        ├── relations.yaml            ← 83 Arcs [→ arc-kinds/]               │
     │                        └── organizing-principles.yaml← [→ taxonomy.yaml] facet defs        │
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
  │ -GRAPH.md    │   │  src/graph/  │   │ nodes w/     │   │ :Meta nodes  │   │ 01-concepts-mvp  │
  │              │   │              │   │ schema_hint  │   │ w/ cypher_   │   │ 02-locale-know.  │
  │              │   │              │   │              │   │ pattern      │   │ 03-prompts       │
  └──────────────┘   └──────────────┘   └──────────────┘   └──────────────┘   │ 04-project       │
                                                                              │ 05-missing-nodes │
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
  │  Commands (all implemented, 201 tests):                                                     │
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
│   seed/ (7 fichiers, executes en ordre)                                                             │
│   ├── 00-constraints.cypher        ← Contraintes d'unicite (UNIQUE, EXISTS)                         │
│   ├── 01-concepts-mvp.cypher       ← Locales + Concepts invariants (Tier Free, Pro, etc.)           │
│   ├── 02-locale-knowledge.cypher   ← LocaleIdentity, LocaleVoice, LocaleCulture, etc.               │
│   ├── 02-vector-indexes.cypher     ← Index vectoriels pour recherche semantique                     │
│   ├── 03-prompts-v720.cypher       ← BlockPrompt, PagePrompt (v7.2.0+)                              │
│   ├── 04-project-qrcode-ai.cypher  ← Projet QR Code AI avec Pages, Blocks                           │
│   └── 05-missing-nodes.cypher      ← Nodes manquants (cleanup)                                      │
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
│   │  │  QUERY MODE        │  Visual Encoding (v9):                                             │   │
│   │  │  Faceted filters   │  ├── Fill color   → Layer (9 colors)                               │   │
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
║                    │  (source truth)  │     models/nodes/*.yaml + relations.yaml                  ║
║                    └────────┬─────────┘     + organizing-principles.yaml                         ║
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
║        │          │ Concept │◀────│USES_CONCEPT                                                   ║
║        │          └────┬────┘     └─────────┘                                                     ║
║        │               │                                                                          ║
║   3. LOCALIZATION      │                                                                          ║
║        │          ┌────┴─────┐                                                                    ║
║        │          │HAS_L10N  │                                                                    ║
║        │          ▼          │                                                                    ║
║   ┌────┴─────┐  ┌────────────┴───┐                                                                ║
║   │ProjectL10n│  │  ConceptL10n   │──────┐                                                        ║
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

## Key Numbers (v9.5.0)

| Metric | Value |
|--------|-------|
| Kind (node types) | 44 |
| ArcKind (relations) | 83 |
| Realms | 3 (global, project, shared) |
| Layers | 9 |
| Traits | 5 |
| ArcFamilies | 5 |
| Meta-node total | 141 (3+9+44+5+5+83+1 bridge type) |
| Locale Knowledge nodes | 14 |
| Seed files | 7 |
| Migrations | 6 |
| Locales supported | 200+ |
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
novanet arc create --from=a --to=b --kind=USES_CONCEPT

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
