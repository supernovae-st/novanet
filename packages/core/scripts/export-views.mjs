#!/usr/bin/env node
// packages/core/scripts/export-views.mjs
// Export views.yaml as canonical JSON for cross-validation with Rust

import { promises as fs } from 'fs';
import { createHash } from 'crypto';
import path from 'path';
import yaml from 'js-yaml';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

function hashCypher(cypher) {
  if (!cypher) return '';
  const hash = createHash('sha256');
  hash.update(cypher.trim());
  return hash.digest('hex').slice(0, 8);
}

async function main() {
  // Find views.yaml
  const viewsPath = path.join(__dirname, '../models/views.yaml');
  const content = await fs.readFile(viewsPath, 'utf-8');
  const registry = yaml.load(content);

  // Build canonical format
  const views = registry.views.map(v => ({
    applicable_types: v.applicable_types || [],
    category: v.category,
    color: v.color,
    contextual: v.contextual || false,
    cypher_hash: hashCypher(v.cypher),
    description: v.description,
    icon: {
      terminal: v.icon.terminal,
      web: v.icon.web,
    },
    id: v.id,
    name: v.name,
    root_type: v.root_type === undefined ? null : v.root_type,
  }));

  // Sort by id
  views.sort((a, b) => a.id.localeCompare(b.id));

  const output = {
    count: views.length,
    version: registry.version,
    views,
  };

  console.log(JSON.stringify(output, null, 2));
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
