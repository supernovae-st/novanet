/**
 * NodeConfig - Pre-computed Lookup Tables for O(1) Access
 *
 * Provides instant access to node sizes and colors without runtime computation.
 * This eliminates the performance overhead of computing styles on every render,
 * which is critical when rendering 19k+ nodes.
 *
 * v0.17.2: 57 nodes across 2 realms (36 SHARED + 21 ORG)
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
// Size Lookup Table (v0.17.2: 57 nodes)
// =============================================================================

/**
 * Pre-computed sizes for all 57 node types (v0.17.2)
 *
 * Size categories:
 * - Large (280x140): Root nodes (Project, OrgConfig)
 * - Medium-Large (240x120): Page, Entity, main content nodes
 * - Medium (220x110): Content nodes, instructions
 * - Medium-Small (200x100): Standard nodes
 * - Small (180x90): Auxiliary nodes
 * - Extra-Small (160x80): Knowledge atoms
 */
export const NODE_SIZES: Record<NodeType, NodeSize> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM (36 nodes) — v0.17.2: Term/TermSet/SEOKeywordMetrics/Market removed
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — Locale definition + classifications + SEO format
  EntityCategory: { width: 420, height: 320 },  // v0.13.1: Large "Classification Nexus" card (taller to prevent truncation)
  Locale: { width: 220, height: 110 },
  SEOKeywordFormat: { width: 180, height: 90 },

  // locale (5) — Locale SETTINGS (not the Locale definition itself) — v0.17.2: Market removed
  Formatting: { width: 160, height: 80 },
  Slugification: { width: 160, height: 80 },
  Adaptation: { width: 160, height: 80 },
  Style: { width: 180, height: 90 },
  Culture: { width: 180, height: 90 },

  // geography (7) — v0.12.4: Country added
  Continent: { width: 200, height: 100 },
  Country: { width: 200, height: 100 },
  GeoRegion: { width: 180, height: 90 },
  GeoSubRegion: { width: 180, height: 90 },
  IncomeGroup: { width: 180, height: 90 },
  LendingCategory: { width: 180, height: 90 },
  EconomicRegion: { width: 180, height: 90 },

  // knowledge (21) — Sets + Atoms + Linguistic/Cultural taxonomy + SEO/GEO — v0.17.2: Term/TermSet/SEOKeywordMetrics removed
  ExpressionSet: { width: 160, height: 80 },
  PatternSet: { width: 160, height: 80 },
  CultureSet: { width: 160, height: 80 },
  TabooSet: { width: 160, height: 80 },
  AudienceSet: { width: 160, height: 80 },
  Expression: { width: 160, height: 80 },
  Pattern: { width: 160, height: 80 },
  CultureRef: { width: 160, height: 80 },
  Taboo: { width: 160, height: 80 },
  AudienceTrait: { width: 160, height: 80 },
  LanguageFamily: { width: 180, height: 90 },
  LanguageBranch: { width: 180, height: 90 },
  CulturalRealm: { width: 180, height: 90 },
  CulturalSubRealm: { width: 180, height: 90 },
  PopulationCluster: { width: 180, height: 90 },
  PopulationSubCluster: { width: 180, height: 90 },
  // knowledge — SEO/GEO (5) — v11.5: moved from org to shared/knowledge
  SEOKeyword: { width: 200, height: 100 },
  SEOKeywordSet: { width: 180, height: 90 },
  GEOQuery: { width: 200, height: 100 },
  GEOQuerySet: { width: 180, height: 90 },
  GEOAnswer: { width: 180, height: 90 },

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (21 nodes) — v0.12.4: Brand Architecture (+4), PageStructure/PageInstruction deleted (-2)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1)
  OrgConfig: { width: 280, height: 140 },

  // foundation (8) — v0.17.2: ProjectSEOScope, ProjectGEOScope added
  // v0.13.1: Project "Mission Control" premium card - holographic/matrix effects (widened for content)
  Project: { width: 540, height: 380 },
  Brand: { width: 220, height: 110 },
  BrandDesign: { width: 200, height: 100 },
  BrandPrinciples: { width: 200, height: 100 },
  PromptStyle: { width: 200, height: 100 },
  ProjectNative: { width: 220, height: 110 },
  ProjectSEOScope: { width: 200, height: 100 },
  ProjectGEOScope: { width: 200, height: 100 },

  // structure (3)
  Page: { width: 320, height: 180 },
  Block: { width: 280, height: 160 },
  ContentSlot: { width: 200, height: 100 },

  // semantic (2) — v0.17.2: AudiencePersona/ChannelSurface removed
  Entity: { width: 360, height: 220 },
  EntityNative: { width: 340, height: 200 },

  // instruction (4) — v0.12.4: PageStructure, PageInstruction deleted
  BlockType: { width: 200, height: 100 },
  BlockInstruction: { width: 180, height: 90 },
  BlockRules: { width: 180, height: 90 },
  PromptArtifact: { width: 200, height: 100 },

  // output (3) - v0.13.1: Increased sizes for parent reference indicator
  PageNative: { width: 320, height: 180 },
  BlockNative: { width: 300, height: 170 },
  OutputArtifact: { width: 200, height: 100 },
};

