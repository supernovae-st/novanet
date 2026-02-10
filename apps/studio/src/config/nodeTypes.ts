// =============================================================================
// NODE TYPE CONFIGURATION (v11.5.0)
// =============================================================================
// Visual configuration for all 60 NovaNet node types
// v11.5: SEO/GEO moved to shared/knowledge, 10 layers (4 shared + 6 org), 2 realms
// NodeType, Layer, KIND_META imported from @novanet/core (Single Source of Truth)

import type { NodeType, Layer } from '@novanet/core/types';
import { NODE_TYPES, KIND_META } from '@novanet/core/types';

// =============================================================================
// NODE LAYERS (v11.5 - 10 layers, 60 nodes)
// Derived from KIND_META — single source of truth
// =============================================================================

/**
 * Node layers with their types (v11.5 - 60 nodes across 10 layers)
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
 * All node type configurations (v11.5 - 60 nodes)
 * Aligned with @novanet/core NODE_TYPES
 */
export const nodeTypeConfigs: Record<NodeType, NodeTypeConfig> = {
  // ==========================================================================
  // SHARED REALM — LOCALE LAYER (7 nodes)
  // ==========================================================================
  Locale: {
    type: 'Locale',
    label: 'Locale',
    icon: '🌍',
    color: '#10b981',
    colorClass: 'bg-emerald-500',
    size: 20,
    layer: 'locale',
  },
  Formatting: {
    type: 'Formatting',
    label: 'Formatting',
    icon: '📝',
    color: '#06b6d4',
    colorClass: 'bg-cyan-500',
    size: 12,
    layer: 'locale',
  },
  Slugification: {
    type: 'Slugification',
    label: 'Slugification',
    icon: '🔗',
    color: '#0891b2',
    colorClass: 'bg-cyan-600',
    size: 12,
    layer: 'locale',
  },
  Adaptation: {
    type: 'Adaptation',
    label: 'Adaptation',
    icon: '🔄',
    color: '#0e7490',
    colorClass: 'bg-cyan-700',
    size: 12,
    layer: 'locale',
  },
  Style: {
    type: 'Style',
    label: 'Style',
    icon: '🎭',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 14,
    layer: 'locale',
  },
  Culture: {
    type: 'Culture',
    label: 'Culture',
    icon: '🏺',
    color: '#d97706',
    colorClass: 'bg-amber-600',
    size: 16,
    layer: 'locale',
  },
  Market: {
    type: 'Market',
    label: 'Market',
    icon: '📊',
    color: '#059669',
    colorClass: 'bg-emerald-600',
    size: 16,
    layer: 'locale',
  },

  // ==========================================================================
  // SHARED REALM — GEOGRAPHY LAYER (6 nodes)
  // ==========================================================================
  Continent: {
    type: 'Continent',
    label: 'Continent',
    icon: '🗺️',
    color: '#2563eb',
    colorClass: 'bg-blue-600',
    size: 18,
    layer: 'geography',
  },
  GeoRegion: {
    type: 'GeoRegion',
    label: 'Geo Region',
    icon: '🌐',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 16,
    layer: 'geography',
  },
  GeoSubRegion: {
    type: 'GeoSubRegion',
    label: 'Geo Sub-Region',
    icon: '🗾',
    color: '#60a5fa',
    colorClass: 'bg-blue-400',
    size: 14,
    layer: 'geography',
  },
  IncomeGroup: {
    type: 'IncomeGroup',
    label: 'Income Group',
    icon: '💰',
    color: '#16a34a',
    colorClass: 'bg-green-600',
    size: 14,
    layer: 'geography',
  },
  LendingCategory: {
    type: 'LendingCategory',
    label: 'Lending Category',
    icon: '🏦',
    color: '#0891b2',
    colorClass: 'bg-cyan-600',
    size: 14,
    layer: 'geography',
  },
  EconomicRegion: {
    type: 'EconomicRegion',
    label: 'Economic Region',
    icon: '💹',
    color: '#059669',
    colorClass: 'bg-emerald-600',
    size: 14,
    layer: 'geography',
  },

  // ==========================================================================
  // SHARED REALM — CONFIG LAYER (1 node) — v11.4: classification nodes
  // ==========================================================================
  EntityCategory: {
    type: 'EntityCategory',
    label: 'Entity Category',
    icon: '🏷️',
    color: '#64748b',
    colorClass: 'bg-slate-500',
    size: 16,
    layer: 'config',
  },

  // ==========================================================================
  // SHARED REALM — KNOWLEDGE LAYER (26 nodes) — containers, atoms, SEO/GEO
  // ==========================================================================
  TermSet: {
    type: 'TermSet',
    label: 'Term Set',
    icon: '📚',
    color: '#22c55e',
    colorClass: 'bg-green-500',
    size: 10,
    layer: 'knowledge',
  },
  ExpressionSet: {
    type: 'ExpressionSet',
    label: 'Expression Set',
    icon: '💭',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 10,
    layer: 'knowledge',
  },
  PatternSet: {
    type: 'PatternSet',
    label: 'Pattern Set',
    icon: '🔣',
    color: '#f472b6',
    colorClass: 'bg-pink-400',
    size: 10,
    layer: 'knowledge',
  },
  CultureSet: {
    type: 'CultureSet',
    label: 'Culture Set',
    icon: '🏛️',
    color: '#86efac',
    colorClass: 'bg-green-300',
    size: 10,
    layer: 'knowledge',
  },
  TabooSet: {
    type: 'TabooSet',
    label: 'Taboo Set',
    icon: '⛔',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 10,
    layer: 'knowledge',
  },
  AudienceSet: {
    type: 'AudienceSet',
    label: 'Audience Set',
    icon: '👥',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 10,
    layer: 'knowledge',
  },
  Term: {
    type: 'Term',
    label: 'Term',
    icon: '📖',
    color: '#22c55e',
    colorClass: 'bg-green-500',
    size: 8,
    layer: 'knowledge',
  },
  Expression: {
    type: 'Expression',
    label: 'Expression',
    icon: '💬',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 8,
    layer: 'knowledge',
  },
  Pattern: {
    type: 'Pattern',
    label: 'Pattern',
    icon: '🔣',
    color: '#f472b6',
    colorClass: 'bg-pink-400',
    size: 8,
    layer: 'knowledge',
  },
  CultureRef: {
    type: 'CultureRef',
    label: 'Culture Ref',
    icon: '🌍',
    color: '#86efac',
    colorClass: 'bg-green-300',
    size: 8,
    layer: 'knowledge',
  },
  Taboo: {
    type: 'Taboo',
    label: 'Taboo',
    icon: '🚫',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 8,
    layer: 'knowledge',
  },
  AudienceTrait: {
    type: 'AudienceTrait',
    label: 'Audience Trait',
    icon: '👤',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 8,
    layer: 'knowledge',
  },
  LanguageFamily: {
    type: 'LanguageFamily',
    label: 'Language Family',
    icon: '🗣️',
    color: '#7c3aed',
    colorClass: 'bg-violet-600',
    size: 14,
    layer: 'knowledge',
  },
  LanguageBranch: {
    type: 'LanguageBranch',
    label: 'Language Branch',
    icon: '🌿',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 12,
    layer: 'knowledge',
  },
  CulturalRealm: {
    type: 'CulturalRealm',
    label: 'Cultural Realm',
    icon: '🎪',
    color: '#db2777',
    colorClass: 'bg-pink-600',
    size: 14,
    layer: 'knowledge',
  },
  CulturalSubRealm: {
    type: 'CulturalSubRealm',
    label: 'Cultural Sub-Realm',
    icon: '🎭',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 12,
    layer: 'knowledge',
  },
  PopulationCluster: {
    type: 'PopulationCluster',
    label: 'Population Cluster',
    icon: '👨‍👩‍👧‍👦',
    color: '#0284c7',
    colorClass: 'bg-sky-600',
    size: 14,
    layer: 'knowledge',
  },
  PopulationSubCluster: {
    type: 'PopulationSubCluster',
    label: 'Population Sub-Cluster',
    icon: '👥',
    color: '#0ea5e9',
    colorClass: 'bg-sky-500',
    size: 12,
    layer: 'knowledge',
  },

  // ==========================================================================
  // ORG REALM — CONFIG LAYER (1 node) — v11.3: Organization + Tenant merged
  // ==========================================================================
  OrgConfig: {
    type: 'OrgConfig',
    label: 'Org Config',
    icon: '🏢',
    color: '#0ea5e9',
    colorClass: 'bg-sky-500',
    size: 22,
    layer: 'config',
  },

  // ==========================================================================
  // ORG REALM — FOUNDATION LAYER (3 nodes)
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
  ProjectContent: {
    type: 'ProjectContent',
    label: 'Project Content',
    icon: '🌐',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 18,
    layer: 'foundation',
  },

  // ==========================================================================
  // ORG REALM — STRUCTURE LAYER (3 nodes)
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
  // ORG REALM — SEMANTIC LAYER (4 nodes)
  // ==========================================================================
  Entity: {
    type: 'Entity',
    label: 'Entity',
    icon: '💡',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 20,
    layer: 'semantic',
  },
  EntityContent: {
    type: 'EntityContent',
    label: 'Entity Content',
    icon: '💬',
    color: '#fbbf24',
    colorClass: 'bg-yellow-400',
    size: 16,
    layer: 'semantic',
  },
  AudiencePersona: {
    type: 'AudiencePersona',
    label: 'Audience Persona',
    icon: '👤',
    color: '#92400e',
    colorClass: 'bg-amber-800',
    size: 16,
    layer: 'semantic',
  },
  ChannelSurface: {
    type: 'ChannelSurface',
    label: 'Channel Surface',
    icon: '📡',
    color: '#78350f',
    colorClass: 'bg-amber-900',
    size: 16,
    layer: 'semantic',
  },

  // ==========================================================================
  // ORG REALM — INSTRUCTION LAYER (7 nodes)
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
  BlockInstruction: {
    type: 'BlockInstruction',
    label: 'Block Instruction',
    icon: '📋',
    color: '#7dd3fc',
    colorClass: 'bg-sky-300',
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
  // SHARED REALM — CONFIG LAYER — SEOKeywordFormat (1 node)
  // v11.5: Classification config node
  // ==========================================================================
  SEOKeywordFormat: {
    type: 'SEOKeywordFormat',
    label: 'SEO Keyword Format',
    icon: '📋',
    color: '#64748b',
    colorClass: 'bg-slate-500',
    size: 14,
    layer: 'config',
  },

  // ==========================================================================
  // SHARED REALM — KNOWLEDGE LAYER — SEO nodes (3 nodes)
  // v11.5: SEOKeyword, SEOKeywordMetrics, SEOKeywordSet
  // ==========================================================================
  SEOKeyword: {
    type: 'SEOKeyword',
    label: 'SEO Keyword',
    icon: '🔍',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 16,
    layer: 'knowledge',
  },
  SEOKeywordMetrics: {
    type: 'SEOKeywordMetrics',
    label: 'SEO Metrics',
    icon: '📊',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 10,
    layer: 'knowledge',
  },
  SEOKeywordSet: {
    type: 'SEOKeywordSet',
    label: 'SEO Keyword Set',
    icon: '📑',
    color: '#7c3aed',
    colorClass: 'bg-violet-600',
    size: 12,
    layer: 'knowledge',
  },

  // ==========================================================================
  // SHARED REALM — KNOWLEDGE LAYER — GEO nodes (3 nodes)
  // v11.5: GEOQuery, GEOQuerySet, GEOAnswer
  // ==========================================================================
  GEOQuery: {
    type: 'GEOQuery',
    label: 'GEO Query',
    icon: '🤖',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 16,
    layer: 'knowledge',
  },
  GEOQuerySet: {
    type: 'GEOQuerySet',
    label: 'GEO Query Set',
    icon: '🗃️',
    color: '#6d28d9',
    colorClass: 'bg-violet-700',
    size: 12,
    layer: 'knowledge',
  },
  GEOAnswer: {
    type: 'GEOAnswer',
    label: 'GEO Answer',
    icon: '💬',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 14,
    layer: 'knowledge',
  },

  // ==========================================================================
  // ORG REALM — OUTPUT LAYER (3 nodes)
  // ==========================================================================
  PageGenerated: {
    type: 'PageGenerated',
    label: 'Page Generated',
    icon: '📃',
    color: '#f97316',
    colorClass: 'bg-orange-500',
    size: 16,
    layer: 'output',
  },
  BlockGenerated: {
    type: 'BlockGenerated',
    label: 'Block Generated',
    icon: '📝',
    color: '#fb923c',
    colorClass: 'bg-orange-400',
    size: 14,
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
 * Core types for default filter (structure nodes) - v10.3 Entity-Centric
 */
export const CORE_TYPES: NodeType[] = [
  'Project',
  'Entity',
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
 * Layer configuration for hierarchical display (v11.3)
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
 * v11.5: 10 layers (4 shared + 6 org) — SEO/GEO moved to shared/knowledge
 */
export const NODE_VISUAL_LAYERS: LayerConfig[] = [
  // SHARED realm (4 layers: config, locale, geography, knowledge)
  {
    id: 'locale',
    label: 'Locale',
    icon: '🌐',
    color: '#2aa198',
    colorLight: '#34d399',
    nodeTypes: getNodeTypesByLayer('locale'),
  },
  {
    id: 'geography',
    label: 'Geography',
    icon: '🗺️',
    color: '#268bd2',
    colorLight: '#60a5fa',
    nodeTypes: getNodeTypesByLayer('geography'),
  },
  {
    id: 'knowledge',
    label: 'Knowledge',
    icon: '📚',
    color: '#6c71c4',
    colorLight: '#a78bfa',
    nodeTypes: getNodeTypesByLayer('knowledge'),
  },
  // ORG realm (6 layers: config, foundation, structure, semantic, instruction, output)
  // Note: 'config' layer contains nodes from BOTH realms (EntityCategory in shared, OrgConfig in org)
  {
    id: 'config',
    label: 'Config',
    icon: '⚙️',
    color: '#657b83',
    colorLight: '#93a1a1',
    nodeTypes: getNodeTypesByLayer('config'),
  },
  {
    id: 'foundation',
    label: 'Foundation',
    icon: '🏛️',
    color: '#d33682',
    colorLight: '#f472b6',
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
    color: '#2aa198',
    colorLight: '#5eead4',
    nodeTypes: getNodeTypesByLayer('instruction'),
  },
  // v11.4: seo and geo layers REMOVED - nodes moved to shared/knowledge
  {
    id: 'output',
    label: 'Output',
    icon: '✨',
    color: '#dc322f',
    colorLight: '#f87171',
    nodeTypes: getNodeTypesByLayer('output'),
  },
];

/**
 * Get layer config by id
 */
export function getLayerConfig(layerId: Layer): LayerConfig | undefined {
  return NODE_VISUAL_LAYERS.find((c) => c.id === layerId);
}

// =============================================================================
// VALIDATION: Ensure nodeTypeConfigs covers all NODE_TYPES from Core
// =============================================================================

// This will cause a TypeScript error if nodeTypeConfigs is missing any NodeType
const _validateCoverage: Record<NodeType, NodeTypeConfig> = nodeTypeConfigs;
void _validateCoverage; // Prevent unused variable warning
