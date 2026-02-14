// =============================================================================
// CONTEXT VIEW TYPES CONFIGURATION (v11.6)
// =============================================================================
// Defines all 14 context views with their visual styles, effects, and node mappings
//
// NOTE: Views are now defined in packages/core/models/views.yaml (v0.12.5).
// Use @novanet/core/filters ViewLoader.loadRegistry() instead.

import type { NodeType } from '@novanet/core/types';

// =============================================================================
// VIEW TYPE DEFINITIONS
// =============================================================================

export type ViewId =
  | 'composition'
  | 'entities'
  | 'knowledge'
  | 'locales'
  | 'geographic'
  | 'seo-intel'
  | 'geo-intel'
  | 'generation'
  | 'project'
  | 'categories'
  | 'content'
  | 'brand'
  | 'metrics'
  | 'cross-realm';

export type AsciiStyle = 'tree' | 'flow' | 'compact';

export type ViewEffect =
  | 'particles'
  | 'border-beam'
  | 'matrix-rain'
  | 'orbiting-circles'
  | 'globe-pulse'
  | 'ripple'
  | 'radar-sweep'
  | 'meteor'
  | 'shimmer'
  | 'pulse'
  | 'typewriter'
  | 'color-wave'
  | 'chart-rise'
  | 'portal';

export interface ViewTypeConfig {
  id: ViewId;
  label: string;
  icon: string;
  description: string;
  /** Node types this view applies to */
  applicableTo: NodeType[];
  /** ASCII preview style */
  style: AsciiStyle;
  /** Visual effect when view loads */
  effect: ViewEffect;
  /** Matrix transition color (hex) */
  transitionColor: string;
  /** Priority order when multiple views available (lower = higher priority) */
  priority: number;
}

// =============================================================================
// VIEW CONFIGURATIONS (14 Views)
// =============================================================================

export const VIEW_TYPES: Record<ViewId, ViewTypeConfig> = {
  // --------------------------------------------------------------------------
  // TREE STYLE VIEWS (hierarchies)
  // --------------------------------------------------------------------------
  composition: {
    id: 'composition',
    label: 'Composition',
    icon: '📦',
    description: 'Page and Block hierarchy',
    applicableTo: ['Page', 'Block'],
    style: 'tree',
    effect: 'particles',
    transitionColor: '#0ea5e9', // structure blue
    priority: 1,
  },
  knowledge: {
    id: 'knowledge',
    label: 'Knowledge',
    icon: '🧠',
    description: 'Knowledge atoms (Terms, Expressions, Patterns)',
    applicableTo: ['Locale'],
    style: 'tree',
    effect: 'matrix-rain',
    transitionColor: '#22c55e', // knowledge green
    priority: 1,
  },
  geographic: {
    id: 'geographic',
    label: 'Geographic',
    icon: '🗺️',
    description: 'Country and Region hierarchy',
    applicableTo: ['Locale', 'Continent', 'GeoRegion', 'GeoSubRegion'],
    style: 'tree',
    effect: 'globe-pulse',
    transitionColor: '#14b8a6', // geo teal
    priority: 2,
  },
  project: {
    id: 'project',
    label: 'Project',
    icon: '🏗️',
    description: 'Project structure (Pages, Entities)',
    applicableTo: ['Project'],
    style: 'tree',
    effect: 'shimmer',
    transitionColor: '#6366f1', // foundation indigo
    priority: 1,
  },
  brand: {
    id: 'brand',
    label: 'Brand',
    icon: '🎨',
    description: 'Brand identity configuration',
    applicableTo: ['Project', 'Brand'],  // v0.12.4: BrandIdentity → Brand
    style: 'tree',
    effect: 'color-wave',
    transitionColor: '#d946ef', // brand fuchsia
    priority: 3,
  },

  // --------------------------------------------------------------------------
  // FLOW STYLE VIEWS (relations)
  // --------------------------------------------------------------------------
  entities: {
    id: 'entities',
    label: 'Entities',
    icon: '🔗',
    description: 'Connected entities and semantic links',
    applicableTo: ['Page', 'Block', 'Entity'],
    style: 'flow',
    effect: 'border-beam',
    transitionColor: '#06b6d4', // semantic cyan
    priority: 2,
  },
  'seo-intel': {
    id: 'seo-intel',
    label: 'SEO Intel',
    icon: '🎯',
    description: 'SEO keywords and clusters',
    applicableTo: ['Entity', 'Page', 'SEOKeyword', 'SEOKeywordSet'],
    style: 'flow',
    effect: 'ripple',
    transitionColor: '#8b5cf6', // mining purple
    priority: 2,
  },
  'geo-intel': {
    id: 'geo-intel',
    label: 'GEO Intel',
    icon: '🔮',
    description: 'GEO queries and AI answers',
    applicableTo: ['Entity', 'GEOQuery', 'GEOQuerySet', 'GEOAnswer'],
    style: 'flow',
    effect: 'radar-sweep',
    transitionColor: '#a855f7', // geo purple
    priority: 3,
  },
  generation: {
    id: 'generation',
    label: 'Generation',
    icon: '⚡',
    description: 'Generation pipeline and outputs',
    applicableTo: ['Page', 'Block', 'PageGenerated', 'BlockGenerated'],
    style: 'flow',
    effect: 'meteor',
    transitionColor: '#ec4899', // gen pink
    priority: 3,
  },
  categories: {
    id: 'categories',
    label: 'Categories',
    icon: '🏷️',
    description: 'Entity category classification',
    applicableTo: ['Entity', 'EntityCategory'],
    style: 'flow',
    effect: 'pulse',
    transitionColor: '#64748b', // config slate
    priority: 4,
  },
  'cross-realm': {
    id: 'cross-realm',
    label: 'Cross-Realm',
    icon: '🔄',
    description: 'Cross-realm connections (org ↔ shared)',
    applicableTo: [
      'Entity', 'Page', 'Block', 'Locale', 'Project',
      'SEOKeyword', 'GEOQuery', 'EntityCategory',
    ],
    style: 'flow',
    effect: 'portal',
    transitionColor: '#f472b6', // cross pink
    priority: 5,
  },

  // --------------------------------------------------------------------------
  // COMPACT STYLE VIEWS (distributions)
  // --------------------------------------------------------------------------
  locales: {
    id: 'locales',
    label: 'Locales',
    icon: '🌍',
    description: 'Locale coverage and content status',
    applicableTo: ['Page', 'Entity', 'Project', 'Block'],
    style: 'compact',
    effect: 'orbiting-circles',
    transitionColor: '#f59e0b', // locale amber
    priority: 2,
  },
  content: {
    id: 'content',
    label: 'Content',
    icon: '📝',
    description: 'Content per locale',
    applicableTo: ['Entity', 'Project', 'EntityContent', 'ProjectContent'],
    style: 'compact',
    effect: 'typewriter',
    transitionColor: '#fbbf24', // content yellow
    priority: 3,
  },
  metrics: {
    id: 'metrics',
    label: 'Metrics',
    icon: '📊',
    description: 'Performance metrics and analytics',
    applicableTo: ['SEOKeyword', 'SEOKeywordMetrics', 'GEOQuery', 'GEOAnswer'],
    style: 'compact',
    effect: 'chart-rise',
    transitionColor: '#a78bfa', // metrics violet
    priority: 4,
  },
};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Get available views for a node type
 */
