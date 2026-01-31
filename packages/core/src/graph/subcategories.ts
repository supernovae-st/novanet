// packages/core/src/graph/subcategories.ts
// NODE_SUBCATEGORIES mapping all 35 node types to their subcategories
// AUTO-GENERATED from models/nodes/ folder structure
// Generated: 2026-01-31
// Run: pnpm schema:generate

import type { NodeType } from '../types/nodes.js';
import type { Subcategory } from './types.js';

// =============================================================================
// NODE_SUBCATEGORIES - Maps each NodeType to its subcategory
// =============================================================================

/**
 * Maps each NodeType to its subcategory within its scope.
 * AUTO-GENERATED from models/nodes/ folder structure.
 *
 * Subcategories by scope:
 * - Project: foundation, structure, semantic, instruction, output
 * - Global: config, knowledge
 * - Shared: seo, geo
 */
export const NODE_SUBCATEGORIES: Record<NodeType, Subcategory> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // PROJECT SCOPE (14 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // foundation (3 nodes) - matches models/nodes/project/foundation/
  BrandIdentity: 'foundation',
  Project: 'foundation',
  ProjectL10n: 'foundation',

  // instruction (5 nodes) - matches models/nodes/project/instruction/
  BlockPrompt: 'instruction',
  BlockRules: 'instruction',
  BlockType: 'instruction',
  PagePrompt: 'instruction',
  PageType: 'instruction',

  // output (2 nodes) - matches models/nodes/project/output/
  BlockL10n: 'output',
  PageL10n: 'output',

  // semantic (2 nodes) - matches models/nodes/project/semantic/
  Concept: 'semantic',
  ConceptL10n: 'semantic',

  // structure (2 nodes) - matches models/nodes/project/structure/
  Block: 'structure',
  Page: 'structure',

  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL SCOPE (15 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // config (1 node) - matches models/nodes/global/config/
  Locale: 'config',

  // knowledge (14 nodes) - matches models/nodes/global/knowledge/
  Constraint: 'knowledge',
  Expression: 'knowledge',
  LocaleCulture: 'knowledge',
  LocaleCultureReferences: 'knowledge',
  LocaleIdentity: 'knowledge',
  LocaleLexicon: 'knowledge',
  LocaleMarket: 'knowledge',
  LocaleRulesAdaptation: 'knowledge',
  LocaleRulesFormatting: 'knowledge',
  LocaleRulesSlug: 'knowledge',
  LocaleVoice: 'knowledge',
  Metaphor: 'knowledge',
  Pattern: 'knowledge',
  Reference: 'knowledge',

  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED SCOPE (6 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // geo (3 nodes) - matches models/nodes/shared/geo/
  GEOMiningRun: 'geo',
  GEOSeedL10n: 'geo',
  GEOSeedMetrics: 'geo',

  // seo (3 nodes) - matches models/nodes/shared/seo/
  SEOKeywordL10n: 'seo',
  SEOKeywordMetrics: 'seo',
  SEOMiningRun: 'seo',

};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Get the subcategory for a node type
 * @param nodeType - The node type to look up
 * @returns The subcategory for the node type
 */
export function getSubcategory(nodeType: NodeType): Subcategory {
  return NODE_SUBCATEGORIES[nodeType];
}

/**
 * Get all node types in a subcategory
 * @param subcategory - The subcategory to look up
 * @returns Array of node types belonging to the subcategory
 */
export function getNodeTypesBySubcategory(subcategory: Subcategory): NodeType[] {
  return (Object.entries(NODE_SUBCATEGORIES) as [NodeType, Subcategory][])
    .filter(([, subcat]) => subcat === subcategory)
    .map(([nodeType]) => nodeType);
}
