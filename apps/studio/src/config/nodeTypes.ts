// =============================================================================
// NODE TYPE CONFIGURATION (v8.1.0)
// =============================================================================
// Visual configuration for all 35 NovaNet node types
// NodeType is imported from @novanet/core (Single Source of Truth)

import type { NodeType } from '@novanet/core/types';
import { NODE_TYPES } from '@novanet/core/types';

// =============================================================================
// NODE CATEGORIES (v8.1.0 - 6 categories, 35 nodes)
// =============================================================================

/**
 * Node category type (6 categories in v8.1.0)
 * Used for grouping nodes in UI and filtering
 */
export type NodeCategory = 'project' | 'content' | 'locale' | 'generation' | 'seo' | 'geo';

/**
 * Node categories with their types (v8.1.0 - 35 nodes across 6 categories)
 * Re-exported for convenience - also available from @/lib/filterAdapter
 */
export const NODE_CATEGORIES: Record<NodeCategory, NodeType[]> = {
  // Project category (3 nodes)
  project: ['Project', 'BrandIdentity', 'ProjectL10n'],
  // Content category (6 nodes)
  content: ['Concept', 'ConceptL10n', 'Page', 'PageType', 'Block', 'BlockType'],
  // Locale category (15 nodes - Locale + 14 LocaleKnowledge)
  locale: [
    'Locale',
    'LocaleIdentity',
    'LocaleVoice',
    'LocaleCulture',
    'LocaleCultureReferences',
    'LocaleMarket',
    'LocaleLexicon',
    'LocaleRulesAdaptation',
    'LocaleRulesFormatting',
    'LocaleRulesSlug',
    'Expression',
    'Reference',
    'Metaphor',
    'Pattern',
    'Constraint',
  ],
  // Generation category (5 nodes)
  generation: ['PagePrompt', 'BlockPrompt', 'BlockRules', 'PageL10n', 'BlockL10n'],
  // SEO category (3 nodes)
  seo: ['SEOKeywordL10n', 'SEOKeywordMetrics', 'SEOMiningRun'],
  // GEO category (3 nodes)
  geo: ['GEOSeedL10n', 'GEOSeedMetrics', 'GEOMiningRun'],
};

// =============================================================================
// NODE TYPE VISUAL CONFIG
// =============================================================================

/**
 * Node type visual configuration
 * Used for rendering in React Flow and force-graph
 */
export interface NodeTypeConfig {
  type: NodeType;
  label: string;
  icon: string;
  color: string;
  colorClass: string;
  size: number;
  category: NodeCategory;
}

/**
 * All node type configurations (v8.1.0 - 35 nodes)
 * Aligned with @novanet/core NODE_TYPES
 */
