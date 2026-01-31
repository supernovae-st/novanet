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
    expect(cypher).toContain(':Scope {key:');
    expect(cypher).toContain(':Subcategory {key:');
    expect(cypher).toContain(':NodeTypeMeta {label:');
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
