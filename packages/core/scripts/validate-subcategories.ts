#!/usr/bin/env tsx
// scripts/validate-subcategories.ts
// Validate that subcategories.ts matches the folder structure
//
// Usage: npm run validate:subcategories
//
// Exits with code 0 if synchronized, 1 if out of sync

import * as fs from 'fs/promises';
import * as path from 'path';
import { SubcategoryGenerator } from '../src/generators/SubcategoryGenerator.js';

const MODELS_DIR = path.join(process.cwd(), 'models/nodes');
const OUTPUT_PATH = path.join(process.cwd(), 'src/graph/subcategories.ts');

/**
 * Normalize content for comparison (ignore generation timestamp)
 */
function normalizeContent(content: string): string {
  return content
    .replace(/\/\/ Generated: .+/, '// Generated: TIMESTAMP')
    .trim();
}

async function main(): Promise<void> {
  console.log('🔍 Validating subcategories.ts matches folder structure...\n');

  try {
    // Generate expected content from folder structure
    const expected = await SubcategoryGenerator.generate({
      modelsDir: MODELS_DIR,
    });

    // Read actual file
    let actual: string;
    try {
      actual = await fs.readFile(OUTPUT_PATH, 'utf-8');
    } catch {
      console.error('❌ Error: src/graph/subcategories.ts does not exist');
      console.error('   Run: npm run generate:subcategories');
      process.exit(1);
    }

    // Compare (ignoring timestamps)
    const normalizedExpected = normalizeContent(expected);
    const normalizedActual = normalizeContent(actual);

    if (normalizedExpected === normalizedActual) {
      console.log('✅ subcategories.ts is synchronized with folder structure');
      process.exit(0);
    } else {
      console.error('❌ subcategories.ts is OUT OF SYNC with folder structure\n');
      console.error('   The TypeScript file does not match models/nodes/ structure.');
      console.error('   This can happen when:');
      console.error('   - A YAML file was added/removed/moved');
      console.error('   - The file was manually edited\n');
      console.error('   Fix: npm run generate:subcategories\n');

      // Show what changed (simple diff)
      const expectedLines = normalizedExpected.split('\n');
      const actualLines = normalizedActual.split('\n');

      let differences = 0;
      for (let i = 0; i < Math.max(expectedLines.length, actualLines.length); i++) {
        if (expectedLines[i] !== actualLines[i] && differences < 10) {
          console.error(`   Line ${i + 1}:`);
          console.error(`     Expected: ${expectedLines[i] || '(missing)'}`);
          console.error(`     Actual:   ${actualLines[i] || '(missing)'}`);
          differences++;
        }
      }

      if (differences >= 10) {
        console.error('   ... (more differences)');
      }

      process.exit(1);
    }
  } catch (error) {
    console.error('❌ Error:', error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

main();
