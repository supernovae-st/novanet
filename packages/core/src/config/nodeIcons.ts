// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// v11.2.0 — Realm renames (shared/org), job nodes removed

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // SHARED — config (14)
  Locale: '🌍',
  EntityCategory: '🏷️',
  Formatting: '📝',
  Slugification: '🔗',
  Adaptation: '🔄',
  Style: '🎭',
  Culture: '🏺',
  Market: '📊',
  Continent: '🗺️',
  GeoRegion: '🌐',
  GeoSubRegion: '🗾',
  IncomeGroup: '💰',
  LendingCategory: '🏦',
  EconomicRegion: '💹',

  // SHARED — locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
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

  // ORG — seo (8) — SEO + GEO (Generative Engine Optimization)
  SEOKeyword: '🔍',
  SEOKeywordMetrics: '📊',
  SEOComparison: '⚖️',
  SEOPreposition: '🔗',
  SEOQuestion: '❓',
  GEOQuery: '🤖',
  GEOAnswer: '💬',
  GEOMetrics: '📊',

  // ORG — config (2)
  Organization: '🏢',
  Tenant: '🏠',

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
