// src/types/nodes.ts
// Single source of truth for all 35 NovaNet node types
// v9.0.0

// =============================================================================
// NODE TYPES (35 nodes)
// =============================================================================

export const NODE_TYPES = [
  // Invariant (11)
  'Project', 'BrandIdentity', 'Concept', 'Page', 'Block',
  'PageType', 'BlockType', 'PagePrompt', 'BlockPrompt', 'BlockRules', 'Locale',
  // Localized (6)
  'ProjectL10n', 'ConceptL10n', 'PageL10n', 'BlockL10n',
  'SEOKeywordL10n', 'GEOSeedL10n',
  // Knowledge (14)
  'LocaleIdentity', 'LocaleVoice', 'LocaleCulture', 'LocaleCultureReferences',
  'LocaleMarket', 'LocaleLexicon', 'LocaleRulesAdaptation', 'LocaleRulesFormatting',
  'LocaleRulesSlug', 'Expression', 'Reference', 'Metaphor', 'Pattern', 'Constraint',
  // Derived (2)
  'SEOKeywordMetrics', 'GEOSeedMetrics',
  // Job (2)
  'SEOMiningRun', 'GEOMiningRun',
] as const;

export type NodeType = typeof NODE_TYPES[number];

// =============================================================================
// v9 TAXONOMY TYPES
// =============================================================================

export type Realm = 'global' | 'project' | 'shared';

export type Layer =
  | 'config' | 'knowledge'
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output'
  | 'seo' | 'geo';

export type Trait = 'invariant' | 'localized' | 'knowledge' | 'derived' | 'job';

// =============================================================================
// KIND_META — unified classification for all 35 node types
// Replaces NODE_SCOPES, NODE_BEHAVIORS, NODE_CATEGORIES (v8)
// =============================================================================

export interface KindMeta {
  realm: Realm;
  layer: Layer;
  trait: Trait;
}

export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // PROJECT REALM — foundation (3)
  // ═══════════════════════════════════════════════════════════════════════════
  Project:      { realm: 'project', layer: 'foundation',  trait: 'invariant' },
  BrandIdentity:{ realm: 'project', layer: 'foundation',  trait: 'invariant' },
  ProjectL10n:  { realm: 'project', layer: 'foundation',  trait: 'localized' },

  // PROJECT REALM — structure (2)
  Page:         { realm: 'project', layer: 'structure',   trait: 'invariant' },
  Block:        { realm: 'project', layer: 'structure',   trait: 'invariant' },

  // PROJECT REALM — semantic (2)
  Concept:      { realm: 'project', layer: 'semantic',    trait: 'invariant' },
  ConceptL10n:  { realm: 'project', layer: 'semantic',    trait: 'localized' },

  // PROJECT REALM — instruction (5)
  PageType:     { realm: 'project', layer: 'instruction', trait: 'invariant' },
  BlockType:    { realm: 'project', layer: 'instruction', trait: 'invariant' },
  PagePrompt:   { realm: 'project', layer: 'instruction', trait: 'invariant' },
  BlockPrompt:  { realm: 'project', layer: 'instruction', trait: 'invariant' },
  BlockRules:   { realm: 'project', layer: 'instruction', trait: 'invariant' },

  // PROJECT REALM — output (2)
  PageL10n:     { realm: 'project', layer: 'output',      trait: 'localized' },
  BlockL10n:    { realm: 'project', layer: 'output',      trait: 'localized' },

  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM — config (1)
  // ═══════════════════════════════════════════════════════════════════════════
  Locale:       { realm: 'global',  layer: 'config',      trait: 'invariant' },

  // GLOBAL REALM — knowledge (14)
  LocaleIdentity:          { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleVoice:             { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleCulture:           { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleCultureReferences: { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleMarket:            { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleLexicon:           { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleRulesAdaptation:   { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleRulesFormatting:   { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  LocaleRulesSlug:         { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Expression:              { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Reference:               { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Metaphor:                { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Pattern:                 { realm: 'global', layer: 'knowledge', trait: 'knowledge' },
  Constraint:              { realm: 'global', layer: 'knowledge', trait: 'knowledge' },

  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM — seo (3)
  // ═══════════════════════════════════════════════════════════════════════════
  SEOKeywordL10n:    { realm: 'shared', layer: 'seo', trait: 'localized' },
  SEOKeywordMetrics: { realm: 'shared', layer: 'seo', trait: 'derived' },
  SEOMiningRun:      { realm: 'shared', layer: 'seo', trait: 'job' },

  // SHARED REALM — geo (3)
  GEOSeedL10n:    { realm: 'shared', layer: 'geo', trait: 'localized' },
  GEOSeedMetrics: { realm: 'shared', layer: 'geo', trait: 'derived' },
  GEOMiningRun:   { realm: 'shared', layer: 'geo', trait: 'job' },
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
