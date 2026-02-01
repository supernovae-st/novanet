# Organizing Principles Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add Scope, Subcategory, and NodeTypeMeta as first-class Neo4j nodes for LLM context, queryable taxonomy, and self-describing graph.

**Architecture:** Create `organizing-principles.yaml` as single source of truth, add `OrganizingPrinciplesGenerator` to generate Cypher seed, add constraints, update sync tests.

**Tech Stack:** TypeScript, YAML, Neo4j Cypher, Vitest

---

## Phase 1: Foundation

### Task 1: Create organizing-principles.yaml

**Files:**
- Create: `packages/core/models/organizing-principles.yaml`

**Step 1: Create the YAML file**

```yaml
# packages/core/models/organizing-principles.yaml
# Source of truth for NovaNet taxonomy structure
# v8.3.0 - Organizing Principles as Neo4j nodes
#
# This file defines:
# - 3 Scopes (Global, Project, Shared)
# - 9 Subcategories
# - Metadata for each level (llm_context, emoji, color)
#
# Generated artifacts:
# - packages/db/seed/00.5-organizing-principles.cypher
# - Optionally: hierarchy.ts (future)

version: "8.3.0"

scopes:
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"
    llm_context: |
      Shared across ALL projects. Locale-specific knowledge that applies
      universally: cultural norms, linguistic patterns, voice guidelines,
      idiomatic expressions. These nodes are READ-ONLY at project level.
      Changes here affect all projects using that locale.
    subcategories:
      - key: config
        display_name: Configuration
        emoji: "⚙️"
        llm_context: |
          Core configuration nodes. Locale definitions with their properties
          (language code, region, writing direction). Entry point for all
          locale-specific knowledge traversal.

      - key: knowledge
        display_name: Locale Knowledge
        emoji: "📚"
        llm_context: |
          Deep locale-specific knowledge for native content generation.
          Cultural norms, linguistic patterns, voice guidelines, idiomatic
          expressions, formatting conventions. This is what makes generated
          content feel NATIVE rather than translated.

  - key: project
    display_name: Project
    emoji: "📦"
    color: "#6c71c4"
    llm_context: |
      Business-specific nodes for a single project. Contains brand identity,
      page structure, semantic concepts, generation prompts, and localized
      outputs. These nodes define WHAT content to generate and HOW to
      structure it for this specific product/service.
    subcategories:
      - key: foundation
        display_name: Foundation
        emoji: "🏛️"
        llm_context: |
          Core project identity. Brand voice, visual identity, value
          proposition. These nodes anchor ALL content generation for the
          project - every generated block must align with foundation.

      - key: structure
        display_name: Structure
        emoji: "🏗️"
        llm_context: |
          Information architecture. Pages, blocks, and their types. Defines
          the SKELETON of the website - what pages exist, what blocks compose
          each page, and the rules for each block type.

      - key: semantic
        display_name: Semantic Layer
        emoji: "💡"
        llm_context: |
          Meaning and concepts. Invariant ideas (Concept) that get localized
          per locale (ConceptL10n). The WHAT of content - pricing tiers,
          features, benefits, use cases. Concepts link via SEMANTIC_LINK
          for spreading activation during generation.

      - key: instruction
        display_name: Instructions
        emoji: "📝"
        llm_context: |
          Generation directives. Prompts and rules that guide the LLM during
          content generation. PagePrompt for page-level guidance, BlockPrompt
          for block-specific instructions, BlockRules for constraints.

      - key: output
        display_name: Generated Output
        emoji: "✨"
        llm_context: |
          LLM-generated content. The final localized pages and blocks ready
          for rendering. These are the RESULTS of the generation pipeline -
          created by combining foundation, structure, semantic, and
          instruction nodes with locale knowledge.

  - key: shared
    display_name: Shared
    emoji: "🎯"
    color: "#cb4b16"
    llm_context: |
      Cross-project resources that can be linked to multiple projects.
      SEO keywords and GEO seeds with their metrics. These enable
      competitive intelligence sharing across the portfolio.
    subcategories:
      - key: seo
        display_name: SEO Intelligence
        emoji: "🔍"
        llm_context: |
          Search engine optimization data. Keywords with their localized
          forms, search volume metrics, and mining run history. Used to
          inject relevant keywords into generated content for organic
          search visibility.

      - key: geo
        display_name: GEO Intelligence
        emoji: "📍"
        llm_context: |
          Geographic/local SEO data. Location-based seeds with their
          localized forms, metrics, and mining history. Used for local
          business visibility and location-specific content generation.
```

