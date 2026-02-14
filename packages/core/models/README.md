# NovaNet Models v0.12.0 "Class Act"

YAML schema definitions for Neo4j graph database.

> **Graph Diagram**: See [complete-graph.md](docs/complete-graph.md) for auto-generated Mermaid diagram.
> **Rust CLI**: Run `cd tools/novanet && cargo run -- doc generate` to regenerate all view documentation.

## Directory Structure

```
models/
├── _index.yaml              # Graph overview + file index
├── taxonomy.yaml            # Realm/Layer/Trait definitions (colors, icons)
├── visual-encoding.yaml     # Visual channel mappings
├── README.md                # This file
├── node-classes/              # ONE FILE PER NODE CLASS (59 files)
│   ├── shared/              # ◉ SHARED realm (39 nodes)
│   │   ├── config/          #    Locale, EntityCategory, OrgConfig (3)
│   │   ├── locale/          #    LocaleVoice, LocaleCulture, LocaleStyle... (6)
│   │   ├── geography/       #    Region, Country, City... (6)
│   │   └── knowledge/       #    Terms, Expressions, SEO, GEO... (24)
│   └── org/                 # ◎ ORG realm (20 nodes)
│       ├── config/          #    Organization (1)
│       ├── foundation/      #    Project, ProjectContent, BrandIdentity (3)
│       ├── structure/       #    Page, Block, ContentSlot (3)
│       ├── semantic/        #    Entity, EntityContent, Audience, Channel (4)
│       ├── instruction/     #    PageStructure, PageInstruction, BlockType, BlockInstruction... (6)
│       └── output/          #    PageGenerated, BlockGenerated, OutputArtifact (3)
├── arc-classes/               # ONE FILE PER ARC CLASS (114 arcs)
│   ├── ownership/           # HAS_* arcs (43)
│   ├── localization/        # FOR_LOCALE, SUPPORTS_LOCALE (14)
│   ├── semantic/            # USES_*, SEMANTIC_LINK (26)
│   ├── generation/          # HAS_GENERATED, GENERATED_BY (18)
│   └── mining/              # SEO/GEO targeting arcs (13)
├── views/                   # View definitions (YAML)
│   ├── _registry.yaml       # View registry
│   └── *.yaml               # Individual views
└── docs/                    # Auto-generated view docs (Mermaid)
    └── *.md
```

## 2-Realm Architecture (v0.12.0)

| Realm | Count | Description | Layers |
|-------|-------|-------------|--------|
| ◉ **SHARED** | 39 | Universal knowledge (READ-ONLY) | config(3), locale(6), geography(6), knowledge(24) |
| ◎ **ORG** | 20 | Organization-specific content | config(1), foundation(3), structure(3), semantic(4), instruction(6), output(3) |

## Data Origin Traits (ADR-024)

Trait answers: **"WHERE does the data come from?"**

| Trait | Icon | Description | Count |
|-------|------|-------------|-------|
| **defined** | ■ | Human-created once (templates, configs) | 31 |
| **authored** | □ | Human-written per locale (editorial) | 2 |
| **imported** | ◇ | External data brought in (corpora, SEO) | 22 |
| **generated** | ★ | Produced by NovaNet LLM | 4 |
| **retrieved** | ⋆ | Fetched from external APIs (GEO) | 3 |

## Nomenclature v0.12.0

```
*Content suffix  = Authored content for defined nodes (EntityContent, ProjectContent)
*Generated       = LLM-generated output (PageGenerated, BlockGenerated)
*Structure       = JSON defining composition (PageStructure)
*Instruction     = Markdown with @refs for LLM (PageInstruction, BlockInstruction)
*Set            = Container nodes (TermSet, ExpressionSet, etc.)
Atoms           = Granular knowledge (Term, Expression, Pattern, etc.)
*Metrics        = External API snapshots (SEOKeywordMetrics, GEOMetrics)
```

## Core Pattern: Defined → Authored/Generated

```
Defined (structure)         Authored/Generated (content)
──────────────────          ─────────────────────────────
Entity.key                  EntityContent.title (per locale)
Page.key                    PageGenerated.assembled (per locale)
Block.key                   BlockGenerated.generated (per locale)
PageStructure.blocks        PageInstruction.markdown (@refs)
```

## Instruction Layer (ADR-025)

```
Page
├── [:HAS_STRUCTURE] → PageStructure (JSON: block order)
├── [:HAS_INSTRUCTION] → PageInstruction (Markdown compiled from BlockInstructions)
└── [:HAS_BLOCK {order: N}] → Block
    ├── [:OF_TYPE] → BlockType (JSON schema)
    └── [:HAS_INSTRUCTION] → BlockInstruction (Markdown with @entity refs)
```

## Arc Families (114 arcs)

| Family | Count | Description |
|--------|-------|-------------|
| **ownership** | 43 | HAS_* parent-child relationships |
| **localization** | 14 | FOR_LOCALE, SUPPORTS_LOCALE |
| **semantic** | 26 | USES_*, SEMANTIC_LINK |
| **generation** | 18 | HAS_GENERATED, GENERATED_BY |
| **mining** | 13 | SEO/GEO targeting |

## Statistics

| Metric | Count |
|--------|-------|
| **Total Node Classes** | 59 |
| **Total Arc Classes** | 114 |
| **Realms** | 2 (shared, org) |
| **Layers** | 10 (4 shared + 6 org) |
| **Traits** | 5 (defined, authored, imported, generated, retrieved) |

## Adding New Nodes

Use the Claude Code skill:

```bash
/schema:add-node MyNewNode
```

This launches a Socratic discovery workflow that:
1. Determines realm (shared/org)
2. Determines layer (10 options)
3. Determines trait (5 options)
4. Creates YAML definition
5. Regenerates TypeScript types

Or manually:

1. Create `models/node-classes/{realm}/{layer}/{node-name}.yaml`
2. Add to `models/_index.yaml` files list
3. Run `cd tools/novanet && cargo run -- schema generate`
4. Run `cd tools/novanet && cargo run -- schema validate`

## Commands

```bash
# Regenerate all artifacts (TypeScript, Cypher, Mermaid)
cd tools/novanet && cargo run -- schema generate

# Validate YAML coherence
cd tools/novanet && cargo run -- schema validate

# Regenerate view documentation
cd tools/novanet && cargo run -- doc generate

# List available views
cd tools/novanet && cargo run -- doc generate --list
```

## References

- **ADR-023**: Class/Instance Terminology (Kind→Class, Meta→Schema)
- **ADR-024**: Trait as Data Origin (defined/authored/imported/generated/retrieved)
- **ADR-025**: Instruction Layer (PageStructure, PageInstruction, BlockInstruction)

See `.claude/rules/novanet-decisions.md` for full ADR documentation.
