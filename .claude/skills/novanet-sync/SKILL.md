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

## Source of Truth (v11.3)

```
packages/core/models/
+-- node-kinds/                   <- 61 YAML files (one per NodeKind)
|   +-- shared/                   <- Realm: shared (32 nodes)
|   |   +-- locale/               <-   Layer: locale (7 nodes)
|   |   +-- geography/            <-   Layer: geography (6 nodes)
|   |   +-- knowledge/            <-   Layer: knowledge (19 nodes)
|   +-- org/                      <- Realm: org (29 nodes)
|       +-- config/               <-   Layer: config (OrgConfig)
|       +-- foundation/           <-   Layer: foundation
|       +-- structure/            <-   Layer: structure
|       +-- semantic/             <-   Layer: semantic (Entity, EntityContent)
|       +-- instruction/          <-   Layer: instruction
|       +-- seo/                  <-   Layer: seo (SEOKeyword)
|       +-- geo/                  <-   Layer: geo (GEOQuery, GEOAnswer, GEOMetrics)
|       +-- output/               <-   Layer: output
+-- arc-kinds/                    <- 125 YAML files (one per ArcKind)
+-- relations.yaml                <- Legacy format (kept for parser compatibility)
+-- taxonomy.yaml                 <- v11.3: 2 Realms, 11 Layers, 5 Traits
```

## Generated Artifacts

`novanet schema generate` produces 12 artifacts:

| Source | Generator | Output |
|--------|-----------|--------|
| taxonomy.yaml | OrganizingGenerator | seed/00.5-taxonomy.cypher |
| node-kinds/ | NodeKindGenerator | seed/01-kinds.cypher |
| arc-kinds/ | ArcKindGenerator | seed/02-arc-kinds.cypher |
| node-kinds/ | LayerGenerator | src/graph/layers.ts |
| models/ | MermaidGenerator | models/docs/complete-graph.md |
| node-kinds/ | AutowireGenerator | seed/99-autowire-kinds.cypher |
| node-kinds/ | HierarchyGenerator | src/graph/hierarchy.ts |
| taxonomy.yaml | ColorsGenerator | apps/studio/src/design/colors/generated.ts |
| visual-encoding.yaml | IconsGenerator | apps/studio/src/design/icons/nodeIcons.generated.ts |
| visual-encoding.yaml | VisualEncodingGenerator | src/graph/visual-encoding.ts |
| views/*.yaml | ViewsGenerator | src/filters/views.generated.ts |
| visual-encoding.yaml | TuiIconsGenerator | tools/novanet/src/tui/icons.rs |

`novanet doc generate` produces 11 view-specific Mermaid diagrams from `models/views/`.

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
`@novanet/schema-tools` has been eliminated - the Rust binary handles all generation.

**LayerGenerator (`generators/layer.rs`):**
- Scans folder structure: `models/node-kinds/{realm}/{layer}/*.yaml`
- Extracts Kind label from filename (kebab-case -> PascalCase)
- Generates `KIND_LAYERS` mapping (Kind -> Layer/Realm/Trait) via Tera template

**MermaidGenerator (`generators/mermaid.rs`):**
- Reads `models/node-kinds/` and `models/arc-kinds/`
- Generates Mermaid flowchart with all 61 Kinds and 125 arcs
- Groups by Realm (Shared, Org)
- Colors by Layer (11 distinct colors)