**Step 2: Verify file was created**

Run: `cat packages/core/models/organizing-principles.yaml | head -20`
Expected: YAML header with version 8.3.0

**Step 3: Commit**

```bash
git add packages/core/models/organizing-principles.yaml
git commit -m "feat(core): add organizing-principles.yaml as source of truth

- Define 3 scopes with llm_context, emoji, color
- Define 9 subcategories with llm_context
- v8.3.0 taxonomy structure for Neo4j nodes"
```

---

### Task 2: Add Neo4j constraints for organizing principles

**Files:**
- Modify: `packages/db/seed/00-constraints.cypher`

**Step 1: Add organizing principles section**

Add after line 105 (after existing constraints):

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// ORGANIZING PRINCIPLES (v8.3.0)
// Meta-schema nodes that describe the taxonomy structure
// ═══════════════════════════════════════════════════════════════════════════════

CREATE CONSTRAINT scope_key IF NOT EXISTS FOR (s:Scope) REQUIRE s.key IS UNIQUE;
CREATE CONSTRAINT subcategory_key IF NOT EXISTS FOR (sub:Subcategory) REQUIRE sub.key IS UNIQUE;
CREATE CONSTRAINT nodetypemeta_label IF NOT EXISTS FOR (ntm:NodeTypeMeta) REQUIRE ntm.label IS UNIQUE;
```

**Step 2: Verify constraints compile**

Run: `docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword "SHOW CONSTRAINTS" | head -20`
Expected: No syntax errors (constraints may not exist yet)

**Step 3: Commit**

```bash
git add packages/db/seed/00-constraints.cypher
git commit -m "feat(db): add constraints for Scope, Subcategory, NodeTypeMeta

- scope_key: unique key for Scope nodes
- subcategory_key: unique key for Subcategory nodes
- nodetypemeta_label: unique label for NodeTypeMeta nodes
- v8.3.0 organizing principles"
```

---

### Task 3: Create OrganizingPrinciplesGenerator

**Files:**
- Create: `packages/schema-tools/src/generators/OrganizingPrinciplesGenerator.ts`

**Step 1: Write the failing test**

Create: `packages/schema-tools/src/__tests__/organizing-principles.test.ts`

```typescript
// packages/schema-tools/src/__tests__/organizing-principles.test.ts
// Tests for OrganizingPrinciplesGenerator
// v1.0.0

import { describe, it, expect } from 'vitest';
import * as path from 'path';
import { fileURLToPath } from 'url';
import { OrganizingPrinciplesGenerator } from '../generators/OrganizingPrinciplesGenerator.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const SCHEMA_TOOLS_ROOT = path.join(__dirname, '../..');
const CORE_DIR = path.join(SCHEMA_TOOLS_ROOT, '../core');
const MODELS_DIR = path.join(CORE_DIR, 'models');

