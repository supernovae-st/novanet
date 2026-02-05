/**
 * Node & Arc Gradient Colors — v9.5.0
 *
 * SVG gradient pairs (primary/secondary) for React Flow node rendering.
 * Organized by visual category: layer, structural, locale-knowledge, arc.
 *
 * Used by:
 * - TurboNode (layer gradients)
 * - StructuralNode (type-specific gradients)
 * - LocaleKnowledgeNode (locale knowledge gradients)
 * - NodeDetailsPanel / ArcDetailsPanel (display colors)
 * - SchemaNode (unified node type lookup)
 *
 * Note: FloatingEdge has its own complex style system with animations,
 * markers, and line styles - kept separate for maintainability.
 *
 * @see layerColors.ts for Tailwind token set (bg/text/border classes)
 */

import type { NodeType } from '@/types';
import type { Layer } from '@novanet/core/types';

// =============================================================================
// Color Types
// =============================================================================

export interface GradientColors {
  primary: string;
  secondary: string;
}

// =============================================================================
// Layer Gradient Colors (used by TurboNode)
// =============================================================================

const LAYER_GRADIENT_COLORS: Record<Layer, GradientColors> = {
  foundation: { primary: '#6c71c4', secondary: '#6366f1' },   // Solarized violet -> Indigo
  structure: { primary: '#859900', secondary: '#a3e635' },     // Solarized green -> Lime
  semantic: { primary: '#b58900', secondary: '#f59e0b' },      // Solarized yellow -> Amber
  instruction: { primary: '#d33682', secondary: '#ec4899' },   // Solarized magenta -> Pink
  output: { primary: '#dc322f', secondary: '#f97316' },        // Solarized red -> Orange
  config: { primary: '#2aa198', secondary: '#14b8a6' },        // Solarized cyan -> Teal
  'locale-knowledge': { primary: '#268bd2', secondary: '#3b82f6' }, // Solarized blue -> Blue
  seo: { primary: '#cb4b16', secondary: '#f97316' },           // Solarized orange -> Orange
};

const DEFAULT_LAYER_COLORS: GradientColors = { primary: '#6366f1', secondary: '#8b5cf6' };

/**
 * Get gradient colors for a layer
 */
export function getLayerGradientColors(layer: Layer | undefined): GradientColors {
  if (!layer) return DEFAULT_LAYER_COLORS;
  return LAYER_GRADIENT_COLORS[layer] ?? DEFAULT_LAYER_COLORS;
}

// =============================================================================
// Structural Node Colors (type-specific)
// =============================================================================

const STRUCTURAL_COLORS: Record<string, GradientColors> = {
  Project: { primary: '#8b5cf6', secondary: '#6366f1' },     // Violet -> Indigo
  Page: { primary: '#3b82f6', secondary: '#06b6d4' },        // Blue -> Cyan
  Block: { primary: '#06b6d4', secondary: '#14b8a6' },       // Cyan -> Teal
  BlockType: { primary: '#14b8a6', secondary: '#10b981' },   // Teal -> Emerald
  Entity: { primary: '#f59e0b', secondary: '#f97316' },      // Amber -> Orange (v10.3 Entity-Centric)
  Locale: { primary: '#10b981', secondary: '#22c55e' },      // Emerald -> Green
  BrandIdentity: { primary: '#6d28d9', secondary: '#7c3aed' }, // Purple
  ProjectL10n: { primary: '#a78bfa', secondary: '#8b5cf6' }, // Light Violet
};

const DEFAULT_STRUCTURAL_COLORS: GradientColors = { primary: '#6366f1', secondary: '#8b5cf6' };

/**
 * Get gradient colors for structural node types
 */
export function getStructuralColors(type: string): GradientColors {
  return STRUCTURAL_COLORS[type] ?? DEFAULT_STRUCTURAL_COLORS;
}

// =============================================================================
// Locale Knowledge Colors
// =============================================================================

