# Schema Sync Validation System

> **Superseded by v9**: `@novanet/schema-tools` is eliminated in v9. All schema validation
> and generation is handled by the single `novanet` Rust binary (`tools/novanet/`).
> See [`2026-02-01-ontology-v9-design.md`](2026-02-01-ontology-v9-design.md).

> Design document for ensuring 100% synchronization between YAML sources and derived artifacts.

**Date**: 2026-01-30
**Status**: Superseded by v9 Rust-first architecture
**Architecture**: ~~Dedicated `packages/schema-tools` package~~ → `tools/novanet/` Rust binary

---

## Problem Statement

The NovaNet ontology has YAML as the **single source of truth**, but derived artifacts (TypeScript, Mermaid, Neo4j) can drift out of sync. We discovered a ~70% sync score in Mermaid diagrams, indicating the validation system was insufficient.

**Root Cause**: Generators exist but weren't integrated into the development workflow.

**Decision**: Create a dedicated `packages/schema-tools` package to cleanly separate build-time tools from runtime code.

---

## Architecture Decision

### Why a Dedicated Package?

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SEPARATION OF CONCERNS                                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  packages/core         = RUNTIME code (types, schemas, filters)                 │
│                          → Used by studio, cli at runtime                       │
│                          → Shipped to production                                │
│                                                                                 │
│  packages/schema-tools = BUILD-TIME code (generators, validators)               │
│                          → Used during development/CI                           │
│                          → Never shipped to production                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

Generators are **build-time tools**, not runtime dependencies. They belong in their own package.

---

## New Monorepo Structure

```
packages/
├── core/                    # Runtime: types, schemas, filters
│   ├── src/
│   │   ├── types/           # TypeScript types
│   │   ├── schemas/         # Zod schemas
│   │   ├── filters/         # NovaNetFilter, CypherGenerator
│   │   └── graph/           # subcategories.ts (GENERATED)
│   └── models/              # YAML source of truth
│       ├── _index.yaml
│       ├── relations.yaml
│       ├── nodes/
│       └── docs/views/      # VIEW-COMPLETE-GRAPH.md (GENERATED)
│
├── schema-tools/            # NEW: Build-time generators & validators
│   ├── src/
│   │   ├── generators/      # MermaidGenerator, SubcategoryGenerator, etc.
│   │   ├── validators/      # Sync validators
│   │   ├── parsers/         # RelationsParser, ViewParser
│   │   └── index.ts
│   ├── scripts/
│   │   ├── generate-all.ts
│   │   └── validate-sync.ts
│   ├── package.json
│   └── tsconfig.json
│
├── db/                      # Neo4j infrastructure
└── cli/                     # Developer CLI tools
```

---

## Migration Plan

### Files to Move from core → schema-tools

| Current Location | New Location |
|------------------|--------------|
| `core/src/generators/MermaidGenerator.ts` | `schema-tools/src/generators/` |
| `core/src/generators/SubcategoryGenerator.ts` | `schema-tools/src/generators/` |
| `core/src/generators/MarkdownGenerator.ts` | `schema-tools/src/generators/` |
| `core/src/generators/CypherExporter.ts` | `schema-tools/src/generators/` |
| `core/src/generators/RelationsParser.ts` | `schema-tools/src/parsers/` |
| `core/src/generators/ViewParser.ts` | `schema-tools/src/parsers/` |
| `core/src/generators/colors.ts` | `schema-tools/src/config/` |
| `core/src/generators/schemas.ts` | `schema-tools/src/schemas/` |
| `core/src/generators/types.ts` | `schema-tools/src/types/` |
| `core/scripts/generate-subcategories.ts` | `schema-tools/scripts/` |
| `core/scripts/validate-subcategories.ts` | `schema-tools/scripts/` |

### Files to Keep in core

| File | Reason |
|------|--------|
| `core/src/types/` | Runtime types used by studio |
| `core/src/schemas/` | Runtime Zod schemas |
| `core/src/filters/` | Runtime filters |
| `core/src/graph/subcategories.ts` | Generated, but used at runtime |
| `core/models/` | YAML source of truth |

---

## Implementation Tasks

### Task 1: Create packages/schema-tools structure

```bash
mkdir -p packages/schema-tools/src/{generators,validators,parsers,config,types}
mkdir -p packages/schema-tools/scripts
```

### Task 2: Create package.json

**File**: `packages/schema-tools/package.json`

```json
{
  "name": "@novanet/schema-tools",
  "version": "1.0.0",
  "description": "Build-time generators and validators for NovaNet schema",
  "type": "module",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "generate:all": "tsx scripts/generate-all.ts",
    "generate:mermaid": "tsx scripts/generate-mermaid.ts",
    "generate:subcategories": "tsx scripts/generate-subcategories.ts",
    "validate:sync": "tsx scripts/validate-sync.ts",
    "test": "jest"
  },
  "dependencies": {
    "yaml": "^2.3.4",
    "zod": "^3.22.4"
  },
  "devDependencies": {
    "@types/node": "^20.10.0",
    "tsx": "^4.7.0",
    "typescript": "^5.9.0"
  }
}
```