describe('OrganizingPrinciplesGenerator', () => {
  it('generates valid Cypher with MERGE statements', async () => {
    const cypher = await OrganizingPrinciplesGenerator.generate({
      organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
      modelsDir: path.join(MODELS_DIR, 'nodes'),
    });

    // Basic structure checks
    expect(cypher).toContain('// Organizing Principles Seed');
    expect(cypher).toContain('MERGE (s:Scope {key:');
    expect(cypher).toContain('MERGE (sub:Subcategory {key:');
    expect(cypher).toContain('MERGE (t:NodeTypeMeta {label:');
    expect(cypher).toContain(':HAS_SUBCATEGORY');
    expect(cypher).toContain(':DEFINES_TYPE');
  });

  it('includes all 3 scopes', async () => {
    const cypher = await OrganizingPrinciplesGenerator.generate({
      organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
      modelsDir: path.join(MODELS_DIR, 'nodes'),
    });

    expect(cypher).toContain("key: 'global'");
    expect(cypher).toContain("key: 'project'");
    expect(cypher).toContain("key: 'shared'");
  });

  it('includes all 9 subcategories', async () => {
    const cypher = await OrganizingPrinciplesGenerator.generate({
      organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
      modelsDir: path.join(MODELS_DIR, 'nodes'),
    });

    const subcategories = ['config', 'knowledge', 'foundation', 'structure',
                          'semantic', 'instruction', 'output', 'seo', 'geo'];
    for (const sub of subcategories) {
      expect(cypher).toContain(`key: '${sub}'`);
    }
  });

  it('throws on missing organizing-principles.yaml', async () => {
    await expect(
      OrganizingPrinciplesGenerator.generate({
        organizingPrinciplesPath: '/nonexistent/path.yaml',
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      })
    ).rejects.toThrow();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `pnpm --filter @novanet/schema-tools test organizing-principles`
Expected: FAIL with "Cannot find module"

**Step 3: Implement the generator**

Create: `packages/schema-tools/src/generators/OrganizingPrinciplesGenerator.ts`

```typescript
// packages/schema-tools/src/generators/OrganizingPrinciplesGenerator.ts
// Generates Cypher seed for organizing principles from YAML
// v1.0.0 - Source of truth: organizing-principles.yaml

import * as fs from 'fs/promises';
import * as path from 'path';
import { parse as parseYaml } from 'yaml';
import { filePathToNodeName } from '../utils/filePathToNodeName.js';

// =============================================================================
// TYPES
// =============================================================================

interface SubcategoryDef {
  key: string;
  display_name: string;
  emoji: string;
  llm_context: string;
}

interface ScopeDef {
  key: string;
  display_name: string;
  emoji: string;
  color: string;
  llm_context: string;
  subcategories: SubcategoryDef[];
}

interface OrganizingPrinciplesYaml {
  version: string;
  scopes: ScopeDef[];
}

export interface OrganizingPrinciplesGeneratorConfig {
  /** Path to organizing-principles.yaml */
  organizingPrinciplesPath: string;
  /** Path to models/nodes/ for scanning node types */
  modelsDir: string;
}

// =============================================================================
// GENERATOR
// =============================================================================

/**
 * Generates Cypher seed file for organizing principles.
 *
 * Creates:
 * - Scope nodes (3)
 * - Subcategory nodes (9)
 * - NodeTypeMeta nodes (35)
 * - HAS_SUBCATEGORY relationships
 * - DEFINES_TYPE relationships
 */
export class OrganizingPrinciplesGenerator {
  /**
   * Generate Cypher seed content
   */
  static async generate(config: OrganizingPrinciplesGeneratorConfig): Promise<string> {
    // Load organizing-principles.yaml
    const yamlContent = await fs.readFile(config.organizingPrinciplesPath, 'utf-8');
    const data = parseYaml(yamlContent) as OrganizingPrinciplesYaml;

    // Scan folder structure for node types per subcategory
    const nodeTypesBySubcategory = await this.scanNodeTypes(config.modelsDir);

    // Generate Cypher
    return this.generateCypher(data, nodeTypesBySubcategory);
  }

  /**
   * Scan models/nodes/ to get node types per subcategory
   */
  private static async scanNodeTypes(
    modelsDir: string
  ): Promise<Map<string, string[]>> {
    const result = new Map<string, string[]>();

    const scopeFolders = await fs.readdir(modelsDir, { withFileTypes: true });
    for (const scopeFolder of scopeFolders) {
      if (!scopeFolder.isDirectory()) continue;

      const scopePath = path.join(modelsDir, scopeFolder.name);
      const subcategoryFolders = await fs.readdir(scopePath, { withFileTypes: true });

      for (const subcategoryFolder of subcategoryFolders) {
        if (!subcategoryFolder.isDirectory()) continue;

        const subcategory = subcategoryFolder.name;
        const subcategoryPath = path.join(scopePath, subcategoryFolder.name);
        const yamlFiles = await fs.readdir(subcategoryPath);

        const nodeTypes: string[] = [];
        for (const filename of yamlFiles) {
          if (!filename.endsWith('.yaml')) continue;
          nodeTypes.push(filePathToNodeName(filename));
        }

        if (nodeTypes.length > 0) {
          result.set(subcategory, nodeTypes.sort());
        }
      }
    }

    return result;
  }

  /**
   * Generate Cypher content
   */
  private static generateCypher(
    data: OrganizingPrinciplesYaml,
    nodeTypesBySubcategory: Map<string, string[]>
  ): string {
    const lines: string[] = [];
    const timestamp = new Date().toISOString().split('T')[0];

    // Header
    lines.push('// Organizing Principles Seed v8.3.0');
    lines.push('// AUTO-GENERATED from organizing-principles.yaml');
    lines.push(`// Generated: ${timestamp}`);
    lines.push('// Run: pnpm schema:generate');
    lines.push('//');
    lines.push('// Creates: Scope, Subcategory, NodeTypeMeta nodes');
    lines.push('// Uses MERGE for idempotent execution');
    lines.push('');

    // Create Scopes
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('// SCOPES (3)');
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('');

    for (const scope of data.scopes) {
      const llmContext = this.escapeCypher(scope.llm_context.trim());
      lines.push(`MERGE (s_${scope.key}:Scope {key: '${scope.key}'})`);
      lines.push('ON CREATE SET');
      lines.push(`  s_${scope.key}.display_name = '${scope.display_name}',`);
      lines.push(`  s_${scope.key}.emoji = '${scope.emoji}',`);
      lines.push(`  s_${scope.key}.color = '${scope.color}',`);
      lines.push(`  s_${scope.key}.llm_context = '${llmContext}',`);
      lines.push(`  s_${scope.key}.created_at = datetime()`);
      lines.push('ON MATCH SET');
      lines.push(`  s_${scope.key}.display_name = '${scope.display_name}',`);
      lines.push(`  s_${scope.key}.emoji = '${scope.emoji}',`);
      lines.push(`  s_${scope.key}.color = '${scope.color}',`);
      lines.push(`  s_${scope.key}.llm_context = '${llmContext}',`);
      lines.push(`  s_${scope.key}.updated_at = datetime();`);
      lines.push('');
    }

    // Create Subcategories
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('// SUBCATEGORIES (9)');
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('');

    for (const scope of data.scopes) {
      lines.push(`// ${scope.display_name} subcategories`);
      for (const sub of scope.subcategories) {
        const llmContext = this.escapeCypher(sub.llm_context.trim());
        lines.push(`MERGE (sub_${sub.key}:Subcategory {key: '${sub.key}'})`);
        lines.push('ON CREATE SET');
        lines.push(`  sub_${sub.key}.display_name = '${sub.display_name}',`);
        lines.push(`  sub_${sub.key}.emoji = '${sub.emoji}',`);
        lines.push(`  sub_${sub.key}.llm_context = '${llmContext}',`);
        lines.push(`  sub_${sub.key}.created_at = datetime()`);
        lines.push('ON MATCH SET');
        lines.push(`  sub_${sub.key}.display_name = '${sub.display_name}',`);
        lines.push(`  sub_${sub.key}.emoji = '${sub.emoji}',`);
        lines.push(`  sub_${sub.key}.llm_context = '${llmContext}',`);
        lines.push(`  sub_${sub.key}.updated_at = datetime();`);
        lines.push('');
        lines.push(`MERGE (s_${scope.key})-[:HAS_SUBCATEGORY]->(sub_${sub.key});`);
        lines.push('');
      }
    }

    // Create NodeTypeMeta
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('// NODE TYPE META (35)');
    lines.push('// ═══════════════════════════════════════════════════════════════════════════════');
    lines.push('');

    for (const scope of data.scopes) {
      for (const sub of scope.subcategories) {
        const nodeTypes = nodeTypesBySubcategory.get(sub.key) || [];
        if (nodeTypes.length === 0) continue;

        lines.push(`// ${sub.display_name} (${nodeTypes.length} types)`);
        for (const nodeType of nodeTypes) {
          const varName = `t_${nodeType.toLowerCase().replace(/l10n/g, 'l10n')}`;
          lines.push(`MERGE (${varName}:NodeTypeMeta {label: '${nodeType}'})`);
          lines.push('ON CREATE SET');
          lines.push(`  ${varName}.display_name = '${nodeType}',`);
          lines.push(`  ${varName}.yaml_path = 'models/nodes/${scope.key}/${sub.key}/${this.toKebabCase(nodeType)}.yaml',`);
          lines.push(`  ${varName}.created_at = datetime()`);
          lines.push('ON MATCH SET');
          lines.push(`  ${varName}.updated_at = datetime();`);
          lines.push('');
          lines.push(`MERGE (sub_${sub.key})-[:DEFINES_TYPE]->(${varName});`);
          lines.push('');
        }
      }
    }

    return lines.join('\n');
  }

  /**
   * Escape string for Cypher
   */
  private static escapeCypher(str: string): string {
    return str
      .replace(/\\/g, '\\\\')
      .replace(/'/g, "\\'")
      .replace(/\n/g, ' ')
      .replace(/\s+/g, ' ');
  }

  /**
   * Convert PascalCase to kebab-case
   */
  private static toKebabCase(str: string): string {
    return str
      .replace(/([a-z])([A-Z])/g, '$1-$2')
      .replace(/([A-Z])([A-Z][a-z])/g, '$1-$2')
      .toLowerCase();
  }
}
```

**Step 4: Export from index**

Modify: `packages/schema-tools/src/index.ts`

Add after existing exports:
```typescript
export { OrganizingPrinciplesGenerator } from './generators/OrganizingPrinciplesGenerator.js';
```

**Step 5: Run test to verify it passes**

Run: `pnpm --filter @novanet/schema-tools test organizing-principles`
Expected: PASS

**Step 6: Commit**

```bash
git add packages/schema-tools/src/generators/OrganizingPrinciplesGenerator.ts
git add packages/schema-tools/src/__tests__/organizing-principles.test.ts
git add packages/schema-tools/src/index.ts
git commit -m "feat(schema-tools): add OrganizingPrinciplesGenerator

- Reads organizing-principles.yaml + folder structure
- Generates Cypher with MERGE for idempotency
- Creates Scope, Subcategory, NodeTypeMeta nodes
- Creates HAS_SUBCATEGORY, DEFINES_TYPE relations
- Full test coverage"
```

---

### Task 4: Generate and commit the seed file

**Files:**
- Create: `packages/db/seed/00.5-organizing-principles.cypher`

**Step 1: Add generate script**

Modify: `packages/schema-tools/scripts/generate-all.ts`

Add import and generation call (after SubcategoryGenerator):
```typescript
import { OrganizingPrinciplesGenerator } from '../src/generators/OrganizingPrinciplesGenerator.js';

// ... in main function, add:

// Generate organizing principles seed
console.log('Generating organizing principles seed...');
const cypherContent = await OrganizingPrinciplesGenerator.generate({
  organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
  modelsDir: path.join(MODELS_DIR, 'nodes'),
});
await fs.writeFile(
  path.join(DB_DIR, 'seed/00.5-organizing-principles.cypher'),
  cypherContent
);
console.log('✓ Generated 00.5-organizing-principles.cypher');
```

**Step 2: Run generator**

Run: `pnpm --filter @novanet/schema-tools generate:all`
Expected: "✓ Generated 00.5-organizing-principles.cypher"

**Step 3: Verify generated file**

Run: `head -30 packages/db/seed/00.5-organizing-principles.cypher`
Expected: Header with version, MERGE statements

**Step 4: Commit**

```bash
git add packages/schema-tools/scripts/generate-all.ts
git add packages/db/seed/00.5-organizing-principles.cypher
git commit -m "feat(db): generate 00.5-organizing-principles.cypher

- Auto-generated from organizing-principles.yaml
- 3 Scope nodes with llm_context
- 9 Subcategory nodes with llm_context
- 35 NodeTypeMeta nodes
- All relationships created"
```

---

### Task 5: Add sync test for organizing principles

**Files:**
- Modify: `packages/schema-tools/src/__tests__/sync.test.ts`

**Step 1: Add sync test**

Add new describe block after existing tests:

```typescript
import { OrganizingPrinciplesGenerator } from '../generators/OrganizingPrinciplesGenerator.js';

// Add path constant
const DB_DIR = path.join(SCHEMA_TOOLS_ROOT, '../db');

// Add new describe block
describe('Organizing Principles (00.5-organizing-principles.cypher)', () => {
  it('committed file matches generated content from organizing-principles.yaml', async () => {
    const committedPath = path.join(DB_DIR, 'seed/00.5-organizing-principles.cypher');

    // Generate fresh content
    const generated = await OrganizingPrinciplesGenerator.generate({
      organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
      modelsDir: path.join(MODELS_DIR, 'nodes'),
    });

    // Read committed file
    const committed = await fs.readFile(committedPath, 'utf-8');

    // Normalize and compare (ignore timestamp line)
    const normalizeContent = (content: string) => {
      return content
        .trim()
        .replace(/\r\n/g, '\n')
        .replace(/^\/\/ Generated: \d{4}-\d{2}-\d{2}$/m, '// Generated: DATE');
    };

    expect(normalizeContent(generated)).toBe(normalizeContent(committed));
  });

  it('includes all scopes from organizing-principles.yaml', async () => {
    const generated = await OrganizingPrinciplesGenerator.generate({
      organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
      modelsDir: path.join(MODELS_DIR, 'nodes'),
    });

    expect(generated).toContain("key: 'global'");
    expect(generated).toContain("key: 'project'");
    expect(generated).toContain("key: 'shared'");
  });
});
```

**Step 2: Run test to verify it passes**

Run: `pnpm --filter @novanet/schema-tools test sync`
Expected: All tests PASS

**Step 3: Commit**

```bash
git add packages/schema-tools/src/__tests__/sync.test.ts
git commit -m "test(schema-tools): add sync test for organizing principles

- Verifies committed Cypher matches generated
- Ensures no drift between YAML and seed file
- CI will catch any sync issues"
```

---

### Task 6: Seed database and verify

**Files:**
- None (runtime verification)

**Step 1: Reset and seed database**

Run: `pnpm infra:reset`
Expected: Neo4j restarts and seeds successfully

**Step 2: Verify organizing principles exist**

Run:
```bash
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (s:Scope) RETURN s.key, s.display_name ORDER BY s.key"
```
Expected:
```
╒════════════╤════════════════╕
│s.key       │s.display_name  │
╞════════════╪════════════════╡
│"global"    │"Global"        │
│"project"   │"Project"       │
│"shared"    │"Shared"        │
╘════════════╧════════════════╛
```

**Step 3: Verify hierarchy query**

Run:
```bash
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)-[:DEFINES_TYPE]->(t:NodeTypeMeta)
   RETURN s.key AS scope, sub.key AS subcategory, count(t) AS types
   ORDER BY s.key, sub.key"
```
Expected: 9 rows showing subcategories with their node type counts

**Step 4: Commit (if any fixes needed)**

No commit needed if all verification passes.

---

## Phase 2: Studio Integration (Future)

### Task 7: Create useOrganizingPrinciples hook

**Files:**
- Create: `apps/studio/src/hooks/useOrganizingPrinciples.ts`

> Note: This task is for Phase 2. Current Studio continues to use TypeScript imports.

**Step 1: Write the hook**

```typescript
// apps/studio/src/hooks/useOrganizingPrinciples.ts
// Query Neo4j for organizing principles hierarchy
// v1.0.0

import { useEffect, useState } from 'react';
import { useNeo4j } from './useNeo4j';
import { SCOPE_HIERARCHY } from '@novanet/core/graph';

interface OrganizingPrinciple {
  scope: string;
  scopeName: string;
  scopeEmoji: string;
  scopeColor: string;
  subcategory: string;
  subcategoryName: string;
  subcategoryEmoji: string;
  nodeTypes: string[];
}

/**
 * Query organizing principles from Neo4j.
 * Falls back to TypeScript SCOPE_HIERARCHY if query fails.
 */
export function useOrganizingPrinciples() {
  const { query } = useNeo4j();
  const [data, setData] = useState<OrganizingPrinciple[] | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchHierarchy() {
      try {
        const result = await query(`
          MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)-[:DEFINES_TYPE]->(t:NodeTypeMeta)
          RETURN
            s.key AS scope,
            s.display_name AS scopeName,
            s.emoji AS scopeEmoji,
            s.color AS scopeColor,
            sub.key AS subcategory,
            sub.display_name AS subcategoryName,
            sub.emoji AS subcategoryEmoji,
            collect(t.label) AS nodeTypes
          ORDER BY s.key, sub.key
        `);
        setData(result.records.map(r => r.toObject() as OrganizingPrinciple));
      } catch (err) {
        console.warn('Failed to query organizing principles, using fallback:', err);
        setError(err as Error);
        // Fallback handled by consumer
      } finally {
        setLoading(false);
      }
    }

    fetchHierarchy();
  }, [query]);

  return { data, loading, error, fallback: SCOPE_HIERARCHY };
}
```

**Step 2: Test and commit** (when implementing Phase 2)

---

## Summary

| Task | Files | Status |
|------|-------|--------|
| 1. Create organizing-principles.yaml | `packages/core/models/organizing-principles.yaml` | Phase 1 |
| 2. Add Neo4j constraints | `packages/db/seed/00-constraints.cypher` | Phase 1 |
| 3. Create OrganizingPrinciplesGenerator | `packages/schema-tools/src/generators/` | Phase 1 |
| 4. Generate seed file | `packages/db/seed/00.5-organizing-principles.cypher` | Phase 1 |
| 5. Add sync test | `packages/schema-tools/src/__tests__/sync.test.ts` | Phase 1 |
| 6. Seed and verify | Runtime verification | Phase 1 |
| 7. Create Studio hook | `apps/studio/src/hooks/` | Phase 2 (future) |

**Total Phase 1 tasks: 6**
**Estimated commits: 5**
