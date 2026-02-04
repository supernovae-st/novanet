# Schema Mode v2 Implementation Plan

> **Superseded by v9**: This plan uses v8 terminology (Scope/Subcategory/NodeTypeMeta/DataMode).
> In v9, DataMode becomes NavigationMode (data/meta/overlay/query), Scope→Realm, Subcategory→Layer.
> See [`2026-02-01-ontology-v9-design.md`](2026-02-01-ontology-v9-design.md).

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform Schema Mode from force-directed layout to hierarchical grouped layout matching the Mermaid diagram in VIEW-COMPLETE-GRAPH.md

**Architecture:** Core provides schema graph data (source of truth), Studio visualizes it with ELK layout and React Flow group nodes. All metadata lives in @novanet/core so any consumer (Studio, agents, APIs) gets identical data.

**Tech Stack:** @novanet/core (TypeScript), @xyflow/react v12 (React Flow), elkjs (layout), Playwright (E2E)

---

## Critical Implementation Notes (From Code Review + Context7)

### P0 - Position Conversion (ELK → React Flow)

**Problem:** ELK returns **absolute** positions for all nodes, but React Flow requires **relative** positions for child nodes when using `parentId`.

**Solution:** After ELK layout, convert absolute positions to relative:
```typescript
// For each child node:
childNode.position = {
  x: elkChild.x - elkParent.x,
  y: elkChild.y - elkParent.y,
};
```

### P0 - Edge Validation

**Problem:** RelationRegistry may reference node types that don't exist in NODE_TYPES (defensive check needed).

**Solution:** Validate source/target types exist before creating edges.

### P1 - Empty Subcategories

**Problem:** Some subcategories may have 0 nodes (edge case), causing ELK to fail.

**Solution:** Skip empty subcategories in ELK graph building.

### P1 - React Flow Parent Node Requirements

From Context7 docs:
- Parent nodes need `type: 'group'` or custom group type
- Child nodes need `parentId` + `extent: 'parent'`
- Parent needs explicit `style: { width, height }` (from ELK layout)
- Children use relative positions within parent

---

## Phase 1: Core Graph Module (Foundation)

### Task 1.1: Create graph module structure in Core

**Files:**
- Create: `packages/core/src/graph/index.ts`
- Create: `packages/core/src/graph/types.ts`

**Step 1: Write the failing test**

```typescript
// packages/core/src/graph/__tests__/types.test.ts
import { SchemaNode, SchemaEdge, Subcategory, ScopeHierarchy } from '../types';

describe('graph/types', () => {
  it('should export SchemaNode interface', () => {
    const node: SchemaNode = {
      id: 'schema-Project',
      nodeType: 'Project',
      scope: 'Project',
      subcategory: 'foundation',
      label: 'Project',
      description: 'Project node',
    };
    expect(node.nodeType).toBe('Project');
  });

  it('should export Subcategory type with all values', () => {
    const subcats: Subcategory[] = [
      'foundation', 'structure', 'semantic', 'instruction', 'output',
      'config', 'knowledge',
      'seo', 'geo'
    ];
    expect(subcats).toHaveLength(9);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd packages/core && pnpm test -- --testPathPattern="graph/types" --watch`
Expected: FAIL with "Cannot find module '../types'"

**Step 3: Write minimal implementation**

```typescript
// packages/core/src/graph/types.ts
import type { NodeType, Scope } from '../types';
import type { RelationType } from '../schemas/relations.schema';

/**
 * Subcategory within a scope (from _index.yaml hierarchy)
 */
export type Subcategory =
  // Project scope
  | 'foundation'
  | 'structure'
  | 'semantic'
  | 'instruction'
  | 'output'
  // Global scope
  | 'config'
  | 'knowledge'
  // Shared scope
  | 'seo'
  | 'geo';

/**
 * Schema node representing a NodeType in the ontology
 */
export interface SchemaNode {
  id: string;
  nodeType: NodeType;
  scope: Scope;
  subcategory: Subcategory;
  label: string;
  description: string;
  behavior: string;
  icon?: string;
  color?: string;
}

/**
 * Schema edge representing a relation type
 */
export interface SchemaEdge {
  id: string;
  relationType: RelationType;
  sourceType: NodeType | NodeType[];
  targetType: NodeType | NodeType[];
  label: string;
  description: string;
  cardinality: string;
}

/**
 * Subcategory metadata
 */
export interface SubcategoryMeta {
  label: string;
  description: string;
  icon: string;
  nodeTypes: NodeType[];
}

/**
 * Scope hierarchy definition
 */
export interface ScopeDefinition {
  scope: Scope;
  label: string;
  icon: string;
  description: string;
  subcategories: Record<Subcategory, SubcategoryMeta>;
}

/**
 * Complete hierarchical schema data
 */
export interface HierarchicalSchemaData {
  scopes: Record<Scope, ScopeDefinition>;
  nodes: SchemaNode[];
  edges: SchemaEdge[];
  stats: {
    totalNodes: number;
    totalEdges: number;
    nodesByScope: Record<Scope, number>;
  };
}

/**
 * Flat schema graph result (for simple consumers)
 */
export interface SchemaGraphResult {
  nodes: SchemaNode[];
  edges: SchemaEdge[];
}
```

```typescript
// packages/core/src/graph/index.ts
export * from './types';
```

**Step 4: Run test to verify it passes**

Run: `cd packages/core && pnpm test -- --testPathPattern="graph/types"`
Expected: PASS

**Step 5: Commit**

```bash
git add packages/core/src/graph/
git commit -m "feat(core): add graph module with schema types

- Add SchemaNode, SchemaEdge interfaces
- Add Subcategory type (9 subcategories across 3 scopes)
- Add HierarchicalSchemaData for grouped schema
- Add ScopeDefinition for hierarchy metadata

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.2: Create NODE_SUBCATEGORIES mapping

**Files:**
- Create: `packages/core/src/graph/subcategories.ts`
- Test: `packages/core/src/graph/__tests__/subcategories.test.ts`

**Step 1: Write the failing test**

```typescript
// packages/core/src/graph/__tests__/subcategories.test.ts
import { NODE_SUBCATEGORIES, getSubcategory, getNodeTypesBySubcategory } from '../subcategories';
import { NODE_TYPES } from '../../types';

