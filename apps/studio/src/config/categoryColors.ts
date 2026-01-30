/**
 * Centralized color configuration for nodes and edges
 *
 * Single source of truth for all color gradients used in:
 * - TurboNode (category colors)
 * - StructuralNode (type colors)
 * - LocaleKnowledgeNode (locale knowledge colors)
 * - NodeDetailsPanel / EdgeDetailsPanel (display colors)
 *
 * Note: FloatingEdge has its own complex style system with animations,
 * markers, and line styles - kept separate for maintainability.
 */

import type { NodeType } from '@/types';
import type { NodeCategory } from '@/lib/filterAdapter';

// =============================================================================
// Color Types
// =============================================================================

export interface GradientColors {
  primary: string;
  secondary: string;
}

// =============================================================================
// Category Colors (used by TurboNode)
// =============================================================================

const CATEGORY_COLORS: Record<NodeCategory, GradientColors> = {
  project: { primary: '#8b5cf6', secondary: '#6366f1' },   // Violet → Indigo
  content: { primary: '#f59e0b', secondary: '#f97316' },   // Amber → Orange
  locale: { primary: '#10b981', secondary: '#22c55e' },    // Emerald → Green
  generation: { primary: '#3b82f6', secondary: '#06b6d4' }, // Blue → Cyan
  seo: { primary: '#ec4899', secondary: '#f43f5e' },       // Pink → Rose
  geo: { primary: '#6366f1', secondary: '#8b5cf6' },       // Indigo → Violet
};

const DEFAULT_CATEGORY_COLORS: GradientColors = { primary: '#6366f1', secondary: '#8b5cf6' };

/**
 * Get gradient colors for a category
 */
export function getCategoryColors(category: NodeCategory | undefined): GradientColors {
  if (!category) return DEFAULT_CATEGORY_COLORS;
  return CATEGORY_COLORS[category] ?? DEFAULT_CATEGORY_COLORS;
}

// =============================================================================
// Structural Node Colors (type-specific)
// =============================================================================

const STRUCTURAL_COLORS: Record<string, GradientColors> = {
  Project: { primary: '#8b5cf6', secondary: '#6366f1' },     // Violet → Indigo
  Page: { primary: '#3b82f6', secondary: '#06b6d4' },        // Blue → Cyan
  Block: { primary: '#06b6d4', secondary: '#14b8a6' },       // Cyan → Teal
  BlockType: { primary: '#14b8a6', secondary: '#10b981' },   // Teal → Emerald
  Concept: { primary: '#f59e0b', secondary: '#f97316' },     // Amber → Orange
  Locale: { primary: '#10b981', secondary: '#22c55e' },      // Emerald → Green
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

const LOCALE_KNOWLEDGE_COLORS: Record<string, GradientColors> = {
  LocaleIdentity: { primary: '#22c55e', secondary: '#10b981' },  // Green → Emerald
  LocaleVoice: { primary: '#4ade80', secondary: '#22c55e' },     // Light Green
  LocaleCulture: { primary: '#86efac', secondary: '#4ade80' },   // Mint → Light Green
  LocaleMarket: { primary: '#6ee7b7', secondary: '#34d399' },    // Emerald tints
  LocaleLexicon: { primary: '#34d399', secondary: '#10b981' },   // Teal → Emerald
  Expression: { primary: '#ec4899', secondary: '#f472b6' },      // Pink
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
  return DEFAULT_CATEGORY_COLORS;
}

// =============================================================================
// Relation/Edge Colors (for edge details panel)
// =============================================================================

/**
 * Get gradient colors for relation/edge types
 * Used in EdgeDetailsPanel for color-coding relationships
 */
export function getRelationColors(type: string): GradientColors {
  // Structural relationships
  if (type.includes('HAS_') || type.includes('CONTAINS')) {
    return { primary: '#3b82f6', secondary: '#06b6d4' }; // Blue → Cyan
  }

  // Localization
  if (type.includes('FOR_LOCALE') || type.includes('L10N') || type.includes('SUPPORTS')) {
    return { primary: '#10b981', secondary: '#22c55e' }; // Emerald → Green
  }

  // Output generation
  if (type.includes('OUTPUT') || type.includes('GENERATES') || type.includes('ASSEMBLES')) {
    return { primary: '#f97316', secondary: '#ef4444' }; // Orange → Red
  }

  // Knowledge
  if (type.includes('IDENTITY') || type.includes('VOICE') || type.includes('CULTURE') || type.includes('MARKET') || type.includes('LEXICON')) {
    return { primary: '#10b981', secondary: '#14b8a6' }; // Emerald → Teal
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
  return { primary: '#6366f1', secondary: '#8b5cf6' }; // Indigo → Violet
}

// =============================================================================
// Exports for constants (if needed directly)
// =============================================================================

export const COLORS = {
  CATEGORY: CATEGORY_COLORS,
  STRUCTURAL: STRUCTURAL_COLORS,
  LOCALE_KNOWLEDGE: LOCALE_KNOWLEDGE_COLORS,
  DEFAULT: DEFAULT_CATEGORY_COLORS,
} as const;
