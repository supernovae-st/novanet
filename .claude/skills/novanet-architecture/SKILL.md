---
name: novanet-architecture
description: Display the complete NovaNet architecture diagram in ASCII. Use when user asks about architecture, system overview, how components connect, or wants to understand the codebase structure.
disable-model-invocation: false
user-invocable: true
---

# NovaNet Architecture Overview

Display the complete NovaNet architecture diagram showing:
- Source of truth (YAML models)
- Generators (Mermaid, Subcategory)
- Neo4j infrastructure
- Studio visualization
- Source of Truth Pipeline

## Instructions

Based on the `$ARGUMENTS` provided, display the appropriate section:

- **"source"** or **"yaml"** - Show Source de Verite section only
- **"infra"** or **"neo4j"** - Show Infrastructure section only
- **"studio"** - Show Studio section only
- **"packages"** or **"deps"** - Show Packages Dependency Graph
- **"flow"** or **"generation"** - Show Data Flow (Generation Pipeline)
- **"pipeline"** or **"sync"** - Show Source of Truth Pipeline
- **"locale"** or **"knowledge"** - Show Locale Knowledge Structure
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
│   ├── _index.yaml                     ← Index du graphe (structure, changelog)                      │
│   ├── nodes/                          ← 35 fichiers YAML (1 par NodeType)                           │
│   │   ├── global/config/              ← Locale                                                      │
│   │   ├── global/knowledge/           ← 14 LocaleKnowledge nodes                                    │
│   │   │   ├── locale-identity.yaml    ← Endonym, script, direction                                  │
│   │   │   ├── locale-voice.yaml       ← Formality, directness, emotion                              │
│   │   │   ├── locale-culture.yaml     ← Taboos, sensitivities, Hofstede                             │
│   │   │   ├── locale-market.yaml      ← Currency, payments, trust signals                           │
│   │   │   ├── locale-lexicon.yaml     ← Domain vocabulary                                           │
│   │   │   ├── locale-rules-*.yaml     ← Adaptation, Formatting, Slug rules                          │
│   │   │   └── expression.yaml, etc.   ← Reference, Metaphor, Pattern, Constraint                    │
│   │   ├── project/foundation/         ← Project, BrandIdentity, ProjectL10n                         │
│   │   ├── project/structure/          ← Page, Block, PageType, BlockType                            │
│   │   ├── project/semantic/           ← Concept, ConceptL10n                                        │
│   │   ├── project/instruction/        ← PagePrompt, BlockPrompt, BlockRules                         │
│   │   ├── project/output/             ← PageL10n, BlockL10n                                         │
│   │   ├── shared/seo/                 ← SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun             │
│   │   └── shared/geo/                 ← GEOSeedL10n, GEOSeedMetrics, GEOMiningRun                   │
│   │                                                                                                 │
│   ├── relations.yaml                  ← 47 types de relations Neo4j                                 │
│   └── views/                          ← Definitions de vues YAML                                    │
│                                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
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
     │                        ├── nodes/           ← 35 NodeTypes                                  │
     │                        └── relations.yaml   ← 47 Relations                                  │
     └─────────────────────────────────────────────┬───────────────────────────────────────────────┘
                                                   │
            ┌──────────────────────────────────────┼──────────────────────────────────────┐
            │                                      │                                      │
            ▼                                      ▼                                      ▼
  ┌─────────────────────────┐         ┌─────────────────────────┐         ┌─────────────────────────┐
  │   📊 MermaidGenerator   │         │  📝 SubcategoryGenerator │         │   🗄️ Manual Cypher     │
  │   @novanet/schema-tools │         │   @novanet/schema-tools  │         │   packages/db/seed/     │
  └───────────┬─────────────┘         └───────────┬─────────────┘         └───────────┬─────────────┘
              │                                   │                                   │
              ▼                                   ▼                                   ▼
  ┌─────────────────────────┐         ┌─────────────────────────┐         ┌─────────────────────────┐
  │  VIEW-COMPLETE-GRAPH.md │         │   subcategories.ts      │         │   00-constraints.cypher │
  │  models/docs/views/     │         │   src/graph/            │         │   01-concepts-mvp.cypher│
  └─────────────────────────┘         └─────────────────────────┘         │   02-locale-knowledge   │
                                                                          │   02-vector-indexes     │
                                                                          │   03-prompts-v720       │
                                                                          │   04-project-qrcode-ai  │
                                                                          │   05-missing-nodes      │
                                                                          └───────────┬─────────────┘
                                                                                      │
     ┌────────────────────────────────────────────────────────────────────────────────┘
     │
     ▼
  ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
  │   🐳 Neo4j (Runtime Database)                                                                │
  │   ├── bolt://localhost:7687                                                                  │
  │   └── Migrations: 001-inverse-relationships → 006-formatting-property-alignment              │
  └──────────────────────────────────────────────────────────────────────────────────────────────┘

  ═════════════════════════════════════════════════════════════════════════════════════════════════
   VALIDATION: pnpm schema:validate    (CI checks TypeScript + Mermaid sync with YAML)
   REGENERATE: pnpm schema:generate    (Rebuilds generated files from YAML)
  ═════════════════════════════════════════════════════════════════════════════════════════════════
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
│   │  API ROUTES (9)                                    │  ZUSTAND STORES (8)                    │   │
│   ├────────────────────────────────────────────────────┼────────────────────────────────────────┤   │
│   │                                                    │                                        │   │
│   │  /api/chat          → Claude AI                    │  graphStore      → nodes, edges       │   │
│   │  /api/graph         → main data                    │  filterStore     → types, presets     │   │
│   │  /api/graph/expand  → neighbors                    │  uiStore         → panels, selection  │   │
│   │  /api/graph/ontology→ metadata                     │  chatStore       → AI messages        │   │
│   │  /api/graph/query   → Cypher exec                  │  queryStore      → Cypher state       │   │
│   │  /api/graph/schema  → schema info                  │  viewStore       → saved views        │   │
│   │  /api/graph/stats   → statistics                   │  aiQueryStore    → AI query state     │   │
│   │  /api/views         → CRUD views                   │  animationStore  → animations         │   │
│   │  /api/views/[id]    → single view                  │                                        │   │
│   │                                                    │                                        │   │
│   └────────────────────────────────────────────────────┴────────────────────────────────────────┘   │
│                                                                                                     │
│   ┌─────────────────────────────────────────────────────────────────────────────────────────────┐   │
│   │  VISUALIZATION                                                                              │   │
│   ├─────────────────────────────────────────────────────────────────────────────────────────────┤   │
│   │                                                                                             │   │
│   │  ┌─────────────────────────────┐    ┌─────────────────────────────┐                         │   │
│   │  │  SCHEMA MODE               │    │  DATA MODE                  │                         │   │
│   │  │  (35 NodeTypes)            │    │  (~19k instances projected) │                         │   │
│   │  │                            │    │                             │                         │   │
│   │  │  React Flow + ELK          │    │  React Flow / Force-Graph   │                         │   │
│   │  │  Hierarchical layout       │    │  Force-directed layout      │                         │   │
│   │  │  Grouped by Scope          │    │  Real Neo4j data            │                         │   │
│   │  └─────────────────────────────┘    └─────────────────────────────┘                         │   │
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
║                    └────────┬─────────┘                                                           ║
║                             │                                                                     ║
║              ┌──────────────┼──────────────┐                                                      ║
║              │              │              │                                                      ║
║              ▼              ▼              ▼                                                      ║
║   ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐                                  ║
║   │  @novanet/cli    │ │ @novanet/studio  │ │ @novanet/        │                                  ║
║   │  (validation)    │ │ (visualization)  │ │ schema-tools     │                                  ║
║   └──────────────────┘ └──────────────────┘ └──────────────────┘                                  ║
║                                               ├─ MermaidGenerator                                 ║
║   ┌──────────────────┐                        ├─ SubcategoryGenerator                             ║
║   │  @novanet/db     │                        └─ validate-sync.ts                                 ║
║   │  (infrastructure)│  ← Standalone (Docker, seeds, migrations)                                  ║
║   └──────────────────┘                                                                            ║
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

## Key Numbers (Verified v8.2.0)

| Metric | Value |
|--------|-------|
| Node types | 35 |
| Relations | 47 |
| Scopes | 3 (Global, Shared, Project) |
| Subcategories | 9 |
| Locale Knowledge nodes | 14 |
| Seed files | 7 |
| Migrations | 6 |
| Locales supported | 200+ |
| API routes | 9 |
| Zustand stores | 8 |

---

## Commands

```bash
# Validate schema sync
pnpm schema:validate

# Regenerate from YAML
pnpm schema:generate

# Reset database
pnpm infra:reset

# View in browser
open http://localhost:7474
```

---

## Usage

User can invoke with:
- `/novanet-arch` or `/novanet-architecture`
- `/novanet-arch source` - YAML source only
- `/novanet-arch pipeline` - Source of Truth Pipeline
- `/novanet-arch locale` - Locale Knowledge Structure
- `/novanet-arch infra` - Infrastructure only
- `/novanet-arch studio` - Studio only
- `/novanet-arch packages` - Package dependencies
- `/novanet-arch flow` - Generation pipeline