### Task 3: Create tsconfig.json

**File**: `packages/schema-tools/tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "declaration": true,
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "resolveJsonModule": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### Task 4: Move generators and update imports

Move files and update import paths. Key changes:

```typescript
// OLD (in core)
import type { Scope } from '../types/nodes.js';

// NEW (in schema-tools)
import type { Scope } from '@novanet/core/types';
```

### Task 5: Create generate-all.ts

**File**: `packages/schema-tools/scripts/generate-all.ts`

```typescript
#!/usr/bin/env tsx
import { MermaidGenerator } from '../src/generators/MermaidGenerator.js';
import { SubcategoryGenerator } from '../src/generators/SubcategoryGenerator.js';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const CORE_DIR = path.join(__dirname, '../../core');
const MODELS_DIR = path.join(CORE_DIR, 'models');

async function main() {
  console.log('═══════════════════════════════════════════════════════');
  console.log('  @novanet/schema-tools - Generate All Artifacts');
  console.log('═══════════════════════════════════════════════════════');

  // 1. Subcategories
  console.log('\n[1/2] Generating subcategories.ts...');
  await SubcategoryGenerator.writeToFile({
    modelsDir: path.join(MODELS_DIR, 'nodes'),
    outputPath: path.join(CORE_DIR, 'src/graph/subcategories.ts'),
  });
  console.log('✅ packages/core/src/graph/subcategories.ts');

  // 2. Mermaid diagram
  console.log('\n[2/2] Generating VIEW-COMPLETE-GRAPH.md...');
  await MermaidGenerator.writeToFile({
    modelsDir: MODELS_DIR,
    outputPath: path.join(MODELS_DIR, 'docs/views/VIEW-COMPLETE-GRAPH.md'),
  });
  console.log('✅ packages/core/models/docs/views/VIEW-COMPLETE-GRAPH.md');

  console.log('\n═══════════════════════════════════════════════════════');
  console.log('  ✅ All artifacts generated successfully');
  console.log('═══════════════════════════════════════════════════════');
}

main().catch((err) => {
  console.error('❌ Generation failed:', err);
  process.exit(1);
});
```

### Task 6: Create validate-sync.ts

**File**: `packages/schema-tools/scripts/validate-sync.ts`

```typescript
#!/usr/bin/env tsx
import { MermaidGenerator } from '../src/generators/MermaidGenerator.js';
import { SubcategoryGenerator } from '../src/generators/SubcategoryGenerator.js';
import * as fs from 'fs/promises';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const CORE_DIR = path.join(__dirname, '../../core');
const MODELS_DIR = path.join(CORE_DIR, 'models');

interface ValidationResult {
  file: string;
  synced: boolean;
  diff?: string;
}

async function validateFile(
  name: string,
  generator: () => Promise<string>,
  committedPath: string
): Promise<ValidationResult> {
  try {
    const generated = await generator();
    const committed = await fs.readFile(committedPath, 'utf-8');

    if (generated.trim() === committed.trim()) {
      return { file: name, synced: true };
    }

    return {
      file: name,
      synced: false,
      diff: `Content differs from committed file`,
    };
  } catch (err) {
    return {
      file: name,
      synced: false,
      diff: `Error: ${err instanceof Error ? err.message : String(err)}`,
    };
  }
}

async function main() {
  console.log('═══════════════════════════════════════════════════════');
  console.log('  @novanet/schema-tools - Validate Sync');
  console.log('═══════════════════════════════════════════════════════\n');

  const results: ValidationResult[] = [];

  // Validate subcategories.ts
  results.push(
    await validateFile(
      'subcategories.ts',
      () => SubcategoryGenerator.generate({
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      }),
      path.join(CORE_DIR, 'src/graph/subcategories.ts')
    )
  );

  // Validate Mermaid
  results.push(
    await validateFile(
      'VIEW-COMPLETE-GRAPH.md',
      () => MermaidGenerator.generate({ modelsDir: MODELS_DIR }),
      path.join(MODELS_DIR, 'docs/views/VIEW-COMPLETE-GRAPH.md')
    )
  );

  // Report
  let allSynced = true;
  for (const result of results) {
    const status = result.synced ? '✅' : '❌';
    console.log(`${status} ${result.file}`);
    if (!result.synced) {
      allSynced = false;
      console.log(`   └── ${result.diff}`);
    }
  }

  console.log('');

  if (!allSynced) {
    console.log('═══════════════════════════════════════════════════════');
    console.log('  ❌ SYNC FAILED');
    console.log('  Run: pnpm --filter @novanet/schema-tools generate:all');
    console.log('═══════════════════════════════════════════════════════');
    process.exit(1);
  }

  console.log('═══════════════════════════════════════════════════════');
  console.log('  ✅ All files in sync with YAML sources');
  console.log('═══════════════════════════════════════════════════════');
}

