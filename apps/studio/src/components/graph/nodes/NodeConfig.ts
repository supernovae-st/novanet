/**
 * NodeConfig - Pre-computed Lookup Tables for O(1) Access (v8.1.0)
 *
 * Provides instant access to node sizes and colors without runtime computation.
 * This eliminates the performance overhead of computing styles on every render,
 * which is critical when rendering 19k+ nodes.
 *
 * Performance impact:
 * - Before: O(n) category lookup + color computation per render
 * - After: O(1) direct property access
 *
 * @example
 * // Fast lookup
 * const config = getNodeConfig('Project');
 * const { width, height } = config.size;
 * const { primary, glow } = config.colors;
 */

import type { NodeType } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

export interface NodeSize {
  width: number;
  height: number;
}

export interface NodeColors {
  /** Primary color - used for borders, icons, badges */
  primary: string;
  /** Secondary color - used for gradients */
  secondary: string;
  /** Tertiary color - lighter accent for backgrounds */
  tertiary: string;
  /** Glow color - for shadows and hover effects (with alpha) */
  glow: string;
}

export interface NodeConfig {
  size: NodeSize;
  colors: NodeColors;
}

// =============================================================================
// Size Lookup Table (v9.7.0 - 46 nodes)
// =============================================================================

/**
 * Pre-computed sizes for all 46 node types (v9.7.0)
 *
 * Size categories:
 * - Large (280x140): Project root nodes
 * - Medium-Large (240x120): Page, Concept, main content nodes
 * - Medium (220x110): Core L10n, prompts
 * - Medium-Small (200x100): Standard nodes
 * - Small (180x90): Auxiliary nodes
 * - Extra-Small (160x80): Mining, metrics nodes
 */
export const NODE_SIZES: Record<NodeType, NodeSize> = {
  // ==========================================================================
  // PROJECT — foundation (3 nodes)
  // ==========================================================================
  Project: { width: 280, height: 140 },
  BrandIdentity: { width: 220, height: 110 },
  ProjectL10n: { width: 220, height: 110 },

  // ==========================================================================
  // PROJECT — structure (3 nodes)
  // ==========================================================================
  Page: { width: 240, height: 120 },
  Block: { width: 200, height: 100 },
  ContentSlot: { width: 180, height: 90 },

  // ==========================================================================
  // PROJECT — semantic (6 nodes)
  // ==========================================================================
  Concept: { width: 240, height: 120 },
  ConceptL10n: { width: 200, height: 100 },
  SearchIntent: { width: 200, height: 100 },
  TopicCluster: { width: 220, height: 110 },
  AudiencePersona: { width: 200, height: 100 },
  ChannelSurface: { width: 200, height: 100 },

  // ==========================================================================
  // PROJECT — instruction (6 nodes)
  // ==========================================================================
  PageType: { width: 200, height: 100 },
  BlockType: { width: 200, height: 100 },
  PagePrompt: { width: 200, height: 100 },
  BlockPrompt: { width: 180, height: 90 },
  BlockRules: { width: 180, height: 90 },
  PromptArtifact: { width: 200, height: 100 },

  // ==========================================================================
  // PROJECT — output (5 nodes)
  // ==========================================================================
  PageL10n: { width: 220, height: 110 },
  BlockL10n: { width: 200, height: 100 },
  GenerationJob: { width: 200, height: 100 },
  OutputArtifact: { width: 180, height: 90 },
  EvaluationSignal: { width: 160, height: 80 },

  // ==========================================================================
  // GLOBAL — config (1 node)
  // ==========================================================================
  Locale: { width: 220, height: 110 },

  // ==========================================================================
  // GLOBAL — knowledge (14 nodes)
  // ==========================================================================
  LocaleIdentity: { width: 180, height: 90 },
  LocaleVoice: { width: 180, height: 90 },
  LocaleCulture: { width: 180, height: 90 },
  LocaleCultureReferences: { width: 180, height: 90 },
  LocaleMarket: { width: 180, height: 90 },
  LocaleLexicon: { width: 200, height: 100 },
  LocaleRulesAdaptation: { width: 160, height: 80 },
  LocaleRulesFormatting: { width: 160, height: 80 },
  LocaleRulesSlug: { width: 160, height: 80 },
  Expression: { width: 160, height: 80 },
  Reference: { width: 160, height: 80 },
  Metaphor: { width: 160, height: 80 },
  Pattern: { width: 160, height: 80 },
  Constraint: { width: 160, height: 80 },

  // ==========================================================================
  // SHARED — seo (3 nodes)
  // ==========================================================================
  SEOKeywordL10n: { width: 200, height: 100 },
  SEOKeywordMetrics: { width: 160, height: 80 },
  SEOMiningRun: { width: 160, height: 80 },

  // ==========================================================================
  // SHARED — geo (5 nodes)
  // ==========================================================================
  Thing: { width: 220, height: 110 },
  ThingL10n: { width: 200, height: 100 },
  GEOSeedL10n: { width: 200, height: 100 },
  GEOSeedMetrics: { width: 160, height: 80 },
  GEOMiningRun: { width: 160, height: 80 },
};