export const nodeTypeConfigs: Record<NodeType, NodeTypeConfig> = {
  // ==========================================================================
  // PROJECT CATEGORY (3 nodes)
  // ==========================================================================
  Project: {
    type: 'Project',
    label: 'Project',
    icon: '📦',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 24,
    category: 'project',
  },
  BrandIdentity: {
    type: 'BrandIdentity',
    label: 'Brand Identity',
    icon: '🎨',
    color: '#6d28d9',
    colorClass: 'bg-violet-700',
    size: 18,
    category: 'project',
  },
  ProjectL10n: {
    type: 'ProjectL10n',
    label: 'Project L10n',
    icon: '🌐',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 18,
    category: 'project',
  },

  // ==========================================================================
  // CONTENT CATEGORY (6 nodes)
  // ==========================================================================
  Concept: {
    type: 'Concept',
    label: 'Concept',
    icon: '💡',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 20,
    category: 'content',
  },
  ConceptL10n: {
    type: 'ConceptL10n',
    label: 'Concept L10n',
    icon: '💬',
    color: '#fbbf24',
    colorClass: 'bg-yellow-400',
    size: 16,
    category: 'content',
  },
  Page: {
    type: 'Page',
    label: 'Page',
    icon: '📄',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 20,
    category: 'content',
  },
  PageType: {
    type: 'PageType',
    label: 'Page Type',
    icon: '📐',
    color: '#2563eb',
    colorClass: 'bg-blue-600',
    size: 16,
    category: 'content',
  },
  Block: {
    type: 'Block',
    label: 'Block',
    icon: '🧱',
    color: '#06b6d4',
    colorClass: 'bg-cyan-500',
    size: 16,
    category: 'content',
  },
  BlockType: {
    type: 'BlockType',
    label: 'Block Type',
    icon: '📋',
    color: '#14b8a6',
    colorClass: 'bg-teal-500',
    size: 16,
    category: 'content',
  },

  // ==========================================================================
  // LOCALE CATEGORY (15 nodes - Locale + 14 LocaleKnowledge)
  // ==========================================================================
  Locale: {
    type: 'Locale',
    label: 'Locale',
    icon: '🌍',
    color: '#10b981',
    colorClass: 'bg-emerald-500',
    size: 20,
    category: 'locale',
  },
  LocaleIdentity: {
    type: 'LocaleIdentity',
    label: 'Identity',
    icon: '🆔',
    color: '#22c55e',
    colorClass: 'bg-green-500',
    size: 14,
    category: 'locale',
  },
  LocaleVoice: {
    type: 'LocaleVoice',
    label: 'Voice',
    icon: '🎭',
    color: '#4ade80',
    colorClass: 'bg-green-400',
    size: 14,
    category: 'locale',
  },
  LocaleCulture: {
    type: 'LocaleCulture',
    label: 'Culture',
    icon: '🏛️',
    color: '#86efac',
    colorClass: 'bg-green-300',
    size: 14,
    category: 'locale',
  },
  LocaleCultureReferences: {
    type: 'LocaleCultureReferences',
    label: 'Culture Refs',
    icon: '🎭',
    color: '#bbf7d0',
    colorClass: 'bg-green-200',
    size: 12,
    category: 'locale',
  },
  LocaleMarket: {
    type: 'LocaleMarket',
    label: 'Market',
    icon: '📈',
    color: '#6ee7b7',
    colorClass: 'bg-emerald-300',
    size: 14,
    category: 'locale',
  },
  LocaleLexicon: {
    type: 'LocaleLexicon',
    label: 'Lexicon',
    icon: '📚',
    color: '#34d399',
    colorClass: 'bg-emerald-400',
    size: 16,
    category: 'locale',
  },
  LocaleRulesAdaptation: {
    type: 'LocaleRulesAdaptation',
    label: 'Adaptation Rules',
    icon: '🔄',
    color: '#059669',
    colorClass: 'bg-emerald-600',
    size: 12,
    category: 'locale',
  },
  LocaleRulesFormatting: {
    type: 'LocaleRulesFormatting',
    label: 'Formatting Rules',
    icon: '📝',
    color: '#047857',
    colorClass: 'bg-emerald-700',
    size: 12,
    category: 'locale',
  },
  LocaleRulesSlug: {
    type: 'LocaleRulesSlug',
    label: 'Slug Rules',
    icon: '🔗',
    color: '#065f46',
    colorClass: 'bg-emerald-800',
    size: 12,
    category: 'locale',
  },
  Expression: {
    type: 'Expression',
    label: 'Expression',
    icon: '💭',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 10,
    category: 'locale',
  },
  Reference: {
    type: 'Reference',
    label: 'Reference',
    icon: '📍',
    color: '#f472b6',
    colorClass: 'bg-pink-400',
    size: 10,
    category: 'locale',
  },
  Metaphor: {
    type: 'Metaphor',
    label: 'Metaphor',
    icon: '🎨',
    color: '#f9a8d4',
    colorClass: 'bg-pink-300',
    size: 10,
    category: 'locale',
  },
  Pattern: {
    type: 'Pattern',
    label: 'Pattern',
    icon: '🔣',
    color: '#fbcfe8',
    colorClass: 'bg-pink-200',
    size: 10,
    category: 'locale',
  },
  Constraint: {
    type: 'Constraint',
    label: 'Constraint',
    icon: '⚠️',
    color: '#fda4af',
    colorClass: 'bg-rose-300',
    size: 10,
    category: 'locale',
  },

  // ==========================================================================
  // GENERATION CATEGORY (5 nodes)
  // ==========================================================================
  PagePrompt: {
    type: 'PagePrompt',
    label: 'Page Prompt',
    icon: '📝',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 14,
    category: 'generation',
  },
  BlockPrompt: {
    type: 'BlockPrompt',
    label: 'Block Prompt',
    icon: '📝',
    color: '#60a5fa',
    colorClass: 'bg-blue-400',
    size: 12,
    category: 'generation',
  },
  BlockRules: {
    type: 'BlockRules',
    label: 'Block Rules',
    icon: '📏',
    color: '#93c5fd',
    colorClass: 'bg-blue-300',
    size: 12,
    category: 'generation',
  },
  PageL10n: {
    type: 'PageL10n',
    label: 'Page L10n',
    icon: '📃',
    color: '#f97316',
    colorClass: 'bg-orange-500',
    size: 16,
    category: 'generation',
  },
  BlockL10n: {
    type: 'BlockL10n',
    label: 'Block L10n',
    icon: '📝',
    color: '#fb923c',
    colorClass: 'bg-orange-400',
    size: 14,
    category: 'generation',
  },

  // ==========================================================================
  // SEO CATEGORY (3 nodes)
  // ==========================================================================
  SEOKeywordL10n: {
    type: 'SEOKeywordL10n',
    label: 'SEO Keyword',
    icon: '🔍',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 16,
    category: 'seo',
  },
  SEOKeywordMetrics: {
    type: 'SEOKeywordMetrics',
    label: 'SEO Metrics',
    icon: '📊',
    color: '#f87171',
    colorClass: 'bg-red-400',
    size: 10,
    category: 'seo',
  },
  SEOMiningRun: {
    type: 'SEOMiningRun',
    label: 'SEO Mining',
    icon: '⚙️',
    color: '#fca5a5',
    colorClass: 'bg-red-300',
    size: 10,
    category: 'seo',
  },

  // ==========================================================================
  // GEO CATEGORY (3 nodes)
  // ==========================================================================
  GEOSeedL10n: {
    type: 'GEOSeedL10n',
    label: 'GEO Seed',
    icon: '🤖',
    color: '#a855f7',
    colorClass: 'bg-purple-500',
    size: 16,
    category: 'geo',
  },
  GEOSeedMetrics: {
    type: 'GEOSeedMetrics',
    label: 'GEO Metrics',
    icon: '📊',
    color: '#c084fc',
    colorClass: 'bg-purple-400',
    size: 10,
    category: 'geo',
  },
  GEOMiningRun: {
    type: 'GEOMiningRun',
    label: 'GEO Mining',
    icon: '⚙️',
    color: '#d8b4fe',
    colorClass: 'bg-purple-300',
    size: 10,
    category: 'geo',
  },
};

