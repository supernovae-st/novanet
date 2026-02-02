#!/usr/bin/env npx tsx
/**
 * NovaNet Type Validation Script (v9.0.0)
 *
 * Validates that Studio types are synchronized with Core (Single Source of Truth).
 *
 * Checks:
 * 1. NODE_TYPES count matches between Core and Studio configs
 * 2. All NodeType values are covered in Studio visual configs
 * 3. No legacy v7.x types exist in Studio
 *
 * Usage:
 *   npx tsx scripts/validate-types.ts
 *   pnpm validate:types
 */

import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

// =============================================================================
// FILE READERS (avoid path alias issues)
// =============================================================================

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT = path.resolve(__dirname, '..');

function extractArrayFromFile(filePath: string, varName: string): string[] {
  const content = fs.readFileSync(path.join(ROOT, filePath), 'utf-8');
  // Match: export const NODE_TYPES = ['...', '...'] as const;
  const regex = new RegExp(`export const ${varName}\\s*=\\s*\\[([^\\]]+)\\]`, 's');
  const match = content.match(regex);
  if (!match) {
    throw new Error(`Could not find ${varName} in ${filePath}`);
  }
  // Extract quoted strings
  const items = match[1].match(/'([^']+)'/g) || [];
  return items.map((s) => s.replace(/'/g, ''));
}

function extractTopLevelObjectKeys(filePath: string, varName: string): string[] {
  const content = fs.readFileSync(path.join(ROOT, filePath), 'utf-8');

  // Find the object start
  const startRegex = new RegExp(`export const ${varName}[^=]*=\\s*{`, 's');
  const startMatch = content.match(startRegex);
  if (!startMatch || startMatch.index === undefined) {
    throw new Error(`Could not find ${varName} in ${filePath}`);
  }

  // Find balanced braces
  let braceCount = 1;
  let i = startMatch.index + startMatch[0].length;
  const startPos = i;
  while (braceCount > 0 && i < content.length) {
    if (content[i] === '{') braceCount++;
    if (content[i] === '}') braceCount--;
    i++;
  }
  const objectContent = content.slice(startPos, i - 1);

  // For Record<NodeType, ...>, match only PascalCase keys at the start of lines
  // These are the node type names like "Project:", "BrandIdentity:", etc.
  const keys: string[] = [];
  const keyRegex = /^\s{2}([A-Z][A-Za-z0-9]+):\s*\{/gm;
  let keyMatch;
  while ((keyMatch = keyRegex.exec(objectContent)) !== null) {
    keys.push(keyMatch[1]);
  }
  return keys;
}

function extractCategoryTypes(filePath: string): string[] {
  const content = fs.readFileSync(path.join(ROOT, filePath), 'utf-8');

  // Match NODE_CATEGORIES object and extract all quoted strings in arrays
  const regex = /export const NODE_CATEGORIES[^=]*=\s*\{([^}]+(?:\{[^}]*\}[^}]*)*)\}/s;
  const match = content.match(regex);
  if (!match) {
    throw new Error(`Could not find NODE_CATEGORIES in ${filePath}`);
  }

  // Extract all quoted strings from arrays
  const allTypes: string[] = [];
  const arrayRegex = /\[([^\]]+)\]/g;
  let arrayMatch;
  while ((arrayMatch = arrayRegex.exec(match[1])) !== null) {
    const items = arrayMatch[1].match(/'([^']+)'/g) || [];
    items.forEach((s) => allTypes.push(s.replace(/'/g, '')));
  }
  return allTypes;
}

// =============================================================================
// VALIDATION HELPERS
// =============================================================================

type ValidationResult = { passed: boolean; message: string };

function success(message: string): ValidationResult {
  return { passed: true, message: `✓ ${message}` };
}

function failure(message: string): ValidationResult {
  return { passed: false, message: `✗ ${message}` };
}

// =============================================================================
// VALIDATIONS
// =============================================================================

function validateNodeTypesCount(
  coreTypes: string[],
  studioConfigs: string[],
  studioSizes: string[],
  studioColors: string[]
): ValidationResult {
  const coreCount = coreTypes.length;
  const studioConfigCount = studioConfigs.length;
  const studioSizesCount = studioSizes.length;
  const studioColorsCount = studioColors.length;

  if (coreCount !== studioConfigCount) {
    return failure(
      `nodeTypeConfigs count mismatch: Core=${coreCount}, Studio=${studioConfigCount}`
    );
  }

  if (coreCount !== studioSizesCount) {
    return failure(
      `NODE_SIZES count mismatch: Core=${coreCount}, Studio=${studioSizesCount}`
    );
  }

  if (coreCount !== studioColorsCount) {
    return failure(
      `NODE_COLORS count mismatch: Core=${coreCount}, Studio=${studioColorsCount}`
    );
  }

  return success(`All configs have ${coreCount} node types (v8.1.0)`);
}

function validateAllTypesPresent(
  coreTypes: string[],
  studioConfigs: string[],
  studioSizes: string[],
  studioColors: string[]
): ValidationResult {
  const configsSet = new Set(studioConfigs);
  const sizesSet = new Set(studioSizes);
  const colorsSet = new Set(studioColors);

  const missingInConfigs: string[] = [];
  const missingInSizes: string[] = [];
  const missingInColors: string[] = [];

  for (const nodeType of coreTypes) {
    if (!configsSet.has(nodeType)) {
      missingInConfigs.push(nodeType);
    }
    if (!sizesSet.has(nodeType)) {
      missingInSizes.push(nodeType);
    }
    if (!colorsSet.has(nodeType)) {
      missingInColors.push(nodeType);
    }
  }

  const errors: string[] = [];
  if (missingInConfigs.length > 0) {
    errors.push(`Missing in nodeTypeConfigs: ${missingInConfigs.join(', ')}`);
  }
  if (missingInSizes.length > 0) {
    errors.push(`Missing in NODE_SIZES: ${missingInSizes.join(', ')}`);
  }
  if (missingInColors.length > 0) {
    errors.push(`Missing in NODE_COLORS: ${missingInColors.join(', ')}`);
  }

  if (errors.length > 0) {
    return failure(errors.join('\n  '));
  }

  return success('All Core NODE_TYPES are present in Studio configs');
}

function validateNoLegacyTypes(
  studioConfigs: string[],
  studioSizes: string[],
  studioColors: string[]
): ValidationResult {
  // Legacy v7.x types that should NOT exist
  const legacyTypes = [
    'PageOutput',
    'BlockOutput',
    'SEOKeyword',
    'SEOVariation',
    'SEOSnapshot',
    'GEOSeed',
    'GEOReformulation',
    'GEOCitation',
    'PageMetrics',
    'AudienceL10n',
    'ValuePropL10n',
    'SocialProofL10n',
  ];

  const configsSet = new Set(studioConfigs);
  const sizesSet = new Set(studioSizes);
  const colorsSet = new Set(studioColors);

  const foundLegacy: string[] = [];

  for (const legacyType of legacyTypes) {
    if (configsSet.has(legacyType)) {
      foundLegacy.push(legacyType);
    }
    if (sizesSet.has(legacyType)) {
      foundLegacy.push(`${legacyType} (sizes)`);
    }
    if (colorsSet.has(legacyType)) {
      foundLegacy.push(`${legacyType} (colors)`);
    }
  }

  if (foundLegacy.length > 0) {
    return failure(`Legacy v7.x types found: ${foundLegacy.join(', ')}`);
  }

  return success('No legacy v7.x types found');
}

function validateCategoryConsistency(
  coreTypes: string[],
  categoryTypes: string[]
): ValidationResult {
  const typesInCategories = new Set(categoryTypes);

  const missingFromCategories: string[] = [];
  for (const nodeType of coreTypes) {
    if (!typesInCategories.has(nodeType)) {
      missingFromCategories.push(nodeType);
    }
  }

  if (missingFromCategories.length > 0) {
    return failure(
      `Types missing from NODE_CATEGORIES: ${missingFromCategories.join(', ')}`
    );
  }

  return success('All types are in NODE_CATEGORIES');
}

// =============================================================================
// MAIN
// =============================================================================

function main(): void {
  console.log('═══════════════════════════════════════════════════════════════');
  console.log('  NovaNet Type Validation (v8.1.0)');
  console.log('═══════════════════════════════════════════════════════════════\n');

  // Extract data from source files
  const coreTypes = extractArrayFromFile('packages/core/src/types/nodes.ts', 'NODE_TYPES');
  const studioConfigs = extractTopLevelObjectKeys(
    'apps/studio/src/config/nodeTypes.ts',
    'nodeTypeConfigs'
  );
  const studioSizes = extractTopLevelObjectKeys(
    'apps/studio/src/components/graph/nodes/NodeConfig.ts',
    'NODE_SIZES'
  );
  const studioColors = extractTopLevelObjectKeys(
    'apps/studio/src/components/graph/nodes/NodeConfig.ts',
    'NODE_COLORS'
  );
  const categoryTypes = extractCategoryTypes('apps/studio/src/config/nodeTypes.ts');

  console.log(`Core NODE_TYPES: ${coreTypes.length} types`);
  console.log(`Studio nodeTypeConfigs: ${studioConfigs.length} types`);
  console.log(`Studio NODE_SIZES: ${studioSizes.length} types`);
  console.log(`Studio NODE_COLORS: ${studioColors.length} types`);
  console.log(`NODE_CATEGORIES: ${categoryTypes.length} types\n`);

  const validations: ValidationResult[] = [
    validateNodeTypesCount(coreTypes, studioConfigs, studioSizes, studioColors),
    validateAllTypesPresent(coreTypes, studioConfigs, studioSizes, studioColors),
    validateNoLegacyTypes(studioConfigs, studioSizes, studioColors),
    validateCategoryConsistency(coreTypes, categoryTypes),
  ];

  let hasErrors = false;

  for (const result of validations) {
    console.log(result.message);
    if (!result.passed) {
      hasErrors = true;
    }
  }

  console.log('\n═══════════════════════════════════════════════════════════════');

  if (hasErrors) {
    console.log('  ❌ VALIDATION FAILED - Types are out of sync!');
    console.log('═══════════════════════════════════════════════════════════════\n');
    process.exit(1);
  } else {
    console.log('  ✅ VALIDATION PASSED - All types synchronized!');
    console.log('═══════════════════════════════════════════════════════════════\n');
    process.exit(0);
  }
}

main();
