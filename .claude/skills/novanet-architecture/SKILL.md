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

## Instructions

When this skill is invoked, display the following ASCII architecture diagram:

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║                              NOVANET - ARCHITECTURE GLOBALE                                       ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  📁 SOURCE DE VERITE                                                                                │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
│   packages/core/models/                                                                             │
│   ├── nodes/                          ← 35 fichiers YAML (1 par NodeType)                           │
│   │   ├── global/config/              ← Locale                                                      │
│   │   ├── global/knowledge/           ← 14 LocaleKnowledge nodes                                    │
│   │   ├── project/foundation/         ← Project, BrandIdentity, ProjectL10n                         │
│   │   ├── project/structure/          ← Page, Block                                                 │
│   │   ├── project/semantic/           ← Concept, ConceptL10n                                        │
│   │   ├── project/instruction/        ← PageType, PagePrompt, BlockType, BlockPrompt, BlockRules    │
│   │   ├── project/output/             ← PageL10n, BlockL10n                                         │
│   │   ├── shared/seo/                 ← SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun             │
│   │   └── shared/geo/                 ← GEOSeedL10n, GEOSeedMetrics, GEOMiningRun                   │
│   │                                                                                                 │
│   └── relations.yaml                  ← 47 types de relations Neo4j                                 │
│                                                                                                     │
└───────────────────────────────────────────────┬─────────────────────────────────────────────────────┘
                                                │
                        ┌───────────────────────┼───────────────────────┐
                        │                       │                       │
                        ▼                       ▼                       ▼
┌───────────────────────────────┐ ┌───────────────────────────────┐ ┌───────────────────────────────┐
│  📊 MermaidGenerator          │ │  📝 SubcategoryGenerator      │ │  🗄️ Neo4j Seeds               │
├───────────────────────────────┤ ├───────────────────────────────┤ ├───────────────────────────────┤
│                               │ │                               │ │                               │
│  YAML → Mermaid diagrams      │ │  YAML folders → TypeScript    │ │  YAML → Cypher scripts        │
│                               │ │                               │ │                               │
│  Output:                      │ │  Output:                      │ │  Output:                      │
│  docs/graphs/*.md             │ │  src/graph/subcategories.ts   │ │  packages/db/seed/*.cypher    │
│                               │ │                               │ │                               │
└───────────────────────────────┘ └───────────────────────────────┘ └───────────────┬───────────────┘
                                                                                    │
                                                                                    ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  🐳 INFRASTRUCTURE (packages/db)                                                                    │
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
│   seed/ (7 fichiers)                                                                                │
│   ├── 00-constraints.cypher    ← Contraintes d'unicite                                              │
│   ├── 01-locales.cypher        ← Locales de base (fr-FR, en-US, ...)                                │
│   ├── 02-locale-knowledge.cypher                                                                    │
│   ├── 03-project.cypher        ← Projet QR Code AI                                                  │
│   ├── 04-pages-blocks.cypher                                                                        │
│   ├── 05-concepts.cypher                                                                            │
│   └── 06-seo-geo.cypher                                                                             │
│                                                                                                     │
└───────────────────────────────────────────────┬─────────────────────────────────────────────────────┘
                                                │
                                                │ Bolt Protocol
                                                ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  🖥️ STUDIO (apps/studio) - Next.js 16 + React 19                                                    │
├─────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                     │
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

Then display the packages dependency graph:

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║  📦 PACKAGES DEPENDENCY GRAPH                                                                     ║
╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                   ║
║                    ┌──────────────────┐                                                           ║
║                    │  @novanet/core   │  ← Types, schemas, generators, filters                    ║
║                    │  (source truth)  │                                                           ║
║                    └────────┬─────────┘                                                           ║
║                             │                                                                     ║
║              ┌──────────────┼──────────────┐                                                      ║
║              │              │              │                                                      ║
║              ▼              ▼              ▼                                                      ║
║   ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐                                  ║
║   │  @novanet/cli    │ │ @novanet/studio  │ │ @novanet/        │                                  ║
║   │  (validation)    │ │ (visualization)  │ │ schema-tools     │                                  ║
║   └──────────────────┘ └──────────────────┘ └──────────────────┘                                  ║
║                                                                                                   ║
║   ┌──────────────────┐                                                                            ║
║   │  @novanet/db     │  ← Standalone (Docker, seeds, migrations)                                  ║
║   │  (infrastructure)│                                                                            ║
║   └──────────────────┘                                                                            ║
║                                                                                                   ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

And finally the data flow diagram:

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

## Key Numbers (Verified)

After displaying the diagrams, show this summary:

| Metric | Value |
|--------|-------|
| Node types | 35 |
| Relations | 47 |
| Scopes | 3 (Global, Shared, Project) |
| Subcategories | 9 |
| API routes | 9 |
| Zustand stores | 8 |
| Seed files | 7 |
| Locales supported | 200+ |

## Usage

User can invoke with:
- `/novanet-architecture` or `/novanet-arch`
- "Show me the NovaNet architecture"
- "How does the system work?"
- "Architecture diagram"
