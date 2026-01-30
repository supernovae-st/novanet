// src/types/nodes.ts
// Single source of truth for all 35 NovaNet node types
// v8.1.0

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
  // LocaleKnowledge (14)
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
// NODE CATEGORIES
// =============================================================================

export type NodeCategory = 'project' | 'content' | 'locale' | 'generation' | 'seo' | 'geo';

export const NODE_CATEGORIES: Record<NodeCategory, readonly NodeType[]> = {
  project: ['Project', 'BrandIdentity', 'ProjectL10n'],
  content: ['Concept', 'ConceptL10n', 'Page', 'Block', 'PageType', 'BlockType'],
  locale: [
    'Locale', 'LocaleIdentity', 'LocaleVoice', 'LocaleCulture', 'LocaleCultureReferences',
    'LocaleMarket', 'LocaleLexicon', 'LocaleRulesAdaptation', 'LocaleRulesFormatting',
    'LocaleRulesSlug', 'Expression', 'Reference', 'Metaphor', 'Pattern', 'Constraint',
  ],
  generation: ['PagePrompt', 'BlockPrompt', 'BlockRules', 'PageL10n', 'BlockL10n'],
  seo: ['SEOKeywordL10n', 'SEOKeywordMetrics', 'SEOMiningRun'],
  geo: ['GEOSeedL10n', 'GEOSeedMetrics', 'GEOMiningRun'],
};

// =============================================================================
// LOCALE BEHAVIOR
// =============================================================================

export type LocaleBehavior = 'invariant' | 'localized' | 'localeKnowledge' | 'derived' | 'job';

export const NODE_BEHAVIORS: Record<NodeType, LocaleBehavior> = {
  // Invariant (11)
  Project: 'invariant',
  BrandIdentity: 'invariant',
  Concept: 'invariant',
  Page: 'invariant',
  Block: 'invariant',
  PageType: 'invariant',
  BlockType: 'invariant',
  PagePrompt: 'invariant',
  BlockPrompt: 'invariant',
  BlockRules: 'invariant',
  Locale: 'invariant',
  // Localized (6)
  ProjectL10n: 'localized',
  ConceptL10n: 'localized',
  PageL10n: 'localized',
  BlockL10n: 'localized',
  SEOKeywordL10n: 'localized',
  GEOSeedL10n: 'localized',
  // LocaleKnowledge (14)
  LocaleIdentity: 'localeKnowledge',
  LocaleVoice: 'localeKnowledge',
  LocaleCulture: 'localeKnowledge',
  LocaleCultureReferences: 'localeKnowledge',
  LocaleMarket: 'localeKnowledge',
  LocaleLexicon: 'localeKnowledge',
  LocaleRulesAdaptation: 'localeKnowledge',
  LocaleRulesFormatting: 'localeKnowledge',
  LocaleRulesSlug: 'localeKnowledge',
  Expression: 'localeKnowledge',
  Reference: 'localeKnowledge',
  Metaphor: 'localeKnowledge',
  Pattern: 'localeKnowledge',
  Constraint: 'localeKnowledge',
  // Derived (2)
  SEOKeywordMetrics: 'derived',
  GEOSeedMetrics: 'derived',
  // Job (2)
  SEOMiningRun: 'job',
  GEOMiningRun: 'job',
};

// =============================================================================
// SCOPES
// =============================================================================

export type Scope = 'Global' | 'Shared' | 'Project';

export const NODE_SCOPES: Record<NodeType, Scope> = {
  // Global (15)
  Locale: 'Global',
  LocaleIdentity: 'Global',
  LocaleVoice: 'Global',
  LocaleCulture: 'Global',
  LocaleCultureReferences: 'Global',
  LocaleMarket: 'Global',
  LocaleLexicon: 'Global',
  LocaleRulesAdaptation: 'Global',
  LocaleRulesFormatting: 'Global',
  LocaleRulesSlug: 'Global',
  Expression: 'Global',
  Reference: 'Global',
  Metaphor: 'Global',
  Pattern: 'Global',
  Constraint: 'Global',
  // Shared (6)
  SEOKeywordL10n: 'Shared',
  SEOKeywordMetrics: 'Shared',
  SEOMiningRun: 'Shared',
  GEOSeedL10n: 'Shared',
  GEOSeedMetrics: 'Shared',
  GEOMiningRun: 'Shared',
  // Project (14)
  Project: 'Project',
  BrandIdentity: 'Project',
  ProjectL10n: 'Project',
  Concept: 'Project',
  ConceptL10n: 'Project',
  Page: 'Project',
  Block: 'Project',
  PageType: 'Project',
  BlockType: 'Project',
  PagePrompt: 'Project',
  BlockPrompt: 'Project',
  BlockRules: 'Project',
  PageL10n: 'Project',
  BlockL10n: 'Project',
};
