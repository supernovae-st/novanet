#!/usr/bin/env node
/**
 * Design System Validation Script
 *
 * Validates synchronization between:
 * - taxonomy.yaml (source of truth)
 * - visual-encoding.yaml (visual rules)
 * - TypeScript design system (arcFamilyPalettes.ts, themes.ts, registry.ts)
 * - Rust TUI (theme.rs)
 *
 * Run: node tools/scripts/validate-design-system.mjs
 *
 * v0.12.0 - Data Origin trait renames
 */

import { readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { createRequire } from 'module';

// Use require for yaml package (may be in node_modules)
const require = createRequire(import.meta.url);
let parseYaml;
try {
  parseYaml = require('js-yaml').load;
} catch {
  try {
    parseYaml = require('yaml').parse;
  } catch {
    // Fallback: simple YAML parser for our needs
    parseYaml = (content) => {
      // Very basic YAML parser - just enough for validation
      const result = { arc_families: [], terminal: { palette_256: {} }, icons: { arc_families: {} } };

      // Parse arc_families
      const arcFamiliesMatch = content.match(/^arc_families:\s*\n((?:  - .*\n)+)/m);
      if (arcFamiliesMatch) {
        const families = [...arcFamiliesMatch[1].matchAll(/  - key: (\w+)\n(?:.*\n)*?    color: "([^"]+)"/g)];
        result.arc_families = families.map(m => ({ key: m[1], color: m[2] }));
      }

      // Parse terminal palette
      const terminalMatch = content.match(/palette_256:\s*\n((?:    \w+: \d+\n)+)/m);
      if (terminalMatch) {
        const colors = [...terminalMatch[1].matchAll(/    (\w+): (\d+)/g)];
        for (const [_, key, value] of colors) {
          result.terminal.palette_256[key] = parseInt(value);
        }
      }

      return result;
    };
  }
}

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
}

function logError(msg) {
  console.log(`${RED}  ✗${RESET} ${msg}`);
  errors++;
}

function logWarn(msg) {
  console.log(`${YELLOW}  ⚠${RESET} ${msg}`);
  warnings++;
}

function logInfo(msg) {
  console.log(`${CYAN}  ℹ${RESET} ${msg}`);
}

// =============================================================================
// Load Files
// =============================================================================

function loadYaml(relativePath) {
  const fullPath = join(ROOT, relativePath);
  if (!existsSync(fullPath)) {
    logError(`File not found: ${relativePath}`);
    return null;
  }
  try {
    const content = readFileSync(fullPath, 'utf-8');
    return parseYaml(content);
  } catch (e) {
    logError(`Failed to parse YAML: ${relativePath} - ${e.message}`);
    return null;
  }
}

function loadTypeScript(relativePath) {
  const fullPath = join(ROOT, relativePath);
  if (!existsSync(fullPath)) {
    logError(`File not found: ${relativePath}`);
    return null;
  }
  return readFileSync(fullPath, 'utf-8');
}

// =============================================================================
// Validators
// =============================================================================