// =============================================================================
// Color Lookup Table (v9.7.0 - 46 nodes)
// =============================================================================

/**
 * Pre-computed colors for all 46 node types (v9.7.0)
 *
 * Color palette aligned with design/nodeColors.ts and nodeTypes.ts:
 * - Foundation: Violet (#8b5cf6 family)
 * - Structure: Blue/Cyan (#3b82f6, #06b6d4 family)
 * - Semantic: Amber (#f59e0b family)
 * - Instruction: Blue (#3b82f6 family)
 * - Output: Orange/Red (#f97316, #ef4444 family)
 * - Knowledge: Emerald/Green/Pink (#10b981, #22c55e, #ec4899 family)
 * - SEO: Red (#ef4444 family)
 * - GEO: Purple (#a855f7 family)
 */
export const NODE_COLORS: Record<NodeType, NodeColors> = {
  // ==========================================================================
  // PROJECT — foundation - Violet tones
  // ==========================================================================
  Project: {
    primary: '#8b5cf6',
    secondary: '#6366f1',
    tertiary: '#a78bfa',
    glow: '#8b5cf640',
  },
  BrandIdentity: {
    primary: '#6d28d9',
    secondary: '#7c3aed',
    tertiary: '#8b5cf6',
    glow: '#6d28d940',
  },
  ProjectL10n: {
    primary: '#a78bfa',
    secondary: '#8b5cf6',
    tertiary: '#c4b5fd',
    glow: '#a78bfa40',
  },

  // ==========================================================================
  // PROJECT — structure - Blue/Cyan tones
  // ==========================================================================
  Page: {
    primary: '#3b82f6',
    secondary: '#06b6d4',
    tertiary: '#60a5fa',
    glow: '#3b82f640',
  },
  Block: {
    primary: '#06b6d4',
    secondary: '#14b8a6',
    tertiary: '#22d3ee',
    glow: '#06b6d440',
  },
  ContentSlot: {
    primary: '#0891b2',
    secondary: '#06b6d4',
    tertiary: '#22d3ee',
    glow: '#0891b240',
  },

  // ==========================================================================
  // PROJECT — semantic - Amber tones (6 nodes)
  // ==========================================================================
  Concept: {
    primary: '#f59e0b',
    secondary: '#f97316',
    tertiary: '#fbbf24',
    glow: '#f59e0b40',
  },
  ConceptL10n: {
    primary: '#fbbf24',
    secondary: '#f59e0b',
    tertiary: '#fcd34d',
    glow: '#fbbf2440',
  },
  SearchIntent: {
    primary: '#d97706',
    secondary: '#f59e0b',
    tertiary: '#fbbf24',
    glow: '#d9770640',
  },
  TopicCluster: {
    primary: '#b45309',
    secondary: '#d97706',
    tertiary: '#f59e0b',
    glow: '#b4530940',
  },
  AudiencePersona: {
    primary: '#92400e',
    secondary: '#b45309',
    tertiary: '#d97706',
    glow: '#92400e40',
  },
  ChannelSurface: {
    primary: '#78350f',
    secondary: '#92400e',
    tertiary: '#b45309',
    glow: '#78350f40',
  },

  // ==========================================================================
  // PROJECT — instruction - Blue tones
  // ==========================================================================
  PageType: {
    primary: '#2563eb',
    secondary: '#3b82f6',
    tertiary: '#60a5fa',
    glow: '#2563eb40',
  },
  BlockType: {
    primary: '#14b8a6',
    secondary: '#10b981',
    tertiary: '#2dd4bf',
    glow: '#14b8a640',
  },
  PagePrompt: {
    primary: '#3b82f6',
    secondary: '#60a5fa',
    tertiary: '#93c5fd',
    glow: '#3b82f640',
  },
  BlockPrompt: {
    primary: '#60a5fa',
    secondary: '#3b82f6',
    tertiary: '#93c5fd',
    glow: '#60a5fa40',
  },
  BlockRules: {
    primary: '#93c5fd',
    secondary: '#60a5fa',
    tertiary: '#bfdbfe',
    glow: '#93c5fd40',
  },
  PromptArtifact: {
    primary: '#1d4ed8',
    secondary: '#2563eb',
    tertiary: '#3b82f6',
    glow: '#1d4ed840',
  },

  // ==========================================================================
  // PROJECT — output - Orange/Red tones
  // ==========================================================================
  PageL10n: {
    primary: '#f97316',
    secondary: '#ef4444',
    tertiary: '#fb923c',
    glow: '#f9731640',
  },
  BlockL10n: {
    primary: '#fb923c',
    secondary: '#f97316',
    tertiary: '#fdba74',
    glow: '#fb923c40',
  },
  GenerationJob: {
    primary: '#ea580c',
    secondary: '#f97316',
    tertiary: '#fb923c',
    glow: '#ea580c40',
  },
  OutputArtifact: {
    primary: '#c2410c',
    secondary: '#ea580c',
    tertiary: '#f97316',
    glow: '#c2410c40',
  },
  EvaluationSignal: {
    primary: '#9a3412',
    secondary: '#c2410c',
    tertiary: '#ea580c',
    glow: '#9a341240',
  },

  // ==========================================================================
  // GLOBAL — config - Emerald tones
  // ==========================================================================
  Locale: {
    primary: '#10b981',
    secondary: '#22c55e',
    tertiary: '#34d399',
    glow: '#10b98140',
  },
  LocaleIdentity: {
    primary: '#22c55e',
    secondary: '#10b981',
    tertiary: '#4ade80',
    glow: '#22c55e40',
  },
  LocaleVoice: {
    primary: '#4ade80',
    secondary: '#22c55e',
    tertiary: '#86efac',
    glow: '#4ade8040',
  },
  LocaleCulture: {
    primary: '#86efac',
    secondary: '#4ade80',
    tertiary: '#bbf7d0',
    glow: '#86efac40',
  },
  LocaleCultureReferences: {
    primary: '#bbf7d0',
    secondary: '#86efac',
    tertiary: '#dcfce7',
    glow: '#bbf7d040',
  },
  LocaleMarket: {
    primary: '#6ee7b7',
    secondary: '#34d399',
    tertiary: '#a7f3d0',
    glow: '#6ee7b740',
  },
  LocaleLexicon: {
    primary: '#34d399',
    secondary: '#10b981',
    tertiary: '#6ee7b7',
    glow: '#34d39940',
  },
  LocaleRulesAdaptation: {
    primary: '#059669',
    secondary: '#10b981',
    tertiary: '#34d399',
    glow: '#05966940',
  },
  LocaleRulesFormatting: {
    primary: '#047857',
    secondary: '#059669',
    tertiary: '#10b981',
    glow: '#04785740',
  },
  LocaleRulesSlug: {
    primary: '#065f46',
    secondary: '#047857',
    tertiary: '#059669',
    glow: '#065f4640',
  },
  Expression: {
    primary: '#ec4899',
    secondary: '#f472b6',
    tertiary: '#f9a8d4',
    glow: '#ec489940',
  },
  Reference: {
    primary: '#f472b6',
    secondary: '#ec4899',
    tertiary: '#f9a8d4',
    glow: '#f472b640',
  },
  Metaphor: {
    primary: '#f9a8d4',
    secondary: '#f472b6',
    tertiary: '#fbcfe8',
    glow: '#f9a8d440',
  },
  Pattern: {
    primary: '#fbcfe8',
    secondary: '#f9a8d4',
    tertiary: '#fce7f3',
    glow: '#fbcfe840',
  },
  Constraint: {
    primary: '#fda4af',
    secondary: '#fb7185',
    tertiary: '#fecdd3',
    glow: '#fda4af40',
  },

  // ==========================================================================
  // SHARED — seo - Red tones
  // ==========================================================================
  SEOKeywordL10n: {
    primary: '#ef4444',
    secondary: '#f87171',
    tertiary: '#fca5a5',
    glow: '#ef444440',
  },
  SEOKeywordMetrics: {
    primary: '#f87171',
    secondary: '#ef4444',
    tertiary: '#fca5a5',
    glow: '#f8717140',
  },
  SEOMiningRun: {
    primary: '#fca5a5',
    secondary: '#f87171',
    tertiary: '#fecaca',
    glow: '#fca5a540',
  },

  // ==========================================================================
  // SHARED — geo - Purple tones
  // ==========================================================================
  Thing: {
    primary: '#7c3aed',
    secondary: '#8b5cf6',
    tertiary: '#a78bfa',
    glow: '#7c3aed40',
  },
  ThingL10n: {
    primary: '#8b5cf6',
    secondary: '#a855f7',
    tertiary: '#c084fc',
    glow: '#8b5cf640',
  },
  GEOSeedL10n: {
    primary: '#a855f7',
    secondary: '#c084fc',
    tertiary: '#d8b4fe',
    glow: '#a855f740',
  },
  GEOSeedMetrics: {
    primary: '#c084fc',
    secondary: '#a855f7',
    tertiary: '#d8b4fe',
    glow: '#c084fc40',
  },
  GEOMiningRun: {
    primary: '#d8b4fe',
    secondary: '#c084fc',
    tertiary: '#e9d5ff',
    glow: '#d8b4fe40',
  },
};