describe('graph/subcategories', () => {
  it('should map all 35 node types to subcategories', () => {
    const mappedTypes = Object.keys(NODE_SUBCATEGORIES);
    expect(mappedTypes).toHaveLength(35);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_SUBCATEGORIES[nodeType]).toBeDefined();
    }
  });

  it('should map Project scope nodes correctly', () => {
    expect(NODE_SUBCATEGORIES.Project).toBe('foundation');
    expect(NODE_SUBCATEGORIES.BrandIdentity).toBe('foundation');
    expect(NODE_SUBCATEGORIES.ProjectL10n).toBe('foundation');
    expect(NODE_SUBCATEGORIES.Page).toBe('structure');
    expect(NODE_SUBCATEGORIES.Block).toBe('structure');
    expect(NODE_SUBCATEGORIES.Concept).toBe('semantic');
    expect(NODE_SUBCATEGORIES.PageL10n).toBe('output');
    expect(NODE_SUBCATEGORIES.BlockL10n).toBe('output');
  });

  it('should map Global scope nodes correctly', () => {
    expect(NODE_SUBCATEGORIES.Locale).toBe('config');
    expect(NODE_SUBCATEGORIES.LocaleVoice).toBe('knowledge');
    expect(NODE_SUBCATEGORIES.LocaleCulture).toBe('knowledge');
    expect(NODE_SUBCATEGORIES.Expression).toBe('knowledge');
  });

  it('should map Shared scope nodes correctly', () => {
    expect(NODE_SUBCATEGORIES.SEOKeywordL10n).toBe('seo');
    expect(NODE_SUBCATEGORIES.SEOMiningRun).toBe('seo');
    expect(NODE_SUBCATEGORIES.GEOSeedL10n).toBe('geo');
    expect(NODE_SUBCATEGORIES.GEOMiningRun).toBe('geo');
  });

  it('getSubcategory should return correct subcategory', () => {
    expect(getSubcategory('Project')).toBe('foundation');
    expect(getSubcategory('Locale')).toBe('config');
  });

  it('getNodeTypesBySubcategory should return correct node types', () => {
    const foundation = getNodeTypesBySubcategory('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('BrandIdentity');
    expect(foundation).toContain('ProjectL10n');
    expect(foundation).toHaveLength(3);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd packages/core && pnpm test -- --testPathPattern="subcategories"`
Expected: FAIL

**Step 3: Write minimal implementation**

```typescript
// packages/core/src/graph/subcategories.ts
import type { NodeType } from '../types';
import type { Subcategory } from './types';

/**
 * Maps each NodeType to its subcategory within its scope.
 * This hierarchy matches the structure in models/_index.yaml
 */
export const NODE_SUBCATEGORIES: Record<NodeType, Subcategory> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // PROJECT SCOPE (14 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // foundation (3 nodes)
  Project: 'foundation',
  BrandIdentity: 'foundation',
  ProjectL10n: 'foundation',

  // structure (4 nodes)
  Page: 'structure',
  Block: 'structure',
  BlockType: 'structure',
  PageType: 'structure',

  // semantic (2 nodes)
  Concept: 'semantic',
  ConceptL10n: 'semantic',

  // instruction (3 nodes)
  PagePrompt: 'instruction',
  BlockPrompt: 'instruction',
  BlockRules: 'instruction',

  // output (2 nodes)
  PageL10n: 'output',
  BlockL10n: 'output',

  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL SCOPE (15 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // config (1 node)
  Locale: 'config',

  // knowledge (14 nodes)
  LocaleIdentity: 'knowledge',
  LocaleVoice: 'knowledge',
  LocaleCulture: 'knowledge',
  LocaleCultureReferences: 'knowledge',
  LocaleMarket: 'knowledge',
  LocaleLexicon: 'knowledge',
  LocaleRulesAdaptation: 'knowledge',
  LocaleRulesFormatting: 'knowledge',
  LocaleRulesSlug: 'knowledge',
  Expression: 'knowledge',
  Reference: 'knowledge',
  Metaphor: 'knowledge',
  Pattern: 'knowledge',
  Constraint: 'knowledge',

  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED SCOPE (6 nodes)
  // ═══════════════════════════════════════════════════════════════════════════

  // seo (3 nodes)
  SEOKeywordL10n: 'seo',
  SEOKeywordMetrics: 'seo',
  SEOMiningRun: 'seo',

  // geo (3 nodes)
  GEOSeedL10n: 'geo',
  GEOSeedMetrics: 'geo',
  GEOMiningRun: 'geo',
};

/**
 * Get the subcategory for a node type
 */
export function getSubcategory(nodeType: NodeType): Subcategory {
  return NODE_SUBCATEGORIES[nodeType];
}

/**
 * Get all node types in a subcategory
 */
export function getNodeTypesBySubcategory(subcategory: Subcategory): NodeType[] {
  return (Object.entries(NODE_SUBCATEGORIES) as [NodeType, Subcategory][])
    .filter(([, subcat]) => subcat === subcategory)
    .map(([nodeType]) => nodeType);
}
```

**Step 4: Update index.ts**

```typescript
// packages/core/src/graph/index.ts
export * from './types';
export * from './subcategories';
```

**Step 5: Run test to verify it passes**

Run: `cd packages/core && pnpm test -- --testPathPattern="subcategories"`
Expected: PASS

**Step 6: Commit**

```bash
git add packages/core/src/graph/
git commit -m "feat(core): add NODE_SUBCATEGORIES mapping

- Map all 35 node types to 9 subcategories
- Project: foundation, structure, semantic, instruction, output
- Global: config, knowledge
- Shared: seo, geo
- Add getSubcategory() and getNodeTypesBySubcategory() helpers

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.3: Create SCOPE_HIERARCHY configuration

**Files:**
- Create: `packages/core/src/graph/hierarchy.ts`
- Test: `packages/core/src/graph/__tests__/hierarchy.test.ts`

**Step 1: Write the failing test**

```typescript
// packages/core/src/graph/__tests__/hierarchy.test.ts
import { SCOPE_HIERARCHY, getScopeDefinition, getSubcategoryMeta } from '../hierarchy';
import type { Scope } from '../../types';

describe('graph/hierarchy', () => {
  it('should define all 3 scopes', () => {
    const scopes = Object.keys(SCOPE_HIERARCHY) as Scope[];
    expect(scopes).toHaveLength(3);
    expect(scopes).toContain('Project');
    expect(scopes).toContain('Global');
    expect(scopes).toContain('Shared');
  });

  it('should have correct Project scope structure', () => {
    const project = SCOPE_HIERARCHY.Project;
    expect(project.label).toBe('PROJECT');
    expect(project.icon).toBe('📦');
    expect(Object.keys(project.subcategories)).toHaveLength(5);
    expect(project.subcategories.foundation.nodeTypes).toContain('Project');
  });

  it('should have correct Global scope structure', () => {
    const global = SCOPE_HIERARCHY.Global;
    expect(global.label).toBe('GLOBAL');
    expect(global.icon).toBe('🌍');
    expect(Object.keys(global.subcategories)).toHaveLength(2);
    expect(global.subcategories.config.nodeTypes).toContain('Locale');
    expect(global.subcategories.knowledge.nodeTypes).toHaveLength(14);
  });

  it('should have correct Shared scope structure', () => {
    const shared = SCOPE_HIERARCHY.Shared;
    expect(shared.label).toBe('SHARED');
    expect(shared.icon).toBe('🎯');
    expect(Object.keys(shared.subcategories)).toHaveLength(2);
  });

  it('getScopeDefinition should return scope metadata', () => {
    const def = getScopeDefinition('Project');
    expect(def.label).toBe('PROJECT');
  });

  it('getSubcategoryMeta should return subcategory metadata', () => {
    const meta = getSubcategoryMeta('Project', 'foundation');
    expect(meta?.label).toBe('Foundation');
    expect(meta?.nodeTypes).toHaveLength(3);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd packages/core && pnpm test -- --testPathPattern="hierarchy"`
Expected: FAIL

**Step 3: Write minimal implementation**

```typescript
// packages/core/src/graph/hierarchy.ts
import type { Scope } from '../types';
import type { Subcategory, ScopeDefinition, SubcategoryMeta } from './types';
import { getNodeTypesBySubcategory } from './subcategories';

/**
 * Complete scope hierarchy definition.
 * This is the single source of truth for the ontology structure.
 */
export const SCOPE_HIERARCHY: Record<Scope, ScopeDefinition> = {
  Project: {
    scope: 'Project',
    label: 'PROJECT',
    icon: '📦',
    description: 'Project-specific content and structure',
    subcategories: {
      foundation: {
        label: 'Foundation',
        description: 'Core project identity and brand',
        icon: '🏛️',
        nodeTypes: getNodeTypesBySubcategory('foundation'),
      },
      structure: {
        label: 'Structure',
        description: 'Page and block organization',
        icon: '🧱',
        nodeTypes: getNodeTypesBySubcategory('structure'),
      },
      semantic: {
        label: 'Semantic',
        description: 'Concepts and meaning',
        icon: '💡',
        nodeTypes: getNodeTypesBySubcategory('semantic'),
      },
      instruction: {
        label: 'Instruction',
        description: 'Prompts and rules for generation',
        icon: '📝',
        nodeTypes: getNodeTypesBySubcategory('instruction'),
      },
      output: {
        label: 'Output',
        description: 'Generated localized content',
        icon: '📄',
        nodeTypes: getNodeTypesBySubcategory('output'),
      },
    } as Record<Subcategory, SubcategoryMeta>,
  },
  Global: {
    scope: 'Global',
    label: 'GLOBAL',
    icon: '🌍',
    description: 'Shared across all projects (Locale knowledge)',
    subcategories: {
      config: {
        label: 'Configuration',
        description: 'Locale configuration',
        icon: '⚙️',
        nodeTypes: getNodeTypesBySubcategory('config'),
      },
      knowledge: {
        label: 'Knowledge',
        description: 'Locale-specific cultural/linguistic knowledge',
        icon: '🧠',
        nodeTypes: getNodeTypesBySubcategory('knowledge'),
      },
    } as Record<Subcategory, SubcategoryMeta>,
  },
  Shared: {
    scope: 'Shared',
    label: 'SHARED',
    icon: '🎯',
    description: 'Shared across projects (SEO/GEO data)',
    subcategories: {
      seo: {
        label: 'SEO',
        description: 'Search engine optimization data',
        icon: '🔍',
        nodeTypes: getNodeTypesBySubcategory('seo'),
      },
      geo: {
        label: 'GEO',
        description: 'Generative engine optimization data',
        icon: '🤖',
        nodeTypes: getNodeTypesBySubcategory('geo'),
      },
    } as Record<Subcategory, SubcategoryMeta>,
  },
};

/**
 * Get scope definition by scope
 */
export function getScopeDefinition(scope: Scope): ScopeDefinition {
  return SCOPE_HIERARCHY[scope];
}

/**
 * Get subcategory metadata
 */
export function getSubcategoryMeta(
  scope: Scope,
  subcategory: Subcategory
): SubcategoryMeta | undefined {
  return SCOPE_HIERARCHY[scope]?.subcategories[subcategory];
}

/**
 * Get all subcategories for a scope
 */
export function getSubcategoriesForScope(scope: Scope): Subcategory[] {
  return Object.keys(SCOPE_HIERARCHY[scope].subcategories) as Subcategory[];
}
```

**Step 4: Update index.ts**

```typescript
// packages/core/src/graph/index.ts
export * from './types';
export * from './subcategories';
export * from './hierarchy';
```

**Step 5: Run test to verify it passes**

Run: `cd packages/core && pnpm test -- --testPathPattern="hierarchy"`
Expected: PASS

**Step 6: Commit**

```bash
git add packages/core/src/graph/
git commit -m "feat(core): add SCOPE_HIERARCHY configuration

- Define complete 3-scope hierarchy (Project, Global, Shared)
- Each scope has subcategories with metadata
- Add getScopeDefinition() and getSubcategoryMeta() helpers
- This is the source of truth for ontology structure

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.4: Create schema graph generator

**Files:**
- Create: `packages/core/src/graph/generator.ts`
- Test: `packages/core/src/graph/__tests__/generator.test.ts`

**Step 1: Write the failing test**

```typescript
// packages/core/src/graph/__tests__/generator.test.ts
import { generateSchemaGraph, getSchemaHierarchy } from '../generator';
import { NODE_TYPES } from '../../types';

describe('graph/generator', () => {
  describe('generateSchemaGraph', () => {
    it('should generate 35 schema nodes', () => {
      const result = generateSchemaGraph();
      expect(result.nodes).toHaveLength(35);
    });

    it('should generate schema edges from RelationRegistry', () => {
      const result = generateSchemaGraph();
      // RelationRegistry has 50 relation types, expanded to ~89 edges
      expect(result.edges.length).toBeGreaterThan(50);
    });

    it('should include all required node properties', () => {
      const result = generateSchemaGraph();
      const projectNode = result.nodes.find(n => n.nodeType === 'Project');

      expect(projectNode).toBeDefined();
      expect(projectNode?.id).toBe('schema-Project');
      expect(projectNode?.scope).toBe('Project');
      expect(projectNode?.subcategory).toBe('foundation');
      expect(projectNode?.label).toBe('Project');
    });

    it('should include all required edge properties', () => {
      const result = generateSchemaGraph();
      const hasPageEdge = result.edges.find(e => e.relationType === 'HAS_PAGE');

      expect(hasPageEdge).toBeDefined();
      expect(hasPageEdge?.sourceType).toBe('Project');
      expect(hasPageEdge?.targetType).toBe('Page');
    });
  });

  describe('getSchemaHierarchy', () => {
    it('should return hierarchical data with all 3 scopes', () => {
      const result = getSchemaHierarchy();
      expect(Object.keys(result.scopes)).toHaveLength(3);
    });

    it('should include stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalNodes).toBe(35);
      expect(result.stats.nodesByScope.Project).toBe(14);
      expect(result.stats.nodesByScope.Global).toBe(15);
      expect(result.stats.nodesByScope.Shared).toBe(6);
    });
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd packages/core && pnpm test -- --testPathPattern="generator"`
Expected: FAIL

**Step 3: Write minimal implementation**

```typescript
// packages/core/src/graph/generator.ts
import { NODE_TYPES, NODE_SCOPES, NODE_BEHAVIORS, type NodeType, type Scope } from '../types';
import { RelationRegistry } from '../schemas/relations.schema';
import type { SchemaNode, SchemaEdge, SchemaGraphResult, HierarchicalSchemaData } from './types';
import { NODE_SUBCATEGORIES, getSubcategory } from './subcategories';
import { SCOPE_HIERARCHY } from './hierarchy';

/**
 * Node display names (can be extended with icons/colors in UI layer)
 */
const NODE_LABELS: Record<NodeType, string> = {
  Project: 'Project',
  BrandIdentity: 'Brand Identity',
  ProjectL10n: 'Project L10n',
  Page: 'Page',
  Block: 'Block',
  BlockType: 'Block Type',
  PageType: 'Page Type',
  Concept: 'Concept',
  ConceptL10n: 'Concept L10n',
  PagePrompt: 'Page Prompt',
  BlockPrompt: 'Block Prompt',
  BlockRules: 'Block Rules',
  PageL10n: 'Page L10n',
  BlockL10n: 'Block L10n',
  Locale: 'Locale',
  LocaleIdentity: 'Locale Identity',
  LocaleVoice: 'Locale Voice',
  LocaleCulture: 'Locale Culture',
  LocaleCultureReferences: 'Culture References',
  LocaleMarket: 'Locale Market',
  LocaleLexicon: 'Locale Lexicon',
  LocaleRulesAdaptation: 'Rules Adaptation',
  LocaleRulesFormatting: 'Rules Formatting',
  LocaleRulesSlug: 'Rules Slug',
  Expression: 'Expression',
  Reference: 'Reference',
  Metaphor: 'Metaphor',
  Pattern: 'Pattern',
  Constraint: 'Constraint',
  SEOKeywordL10n: 'SEO Keyword',
  SEOKeywordMetrics: 'SEO Metrics',
  SEOMiningRun: 'SEO Mining',
  GEOSeedL10n: 'GEO Seed',
  GEOSeedMetrics: 'GEO Metrics',
  GEOMiningRun: 'GEO Mining',
};

/**
 * Scope descriptions for nodes
 */
const SCOPE_DESCRIPTIONS: Record<Scope, string> = {
  Global: 'Shared across all projects (Locale knowledge)',
  Shared: 'Shared across projects (SEO/GEO data)',
  Project: 'Project-specific content and structure',
};

/**
 * Behavior descriptions for nodes
 */
const BEHAVIOR_DESCRIPTIONS: Record<string, string> = {
  invariant: 'Language-independent, same across all locales',
  localized: 'Human-curated localized content',
  localeKnowledge: 'Locale-specific cultural/linguistic knowledge',
  derived: 'Computed from other data (metrics)',
  job: 'Background processing job',
};

/**
 * Generate flat schema graph with all 35 node types and relationships.
 * This is the canonical representation of the NovaNet ontology.
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: SchemaNode[] = [];
  const edges: SchemaEdge[] = [];

  // Generate nodes for all 35 node types
  for (const nodeType of NODE_TYPES) {
    const scope = NODE_SCOPES[nodeType];
    const behavior = NODE_BEHAVIORS[nodeType];
    const subcategory = getSubcategory(nodeType);

    nodes.push({
      id: `schema-${nodeType}`,
      nodeType,
      scope,
      subcategory,
      label: NODE_LABELS[nodeType],
      description: `${SCOPE_DESCRIPTIONS[scope]}. ${BEHAVIOR_DESCRIPTIONS[behavior]}.`,
      behavior,
    });
  }

  // Generate edges from RelationRegistry (single source of truth)
  // P0 FIX: Validate node types exist before creating edges
  const validNodeTypes = new Set(NODE_TYPES);
  let edgeId = 0;

  for (const relation of Object.values(RelationRegistry)) {
    const sourceTypes = Array.isArray(relation.from) ? relation.from : [relation.from];
    const targetTypes = Array.isArray(relation.to) ? relation.to : [relation.to];

    // Create Cartesian product of edges for multi-type relations
    for (const source of sourceTypes) {
      for (const target of targetTypes) {
        // P0: Skip edges with invalid node types
        if (!validNodeTypes.has(source as NodeType) || !validNodeTypes.has(target as NodeType)) {
          console.warn(`Skipping edge with invalid node type: ${source} -> ${target}`);
          continue;
        }

        edges.push({
          id: `schema-edge-${edgeId++}`,
          relationType: relation.type,
          sourceType: source as NodeType,
          targetType: target as NodeType,
          label: relation.type,
          description: relation.description,
          cardinality: relation.cardinality,
        });
      }
    }
  }

  return { nodes, edges };
}

/**
 * Generate hierarchical schema data grouped by scope and subcategory.
 * Used by visualizers that need grouped layout (like Studio).
 */
export function getSchemaHierarchy(): HierarchicalSchemaData {
  const { nodes, edges } = generateSchemaGraph();

  // Count nodes by scope
  const nodesByScope: Record<Scope, number> = {
    Project: 0,
    Global: 0,
    Shared: 0,
  };

  for (const node of nodes) {
    nodesByScope[node.scope]++;
  }

  return {
    scopes: SCOPE_HIERARCHY,
    nodes,
    edges,
    stats: {
      totalNodes: nodes.length,
      totalEdges: edges.length,
      nodesByScope,
    },
  };
}
```

**Step 4: Update index.ts and main exports**

```typescript
// packages/core/src/graph/index.ts
export * from './types';
export * from './subcategories';
export * from './hierarchy';
export * from './generator';
```

```typescript
// packages/core/src/index.ts - Add to existing exports
export * from './graph';
```

**Step 5: Run test to verify it passes**

Run: `cd packages/core && pnpm test -- --testPathPattern="generator"`
Expected: PASS

**Step 6: Commit**

```bash
git add packages/core/src/graph/ packages/core/src/index.ts
git commit -m "feat(core): add schema graph generator

- generateSchemaGraph() returns flat 35 nodes + ~89 edges
- getSchemaHierarchy() returns grouped data for visualizers
- Uses RelationRegistry as single source of truth
- Includes stats for node counts by scope

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 2: Studio Visual Components

### Task 2.1: Install ELK and create layout engine

**Files:**
- Modify: `apps/studio/package.json`
- Create: `apps/studio/src/lib/schemaLayoutELK.ts`
- Test: `apps/studio/src/lib/__tests__/schemaLayoutELK.test.ts`

**Step 1: Install ELK**

```bash
cd apps/studio && pnpm add elkjs
```

**Step 2: Write the failing test**

```typescript
// apps/studio/src/lib/__tests__/schemaLayoutELK.test.ts
import { applySchemaLayout } from '../schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';

describe('schemaLayoutELK', () => {
  it('should layout schema nodes with ELK', async () => {
    const hierarchy = getSchemaHierarchy();
    const result = await applySchemaLayout(hierarchy);

    // Should have group nodes + schema nodes
    expect(result.nodes.length).toBeGreaterThan(35);

    // All nodes should have positions
    for (const node of result.nodes) {
      expect(node.position).toBeDefined();
      expect(typeof node.position.x).toBe('number');
      expect(typeof node.position.y).toBe('number');
    }
  });

  it('should create scope group nodes', async () => {
    const hierarchy = getSchemaHierarchy();
    const result = await applySchemaLayout(hierarchy);

    const scopeGroups = result.nodes.filter(n => n.type === 'scopeGroup');
    expect(scopeGroups).toHaveLength(3);
  });

  it('should create subcategory group nodes', async () => {
    const hierarchy = getSchemaHierarchy();
    const result = await applySchemaLayout(hierarchy);

    const subGroups = result.nodes.filter(n => n.type === 'subcategoryGroup');
    expect(subGroups).toHaveLength(9); // 5 + 2 + 2
  });

  it('should set parent relationships', async () => {
    const hierarchy = getSchemaHierarchy();
    const result = await applySchemaLayout(hierarchy);

    const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');
    for (const node of schemaNodes) {
      expect(node.parentId).toBeDefined();
    }
  });
});
```

**Step 3: Write implementation**

```typescript
// apps/studio/src/lib/schemaLayoutELK.ts
import ELK, { ElkNode, ElkExtendedEdge } from 'elkjs/lib/elk.bundled.js';
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData, SchemaNode, Scope } from '@novanet/core/graph';

const elk = new ELK();

// Layout options for hierarchical schema
const ELK_OPTIONS = {
  'elk.algorithm': 'layered',
  'elk.direction': 'RIGHT',
  'elk.spacing.nodeNode': '20',
  'elk.layered.spacing.nodeNodeBetweenLayers': '50',
  'elk.hierarchyHandling': 'INCLUDE_CHILDREN',
  'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
};

// Node dimensions
const SCOPE_GROUP_PADDING = 40;
const SUBCATEGORY_GROUP_PADDING = 20;
const SCHEMA_NODE_WIDTH = 140;
const SCHEMA_NODE_HEIGHT = 50;

export interface SchemaLayoutResult {
  nodes: Node[];
  edges: Edge[];
}

/**
 * Apply ELK hierarchical layout to schema data
 */
export async function applySchemaLayout(
  hierarchy: HierarchicalSchemaData,
  direction: 'RIGHT' | 'DOWN' = 'RIGHT'
): Promise<SchemaLayoutResult> {
  const elkGraph = buildElkGraph(hierarchy, direction);

  try {
    const layoutedGraph = await elk.layout(elkGraph);
    return convertElkToReactFlow(layoutedGraph, hierarchy);
  } catch (error) {
    console.error('ELK layout failed:', error);
    // Fallback to simple grid layout
    return fallbackGridLayout(hierarchy);
  }
}

/**
 * Build ELK graph structure from hierarchical data
 *
 * P1 FIX: Skip empty subcategories to prevent ELK layout failures
 */
function buildElkGraph(
  hierarchy: HierarchicalSchemaData,
  direction: string
): ElkNode {
  const children: ElkNode[] = [];
  const edges: ElkExtendedEdge[] = [];

  // Create scope groups
  for (const [scopeName, scopeDef] of Object.entries(hierarchy.scopes)) {
    const scope = scopeName as Scope;
    const subcategoryChildren: ElkNode[] = [];

    // Create subcategory groups within scope
    for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      // P1 FIX: Skip empty subcategories (would cause ELK to fail)
      if (!subcatMeta.nodeTypes || subcatMeta.nodeTypes.length === 0) {
        console.warn(`Skipping empty subcategory: ${scope}/${subcatName}`);
        continue;
      }

      const nodeChildren: ElkNode[] = subcatMeta.nodeTypes.map(nodeType => ({
        id: `schema-${nodeType}`,
        width: SCHEMA_NODE_WIDTH,
        height: SCHEMA_NODE_HEIGHT,
        labels: [{ text: nodeType }], // ELK labels for debugging
      }));

      subcategoryChildren.push({
        id: `subcat-${scope}-${subcatName}`,
        layoutOptions: {
          'elk.padding': `[top=${SUBCATEGORY_GROUP_PADDING},left=${SUBCATEGORY_GROUP_PADDING},bottom=${SUBCATEGORY_GROUP_PADDING},right=${SUBCATEGORY_GROUP_PADDING}]`,
        },
        children: nodeChildren,
      });
    }

    // P1 FIX: Skip scope if all subcategories were empty
    if (subcategoryChildren.length === 0) {
      console.warn(`Skipping empty scope: ${scope}`);
      continue;
    }

    children.push({
      id: `scope-${scope}`,
      layoutOptions: {
        'elk.padding': `[top=${SCOPE_GROUP_PADDING},left=${SCOPE_GROUP_PADDING},bottom=${SCOPE_GROUP_PADDING},right=${SCOPE_GROUP_PADDING}]`,
      },
      children: subcategoryChildren,
    });
  }

  // Create edges (only between schema nodes, not groups)
  hierarchy.edges.forEach((edge, index) => {
    const sourceId = `schema-${edge.sourceType}`;
    const targetId = `schema-${edge.targetType}`;

    edges.push({
      id: `edge-${index}`,
      sources: [sourceId],
      targets: [targetId],
    });
  });

  return {
    id: 'root',
    layoutOptions: {
      ...ELK_OPTIONS,
      'elk.direction': direction,
    },
    children,
    edges,
  };
}

/**
 * Convert ELK layout result to React Flow nodes/edges
 *
 * P0 CRITICAL: ELK returns ABSOLUTE positions, but React Flow child nodes
 * require RELATIVE positions to their parent. We must convert:
 *   childPosition = { x: elkChild.x - elkParent.x, y: elkChild.y - elkParent.y }
 */
function convertElkToReactFlow(
  elkGraph: ElkNode,
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Process scope groups (top-level - use absolute positions)
  elkGraph.children?.forEach(scopeGroup => {
    const scopeId = scopeGroup.id;
    const scope = scopeId.replace('scope-', '') as Scope;
    const scopeDef = hierarchy.scopes[scope];

    // Scope group absolute position (from root)
    const scopeAbsX = scopeGroup.x || 0;
    const scopeAbsY = scopeGroup.y || 0;

    // Add scope group node (absolute position, it's top-level)
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: scopeAbsX, y: scopeAbsY },
      style: { width: scopeGroup.width, height: scopeGroup.height },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: hierarchy.stats.nodesByScope[scope],
      },
    });

    // Process subcategory groups
    scopeGroup.children?.forEach(subcatGroup => {
      const subcatId = subcatGroup.id;
      const subcatName = subcatId.split('-').slice(2).join('-');
      const subcatMeta = scopeDef.subcategories[subcatName as keyof typeof scopeDef.subcategories];

      // Subcategory absolute position (from ELK)
      const subcatAbsX = subcatGroup.x || 0;
      const subcatAbsY = subcatGroup.y || 0;

      // P0 FIX: Convert to RELATIVE position within scope parent
      const subcatRelX = subcatAbsX; // Already relative to parent in ELK nested structure
      const subcatRelY = subcatAbsY;

      // Add subcategory group node (relative to scope parent)
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: subcatRelX, y: subcatRelY },
        style: { width: subcatGroup.width, height: subcatGroup.height },
        data: {
          subcategory: subcatName,
          label: subcatMeta?.label || subcatName,
          icon: subcatMeta?.icon || '',
          nodeCount: subcatMeta?.nodeTypes.length || 0,
        },
      });

      // Process schema nodes
      subcatGroup.children?.forEach(schemaNode => {
        const nodeType = schemaNode.id.replace('schema-', '');
        const schemaData = hierarchy.nodes.find(n => n.nodeType === nodeType);

        // Schema node absolute position (from ELK)
        const nodeAbsX = schemaNode.x || 0;
        const nodeAbsY = schemaNode.y || 0;

        // P0 FIX: Convert to RELATIVE position within subcategory parent
        // ELK nested children already have positions relative to their parent
        const nodeRelX = nodeAbsX;
        const nodeRelY = nodeAbsY;

        nodes.push({
          id: schemaNode.id,
          type: 'schemaNode',
          parentId: subcatId,
          extent: 'parent',
          draggable: false, // Schema nodes are not user-draggable
          position: { x: nodeRelX, y: nodeRelY },
          data: {
            nodeType,
            label: schemaData?.label || nodeType,
            description: schemaData?.description || '',
            scope,
            subcategory: subcatName,
          },
        });
      });
    });
  });

  // Convert edges
  hierarchy.edges.forEach((edge, index) => {
    edges.push({
      id: `edge-${index}`,
      source: `schema-${edge.sourceType}`,
      target: `schema-${edge.targetType}`,
      type: 'schemaEdge',
      data: {
        relationType: edge.relationType,
        label: edge.label,
        description: edge.description,
      },
    });
  });

  return { nodes, edges };
}

/**
 * Fallback grid layout if ELK fails
 */
function fallbackGridLayout(hierarchy: HierarchicalSchemaData): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  let x = 0;
  let y = 0;
  const spacing = 200;

  hierarchy.nodes.forEach((node, index) => {
    nodes.push({
      id: `schema-${node.nodeType}`,
      type: 'schemaNode',
      position: { x: (index % 7) * spacing, y: Math.floor(index / 7) * spacing },
      data: {
        nodeType: node.nodeType,
        label: node.label,
        description: node.description,
        scope: node.scope,
        subcategory: node.subcategory,
      },
    });
  });

  hierarchy.edges.forEach((edge, index) => {
    edges.push({
      id: `edge-${index}`,
      source: `schema-${edge.sourceType}`,
      target: `schema-${edge.targetType}`,
      type: 'schemaEdge',
      data: {
        relationType: edge.relationType,
        label: edge.label,
      },
    });
  });

  return { nodes, edges };
}
```

**Step 4: Run test**

Run: `cd apps/studio && pnpm test -- --testPathPattern="schemaLayoutELK"`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/
git commit -m "feat(studio): add ELK layout engine for schema mode

- Install elkjs for hierarchical layout
- Create applySchemaLayout() with group support
- Handle scope groups and subcategory groups
- Fallback to grid layout if ELK fails

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.2: Create GroupNode components

**Files:**
- Create: `apps/studio/src/components/graph/schema/ScopeGroupNode.tsx`
- Create: `apps/studio/src/components/graph/schema/SubcategoryGroupNode.tsx`
- Create: `apps/studio/src/components/graph/schema/SchemaNode.tsx`
- Create: `apps/studio/src/components/graph/schema/index.ts`

**Step 1: Write test for ScopeGroupNode**

```typescript
// apps/studio/src/components/graph/schema/__tests__/ScopeGroupNode.test.tsx
import { render, screen } from '@testing-library/react';
import { ReactFlowProvider } from '@xyflow/react';
import { ScopeGroupNode } from '../ScopeGroupNode';

const mockProps = {
  id: 'scope-Project',
  data: {
    scope: 'Project',
    label: 'PROJECT',
    icon: '📦',
    nodeCount: 14,
  },
  selected: false,
};

describe('ScopeGroupNode', () => {
  it('should render scope label and icon', () => {
    render(
      <ReactFlowProvider>
        <ScopeGroupNode {...mockProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByText('📦 PROJECT')).toBeInTheDocument();
    expect(screen.getByText('14 types')).toBeInTheDocument();
  });
});
```

**Step 2: Write implementation**

```typescript
// apps/studio/src/components/graph/schema/ScopeGroupNode.tsx
import { memo } from 'react';
import { NodeProps, NodeResizer } from '@xyflow/react';
import { cn } from '@/lib/utils';

interface ScopeGroupData {
  scope: string;
  label: string;
  icon: string;
  nodeCount: number;
}

const SCOPE_COLORS: Record<string, string> = {
  Project: 'border-violet-500/50 bg-violet-500/5',
  Global: 'border-emerald-500/50 bg-emerald-500/5',
  Shared: 'border-amber-500/50 bg-amber-500/5',
};

export const ScopeGroupNode = memo(({ data, selected }: NodeProps<ScopeGroupData>) => {
  const colorClass = SCOPE_COLORS[data.scope] || 'border-gray-500/50 bg-gray-500/5';

  return (
    <div
      className={cn(
        'w-full h-full rounded-xl border-2 border-dashed',
        colorClass,
        selected && 'ring-2 ring-white/20'
      )}
    >
      <NodeResizer
        isVisible={selected}
        minWidth={200}
        minHeight={100}
        lineClassName="border-white/20"
        handleClassName="w-2 h-2 bg-white/50 border border-white/20"
      />
      <div className="absolute -top-7 left-3 flex items-center gap-2 px-2 py-1 rounded-md bg-black/80 backdrop-blur-sm">
        <span className="text-sm font-semibold text-white/90">
          {data.icon} {data.label}
        </span>
        <span className="text-xs text-white/50">
          {data.nodeCount} types
        </span>
      </div>
    </div>
  );
});

ScopeGroupNode.displayName = 'ScopeGroupNode';
```

```typescript
// apps/studio/src/components/graph/schema/SubcategoryGroupNode.tsx
import { memo } from 'react';
import { NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';

interface SubcategoryGroupData {
  subcategory: string;
  label: string;
  icon: string;
  nodeCount: number;
}

export const SubcategoryGroupNode = memo(({ data, selected }: NodeProps<SubcategoryGroupData>) => {
  return (
    <div
      className={cn(
        'w-full h-full rounded-lg border border-white/10 bg-white/5',
        selected && 'ring-1 ring-white/20'
      )}
    >
      <div className="absolute -top-5 left-2 flex items-center gap-1.5 px-1.5 py-0.5 rounded bg-black/60 backdrop-blur-sm">
        <span className="text-xs text-white/70">
          {data.icon} {data.label}
        </span>
        <span className="text-xs text-white/40">
          ({data.nodeCount})
        </span>
      </div>
    </div>
  );
});

SubcategoryGroupNode.displayName = 'SubcategoryGroupNode';
```

```typescript
// apps/studio/src/components/graph/schema/SchemaNode.tsx
import { memo } from 'react';
import { NodeProps, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';

interface SchemaNodeData {
  nodeType: string;
  label: string;
  description: string;
  scope: string;
  subcategory: string;
}

const SCOPE_ACCENT: Record<string, string> = {
  Project: 'border-l-violet-500',
  Global: 'border-l-emerald-500',
  Shared: 'border-l-amber-500',
};

export const SchemaNode = memo(({ data, selected }: NodeProps<SchemaNodeData>) => {
  const accentClass = SCOPE_ACCENT[data.scope] || 'border-l-gray-500';

  return (
    <div
      className={cn(
        'px-3 py-2 rounded-md bg-black/80 backdrop-blur-sm border border-white/10',
        'border-l-4',
        accentClass,
        selected && 'ring-2 ring-white/30'
      )}
    >
      <Handle type="target" position={Position.Left} className="w-2 h-2 !bg-white/50" />
      <div className="text-sm font-medium text-white/90">{data.label}</div>
      <div className="text-xs text-white/50 truncate max-w-[120px]">{data.nodeType}</div>
      <Handle type="source" position={Position.Right} className="w-2 h-2 !bg-white/50" />
    </div>
  );
});

SchemaNode.displayName = 'SchemaNode';
```

```typescript
// apps/studio/src/components/graph/schema/index.ts
export { ScopeGroupNode } from './ScopeGroupNode';
export { SubcategoryGroupNode } from './SubcategoryGroupNode';
export { SchemaNode } from './SchemaNode';
```

**Step 3: Run tests**

Run: `cd apps/studio && pnpm test -- --testPathPattern="schema"`
Expected: PASS

**Step 4: Commit**

```bash
git add apps/studio/src/components/graph/schema/
git commit -m "feat(studio): add schema visualization components

- ScopeGroupNode: Container for scope (Project/Global/Shared)
- SubcategoryGroupNode: Container for subcategory
- SchemaNode: Individual node type card
- Color-coded by scope with glassmorphism style

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 3: Integration & Mode Toggle

### Task 3.1: Add collapsed groups state to filterStore

**Files:**
- Modify: `apps/studio/src/stores/filterStore.ts`
- Test: `apps/studio/src/stores/__tests__/filterStore.test.ts`

**Step 1: Write the failing test**

```typescript
// apps/studio/src/stores/__tests__/filterStore.test.ts
import { useFilterStore } from '../filterStore';

describe('filterStore schema mode', () => {
  beforeEach(() => {
    useFilterStore.setState({ collapsedScopes: [], collapsedSubcategories: [] });
  });

  it('should toggle scope collapsed state', () => {
    const { toggleScopeCollapsed, collapsedScopes } = useFilterStore.getState();

    toggleScopeCollapsed('Project');
    expect(useFilterStore.getState().collapsedScopes).toContain('Project');

    toggleScopeCollapsed('Project');
    expect(useFilterStore.getState().collapsedScopes).not.toContain('Project');
  });

  it('should toggle subcategory collapsed state', () => {
    const { toggleSubcategoryCollapsed } = useFilterStore.getState();

    toggleSubcategoryCollapsed('Project', 'foundation');
    expect(useFilterStore.getState().collapsedSubcategories).toContain('Project-foundation');

    toggleSubcategoryCollapsed('Project', 'foundation');
    expect(useFilterStore.getState().collapsedSubcategories).not.toContain('Project-foundation');
  });

  it('should check if scope is collapsed', () => {
    const { toggleScopeCollapsed, isScopeCollapsed } = useFilterStore.getState();

    expect(isScopeCollapsed('Project')).toBe(false);
    toggleScopeCollapsed('Project');
    expect(useFilterStore.getState().isScopeCollapsed('Project')).toBe(true);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd apps/studio && pnpm test -- --testPathPattern="filterStore"`
Expected: FAIL

**Step 3: Update filterStore implementation**

```typescript
// apps/studio/src/stores/filterStore.ts - Add to existing store

interface FilterState {
  // ... existing state ...

  // Schema mode collapsed groups (P1 from code review)
  collapsedScopes: Scope[];
  collapsedSubcategories: string[]; // Format: "Scope-subcategory"

  // Actions
  toggleScopeCollapsed: (scope: Scope) => void;
  toggleSubcategoryCollapsed: (scope: Scope, subcategory: string) => void;
  isScopeCollapsed: (scope: Scope) => boolean;
  isSubcategoryCollapsed: (scope: Scope, subcategory: string) => boolean;
  resetSchemaFilters: () => void;
}

// Add to store creation:
collapsedScopes: [],
collapsedSubcategories: [],

toggleScopeCollapsed: (scope) =>
  set((state) => ({
    collapsedScopes: state.collapsedScopes.includes(scope)
      ? state.collapsedScopes.filter((s) => s !== scope)
      : [...state.collapsedScopes, scope],
  })),

toggleSubcategoryCollapsed: (scope, subcategory) =>
  set((state) => {
    const key = `${scope}-${subcategory}`;
    return {
      collapsedSubcategories: state.collapsedSubcategories.includes(key)
        ? state.collapsedSubcategories.filter((s) => s !== key)
        : [...state.collapsedSubcategories, key],
    };
  }),

isScopeCollapsed: (scope) => get().collapsedScopes.includes(scope),

isSubcategoryCollapsed: (scope, subcategory) =>
  get().collapsedSubcategories.includes(`${scope}-${subcategory}`),

resetSchemaFilters: () =>
  set({ collapsedScopes: [], collapsedSubcategories: [] }),
```

**Step 4: Run test to verify it passes**

Run: `cd apps/studio && pnpm test -- --testPathPattern="filterStore"`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/src/stores/
git commit -m "feat(studio): add collapsed groups state for schema mode

- Add collapsedScopes and collapsedSubcategories to filterStore
- Add toggle/check actions for collapse state
- Add resetSchemaFilters action
- Persisted via Zustand persist middleware

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.2: Register schema node types in Graph2D

**Files:**
- Modify: `apps/studio/src/components/graph/Graph2D.tsx`
- Test: `apps/studio/src/components/graph/__tests__/Graph2D.test.tsx`

**Step 1: Write the failing test**

```typescript
// apps/studio/src/components/graph/__tests__/Graph2D.test.tsx
import { render, screen, waitFor } from '@testing-library/react';
import { Graph2D } from '../Graph2D';

describe('Graph2D schema mode', () => {
  it('should render schema nodes when mode is schema', async () => {
    // Mock uiStore to return schema mode
    vi.mock('@/stores/uiStore', () => ({
      useUIStore: () => ({ dataMode: 'schema' }),
    }));

    render(<Graph2D />);

    await waitFor(() => {
      // Should see scope group nodes
      expect(screen.getByText('📦 PROJECT')).toBeInTheDocument();
    });
  });

  it('should use schemaNodeTypes when in schema mode', () => {
    // Verify nodeTypes includes scopeGroup, subcategoryGroup, schemaNode
    const nodeTypes = {
      scopeGroup: ScopeGroupNode,
      subcategoryGroup: SubcategoryGroupNode,
      schemaNode: SchemaNode,
    };

    expect(nodeTypes.scopeGroup).toBeDefined();
    expect(nodeTypes.subcategoryGroup).toBeDefined();
    expect(nodeTypes.schemaNode).toBeDefined();
  });
});
```

**Step 2: Update Graph2D component**

```typescript
// apps/studio/src/components/graph/Graph2D.tsx
import { useCallback, useMemo, useState, useEffect } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
} from '@xyflow/react';
import { useUIStore } from '@/stores/uiStore';
import { useFilterStore } from '@/stores/filterStore';

// Import schema components
import { ScopeGroupNode, SubcategoryGroupNode, SchemaNode } from './schema';
import { applySchemaLayout } from '@/lib/schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';

// Register custom node types
const nodeTypes = {
  // ... existing node types ...
  scopeGroup: ScopeGroupNode,
  subcategoryGroup: SubcategoryGroupNode,
  schemaNode: SchemaNode,
};

export function Graph2D() {
  const dataMode = useUIStore((s) => s.dataMode);
  const { collapsedScopes, collapsedSubcategories } = useFilterStore();

  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
  const [isLayouting, setIsLayouting] = useState(false);

  // Load schema graph when in schema mode
  useEffect(() => {
    if (dataMode === 'schema') {
      loadSchemaGraph();
    }
  }, [dataMode]);

  const loadSchemaGraph = useCallback(async () => {
    setIsLayouting(true);
    try {
      const hierarchy = getSchemaHierarchy();
      const { nodes: layoutedNodes, edges: layoutedEdges } = await applySchemaLayout(hierarchy);

      // Apply collapsed state (P1: filter out collapsed nodes)
      const visibleNodes = layoutedNodes.filter((node) => {
        if (node.type === 'scopeGroup') {
          return !collapsedScopes.includes(node.data.scope);
        }
        if (node.type === 'subcategoryGroup') {
          const key = `${node.data.scope}-${node.data.subcategory}`;
          return !collapsedSubcategories.includes(key);
        }
        if (node.type === 'schemaNode') {
          // Check if parent scope or subcategory is collapsed
          const parentSubcat = layoutedNodes.find((n) => n.id === node.parentId);
          if (parentSubcat) {
            const parentScope = layoutedNodes.find((n) => n.id === parentSubcat.parentId);
            if (parentScope && collapsedScopes.includes(parentScope.data.scope)) {
              return false;
            }
            const key = `${parentSubcat.data.scope}-${parentSubcat.data.subcategory}`;
            if (collapsedSubcategories.includes(key)) {
              return false;
            }
          }
        }
        return true;
      });

      setNodes(visibleNodes);
      setEdges(layoutedEdges);
    } catch (error) {
      console.error('Failed to load schema graph:', error);
    } finally {
      setIsLayouting(false);
    }
  }, [collapsedScopes, collapsedSubcategories, setNodes, setEdges]);

  // Re-layout when collapsed state changes
  useEffect(() => {
    if (dataMode === 'schema') {
      loadSchemaGraph();
    }
  }, [collapsedScopes, collapsedSubcategories, dataMode, loadSchemaGraph]);

  return (
    <div className="w-full h-full" data-testid="graph-container">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        fitView
        fitViewOptions={{ padding: 0.2 }}
        minZoom={0.1}
        maxZoom={2}
      >
        <Background />
        <Controls />
        <MiniMap />

        {/* Loading indicator */}
        {isLayouting && (
          <div className="absolute inset-0 flex items-center justify-center bg-black/50">
            <div className="text-white">Layouting schema...</div>
          </div>
        )}
      </ReactFlow>
    </div>
  );
}
```

**Step 3: Run test**

Run: `cd apps/studio && pnpm test -- --testPathPattern="Graph2D"`
Expected: PASS

**Step 4: Commit**

```bash
git add apps/studio/src/components/graph/
git commit -m "feat(studio): integrate schema mode in Graph2D

- Register schema node types (scopeGroup, subcategoryGroup, schemaNode)
- Load schema graph when dataMode is 'schema'
- Apply collapsed state from filterStore
- Add loading indicator during ELK layout

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.3: Add error boundary for layout failures

**Files:**
- Create: `apps/studio/src/components/graph/SchemaErrorBoundary.tsx`
- Modify: `apps/studio/src/components/graph/Graph2D.tsx`

**Step 1: Create error boundary**

```typescript
// apps/studio/src/components/graph/SchemaErrorBoundary.tsx
import { Component, type ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class SchemaErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('Schema layout error:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        this.props.fallback || (
          <div className="flex flex-col items-center justify-center h-full p-8 text-center">
            <div className="text-red-500 text-lg mb-2">Layout Error</div>
            <div className="text-white/70 text-sm mb-4">
              Failed to render schema graph
            </div>
            <div className="text-white/50 text-xs font-mono bg-black/50 p-2 rounded max-w-md overflow-auto">
              {this.state.error?.message}
            </div>
            <button
              onClick={() => this.setState({ hasError: false, error: null })}
              className="mt-4 px-4 py-2 bg-white/10 hover:bg-white/20 rounded text-white text-sm"
            >
              Retry
            </button>
          </div>
        )
      );
    }

    return this.props.children;
  }
}
```

**Step 2: Wrap schema mode in error boundary**

```typescript
// In Graph2D.tsx, wrap the schema rendering:
import { SchemaErrorBoundary } from './SchemaErrorBoundary';

// In return statement:
{dataMode === 'schema' && (
  <SchemaErrorBoundary>
    <ReactFlow ... />
  </SchemaErrorBoundary>
)}
```

**Step 3: Commit**

```bash
git add apps/studio/src/components/graph/
git commit -m "feat(studio): add error boundary for schema layout

- Create SchemaErrorBoundary component
- Catch and display layout errors gracefully
- Add retry button to recover from errors

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 4: Hierarchical Filters (Sidebar)

### Task 4.1: Create SchemaFilterPanel component

**Files:**
- Create: `apps/studio/src/components/sidebar/SchemaFilterPanel.tsx`
- Test: `apps/studio/src/components/sidebar/__tests__/SchemaFilterPanel.test.tsx`

**Step 1: Write the failing test**

```typescript
// apps/studio/src/components/sidebar/__tests__/SchemaFilterPanel.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { SchemaFilterPanel } from '../SchemaFilterPanel';

// Mock the stores
vi.mock('@/stores/filterStore', () => ({
  useFilterStore: () => ({
    collapsedScopes: [],
    collapsedSubcategories: [],
    toggleScopeCollapsed: vi.fn(),
    toggleSubcategoryCollapsed: vi.fn(),
    isScopeCollapsed: () => false,
    isSubcategoryCollapsed: () => false,
  }),
}));

describe('SchemaFilterPanel', () => {
  it('should render all 3 scopes', () => {
    render(<SchemaFilterPanel />);

    expect(screen.getByText('📦 PROJECT')).toBeInTheDocument();
    expect(screen.getByText('🌍 GLOBAL')).toBeInTheDocument();
    expect(screen.getByText('🎯 SHARED')).toBeInTheDocument();
  });

  it('should render subcategories for each scope', () => {
    render(<SchemaFilterPanel />);

    // Project subcategories
    expect(screen.getByText('Foundation')).toBeInTheDocument();
    expect(screen.getByText('Structure')).toBeInTheDocument();
    expect(screen.getByText('Semantic')).toBeInTheDocument();

    // Global subcategories
    expect(screen.getByText('Configuration')).toBeInTheDocument();
    expect(screen.getByText('Knowledge')).toBeInTheDocument();

    // Shared subcategories
    expect(screen.getByText('SEO')).toBeInTheDocument();
    expect(screen.getByText('GEO')).toBeInTheDocument();
  });

  it('should show node count for each subcategory', () => {
    render(<SchemaFilterPanel />);

    expect(screen.getByText('(3)')).toBeInTheDocument(); // foundation
    expect(screen.getByText('(14)')).toBeInTheDocument(); // knowledge
  });

  it('should toggle scope when clicked', () => {
    const toggleScopeCollapsed = vi.fn();
    vi.mock('@/stores/filterStore', () => ({
      useFilterStore: () => ({
        toggleScopeCollapsed,
        isScopeCollapsed: () => false,
      }),
    }));

    render(<SchemaFilterPanel />);
    fireEvent.click(screen.getByText('📦 PROJECT'));

    expect(toggleScopeCollapsed).toHaveBeenCalledWith('Project');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd apps/studio && pnpm test -- --testPathPattern="SchemaFilterPanel"`
Expected: FAIL

**Step 3: Write implementation**

```typescript
// apps/studio/src/components/sidebar/SchemaFilterPanel.tsx
import { memo } from 'react';
import { ChevronDown, ChevronRight } from 'lucide-react';
import { SCOPE_HIERARCHY, type Scope, type Subcategory } from '@novanet/core/graph';
import { useFilterStore } from '@/stores/filterStore';
import { cn } from '@/lib/utils';

const SCOPE_ORDER: Scope[] = ['Project', 'Global', 'Shared'];

export const SchemaFilterPanel = memo(function SchemaFilterPanel() {
  const {
    collapsedScopes,
    toggleScopeCollapsed,
    toggleSubcategoryCollapsed,
    isScopeCollapsed,
    isSubcategoryCollapsed,
  } = useFilterStore();

  return (
    <div className="p-4 space-y-4">
      <h3 className="text-sm font-semibold text-white/70 uppercase tracking-wider">
        Schema Filters
      </h3>

      {SCOPE_ORDER.map((scope) => {
        const scopeDef = SCOPE_HIERARCHY[scope];
        const isCollapsed = isScopeCollapsed(scope);

        return (
          <div key={scope} className="space-y-1">
            {/* Scope header */}
            <button
              onClick={() => toggleScopeCollapsed(scope)}
              className={cn(
                'w-full flex items-center justify-between px-2 py-1.5 rounded-md',
                'hover:bg-white/5 transition-colors',
                'text-left'
              )}
              aria-expanded={!isCollapsed}
              aria-controls={`scope-${scope}-content`}
            >
              <span className="flex items-center gap-2 text-sm font-medium text-white/90">
                {scopeDef.icon} {scopeDef.label}
              </span>
              <span className="flex items-center gap-2">
                <span className="text-xs text-white/50">
                  {Object.values(scopeDef.subcategories).reduce(
                    (sum, s) => sum + s.nodeTypes.length,
                    0
                  )} types
                </span>
                {isCollapsed ? (
                  <ChevronRight className="w-4 h-4 text-white/50" />
                ) : (
                  <ChevronDown className="w-4 h-4 text-white/50" />
                )}
              </span>
            </button>

            {/* Subcategories */}
            {!isCollapsed && (
              <div
                id={`scope-${scope}-content`}
                className="ml-4 space-y-0.5"
                role="region"
                aria-label={`${scopeDef.label} subcategories`}
              >
                {Object.entries(scopeDef.subcategories).map(([subcatName, subcatMeta]) => {
                  const isSubCollapsed = isSubcategoryCollapsed(scope, subcatName);

                  return (
                    <button
                      key={subcatName}
                      onClick={() => toggleSubcategoryCollapsed(scope, subcatName)}
                      className={cn(
                        'w-full flex items-center justify-between px-2 py-1 rounded',
                        'hover:bg-white/5 transition-colors',
                        'text-left text-sm',
                        isSubCollapsed ? 'text-white/40' : 'text-white/70'
                      )}
                      aria-pressed={!isSubCollapsed}
                    >
                      <span className="flex items-center gap-1.5">
                        {subcatMeta.icon} {subcatMeta.label}
                      </span>
                      <span className="text-xs text-white/40">
                        ({subcatMeta.nodeTypes.length})
                      </span>
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}

      {/* Stats footer */}
      <div className="pt-2 border-t border-white/10">
        <div className="text-xs text-white/40">
          35 node types • 9 subcategories • 3 scopes
        </div>
      </div>
    </div>
  );
});
```

**Step 4: Run test to verify it passes**

Run: `cd apps/studio && pnpm test -- --testPathPattern="SchemaFilterPanel"`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/src/components/sidebar/
git commit -m "feat(studio): add SchemaFilterPanel component

- Hierarchical filter by scope and subcategory
- Collapsible sections with icons
- Shows node counts per subcategory
- ARIA labels for accessibility (P1 from review)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 4.2: Integrate SchemaFilterPanel in sidebar

**Files:**
- Modify: `apps/studio/src/components/sidebar/FilterSidebar.tsx`

**Step 1: Update FilterSidebar**

```typescript
// apps/studio/src/components/sidebar/FilterSidebar.tsx
import { useUIStore } from '@/stores/uiStore';
import { SchemaFilterPanel } from './SchemaFilterPanel';
import { DataFilterPanel } from './DataFilterPanel'; // Existing component

export function FilterSidebar() {
  const dataMode = useUIStore((s) => s.dataMode);

  return (
    <aside className="w-64 h-full bg-black/80 backdrop-blur-sm border-r border-white/10 overflow-y-auto">
      {dataMode === 'schema' ? (
        <SchemaFilterPanel />
      ) : (
        <DataFilterPanel />
      )}
    </aside>
  );
}
```

**Step 2: Commit**

```bash
git add apps/studio/src/components/sidebar/
git commit -m "feat(studio): integrate SchemaFilterPanel in sidebar

- Show SchemaFilterPanel when dataMode is 'schema'
- Show DataFilterPanel when dataMode is 'data'

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 5: E2E Tests with Playwright

### Task 5.1: Schema mode toggle E2E test

**Files:**
- Create: `apps/studio/e2e/schema-mode.spec.ts`

**Step 1: Write E2E test**

```typescript
// apps/studio/e2e/schema-mode.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Schema Mode', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="graph-container"]');
  });

  test('should toggle between data and schema mode', async ({ page }) => {
    // Initially in data mode
    const toggle = page.getByRole('button', { name: /Data/i });
    await expect(toggle).toBeVisible();

    // Click to switch to schema mode
    await toggle.click();

    // Should show Schema button
    await expect(page.getByRole('button', { name: /Schema/i })).toBeVisible();

    // URL should update
    await expect(page).toHaveURL(/mode=schema/);

    // Should show 35 nodes
    await expect(page.getByText('35 nodes')).toBeVisible();
  });

  test('should display grouped layout in schema mode', async ({ page }) => {
    // Switch to schema mode
    await page.getByRole('button', { name: /Data/i }).click();

    // Wait for layout
    await page.waitForTimeout(500);

    // Should see scope groups
    await expect(page.getByText('📦 PROJECT')).toBeVisible();
    await expect(page.getByText('🌍 GLOBAL')).toBeVisible();
    await expect(page.getByText('🎯 SHARED')).toBeVisible();
  });

  test('should persist schema mode in URL', async ({ page }) => {
    // Go directly to schema mode URL
    await page.goto('/?mode=schema');

    // Should be in schema mode
    await expect(page.getByRole('button', { name: /Schema/i })).toBeVisible();
    await expect(page.getByText('35 nodes')).toBeVisible();
  });

  test('should filter schema nodes by scope', async ({ page }) => {
    // Switch to schema mode
    await page.getByRole('button', { name: /Data/i }).click();

    // Collapse Global scope
    await page.getByText('🌍 GLOBAL').click();

    // Should hide Global nodes
    await expect(page.getByText('Locale')).not.toBeVisible();

    // Should still show Project nodes
    await expect(page.getByText('Project')).toBeVisible();
  });
});
```

**Step 2: Run E2E tests**

```bash
cd apps/studio && pnpm exec playwright test schema-mode.spec.ts
```

---

## Summary

**Total Tasks:** 18 tasks across 5 phases

| Phase | Tasks | Focus |
|-------|-------|-------|
| **Phase 1** | 4 tasks | Core graph module (types, subcategories, hierarchy, generator) |
| **Phase 2** | 2 tasks | Studio visual components (ELK layout, GroupNodes) |
| **Phase 3** | 3 tasks | Graph2D integration, collapsed state, error boundary |
| **Phase 4** | 2 tasks | Hierarchical filters sidebar |
| **Phase 5** | 7 tests | E2E tests with Playwright |

**Key Deliverables:**
1. `@novanet/core/graph` module with schema generation
2. ELK layout engine for hierarchical visualization (with position conversion fix)
3. React Flow group nodes (Scope, Subcategory, Schema)
4. Collapsible filter sidebar with ARIA accessibility
5. Error boundary for graceful failure handling
6. E2E tests with Playwright

**Critical Fixes Applied (from Code Review + Architect + Context7):**
- P0: ELK returns absolute positions → converted to React Flow relative positions
- P0: Edge validation before creating schema edges
- P1: Skip empty subcategories to prevent ELK failures
- P1: Collapsed groups state management in filterStore
- P1: Error boundary for layout failures
- P1: ARIA labels for accessibility

**Success Criteria:**
- [ ] Schema mode shows 35 nodes in grouped layout
- [ ] Layout matches VIEW-COMPLETE-GRAPH.md structure
- [ ] Filters work at scope and subcategory levels
- [ ] URL sync works (`?mode=schema`)
- [ ] All tests pass (unit, integration, E2E)
- [ ] Error states handled gracefully
- [ ] Keyboard navigation works (ARIA)

---

## Recommended Execution Method

**Subagent-Driven Development** (same session, continuous progress)

**Why:**
1. Tasks are mostly independent (can run fresh subagent per task)
2. Code review after each task catches issues early
3. No context switch between planning and implementation
4. Fast iteration with quality gates

**Execution Flow:**
```
For each task:
  1. Dispatch fresh subagent → implements task with TDD
  2. Subagent reports back with summary
  3. Dispatch code-reviewer → validates implementation
  4. Fix any issues found
  5. Mark task complete, next task
```

**Final Verification Checkpoint:**
After all 18 tasks complete:
1. Run full test suite: `pnpm test`
2. Run E2E tests: `pnpm exec playwright test`
3. Type check: `pnpm type-check`
4. Manual verification: Toggle schema mode, check grouped layout
5. Dispatch final code-reviewer for entire implementation

**Required Skills:**
- `superpowers:test-driven-development` - TDD for each task
- `superpowers:requesting-code-review` - Review after each task
- `superpowers:finishing-a-development-branch` - Complete work at end
