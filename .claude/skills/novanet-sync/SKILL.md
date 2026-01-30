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
├── nodes/           ← 35 YAML files (NodeTypes)
│   ├── global/
│   ├── project/
│   └── shared/
└── relations.yaml   ← 47 relations
```

## Generated Artifacts

| Source | Generator | Output |
|--------|-----------|--------|
| models/nodes/ | SubcategoryGenerator | src/graph/subcategories.ts |
| models/ | MermaidGenerator | models/docs/views/VIEW-COMPLETE-GRAPH.md |

## Commands

Based on `$ARGUMENTS`, execute the appropriate action:

### `validate` or `status` (default)

Check if generated files match YAML sources:

```bash
pnpm schema:validate
```

Expected output:
```
═══════════════════════════════════════════════════════
  @novanet/schema-tools - Validate Sync
═══════════════════════════════════════════════════════
Results:
  ✅ subcategories.ts
  ✅ VIEW-COMPLETE-GRAPH.md

  ✅ All files in sync with YAML sources
═══════════════════════════════════════════════════════
```

### `generate` or `fix`

Regenerate all artifacts from YAML:

```bash
pnpm schema:generate
```

Then show git status to see what changed:

```bash
git diff --stat
```

### `all` or `full`

Run both validate and generate:

```bash
pnpm schema:validate || pnpm schema:generate
git diff --stat
```

## CI Integration

The GitHub Actions CI has a `schema-sync` job that:
1. Runs `pnpm schema:validate`
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
pnpm schema:generate

# Review changes
git diff

# Commit regenerated files
git add packages/core/src/graph/subcategories.ts
git add packages/core/models/docs/views/VIEW-COMPLETE-GRAPH.md
git commit -m "chore: regenerate from YAML sources"
```

## Technical Details

**SubcategoryGenerator:**
- Scans folder structure: `models/nodes/{scope}/{subcategory}/*.yaml`
- Extracts NodeType from filename (kebab-case → PascalCase)
- Generates `NODE_SUBCATEGORIES` mapping

**MermaidGenerator:**
- Reads `models/nodes/` and `models/relations.yaml`
- Generates Mermaid flowchart with all 35 nodes and 47 relations
- Groups by scope (Global, Shared, Project)