// =============================================================================
// Color Lookup Table (v0.17.2 - 57 nodes)
// =============================================================================

/**
 * Pre-computed colors for all 57 node types (v0.17.2)
 *
 * Color palette by realm and layer:
 * - SHARED config: Emerald/Cyan tones
 * - SHARED locale: Blue tones
 * - SHARED geography: Green tones
 * - SHARED knowledge: Green/Pink/Purple tones (incl. SEO/GEO)
 * - ORG config: Sky blue (#0ea5e9 family)
 * - ORG foundation: Violet (#8b5cf6 family)
 * - ORG structure: Blue/Cyan (#3b82f6, #06b6d4 family)
 * - ORG semantic: Amber (#f59e0b family)
 * - ORG instruction: Blue (#3b82f6 family)
 * - ORG output: Orange/Red (#f97316, #ef4444 family)
 */
export const NODE_COLORS: Record<NodeType, NodeColors> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM (36 nodes) — v0.17.2: Term/TermSet/SEOKeywordMetrics/Market removed
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — Emerald/Cyan tones
  Locale: {
    primary: '#10b981',
    secondary: '#22c55e',
    tertiary: '#34d399',
    glow: '#10b98140',
  },
  EntityCategory: {
    primary: '#0d9488',
    secondary: '#14b8a6',
    tertiary: '#2dd4bf',
    glow: '#0d948840',
  },
  Formatting: {
    primary: '#06b6d4',
    secondary: '#0891b2',
    tertiary: '#22d3ee',
    glow: '#06b6d440',
  },
  Slugification: {
    primary: '#0891b2',
    secondary: '#0e7490',
    tertiary: '#06b6d4',
    glow: '#0891b240',
  },
  Adaptation: {
    primary: '#0e7490',
    secondary: '#155e75',
    tertiary: '#0891b2',
    glow: '#0e749040',
  },
  Style: {
    primary: '#8b5cf6',
    secondary: '#a78bfa',
    tertiary: '#c4b5fd',
    glow: '#8b5cf640',
  },
  Culture: {
    primary: '#d97706',
    secondary: '#b45309',
    tertiary: '#f59e0b',
    glow: '#d9770640',
  },
  // geography (7)
  Continent: {
    primary: '#2563eb',
    secondary: '#3b82f6',
    tertiary: '#60a5fa',
    glow: '#2563eb40',
  },
  Country: {
    primary: '#1d4ed8',
    secondary: '#2563eb',
    tertiary: '#3b82f6',
    glow: '#1d4ed840',
  },
  GeoRegion: {
    primary: '#3b82f6',
    secondary: '#60a5fa',
    tertiary: '#93c5fd',
    glow: '#3b82f640',
  },
  GeoSubRegion: {
    primary: '#60a5fa',
    secondary: '#93c5fd',
    tertiary: '#bfdbfe',
    glow: '#60a5fa40',
  },
  IncomeGroup: {
    primary: '#16a34a',
    secondary: '#22c55e',
    tertiary: '#4ade80',
    glow: '#16a34a40',
  },
  LendingCategory: {
    primary: '#0891b2',
    secondary: '#06b6d4',
    tertiary: '#22d3ee',
    glow: '#0891b240',
  },
  EconomicRegion: {
    primary: '#059669',
    secondary: '#10b981',
    tertiary: '#34d399',
    glow: '#05966940',
  },

  // shared/knowledge (21) — Green/Pink tones + Linguistic/Cultural taxonomy — v0.17.2: Term/TermSet removed
  ExpressionSet: {
    primary: '#ec4899',
    secondary: '#f472b6',
    tertiary: '#f9a8d4',
    glow: '#ec489940',
  },
  PatternSet: {
    primary: '#f472b6',
    secondary: '#ec4899',
    tertiary: '#f9a8d4',
    glow: '#f472b640',
  },
  CultureSet: {
    primary: '#86efac',
    secondary: '#4ade80',
    tertiary: '#bbf7d0',
    glow: '#86efac40',
  },
  TabooSet: {
    primary: '#ef4444',
    secondary: '#f87171',
    tertiary: '#fca5a5',
    glow: '#ef444440',
  },
  AudienceSet: {
    primary: '#f59e0b',
    secondary: '#fbbf24',
    tertiary: '#fcd34d',
    glow: '#f59e0b40',
  },
  Expression: {
    primary: '#f9a8d4',
    secondary: '#f472b6',
    tertiary: '#fbcfe8',
    glow: '#f9a8d440',
  },
  Pattern: {
    primary: '#c4b5fd',
    secondary: '#a78bfa',
    tertiary: '#ddd6fe',
    glow: '#c4b5fd40',
  },
  CultureRef: {
    primary: '#fcd34d',
    secondary: '#fbbf24',
    tertiary: '#fde68a',
    glow: '#fcd34d40',
  },
  Taboo: {
    primary: '#fca5a5',
    secondary: '#f87171',
    tertiary: '#fecaca',
    glow: '#fca5a540',
  },
  AudienceTrait: {
    primary: '#fde68a',
    secondary: '#fcd34d',
    tertiary: '#fef3c7',
    glow: '#fde68a40',
  },
  LanguageFamily: {
    primary: '#7c3aed',
    secondary: '#8b5cf6',
    tertiary: '#a78bfa',
    glow: '#7c3aed40',
  },
  LanguageBranch: {
    primary: '#8b5cf6',
    secondary: '#a78bfa',
    tertiary: '#c4b5fd',
    glow: '#8b5cf640',
  },
  CulturalRealm: {
    primary: '#db2777',
    secondary: '#ec4899',
    tertiary: '#f472b6',
    glow: '#db277740',
  },
  CulturalSubRealm: {
    primary: '#ec4899',
    secondary: '#f472b6',
    tertiary: '#f9a8d4',
    glow: '#ec489940',
  },
  PopulationCluster: {
    primary: '#0284c7',
    secondary: '#0ea5e9',
    tertiary: '#38bdf8',
    glow: '#0284c740',
  },
  PopulationSubCluster: {
    primary: '#0ea5e9',
    secondary: '#38bdf8',
    tertiary: '#7dd3fc',
    glow: '#0ea5e940',
  },

  // seo/geo (5) — Red/Violet tones — v0.17.2: SEOKeywordMetrics removed
  SEOKeyword: {
    primary: '#ef4444',
    secondary: '#f87171',
    tertiary: '#fca5a5',
    glow: '#ef444440',
  },
  SEOKeywordSet: {
    primary: '#dc2626',
    secondary: '#ef4444',
    tertiary: '#f87171',
    glow: '#dc262640',
  },
  // GEO (Generative Engine Optimization) — Blue/Violet tones
  GEOQuery: {
    primary: '#6366f1',
    secondary: '#818cf8',
    tertiary: '#a5b4fc',
    glow: '#6366f140',
  },
  GEOQuerySet: {
    primary: '#4f46e5',
    secondary: '#6366f1',
    tertiary: '#818cf8',
    glow: '#4f46e540',
  },
  GEOAnswer: {
    primary: '#8b5cf6',
    secondary: '#a78bfa',
    tertiary: '#c4b5fd',
    glow: '#8b5cf640',
  },
  // config — SEOKeywordFormat
  SEOKeywordFormat: {
    primary: '#f97316',
    secondary: '#fb923c',
    tertiary: '#fdba74',
    glow: '#f9731640',
  },

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (21 nodes) — v0.12.4: Brand Architecture (+4), PageStructure/PageInstruction deleted (-2)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1) — Sky blue tone
  OrgConfig: {
    primary: '#0ea5e9',
    secondary: '#38bdf8',
    tertiary: '#7dd3fc',
    glow: '#0ea5e940',
  },
  // foundation (8) — Violet tones — v0.17.2: ProjectSEOScope, ProjectGEOScope added
  Project: {
    primary: '#8b5cf6',
    secondary: '#6366f1',
    tertiary: '#a78bfa',
    glow: '#8b5cf640',
  },
  Brand: {
    primary: '#6d28d9',
    secondary: '#7c3aed',
    tertiary: '#8b5cf6',
    glow: '#6d28d940',
  },
  BrandDesign: {
    primary: '#7c3aed',
    secondary: '#8b5cf6',
    tertiary: '#a78bfa',
    glow: '#7c3aed40',
  },
  BrandPrinciples: {
    primary: '#5b21b6',
    secondary: '#6d28d9',
    tertiary: '#7c3aed',
    glow: '#5b21b640',
  },
  PromptStyle: {
    primary: '#4c1d95',
    secondary: '#5b21b6',
    tertiary: '#6d28d9',
    glow: '#4c1d9540',
  },
  ProjectNative: {
    primary: '#a78bfa',
    secondary: '#8b5cf6',
    tertiary: '#c4b5fd',
    glow: '#a78bfa40',
  },
  ProjectSEOScope: {
    primary: '#7e22ce',
    secondary: '#9333ea',
    tertiary: '#a855f7',
    glow: '#7e22ce40',
  },
  ProjectGEOScope: {
    primary: '#9333ea',
    secondary: '#a855f7',
    tertiary: '#c084fc',
    glow: '#9333ea40',
  },

  // structure (3) — Blue/Cyan tones
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

  // semantic (2) — Amber/Gold tones — v0.17.2: AudiencePersona/ChannelSurface removed
  Entity: {
    primary: '#f59e0b',
    secondary: '#f97316',
    tertiary: '#fbbf24',
    glow: '#f59e0b40',
  },
  EntityNative: {
    primary: '#fbbf24',
    secondary: '#f59e0b',
    tertiary: '#fcd34d',
    glow: '#fbbf2440',
  },

  // instruction (4) — Blue tones
  BlockType: {
    primary: '#14b8a6',
    secondary: '#10b981',
    tertiary: '#2dd4bf',
    glow: '#14b8a640',
  },
  BlockInstruction: {
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

  // output (3) — Orange/Red tones — v11.2: job nodes removed
  PageNative: {
    primary: '#f97316',
    secondary: '#ef4444',
    tertiary: '#fb923c',
    glow: '#f9731640',
  },
  BlockNative: {
    primary: '#fb923c',
    secondary: '#f97316',
    tertiary: '#fdba74',
    glow: '#fb923c40',
  },
  OutputArtifact: {
    primary: '#c2410c',
    secondary: '#ea580c',
    tertiary: '#f97316',
    glow: '#c2410c40',
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