export function getViewsForNodeType(nodeType: NodeType): ViewTypeConfig[] {
  return Object.values(VIEW_TYPES)
    .filter((view) => view.applicableTo.includes(nodeType))
    .sort((a, b) => a.priority - b.priority);
}

/**
 * Get view config by ID
 */
export function getViewConfig(viewId: ViewId): ViewTypeConfig | undefined {
  return VIEW_TYPES[viewId];
}

/**
 * All view IDs as array
 */
export const VIEW_IDS = Object.keys(VIEW_TYPES) as ViewId[];

// =============================================================================
// EFFECT COLOR MAP (for Matrix transition)
// =============================================================================

export const EFFECT_COLORS: Record<ViewEffect, string> = {
  particles: '#0ea5e9',
  'border-beam': '#06b6d4',
  'matrix-rain': '#22c55e',
  'orbiting-circles': '#f59e0b',
  'globe-pulse': '#14b8a6',
  ripple: '#8b5cf6',
  'radar-sweep': '#a855f7',
  meteor: '#ec4899',
  shimmer: '#6366f1',
  pulse: '#64748b',
  typewriter: '#fbbf24',
  'color-wave': '#d946ef',
  'chart-rise': '#a78bfa',
  portal: '#f472b6',
};

// =============================================================================
// ASCII SYMBOLS
// =============================================================================

export const ASCII_SYMBOLS = {
  // Tree structure
  branch: '├─',
  lastBranch: '└─',
  vertical: '│',
  indent: '  ',

  // Flow arrows
  arrow: '──►',
  doubleArrow: '══►',
  multiArrow: '──┬──►',
  splitUp: '╦',
  splitMid: '╠',
  splitDown: '╚',

  // Compact status
  complete: '██',
  partial: '░░',
  empty: '○',
  check: '✓',
  progress: ['░', '▒', '▓', '█'],
} as const;
