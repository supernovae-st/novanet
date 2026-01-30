/**
 * NodeConfig - Pre-computed Lookup Tables for O(1) Access
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

import type { NodeType } from '@/types';

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
// Size Lookup Table
// =============================================================================

/**
 * Pre-computed sizes for all 29 node types (v7.2.5)
 *
 * Size categories:
 * - Large (280x140): Project root nodes
 * - Medium-Large (240x120): Page, Concept, main content nodes
 * - Medium (220x110): Translations, outputs, prompts
 * - Medium-Small (200x100): Standard nodes
 * - Small (180x90): Auxiliary nodes
 * - Extra-Small (160x80): Mining, snapshot nodes
 */
export const NODE_SIZES: Record<NodeType, NodeSize> = {
  // ==========================================================================
  // PROJECT CATEGORY - Business definition (3 nodes)
  // ==========================================================================
  Project: { width: 280, height: 140 },
  BrandIdentity: { width: 220, height: 110 },
  ProjectL10n: { width: 220, height: 110 },

  // ==========================================================================
  // CONTENT CATEGORY - Semantic structure (5 nodes)
  // ==========================================================================
  Concept: { width: 240, height: 120 },
  ConceptL10n: { width: 200, height: 100 },
  Page: { width: 240, height: 120 },
  Block: { width: 200, height: 100 },
  BlockType: { width: 200, height: 100 },

  // ==========================================================================
  // LOCALE CATEGORY - Locale + knowledge (7 nodes)
  // ==========================================================================
  Locale: { width: 220, height: 110 },
  LocaleIdentity: { width: 180, height: 90 },
  LocaleVoice: { width: 180, height: 90 },
  LocaleCulture: { width: 180, height: 90 },
  LocaleMarket: { width: 180, height: 90 },
  LocaleLexicon: { width: 200, height: 100 },
  Expression: { width: 160, height: 80 },

  // ==========================================================================
  // GENERATION CATEGORY - AI prompts + output (5 nodes)
  // ==========================================================================
  PagePrompt: { width: 200, height: 100 },
  BlockPrompt: { width: 180, height: 90 },
  BlockRules: { width: 180, height: 90 },
  PageOutput: { width: 220, height: 110 },
  BlockOutput: { width: 200, height: 100 },

  // ==========================================================================
  // SEO CATEGORY - Keywords + mining (4 nodes)
  // ==========================================================================
  SEOKeyword: { width: 200, height: 100 },
  SEOVariation: { width: 180, height: 90 },
  SEOSnapshot: { width: 160, height: 80 },
  SEOMiningRun: { width: 160, height: 80 },

  // ==========================================================================
  // GEO CATEGORY - Seeds + mining (4 nodes)
  // ==========================================================================
  GEOSeed: { width: 200, height: 100 },
  GEOReformulation: { width: 180, height: 90 },
  GEOCitation: { width: 160, height: 80 },
  GEOMiningRun: { width: 160, height: 80 },

  // ==========================================================================
  // ANALYTICS CATEGORY - External metrics (1 node)
  // ==========================================================================
  PageMetrics: { width: 180, height: 90 },
};

// =============================================================================
// Color Lookup Table
// =============================================================================

/**
 * Pre-computed colors for all 29 node types (v7.2.5)
 *
 * Color palette aligned with categoryColors.ts and nodeTypes.ts:
 * - Project: Violet (#8b5cf6 family)
 * - Content: Amber/Orange (#f59e0b family)
 * - Locale: Emerald/Green (#10b981 family)
 * - Generation: Blue (#3b82f6 family)
 * - SEO: Red (#ef4444 family)
 * - GEO: Purple (#a855f7 family)
 * - Analytics: Cyan (#06b6d4 family)
 */
export const NODE_COLORS: Record<NodeType, NodeColors> = {
  // ==========================================================================
  // PROJECT CATEGORY - Violet tones
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
  // CONTENT CATEGORY - Amber/Orange tones
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
  BlockType: {
    primary: '#14b8a6',
    secondary: '#10b981',
    tertiary: '#2dd4bf',
    glow: '#14b8a640',
  },

  // ==========================================================================
  // LOCALE CATEGORY - Emerald/Green tones
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
  Expression: {
    primary: '#ec4899',
    secondary: '#f472b6',
    tertiary: '#f9a8d4',
    glow: '#ec489940',
  },

  // ==========================================================================
  // GENERATION CATEGORY - Blue tones
  // ==========================================================================
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
  PageOutput: {
    primary: '#f97316',
    secondary: '#ef4444',
    tertiary: '#fb923c',
    glow: '#f9731640',
  },
  BlockOutput: {
    primary: '#fb923c',
    secondary: '#f97316',
    tertiary: '#fdba74',
    glow: '#fb923c40',
  },

  // ==========================================================================
  // SEO CATEGORY - Red tones
  // ==========================================================================
  SEOKeyword: {
    primary: '#ef4444',
    secondary: '#f87171',
    tertiary: '#fca5a5',
    glow: '#ef444440',
  },
  SEOVariation: {
    primary: '#f87171',
    secondary: '#ef4444',
    tertiary: '#fca5a5',
    glow: '#f8717140',
  },
  SEOSnapshot: {
    primary: '#fca5a5',
    secondary: '#f87171',
    tertiary: '#fecaca',
    glow: '#fca5a540',
  },
  SEOMiningRun: {
    primary: '#fecaca',
    secondary: '#fca5a5',
    tertiary: '#fee2e2',
    glow: '#fecaca40',
  },

  // ==========================================================================
  // GEO CATEGORY - Purple tones
  // ==========================================================================
  GEOSeed: {
    primary: '#a855f7',
    secondary: '#c084fc',
    tertiary: '#d8b4fe',
    glow: '#a855f740',
  },
  GEOReformulation: {
    primary: '#c084fc',
    secondary: '#a855f7',
    tertiary: '#d8b4fe',
    glow: '#c084fc40',
  },
  GEOCitation: {
    primary: '#d8b4fe',
    secondary: '#c084fc',
    tertiary: '#e9d5ff',
    glow: '#d8b4fe40',
  },
  GEOMiningRun: {
    primary: '#e9d5ff',
    secondary: '#d8b4fe',
    tertiary: '#f3e8ff',
    glow: '#e9d5ff40',
  },

  // ==========================================================================
  // ANALYTICS CATEGORY - Cyan tones
  // ==========================================================================
  PageMetrics: {
    primary: '#06b6d4',
    secondary: '#22d3ee',
    tertiary: '#67e8f9',
    glow: '#06b6d440',
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
