# Unified Types & Edge Styles Design

**Date**: 2026-01-30
**Version**: v8.1.0
**Status**: Approved

## Overview

Refactoring to unify NodeType definitions and add semantic edge styling for Mermaid diagrams.

### Goals

1. **Unify NodeType** - Single source of truth in `src/types/nodes.ts`
2. **Edge Categories** - 7 semantic categories with distinct Mermaid arrows
3. **Design System** - Extend `colors.ts` to handle both nodes and edges

## Architecture

### 1. Node Types (src/types/nodes.ts)

New file as single source of truth for all 35 node types.

```typescript
// src/types/nodes.ts

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

export type LocaleBehavior = 'invariant' | 'localized' | 'localeKnowledge' | 'derived' | 'job';

export const NODE_BEHAVIORS: Record<NodeType, LocaleBehavior> = {
  // Invariant
  Project: 'invariant', BrandIdentity: 'invariant', Concept: 'invariant',
  Page: 'invariant', Block: 'invariant', PageType: 'invariant', BlockType: 'invariant',
  PagePrompt: 'invariant', BlockPrompt: 'invariant', BlockRules: 'invariant', Locale: 'invariant',
  // Localized
  ProjectL10n: 'localized', ConceptL10n: 'localized', PageL10n: 'localized', BlockL10n: 'localized',
  SEOKeywordL10n: 'localized', GEOSeedL10n: 'localized',
  // LocaleKnowledge
  LocaleIdentity: 'localeKnowledge', LocaleVoice: 'localeKnowledge', LocaleCulture: 'localeKnowledge',
  LocaleCultureReferences: 'localeKnowledge', LocaleMarket: 'localeKnowledge', LocaleLexicon: 'localeKnowledge',
  LocaleRulesAdaptation: 'localeKnowledge', LocaleRulesFormatting: 'localeKnowledge',
  LocaleRulesSlug: 'localeKnowledge', Expression: 'localeKnowledge', Reference: 'localeKnowledge',
  Metaphor: 'localeKnowledge', Pattern: 'localeKnowledge', Constraint: 'localeKnowledge',
  // Derived
  SEOKeywordMetrics: 'derived', GEOSeedMetrics: 'derived',
  // Job
  SEOMiningRun: 'job', GEOMiningRun: 'job',
};

export type Scope = 'Global' | 'Shared' | 'Project';

export const NODE_SCOPES: Record<NodeType, Scope> = {
  // Global (15)
  Locale: 'Global', LocaleIdentity: 'Global', LocaleVoice: 'Global', LocaleCulture: 'Global',
  LocaleCultureReferences: 'Global', LocaleMarket: 'Global', LocaleLexicon: 'Global',
  LocaleRulesAdaptation: 'Global', LocaleRulesFormatting: 'Global', LocaleRulesSlug: 'Global',
  Expression: 'Global', Reference: 'Global', Metaphor: 'Global', Pattern: 'Global', Constraint: 'Global',
  // Shared (6)
  SEOKeywordL10n: 'Shared', SEOKeywordMetrics: 'Shared', SEOMiningRun: 'Shared',
  GEOSeedL10n: 'Shared', GEOSeedMetrics: 'Shared', GEOMiningRun: 'Shared',
  // Project (14)
  Project: 'Project', BrandIdentity: 'Project', ProjectL10n: 'Project',
  Concept: 'Project', ConceptL10n: 'Project', Page: 'Project', Block: 'Project',
  PageType: 'Project', BlockType: 'Project', PagePrompt: 'Project', BlockPrompt: 'Project',
  BlockRules: 'Project', PageL10n: 'Project', BlockL10n: 'Project',
};
```

### 2. Edge Categories (extend colors.ts)

