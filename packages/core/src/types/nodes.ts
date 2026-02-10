// src/types/nodes.ts
// Single source of truth for all 62 NovaNet node types
// v11.2.0 — Realm renames (shared/org), trait split (generated/aggregated)

// =============================================================================
// NODE TYPES (62 nodes across 2 realms)
// =============================================================================

export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM (32 nodes) — v11.2: renamed from global
  // ═══════════════════════════════════════════════════════════════════════════
  // config (14)
  'Locale', 'Formatting', 'Slugification', 'Adaptation', 'Style', 'Culture', 'Market',
  'Continent', 'GeoRegion', 'GeoSubRegion', 'IncomeGroup', 'LendingCategory', 'EconomicRegion',
  'EntityCategory',
  // locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  'LanguageFamily', 'LanguageBranch', 'CulturalRealm', 'CulturalSubRealm', 'PopulationCluster', 'PopulationSubCluster',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (30 nodes) — v11.2: renamed from tenant, job nodes removed
  // ═══════════════════════════════════════════════════════════════════════════
  // config (2)
  'Organization', 'Tenant',
  // foundation (3)
  'Project', 'BrandIdentity', 'ProjectContent',
  // structure (3)
  'Page', 'Block', 'ContentSlot',
  // semantic (4)
  'Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface',
  // seo (6) — v11.2: SEOMiningRun removed (was job node)
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOComparison', 'SEOPreposition', 'SEOQuestion',
  'GEOQuery', 'GEOAnswer', 'GEOMetrics',
  // instruction (7)
  'PageType', 'BlockType', 'PagePrompt', 'BlockPrompt', 'BlockRules', 'BlockInstruction', 'PromptArtifact',
  // output (3) — v11.2: GenerationJob, EvaluationSignal removed (job nodes)
  'PageGenerated', 'BlockGenerated', 'OutputArtifact',
] as const;

export type NodeType = typeof NODE_TYPES[number];

// =============================================================================
// v11.2 TAXONOMY TYPES (2 realms, 9 layers: 2 shared + 7 org)
// =============================================================================

export type Realm = 'shared' | 'org';

export type Layer =
  | 'config' | 'locale-knowledge'  // shared realm layers (2)
  | 'foundation' | 'structure' | 'semantic' | 'seo' | 'instruction' | 'output';  // org realm layers (7)

export type Trait = 'invariant' | 'localized' | 'knowledge' | 'generated' | 'aggregated';

