#!/usr/bin/env tsx
// NovaNet Core Seed Script v7.0.0 - Graph-Native Locale

import { getSession, closeDriver } from '../src/db/client.js';
import { readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const SEED_PATH = join(__dirname, '../neo4j/seed');

async function main() {
  const args = process.argv.slice(2);
  const shouldReset = args.includes('--reset');

  console.log('🚀 NovaNet Core Seed v7.0.0');
  console.log('');

  const session = getSession();

  try {
    if (shouldReset) {
      console.log('🗑️  Resetting database...');
      await session.run('MATCH (n) DETACH DELETE n');
      console.log('   Done.');
      console.log('');
    }

    // Run constraints first
    console.log('📐 Running constraints...');
    const constraintsPath = join(SEED_PATH, '00-constraints.cypher');
    await runCypherFile(session, constraintsPath);
    console.log('   Done.');
    console.log('');

    // Run seed data
    console.log('🌱 Running seed data...');
    const seedPath = join(SEED_PATH, '01-concepts-mvp.cypher');
    await runCypherFile(session, seedPath);
    console.log('   Done.');
    console.log('');

    // Verify
    console.log('✅ Verifying...');
    const result = await session.run(`
      MATCH (n)
      RETURN labels(n)[0] AS label, count(*) AS count
      ORDER BY count DESC
    `);

    console.log('');
    console.log('   Node counts:');
    for (const record of result.records) {
      const label = record.get('label');
      const count = record.get('count').toNumber();
      console.log(`   - ${label}: ${count}`);
    }

    console.log('');
    console.log('🎉 Seed complete!');

  } finally {
    await session.close();
    await closeDriver();
  }
}

async function runCypherFile(session: ReturnType<typeof getSession>, filePath: string) {
  console.log(`   Reading: ${filePath}`);
  const content = readFileSync(filePath, 'utf-8');

  // Split by semicolon but ignore those in strings
  const statements = splitCypherStatements(content);
  console.log(`   Found ${statements.length} statements`);

  let executed = 0;
  let skipped = 0;
  let errors = 0;

  for (const statement of statements) {
    const trimmed = statement.trim();

    // Skip empty lines and comments
    if (!trimmed || trimmed.startsWith('//')) {
      skipped++;
      continue;
    }

    try {
      const result = await session.run(trimmed);
      executed++;

      // Log summary for data-creating statements
      const summary = result.summary.counters.updates();
      if (summary.nodesCreated > 0 || summary.relationshipsCreated > 0) {
        console.log(`   ✓ +${summary.nodesCreated} nodes, +${summary.relationshipsCreated} rels`);
      }
    } catch (error) {
      const msg = String(error);

      // Expected errors for constraints/indexes - safe to skip
      if (msg.includes('already exists') || msg.includes('EquivalentSchemaRuleAlreadyExists')) {
        skipped++;
        continue;
      }

      // Unexpected errors - fail fast
      errors++;
      console.error(`   ✗ FATAL Error: ${trimmed.substring(0, 60)}...`);
      console.error(`     ${msg}`);
      throw new Error(`Seed script failed during statement execution: ${msg.substring(0, 200)}`);
    }
  }

  console.log(`   Summary: ${executed} executed, ${skipped} skipped, ${errors} errors`);
}

function splitCypherStatements(content: string): string[] {
  // Remove line comments first (but preserve string content)
  const lines = content.split('\n');
  const cleanedLines: string[] = [];

  for (const line of lines) {
    const trimmed = line.trim();
    // Skip pure comment lines
    if (trimmed.startsWith('//')) {
      continue;
    }
    // Remove inline comments (but not inside strings)
    let inString = false;
    let stringChar = '';
    let cleanLine = '';

    for (let i = 0; i < line.length; i++) {
      const char = line[i];
      const prevChar = line[i - 1];
      const nextChar = line[i + 1];

      if ((char === '"' || char === "'") && prevChar !== '\\') {
        if (!inString) {
          inString = true;
          stringChar = char;
        } else if (char === stringChar) {
          inString = false;
        }
      }

      // If we hit // outside of string, stop
      if (char === '/' && nextChar === '/' && !inString) {
        break;
      }

      cleanLine += char;
    }

    if (cleanLine.trim()) {
      cleanedLines.push(cleanLine);
    }
  }

  const cleanedContent = cleanedLines.join('\n');

  // Now split by semicolon
  const statements: string[] = [];
  let current = '';
  let inString = false;
  let stringChar = '';

  for (let i = 0; i < cleanedContent.length; i++) {
    const char = cleanedContent[i];
    const prevChar = cleanedContent[i - 1];

    // Handle string boundaries
    if ((char === '"' || char === "'") && prevChar !== '\\') {
      if (!inString) {
        inString = true;
        stringChar = char;
      } else if (char === stringChar) {
        inString = false;
      }
    }

    // Split on semicolon only if not in string
    if (char === ';' && !inString) {
      const trimmed = current.trim();
      if (trimmed) {
        statements.push(trimmed);
      }
      current = '';
    } else {
      current += char;
    }
  }

  // Don't forget the last statement
  const lastTrimmed = current.trim();
  if (lastTrimmed) {
    statements.push(lastTrimmed);
  }

  return statements;
}

main().catch((error) => {
  console.error('❌ Seed failed:', error);
  process.exit(1);
});
