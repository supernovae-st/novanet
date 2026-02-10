// src/types/nodes.ts
// Single source of truth for all 60 NovaNet node types
// v11.5.0 — SEO/GEO moved to shared/knowledge (Knowledge Atoms pattern)

// =============================================================================
// NODE TYPES (60 nodes across 2 realms, 10 layers)
// =============================================================================

export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM (39 nodes) — 4 layers: config, locale, geography, knowledge
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — v11.5: classification nodes + Locale definition + SEO format
  'EntityCategory', 'Locale', 'SEOKeywordFormat',
  // locale (6) — Locale SETTINGS (not the Locale definition itself)
  'Formatting', 'Slugification', 'Adaptation', 'Style', 'Culture', 'Market',
  // geography (6) — Geographic classifications
  'Continent', 'GeoRegion', 'GeoSubRegion', 'IncomeGroup', 'LendingCategory', 'EconomicRegion',
  // knowledge (24) — Sets + Atoms + Linguistic/Cultural taxonomy + SEO/GEO
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  'LanguageFamily', 'LanguageBranch', 'CulturalRealm', 'CulturalSubRealm', 'PopulationCluster', 'PopulationSubCluster',
  // knowledge — SEO/GEO (6) — v11.5: moved from org to shared/knowledge
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOKeywordSet',
  'GEOQuery', 'GEOQuerySet', 'GEOAnswer',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (21 nodes) — 6 layers: config, foundation, structure, semantic, instruction, output
  // v11.4: SEO/GEO nodes moved to shared/knowledge
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1) — v11.3: Organization + Tenant merged into OrgConfig
  'OrgConfig',
  // foundation (3)
  'Project', 'BrandIdentity', 'ProjectContent',
  // structure (3)
  'Page', 'Block', 'ContentSlot',
  // semantic (4)
  'Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface',
  // instruction (7)
  'PageType', 'BlockType', 'PagePrompt', 'BlockPrompt', 'BlockRules', 'BlockInstruction', 'PromptArtifact',
  // output (3)
  'PageGenerated', 'BlockGenerated', 'OutputArtifact',
] as const;

export type NodeType = typeof NODE_TYPES[number];

// =============================================================================
// v11.5 TAXONOMY TYPES (2 realms, 10 layers: 4 shared + 6 org)
// SEO/GEO nodes moved to shared/knowledge (Knowledge Atoms pattern)
// =============================================================================

export type Realm = 'shared' | 'org';

export type Layer =
  | 'config' | 'locale' | 'geography' | 'knowledge'  // shared realm layers (4)
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output';  // org realm layers (6) — note: 'config' shared with shared realm

export type Trait = 'invariant' | 'localized' | 'knowledge' | 'generated' | 'aggregated';

// =============================================================================
// KIND_META — unified classification for all 60 node types
// v11.5.0 — SEO/GEO moved to shared/knowledge (Knowledge Atoms pattern)
// =============================================================================

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM — config (3) — v11.5: classification nodes + Locale definition
  // ═══════════════════════════════════════════════════════════════════════════
  EntityCategory:   { realm: 'shared', layer: 'config', trait: 'invariant' },
  Locale:           { realm: 'shared', layer: 'config', trait: 'invariant' },
  SEOKeywordFormat: { realm: 'shared', layer: 'config', trait: 'invariant' },

  // SHARED REALM — locale (6) — locale SETTINGS
  Formatting:     { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Slugification:  { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Adaptation:     { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Style:          { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Culture:        { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Market:         { realm: 'shared', layer: 'locale', trait: 'knowledge' },

  // SHARED REALM — geography (6)
  Continent:      { realm: 'shared', layer: 'geography', trait: 'invariant' },
  GeoRegion:      { realm: 'shared', layer: 'geography', trait: 'invariant' },
  GeoSubRegion:   { realm: 'shared', layer: 'geography', trait: 'invariant' },
  IncomeGroup:    { realm: 'shared', layer: 'geography', trait: 'invariant' },
  LendingCategory:{ realm: 'shared', layer: 'geography', trait: 'invariant' },
  EconomicRegion: { realm: 'shared', layer: 'geography', trait: 'invariant' },

  // SHARED REALM — knowledge (24) — containers, atoms, SEO/GEO
  TermSet:             { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  ExpressionSet:       { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  PatternSet:          { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  CultureSet:          { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  TabooSet:            { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  AudienceSet:         { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  Term:                { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  Expression:          { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  Pattern:             { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  CultureRef:          { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  Taboo:               { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  AudienceTrait:       { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  LanguageFamily:      { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  LanguageBranch:      { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  CulturalRealm:       { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  CulturalSubRealm:    { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  PopulationCluster:   { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  PopulationSubCluster:{ realm: 'shared', layer: 'knowledge', trait: 'knowledge' },

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM — config (1) — v11.3: Organization + Tenant merged
  // ═══════════════════════════════════════════════════════════════════════════
  OrgConfig: { realm: 'org', layer: 'config', trait: 'invariant' },

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

  // ORG REALM — instruction (7)
  PageType:        { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockType:       { realm: 'org', layer: 'instruction', trait: 'invariant' },
  PagePrompt:      { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockPrompt:     { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockRules:      { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockInstruction:{ realm: 'org', layer: 'instruction', trait: 'invariant' },
  PromptArtifact:  { realm: 'org', layer: 'instruction', trait: 'generated' },

  // SHARED REALM — knowledge (SEO/GEO) — v11.5: moved from org to shared
  SEOKeyword:       { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  SEOKeywordMetrics:{ realm: 'shared', layer: 'knowledge', trait: 'aggregated' },
  SEOKeywordSet:    { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  GEOQuery:         { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  GEOQuerySet:      { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  GEOAnswer:        { realm: 'shared', layer: 'knowledge', trait: 'aggregated' },

  // ORG REALM — output (3)
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
