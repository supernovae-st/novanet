#!/usr/bin/env tsx
// scripts/generate-docs.ts
// Generate Markdown documentation from view definitions
// Usage: npm run generate:docs [--view <view-id>] [--output <dir>]

import * as fs from 'fs/promises';
import * as path from 'path';
import { ViewParser } from '../src/generators/ViewParser.js';
import { MarkdownGenerator } from '../src/generators/MarkdownGenerator.js';
import { CypherExporter } from '../src/generators/CypherExporter.js';

const VIEWS_DIR = path.join(process.cwd(), 'models/views');
const OUTPUT_DIR = path.join(process.cwd(), 'models/docs/views');
const CYPHER_OUTPUT_DIR = path.join(process.cwd(), 'neo4j/queries');

interface GenerateOptions {
  viewId?: string;
  outputDir?: string;
  cypherDir?: string;
  verbose?: boolean;
}

async function parseArgs(): Promise<GenerateOptions> {
  const args = process.argv.slice(2);
  const options: GenerateOptions = {
    outputDir: OUTPUT_DIR,
    cypherDir: CYPHER_OUTPUT_DIR,
    verbose: false,
  };

  for (let i = 0; i < args.length; i++) {
    switch (args[i]) {
      case '--view':
      case '-v':
        options.viewId = args[++i];
        break;
      case '--output':
      case '-o':
        options.outputDir = args[++i];
        break;
      case '--cypher':
      case '-c':
        options.cypherDir = args[++i];
        break;
      case '--verbose':
        options.verbose = true;
        break;
      case '--help':
      case '-h':
        console.log(`
📄 NovaNet Documentation Generator

Usage: npm run generate:docs [options]

Options:
  --view, -v <id>     Generate docs for specific view only
  --output, -o <dir>  Output directory for MD files (default: models/)
  --cypher, -c <dir>  Output directory for Cypher files (default: neo4j/queries/)
  --verbose           Show detailed output
  --help, -h          Show this help message

Examples:
  npm run generate:docs                    # Generate all docs
  npm run generate:docs --view page-gen    # Generate specific view
  npm run generate:docs --verbose          # Verbose output
`);
        process.exit(0);
    }
  }

  return options;
}

async function ensureDir(dir: string): Promise<void> {
  try {
    await fs.mkdir(dir, { recursive: true });
  } catch {
    // Directory exists
  }
}

async function generateDocs(options: GenerateOptions): Promise<void> {
  console.log('📄 NovaNet Documentation Generator v8.0.0\n');

  // Load views
  const allViews = await ViewParser.loadAllViews(VIEWS_DIR);
  const viewsWithDocs = allViews.filter(ViewParser.hasDocs);

  if (viewsWithDocs.length === 0) {
    console.log('⚠️  No views with docs sections found.');
    console.log('   Add a "docs:" section to your view YAML files.');
    return;
  }

  // Filter by view ID if specified
  const viewsToGenerate = options.viewId
    ? viewsWithDocs.filter(v => v.id === options.viewId || v.id.includes(options.viewId!))
    : viewsWithDocs;

  if (viewsToGenerate.length === 0) {
    console.log(`⚠️  No views found matching "${options.viewId}"`);
    return;
  }

  console.log(`📁 Views directory: ${VIEWS_DIR}`);
  console.log(`📂 Output directory: ${options.outputDir}`);
  console.log(`🔍 Found ${viewsToGenerate.length} view(s) with docs\n`);

  await ensureDir(options.outputDir!);
  await ensureDir(options.cypherDir!);

  let generated = 0;
  let failed = 0;

  const RELATIONS_PATH = path.join(process.cwd(), 'models/relations.yaml');
  const INDEX_PATH = path.join(process.cwd(), 'models/_index.yaml');

  for (const view of viewsToGenerate) {
    try {
      // Generate Markdown
      // Use generateAsync with full graph Mermaid for complete-graph view
      const mdResult = view.id === 'complete-graph'
        ? await MarkdownGenerator.generateAsync(view, {
            useFullGraphMermaid: true,
            relationsPath: RELATIONS_PATH,
            indexPath: INDEX_PATH,
          })
        : MarkdownGenerator.generate(view);
      const mdFilename = `VIEW-${view.id.toUpperCase()}.md`;
      const mdPath = path.join(options.outputDir!, mdFilename);

      await fs.writeFile(mdPath, mdResult.content, 'utf-8');

      if (options.verbose) {
        console.log(`  ✅ ${mdFilename}`);
      }

      // Generate Cypher file if view has examples
      if (view.docs?.examples && view.docs.examples.length > 0) {
        const cypherContent = CypherExporter.toCypherFile(view);
        const cypherFilename = `${view.id}.cypher`;
        const cypherPath = path.join(options.cypherDir!, cypherFilename);

        await fs.writeFile(cypherPath, cypherContent, 'utf-8');

        if (options.verbose) {
          console.log(`  ✅ ${cypherFilename}`);
        }
      }

      generated++;
    } catch (error) {
      console.error(`  ❌ Failed: ${view.id}`);
      if (options.verbose) {
        console.error(`     ${error}`);
      }
      failed++;
    }
  }

  console.log('');
  console.log('─'.repeat(50));
  console.log(`✅ Generated: ${generated} file(s)`);
  if (failed > 0) {
    console.log(`❌ Failed: ${failed} file(s)`);
  }
  console.log('');

  // List generated files
  if (!options.verbose && generated > 0) {
    console.log('Generated files:');
    for (const view of viewsToGenerate) {
      console.log(`  • models/VIEW-${view.id.toUpperCase()}.md`);
    }
  }
}

// Main
parseArgs()
  .then(generateDocs)
  .catch(error => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
