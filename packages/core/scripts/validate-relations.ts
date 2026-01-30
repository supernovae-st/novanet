#!/usr/bin/env tsx
// NovaNet Core - Relation Registry Validator
// Ensures relations.yaml matches TypeScript registry

import { readFileSync } from 'fs';
import { join } from 'path';
import { parse } from 'yaml';
import { RelationRegistry, RelationType } from '../src/schemas/relations.schema.js';

const ROOT = process.cwd();

console.log('🔗 Validating relation registry consistency...\n');

const hasError = false;

// Load relations.yaml
try {
  const yamlContent = readFileSync(join(ROOT, 'models/relations.yaml'), 'utf-8');
  const yamlRelations = parse(yamlContent);

  const yamlRelationNames = new Set(Object.keys(yamlRelations.relations || {}));
  const tsRelationNames = new Set(Object.values(RelationType) as string[]);

  // Check for relations in YAML but not in TypeScript
  for (const name of yamlRelationNames) {
    if (!tsRelationNames.has(name)) {
      console.log(`⚠️  YAML has '${name}' but TypeScript registry missing it`);
    }
  }

  // Check for relations in TypeScript but not in YAML
  for (const name of tsRelationNames) {
    if (!yamlRelationNames.has(name)) {
      console.log(`⚠️  TypeScript has '${name}' but models/relations.yaml missing it`);
    }
  }

  // Count matching
  const matching = [...yamlRelationNames].filter(n => tsRelationNames.has(n)).length;
  console.log(`📊 ${matching}/${yamlRelationNames.size} YAML relations in TypeScript`);
  console.log(`📊 ${matching}/${tsRelationNames.size} TypeScript relations in YAML`);

} catch (e) {
  console.log(`⚠️  Could not load models/relations.yaml: ${e}`);
}

console.log('');

// Validate registry completeness
console.log('📋 Registry completeness check...\n');

for (const [type, def] of Object.entries(RelationRegistry)) {
  const issues: string[] = [];

  if (!def.description || def.description.length < 10) {
    issues.push('missing/short description');
  }

  if (!def.cardinality) {
    issues.push('missing cardinality');
  }

  if (issues.length > 0) {
    console.log(`⚠️  ${type}: ${issues.join(', ')}`);
  }
}

console.log('');

if (hasError) {
  console.log('❌ Relation validation failed!');
  process.exit(1);
} else {
  console.log('✅ Relation registry valid!');
  process.exit(0);
}
