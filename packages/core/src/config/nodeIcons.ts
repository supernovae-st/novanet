// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// v10.7.0 — 7-node locale knowledge architecture (global + tenant)

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // GLOBAL — config (7) - v10.7: added Culture, Market
  Locale: '🌍',
  Formatting: '📝',
  Slugification: '🔗',
  Adaptation: '🔄',
  Style: '🎭',
  Culture: '🏺',
  Market: '📊',

  // GLOBAL — locale-knowledge (12) — Sets + Atoms
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

  // GLOBAL — seo (6)
  SEOKeyword: '🔍',
  SEOKeywordMetrics: '📊',
  SEOMiningRun: '⚙️',
  SEOComparison: '⚖️',
  SEOPreposition: '🔗',
  SEOQuestion: '❓',

  // TENANT — config (1)
  Organization: '🏢',

  // TENANT — foundation (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectL10n: '🌐',

  // TENANT — structure (3)
  Page: '📄',
  Block: '🧱',
  ContentSlot: '🔲',

  // TENANT — semantic (4)
  Entity: '🏷️',
  EntityL10n: '🌐',
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
  PageL10n: '📃',
  BlockL10n: '📝',
  GenerationJob: '🚀',
  OutputArtifact: '📦',
  EvaluationSignal: '📊',
};
