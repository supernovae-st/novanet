#!/usr/bin/env tsx
// scripts/generate-all.ts
// Orchestrates all schema artifact generation from YAML sources

import { MermaidGenerator } from '../src/generators/MermaidGenerator.js';
import { SubcategoryGenerator } from '../src/generators/SubcategoryGenerator.js';
import { OrganizingPrinciplesGenerator } from '../src/generators/OrganizingPrinciplesGenerator.js';
import * as fs from 'fs/promises';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const CORE_DIR = path.join(__dirname, '../../core');
const MODELS_DIR = path.join(CORE_DIR, 'models');
const DB_DIR = path.join(__dirname, '../../db');

async function main() {
  console.log('═══════════════════════════════════════════════════════');
  console.log('  @novanet/schema-tools - Generate All Artifacts');
  console.log('═══════════════════════════════════════════════════════');
  console.log(`  Source: ${MODELS_DIR}`);
  console.log('');

  const startTime = Date.now();

  // 1. Subcategories
  console.log('[1/3] Generating subcategories.ts...');
  try {
    await SubcategoryGenerator.writeToFile({
      modelsDir: path.join(MODELS_DIR, 'nodes'),
      outputPath: path.join(CORE_DIR, 'src/graph/subcategories.ts'),
    });
    console.log('  ✅ packages/core/src/graph/subcategories.ts');
  } catch (error) {
    console.error('  ❌ Failed:', error instanceof Error ? error.message : error);
    process.exit(1);
  }

  // 2. Mermaid diagram
  console.log('[2/3] Generating VIEW-COMPLETE-GRAPH.md...');
  try {
    await MermaidGenerator.writeToFile({
      modelsDir: MODELS_DIR,
      outputPath: path.join(MODELS_DIR, 'docs/views/VIEW-COMPLETE-GRAPH.md'),
    });
    console.log('  ✅ packages/core/models/docs/views/VIEW-COMPLETE-GRAPH.md');
  } catch (error) {
    console.error('  ❌ Failed:', error instanceof Error ? error.message : error);
    process.exit(1);
  }

  // 3. Organizing principles seed
  console.log('[3/3] Generating 00.5-organizing-principles.cypher...');
  try {
    const cypherContent = await OrganizingPrinciplesGenerator.generate({
      organizingPrinciplesPath: path.join(MODELS_DIR, 'organizing-principles.yaml'),
      modelsDir: path.join(MODELS_DIR, 'nodes'),
    });
    await fs.writeFile(
      path.join(DB_DIR, 'seed/00.5-organizing-principles.cypher'),
      cypherContent
    );
    console.log('  ✅ packages/db/seed/00.5-organizing-principles.cypher');
  } catch (error) {
    console.error('  ❌ Failed:', error instanceof Error ? error.message : error);
    process.exit(1);
  }

  const elapsed = Date.now() - startTime;
  console.log('');
  console.log('═══════════════════════════════════════════════════════');
  console.log(`  ✅ All artifacts generated successfully (${elapsed}ms)`);
  console.log('═══════════════════════════════════════════════════════');
}

main().catch((err) => {
  console.error('❌ Generation failed:', err);
  process.exit(1);
});
