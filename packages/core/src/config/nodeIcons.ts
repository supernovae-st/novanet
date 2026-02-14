// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// v11.5.0 — SEO/GEO moved to shared/knowledge, Locale to shared/config

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // SHARED — config (3)
  EntityCategory: '🏷️',
  Locale: '🌍',
  SEOKeywordFormat: '📋',

  // SHARED — locale (6)
  Formatting: '📝',
  Slugification: '🔗',
  Adaptation: '🔄',
  Style: '🎭',
  Culture: '🏺',
  Market: '📊',

  // SHARED — geography (7) — v0.12.4: Country added
  Continent: '🗺️',
  Country: '🏳️',
  GeoRegion: '🌐',
  GeoSubRegion: '🗾',
  IncomeGroup: '💰',
  LendingCategory: '🏦',
  EconomicRegion: '💹',

  // SHARED — knowledge (24) — includes SEO/GEO
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

  // SHARED — knowledge (SEO/GEO) — v11.5: moved from org
  SEOKeyword: '🔍',
  SEOKeywordMetrics: '📊',
  SEOKeywordSet: '📑',
  GEOQuery: '🤖',
  GEOQuerySet: '🗃️',
  GEOAnswer: '💬',

  // ORG — config (1) — v11.3: Organization + Tenant merged
  OrgConfig: '🏢',

  // ORG — foundation (6) — v0.12.4: Brand Architecture
  Project: '📦',
  Brand: '🎨',
  BrandDesign: '🎨',
  BrandPrinciples: '💡',
  PromptStyle: '✨',
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

  // ORG — instruction (4) — v0.12.4: PageStructure, PageInstruction deleted
  BlockType: '📋',
  BlockInstruction: '📜',
  BlockRules: '📏',
  PromptArtifact: '📋',

  // ORG — output (3)
  PageGenerated: '📃',
  BlockGenerated: '📝',
  OutputArtifact: '📦',
};