main().catch((err) => {
  console.error('❌ Validation error:', err);
  process.exit(1);
});
```

### Task 7: Add root-level npm scripts

**File**: `package.json` (monorepo root)

```json
{
  "scripts": {
    "schema:generate": "pnpm --filter @novanet/schema-tools generate:all",
    "schema:validate": "pnpm --filter @novanet/schema-tools validate:sync"
  }
}
```

### Task 8: Configure pre-commit hook

**File**: `.husky/pre-commit`

```bash
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

# Check if YAML sources changed
if git diff --cached --name-only | grep -q "packages/core/models/"; then
  echo "📋 YAML models changed - validating sync..."
  pnpm schema:validate

  if [ $? -ne 0 ]; then
    echo ""
    echo "❌ Schema sync validation failed!"
    echo "   Run: pnpm schema:generate"
    echo "   Then: git add the generated files"
    exit 1
  fi
fi
```

### Task 9: Add CI job

**File**: `.github/workflows/ci.yml`

```yaml
  schema-sync:
    name: Schema Sync Validation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v2
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'
      - run: pnpm install
      - run: pnpm schema:validate
```

### Task 10: Create sync tests

**File**: `packages/schema-tools/src/__tests__/sync.test.ts`

```typescript
import { MermaidGenerator } from '../generators/MermaidGenerator';
import { SubcategoryGenerator } from '../generators/SubcategoryGenerator';
import * as fs from 'fs/promises';
import * as path from 'path';

const CORE_DIR = path.join(__dirname, '../../../core');
const MODELS_DIR = path.join(CORE_DIR, 'models');

describe('YAML → Artifact Synchronization', () => {
  describe('Mermaid Diagram', () => {
    it('VIEW-COMPLETE-GRAPH.md matches generated content', async () => {
      const generated = await MermaidGenerator.generate({
        modelsDir: MODELS_DIR,
      });

      const committed = await fs.readFile(
        path.join(MODELS_DIR, 'docs/views/VIEW-COMPLETE-GRAPH.md'),
        'utf-8'
      );

      expect(generated.trim()).toBe(committed.trim());
    });
  });

  describe('Subcategories', () => {
    it('subcategories.ts matches generated content', async () => {
      const generated = await SubcategoryGenerator.generate({
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      });

      const committed = await fs.readFile(
        path.join(CORE_DIR, 'src/graph/subcategories.ts'),
        'utf-8'
      );

      expect(generated.trim()).toBe(committed.trim());
    });
  });
});
```

---

## Workflow Summary

### Developer Workflow

```
1. Edit YAML sources (packages/core/models/)
2. Run: pnpm schema:generate
3. Commit changes (YAML + generated files)
4. Pre-commit hook validates sync
5. CI validates sync on push
```

### Command Reference

| Command | Description |
|---------|-------------|
| `pnpm schema:generate` | Regenerate all artifacts from YAML |
| `pnpm schema:validate` | Check if artifacts are in sync |
| `pnpm --filter @novanet/schema-tools test` | Run sync tests |

### Validation Layers

| Layer | Trigger | Action |
|-------|---------|--------|
| **Local** | `pnpm schema:generate` | Regenerate artifacts |
| **Pre-commit** | `git commit` | Validate sync, block if out of sync |
| **CI** | `git push` | Validate sync, fail PR if out of sync |
| **Tests** | `pnpm test` | sync.test.ts |

---

## Benefits

1. **Clean Architecture**: Build-time tools separate from runtime code
2. **Single Responsibility**: schema-tools does one thing well
3. **Monorepo Integration**: Standard pnpm filter commands
4. **Automated Validation**: Pre-commit + CI catch drift
5. **Evolutionary Tests**: Tests read YAML, evolve with ontology

---

## Implementation Checklist

- [ ] Create `packages/schema-tools/` directory structure
- [ ] Create `package.json` and `tsconfig.json`
- [ ] Move generators from `core/src/generators/`
- [ ] Move parsers (RelationsParser, ViewParser)
- [ ] Update imports to use `@novanet/core/types`
- [ ] Create `scripts/generate-all.ts`
- [ ] Create `scripts/validate-sync.ts`
- [ ] Add root-level npm scripts
- [ ] Configure `.husky/pre-commit`
- [ ] Add CI job to `.github/workflows/ci.yml`
- [ ] Create sync tests
- [ ] Update `core/src/generators/index.ts` (remove moved exports)
- [ ] Test full workflow
