import type { NodeType } from '@/types';

/**
 * Node type configuration
 * Categories aligned with novanet-core v7.2.5:
 * - project: Business definition nodes (3 nodes)
 * - content: Semantic content structure (Concept, Page, Block + ConceptL10n)
 * - locale: Locale + all knowledge nodes
 * - generation: AI prompts + generated output
 * - seo: SEO keywords + mining
 * - geo: GEO seeds + mining
 * - analytics: External metrics (GA/Ahrefs)
 */
export interface NodeTypeConfig {
  type: NodeType;
  label: string;
  icon: string;
  color: string;
  colorClass: string;
  size: number;
  category: 'project' | 'content' | 'locale' | 'generation' | 'seo' | 'geo' | 'analytics';
}

/**
 * All node type configurations (v7.2.5)
 * Aligned with novanet-core/models/_index.yaml
 */
export const nodeTypeConfigs: Record<NodeType, NodeTypeConfig> = {
  // ==========================================================================
  // CATEGORY: PROJECT (📦) - Business definition (3 nodes)
  // Note: Audience merged into ProjectL10n.target_audience (v7.2.5)
  // Note: ValuePropL10n + SocialProofL10n removed (v7.2.5)
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
  // Note: Audience merged into ProjectL10n.target_audience (v7.2.5)
  // Note: ValuePropL10n + SocialProofL10n removed (v7.2.5)
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
  // CATEGORY: CONTENT (💡) - Semantic content structure (5 nodes)
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
    size: 18,
    category: 'content',
  },

  // ==========================================================================
  // CATEGORY: LOCALE (🌍) - Locale + knowledge nodes (7 nodes)
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
  Expression: {
    type: 'Expression',
    label: 'Expression',
    icon: '💭',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 10,
    category: 'locale',
  },

  // ==========================================================================
  // CATEGORY: GENERATION (🤖) - AI prompts + generated output (5 nodes)
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
  PageOutput: {
    type: 'PageOutput',
    label: 'Page Output',
    icon: '📃',
    color: '#f97316',
    colorClass: 'bg-orange-500',
    size: 16,
    category: 'generation',
  },
  BlockOutput: {
    type: 'BlockOutput',
    label: 'Block Output',
    icon: '📝',
    color: '#fb923c',
    colorClass: 'bg-orange-400',
    size: 14,
    category: 'generation',
  },

  // ==========================================================================
  // CATEGORY: SEO (🔍) - SEO keywords + mining (4 nodes)
  // ==========================================================================
  SEOKeyword: {
    type: 'SEOKeyword',
    label: 'SEO Keyword',
    icon: '🔍',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 16,
    category: 'seo',
  },
  SEOVariation: {
    type: 'SEOVariation',
    label: 'SEO Variation',
    icon: '🔀',
    color: '#f87171',
    colorClass: 'bg-red-400',
    size: 12,
    category: 'seo',
  },
  SEOSnapshot: {
    type: 'SEOSnapshot',
    label: 'SEO Snapshot',
    icon: '📸',
    color: '#fca5a5',
    colorClass: 'bg-red-300',
    size: 10,
    category: 'seo',
  },
  SEOMiningRun: {
    type: 'SEOMiningRun',
    label: 'SEO Mining',
    icon: '⚙️',
    color: '#fecaca',
    colorClass: 'bg-red-200',
    size: 10,
    category: 'seo',
  },

  // ==========================================================================
  // CATEGORY: GEO (🎯) - GEO seeds + mining (4 nodes)
  // ==========================================================================
  GEOSeed: {
    type: 'GEOSeed',
    label: 'GEO Seed',
    icon: '🤖',
    color: '#a855f7',
    colorClass: 'bg-purple-500',
    size: 16,
    category: 'geo',
  },
  GEOReformulation: {
    type: 'GEOReformulation',
    label: 'GEO Reformulation',
    icon: '🔄',
    color: '#c084fc',
    colorClass: 'bg-purple-400',
    size: 12,
    category: 'geo',
  },
  GEOCitation: {
    type: 'GEOCitation',
    label: 'GEO Citation',
    icon: '📍',
    color: '#d8b4fe',
    colorClass: 'bg-purple-300',
    size: 10,
    category: 'geo',
  },
  GEOMiningRun: {
    type: 'GEOMiningRun',
    label: 'GEO Mining',
    icon: '⚙️',
    color: '#e9d5ff',
    colorClass: 'bg-purple-200',
    size: 10,
    category: 'geo',
  },

  // ==========================================================================
  // CATEGORY: ANALYTICS (📊) - External metrics (1 node)
  // ==========================================================================
  PageMetrics: {
    type: 'PageMetrics',
    label: 'Page Metrics',
    icon: '📊',
    color: '#93c5fd',
    colorClass: 'bg-blue-300',
    size: 12,
    category: 'analytics',
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
 * All node types array
 */
export const ALL_NODE_TYPES: NodeType[] = Object.keys(nodeTypeConfigs) as NodeType[];

/**
 * Locale types - Locale + knowledge (v7.2.5)
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
export type NodeCategory = NodeTypeConfig['category'];

export interface CategoryConfig {
  id: NodeCategory;
  label: string;
  icon: string;
  color: string;
  colorLight: string;
  nodeTypes: NodeType[];
}

/**
 * All categories with their configuration (ordered for display)
 */
export const NODE_CATEGORIES: CategoryConfig[] = [
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
  {
    id: 'analytics',
    label: 'Analytics',
    icon: '📊',
    color: '#06b6d4',
    colorLight: '#22d3ee',
    nodeTypes: getNodeTypesByCategory('analytics'),
  },
];

/**
 * Get category config by id
 */
export function getCategoryConfig(categoryId: NodeCategory): CategoryConfig | undefined {
  return NODE_CATEGORIES.find((c) => c.id === categoryId);
}

/**
 * Get category for a node type
 */
export function getCategoryForNodeType(nodeType: NodeType): CategoryConfig | undefined {
  const config = nodeTypeConfigs[nodeType];
  return config ? getCategoryConfig(config.category) : undefined;
}
