// src/types/nodes.ts
// Single source of truth for all 48 NovaNet node types
// v10.7.0 — 7-node locale knowledge architecture (GLOBAL / TENANT)

// =============================================================================
// NODE TYPES (48 nodes across 2 realms)
// =============================================================================

export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM (25 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (7) - v10.7: added Culture, Market
  'Locale', 'Formatting', 'Slugification', 'Adaptation', 'Style', 'Culture', 'Market',
  // locale-knowledge (12) — Sets + Atoms
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  // seo (6) — Keywords, metrics, mining, comparisons, prepositions, questions
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOMiningRun', 'SEOComparison', 'SEOPreposition', 'SEOQuestion',

  // ═══════════════════════════════════════════════════════════════════════════
  // TENANT REALM (23 nodes) — v10.6: merged organization + project
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1)
  'Organization',
  // foundation (3)
  'Project', 'BrandIdentity', 'ProjectL10n',
  // structure (3)
  'Page', 'Block', 'ContentSlot',
  // semantic (4)
  'Entity', 'EntityL10n', 'AudiencePersona', 'ChannelSurface',
  // instruction (7)
  'PageType', 'BlockType', 'PagePrompt', 'BlockPrompt', 'BlockRules', 'BlockInstruction', 'PromptArtifact',
  // output (5)
  'PageL10n', 'BlockL10n', 'GenerationJob', 'OutputArtifact', 'EvaluationSignal',
] as const;

export type NodeType = typeof NODE_TYPES[number];

// =============================================================================
// v10.6 TAXONOMY TYPES (2 realms, 8 layers)
// =============================================================================

export type Realm = 'global' | 'tenant';

export type Layer =
  | 'config' | 'locale-knowledge' | 'seo'  // global realm layers
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output';  // tenant realm layers

export type Trait = 'invariant' | 'localized' | 'knowledge' | 'derived' | 'job';

// =============================================================================
// KIND_META — unified classification for all 46 node types
// v10.6.0 — 2-Realm Architecture (global + tenant)
// =============================================================================

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM — config (7) - v10.7: added Culture, Market
  // ═══════════════════════════════════════════════════════════════════════════
  Locale:        { realm: 'global', layer: 'config', trait: 'invariant' },
  Formatting:    { realm: 'global', layer: 'config', trait: 'knowledge' },
  Slugification: { realm: 'global', layer: 'config', trait: 'knowledge' },
  Adaptation:    { realm: 'global', layer: 'config', trait: 'knowledge' },
  Style:         { realm: 'global', layer: 'config', trait: 'knowledge' },
  Culture:       { realm: 'global', layer: 'config', trait: 'knowledge' },
  Market:        { realm: 'global', layer: 'config', trait: 'knowledge' },

  // GLOBAL REALM — locale-knowledge (12) — Sets + Atoms
  TermSet:       { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  ExpressionSet: { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  PatternSet:    { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  CultureSet:    { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  TabooSet:      { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  AudienceSet:   { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Term:          { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Expression:    { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Pattern:       { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  CultureRef:    { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Taboo:         { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  AudienceTrait: { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },

  // GLOBAL REALM — seo (6)
  SEOKeyword:       { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOKeywordMetrics:{ realm: 'global', layer: 'seo', trait: 'derived' },
  SEOMiningRun:     { realm: 'global', layer: 'seo', trait: 'job' },
  SEOComparison:    { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOPreposition:   { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOQuestion:      { realm: 'global', layer: 'seo', trait: 'localized' },

  // ═══════════════════════════════════════════════════════════════════════════
  // TENANT REALM — config (1)
  // ═══════════════════════════════════════════════════════════════════════════
  Organization: { realm: 'tenant', layer: 'config',      trait: 'invariant' },

  // TENANT REALM — foundation (3)
  Project:      { realm: 'tenant', layer: 'foundation',  trait: 'invariant' },
  BrandIdentity:{ realm: 'tenant', layer: 'foundation',  trait: 'invariant' },
  ProjectL10n:  { realm: 'tenant', layer: 'foundation',  trait: 'localized' },

  // TENANT REALM — structure (3)
  Page:         { realm: 'tenant', layer: 'structure',   trait: 'invariant' },
  Block:        { realm: 'tenant', layer: 'structure',   trait: 'invariant' },
  ContentSlot:  { realm: 'tenant', layer: 'structure',   trait: 'invariant' },

  // TENANT REALM — semantic (4)
  Entity:          { realm: 'tenant', layer: 'semantic', trait: 'invariant' },
  EntityL10n:      { realm: 'tenant', layer: 'semantic', trait: 'localized' },
  AudiencePersona: { realm: 'tenant', layer: 'semantic', trait: 'invariant' },
  ChannelSurface:  { realm: 'tenant', layer: 'semantic', trait: 'invariant' },

  // TENANT REALM — instruction (7)
  PageType:        { realm: 'tenant', layer: 'instruction', trait: 'invariant' },
  BlockType:       { realm: 'tenant', layer: 'instruction', trait: 'invariant' },
  PagePrompt:      { realm: 'tenant', layer: 'instruction', trait: 'invariant' },
  BlockPrompt:     { realm: 'tenant', layer: 'instruction', trait: 'invariant' },
  BlockRules:      { realm: 'tenant', layer: 'instruction', trait: 'invariant' },
  BlockInstruction:{ realm: 'tenant', layer: 'instruction', trait: 'invariant' },
  PromptArtifact:  { realm: 'tenant', layer: 'instruction', trait: 'derived' },

  // TENANT REALM — output (5)
  PageL10n:         { realm: 'tenant', layer: 'output', trait: 'localized' },
  BlockL10n:        { realm: 'tenant', layer: 'output', trait: 'localized' },
  GenerationJob:    { realm: 'tenant', layer: 'output', trait: 'job' },
  OutputArtifact:   { realm: 'tenant', layer: 'output', trait: 'derived' },
  EvaluationSignal: { realm: 'tenant', layer: 'output', trait: 'derived' },
};

// =============================================================================
// DERIVED MAPS — computed from KIND_META
// =============================================================================

function deriveMap<K extends keyof KindMeta>(field: K): Record<NodeType, KindMeta[K]> {
  return Object.fromEntries(
    Object.entries(KIND_META).map(([k, v]) => [k, v[field]])
  ) as Record<NodeType, KindMeta[K]>;
}

export const NODE_REALMS: Record<NodeType, Realm> = deriveMap('realm');
export const NODE_TRAITS: Record<NodeType, Trait> = deriveMap('trait');
