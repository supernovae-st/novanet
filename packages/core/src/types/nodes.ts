// src/types/nodes.ts
// Single source of truth for all 42 NovaNet node types
// v10.3.0 — Entity-Centric Architecture, GEO removed, 2 realms (global/project)

// =============================================================================
// NODE TYPES (42 nodes across 2 realms)
// =============================================================================

export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM (22 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1)
  'Locale',
  // knowledge containers (10) — v10 tiered model: technical/style/semantic
  'Formatting', 'Slugification', 'Adaptation',  // Technical tier
  'Style',                                       // Style tier
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet', // Semantic tier
  // knowledge atoms (6)
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  // seo (3) — v10.2: moved from shared to global
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOMiningRun',
  // semantic (2) — v10.3: Entity-Centric Architecture
  'Entity', 'EntityL10n',

  // ═══════════════════════════════════════════════════════════════════════════
  // PROJECT REALM (20 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // foundation (3)
  'Project', 'BrandIdentity', 'ProjectL10n',
  // structure (5)
  'Page', 'Block', 'ContentSlot', 'PageType', 'BlockType',
  // semantic (2) — v10.3: AudiencePersona, ChannelSurface only (Entity/EntityL10n moved to global)
  'AudiencePersona', 'ChannelSurface',
  // instruction (5)
  'PagePrompt', 'BlockPrompt', 'BlockRules', 'BlockInstruction', 'PromptArtifact',
  // output (5)
  'PageL10n', 'BlockL10n', 'GenerationJob', 'OutputArtifact', 'EvaluationSignal',
] as const;

export type NodeType = typeof NODE_TYPES[number];

// =============================================================================
// v10.3 TAXONOMY TYPES (2 realms, 8 layers)
// =============================================================================

export type Realm = 'global' | 'project';

export type Layer =
  | 'config' | 'knowledge' | 'seo'  // global realm layers
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output';  // project realm layers

export type Trait = 'invariant' | 'localized' | 'knowledge' | 'derived' | 'job';

// =============================================================================
// KIND_META — unified classification for all 43 node types
// v10.3.0 — Entity-Centric Architecture, GEO removed
// =============================================================================

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM — config (1)
  // ═══════════════════════════════════════════════════════════════════════════
  Locale:       { realm: 'global',  layer: 'config',      trait: 'invariant' },

  // GLOBAL REALM — knowledge containers (10) — v10 tiered model
  // Technical tier: formatting rules, deterministic
  Formatting:    { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Slugification: { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Adaptation:    { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  // Style tier: voice & tone parameters
  Style:         { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  // Semantic tier: meaning-bearing content sets
  TermSet:       { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  ExpressionSet: { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  PatternSet:    { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  CultureSet:    { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  TabooSet:      { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  AudienceSet:   { realm: 'global', layer: 'knowledge', trait: 'knowledge' },

  // GLOBAL REALM — knowledge atoms (6)
  Term:          { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Expression:    { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Pattern:       { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  CultureRef:    { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Taboo:         { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  AudienceTrait: { realm: 'global', layer: 'knowledge', trait: 'knowledge' },

  // GLOBAL REALM — seo (3) — v10.2: moved from shared realm
  SEOKeyword:       { realm: 'global', layer: 'seo', trait: 'localized' },
  SEOKeywordMetrics:{ realm: 'global', layer: 'seo', trait: 'derived' },
  SEOMiningRun:     { realm: 'global', layer: 'seo', trait: 'job' },

  // GLOBAL REALM — semantic (2) — v10.3: Entity-Centric Architecture
  Entity:        { realm: 'global', layer: 'semantic', trait: 'invariant' },
  EntityL10n:    { realm: 'global', layer: 'semantic', trait: 'localized' },

  // ═══════════════════════════════════════════════════════════════════════════
  // PROJECT REALM — foundation (3)
  // ═══════════════════════════════════════════════════════════════════════════
  Project:      { realm: 'project', layer: 'foundation',  trait: 'invariant' },
  BrandIdentity:{ realm: 'project', layer: 'foundation',  trait: 'invariant' },
  ProjectL10n:  { realm: 'project', layer: 'foundation',  trait: 'localized' },

  // PROJECT REALM — structure (5)
  Page:         { realm: 'project', layer: 'structure',   trait: 'invariant' },
  Block:        { realm: 'project', layer: 'structure',   trait: 'invariant' },
  ContentSlot:  { realm: 'project', layer: 'structure',   trait: 'invariant' },
  PageType:     { realm: 'project', layer: 'structure',   trait: 'invariant' },
  BlockType:    { realm: 'project', layer: 'structure',   trait: 'invariant' },

  // PROJECT REALM — semantic (2) — v10.3: Entity/EntityL10n moved to global
  AudiencePersona: { realm: 'project', layer: 'semantic', trait: 'invariant' },
  ChannelSurface:  { realm: 'project', layer: 'semantic', trait: 'invariant' },

  // PROJECT REALM — instruction (5)
  PagePrompt:      { realm: 'project', layer: 'instruction', trait: 'invariant' },
  BlockPrompt:     { realm: 'project', layer: 'instruction', trait: 'invariant' },
  BlockRules:      { realm: 'project', layer: 'instruction', trait: 'invariant' },
  BlockInstruction:{ realm: 'project', layer: 'instruction', trait: 'invariant' },
  PromptArtifact:  { realm: 'project', layer: 'instruction', trait: 'derived' },

  // PROJECT REALM — output (5)
  PageL10n:         { realm: 'project', layer: 'output', trait: 'localized' },
  BlockL10n:        { realm: 'project', layer: 'output', trait: 'localized' },
  GenerationJob:    { realm: 'project', layer: 'output', trait: 'job' },
  OutputArtifact:   { realm: 'project', layer: 'output', trait: 'derived' },
  EvaluationSignal: { realm: 'project', layer: 'output', trait: 'derived' },
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

// =============================================================================
// DEPRECATION ALIASES (backwards compatibility)
// =============================================================================

/** @deprecated Use Entity instead (v10.3) */
export type Concept = 'Entity';
/** @deprecated Use EntityL10n instead (v10.3) */
export type ConceptL10n = 'EntityL10n';
