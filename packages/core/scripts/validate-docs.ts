#!/usr/bin/env tsx
// scripts/validate-docs.ts
// Validate that generated docs match source of truth (relations.yaml, _index.yaml, views/*.yaml)
// v8.1.0: Uses same async generation as generate-docs.ts
//
// Usage: npm run validate:docs [--fix] [--verbose]

import * as fs from 'fs/promises';
import * as path from 'path';
import { ViewParser } from '../src/generators/ViewParser.js';
import { MarkdownGenerator } from '../src/generators/MarkdownGenerator.js';
import type { ExtendedViewDefinition, ViewDocs } from '../src/generators/types.js';

type ViewWithDocs = ExtendedViewDefinition & { docs: ViewDocs };

const VIEWS_DIR = path.join(process.cwd(), 'models/views');
const OUTPUT_DIR = path.join(process.cwd(), 'models/docs/views');
const RELATIONS_PATH = path.join(process.cwd(), 'models/relations.yaml');
const INDEX_PATH = path.join(process.cwd(), 'models/_index.yaml');

interface ValidationResult {
  viewId: string;
  filename: string;
  status: 'ok' | 'outdated' | 'missing' | 'fixed';
  details?: string;
  diff?: { expected: number; actual: number };
}

interface ValidateOptions {
  fix: boolean;
  verbose: boolean;
}

function parseArgs(): ValidateOptions {
  const args = process.argv.slice(2);
  return {
    fix: args.includes('--fix') || args.includes('-f'),
    verbose: args.includes('--verbose') || args.includes('-v'),
  };
}

async function fileExists(filePath: string): Promise<boolean> {
  try {
    await fs.access(filePath);
    return true;
  } catch {
    return false;
  }
}

/**
 * Normalize content for comparison:
 * - Remove timestamps (> Last updated:)
 * - Trim trailing whitespace
 * - Normalize line endings
 */
function normalizeContent(content: string): string {
  return content
    .split('\n')
    .map(line => line.trimEnd())
    .filter(line => !line.startsWith('> Last updated:'))
    .join('\n')
    .trim();
}

/**
 * Generate expected content for a view.
 * Uses same logic as generate-docs.ts: async generation for complete-graph.
 */
async function generateExpectedContent(view: ViewWithDocs): Promise<string> {
  // Use generateAsync with full graph Mermaid for complete-graph view
  const result = view.id === 'complete-graph'
    ? await MarkdownGenerator.generateAsync(view, {
        useFullGraphMermaid: true,
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
        includeTimestamp: false,
      })
    : MarkdownGenerator.generate(view, { includeTimestamp: false });

  return result.content;
}

async function validateDocs(options: ValidateOptions): Promise<void> {
  console.log('🔍 Validating view documentation against source of truth...\n');

  if (options.fix) {
    console.log('🔧 Fix mode enabled: will regenerate outdated files\n');
  }

  // Load views with docs
  const allViews = await ViewParser.loadAllViews(VIEWS_DIR);
  const viewsWithDocs = allViews.filter(ViewParser.hasDocs);

  if (viewsWithDocs.length === 0) {
    console.log('✅ No views with docs sections found. Nothing to validate.');
    return;
  }

  console.log(`📁 Source files:`);
  console.log(`   • ${VIEWS_DIR}/*.yaml (view definitions)`);
  console.log(`   • ${RELATIONS_PATH} (relation definitions)`);
  console.log(`   • ${INDEX_PATH} (node index)`);
  console.log(`📂 Output: ${OUTPUT_DIR}`);
  console.log(`📊 Found ${viewsWithDocs.length} view(s) with docs\n`);

  const results: ValidationResult[] = [];

  for (const view of viewsWithDocs) {
    const filename = `VIEW-${view.id.toUpperCase()}.md`;
    const filePath = path.join(OUTPUT_DIR, filename);

    // Check if file exists
    if (!await fileExists(filePath)) {
      if (options.fix) {
        // Generate missing file
        const expected = await generateExpectedContent(view);
        await fs.mkdir(OUTPUT_DIR, { recursive: true });
        await fs.writeFile(filePath, expected, 'utf-8');
        results.push({
          viewId: view.id,
          filename,
          status: 'fixed',
          details: 'Generated missing file',
        });
      } else {
        results.push({
          viewId: view.id,
          filename,
          status: 'missing',
          details: 'File does not exist',
        });
      }
      continue;
    }

    // Generate expected content (using same async method as generate-docs.ts)
    const expected = await generateExpectedContent(view);

    // Read committed content
    const committed = await fs.readFile(filePath, 'utf-8');

    // Normalize for comparison
    const normalizedExpected = normalizeContent(expected);
    const normalizedCommitted = normalizeContent(committed);

    if (normalizedExpected === normalizedCommitted) {
      results.push({
        viewId: view.id,
        filename,
        status: 'ok',
      });
    } else {
      if (options.fix) {
        // Regenerate outdated file
        await fs.writeFile(filePath, expected, 'utf-8');
        results.push({
          viewId: view.id,
          filename,
          status: 'fixed',
          details: 'Regenerated from source',
        });
      } else {
        // Calculate diff stats for verbose output
        const expectedLines = normalizedExpected.split('\n').length;
        const committedLines = normalizedCommitted.split('\n').length;

        results.push({
          viewId: view.id,
          filename,
          status: 'outdated',
          details: 'Content differs from source of truth',
          diff: { expected: expectedLines, actual: committedLines },
        });
      }
    }
  }

  // Print results
  console.log('Results:');
  console.log('─'.repeat(70));

  let hasErrors = false;

  for (const result of results) {
    const icon = result.status === 'ok' ? '✅' :
                 result.status === 'fixed' ? '🔧' :
                 result.status === 'outdated' ? '⚠️' : '❌';

    console.log(`${icon} ${result.filename}`);

    if (result.details) {
      console.log(`   └─ ${result.details}`);
      if (result.status === 'outdated' || result.status === 'missing') {
        hasErrors = true;
      }
    }

    if (options.verbose && result.diff) {
      console.log(`   └─ Lines: expected ${result.diff.expected}, actual ${result.diff.actual}`);
    }
  }

  console.log('');
  console.log('─'.repeat(70));

  const counts = {
    ok: results.filter(r => r.status === 'ok').length,
    fixed: results.filter(r => r.status === 'fixed').length,
    outdated: results.filter(r => r.status === 'outdated').length,
    missing: results.filter(r => r.status === 'missing').length,
  };

  console.log(`✅ Up to date: ${counts.ok}`);
  if (counts.fixed > 0) {
    console.log(`🔧 Fixed: ${counts.fixed}`);
  }
  if (counts.outdated > 0) {
    console.log(`⚠️  Outdated: ${counts.outdated}`);
  }
  if (counts.missing > 0) {
    console.log(`❌ Missing: ${counts.missing}`);
  }
  console.log('');

  if (hasErrors) {
    console.log('💡 To fix, run: npm run validate:docs -- --fix');
    console.log('   Or run: npm run generate:docs\n');
    process.exit(1);
  }

  if (counts.fixed > 0) {
    console.log('✅ All issues fixed!\n');
  } else {
    console.log('✅ All documentation matches source of truth!\n');
  }
}

// Main
const options = parseArgs();
validateDocs(options).catch(error => {
  console.error('❌ Error:', error.message);
  process.exit(1);
});
