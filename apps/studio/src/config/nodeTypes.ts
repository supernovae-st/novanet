// =============================================================================
// NODE TYPE CONFIGURATION (v9.0.1)
// =============================================================================
// Visual configuration for all 44 NovaNet node types
// NodeType, Layer, KIND_META imported from @novanet/core (Single Source of Truth)

import type { NodeType, Layer } from '@novanet/core/types';
import { NODE_TYPES, KIND_META } from '@novanet/core/types';

// =============================================================================
// NODE LAYERS (v9.0.1 - 9 layers, 44 nodes)
// Derived from KIND_META — single source of truth
// =============================================================================

/**
 * Node layers with their types (v9 - 44 nodes across 9 layers)
 * Derived from KIND_META in @novanet/core
 */
export const NODE_LAYERS: Record<Layer, NodeType[]> = Object.entries(KIND_META).reduce(
  (acc, [nodeType, meta]) => {
    if (!acc[meta.layer]) acc[meta.layer] = [];
    acc[meta.layer].push(nodeType as NodeType);
    return acc;
  },
  {} as Record<Layer, NodeType[]>
);

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
  layer: Layer;
}

/**
 * All node type configurations (v9.0.1 - 44 nodes)
 * Aligned with @novanet/core NODE_TYPES
 */
export const nodeTypeConfigs: Record<NodeType, NodeTypeConfig> = {
  // ==========================================================================
  // FOUNDATION LAYER (3 nodes)
  // ==========================================================================
  Project: {
    type: 'Project',
    label: 'Project',
    icon: '📦',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 24,
    layer: 'foundation',
  },
  BrandIdentity: {
    type: 'BrandIdentity',
    label: 'Brand Identity',
    icon: '🎨',
    color: '#6d28d9',
    colorClass: 'bg-violet-700',
    size: 18,
    layer: 'foundation',
  },
  ProjectL10n: {
    type: 'ProjectL10n',
    label: 'Project L10n',
    icon: '🌐',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 18,
    layer: 'foundation',
  },

  // ==========================================================================
  // SEMANTIC LAYER (4 nodes)
  // ==========================================================================
  Concept: {
    type: 'Concept',
    label: 'Concept',
    icon: '💡',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 20,
    layer: 'semantic',
  },
  ConceptL10n: {
    type: 'ConceptL10n',
    label: 'Concept L10n',
    icon: '💬',
    color: '#fbbf24',
    colorClass: 'bg-yellow-400',
    size: 16,
    layer: 'semantic',
  },
  SearchIntent: {
    type: 'SearchIntent',
    label: 'Search Intent',
    icon: '🔎',
    color: '#d97706',
    colorClass: 'bg-amber-600',
    size: 16,
    layer: 'semantic',
  },
  TopicCluster: {
    type: 'TopicCluster',
    label: 'Topic Cluster',
    icon: '🗂️',
    color: '#b45309',
    colorClass: 'bg-amber-700',
    size: 18,
    layer: 'semantic',
  },

  // ==========================================================================
  // STRUCTURE LAYER (3 nodes)
  // ==========================================================================
  Page: {
    type: 'Page',
    label: 'Page',
    icon: '📄',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 20,
    layer: 'structure',
  },
  Block: {
    type: 'Block',
    label: 'Block',
    icon: '🧱',
    color: '#06b6d4',
    colorClass: 'bg-cyan-500',
    size: 16,
    layer: 'structure',
  },
  ContentSlot: {
    type: 'ContentSlot',
    label: 'Content Slot',
    icon: '🔲',
    color: '#0891b2',
    colorClass: 'bg-cyan-600',
    size: 14,
    layer: 'structure',
  },

  // ==========================================================================
  // INSTRUCTION LAYER (6 nodes — types + prompts + rules + artifact)
  // ==========================================================================
  PageType: {
    type: 'PageType',
    label: 'Page Type',
    icon: '📐',
    color: '#2563eb',
    colorClass: 'bg-blue-600',
    size: 16,
    layer: 'instruction',
  },
  BlockType: {
    type: 'BlockType',
    label: 'Block Type',
    icon: '📋',
    color: '#14b8a6',
    colorClass: 'bg-teal-500',
    size: 16,
    layer: 'instruction',
  },

  // ==========================================================================
  // CONFIG LAYER (1 node)
  // ==========================================================================
  Locale: {
    type: 'Locale',
    label: 'Locale',
    icon: '🌍',
    color: '#10b981',
    colorClass: 'bg-emerald-500',
    size: 20,
    layer: 'config',
  },

  // ==========================================================================
  // KNOWLEDGE LAYER (14 nodes — Locale Knowledge)
  // ==========================================================================
  LocaleIdentity: {
    type: 'LocaleIdentity',
    label: 'Identity',
    icon: '🆔',
    color: '#22c55e',
    colorClass: 'bg-green-500',
    size: 14,
    layer: 'knowledge',
  },
  LocaleVoice: {
    type: 'LocaleVoice',
    label: 'Voice',
    icon: '🎭',
    color: '#4ade80',
    colorClass: 'bg-green-400',
    size: 14,
    layer: 'knowledge',
  },
  LocaleCulture: {
    type: 'LocaleCulture',
    label: 'Culture',
    icon: '🏛️',
    color: '#86efac',
    colorClass: 'bg-green-300',
    size: 14,
    layer: 'knowledge',
  },
  LocaleCultureReferences: {
    type: 'LocaleCultureReferences',
    label: 'Culture Refs',
    icon: '🎭',
    color: '#bbf7d0',
    colorClass: 'bg-green-200',
    size: 12,
    layer: 'knowledge',
  },
  LocaleMarket: {
    type: 'LocaleMarket',
    label: 'Market',
    icon: '📈',
    color: '#6ee7b7',
    colorClass: 'bg-emerald-300',
    size: 14,
    layer: 'knowledge',
  },
  LocaleLexicon: {
    type: 'LocaleLexicon',
    label: 'Lexicon',
    icon: '📚',
    color: '#34d399',
    colorClass: 'bg-emerald-400',
    size: 16,
    layer: 'knowledge',
  },
  LocaleRulesAdaptation: {
    type: 'LocaleRulesAdaptation',
    label: 'Adaptation Rules',
    icon: '🔄',
    color: '#059669',
    colorClass: 'bg-emerald-600',
    size: 12,
    layer: 'knowledge',
  },
  LocaleRulesFormatting: {
    type: 'LocaleRulesFormatting',
    label: 'Formatting Rules',
    icon: '📝',
    color: '#047857',
    colorClass: 'bg-emerald-700',
    size: 12,
    layer: 'knowledge',
  },
  LocaleRulesSlug: {
    type: 'LocaleRulesSlug',
    label: 'Slug Rules',
    icon: '🔗',
    color: '#065f46',
    colorClass: 'bg-emerald-800',
    size: 12,
    layer: 'knowledge',
  },
  Expression: {
    type: 'Expression',
    label: 'Expression',
    icon: '💭',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 10,
    layer: 'knowledge',
  },
  Reference: {
    type: 'Reference',
    label: 'Reference',
    icon: '📍',
    color: '#f472b6',
    colorClass: 'bg-pink-400',
    size: 10,
    layer: 'knowledge',
  },
  Metaphor: {
    type: 'Metaphor',
    label: 'Metaphor',
    icon: '🎨',
    color: '#f9a8d4',
    colorClass: 'bg-pink-300',
    size: 10,
    layer: 'knowledge',
  },
  Pattern: {
    type: 'Pattern',
    label: 'Pattern',
    icon: '🔣',
    color: '#fbcfe8',
    colorClass: 'bg-pink-200',
    size: 10,
    layer: 'knowledge',
  },
  Constraint: {
    type: 'Constraint',
    label: 'Constraint',
    icon: '⚠️',
    color: '#fda4af',
    colorClass: 'bg-rose-300',
    size: 10,
    layer: 'knowledge',
  },

  // (PagePrompt, BlockPrompt, BlockRules already in INSTRUCTION LAYER above)
  PagePrompt: {
    type: 'PagePrompt',
    label: 'Page Prompt',
    icon: '📝',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 14,
    layer: 'instruction',
  },
  BlockPrompt: {
    type: 'BlockPrompt',
    label: 'Block Prompt',
    icon: '📝',
    color: '#60a5fa',
    colorClass: 'bg-blue-400',
    size: 12,
    layer: 'instruction',
  },
  BlockRules: {
    type: 'BlockRules',
    label: 'Block Rules',
    icon: '📏',
    color: '#93c5fd',
    colorClass: 'bg-blue-300',
    size: 12,
    layer: 'instruction',
  },
  PromptArtifact: {
    type: 'PromptArtifact',
    label: 'Prompt Artifact',
    icon: '📋',
    color: '#bfdbfe',
    colorClass: 'bg-blue-200',
    size: 10,
    layer: 'instruction',
  },

  // ==========================================================================
  // OUTPUT LAYER (5 nodes — LLM-generated content + jobs + signals)
  // ==========================================================================
  PageL10n: {
    type: 'PageL10n',
    label: 'Page L10n',
    icon: '📃',
    color: '#f97316',
    colorClass: 'bg-orange-500',
    size: 16,
    layer: 'output',
  },
  BlockL10n: {
    type: 'BlockL10n',
    label: 'Block L10n',
    icon: '📝',
    color: '#fb923c',
    colorClass: 'bg-orange-400',
    size: 14,
    layer: 'output',
  },
  GenerationJob: {
    type: 'GenerationJob',
    label: 'Generation Job',
    icon: '🚀',
    color: '#ea580c',
    colorClass: 'bg-orange-600',
    size: 16,
    layer: 'output',
  },
  OutputArtifact: {
    type: 'OutputArtifact',
    label: 'Output Artifact',
    icon: '📦',
    color: '#c2410c',
    colorClass: 'bg-orange-700',
    size: 12,
    layer: 'output',
  },
  EvaluationSignal: {
    type: 'EvaluationSignal',
    label: 'Evaluation Signal',
    icon: '📊',
    color: '#9a3412',
    colorClass: 'bg-orange-800',
    size: 10,
    layer: 'output',
  },

  // ==========================================================================
  // SEO LAYER (3 nodes)
  // ==========================================================================
  SEOKeywordL10n: {
    type: 'SEOKeywordL10n',
    label: 'SEO Keyword',
    icon: '🔍',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 16,
    layer: 'seo',
  },
  SEOKeywordMetrics: {
    type: 'SEOKeywordMetrics',
    label: 'SEO Metrics',
    icon: '📊',
    color: '#f87171',
    colorClass: 'bg-red-400',
    size: 10,
    layer: 'seo',
  },
  SEOMiningRun: {
    type: 'SEOMiningRun',
    label: 'SEO Mining',
    icon: '⚙️',
    color: '#fca5a5',
    colorClass: 'bg-red-300',
    size: 10,
    layer: 'seo',
  },

  // ==========================================================================
  // GEO LAYER (5 nodes)
  // ==========================================================================
  Thing: {
    type: 'Thing',
    label: 'Thing',
    icon: '📍',
    color: '#7c3aed',
    colorClass: 'bg-violet-600',
    size: 18,
    layer: 'geo',
  },
  ThingL10n: {
    type: 'ThingL10n',
    label: 'Thing L10n',
    icon: '🌐',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 16,
    layer: 'geo',
  },
  GEOSeedL10n: {
    type: 'GEOSeedL10n',
    label: 'GEO Seed',
    icon: '🤖',
    color: '#a855f7',
    colorClass: 'bg-purple-500',
    size: 16,
    layer: 'geo',
  },
  GEOSeedMetrics: {
    type: 'GEOSeedMetrics',
    label: 'GEO Metrics',
    icon: '📊',
    color: '#c084fc',
    colorClass: 'bg-purple-400',
    size: 10,
    layer: 'geo',
  },
  GEOMiningRun: {
    type: 'GEOMiningRun',
    label: 'GEO Mining',
    icon: '⚙️',
    color: '#d8b4fe',
    colorClass: 'bg-purple-300',
    size: 10,
    layer: 'geo',
  },
};

