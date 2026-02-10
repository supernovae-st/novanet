#!/usr/bin/env node
/**
 * Unified Color System Validation Script
 *
 * Validates the color coherence across the entire codebase:
 * - palette.ts derives from generated.ts (taxonomy.yaml source of truth)
 * - nodeColors.ts uses unified palette
 * - colorPalette.ts (3D) re-exports from unified palette
 * - arcParticles.ts uses unified arc family detection
 * - Sidebar components use unified getters
 *
 * Run: node tools/scripts/validate-unified-colors.mjs
 *
 * v11.7.0 - Unified Color System
 */

import { readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, '../..');

// ANSI colors
const RED = '\x1b[31m';
const GREEN = '\x1b[32m';
const YELLOW = '\x1b[33m';
const CYAN = '\x1b[36m';
const RESET = '\x1b[0m';
const BOLD = '\x1b[1m';

let errors = 0;
let warnings = 0;
let checks = 0;

function logHeader(title) {
  console.log(`\n${BOLD}${CYAN}═══════════════════════════════════════════════════════════════${RESET}`);
  console.log(`${BOLD}${CYAN}  ${title}${RESET}`);
  console.log(`${CYAN}═══════════════════════════════════════════════════════════════${RESET}\n`);
}

function logSection(title) {
  console.log(`\n${BOLD}┌─ ${title}${RESET}`);
}

function logOk(msg) {
  console.log(`${GREEN}  ✓${RESET} ${msg}`);
  checks++;
}

function logError(msg) {
  console.log(`${RED}  ✗${RESET} ${msg}`);
  errors++;
  checks++;
}

function logWarn(msg) {
  console.log(`${YELLOW}  ⚠${RESET} ${msg}`);
  warnings++;
}

function logInfo(msg) {
  console.log(`${CYAN}  ℹ${RESET} ${msg}`);
}

function loadFile(relativePath) {
  const fullPath = join(ROOT, relativePath);
  if (!existsSync(fullPath)) {
    return null;
  }
  return readFileSync(fullPath, 'utf-8');
}

// =============================================================================
// Validators
// =============================================================================

function validatePaletteExists() {
  logSection('Unified Palette File');

  const palettePath = 'apps/studio/src/design/colors/palette.ts';
  const palette = loadFile(palettePath);

  if (!palette) {
    logError(`palette.ts not found at ${palettePath}`);
    return null;
  }

  logOk('palette.ts exists');

  // Check imports from generated.ts
  if (palette.includes("from './generated'")) {
    logOk('Imports from generated.ts (taxonomy.yaml source)');
  } else {
    logError('Does NOT import from generated.ts');
  }

  // Check required exports
  const requiredExports = [
    'getLayerPalette',
    'getRealmPalette',
    'getTraitPalette',
    'getArcPalette',
    'getArcFamily',
    'LAYER_HEX',
    'REALM_HEX',
    'TRAIT_HEX',
    'ARC_FAMILY_HEX',
  ];

  for (const exp of requiredExports) {
    if (palette.includes(`export function ${exp}`) || palette.includes(`export const ${exp}`)) {
      logOk(`Exports ${exp}`);
    } else {
      logError(`Missing export: ${exp}`);
    }
  }

  return palette;
}