// =============================================================================
// Defaults
// =============================================================================

/** Default size for unknown node types */
export const DEFAULT_NODE_SIZE: NodeSize = {
  width: 200,
  height: 100,
};

/** Default colors for unknown node types (indigo) */
export const DEFAULT_NODE_COLORS: NodeColors = {
  primary: '#6366f1',
  secondary: '#8b5cf6',
  tertiary: '#a5b4fc',
  glow: '#6366f140',
};

// =============================================================================
// Config Cache (Memoization)
// =============================================================================

/**
 * Cache for combined config objects.
 * Ensures getNodeConfig returns the same reference for identical types,
 * enabling React's referential equality checks to skip re-renders.
 */
const configCache = new Map<string, NodeConfig>();

/**
 * Get the complete configuration for a node type.
 *
 * Returns a cached object combining size and colors for the given type.
 * For unknown types, returns default configuration.
 *
 * @param type - The node type to get configuration for
 * @returns Combined size and colors configuration
 *
 * @example
 * const config = getNodeConfig('Project');
 * // { size: { width: 280, height: 140 }, colors: { primary: '#8b5cf6', ... } }
 */
export function getNodeConfig(type: NodeType): NodeConfig {
  // Check cache first
  if (configCache.has(type)) {
    return configCache.get(type)!;
  }

  // Check if type exists in lookup tables
  const size = NODE_SIZES[type] ?? DEFAULT_NODE_SIZE;
  const colors = NODE_COLORS[type] ?? DEFAULT_NODE_COLORS;

  const config: NodeConfig = { size, colors };

  // Cache for future lookups
  configCache.set(type, config);

  return config;
}

/**
 * Clear the config cache.
 * Useful for testing or if lookup tables are modified at runtime.
 */
export function clearConfigCache(): void {
  configCache.clear();
}

/**
 * Get cache statistics for monitoring.
 */
export function getConfigCacheStats(): { size: number } {
  return { size: configCache.size };
}

// =============================================================================
// VALIDATION: Ensure lookup tables cover all NodeTypes from Core
// =============================================================================

// TypeScript compile-time validation - will error if any NodeType is missing
const _validateSizes: Record<NodeType, NodeSize> = NODE_SIZES;
const _validateColors: Record<NodeType, NodeColors> = NODE_COLORS;
void _validateSizes;
void _validateColors;