/**
 * Get all node types by category
 */
export function getNodeTypesByCategory(category: NodeTypeConfig['category']): NodeType[] {
  return Object.values(nodeTypeConfigs)
    .filter((config) => config.category === category)
    .map((config) => config.type);
}

/**
 * All node types array (from Core - Single Source of Truth)
 */
export const ALL_NODE_TYPES: readonly NodeType[] = NODE_TYPES;

/**
 * Locale types - Locale + all 14 knowledge nodes
 */
export const LOCALE_TYPES: NodeType[] = getNodeTypesByCategory('locale');

/**
 * Core types for default filter (structure nodes)
 */
export const CORE_TYPES: NodeType[] = [
  'Project',
  'Concept',
  'Page',
  'Block',
  'BlockType',
  'Locale',
];

/**
 * Alias for backwards compatibility
 */
export const NODE_TYPE_CONFIG = nodeTypeConfigs;

/**
 * Category configuration for hierarchical display
 */
export type VisualNodeCategory = NodeTypeConfig['category'];

export interface CategoryConfig {
  id: VisualNodeCategory;
  label: string;
  icon: string;
  color: string;
  colorLight: string;
  nodeTypes: NodeType[];
}

/**
 * All categories with their configuration (ordered for display)
 */
export const NODE_VISUAL_CATEGORIES: CategoryConfig[] = [
  {
    id: 'project',
    label: 'Project',
    icon: '📦',
    color: '#8b5cf6',
    colorLight: '#a78bfa',
    nodeTypes: getNodeTypesByCategory('project'),
  },
  {
    id: 'content',
    label: 'Content',
    icon: '💡',
    color: '#f59e0b',
    colorLight: '#fbbf24',
    nodeTypes: getNodeTypesByCategory('content'),
  },
  {
    id: 'locale',
    label: 'Locale',
    icon: '🌍',
    color: '#10b981',
    colorLight: '#34d399',
    nodeTypes: getNodeTypesByCategory('locale'),
  },
  {
    id: 'generation',
    label: 'Generation',
    icon: '🤖',
    color: '#3b82f6',
    colorLight: '#60a5fa',
    nodeTypes: getNodeTypesByCategory('generation'),
  },
  {
    id: 'seo',
    label: 'SEO',
    icon: '🔍',
    color: '#ef4444',
    colorLight: '#f87171',
    nodeTypes: getNodeTypesByCategory('seo'),
  },
  {
    id: 'geo',
    label: 'GEO',
    icon: '🎯',
    color: '#a855f7',
    colorLight: '#c084fc',
    nodeTypes: getNodeTypesByCategory('geo'),
  },
];

/**
 * Get category config by id
 */
export function getCategoryConfig(categoryId: VisualNodeCategory): CategoryConfig | undefined {
  return NODE_VISUAL_CATEGORIES.find((c) => c.id === categoryId);
}

/**
 * Get category for a node type
 */
export function getCategoryForNodeType(nodeType: NodeType): CategoryConfig | undefined {
  const config = nodeTypeConfigs[nodeType];
  return config ? getCategoryConfig(config.category) : undefined;
}

// =============================================================================
// VALIDATION: Ensure nodeTypeConfigs covers all NODE_TYPES from Core
// =============================================================================

// This will cause a TypeScript error if nodeTypeConfigs is missing any NodeType
const _validateCoverage: Record<NodeType, NodeTypeConfig> = nodeTypeConfigs;
void _validateCoverage; // Prevent unused variable warning
