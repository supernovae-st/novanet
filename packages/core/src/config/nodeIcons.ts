// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// v11.3.0 — 3-layer shared realm, GEO layer, OrgConfig consolidation

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // SHARED — locale (7)
  Locale: '🌍',
  Formatting: '📝',
  Slugification: '🔗',
  Adaptation: '🔄',
  Style: '🎭',
  Culture: '🏺',
  Market: '📊',

  // SHARED — geography (6)
  Continent: '🗺️',
  GeoRegion: '🌐',
  GeoSubRegion: '🗾',
  IncomeGroup: '💰',
  LendingCategory: '🏦',
  EconomicRegion: '💹',

  // SHARED — knowledge (19)
  EntityCategory: '🏷️',
  TermSet: '📚',
  ExpressionSet: '💭',
  PatternSet: '🔣',
  CultureSet: '🏛️',
  TabooSet: '⛔',
  AudienceSet: '👥',
  Term: '📖',
  Expression: '💬',
  Pattern: '🔄',
  CultureRef: '🎭',
  Taboo: '🚫',
  AudienceTrait: '👤',
  LanguageFamily: '🗣️',
  LanguageBranch: '🌿',
  CulturalRealm: '🎪',
  CulturalSubRealm: '🎭',
  PopulationCluster: '👨‍👩‍👧‍👦',
  PopulationSubCluster: '👥',

  // ORG — config (1) — v11.3: Organization + Tenant merged
  OrgConfig: '🏢',

  // ORG — seo (5)
  SEOKeyword: '🔍',
  SEOKeywordMetrics: '📊',
  SEOComparison: '⚖️',
  SEOPreposition: '🔗',
  SEOQuestion: '❓',

  // ORG — geo (3) — v11.3: new layer split from seo
  GEOQuery: '🤖',
  GEOAnswer: '💬',
  GEOMetrics: '📊',

  // ORG — foundation (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectContent: '🌐',

  // ORG — structure (3)
  Page: '📄',
  Block: '🧱',
  ContentSlot: '🔲',

  // ORG — semantic (4)
  Entity: '🏷️',
  EntityContent: '🌐',
  AudiencePersona: '👤',
  ChannelSurface: '📡',

  // ORG — instruction (7)
  PageType: '📐',
  BlockType: '📋',
  PagePrompt: '📝',
  BlockPrompt: '📝',
  BlockRules: '📏',
  BlockInstruction: '📜',
  PromptArtifact: '📋',

  // ORG — output (3)
  PageGenerated: '📃',
  BlockGenerated: '📝',
  OutputArtifact: '📦',
};
