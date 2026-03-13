// =============================================================================
// NODE TYPE CONFIGURATION (v0.17.0)
// =============================================================================
// Visual configuration for all 60 NovaNet node types
// 10 layers (4 shared + 6 org), 2 realms
// NodeType, Layer, CLASS_TAXONOMY imported from @novanet/core (Single Source of Truth)
// v0.17.0: Market, Term, TermSet, SEOKeywordMetrics, AudiencePersona, ChannelSurface removed
//          ProjectSEOScope, ProjectGEOScope added

import type { NodeType, Layer } from '@novanet/core/types';
import { NODE_TYPES, CLASS_TAXONOMY } from '@novanet/core/types';

// =============================================================================
// NODE LAYERS (10 layers, 60 nodes)
// Derived from CLASS_TAXONOMY — single source of truth
// =============================================================================

/**
 * Node layers with their types (60 nodes across 10 layers)
 * Derived from CLASS_TAXONOMY in @novanet/core
 */
export const NODE_LAYERS: Record<Layer, NodeType[]> = Object.entries(CLASS_TAXONOMY).reduce(
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
 * All node type configurations (60 nodes)
 * Aligned with @novanet/core NODE_TYPES
 * v0.17.0: Market, Term, TermSet, SEOKeywordMetrics, AudiencePersona, ChannelSurface removed
 *          ProjectSEOScope, ProjectGEOScope added
 */
export const nodeTypeConfigs: Record<NodeType, NodeTypeConfig> = {
  // ==========================================================================
  // SHARED REALM — LOCALE LAYER (5 nodes) — v0.17.0: Market removed
  // Note: Locale node moved to config layer per schema
  // ==========================================================================
  Formatting: {
    type: 'Formatting',
    label: 'Formatting',
    icon: 'hash',
    color: '#06b6d4',
    colorClass: 'bg-cyan-500',
    size: 12,
    layer: 'locale',
  },
  Slugification: {
    type: 'Slugification',
    label: 'Slugification',
    icon: 'link',
    color: '#0891b2',
    colorClass: 'bg-cyan-600',
    size: 12,
    layer: 'locale',
  },
  Adaptation: {
    type: 'Adaptation',
    label: 'Adaptation',
    icon: 'droplet',
    color: '#0e7490',
    colorClass: 'bg-cyan-700',
    size: 12,
    layer: 'locale',
  },
  Style: {
    type: 'Style',
    label: 'Style',
    icon: 'palette',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 14,
    layer: 'locale',
  },
  Culture: {
    type: 'Culture',
    label: 'Culture',
    icon: 'globe',
    color: '#d97706',
    colorClass: 'bg-amber-600',
    size: 16,
    layer: 'locale',
  },
  // v0.17.0: Market removed

  // ==========================================================================
  // SHARED REALM — GEOGRAPHY LAYER (7 nodes)
  // ==========================================================================
  Continent: {
    type: 'Continent',
    label: 'Continent',
    icon: 'globe',
    color: '#2563eb',
    colorClass: 'bg-blue-600',
    size: 18,
    layer: 'geography',
  },
  // v0.12.4: Country added to shared/geography
  Country: {
    type: 'Country',
    label: 'Country',
    icon: 'flag',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 16,
    layer: 'geography',
  },
  GeoRegion: {
    type: 'GeoRegion',
    label: 'Geo Region',
    icon: 'map',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 16,
    layer: 'geography',
  },
  GeoSubRegion: {
    type: 'GeoSubRegion',
    label: 'Geo Sub-Region',
    icon: 'map-pin',
    color: '#60a5fa',
    colorClass: 'bg-blue-400',
    size: 14,
    layer: 'geography',
  },
  IncomeGroup: {
    type: 'IncomeGroup',
    label: 'Income Group',
    icon: 'dollar-sign',
    color: '#16a34a',
    colorClass: 'bg-green-600',
    size: 14,
    layer: 'geography',
  },
  LendingCategory: {
    type: 'LendingCategory',
    label: 'Lending Category',
    icon: 'landmark',
    color: '#0891b2',
    colorClass: 'bg-cyan-600',
    size: 14,
    layer: 'geography',
  },
  EconomicRegion: {
    type: 'EconomicRegion',
    label: 'Economic Region',
    icon: 'bar-chart-2',
    color: '#059669',
    colorClass: 'bg-emerald-600',
    size: 14,
    layer: 'geography',
  },

  // ==========================================================================
  // SHARED REALM — CONFIG LAYER (3 nodes) — v0.17.0: Locale, SEOKeywordFormat moved here
  // ==========================================================================
  EntityCategory: {
    type: 'EntityCategory',
    label: 'Entity Category',
    icon: 'folder',
    color: '#64748b',
    colorClass: 'bg-slate-500',
    size: 16,
    layer: 'config',
  },
  Locale: {
    type: 'Locale',
    label: 'Locale',
    icon: 'globe',
    color: '#10b981',
    colorClass: 'bg-emerald-500',
    size: 20,
    layer: 'config',
  },
  SEOKeywordFormat: {
    type: 'SEOKeywordFormat',
    label: 'SEO Keyword Format',
    icon: 'component',
    color: '#64748b',
    colorClass: 'bg-slate-500',
    size: 14,
    layer: 'config',
  },

  // ==========================================================================
  // SHARED REALM — KNOWLEDGE LAYER (21 nodes) — containers, atoms, SEO/GEO
  // v0.17.0: TermSet, Term removed (denomination_forms covers terminology)
  // ==========================================================================
  ExpressionSet: {
    type: 'ExpressionSet',
    label: 'Expression Set',
    icon: 'message-square',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 10,
    layer: 'knowledge',
  },
  PatternSet: {
    type: 'PatternSet',
    label: 'Pattern Set',
    icon: 'clipboard',
    color: '#f472b6',
    colorClass: 'bg-pink-400',
    size: 10,
    layer: 'knowledge',
  },
  CultureSet: {
    type: 'CultureSet',
    label: 'Culture Set',
    icon: 'drama',
    color: '#86efac',
    colorClass: 'bg-green-300',
    size: 10,
    layer: 'knowledge',
  },
  TabooSet: {
    type: 'TabooSet',
    label: 'Taboo Set',
    icon: 'ban',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 10,
    layer: 'knowledge',
  },
  AudienceSet: {
    type: 'AudienceSet',
    label: 'Audience Set',
    icon: 'users',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 10,
    layer: 'knowledge',
  },
  // v0.17.0: Term removed (denomination_forms covers terminology)
  Expression: {
    type: 'Expression',
    label: 'Expression',
    icon: 'message-square',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 8,
    layer: 'knowledge',
  },
  Pattern: {
    type: 'Pattern',
    label: 'Pattern',
    icon: 'clipboard',
    color: '#f472b6',
    colorClass: 'bg-pink-400',
    size: 8,
    layer: 'knowledge',
  },
  CultureRef: {
    type: 'CultureRef',
    label: 'Culture Ref',
    icon: 'globe',
    color: '#86efac',
    colorClass: 'bg-green-300',
    size: 8,
    layer: 'knowledge',
  },
  Taboo: {
    type: 'Taboo',
    label: 'Taboo',
    icon: 'ban',
    color: '#ef4444',
    colorClass: 'bg-red-500',
    size: 8,
    layer: 'knowledge',
  },
  AudienceTrait: {
    type: 'AudienceTrait',
    label: 'Audience Trait',
    icon: 'user',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 8,
    layer: 'knowledge',
  },
  LanguageFamily: {
    type: 'LanguageFamily',
    label: 'Language Family',
    icon: 'message-circle',
    color: '#7c3aed',
    colorClass: 'bg-violet-600',
    size: 14,
    layer: 'knowledge',
  },
  LanguageBranch: {
    type: 'LanguageBranch',
    label: 'Language Branch',
    icon: 'leaf',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 12,
    layer: 'knowledge',
  },
  CulturalRealm: {
    type: 'CulturalRealm',
    label: 'Cultural Realm',
    icon: 'drama',
    color: '#db2777',
    colorClass: 'bg-pink-600',
    size: 14,
    layer: 'knowledge',
  },
  CulturalSubRealm: {
    type: 'CulturalSubRealm',
    label: 'Cultural Sub-Realm',
    icon: 'palette',
    color: '#ec4899',
    colorClass: 'bg-pink-500',
    size: 12,
    layer: 'knowledge',
  },
  PopulationCluster: {
    type: 'PopulationCluster',
    label: 'Population Cluster',
    icon: 'users',
    color: '#0284c7',
    colorClass: 'bg-sky-600',
    size: 14,
    layer: 'knowledge',
  },
  PopulationSubCluster: {
    type: 'PopulationSubCluster',
    label: 'Population Sub-Cluster',
    icon: 'users',
    color: '#0ea5e9',
    colorClass: 'bg-sky-500',
    size: 12,
    layer: 'knowledge',
  },

  // ==========================================================================
  // ORG REALM — CONFIG LAYER (1 node) — v0.12.0: OrgConfig
  // ==========================================================================
  OrgConfig: {
    type: 'OrgConfig',
    label: 'Org Config',
    icon: 'building-2',
    color: '#0ea5e9',
    colorClass: 'bg-sky-500',
    size: 22,
    layer: 'config',
  },

  // ==========================================================================
  // ORG REALM — FOUNDATION LAYER (8 nodes) — v0.17.0: ProjectSEOScope, ProjectGEOScope added
  // ==========================================================================
  Project: {
    type: 'Project',
    label: 'Project',
    icon: 'folder',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 24,
    layer: 'foundation',
  },
  // v0.12.4: BrandIdentity → Brand, new Brand Architecture nodes
  Brand: {
    type: 'Brand',
    label: 'Brand',
    icon: 'circle',
    color: '#6d28d9',
    colorClass: 'bg-violet-700',
    size: 18,
    layer: 'foundation',
  },
  BrandDesign: {
    type: 'BrandDesign',
    label: 'Brand Design',
    icon: 'circle',
    color: '#7c3aed',
    colorClass: 'bg-violet-600',
    size: 16,
    layer: 'foundation',
  },
  BrandPrinciples: {
    type: 'BrandPrinciples',
    label: 'Brand Principles',
    icon: 'circle',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 16,
    layer: 'foundation',
  },
  PromptStyle: {
    type: 'PromptStyle',
    label: 'Prompt Style',
    icon: 'circle',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 16,
    layer: 'foundation',
  },
  ProjectNative: {
    type: 'ProjectNative',
    label: 'Project Native',
    icon: 'map',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 18,
    layer: 'foundation',
  },
  // v0.17.0: ProjectSEOScope, ProjectGEOScope added
  ProjectSEOScope: {
    type: 'ProjectSEOScope',
    label: 'Project SEO Scope',
    icon: 'search',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 14,
    layer: 'foundation',
  },
  ProjectGEOScope: {
    type: 'ProjectGEOScope',
    label: 'Project GEO Scope',
    icon: 'bot',
    color: '#6d28d9',
    colorClass: 'bg-violet-700',
    size: 14,
    layer: 'foundation',
  },

  // ==========================================================================
  // ORG REALM — STRUCTURE LAYER (3 nodes)
  // ==========================================================================
  Page: {
    type: 'Page',
    label: 'Page',
    icon: 'file-text',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 20,
    layer: 'structure',
  },
  Block: {
    type: 'Block',
    label: 'Block',
    icon: 'square',
    color: '#06b6d4',
    colorClass: 'bg-cyan-500',
    size: 16,
    layer: 'structure',
  },
  ContentSlot: {
    type: 'ContentSlot',
    label: 'Content Slot',
    icon: 'inbox',
    color: '#0891b2',
    colorClass: 'bg-cyan-600',
    size: 14,
    layer: 'structure',
  },

  // ==========================================================================
  // ORG REALM — SEMANTIC LAYER (2 nodes) — v0.17.0: AudiencePersona, ChannelSurface removed
  // ==========================================================================
  Entity: {
    type: 'Entity',
    label: 'Entity',
    icon: 'diamond',
    color: '#f59e0b',
    colorClass: 'bg-amber-500',
    size: 20,
    layer: 'semantic',
  },
  EntityNative: {
    type: 'EntityNative',
    label: 'Entity Native',
    icon: 'message-square',
    color: '#fbbf24',
    colorClass: 'bg-yellow-400',
    size: 16,
    layer: 'semantic',
  },
  // v0.17.0: AudiencePersona, ChannelSurface removed

  // ==========================================================================
  // ORG REALM — INSTRUCTION LAYER (4 nodes) — v0.12.4: PageStructure, PageInstruction deleted
  // ==========================================================================
  BlockType: {
    type: 'BlockType',
    label: 'Block Type',
    icon: 'component',
    color: '#14b8a6',
    colorClass: 'bg-teal-500',
    size: 16,
    layer: 'instruction',
  },
  BlockInstruction: {
    type: 'BlockInstruction',
    label: 'Block Instruction',
    icon: 'hash',
    color: '#60a5fa',
    colorClass: 'bg-blue-400',
    size: 12,
    layer: 'instruction',
  },
  PromptArtifact: {
    type: 'PromptArtifact',
    label: 'Prompt Artifact',
    icon: 'component',
    color: '#bfdbfe',
    colorClass: 'bg-blue-200',
    size: 10,
    layer: 'instruction',
  },

  // ==========================================================================
  // SHARED REALM — KNOWLEDGE LAYER — SEO nodes (2 nodes)
  // v0.17.0: SEOKeywordMetrics removed (metrics stored directly on SEOKeyword)
  // ==========================================================================
  SEOKeyword: {
    type: 'SEOKeyword',
    label: 'SEO Keyword',
    icon: 'search',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 16,
    layer: 'knowledge',
  },
  // v0.17.0: SEOKeywordMetrics removed
  SEOKeywordSet: {
    type: 'SEOKeywordSet',
    label: 'SEO Keyword Set',
    icon: 'package',
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
    icon: 'bot',
    color: '#8b5cf6',
    colorClass: 'bg-violet-500',
    size: 16,
    layer: 'knowledge',
  },
  GEOQuerySet: {
    type: 'GEOQuerySet',
    label: 'GEO Query Set',
    icon: 'package',
    color: '#6d28d9',
    colorClass: 'bg-violet-700',
    size: 12,
    layer: 'knowledge',
  },
  GEOAnswer: {
    type: 'GEOAnswer',
    label: 'GEO Answer',
    icon: 'message-square',
    color: '#a78bfa',
    colorClass: 'bg-violet-400',
    size: 14,
    layer: 'knowledge',
  },

  // ==========================================================================
  // ORG REALM — OUTPUT LAYER (6 nodes) — v0.19.0: +3 Enrichments
  // ==========================================================================
  PageNative: {
    type: 'PageNative',
    label: 'Page Native',
    icon: 'clipboard',
    color: '#f97316',
    colorClass: 'bg-orange-500',
    size: 16,
    layer: 'output',
  },
  BlockNative: {
    type: 'BlockNative',
    label: 'Block Native',
    icon: 'hash',
    color: '#fb923c',
    colorClass: 'bg-orange-400',
    size: 14,
    layer: 'output',
  },
  OutputArtifact: {
    type: 'OutputArtifact',
    label: 'Output Artifact',
    icon: 'folder',
    color: '#c2410c',
    colorClass: 'bg-orange-700',
    size: 12,
    layer: 'output',
  },
  CultureRefEnrichment: {
    type: 'CultureRefEnrichment',
    label: 'CultureRef Enrichment',
    icon: 'globe',
    color: '#ea580c',
    colorClass: 'bg-orange-600',
    size: 12,
    layer: 'output',
  },
  ExpressionEnrichment: {
    type: 'ExpressionEnrichment',
    label: 'Expression Enrichment',
    icon: 'message-circle',
    color: '#d97706',
    colorClass: 'bg-amber-600',
    size: 12,
    layer: 'output',
  },
  TabooEnrichment: {
    type: 'TabooEnrichment',
    label: 'Taboo Enrichment',
    icon: 'shield-alert',
    color: '#dc2626',
    colorClass: 'bg-red-600',
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
  // Colors from taxonomy.yaml via generated.ts (Single Source of Truth)
  {
    id: 'locale',
    label: 'Locale',
    icon: 'map',
    color: '#64748b',  // slate-500 (from taxonomy.yaml)
    colorLight: '#94a3b8',  // slate-400
    nodeTypes: getNodeTypesByLayer('locale'),
  },
  {
    id: 'geography',
    label: 'Geography',
    icon: 'globe',
    color: '#10b981',  // emerald-500 (from taxonomy.yaml)
    colorLight: '#34d399',  // emerald-400
    nodeTypes: getNodeTypesByLayer('geography'),
  },
  {
    id: 'knowledge',
    label: 'Knowledge',
    icon: 'book-open',
    color: '#8b5cf6',  // violet-500 (from taxonomy.yaml)
    colorLight: '#a78bfa',  // violet-400
    nodeTypes: getNodeTypesByLayer('knowledge'),
  },
  // ORG realm (6 layers: config, foundation, structure, semantic, instruction, output)
  // Note: 'config' layer contains nodes from BOTH realms (EntityCategory in shared, OrgConfig in org)
  {
    id: 'config',
    label: 'Config',
    icon: 'terminal',
    color: '#64748b',  // slate-500 (from taxonomy.yaml)
    colorLight: '#94a3b8',  // slate-400
    nodeTypes: getNodeTypesByLayer('config'),
  },
  {
    id: 'foundation',
    label: 'Foundation',
    icon: 'drama',
    color: '#3b82f6',  // blue-500 (from taxonomy.yaml)
    colorLight: '#60a5fa',  // blue-400
    nodeTypes: getNodeTypesByLayer('foundation'),
  },
  {
    id: 'structure',
    label: 'Structure',
    icon: 'layout-grid',
    color: '#06b6d4',  // cyan-500 (from taxonomy.yaml)
    colorLight: '#22d3ee',  // cyan-400
    nodeTypes: getNodeTypesByLayer('structure'),
  },
  {
    id: 'semantic',
    label: 'Semantic',
    icon: 'diamond',
    color: '#f97316',  // orange-500 (from taxonomy.yaml)
    colorLight: '#fb923c',  // orange-400
    nodeTypes: getNodeTypesByLayer('semantic'),
  },
  {
    id: 'instruction',
    label: 'Instruction',
    icon: 'hash',
    color: '#eab308',  // yellow-500 (from taxonomy.yaml)
    colorLight: '#facc15',  // yellow-400
    nodeTypes: getNodeTypesByLayer('instruction'),
  },
  // v11.4: seo and geo layers REMOVED - nodes moved to shared/knowledge
  {
    id: 'output',
    label: 'Output',
    icon: 'circle',
    color: '#22c55e',  // green-500 (from taxonomy.yaml)
    colorLight: '#4ade80',  // green-400
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
// ALIASES
// =============================================================================

/**
 * Alias for nodeTypeConfigs (used throughout the codebase)
 */
export const NODE_TYPE_CONFIG = nodeTypeConfigs;

// =============================================================================
// VALIDATION: Ensure nodeTypeConfigs covers all NODE_TYPES from Core
// =============================================================================

// This will cause a TypeScript error if nodeTypeConfigs is missing any NodeType
const _validateCoverage: Record<NodeType, NodeTypeConfig> = nodeTypeConfigs;
void _validateCoverage; // Prevent unused variable warning
