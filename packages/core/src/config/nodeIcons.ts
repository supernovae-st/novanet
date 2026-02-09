// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// v11.1.0 — EntityCategory classification system

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // GLOBAL — config (14) - v11.1: added EntityCategory
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

  // GLOBAL — locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
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

  // GLOBAL — seo (9) — SEO + GEO (Generative Engine Optimization)
  SEOKeyword: '🔍',
  SEOKeywordMetrics: '📊',
  SEOMiningRun: '⚙️',
  SEOComparison: '⚖️',
  SEOPreposition: '🔗',
  SEOQuestion: '❓',
  GEOQuery: '🤖',
  GEOAnswer: '💬',
  GEOMetrics: '📊',

  // TENANT — config (2)
  Organization: '🏢',
  Tenant: '🏠',

  // TENANT — foundation (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectContent: '🌐',

  // TENANT — structure (3)
  Page: '📄',
  Block: '🧱',
  ContentSlot: '🔲',

  // TENANT — semantic (4)
  Entity: '🏷️',
  EntityContent: '🌐',
  AudiencePersona: '👤',
  ChannelSurface: '📡',

  // TENANT — instruction (7)
  PageType: '📐',
  BlockType: '📋',
  PagePrompt: '📝',
  BlockPrompt: '📝',
  BlockRules: '📏',
  BlockInstruction: '📜',
  PromptArtifact: '📋',

  // TENANT — output (5)
  PageGenerated: '📃',
  BlockGenerated: '📝',
  GenerationJob: '🚀',
  OutputArtifact: '📦',
  EvaluationSignal: '📊',
};