// v10 tiered knowledge model colors
const LOCALE_KNOWLEDGE_COLORS: Record<string, GradientColors> = {
  // Technical tier
  Formatting: { primary: '#22c55e', secondary: '#10b981' },      // Green -> Emerald
  Slugification: { primary: '#4ade80', secondary: '#22c55e' },   // Light Green
  Adaptation: { primary: '#86efac', secondary: '#4ade80' },      // Mint -> Light Green
  // Style tier
  Style: { primary: '#6ee7b7', secondary: '#34d399' },           // Emerald tints
  // Semantic tier
  TermSet: { primary: '#34d399', secondary: '#10b981' },         // Teal -> Emerald
  ExpressionSet: { primary: '#ec4899', secondary: '#f472b6' },   // Pink
  PatternSet: { primary: '#a855f7', secondary: '#9333ea' },      // Purple
  CultureSet: { primary: '#f97316', secondary: '#ea580c' },      // Orange
  TabooSet: { primary: '#ef4444', secondary: '#dc2626' },        // Red
  AudienceSet: { primary: '#3b82f6', secondary: '#2563eb' },     // Blue
};

const DEFAULT_LOCALE_KNOWLEDGE_COLORS: GradientColors = { primary: '#10b981', secondary: '#059669' };

/**
 * Get gradient colors for locale knowledge node types
 */
export function getLocaleKnowledgeColors(type: string): GradientColors {
  return LOCALE_KNOWLEDGE_COLORS[type] ?? DEFAULT_LOCALE_KNOWLEDGE_COLORS;
}

// =============================================================================
// Node Type Colors (unified lookup for any node type)
// =============================================================================

/**
 * Get gradient colors for any node type
 * Tries structural colors first, then locale knowledge, then defaults
 */
export function getNodeTypeColors(type: NodeType | string): GradientColors {
  // Check structural colors first
  if (type in STRUCTURAL_COLORS) {
    return STRUCTURAL_COLORS[type];
  }
  // Check locale knowledge colors
  if (type in LOCALE_KNOWLEDGE_COLORS) {
    return LOCALE_KNOWLEDGE_COLORS[type];
  }
  // Default
  return DEFAULT_LAYER_COLORS;
}

// =============================================================================
// Arc Colors (for arc details panel)
// =============================================================================

/**
 * Get gradient colors for arc types (relationships)
 * Used in ArcDetailsPanel for color-coding arcs
 */
export function getRelationColors(type: string): GradientColors {
  // Structural relationships
  if (type.includes('HAS_') || type.includes('CONTAINS')) {
    return { primary: '#3b82f6', secondary: '#06b6d4' }; // Blue -> Cyan
  }

  // Localization
  if (type.includes('FOR_LOCALE') || type.includes('L10N') || type.includes('SUPPORTS')) {
    return { primary: '#10b981', secondary: '#22c55e' }; // Emerald -> Green
  }

  // Output generation
  if (type.includes('OUTPUT') || type.includes('GENERATES') || type.includes('ASSEMBLES')) {
    return { primary: '#f97316', secondary: '#ef4444' }; // Orange -> Red
  }

  // Knowledge
  if (type.includes('IDENTITY') || type.includes('VOICE') || type.includes('CULTURE') || type.includes('MARKET') || type.includes('LEXICON')) {
    return { primary: '#10b981', secondary: '#14b8a6' }; // Emerald -> Teal
  }

  // SEO/GEO
  if (type.includes('SEO') || type.includes('TARGETS')) {
    return { primary: '#ef4444', secondary: '#f87171' }; // Red
  }
  if (type.includes('GEO')) {
    return { primary: '#a855f7', secondary: '#c084fc' }; // Purple
  }

  // Semantic
  if (type.includes('SEMANTIC') || type.includes('INFLUENCED')) {
    return { primary: '#ec4899', secondary: '#f472b6' }; // Pink
  }

  // Fallback
  return { primary: '#6366f1', secondary: '#8b5cf6' }; // Indigo -> Violet
}

// =============================================================================
// Exports for constants (if needed directly)
// =============================================================================

export const COLORS = {
  LAYER: LAYER_GRADIENT_COLORS,
  STRUCTURAL: STRUCTURAL_COLORS,
  LOCALE_KNOWLEDGE: LOCALE_KNOWLEDGE_COLORS,
  DEFAULT: DEFAULT_LAYER_COLORS,
} as const;