function validateNodeColorsUsesUnified() {
  logSection('nodeColors.ts Uses Unified Palette');

  const nodeColors = loadFile('apps/studio/src/design/nodeColors.ts');
  if (!nodeColors) {
    logError('nodeColors.ts not found');
    return;
  }

  // Check it imports from unified palette
  if (nodeColors.includes("from '@/design/colors/palette'") ||
      nodeColors.includes("from './colors/palette'")) {
    logOk('Imports from unified palette');
  } else {
    logError('Does NOT import from unified palette');
  }

  // Check it uses getLayerPalette
  if (nodeColors.includes('getLayerPalette')) {
    logOk('Uses getLayerPalette for layer colors');
  } else {
    logError('Does NOT use getLayerPalette');
  }

  // Check it uses getArcPalette
  if (nodeColors.includes('getArcPalette')) {
    logOk('Uses getArcPalette for arc colors');
  } else {
    logError('Does NOT use getArcPalette');
  }

  // Check for hardcoded color definitions (should be removed)
  const hardcodedLayerColors = nodeColors.match(/LAYER_GRADIENT_COLORS:\s*Record.*=\s*\{[^}]+#[a-fA-F0-9]{6}/);
  if (hardcodedLayerColors) {
    logError('Still has hardcoded LAYER_GRADIENT_COLORS');
  } else {
    logOk('No hardcoded LAYER_GRADIENT_COLORS');
  }
}

function validateColorPalette3DUsesUnified() {
  logSection('colorPalette.ts (3D) Re-exports from Unified');

  const colorPalette = loadFile('apps/studio/src/lib/graph3d/colorPalette.ts');
  if (!colorPalette) {
    logError('colorPalette.ts not found');
    return;
  }

  // Check it imports from unified palette
  if (colorPalette.includes("from '@/design/colors/palette'")) {
    logOk('Imports from unified palette');
  } else {
    logError('Does NOT import from unified palette');
  }

  // Check it re-exports unified colors
  const unifiedReExports = ['LAYER_HEX', 'REALM_HEX', 'TRAIT_HEX', 'ARC_FAMILY_HEX'];
  for (const reExport of unifiedReExports) {
    if (colorPalette.includes(reExport)) {
      logOk(`Uses ${reExport} from unified palette`);
    } else {
      logWarn(`May not use ${reExport} from unified palette`);
    }
  }

  // Check for hardcoded colors (should be removed)
  const hardcodedColors = colorPalette.match(/config:\s*'#[a-fA-F0-9]{6}'/);
  if (hardcodedColors) {
    logError('Still has hardcoded layer colors');
  } else {
    logOk('No hardcoded layer colors');
  }
}

function validateArcParticlesUsesUnified() {
  logSection('arcParticles.ts Uses Unified Arc Family Detection');

  const arcParticles = loadFile('apps/studio/src/lib/graph3d/arcParticles.ts');
  if (!arcParticles) {
    logError('arcParticles.ts not found');
    return;
  }

  // Check it imports from unified palette
  if (arcParticles.includes("from '@/design/colors/palette'")) {
    logOk('Imports from unified palette');
  } else {
    logError('Does NOT import from unified palette');
  }

  // Check it uses getArcFamily from unified
  if (arcParticles.includes('getArcFamily') || arcParticles.includes('getArcFamilyUnified')) {
    logOk('Uses unified getArcFamily');
  } else {
    logError('Does NOT use unified getArcFamily');
  }

  // Check detectArcFamily is simplified (should delegate to unified)
  const detectArcFamily = arcParticles.match(/export function detectArcFamily[^{]+\{([^}]+)\}/s);
  if (detectArcFamily) {
    const body = detectArcFamily[1];
    // Should be a simple delegation, not a complex implementation
    if (body.length < 200 && body.includes('getArcFamilyUnified')) {
      logOk('detectArcFamily delegates to unified getArcFamily');
    } else if (body.includes('OWNERSHIP') || body.includes('type.startsWith')) {
      logError('detectArcFamily has duplicate logic (should delegate)');
    } else {
      logOk('detectArcFamily is simplified');
    }
  }
}

function validateSidebarUsesUnified() {
  logSection('Sidebar Components Use Unified Palette');

  const sidebarFiles = [
    { path: 'apps/studio/src/components/sidebar/TabbedArcPanel.tsx', name: 'TabbedArcPanel' },
    { path: 'apps/studio/src/components/sidebar/ArcDetailsPanel.tsx', name: 'ArcDetailsPanel' },
    { path: 'apps/studio/src/components/sidebar/TabbedDetailPanel.tsx', name: 'TabbedDetailPanel' },
  ];

  for (const { path, name } of sidebarFiles) {
    const content = loadFile(path);
    if (!content) {
      logError(`${name} not found`);
      continue;
    }

    // Check for unified palette import
    if (content.includes("from '@/design/colors/palette'")) {
      logOk(`${name}: imports from unified palette`);
    } else if (content.includes("from '@/design/nodeColors'")) {
      // This is OK if nodeColors now uses unified
      logOk(`${name}: imports from nodeColors (which uses unified)`);
    } else {
      logWarn(`${name}: may not use unified palette`);
    }
  }
}

function validateColorConsistency() {
  logSection('Color Consistency Check');

  // Load generated.ts to get the authoritative colors
  const generated = loadFile('apps/studio/src/design/colors/generated.ts');
  if (!generated) {
    logError('generated.ts not found');
    return;
  }

  // Extract arc family colors from generated.ts
  const arcFamilyColors = {};
  const arcFamilyMatch = generated.match(/ARC_FAMILY_COLORS[^{]+\{([^}]+)\}/s);
  if (arcFamilyMatch) {
    const families = [...arcFamilyMatch[1].matchAll(/(\w+):\s*\{[^}]*color:\s*['"]([^'"]+)['"]/g)];
    for (const [_, family, color] of families) {
      arcFamilyColors[family] = color.toLowerCase();
    }
  }

  logInfo(`Found ${Object.keys(arcFamilyColors).length} arc family colors in generated.ts`);

  // Verify palette.ts uses these colors
  const palette = loadFile('apps/studio/src/design/colors/palette.ts');
  if (palette) {
    // Check that palette imports ARC_FAMILY_COLORS from generated
    if (palette.includes('ARC_FAMILY_COLORS') && palette.includes("from './generated'")) {
      logOk('palette.ts imports ARC_FAMILY_COLORS from generated.ts');
    } else {
      logError('palette.ts does not properly import ARC_FAMILY_COLORS');
    }
  }

  // Verify arcFamilyPalettes.ts in graph/edges uses same colors
  const edgePalettes = loadFile('apps/studio/src/components/graph/edges/system/arcFamilyPalettes.ts');
  if (edgePalettes) {
    let allMatch = true;
    for (const [family, expectedColor] of Object.entries(arcFamilyColors)) {
      const regex = new RegExp(`${family}:\\s*['"]([^'"]+)['"]`);
      const match = edgePalettes.match(regex);
      if (match) {
        const edgeColor = match[1].toLowerCase();
        if (edgeColor === expectedColor) {
          logOk(`${family}: graph edge color matches (${expectedColor})`);
        } else {
          logError(`${family}: graph edge color ${edgeColor} != generated ${expectedColor}`);
          allMatch = false;
        }
      }
    }
    if (allMatch) {
      logOk('All arc family colors are synchronized');
    }
  }
}

function validateNoLegacyPatterns() {
  logSection('Legacy Pattern Detection');

  const filesToCheck = [
    'apps/studio/src/design/nodeColors.ts',
    'apps/studio/src/lib/graph3d/colorPalette.ts',
    'apps/studio/src/lib/graph3d/arcParticles.ts',
  ];

  for (const filePath of filesToCheck) {
    const content = loadFile(filePath);
    if (!content) continue;

    const fileName = filePath.split('/').pop();

    // Check for hardcoded hex colors in const declarations (outside of defaults)
    const hardcodedCount = (content.match(/#[a-fA-F0-9]{6}/g) || []).length;
    if (hardcodedCount > 5) {
      logWarn(`${fileName}: has ${hardcodedCount} hex colors (may have legacy patterns)`);
    } else {
      logOk(`${fileName}: minimal hardcoded colors (${hardcodedCount})`);
    }

    // Check for old pattern: type.includes('HAS_') for arc detection
    if (content.includes("type.includes('HAS_')") || content.includes("type.startsWith('HAS_')")) {
      // Only error if this is NOT the palette.ts (where the canonical logic lives)
      if (!filePath.includes('palette.ts')) {
        logWarn(`${fileName}: has legacy pattern-based arc detection`);
      }
    }
  }
}

// =============================================================================
// Main
// =============================================================================

async function main() {
  logHeader('NovaNet Unified Color System Validation v11.7.0');

  const palette = validatePaletteExists();
  if (!palette) {
    console.log(`\n${RED}${BOLD}FATAL: Unified palette not found${RESET}\n`);
    process.exit(1);
  }

  validateNodeColorsUsesUnified();
  validateColorPalette3DUsesUnified();
  validateArcParticlesUsesUnified();
  validateSidebarUsesUnified();
  validateColorConsistency();
  validateNoLegacyPatterns();

  // Summary
  logHeader('Validation Summary');

  console.log(`  Total checks: ${BOLD}${checks}${RESET}`);
  console.log(`  ${GREEN}Passed:${RESET}       ${BOLD}${checks - errors}${RESET}`);
  console.log(`  ${RED}Errors:${RESET}       ${BOLD}${errors}${RESET}`);
  console.log(`  ${YELLOW}Warnings:${RESET}     ${BOLD}${warnings}${RESET}`);
  console.log('');

  if (errors === 0 && warnings === 0) {
    console.log(`${GREEN}${BOLD}  ✓ Unified color system is fully synchronized!${RESET}\n`);
    process.exit(0);
  } else if (errors === 0) {
    console.log(`${YELLOW}${BOLD}  ⚠ ${warnings} warning(s), 0 errors${RESET}\n`);
    process.exit(0);
  } else {
    console.log(`${RED}${BOLD}  ✗ ${errors} error(s), ${warnings} warning(s)${RESET}\n`);
    process.exit(1);
  }
}

main().catch(e => {
  console.error(`${RED}FATAL: ${e.message}${RESET}`);
  process.exit(1);
});
