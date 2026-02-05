// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// v10.3.0 — Entity-Centric Architecture, GEO removed

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // GLOBAL — config (1)
  Locale: '🌍',

  // GLOBAL — knowledge containers (10) — v10 tiered model
  Formatting: '📝',
  Slugification: '🔗',
  Adaptation: '🔄',
  Style: '🎭',
  TermSet: '📚',
  ExpressionSet: '💭',
  PatternSet: '🔣',
  CultureSet: '🏛️',
  TabooSet: '⛔',
  AudienceSet: '👥',

  // GLOBAL — knowledge atoms (6)
  Term: '📖',
  Expression: '💬',
  Pattern: '🔄',
  CultureRef: '🎭',
  Taboo: '🚫',
  AudienceTrait: '👤',

  // GLOBAL — seo (3)
  SEOKeyword: '🔍',
  SEOKeywordMetrics: '📊',
  SEOMiningRun: '⚙️',

  // GLOBAL — semantic (2) — v10.3 Entity-Centric Architecture
  Entity: '💡',
  EntityL10n: '💬',

  // PROJECT — foundation (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectL10n: '🌐',

  // PROJECT — structure (5)
  Page: '📄',
  Block: '🧱',
  ContentSlot: '🔲',
  PageType: '📐',
  BlockType: '📋',

  // PROJECT — semantic (2)
  AudiencePersona: '👤',
  ChannelSurface: '📡',

  // PROJECT — instruction (5)
  PagePrompt: '📝',
  BlockPrompt: '📝',
  BlockRules: '📏',
  BlockInstruction: '📜',
  PromptArtifact: '📋',

  // PROJECT — output (5)
  PageL10n: '📃',
  BlockL10n: '📝',
  GenerationJob: '🚀',
  OutputArtifact: '📦',
  EvaluationSignal: '📊',
};