```typescript
// Added to src/generators/colors.ts

export type EdgeCategory =
  | 'ownership'      // HAS_PAGE, HAS_BLOCK, HAS_CONCEPT
  | 'localization'   // FOR_LOCALE, HAS_L10N, HAS_OUTPUT
  | 'generation'     // GENERATED, ASSEMBLES, HAS_PROMPT
  | 'semantic'       // SEMANTIC_LINK, USES_CONCEPT
  | 'targeting'      // TARGETS_SEO, HAS_SEO_TARGET
  | 'inverse'        // L10N_OF, OUTPUT_OF, BLOCK_OF
  | 'hierarchy';     // OF_TYPE, HAS_RULES, SUBTOPIC_OF

export interface MermaidArrow {
  start: string;
  end: string;
}

export const EDGE_MERMAID_ARROWS: Record<EdgeCategory, MermaidArrow> = {
  ownership:     { start: '--',  end: '-->'  },  // A -->|REL| B
  localization:  { start: '-.',  end: '.->'  },  // A -.->|REL| B
  generation:    { start: '==',  end: '==>'  },  // A ==>|REL| B
  semantic:      { start: '~~',  end: '~~>'  },  // A ~~>|REL| B (Mermaid unsupported, fallback)
  targeting:     { start: '--',  end: '--o'  },  // A --o|REL| B
  inverse:       { start: '<--', end: '--'   },  // A <--|REL| B
  hierarchy:     { start: '--',  end: '-->'  },  // A -->|REL| B (same as ownership)
};

export const EDGE_COLORS: Record<EdgeCategory, string> = {
  ownership:    '#3b82f6',  // blue-500
  localization: '#22c55e',  // green-500
  generation:   '#a855f7',  // purple-500
  semantic:     '#f97316',  // orange-500
  targeting:    '#ec4899',  // pink-500
  inverse:      '#6b7280',  // gray-500
  hierarchy:    '#06b6d4',  // cyan-500
};

export const EDGE_TO_CATEGORY: Record<string, EdgeCategory> = {
  // Ownership (structure)
  HAS_CONCEPT: 'ownership',
  HAS_PAGE: 'ownership',
  HAS_BRAND_IDENTITY: 'ownership',
  HAS_BLOCK: 'ownership',
  HAS_IDENTITY: 'ownership',
  HAS_VOICE: 'ownership',
  HAS_CULTURE: 'ownership',
  HAS_MARKET: 'ownership',
  HAS_LEXICON: 'ownership',
  HAS_EXPRESSION: 'ownership',
  HAS_CULTURE_REFERENCES: 'ownership',
  HAS_REFERENCE: 'ownership',
  HAS_METAPHOR: 'ownership',
  HAS_PATTERN: 'ownership',
  HAS_CONSTRAINT: 'ownership',
  HAS_RULES_ADAPTATION: 'ownership',
  HAS_RULES_FORMATTING: 'ownership',
  HAS_RULES_SLUG: 'ownership',
  HAS_METRICS: 'ownership',

  // Localization
  FOR_LOCALE: 'localization',
  HAS_L10N: 'localization',
  HAS_OUTPUT: 'localization',
  SUPPORTS_LOCALE: 'localization',
  DEFAULT_LOCALE: 'localization',
  FALLBACK_TO: 'localization',
  VARIANT_OF: 'localization',
  HAS_LOCALIZED_CONTENT: 'localization',

  // Generation (AI output)
  GENERATED: 'generation',
  ASSEMBLES: 'generation',
  HAS_PROMPT: 'generation',
  GENERATED_FROM: 'generation',
  INFLUENCED_BY: 'generation',
  PREVIOUS_VERSION: 'generation',

  // Semantic (concepts)
  SEMANTIC_LINK: 'semantic',
  USES_CONCEPT: 'semantic',
  LINKS_TO: 'semantic',

  // Targeting (SEO/GEO)
  TARGETS_SEO: 'targeting',
  TARGETS_GEO: 'targeting',
  HAS_SEO_TARGET: 'targeting',
  HAS_GEO_TARGET: 'targeting',
  SEO_MINES: 'targeting',
  GEO_MINES: 'targeting',

  // Inverse
  L10N_OF: 'inverse',
  OUTPUT_OF: 'inverse',
  BLOCK_OF: 'inverse',
  USED_BY: 'inverse',
  BELONGS_TO_PROJECT_L10N: 'inverse',

  // Hierarchy (types/rules)
  OF_TYPE: 'hierarchy',
  HAS_RULES: 'hierarchy',
  SUBTOPIC_OF: 'hierarchy',
};
```

### 3. MermaidGenerator Updates

```typescript
// In generateMermaid() function, replace edge rendering:

// Before:
lines.push(`  ${edge.from} -->|${edge.relation}| ${edge.to}`);

// After:
const category = EDGE_TO_CATEGORY[edge.relation] ?? 'ownership';
const arrow = EDGE_MERMAID_ARROWS[category];
lines.push(`  ${edge.from} ${arrow.start}|${edge.relation}|${arrow.end} ${edge.to}`);

// Add linkStyle for colors (after edges):
const edgesByCategory = groupEdgesByCategory(sortedEdges);
let linkIndex = 0;
for (const [category, edges] of edgesByCategory) {
  const indices = edges.map(() => linkIndex++);
  const color = EDGE_COLORS[category];
  lines.push(`  linkStyle ${indices.join(',')} stroke:${color}`);
}
```

### 4. Import Updates

```typescript
// src/filters/types.ts
import { NodeType, NodeCategory, NODE_CATEGORIES } from '../types/nodes.js';
// Remove local NodeType definition

// src/generators/schemas.ts
import { NODE_TYPES } from '../types/nodes.js';
const NodeTypeSchema = z.enum(NODE_TYPES);
// Remove hardcoded array
```

## Files Changed

| File | Action | Description |
|------|--------|-------------|
| `src/types/nodes.ts` | CREATE | Single source of truth for 35 nodes |
| `src/types/index.ts` | MODIFY | Re-export nodes.ts |
| `src/generators/colors.ts` | EXTEND | Add EdgeCategory, arrows, mappings |
| `src/generators/schemas.ts` | MODIFY | Import NODE_TYPES, remove duplication |
| `src/generators/MermaidGenerator.ts` | MODIFY | Use edge categories for arrows |
| `src/filters/types.ts` | MODIFY | Import NodeType from types/nodes |

## Tests

1. **types.test.ts** - Verify 35 nodes, sync with _index.yaml
2. **colors.test.ts** - Verify all 50 relations have category mapping
3. **mermaid.test.ts** - Verify correct arrows per category

## Migration

1. Create `src/types/nodes.ts`
2. Update imports in `filters/types.ts` and `generators/schemas.ts`
3. Extend `colors.ts` with edge definitions
4. Update `MermaidGenerator.ts` to use new arrows
5. Run tests, regenerate docs
