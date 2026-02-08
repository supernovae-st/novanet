// src/types/nodes.ts
// Single source of truth for all 64 NovaNet node types
// v10.9.0 — Typed semantic arcs + GEO layer (GLOBAL / TENANT)

// =============================================================================
// NODE TYPES (64 nodes across 2 realms)
// =============================================================================

export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM (40 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (13) - v10.8: added geographic taxonomy
  'Locale', 'Formatting', 'Slugification', 'Adaptation', 'Style', 'Culture', 'Market',
  'Continent', 'GeoRegion', 'GeoSubRegion', 'IncomeGroup', 'LendingCategory', 'EconomicRegion',
  // locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  'LanguageFamily', 'LanguageBranch', 'CulturalRealm', 'CulturalSubRealm', 'PopulationCluster', 'PopulationSubCluster',
  // seo (9) — Keywords, metrics, mining, comparisons, prepositions, questions + GEO
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOMiningRun', 'SEOComparison', 'SEOPreposition', 'SEOQuestion',
  'GEOQuery', 'GEOAnswer', 'GEOMetrics',

  // ═══════════════════════════════════════════════════════════════════════════
  // TENANT REALM (24 nodes) — v10.6: merged organization + project
  // ═══════════════════════════════════════════════════════════════════════════
  // config (2)
  'Organization', 'Tenant',
  // foundation (3)
  'Project', 'BrandIdentity', 'ProjectL10n',
  // structure (3)
  'Page', 'Block', 'ContentSlot',
  // semantic (4)
  'Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface',
  // instruction (7)
  'PageType', 'BlockType', 'PagePrompt', 'BlockPrompt', 'BlockRules', 'BlockInstruction', 'PromptArtifact',
  // output (5)
  'PageGenerated', 'BlockGenerated', 'GenerationJob', 'OutputArtifact', 'EvaluationSignal',
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
// KIND_META — unified classification for all 64 node types
// v10.9.0 — Typed semantic arcs + GEO layer
// =============================================================================

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM — config (13) - v10.8: added geographic taxonomy
  // ═══════════════════════════════════════════════════════════════════════════
  Locale:         { realm: 'global', layer: 'config', trait: 'invariant' },
  Formatting:     { realm: 'global', layer: 'config', trait: 'knowledge' },
  Slugification:  { realm: 'global', layer: 'config', trait: 'knowledge' },
  Adaptation:     { realm: 'global', layer: 'config', trait: 'knowledge' },
  Style:          { realm: 'global', layer: 'config', trait: 'knowledge' },
  Culture:        { realm: 'global', layer: 'config', trait: 'knowledge' },
  Market:         { realm: 'global', layer: 'config', trait: 'knowledge' },
  Continent:      { realm: 'global', layer: 'config', trait: 'invariant' },
  GeoRegion:      { realm: 'global', layer: 'config', trait: 'invariant' },
  GeoSubRegion:   { realm: 'global', layer: 'config', trait: 'invariant' },
  IncomeGroup:    { realm: 'global', layer: 'config', trait: 'invariant' },
  LendingCategory:{ realm: 'global', layer: 'config', trait: 'invariant' },
  EconomicRegion: { realm: 'global', layer: 'config', trait: 'invariant' },

  // GLOBAL REALM — locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
  TermSet:             { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  ExpressionSet:       { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  PatternSet:          { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  CultureSet:          { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  TabooSet:            { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  AudienceSet:         { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Term:                { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Expression:          { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Pattern:             { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  CultureRef:          { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  Taboo:               { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  AudienceTrait:       { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  LanguageFamily:      { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  LanguageBranch:      { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  CulturalRealm:       { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  CulturalSubRealm:    { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  PopulationCluster:   { realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },
  PopulationSubCluster:{ realm: 'global', layer: 'locale-knowledge', trait: 'knowledge' },

  // GLOBAL REALM — seo (9) — SEO + GEO (Generative Engine Optimization)
  SEOKeyword:       { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOKeywordMetrics:{ realm: 'global', layer: 'seo', trait: 'derived' },
  SEOMiningRun:     { realm: 'global', layer: 'seo', trait: 'job' },
  SEOComparison:    { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOPreposition:   { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOQuestion:      { realm: 'global', layer: 'seo', trait: 'localized' },
  GEOQuery:         { realm: 'global', layer: 'seo', trait: 'knowledge' },
  GEOAnswer:        { realm: 'global', layer: 'seo', trait: 'derived' },
  GEOMetrics:       { realm: 'global', layer: 'seo', trait: 'derived' },

  // ═══════════════════════════════════════════════════════════════════════════
  // TENANT REALM — config (2)
  // ═══════════════════════════════════════════════════════════════════════════
  Organization: { realm: 'tenant', layer: 'config',      trait: 'invariant' },
  Tenant:       { realm: 'tenant', layer: 'config',      trait: 'invariant' },

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
  EntityContent:      { realm: 'tenant', layer: 'semantic', trait: 'localized' },
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

  // TENANT REALM — output (5) — v10.9.0: derived trait for LLM-generated outputs
  PageGenerated:         { realm: 'tenant', layer: 'output', trait: 'derived' },
  BlockGenerated:        { realm: 'tenant', layer: 'output', trait: 'derived' },
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