// =============================================================================
// KIND_META — unified classification for all 62 node types
// v11.1.0 — EntityCategory classification system
// =============================================================================

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM — config (14) — v11.2: renamed from global
  // ═══════════════════════════════════════════════════════════════════════════
  Locale:         { realm: 'shared', layer: 'config', trait: 'invariant' },
  Formatting:     { realm: 'shared', layer: 'config', trait: 'knowledge' },
  Slugification:  { realm: 'shared', layer: 'config', trait: 'knowledge' },
  Adaptation:     { realm: 'shared', layer: 'config', trait: 'knowledge' },
  Style:          { realm: 'shared', layer: 'config', trait: 'knowledge' },
  Culture:        { realm: 'shared', layer: 'config', trait: 'knowledge' },
  Market:         { realm: 'shared', layer: 'config', trait: 'knowledge' },
  Continent:      { realm: 'shared', layer: 'config', trait: 'invariant' },
  GeoRegion:      { realm: 'shared', layer: 'config', trait: 'invariant' },
  GeoSubRegion:   { realm: 'shared', layer: 'config', trait: 'invariant' },
  IncomeGroup:    { realm: 'shared', layer: 'config', trait: 'invariant' },
  LendingCategory:{ realm: 'shared', layer: 'config', trait: 'invariant' },
  EconomicRegion: { realm: 'shared', layer: 'config', trait: 'invariant' },
  EntityCategory: { realm: 'shared', layer: 'config', trait: 'invariant' },

  // SHARED REALM — locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
  TermSet:             { realm: 'shared', layer: 'locale-knowledge', trait: 'invariant' },
  ExpressionSet:       { realm: 'shared', layer: 'locale-knowledge', trait: 'invariant' },
  PatternSet:          { realm: 'shared', layer: 'locale-knowledge', trait: 'invariant' },
  CultureSet:          { realm: 'shared', layer: 'locale-knowledge', trait: 'invariant' },
  TabooSet:            { realm: 'shared', layer: 'locale-knowledge', trait: 'invariant' },
  AudienceSet:         { realm: 'shared', layer: 'locale-knowledge', trait: 'invariant' },
  Term:                { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  Expression:          { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  Pattern:             { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  CultureRef:          { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  Taboo:               { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  AudienceTrait:       { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  LanguageFamily:      { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  LanguageBranch:      { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  CulturalRealm:       { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  CulturalSubRealm:    { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  PopulationCluster:   { realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },
  PopulationSubCluster:{ realm: 'shared', layer: 'locale-knowledge', trait: 'knowledge' },

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM — config (2) — v11.2: renamed from tenant
  // ═══════════════════════════════════════════════════════════════════════════
  Organization: { realm: 'org', layer: 'config',      trait: 'invariant' },
  Tenant:       { realm: 'org', layer: 'config',      trait: 'invariant' },

  // ORG REALM — foundation (3)
  Project:        { realm: 'org', layer: 'foundation',  trait: 'invariant' },
  BrandIdentity:  { realm: 'org', layer: 'foundation',  trait: 'invariant' },
  ProjectContent: { realm: 'org', layer: 'foundation',  trait: 'localized' },

  // ORG REALM — structure (3)
  Page:         { realm: 'org', layer: 'structure',   trait: 'invariant' },
  Block:        { realm: 'org', layer: 'structure',   trait: 'invariant' },
  ContentSlot:  { realm: 'org', layer: 'structure',   trait: 'invariant' },

  // ORG REALM — semantic (4)
  Entity:          { realm: 'org', layer: 'semantic', trait: 'invariant' },
  EntityContent:   { realm: 'org', layer: 'semantic', trait: 'localized' },
  AudiencePersona: { realm: 'org', layer: 'semantic', trait: 'invariant' },
  ChannelSurface:  { realm: 'org', layer: 'semantic', trait: 'invariant' },

  // ORG REALM — seo (8) — v11.2: SEOMiningRun removed, aggregated trait split
  SEOKeyword:       { realm: 'org', layer: 'seo', trait: 'knowledge' },
  SEOKeywordMetrics:{ realm: 'org', layer: 'seo', trait: 'aggregated' },
  SEOComparison:    { realm: 'org', layer: 'seo', trait: 'knowledge' },
  SEOPreposition:   { realm: 'org', layer: 'seo', trait: 'knowledge' },
  SEOQuestion:      { realm: 'org', layer: 'seo', trait: 'knowledge' },
  GEOQuery:         { realm: 'org', layer: 'seo', trait: 'knowledge' },
  GEOAnswer:        { realm: 'org', layer: 'seo', trait: 'aggregated' },
  GEOMetrics:       { realm: 'org', layer: 'seo', trait: 'aggregated' },

  // ORG REALM — instruction (7)
  PageType:        { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockType:       { realm: 'org', layer: 'instruction', trait: 'invariant' },
  PagePrompt:      { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockPrompt:     { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockRules:      { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockInstruction:{ realm: 'org', layer: 'instruction', trait: 'invariant' },
  PromptArtifact:  { realm: 'org', layer: 'instruction', trait: 'generated' },

  // ORG REALM — output (3) — v11.2: job nodes removed, generated trait
  PageGenerated:  { realm: 'org', layer: 'output', trait: 'generated' },
  BlockGenerated: { realm: 'org', layer: 'output', trait: 'generated' },
  OutputArtifact: { realm: 'org', layer: 'output', trait: 'generated' },
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
