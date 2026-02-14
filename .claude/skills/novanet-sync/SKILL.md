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

## Source of Truth (v12.0)

```
packages/core/models/
+-- node-classes/                   <- 59 YAML files (one per NodeClass)
|   +-- shared/                   <- Realm: shared (40 nodes)
|   |   +-- config/               <-   Layer: config (EntityCategory, Locale, SEOKeywordFormat)
|   |   +-- locale/               <-   Layer: locale (6 nodes)
|   |   +-- geography/            <-   Layer: geography (6 nodes)
|   |   +-- knowledge/            <-   Layer: knowledge (24 nodes, incl. SEO/GEO)
|   +-- org/                      <- Realm: org (20 nodes)
|       +-- config/               <-   Layer: config (OrgConfig)
|       +-- foundation/           <-   Layer: foundation (3 nodes)
|       +-- structure/            <-   Layer: structure (3 nodes)
|       +-- semantic/             <-   Layer: semantic (Entity, EntityContent, etc.)
|       +-- instruction/          <-   Layer: instruction (7 nodes)
|       +-- output/               <-   Layer: output (3 nodes)
+-- arc-classes/                    <- 114 YAML files (one per ArcClass)
+-- relations.yaml                <- Legacy format (kept for parser compatibility)
+-- taxonomy.yaml                 <- v12.0: 2 Realms, 10 Layers, 5 Traits (defined/authored/imported/generated/retrieved)
```

## Generated Artifacts

`novanet schema generate` produces 12 artifacts:

| Source | Generator | Output |
|--------|-----------|--------|
| taxonomy.yaml | OrganizingGenerator | seed/00.5-taxonomy.cypher |
| node-classes/ | NodeClassGenerator | seed/01-classes.cypher |
| arc-classes/ | ArcClassGenerator | seed/02-arc-classes.cypher |
| node-classes/ | LayerGenerator | src/graph/layers.ts |
| models/ | MermaidGenerator | models/docs/complete-graph.md |
| node-classes/ | AutowireGenerator | seed/99-autowire-classes.cypher |
| node-classes/ | HierarchyGenerator | src/graph/hierarchy.ts |
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
- Scans folder structure: `models/node-classes/{realm}/{layer}/*.yaml`
- Extracts Kind label from filename (kebab-case -> PascalCase)
- Generates `KIND_LAYERS` mapping (Kind -> Layer/Realm/Trait) via Tera template

**MermaidGenerator (`generators/mermaid.rs`):**
- Reads `models/node-classes/` and `models/arc-classes/`
- Generates Mermaid flowchart with all 59 Classes and 114 arcs
- Groups by Realm (Shared, Org)
- Colors by Layer (10 distinct colors)