function validateArcFamilies(taxonomy, arcFamilyPalettesTS, generatedTS) {
  logSection('Arc Families (taxonomy.yaml ↔ TypeScript)');

  const yamlFamilies = taxonomy.arc_families.map(f => f.key);

  // v11.7: Check generated.ts for ARC_FAMILY_COLORS (unified palette system)
  const generatedMatch = generatedTS?.match(/ARC_FAMILY_COLORS:\s*Record<ArcFamilyKey,\s*ColorTokens>\s*=\s*\{([\s\S]*?)\n\};/);
  if (generatedMatch) {
    const tsFamilies = [...generatedMatch[1].matchAll(/'(\w+)':\s*\{/g)].map(m => m[1]);

    for (const family of yamlFamilies) {
      if (tsFamilies.includes(family)) {
        logOk(`${family}: defined in both YAML and generated.ts`);
      } else {
        logError(`${family}: missing in generated.ts ARC_FAMILY_COLORS`);
      }
    }

    for (const family of tsFamilies) {
      if (!yamlFamilies.includes(family)) {
        logWarn(`${family}: defined in generated.ts but not in taxonomy.yaml`);
      }
    }
    return;
  }

  // Fallback: check arcFamilyPalettes.ts for legacy format
  const tsMatch = arcFamilyPalettesTS.match(/ARC_FAMILY_COLORS:\s*Record<ArcFamily,\s*string>\s*=\s*\{([^}]+)\}/s);
  if (!tsMatch) {
    logError('Could not find ARC_FAMILY_COLORS in generated.ts or arcFamilyPalettes.ts');
    return;
  }

  const tsFamilies = [...tsMatch[1].matchAll(/(\w+):\s*['"]#/g)].map(m => m[1]);

  for (const family of yamlFamilies) {
    if (tsFamilies.includes(family)) {
      logOk(`${family}: defined in both YAML and TypeScript`);
    } else {
      logError(`${family}: missing in TypeScript ARC_FAMILY_COLORS`);
    }
  }

  for (const family of tsFamilies) {
    if (!yamlFamilies.includes(family)) {
      logWarn(`${family}: defined in TypeScript but not in taxonomy.yaml`);
    }
  }
}

function validateArcFamilyColors(taxonomy, arcFamilyPalettesTS, generatedTS) {
  logSection('Arc Family Colors (taxonomy.yaml ↔ TypeScript)');

  for (const family of taxonomy.arc_families) {
    const yamlColor = family.color.toLowerCase();

    // v11.7: Check generated.ts first (unified palette system)
    if (generatedTS) {
      const regex = new RegExp(`'${family.key}':\\s*\\{[^}]*color:\\s*'([^']+)'`, 's');
      const genMatch = generatedTS.match(regex);

      if (genMatch) {
        const tsColor = genMatch[1].toLowerCase();
        if (yamlColor === tsColor) {
          logOk(`${family.key}: ${yamlColor} ✓`);
        } else {
          logError(`${family.key}: YAML=${yamlColor}, TS=${tsColor} (mismatch!)`);
        }
        continue;
      }
    }

    // Fallback: check arcFamilyPalettes.ts for legacy format
    const regex = new RegExp(`${family.key}:\\s*['"]([^'"]+)['"]`, 'i');
    const tsMatch = arcFamilyPalettesTS.match(regex);

    if (!tsMatch) {
      logError(`${family.key}: color not found in TypeScript`);
      continue;
    }

    const tsColor = tsMatch[1].toLowerCase();

    if (yamlColor === tsColor) {
      logOk(`${family.key}: ${yamlColor} ✓`);
    } else {
      logError(`${family.key}: YAML=${yamlColor}, TS=${tsColor} (mismatch!)`);
    }
  }
}

function validateArcFamilyEffects(arcFamilyPalettesTS) {
  logSection('Arc Family Effects (v0.12.0)');

  const expectedEffects = {
    ownership: 'energyPulse',
    localization: 'dnaHelix',
    semantic: 'zigzag',
    generation: 'matrixCode',
    mining: 'radarSweep',
  };

  // Extract ARC_FAMILY_EFFECTS from TypeScript (supports quoted or unquoted values)
  const effectsMatch = arcFamilyPalettesTS.match(/ARC_FAMILY_EFFECTS:\s*Record<ArcFamilyKey,\s*ArcFamilyEffect>\s*=\s*\{([^}]+)\}/s);
  if (!effectsMatch) {
    logError('Could not find ARC_FAMILY_EFFECTS in arcFamilyPalettes.ts');
    return;
  }

  for (const [family, expectedEffect] of Object.entries(expectedEffects)) {
    // Match both quoted ('energyPulse') and unquoted (energyPulse) values
    const regex = new RegExp(`${family}:\\s*['"]?([^'",\\s}]+)['"]?`);
    const match = effectsMatch[1].match(regex);

    if (!match) {
      logError(`${family}: effect not defined in ARC_FAMILY_EFFECTS`);
      continue;
    }

    if (match[1] === expectedEffect) {
      logOk(`${family} → ${expectedEffect}`);
    } else {
      logError(`${family}: expected ${expectedEffect}, got ${match[1]}`);
    }
  }
}

function validateEffectPrimitives(typesTS) {
  logSection('Effect Primitives (types.ts)');

  const requiredPrimitives = [
    // Core effects
    'emit', 'particles', 'trail', 'impact', 'glow', 'zigzag', 'interference', 'scanline',
    // Family-specific effects (v0.12.0)
    'energyPulse', 'dnaHelix', 'matrixCode', 'radarSweep',
  ];

  const effectMatch = typesTS.match(/export type EffectPrimitive\s*=([^;]+);/s);
  if (!effectMatch) {
    logError('Could not find EffectPrimitive type in types.ts');
    return;
  }

  const effectType = effectMatch[1];

  for (const primitive of requiredPrimitives) {
    if (effectType.includes(`'${primitive}'`)) {
      logOk(`${primitive}: defined in EffectPrimitive`);
    } else {
      logError(`${primitive}: missing from EffectPrimitive type`);
    }
  }
}

function validateVisualEncodingIcons(visualEncoding) {
  logSection('Visual Encoding Icons (arc_families)');

  const expectedFamilies = ['ownership', 'localization', 'semantic', 'generation', 'mining'];
  const icons = visualEncoding.icons?.arc_families || {};

  for (const family of expectedFamilies) {
    if (icons[family]) {
      const { web, terminal } = icons[family];
      if (web && terminal) {
        logOk(`${family}: web=${web}, terminal=${terminal}`);
      } else {
        logWarn(`${family}: missing web or terminal icon`);
      }
    } else {
      logError(`${family}: not defined in visual-encoding.yaml icons.arc_families`);
    }
  }
}

function validateRegistryImports(registryTS) {
  logSection('Registry Theme Resolution (registry.ts)');

  // Check getArcFamilyEffect is imported
  if (registryTS.includes('getArcFamilyEffect')) {
    logOk('getArcFamilyEffect is imported');
  } else {
    logError('getArcFamilyEffect is NOT imported - arc family effects won\'t be used');
  }

  // Check familyEffect is used in theme resolution
  if (registryTS.includes('familyEffect') && registryTS.includes('effects: [familyEffect')) {
    logOk('Arc family effect is injected as primary effect');
  } else {
    logError('Arc family effect is NOT injected into theme effects');
  }
}

function validateNoEffectsOverrides(themesTS) {
  logSection('Theme Overrides (themes.ts)');

  // Check for overrides that still use effects: DEFAULT_EFFECTS pattern (should be removed)
  const overridesMatch = themesTS.match(/RELATION_OVERRIDES[^{]*\{([^]*)\}\s*as\s*const/s);
  if (!overridesMatch) {
    logWarn('Could not parse RELATION_OVERRIDES');
    return;
  }

  const overrides = overridesMatch[1];

  // Count overrides with explicit effects that might override arc family effects
  const effectOverrides = [...overrides.matchAll(/(\w+):\s*\{[^}]*effects:\s*\[/g)];

  if (effectOverrides.length === 0) {
    logOk('No relation overrides have explicit effects (using arc family effects)');
  } else {
    for (const [_, relation] of effectOverrides) {
      logWarn(`${relation}: has explicit effects override (may override arc family effect)`);
    }
  }
}

function validateTerminalPalette(taxonomy) {
  logSection('Terminal Palette (TUI graceful degradation)');

  const terminal = taxonomy.terminal;
  if (!terminal) {
    logError('No terminal section in taxonomy.yaml');
    return;
  }

  const expectedColors = [
    'ownership_arc', 'localization_arc', 'semantic_arc', 'generation_arc', 'mining_arc'
  ];

  const palette256 = terminal.palette_256 || {};

  for (const colorKey of expectedColors) {
    if (palette256[colorKey] !== undefined) {
      logOk(`${colorKey}: xterm-256 color ${palette256[colorKey]}`);
    } else {
      logWarn(`${colorKey}: not defined in terminal.palette_256`);
    }
  }
}

function validateRustTheme() {
  logSection('Rust TUI Theme (theme.rs)');

  const themeRsPath = 'tools/novanet/src/tui/theme.rs';
  const themeRS = loadTypeScript(themeRsPath);

  if (!themeRS) return;

  // Check arc family module exists
  if (themeRS.includes('pub mod arc_family')) {
    logOk('arc_family module exists');
  } else {
    logError('arc_family module not found');
  }

  // Check all arc families have colors
  const expectedFamilies = ['ownership', 'localization', 'semantic', 'generation', 'mining'];

  for (const family of expectedFamilies) {
    if (themeRS.includes(`"${family}"`)) {
      logOk(`${family}: handled in Rust theme`);
    } else {
      logWarn(`${family}: may not be handled in Rust theme`);
    }
  }
}

// =============================================================================
// Realm, Layer, Trait Validators (v0.12.0)
// =============================================================================

function validateRealms(taxonomy, hierarchyTS) {
  logSection('Realms (taxonomy.yaml ↔ hierarchy.ts)');

  const expectedRealms = ['shared', 'org'];
  const yamlRealms = taxonomy.node_realms?.map(r => r.key) || [];

  // Check YAML has expected realms
  for (const realm of expectedRealms) {
    if (yamlRealms.includes(realm)) {
      logOk(`${realm}: defined in taxonomy.yaml`);
    } else {
      logError(`${realm}: missing from taxonomy.yaml`);
    }
  }

  // Check TypeScript
  for (const realm of expectedRealms) {
    const regex = new RegExp(`^\\s*${realm}:\\s*\\{`, 'm');
    if (regex.test(hierarchyTS)) {
      logOk(`${realm}: defined in hierarchy.ts`);
    } else {
      logError(`${realm}: missing from hierarchy.ts`);
    }
  }
}

function validateLayers(taxonomy, hierarchyTS, layersTS) {
  logSection('Layers (taxonomy.yaml ↔ TypeScript)');

  const sharedLayers = ['config', 'locale', 'geography', 'knowledge'];
  const orgLayers = ['config', 'semantic', 'foundation', 'structure', 'instruction', 'output'];

  // Check YAML structure
  const yamlSharedLayers = taxonomy.node_realms?.find(r => r.key === 'shared')?.layers?.map(l => l.key) || [];
  const yamlOrgLayers = taxonomy.node_realms?.find(r => r.key === 'org')?.layers?.map(l => l.key) || [];

  logInfo(`Shared layers in YAML: ${yamlSharedLayers.length} (expected: ${sharedLayers.length})`);
  for (const layer of sharedLayers) {
    if (yamlSharedLayers.includes(layer)) {
      logOk(`shared/${layer}: defined`);
    } else {
      logError(`shared/${layer}: missing from taxonomy.yaml`);
    }
  }

  logInfo(`Org layers in YAML: ${yamlOrgLayers.length} (expected: ${orgLayers.length})`);
  for (const layer of orgLayers) {
    if (yamlOrgLayers.includes(layer)) {
      logOk(`org/${layer}: defined`);
    } else {
      logError(`org/${layer}: missing from taxonomy.yaml`);
    }
  }

  // Check TypeScript layers.ts has NODE_LAYERS export
  if (layersTS.includes('export const NODE_LAYERS')) {
    logOk('NODE_LAYERS exported from layers.ts');
  } else {
    logError('NODE_LAYERS not found in layers.ts');
  }
}

function validateTraits(taxonomy, typesTS) {
  logSection('Node Traits (taxonomy.yaml ↔ TypeScript)');

  // v0.12.0: Data Origin trait renames
  const expectedTraits = ['defined', 'authored', 'imported', 'generated', 'retrieved'];
  const yamlTraits = taxonomy.node_traits?.map(t => t.key) || [];

  // Check YAML
  for (const trait of expectedTraits) {
    if (yamlTraits.includes(trait)) {
      const traitDef = taxonomy.node_traits.find(t => t.key === trait);
      logOk(`${trait}: border_style=${traitDef.border_style}`);
    } else {
      logError(`${trait}: missing from taxonomy.yaml`);
    }
  }

  // Check TypeScript Trait type (in core types)
  const nodeTypesTS = loadTypeScript('packages/core/src/types/nodes.ts');
  if (nodeTypesTS) {
    // TypeScript uses `Trait` (short form) not `NodeTrait` (full form)
    const traitMatch = nodeTypesTS.match(/export type Trait\s*=([^;]+);/s);
    if (traitMatch) {
      for (const trait of expectedTraits) {
        if (traitMatch[1].includes(`'${trait}'`)) {
          logOk(`${trait}: in Trait type`);
        } else {
          logError(`${trait}: missing from Trait type`);
        }
      }
    } else {
      logError('Trait type not found in nodes.ts');
    }
  }
}

function validateNodeCounts(layersTS) {
  logSection('Node Counts (layers.ts consistency)');

  // v0.12.0: 59 nodes (39 shared + 20 org)
  const expectedCounts = {
    total: 59,
    shared: 39,
    org: 20,
    sharedByLayer: {
      config: 3,
      geography: 6,
      knowledge: 24,
      locale: 6,
    },
    orgByLayer: {
      config: 1,
      foundation: 3,
      instruction: 6,
      output: 3,
      semantic: 4,
      structure: 3,
    },
  };

  // Extract comments from layers.ts
  const sharedMatch = layersTS.match(/SHARED REALM \((\d+) nodes\)/);
  const orgMatch = layersTS.match(/ORG REALM \((\d+) nodes\)/);

  if (sharedMatch) {
    const count = parseInt(sharedMatch[1]);
    if (count === expectedCounts.shared) {
      logOk(`Shared realm: ${count} nodes (matches expected)`);
    } else {
      logError(`Shared realm: ${count} nodes (expected ${expectedCounts.shared})`);
    }
  } else {
    logWarn('Could not parse shared realm node count');
  }

  if (orgMatch) {
    const count = parseInt(orgMatch[1]);
    if (count === expectedCounts.org) {
      logOk(`Org realm: ${count} nodes (matches expected)`);
    } else {
      logError(`Org realm: ${count} nodes (expected ${expectedCounts.org})`);
    }
  } else {
    logWarn('Could not parse org realm node count');
  }

  // Verify layer counts via comments
  for (const [layer, expected] of Object.entries(expectedCounts.sharedByLayer)) {
    const regex = new RegExp(`// ${layer} \\((\\d+) nodes?\\)`, 'i');
    const match = layersTS.match(regex);
    if (match) {
      const count = parseInt(match[1]);
      if (count === expected) {
        logOk(`shared/${layer}: ${count} nodes`);
      } else {
        logError(`shared/${layer}: ${count} nodes (expected ${expected})`);
      }
    }
  }

  for (const [layer, expected] of Object.entries(expectedCounts.orgByLayer)) {
    const regex = new RegExp(`// ${layer} \\((\\d+) nodes?\\)`, 'i');
    // Need to find it in the ORG section
    const orgSection = layersTS.split('ORG REALM')[1] || '';
    const match = orgSection.match(regex);
    if (match) {
      const count = parseInt(match[1]);
      if (count === expected) {
        logOk(`org/${layer}: ${count} nodes`);
      } else {
        logError(`org/${layer}: ${count} nodes (expected ${expected})`);
      }
    }
  }

  // Calculate actual total
  const nodeAssignments = [...layersTS.matchAll(/^\s+(\w+):\s*'(config|locale|geography|knowledge|semantic|foundation|structure|instruction|output)'/gm)];
  logInfo(`Total nodes in NODE_LAYERS: ${nodeAssignments.length}`);
  if (nodeAssignments.length === expectedCounts.total) {
    logOk(`Total: ${nodeAssignments.length} nodes (matches expected)`);
  } else {
    logError(`Total: ${nodeAssignments.length} nodes (expected ${expectedCounts.total})`);
  }
}

function validateVisualEncodingTraits(visualEncoding, taxonomy) {
  logSection('Visual Encoding Trait Icons');

  // v0.12.0: Data Origin trait renames
  const expectedTraits = ['defined', 'authored', 'imported', 'generated', 'retrieved'];
  const traitIcons = visualEncoding.icons?.traits || {};

  for (const trait of expectedTraits) {
    if (traitIcons[trait]) {
      const { web, terminal } = traitIcons[trait];
      if (web && terminal) {
        logOk(`${trait}: web=${web}, terminal=${terminal}`);
      } else {
        logWarn(`${trait}: missing web or terminal icon`);
      }
    } else {
      logError(`${trait}: not defined in visual-encoding.yaml icons.traits`);
    }
  }
}

function validateVisualEncodingLayers(visualEncoding) {
  logSection('Visual Encoding Layer Icons');

  const expectedLayers = ['config', 'locale', 'geography', 'knowledge', 'semantic', 'foundation', 'structure', 'instruction', 'output'];
  const layerIcons = visualEncoding.icons?.layers || {};

  for (const layer of expectedLayers) {
    if (layerIcons[layer]) {
      const { web, terminal } = layerIcons[layer];
      if (web && terminal) {
        logOk(`${layer}: web=${web}, terminal=${terminal}`);
      } else {
        logWarn(`${layer}: missing web or terminal icon`);
      }
    } else {
      logError(`${layer}: not defined in visual-encoding.yaml icons.layers`);
    }
  }
}

// =============================================================================
// Main
// =============================================================================

async function main() {
  logHeader('NovaNet Design System Validation v0.12.0');

  // Load files
  const taxonomy = loadYaml('packages/core/models/taxonomy.yaml');
  const visualEncoding = loadYaml('packages/core/models/visual-encoding.yaml');
  const arcFamilyPalettesTS = loadTypeScript('apps/studio/src/components/graph/edges/system/arcFamilyPalettes.ts');
  const generatedTS = loadTypeScript('apps/studio/src/design/colors/generated.ts');  // v11.7 unified palette
  const typesTS = loadTypeScript('apps/studio/src/components/graph/edges/system/types.ts');
  const registryTS = loadTypeScript('apps/studio/src/components/graph/edges/system/registry.ts');
  const themesTS = loadTypeScript('apps/studio/src/components/graph/edges/system/themes.ts');
  const hierarchyTS = loadTypeScript('packages/core/src/graph/hierarchy.ts');
  const layersTS = loadTypeScript('packages/core/src/graph/layers.ts');

  if (!taxonomy || !visualEncoding || !arcFamilyPalettesTS || !typesTS || !registryTS || !themesTS) {
    console.log(`\n${RED}${BOLD}FATAL: Could not load required files${RESET}\n`);
    process.exit(1);
  }

  // Run validations - Arc System (v0.12.0)
  validateArcFamilies(taxonomy, arcFamilyPalettesTS, generatedTS);
  validateArcFamilyColors(taxonomy, arcFamilyPalettesTS, generatedTS);
  validateArcFamilyEffects(arcFamilyPalettesTS);
  validateEffectPrimitives(typesTS);
  validateVisualEncodingIcons(visualEncoding);
  validateRegistryImports(registryTS);
  validateNoEffectsOverrides(themesTS);
  validateTerminalPalette(taxonomy);
  validateRustTheme();

  // Run validations - Taxonomy Structure (v0.12.0)
  if (hierarchyTS && layersTS) {
    validateRealms(taxonomy, hierarchyTS);
    validateLayers(taxonomy, hierarchyTS, layersTS);
    validateTraits(taxonomy, typesTS);
    validateNodeCounts(layersTS);
    validateVisualEncodingTraits(visualEncoding, taxonomy);
    validateVisualEncodingLayers(visualEncoding);
  } else {
    logWarn('Skipping taxonomy structure validation (missing hierarchy.ts or layers.ts)');
  }

  // Summary
  logHeader('Validation Summary');

  if (errors === 0 && warnings === 0) {
    console.log(`${GREEN}${BOLD}  ✓ All checks passed!${RESET}\n`);
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
