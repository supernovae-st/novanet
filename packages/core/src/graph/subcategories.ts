// packages/core/src/graph/subcategories.ts
// NODE_SUBCATEGORIES mapping all 35 node types to their subcategories
// v1.0.0

import type { NodeType } from '../types/nodes.js';
import type { Subcategory } from './types.js';

// =============================================================================
// NODE_SUBCATEGORIES - Maps each NodeType to its subcategory
// =============================================================================

/**
 * Maps each NodeType to its subcategory within its scope.
 * This hierarchy matches the structure in models/_index.yaml
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

  // foundation (3 nodes)
  Project: 'foundation',
  BrandIdentity: 'foundation',
  ProjectL10n: 'foundation',

  // structure (4 nodes)
  Page: 'structure',
  Block: 'structure',
  BlockType: 'structure',
  PageType: 'structure',

  // semantic (2 nodes)
  Concept: 'semantic',
  ConceptL10n: 'semantic',

  // instruction (3 nodes)
  PagePrompt: 'instruction',
  BlockPrompt: 'instruction',
  BlockRules: 'instruction',

  // output (2 nodes)
  PageL10n: 'output',
  BlockL10n: 'output',

  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL SCOPE (15 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // config (1 node)
  Locale: 'config',

  // knowledge (14 nodes)
  LocaleIdentity: 'knowledge',
  LocaleVoice: 'knowledge',
  LocaleCulture: 'knowledge',
  LocaleCultureReferences: 'knowledge',
  LocaleMarket: 'knowledge',
  LocaleLexicon: 'knowledge',
  LocaleRulesAdaptation: 'knowledge',
  LocaleRulesFormatting: 'knowledge',
  LocaleRulesSlug: 'knowledge',
  Expression: 'knowledge',
  Reference: 'knowledge',
  Metaphor: 'knowledge',
  Pattern: 'knowledge',
  Constraint: 'knowledge',

  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED SCOPE (6 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // seo (3 nodes)
  SEOKeywordL10n: 'seo',
  SEOKeywordMetrics: 'seo',
  SEOMiningRun: 'seo',

  // geo (3 nodes)
  GEOSeedL10n: 'geo',
  GEOSeedMetrics: 'geo',
  GEOMiningRun: 'geo',
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
