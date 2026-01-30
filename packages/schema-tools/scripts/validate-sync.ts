#!/usr/bin/env tsx
// scripts/validate-sync.ts
// Validates that committed artifacts match what would be generated from YAML sources

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
  relativePath: string;
  synced: boolean;
  error?: string;
}

async function validateFile(
  name: string,
  relativePath: string,
  generator: () => Promise<string>,
  committedPath: string
): Promise<ValidationResult> {
  try {
    const generated = await generator();
    const committed = await fs.readFile(committedPath, 'utf-8');

    // Normalize line endings and trim
    const normalizedGenerated = generated.trim().replace(/\r\n/g, '\n');
    const normalizedCommitted = committed.trim().replace(/\r\n/g, '\n');

    if (normalizedGenerated === normalizedCommitted) {
      return { file: name, relativePath, synced: true };
    }

    return {
      file: name,
      relativePath,
      synced: false,
      error: 'Content differs from committed file',
    };
  } catch (err) {
    return {
      file: name,
      relativePath,
      synced: false,
      error: err instanceof Error ? err.message : String(err),
    };
  }
}

async function main() {
  console.log('═══════════════════════════════════════════════════════');
  console.log('  @novanet/schema-tools - Validate Sync');
  console.log('═══════════════════════════════════════════════════════');
  console.log(`  Source: ${MODELS_DIR}`);
  console.log('');

  const results: ValidationResult[] = [];

  // Validate subcategories.ts
  const subcategoriesPath = path.join(CORE_DIR, 'src/graph/subcategories.ts');
  results.push(
    await validateFile(
      'subcategories.ts',
      'packages/core/src/graph/subcategories.ts',
      () => SubcategoryGenerator.generate({
        modelsDir: path.join(MODELS_DIR, 'nodes'),
      }),
      subcategoriesPath
    )
  );

  // Validate Mermaid
  const mermaidPath = path.join(MODELS_DIR, 'docs/views/VIEW-COMPLETE-GRAPH.md');
  results.push(
    await validateFile(
      'VIEW-COMPLETE-GRAPH.md',
      'packages/core/models/docs/views/VIEW-COMPLETE-GRAPH.md',
      () => MermaidGenerator.generate({ modelsDir: MODELS_DIR }),
      mermaidPath
    )
  );

  // Report
  let allSynced = true;
  console.log('Results:');
  for (const result of results) {
    const status = result.synced ? '✅' : '❌';
    console.log(`  ${status} ${result.file}`);
    if (!result.synced) {
      allSynced = false;
      console.log(`     └── ${result.error}`);
      console.log(`     └── Path: ${result.relativePath}`);
    }
  }

  console.log('');

  if (!allSynced) {
    console.log('═══════════════════════════════════════════════════════');
    console.log('  ❌ SYNC FAILED');
    console.log('');
    console.log('  To fix, run:');
    console.log('    pnpm schema:generate');
    console.log('');
    console.log('  Then commit the regenerated files.');
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
