# 🪽 NovaNet Models v10.4.0

YAML schema definitions for Neo4j graph database.

> **Graph Diagram**: See [VIEW-COMPLETE-GRAPH.md](docs/views/VIEW-COMPLETE-GRAPH.md) for auto-generated Mermaid diagram.
> Run `npm run generate:docs` to regenerate all view documentation.

## Directory Structure

```
models/
├── _index.yaml              # Graph overview + file index
├── relations.yaml           # All Neo4j relationships (50)
├── README.md                # This file
├── nodes/                   # ONE FILE PER NODE TYPE (42 files)
│   ├── global/              # 🌍 GLOBAL realm (19 nodes)
│   │   ├── config/          #    Locale
│   │   ├── knowledge/       #    Entity, EntityL10n, Knowledge Atoms
│   │   └── seo/             #    SEOKeyword, SEOKeywordMetrics, SEOMiningRun
│   └── project/             # 📦 PROJECT realm (23 nodes)
│       ├── foundation/      #    Project, ProjectL10n, BrandIdentity
│       ├── structure/       #    Page, Block, ContentSlot
│       ├── semantic/        #    AudiencePersona, ChannelSurface
│       ├── instruction/     #    PagePrompt, BlockPrompt, BlockType, PageType, BlockRules, BlockInstruction, PromptArtifact
│       └── output/          #    PageL10n, BlockL10n, GenerationJob, OutputArtifact, EvaluationSignal
├── views/                   # View definitions (YAML)
│   ├── _registry.yaml
│   └── *.yaml
├── docs/views/              # Auto-generated view docs (MD)
│   └── VIEW-*.md
├── config/                  # Configuration files
├── schema/                  # JSON schemas
└── archive/                 # Deprecated files
```

## Multi-Tenant Architecture

NovaNet uses a **2-realm architecture** (v10.2 merged SHARED into GLOBAL):

| Realm | Count | Description | Examples |
|-------|-------|-------------|----------|
| 🌍 **GLOBAL** | 19 | Locale knowledge + shared data | Locale, Entity, EntityL10n, Knowledge Atoms, SEOKeyword |
| 📦 **PROJECT** | 23 | Per-project content | Project, Page, Block, PageL10n, BlockL10n, GenerationJob |

## Locale Behavior Classification

| Behavior | Icon | Description | Nodes |
|----------|------|-------------|-------|
| **INVARIANT** | 🔵 | Defined once, language-independent | Project, Entity, Page, Block, Locale |
| **LOCALIZED** | 🟢 | Per-locale, has `:FOR_LOCALE` | ProjectL10n, EntityL10n, PageL10n, BlockL10n |
| **LOCALE_KNOWLEDGE** | 🟣 | Knowledge ABOUT a locale | LocaleIdentity, LocaleVoice, LocaleCulture, Expression |
| **DERIVED** | ⚪ | Inherits locale from parent | SEOKeywordMetrics, GEOSeedMetrics |
| **JOB** | ⚙️ | Background jobs, no locale | SEOMiningRun, GEOMiningRun |

## Nomenclature v10.4.0

```
*L10n suffix    = ALL localized content (human OR LLM generated)
:HAS_L10N       = human-curated content (EntityL10n, ProjectL10n)
:HAS_OUTPUT     = LLM-generated content (PageL10n, BlockL10n)
*Set            = Container nodes (TermSet, ExpressionSet, PatternSet, etc.)
Atoms           = Granular knowledge (Term, Expression, Pattern, Taboo, CultureRef, AudienceTrait)
*Metrics        = Time-series observations (SEOKeywordMetrics)
```

## Core Pattern: Invariant -> Localized

```
Invariant (EN)              Localized (generated natively)
──────────────              ────────────────────────────────
Entity.key                 EntityL10n.title (per locale)
Page.instructions           PageL10n.assembled (per locale)
Block.instructions          BlockL10n.generated (per locale)
```

## Locale Knowledge Hierarchy

