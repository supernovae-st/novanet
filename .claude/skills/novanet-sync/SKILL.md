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
├── node-kinds/                   ← 45 YAML files (one per NodeKind)
│   ├── global/                   ← Realm: global (19 nodes)
│   │   ├── config/               ←   Layer: config (Locale + utilities)
│   │   ├── locale-knowledge/     ←   Layer: locale-knowledge (v10.5: renamed)
│   │   └── seo/                  ←   Layer: seo
│   ├── organization/             ← Realm: organization (NEW in v10.5)
│   │   ├── config/               ←   Layer: config (Organization node)
│   │   └── semantic/             ←   Layer: semantic (Org-level Entity)
│   └── project/                  ← Realm: project (23 nodes)
│       ├── foundation/           ←   Layer: foundation
│       ├── structure/            ←   Layer: structure
│       ├── semantic/             ←   Layer: semantic
│       ├── instruction/          ←   Layer: instruction
│       └── output/               ←   Layer: output
├── arc-kinds/                    ← 64 YAML files (one per ArcKind)
├── relations.yaml                ← Legacy format (kept for parser compatibility)
└── taxonomy.yaml                 ← v10.5: 3 Realms, 10 Layers, 5 Traits
```

## Generated Artifacts

`novanet schema generate` produces 7 artifacts:

| Source | Generator | Output |
|--------|-----------|--------|
| models/ | OrganizingGenerator | seed/00.5-organizing-principles.cypher |
| node-kinds/ | KindGenerator | seed/99-kind-meta-nodes.cypher |
| arc-kinds/ | ArcSchemaGenerator | seed/99-arc-schema-meta-nodes.cypher |
| node-kinds/ | AutowireGenerator | seed/99-autowire-kinds.cypher |
| node-kinds/ | LayerGenerator | src/graph/layers.ts |
| node-kinds/ | HierarchyGenerator | src/graph/hierarchy.ts |
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
- Scans folder structure: `models/node-kinds/{realm}/{layer}/*.yaml`
- Extracts Kind label from filename (kebab-case -> PascalCase)
- Generates `KIND_LAYERS` mapping (Kind -> Layer/Realm/Trait) via Tera template

**MermaidGenerator (`generators/mermaid.rs`):**
- Reads `models/node-kinds/` and `models/arc-kinds/`
- Generates Mermaid flowchart with all 45 Kinds and 64 arcs
- Groups by Realm (Global, Organization, Project)
- Colors by Layer (10 distinct colors)
