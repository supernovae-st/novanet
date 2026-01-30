# NovaNet Models v8.0.0

YAML schema definitions for Neo4j graph database.

> **Graph Diagram**: See [VIEW-COMPLETE-GRAPH.md](docs/views/VIEW-COMPLETE-GRAPH.md) for auto-generated Mermaid diagram.
> Run `npm run generate:docs` to regenerate all view documentation.

## Directory Structure

```
models/
├── _index.yaml            # Graph overview + file index
├── relations.yaml         # All Neo4j relationships (50)
├── README.md              # This file
├── nodes/                 # ONE FILE PER NODE TYPE (35 files)
│   ├── project/           # 📦 Foundation (3)
│   ├── content/           # 💡 Structure + Semantic (5)
│   ├── locale/            # 🌍 Locale + Knowledge (15)
│   ├── generation/        # ⚡ Prompts + Output (5)
│   ├── seo/               # 🔍 SEO nodes (3)
│   └── geo/               # 🎯 GEO nodes (3)
├── views/                 # View definitions (YAML)
│   ├── _registry.yaml
│   └── *.yaml
└── docs/views/            # Auto-generated view docs (MD)
    └── VIEW-*.md
```

## Multi-Tenant Architecture

NovaNet uses a **3-layer scope architecture**:

| Scope | Count | Description | Examples |
|-------|-------|-------------|----------|
| 🌍 **GLOBAL** | 15 | Shared locale knowledge | Locale, LocaleVoice, LocaleCulture, Expression |
| 🎯 **SHARED** | 6 | Reusable targeting data | SEOKeywordL10n, GEOSeedL10n, *Metrics, *MiningRun |
| 📦 **PROJECT** | 14 | Per-project content | Project, Page, Block, Concept, PageL10n, BlockL10n |

## Locale Behavior Classification

| Behavior | Icon | Description | Nodes |
|----------|------|-------------|-------|
| **INVARIANT** | 🔵 | Defined once, language-independent | Project, Concept, Page, Block, Locale |
| **LOCALIZED** | 🟢 | Per-locale, has `:FOR_LOCALE` | ProjectL10n, ConceptL10n, PageL10n, BlockL10n |
| **LOCALE_KNOWLEDGE** | 🟣 | Knowledge ABOUT a locale | LocaleIdentity, LocaleVoice, LocaleCulture, Expression |
| **DERIVED** | ⚪ | Inherits locale from parent | SEOKeywordMetrics, GEOSeedMetrics |
| **JOB** | ⚙️ | Background jobs, no locale | SEOMiningRun, GEOMiningRun |

## Nomenclature v8.0.0

```
*L10n suffix    = ALL localized content (human OR LLM generated)
:HAS_L10N       = human-curated content (ConceptL10n, ProjectL10n)
:HAS_OUTPUT     = LLM-generated content (PageL10n, BlockL10n)
Locale*         = Locale Knowledge nodes (LocaleVoice, LocaleCulture, etc.)
```

## Core Pattern: Invariant -> Localized

```
Invariant (EN)              Localized (generated natively)
──────────────              ────────────────────────────────
Concept.key                 ConceptL10n.title (per locale)
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
| [Concept Network](docs/views/VIEW-CONCEPT-ECOSYSTEM.md) | Localization | Concept graph with semantic links |
| [Spreading Activation](docs/views/VIEW-BLOCK-SEMANTIC-NETWORK.md) | Semantic | Temperature-based semantic traversal |
| [SEO Pipeline](docs/views/VIEW-SEO-PIPELINE.md) | Mining | SEO keyword mining workflow |
| [GEO Pipeline](docs/views/VIEW-GEO-PIPELINE.md) | Mining | GEO answer engine optimization |

## Relationship Properties Reference

| Relationship | Props | Description |
|--------------|-------|-------------|
| `SUPPORTS_LOCALE` | `status` | active \| pending \| disabled |
| `DEFAULT_LOCALE` | (none) | Exactly one per project |
| `HAS_BLOCK` | `position` | Display order |
| `USES_CONCEPT` | `purpose, temperature` | primary/secondary/contextual |
| `SEMANTIC_LINK` | `type, temperature` | Concept relationships |
| `HAS_SEO_TARGET` | `role, priority` | locale-aligned: primary/secondary/long-tail |
| `HAS_GEO_TARGET` | `role, priority` | locale-aligned: primary/contextual |
| `PREVIOUS_VERSION` | (none) | History chain: BlockL10n/PageL10n → previous |
| `LINKS_TO` | `concept_key, context, seo_weight` | Explicit internal link (Page → Page) |
| `SUBTOPIC_OF` | (none) | Pillar-cluster hierarchy (Page → Page) |

## Statistics

| Metric | Count |
|--------|-------|
| **Total Nodes** | 35 |
| **Total Relations** | 50 |
| **Scope Layers** | 3 (Global, Shared, Project) |
| **Locale Behavior Categories** | 5 |
| **Inverse Relations** | 5 |

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
- `index.ts` - Core types (Project, Concept, Page, Block, SEO, GEO)
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