```
Locale ──[:HAS_IDENTITY]──> LocaleIdentity
       ──[:HAS_VOICE]─────> LocaleVoice
       ──[:HAS_CULTURE]───> LocaleCulture ──[:HAS_CONSTRAINT]──> Constraint
       │                                  └─[:HAS_CULTURE_REFERENCES]──> LocaleCultureReferences
       │                                                                  ├─[:HAS_REFERENCE]──> Reference
       │                                                                  └─[:HAS_METAPHOR]──> Metaphor
       ──[:HAS_MARKET]────> LocaleMarket
       ──[:HAS_LEXICON]───> LocaleLexicon ──[:HAS_EXPRESSION]──> Expression
       ──[:HAS_RULES_*]───> LocaleRules* ──[:HAS_PATTERN]──> Pattern
```

## View Documentation (Auto-Generated)

| View | Category | Description |
|------|----------|-------------|
| [Complete Graph](docs/views/VIEW-COMPLETE-GRAPH.md) | Overview | Full graph with all 35 nodes and 50 relations |
| [Project Context](docs/views/VIEW-PROJECT-CONTEXT.md) | Overview | Project boundaries and configuration |
| [Page Generation](docs/views/VIEW-PAGE-GENERATION-CONTEXT.md) | Generation | Orchestrator context for page generation |
| [Block Generation](docs/views/VIEW-BLOCK-GENERATION.md) | Generation | Sub-agent context for block generation |
| [Locale Knowledge](docs/views/VIEW-LOCALE-FULL-KNOWLEDGE.md) | Localization | Complete locale knowledge system |
| [Entity Network](docs/views/VIEW-CONCEPT-ECOSYSTEM.md) | Localization | Entity graph with semantic links |
| [Spreading Activation](docs/views/VIEW-BLOCK-SEMANTIC-NETWORK.md) | Semantic | Temperature-based semantic traversal |
| [SEO Pipeline](docs/views/VIEW-SEO-PIPELINE.md) | Mining | SEO keyword mining workflow |
| [GEO Pipeline](docs/views/VIEW-GEO-PIPELINE.md) | Mining | GEO answer engine optimization |

## Relationship Properties Reference

| Relationship | Props | Description |
|--------------|-------|-------------|
| `SUPPORTS_LOCALE` | `status` | active \| pending \| disabled |
| `DEFAULT_LOCALE` | (none) | Exactly one per project |
| `HAS_BLOCK` | `position` | Display order |
| `USES_ENTITY` | `purpose, temperature` | primary/secondary/contextual (renamed in v10.3) |
| `SEMANTIC_LINK` | `type, temperature` | Entity relationships |
| `HAS_SEO_TARGET` | `role, priority` | locale-aligned: primary/secondary/long-tail |
| `HAS_GEO_TARGET` | `role, priority` | locale-aligned: primary/contextual |
| `PREVIOUS_VERSION` | (none) | History chain: BlockL10n/PageL10n → previous |
| `LINKS_TO` | `concept_key, context, seo_weight` | Explicit internal link (Page → Page) |
| `SUBTOPIC_OF` | (none) | Pillar-cluster hierarchy (Page → Page) |

## Statistics

| Metric | Count |
|--------|-------|
| **Total Nodes** | 42 |
| **Total Arcs** | 77 |
| **Realms** | 2 (Global, Project) |
| **Layers** | 8 |
| **Node Traits** | 5 (invariant, localized, knowledge, derived, job) |

## Adding New Nodes

1. Create `models/nodes/{category}/{node-name}.yaml`
2. Add to `models/_index.yaml` files list
3. Add TypeScript types to `src/types/`
4. Add Zod schema to `src/schemas/`
5. Add to `src/filters/types.ts` NodeType union + NODE_CATEGORIES
6. Add Neo4j constraints to `neo4j/seed/00-constraints.cypher`
7. Run validations:

```bash
npm run build
npm run validate:schema
npm run validate:relations
```

## TypeScript Types

Models are mirrored in TypeScript at `src/types/`:
- `index.ts` - Core types (Project, Entity, Page, Block, SEO, GEO)
- `locale-knowledge.ts` - Locale Knowledge types
- `prompts.ts` - PagePrompt, BlockPrompt, BlockRules types

## Commands

```bash
# Regenerate view documentation
npm run generate:docs

# Validate schemas
npm run validate:schema
npm run validate:relations
npm run validate:version
```
