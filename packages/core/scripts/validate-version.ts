#!/usr/bin/env tsx
// NovaNet Core - Version Consistency Validator
// Ensures all files reference the same version

import { readFileSync } from 'fs';
import { join } from 'path';

const ROOT = process.cwd();

interface VersionCheck {
  file: string;
  pattern: RegExp;
  version?: string;
}

const checks: VersionCheck[] = [
  { file: 'package.json', pattern: /"version":\s*"([^"]+)"/ },
  { file: 'models/_index.yaml', pattern: /v(\d+\.\d+\.\d+)/ },
  { file: 'src/types/index.ts', pattern: /v(\d+\.\d+\.\d+)/ },
  { file: 'neo4j/seed/01-concepts-mvp.cypher', pattern: /v(\d+\.\d+\.\d+)/ },
];

console.log('🔢 Checking version consistency...\n');

let baseVersion: string | null = null;
let hasError = false;

for (const check of checks) {
  try {
    const content = readFileSync(join(ROOT, check.file), 'utf-8');
    const match = content.match(check.pattern);

    if (match) {
      check.version = match[1];

      if (!baseVersion) {
        baseVersion = check.version;
        console.log(`📦 Base version: ${baseVersion} (from ${check.file})`);
      } else if (check.version !== baseVersion) {
        console.log(`❌ ${check.file}: ${check.version} (expected ${baseVersion})`);
        hasError = true;
      } else {
        console.log(`✅ ${check.file}: ${check.version}`);
      }
    } else {
      console.log(`⚠️  ${check.file}: No version found`);
    }
  } catch (_e) {
    console.log(`⚠️  ${check.file}: File not found`);
  }
}

console.log('');

if (hasError) {
  console.log('❌ Version mismatch detected! Please align all versions.');
  process.exit(1);
} else {
  console.log('✅ All versions consistent!');
  process.exit(0);
}
