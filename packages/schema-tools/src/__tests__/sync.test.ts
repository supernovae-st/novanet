// packages/schema-tools/src/__tests__/sync.test.ts
// Evolutionary tests for YAML → Artifact synchronization
// These tests read from YAML sources and evolve with the ontology
//
// v1.0.0 - Initial test suite
// Run: pnpm --filter @novanet/schema-tools test

import { describe, it, expect } from 'vitest';
import * as fs from 'fs/promises';
import * as path from 'path';
import { fileURLToPath } from 'url';
import { MermaidGenerator } from '../generators/MermaidGenerator.js';
import { SubcategoryGenerator } from '../generators/SubcategoryGenerator.js';
import { OrganizingPrinciplesGenerator } from '../generators/OrganizingPrinciplesGenerator.js';

// =============================================================================
// PATH SETUP
// =============================================================================

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const SCHEMA_TOOLS_ROOT = path.join(__dirname, '../..');
const CORE_DIR = path.join(SCHEMA_TOOLS_ROOT, '../core');
const DB_DIR = path.join(SCHEMA_TOOLS_ROOT, '../db');
const MODELS_DIR = path.join(CORE_DIR, 'models');

// =============================================================================
// SYNC TESTS - Evolutionary (read from YAML, not hardcoded)
// =============================================================================

describe('YAML → Artifact Synchronization', () => {
  /**
   * These tests verify that committed artifacts match what would be generated
   * from YAML sources. They are "evolutionary" because they:
   * - Read from YAML (single source of truth)
   * - Don't hardcode node counts or specific values
   * - Automatically adapt as ontology evolves
   */

  describe('Mermaid Diagram (VIEW-COMPLETE-GRAPH.md)', () => {
    it('committed file matches generated content from relations.yaml + _index.yaml', async () => {
      const committedPath = path.join(MODELS_DIR, 'docs/views/VIEW-COMPLETE-GRAPH.md');

      // Generate fresh content from YAML sources (as Markdown with wrapper)
      const generated = await MermaidGenerator.generateMarkdown({
        modelsDir: MODELS_DIR,
      });

      // Read committed file
      const committed = await fs.readFile(committedPath, 'utf-8');

      // Normalize line endings and compare
      const normalizedGenerated = generated.trim().replace(/\r\n/g, '\n');
      const normalizedCommitted = committed.trim().replace(/\r\n/g, '\n');

      expect(normalizedGenerated).toBe(normalizedCommitted);
    });

    it('generates valid Mermaid flowchart syntax', async () => {
      const generated = await MermaidGenerator.generate({
        modelsDir: MODELS_DIR,
      });

      // Basic structural checks
      expect(generated).toContain('flowchart TB');
      expect(generated).toContain('subgraph');
      expect(generated).toContain('classDef');

      // Should have all three scope subgraphs
      expect(generated).toContain('GLOBAL_LAYER');
      expect(generated).toContain('SHARED_LAYER');
      expect(generated).toContain('PROJECT_LAYER');
    });
  });

  describe('Subcategories (subcategories.ts)', () => {
    it('committed file matches generated content from folder structure', async () => {
      const committedPath = path.join(CORE_DIR, 'src/graph/subcategories.ts');

      // Generate fresh content from folder structure
      const generated = await SubcategoryGenerator.generate({
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      });

      // Read committed file
      const committed = await fs.readFile(committedPath, 'utf-8');

      // Normalize line endings and compare
      const normalizedGenerated = generated.trim().replace(/\r\n/g, '\n');
      const normalizedCommitted = committed.trim().replace(/\r\n/g, '\n');

      expect(normalizedGenerated).toBe(normalizedCommitted);
    });

    it('generates valid TypeScript export structure', async () => {
      const generated = await SubcategoryGenerator.generate({
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      });

      // Basic structural checks
      expect(generated).toContain('export const NODE_SUBCATEGORIES');
      expect(generated).toContain("import type { NodeType }");
      expect(generated).toContain("import type { Subcategory }");
      expect(generated).toContain('export function getSubcategory');
      expect(generated).toContain('export function getNodeTypesBySubcategory');
    });

    it('maps all YAML files to subcategories', async () => {
      const generated = await SubcategoryGenerator.generate({
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      });

      // Count the number of YAML files in models/nodes
      const yamlFiles = await countYamlFiles(path.join(MODELS_DIR, 'nodes'));

      // Count entries in generated content (lines with NodeType: 'subcategory',)
      const nodeEntries = (generated.match(/^\s+\w+:\s+'[a-z]+',$/gm) || []).length;

      expect(nodeEntries).toBe(yamlFiles);
    });
  });

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
});

// =============================================================================
// GENERATOR TESTS
// =============================================================================

describe('MermaidGenerator', () => {
  it('throws on empty modelsDir', async () => {
    await expect(
      MermaidGenerator.generate({ modelsDir: '' })
    ).rejects.toThrow('modelsDir cannot be empty');
  });

  it('throws on non-existent modelsDir', async () => {
    await expect(
      MermaidGenerator.generate({ modelsDir: '/nonexistent/path' })
    ).rejects.toThrow('Failed to load');
  });
});

describe('SubcategoryGenerator', () => {
  it('throws on empty modelsDir', async () => {
    await expect(
      SubcategoryGenerator.generate({ modelsDir: '' })
    ).rejects.toThrow('modelsDir cannot be empty');
  });

  it('throws on non-existent modelsDir', async () => {
    await expect(
      SubcategoryGenerator.generate({ modelsDir: '/nonexistent/path' })
    ).rejects.toThrow('modelsDir does not exist');
  });
});

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Count YAML files recursively in a directory
 */
async function countYamlFiles(dir: string): Promise<number> {
  let count = 0;

  const entries = await fs.readdir(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      count += await countYamlFiles(fullPath);
    } else if (entry.name.endsWith('.yaml')) {
      count++;
    }
  }

  return count;
}
