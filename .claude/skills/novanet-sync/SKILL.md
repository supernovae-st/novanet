---
name: novanet-sync
description: Validate and regenerate TypeScript/Mermaid from YAML sources. Use when YAML models change, sync validation fails, or user asks about schema synchronization.
disable-model-invocation: false
user-invocable: true
allowed-tools: Bash
argument-hint: [validate|generate|status]
---

# NovaNet Schema Sync

Synchronize generated artifacts with YAML source of truth.

## Source of Truth

```
packages/core/models/
├── nodes/                        ← 35 YAML files (one per Kind)
│   ├── global/                   ← Realm: global
│   │   ├── config/               ←   Layer: config
│   │   └── knowledge/            ←   Layer: knowledge
│   ├── project/                  ← Realm: project
│   │   ├── foundation/           ←   Layer: foundation
│   │   ├── structure/            ←   Layer: structure
│   │   ├── semantic/             ←   Layer: semantic
│   │   ├── instruction/          ←   Layer: instruction
│   │   └── output/               ←   Layer: output
│   └── shared/                   ← Realm: shared
│       ├── seo/                  ←   Layer: seo
│       └── geo/                  ←   Layer: geo
├── relations.yaml                ← 50 Arc types (→ arc-kinds/ in v9.5)
└── organizing-principles.yaml    ← v9: facet defs (→ taxonomy.yaml in v9.5)
```

## Generated Artifacts

`novanet schema generate` produces 7 artifacts:

| Source | Generator | Output |
|--------|-----------|--------|
| models/ | OrganizingGenerator | seed/00.5-organizing-principles.cypher |
| models/nodes/ | KindGenerator | seed/99-kind-meta-nodes.cypher |
| models/relations.yaml | ArcSchemaGenerator | seed/99-edge-schema-meta-nodes.cypher |
| models/nodes/ | AutowireGenerator | seed/99-autowire-kinds.cypher |
| models/nodes/ | LayerGenerator | src/graph/layers.ts |
| models/nodes/ | HierarchyGenerator | src/graph/hierarchy.ts |
| models/ | MermaidGenerator | models/docs/views/VIEW-COMPLETE-GRAPH.md |

`novanet doc generate` produces 12 view-specific Mermaid diagrams from `models/views/`.

## Commands

Based on `$ARGUMENTS`, execute the appropriate action:

### `validate` or `status` (default)

Check if generated files match YAML sources:

```bash
novanet schema validate
```

Reports errors and warnings about YAML coherence (duplicate keys, missing refs, etc.).

### `generate` or `fix`

Regenerate all artifacts from YAML:

```bash
novanet schema generate
```

Then show git status to see what changed:

```bash
git diff --stat
```

### `all` or `full`

Run both validate and generate:

```bash
novanet schema validate || novanet schema generate
git diff --stat
```

## CI Integration

The GitHub Actions CI has a `schema-sync` job that:
1. Runs `novanet schema validate`
2. Fails the build if any file is out of sync

## Workflow

When you modify YAML models:

1. Edit YAML files in `packages/core/models/`
2. Run `/novanet-sync generate` to regenerate
3. Commit both YAML changes AND generated files
4. CI will verify sync on PR

## Troubleshooting

If CI fails with "SYNC FAILED":

```bash
# Pull latest changes
git pull

# Regenerate from YAML
novanet schema generate

# Review changes
git diff

# Commit regenerated files
git add packages/core/src/graph/layers.ts packages/core/src/graph/hierarchy.ts
git add packages/core/models/docs/views/VIEW-COMPLETE-GRAPH.md
git add packages/db/seed/00.5-* packages/db/seed/99-*
git commit -m "chore: regenerate from YAML sources"
```

## Technical Details

All generators live in `tools/novanet/src/generators/` (Rust-first architecture).
`@novanet/schema-tools` has been eliminated — the Rust binary handles all generation.

**LayerGenerator (`generators/layer.rs`):**
- Scans folder structure: `models/nodes/{realm}/{layer}/*.yaml`
- Extracts Kind label from filename (kebab-case -> PascalCase)
- Generates `KIND_LAYERS` mapping (Kind -> Layer/Realm/Trait) via Tera template

**MermaidGenerator (`generators/mermaid.rs`):**
- Reads `models/nodes/` and `models/relations.yaml`
- Generates Mermaid flowchart with all 44 Kinds and 50 relations
- Groups by Realm (Global, Shared, Project)
- Colors by Layer (9 distinct colors)
