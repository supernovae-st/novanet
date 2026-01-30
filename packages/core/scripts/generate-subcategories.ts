#!/usr/bin/env tsx
// scripts/generate-subcategories.ts
// Generate subcategories.ts from models/nodes/ folder structure
//
// Usage:
//   npm run generate:subcategories           # Generate and write to file
//   npm run generate:subcategories -- --dry-run  # Print to stdout only

import * as path from 'path';
import { SubcategoryGenerator } from '../src/generators/SubcategoryGenerator.js';

const MODELS_DIR = path.join(process.cwd(), 'models/nodes');
const OUTPUT_PATH = path.join(process.cwd(), 'src/graph/subcategories.ts');

async function main(): Promise<void> {
  const args = process.argv.slice(2);
  const dryRun = args.includes('--dry-run');

  console.log('📁 SubcategoryGenerator');
  console.log('   Source: models/nodes/ folder structure');
  console.log(`   Target: src/graph/subcategories.ts\n`);

  const config = {
    modelsDir: MODELS_DIR,
    outputPath: OUTPUT_PATH,
  };

  try {
    if (dryRun) {
      console.log('🔍 Dry run - generating content:\n');
      const content = await SubcategoryGenerator.generate(config);
      console.log(content);
    } else {
      await SubcategoryGenerator.writeToFile(config);
      console.log('✅ Generated src/graph/subcategories.ts');
      console.log('   TypeScript is now synchronized with YAML folder structure.');
    }
  } catch (error) {
    console.error('❌ Error:', error instanceof Error ? error.message : error);
    process.exit(1);
  }
}

main();
