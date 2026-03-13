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
 * v0.12.5 - Load arc families from individual YAML files
 * v0.19.0 - Traits deprecated, node counts updated (59/36/23)
 *           Fallback YAML parser handles individual arc-family files
 */

import { readFileSync, existsSync, readdirSync } from 'fs';
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
      const result = {};

      // Detect individual arc-family file format (arc_family: root key)
      const arcFamilyMatch = content.match(/^arc_family:\s*\n([\s\S]*)/m);
      if (arcFamilyMatch) {
        const block = arcFamilyMatch[1];
        const keyMatch = block.match(/^\s+key:\s*(\S+)/m);
        const colorMatch = block.match(/^\s+color:\s*"([^"]+)"/m);
        const displayMatch = block.match(/^\s+display_name:\s*(\S+)/m);
        result.arc_family = {
          key: keyMatch?.[1],
          color: colorMatch?.[1],
          display_name: displayMatch?.[1],
        };
        return result;
      }

      // Parse arc_families list (legacy taxonomy.yaml format)
      const arcFamiliesMatch = content.match(/^arc_families:\s*\n((?:  - .*\n)+)/m);
      if (arcFamiliesMatch) {
        const families = [...arcFamiliesMatch[1].matchAll(/  - key: (\w+)\n(?:.*\n)*?    color: "([^"]+)"/g)];
        result.arc_families = families.map(m => ({ key: m[1], color: m[2] }));
      }

      // Parse terminal palette
      const terminalMatch = content.match(/palette_256:\s*\n((?:    \w+: \d+\n)+)/m);
      if (terminalMatch) {
        result.terminal = { palette_256: {} };
        const colors = [...terminalMatch[1].matchAll(/    (\w+): (\d+)/g)];
        for (const [_, key, value] of colors) {
          result.terminal.palette_256[key] = parseInt(value);
        }
      }

      // Parse icons section (visual-encoding.yaml)
      if (content.includes('icons:')) {
        result.icons = { arc_families: {} };
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

/**
 * Load arc families from individual YAML files (v0.12.5+)
 * Falls back to taxonomy.arc_families if individual files don't exist
 */
function loadArcFamilies(taxonomy) {
  // First, check if arc_families exists in taxonomy (legacy format)
  if (taxonomy?.arc_families?.length > 0) {
    return taxonomy.arc_families;
  }

  // v0.12.5+: Load from individual files
  const arcFamiliesDir = join(ROOT, 'packages/core/models/arc-families');
  if (!existsSync(arcFamiliesDir)) {
    logError('arc-families directory not found and taxonomy.arc_families is empty');
    return [];
  }

  const arcFamilies = [];
  const files = readdirSync(arcFamiliesDir).filter(f => f.endsWith('.yaml') && !f.startsWith('_'));

  for (const file of files) {
    const content = loadYaml(`packages/core/models/arc-families/${file}`);
    if (content?.arc_family) {
      arcFamilies.push({
        key: content.arc_family.key,
        color: content.arc_family.color,
        display_name: content.arc_family.display_name,
      });
    }
  }

  if (arcFamilies.length === 0) {
    logError('No arc families found in arc-families/*.yaml files');
  } else {
    logInfo(`Loaded ${arcFamilies.length} arc families from individual YAML files`);
  }

  return arcFamilies;
}

/**
 * Load node realms from individual YAML files (v0.12.5+)
 * Falls back to taxonomy.node_realms if individual files don't exist
 */
function loadRealms(taxonomy) {
  // First, check if node_realms exists in taxonomy (legacy format)
  if (taxonomy?.node_realms?.length > 0) {
    return taxonomy.node_realms;
  }

  // v0.12.5+: Load from individual files
  const realmsDir = join(ROOT, 'packages/core/models/realms');
  if (!existsSync(realmsDir)) {
    logWarn('realms directory not found, using expected defaults');
    return [{ key: 'shared', layers: [] }, { key: 'org', layers: [] }];
  }

  const realms = [];
  const files = readdirSync(realmsDir).filter(f => f.endsWith('.yaml') && !f.startsWith('_'));

  for (const file of files) {
    const content = loadYaml(`packages/core/models/realms/${file}`);
    if (content?.realm) {
      realms.push({
        key: content.realm.key,
        layers: content.realm.layers?.map(l => ({ key: l })) || [],
        color: content.realm.color,
      });
    }
  }

  if (realms.length > 0) {
    logInfo(`Loaded ${realms.length} realms from individual YAML files`);
  }

  return realms;
}

/**
 * Load node traits from individual YAML files (v0.12.5+)
 * Falls back to taxonomy.node_traits if individual files don't exist
 */
function loadTraits(taxonomy) {
  // First, check if node_traits exists in taxonomy (legacy format)
  if (taxonomy?.node_traits?.length > 0) {
    return taxonomy.node_traits;
  }

  // v0.12.5+: Load from individual files
  const traitsDir = join(ROOT, 'packages/core/models/traits');
  if (!existsSync(traitsDir)) {
    logWarn('traits directory not found');
    return [];
  }

  const traits = [];
  const files = readdirSync(traitsDir).filter(f => f.endsWith('.yaml') && !f.startsWith('_'));

  for (const file of files) {
    const content = loadYaml(`packages/core/models/traits/${file}`);
    if (content?.trait) {
      traits.push({
        key: content.trait.key,
        border_style: content.trait.border_style,
        color: content.trait.color,
      });
    }
  }

  if (traits.length > 0) {
    logInfo(`Loaded ${traits.length} traits from individual YAML files`);
  }

  return traits;
}

// =============================================================================
// Validators
// =============================================================================

function validateArcFamilies(arcFamilies, arcFamilyPalettesTS, generatedTS) {
  logSection('Arc Families (taxonomy.yaml ↔ TypeScript)');

  const yamlFamilies = arcFamilies.map(f => f.key);

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

function validateArcFamilyColors(arcFamilies, arcFamilyPalettesTS, generatedTS) {
  logSection('Arc Family Colors (taxonomy.yaml ↔ TypeScript)');

  for (const family of arcFamilies) {
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

function validateRealms(realms, hierarchyTS) {
  logSection('Realms (YAML ↔ hierarchy.ts)');

  const expectedRealms = ['shared', 'org'];
  const yamlRealms = realms.map(r => r.key);

  // Check YAML has expected realms
  for (const realm of expectedRealms) {
    if (yamlRealms.includes(realm)) {
      logOk(`${realm}: defined in YAML`);
    } else {
      logError(`${realm}: missing from YAML`);
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

function validateLayers(realms, hierarchyTS, layersTS) {
  logSection('Layers (YAML ↔ TypeScript)');

  const sharedLayers = ['config', 'locale', 'geography', 'knowledge'];
  const orgLayers = ['config', 'semantic', 'foundation', 'structure', 'instruction', 'output'];

  // Check YAML structure (v0.12.5: realms is now the loaded array)
  const sharedRealm = realms.find(r => r.key === 'shared');
  const orgRealm = realms.find(r => r.key === 'org');
  const yamlSharedLayers = sharedRealm?.layers?.map(l => l.key || l) || [];
  const yamlOrgLayers = orgRealm?.layers?.map(l => l.key || l) || [];

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

function validateTraits(traits, typesTS) {
  logSection('Node Traits (v0.19.0: DEPRECATED)');

  // v0.19.0: Traits removed from schema (ADR-024 deprecated)
  // Provenance is now tracked per-instance on nodes that need it
  if (traits.length === 0) {
    logOk('Traits correctly removed from schema (v0.19.0 ADR-024 deprecated)');
    return;
  }

  logWarn(`${traits.length} traits still found — expected 0 after v0.19.0 deprecation`);
}

function validateNodeCounts(layersTS) {
  logSection('Node Counts (layers.ts consistency)');

  // v0.19.0: 59 nodes (36 shared + 23 org)
  const expectedCounts = {
    total: 59,
    shared: 36,
    org: 23,
    sharedByLayer: {
      config: 3,
      geography: 7,
      knowledge: 21,
      locale: 5,
    },
    orgByLayer: {
      config: 1,
      foundation: 8,
      instruction: 3,
      output: 6,
      semantic: 2,
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

function validateVisualEncodingTraits(visualEncoding, traits) {
  logSection('Visual Encoding Trait Icons (v0.19.0: DEPRECATED)');

  // v0.19.0: Traits deprecated — icons may still exist for backward compat
  if (traits.length === 0) {
    logOk('Traits deprecated — skipping visual encoding trait validation');
    return;
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
  logHeader('NovaNet Design System Validation v0.12.5');

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

  // v0.12.5+: Load taxonomy elements from individual YAML files or taxonomy
  const arcFamilies = loadArcFamilies(taxonomy);
  if (arcFamilies.length === 0) {
    console.log(`\n${RED}${BOLD}FATAL: No arc families found${RESET}\n`);
    process.exit(1);
  }

  const realms = loadRealms(taxonomy);
  const traits = loadTraits(taxonomy);

  // Run validations - Arc System (v0.12.0)
  validateArcFamilies(arcFamilies, arcFamilyPalettesTS, generatedTS);
  validateArcFamilyColors(arcFamilies, arcFamilyPalettesTS, generatedTS);
  validateArcFamilyEffects(arcFamilyPalettesTS);
  validateEffectPrimitives(typesTS);
  validateVisualEncodingIcons(visualEncoding);
  validateRegistryImports(registryTS);
  validateNoEffectsOverrides(themesTS);
  validateTerminalPalette(taxonomy);
  validateRustTheme();

  // Run validations - Taxonomy Structure (v0.12.0)
  if (hierarchyTS && layersTS) {
    validateRealms(realms, hierarchyTS);
    validateLayers(realms, hierarchyTS, layersTS);
    validateTraits(traits, typesTS);
    validateNodeCounts(layersTS);
    validateVisualEncodingTraits(visualEncoding, traits);
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
