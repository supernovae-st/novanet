#!/usr/bin/env node
/**
 * Split relations.yaml into individual arc-kinds/{family}/{name}.yaml files.
 *
 * Usage:
 *   node tools/scripts/split-relations.mjs
 */

import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
// Use js-yaml or yaml from core package
// We'll manually install yaml in the script context
// For now, use a simple YAML parser approach
import * as yaml from '../../packages/core/node_modules/yaml/dist/index.js';
const { parse, stringify } = yaml;

const __dirname = dirname(fileURLToPath(import.meta.url));

// Find monorepo root
function findRoot() {
  let current = __dirname;
  while (current !== '/') {
    if (existsSync(join(current, 'pnpm-workspace.yaml'))) {
      return current;
    }
    current = dirname(current);
  }
  throw new Error('Could not find monorepo root');
}

const ROOT = findRoot();
const RELATIONS_PATH = join(ROOT, 'packages/core/models/relations.yaml');
const ARC_KINDS_DIR = join(ROOT, 'packages/core/models/arc-kinds');

function toKebabCase(name) {
  return name.toLowerCase().replace(/_/g, '-');
}

function inferScope(source, target) {
  if (source === target) return 'intra_realm';
  return 'intra_realm';
}

function buildCypherPattern(relType, source, target, properties) {
  const srcStr = Array.isArray(source) ? source[0] : source;
  const tgtStr = Array.isArray(target) ? target[0] : target;

  if (properties && properties.length > 0) {
    const propNames = properties
      .map((p) => (typeof p === 'string' ? p : p.name || p))
      .join(', ');
    return `(${srcStr})-[:${relType} {${propNames}}]->(${tgtStr})`;
  }
  return `(${srcStr})-[:${relType}]->(${tgtStr})`;
}

function writeArcKind(rel, outputPath) {
  const {
    type: relType,
    family,
    source,
    target,
    cardinality = 'many_to_many',
    llm_context: llmContext = '',
    properties,
    inverse_of: inverseOf,
    is_self_referential: isSelfRef,
  } = rel;

  // Build arc document
  const arc = {
    name: relType,
    family,
    scope: inferScope(source, target),
    source,
    target,
    cardinality,
  };

  if (isSelfRef) {
    arc.is_self_referential = true;
  }

  if (properties) {
    arc.properties = properties.map((prop) => {
      if (typeof prop === 'string') {
        return { name: prop, type: 'string', required: false };
      }
      return prop;
    });
  }

  if (inverseOf) {
    arc.inverse_of = inverseOf;
  }

  arc.llm_context = llmContext;
  arc.cypher_pattern = buildCypherPattern(relType, source, target, properties);

  const doc = { arc };

  // Build header comment
  const shortDesc = llmContext
    ? llmContext.split('.')[0]
    : `${relType} relationship`;
  const header = `# packages/core/models/arc-kinds/${family}/${toKebabCase(relType)}.yaml
# ArcKind: ${relType} — ${shortDesc}

`;

  // Ensure directory exists
  mkdirSync(dirname(outputPath), { recursive: true });

  // Write file
  const yamlContent = stringify(doc, { lineWidth: 100 });
  writeFileSync(outputPath, header + yamlContent, 'utf8');

  return outputPath;
}

function main() {
  // Load relations.yaml
  const content = readFileSync(RELATIONS_PATH, 'utf8');
  const data = parse(content);
  const relations = data.relations || [];

  console.log(`Found ${relations.length} relations in relations.yaml`);

  let created = 0;
  let skipped = 0;
  const errors = [];

  for (const rel of relations) {
    if (!rel.type) continue;

    const { type: relType, family } = rel;

    if (!family) {
      errors.push(`${relType}: missing family`);
      continue;
    }

    const filename = `${toKebabCase(relType)}.yaml`;
    const outputPath = join(ARC_KINDS_DIR, family, filename);

    if (existsSync(outputPath)) {
      console.log(`  SKIP ${family}/${filename} (already exists)`);
      skipped++;
      continue;
    }

    try {
      writeArcKind(rel, outputPath);
      console.log(`  CREATE ${family}/${filename}`);
      created++;
    } catch (e) {
      errors.push(`${relType}: ${e.message}`);
    }
  }

  console.log(`\nSummary:`);
  console.log(`  Created: ${created}`);
  console.log(`  Skipped: ${skipped}`);
  console.log(`  Errors: ${errors.length}`);

  if (errors.length > 0) {
    console.log('\nErrors:');
    for (const e of errors) {
      console.log(`  - ${e}`);
    }
  }
}

main();