/**
 * Get all node types by layer
 */
export function getNodeTypesByLayer(layer: Layer): NodeType[] {
  return Object.values(nodeTypeConfigs)
    .filter((config) => config.layer === layer)
    .map((config) => config.type);
}

/**
 * All node types array (from Core - Single Source of Truth)
 */
export const ALL_NODE_TYPES: readonly NodeType[] = NODE_TYPES;

/**
 * Locale types - config (Locale) + all 14 knowledge nodes
 */
export const LOCALE_TYPES: NodeType[] = [
  ...getNodeTypesByLayer('config'),
  ...getNodeTypesByLayer('knowledge'),
];

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
 * Layer configuration for hierarchical display (v9)
 */
export interface LayerConfig {
  id: Layer;
  label: string;
  icon: string;
  color: string;
  colorLight: string;
  nodeTypes: NodeType[];
}

/**
 * All layers with their configuration (ordered for display)
 * Colors from LAYER_COLORS (Solarized palette, ADR-014)
 */
export const NODE_VISUAL_LAYERS: LayerConfig[] = [
  {
    id: 'foundation',
    label: 'Foundation',
    icon: '🏛️',
    color: '#6c71c4',
    colorLight: '#a78bfa',
    nodeTypes: getNodeTypesByLayer('foundation'),
  },
  {
    id: 'structure',
    label: 'Structure',
    icon: '🏗️',
    color: '#859900',
    colorLight: '#a3e635',
    nodeTypes: getNodeTypesByLayer('structure'),
  },
  {
    id: 'semantic',
    label: 'Semantic',
    icon: '💡',
    color: '#b58900',
    colorLight: '#fbbf24',
    nodeTypes: getNodeTypesByLayer('semantic'),
  },
  {
    id: 'instruction',
    label: 'Instruction',
    icon: '📝',
    color: '#d33682',
    colorLight: '#f472b6',
    nodeTypes: getNodeTypesByLayer('instruction'),
  },
  {
    id: 'output',
    label: 'Output',
    icon: '✨',
    color: '#dc322f',
    colorLight: '#f87171',
    nodeTypes: getNodeTypesByLayer('output'),
  },
  {
    id: 'config',
    label: 'Config',
    icon: '⚙️',
    color: '#2aa198',
    colorLight: '#34d399',
    nodeTypes: getNodeTypesByLayer('config'),
  },
  {
    id: 'knowledge',
    label: 'Knowledge',
    icon: '📚',
    color: '#268bd2',
    colorLight: '#60a5fa',
    nodeTypes: getNodeTypesByLayer('knowledge'),
  },
  {
    id: 'seo',
    label: 'SEO',
    icon: '🔍',
    color: '#cb4b16',
    colorLight: '#fb923c',
    nodeTypes: getNodeTypesByLayer('seo'),
  },
  {
    id: 'geo',
    label: 'GEO',
    icon: '📍',
    color: '#93a1a1',
    colorLight: '#c084fc',
    nodeTypes: getNodeTypesByLayer('geo'),
  },
];

/**
 * Get layer config by id
 */
export function getLayerConfig(layerId: Layer): LayerConfig | undefined {
  return NODE_VISUAL_LAYERS.find((c) => c.id === layerId);
}

/**
 * Get layer config for a node type
 */
export function getLayerForNodeType(nodeType: NodeType): LayerConfig | undefined {
  const config = nodeTypeConfigs[nodeType];
  return config ? getLayerConfig(config.layer) : undefined;
}

// =============================================================================
// VALIDATION: Ensure nodeTypeConfigs covers all NODE_TYPES from Core
// =============================================================================

// This will cause a TypeScript error if nodeTypeConfigs is missing any NodeType
const _validateCoverage: Record<NodeType, NodeTypeConfig> = nodeTypeConfigs;
void _validateCoverage; // Prevent unused variable warning
