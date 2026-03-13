# Sidebar Query Refactor - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform sidebar from client-side filtering to query execution for better UX and architecture alignment with Neo4j Browser patterns.

**Architecture:** Replace "Filters" tab with "Query" tab. When user clicks a Quick Query, it executes a real Cypher query against Neo4j (not just client-side filtering). Remove LocaleDropdown. Keep node labels as query launchers.

**Tech Stack:** React, Zustand, Neo4j Cypher, TypeScript

---

## Summary of Changes

```
BEFORE                           AFTER
──────                           ─────
Sidebar Tabs:                    Sidebar Tabs:
├── Database                     ├── Database (unchanged)
└── Filters                      └── Query
    ├── QuickViews (filter)          ├── QueryLibrary (executes Cypher)
    ├── LocaleDropdown               └── NodeLabels (executes Cypher)
    └── LabelFilter (filter)

User clicks "Content Pipeline":  User clicks "Content Pipeline":
→ Filters visible nodes          → Executes: MATCH (n) WHERE n:Page OR n:Block...
→ No API call                    → Fetches fresh data from Neo4j
→ Limited to loaded data         → Shows actual database content
```

---

## Task 1: Create QueryLibrary Config

**Files:**
- Create: `src/config/queryLibrary.ts`

**Step 1: Write the config file**

```typescript
/**
 * QueryLibrary - Pre-made Cypher queries for common workflows
 *
 * Each query represents a useful view of the NovaNet graph.
 * Clicking a query executes it against Neo4j.
 */

export interface QueryPreset {
  /** Unique identifier */
  id: string;
  /** Display name */
  name: string;
  /** Brief description */
  description: string;
  /** Emoji icon */
  icon: string;
  /** Cypher query to execute */
  cypher: string;
  /** Category for grouping */
  category: 'workflow' | 'exploration' | 'analytics';
}

const QUERY_LIMIT = 200;

export const QUERY_LIBRARY: QueryPreset[] = [
  // === WORKFLOW QUERIES ===
  {
    id: 'content-pipeline',
    name: 'Content Pipeline',
    description: 'Page → Block → Output structure',
    icon: '📄',
    category: 'workflow',
    cypher: `MATCH (p:Page)-[:HAS_BLOCK]->(b:Block)-[:OF_TYPE]->(bt:BlockType)
OPTIONAL MATCH (b)-[:HAS_OUTPUT]->(bo:BlockOutput)
OPTIONAL MATCH (p)-[:HAS_OUTPUT]->(po:PageOutput)
RETURN p, b, bt, bo, po LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'Locale → Identity / Voice / Culture',
    icon: '🌍',
    category: 'workflow',
    cypher: `MATCH (l:Locale)
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)
RETURN l, li, lv, lc, lm, ll LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'concept-network',
    name: 'Concept Network',
    description: 'Concepts with semantic links',
    icon: '🧠',
    category: 'workflow',
    cypher: `MATCH (c:Concept)
OPTIONAL MATCH (c)-[sl:SEMANTIC_LINK]-(c2:Concept)
OPTIONAL MATCH (c)-[:HAS_L10N]->(cl:ConceptL10n)
RETURN c, sl, c2, cl LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'project-structure',
    name: 'Project Structure',
    description: 'Project → Brand → L10n',
    icon: '🏢',
    category: 'workflow',
    cypher: `MATCH (p:Project)
OPTIONAL MATCH (p)-[:HAS_BRAND_IDENTITY]->(bi:BrandIdentity)
OPTIONAL MATCH (p)-[:HAS_L10N]->(pl:ProjectL10n)
OPTIONAL MATCH (p)-[:HAS_PAGE]->(pg:Page)
RETURN p, bi, pl, pg LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'generation-pipeline',
    name: 'Generation Pipeline',
    description: 'Prompts → Rules → Outputs',
    icon: '🤖',
    category: 'workflow',
    cypher: `MATCH (b:Block)
OPTIONAL MATCH (b)-[:HAS_PROMPT]->(bp:BlockPrompt)
OPTIONAL MATCH (b)-[:HAS_OUTPUT]->(bo:BlockOutput)
RETURN b, bp, bo LIMIT ${QUERY_LIMIT}`,
  },

  // === EXPLORATION QUERIES ===
  {
    id: 'seo-keywords',
    name: 'SEO Keywords',
    description: 'Keywords with volume & difficulty',
    icon: '🔍',
    category: 'exploration',
    cypher: `MATCH (s:SEOKeyword)
OPTIONAL MATCH (s)<-[:TARGETS_SEO]-(c:Concept)
OPTIONAL MATCH (s)-[:FOR_LOCALE]->(l:Locale)
RETURN s, c, l ORDER BY s.volume DESC LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'geo-seeds',
    name: 'GEO Seeds',
    description: 'AI optimization seeds',
    icon: '🎯',
    category: 'exploration',
    cypher: `MATCH (g:GEOSeed)
OPTIONAL MATCH (g)<-[:TARGETS_GEO]-(c:Concept)
OPTIONAL MATCH (g)-[:FOR_LOCALE]->(l:Locale)
RETURN g, c, l LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'expressions',
    name: 'Marketing Expressions',
    description: 'Locale-specific expressions',
    icon: '💬',
    category: 'exploration',
    cypher: `MATCH (ll:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
MATCH (l:Locale)-[:HAS_LEXICON]->(ll)
RETURN l, ll, e LIMIT ${QUERY_LIMIT}`,
  },

  // === ANALYTICS QUERIES ===
  {
    id: 'content-gaps',
    name: 'Content Gaps',
    description: 'SEO/GEO gaps to fill',
    icon: '📊',
    category: 'analytics',
    cypher: `MATCH (sv:SEOVariation {content_gap: true})
OPTIONAL MATCH (sv)<-[:HAS_VARIATION]-(sk:SEOKeyword)
RETURN sv, sk ORDER BY sv.volume DESC LIMIT ${QUERY_LIMIT}
UNION
MATCH (gr:GEOReformulation {content_gap: true})
OPTIONAL MATCH (gr)<-[:HAS_REFORMULATION]-(gs:GEOSeed)
RETURN gr, gs ORDER BY gr.frequency DESC LIMIT ${QUERY_LIMIT}`,
  },
  {
    id: 'recent-outputs',
    name: 'Recent Outputs',
    description: 'Latest generated content',
    icon: '🕐',
    category: 'analytics',
    cypher: `MATCH (bo:BlockOutput)
OPTIONAL MATCH (bo)<-[:HAS_OUTPUT]-(b:Block)
RETURN bo, b ORDER BY bo.generated_at DESC LIMIT 50`,
  },
];

/**
 * Generate query to fetch nodes by label
 */
export function queryByLabel(label: string): string {
  return `MATCH (n:${label}) RETURN n LIMIT ${QUERY_LIMIT}`;
}

/**
 * Generate query to fetch nodes by label with relationships
 */
export function queryByLabelWithRels(label: string): string {
  return `MATCH (n:${label})-[r]-(m) RETURN n, r, m LIMIT ${QUERY_LIMIT}`;
}
```

**Step 2: Verify file created**

Run: `cat src/config/queryLibrary.ts | head -20`
Expected: File content visible

**Step 3: Commit**

```bash
git add src/config/queryLibrary.ts
git commit -m "feat(query): add QueryLibrary with pre-made Cypher queries"
```

---

## Task 2: Create QueryLibrary Component

**Files:**
- Create: `src/components/sidebar/QueryLibrary.tsx`

**Step 1: Write the component**

```typescript
'use client';

/**
 * QueryLibrary - Pre-made Cypher query launcher
 *
 * Features:
 * - Grouped by category (workflow, exploration, analytics)
 * - Click to execute query against Neo4j
 * - Shows loading state while query executes
 * - Highlights active query
 */

import { useState, useMemo } from 'react';
import { Play, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useQueryStore } from '@/stores/queryStore';
import { QUERY_LIBRARY, type QueryPreset } from '@/config/queryLibrary';

interface QueryLibraryProps {
  className?: string;
}

const CATEGORY_CONFIG = {
  workflow: { label: 'WORKFLOW', icon: '⚡' },
  exploration: { label: 'EXPLORATION', icon: '🔭' },
  analytics: { label: 'ANALYTICS', icon: '📈' },
};

const CATEGORY_ORDER: Array<'workflow' | 'exploration' | 'analytics'> = [
  'workflow',
  'exploration',
  'analytics',
];

export function QueryLibrary({ className }: QueryLibraryProps) {
  const currentQuery = useQueryStore((state) => state.currentQuery);
  const isExecuting = useQueryStore((state) => state.isExecuting);
  const executeQuery = useQueryStore((state) => state.executeQuery);

  const [executingId, setExecutingId] = useState<string | null>(null);

  // Group queries by category
  const categorizedQueries = useMemo(() => {
    return CATEGORY_ORDER.map((category) => ({
      category,
      ...CATEGORY_CONFIG[category],
      queries: QUERY_LIBRARY.filter((q) => q.category === category),
    })).filter((cat) => cat.queries.length > 0);
  }, []);

  // Handle query execution
  const handleExecute = async (query: QueryPreset) => {
    setExecutingId(query.id);
    try {
      await executeQuery(query.cypher);
    } finally {
      setExecutingId(null);
    }
  };

  // Check if a query is active (matches current query)
  const isActive = (query: QueryPreset) => currentQuery === query.cypher;

  return (
    <div className={cn('space-y-4', className)}>
      {/* Header */}
      <h3 className="text-xs font-semibold text-white/50 uppercase tracking-wider px-1">
        Query Library
      </h3>

      {/* Categories */}
      {categorizedQueries.map(({ category, label, queries }) => (
        <div key={category} className="space-y-1.5">
          {/* Category Header */}
          <div className="flex items-center gap-2 px-1 py-1">
            <span className="text-[10px] font-semibold text-white/40 uppercase tracking-widest">
              {label}
            </span>
            <div className="flex-1 h-px bg-white/[0.06]" />
          </div>

          {/* Queries */}
          <div className="space-y-1">
            {queries.map((query) => {
              const active = isActive(query);
              const executing = executingId === query.id;

              return (
                <button
                  key={query.id}
                  onClick={() => handleExecute(query)}
                  disabled={isExecuting}
                  className={cn(
                    'w-full text-left p-3 rounded-lg transition-all duration-150',
                    'border group',
                    active
                      ? 'bg-novanet-500/15 border-novanet-500/40 shadow-sm shadow-novanet-500/10'
                      : 'bg-white/[0.03] border-white/[0.06] hover:bg-white/[0.06] hover:border-white/10',
                    isExecuting && !executing && 'opacity-50 cursor-not-allowed'
                  )}
                >
                  {/* Icon, Name, and Play button */}
                  <div className="flex items-center gap-2">
                    <span className={cn('text-base', active && 'scale-110')}>
                      {query.icon}
                    </span>
                    <span className={cn('font-medium text-sm flex-1', active ? 'text-white' : 'text-white/80')}>
                      {query.name}
                    </span>
                    {executing ? (
                      <Loader2 className="w-4 h-4 text-novanet-400 animate-spin" />
                    ) : (
                      <Play className={cn(
                        'w-3.5 h-3.5 transition-opacity',
                        active ? 'text-novanet-400 opacity-100' : 'text-white/30 opacity-0 group-hover:opacity-100'
                      )} />
                    )}
                  </div>

                  {/* Description */}
                  <p className={cn('text-[11px] mt-1 pl-6', active ? 'text-white/60' : 'text-white/40')}>
                    {query.description}
                  </p>
                </button>
              );
            })}
          </div>
        </div>
      ))}

      {/* Footer hint */}
      <p className="text-[10px] text-white/30 text-center px-2 pt-1">
        Click to execute query
      </p>
    </div>
  );
}
```

**Step 2: Commit**

```bash
git add src/components/sidebar/QueryLibrary.tsx
git commit -m "feat(query): add QueryLibrary component"
```

---

## Task 3: Create NodeLabels Query Component

**Files:**
- Create: `src/components/sidebar/NodeLabels.tsx`

**Step 1: Write the component**

Transforms LabelFilter from client-side filtering to query execution.

```typescript
'use client';

/**
 * NodeLabels - Click node type to query all nodes of that type
 *
 * Similar to LabelFilter but executes Cypher queries instead of filtering.
 * - Click = query all nodes of type
 * - Shift+click = query with relationships
 */

import { useState, useMemo, type MouseEvent } from 'react';
import { Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useGraphStore } from '@/stores/graphStore';
import { useQueryStore } from '@/stores/queryStore';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { NODE_CATEGORIES, type NodeCategory } from '@/lib/filterAdapter';
import { queryByLabel, queryByLabelWithRels } from '@/config/queryLibrary';
import type { NodeType } from '@/types';

const CATEGORY_CONFIG: Record<NodeCategory, { label: string }> = {
  project: { label: 'PROJECT' },
  content: { label: 'CONTENT' },
  locale: { label: 'LOCALE' },
  generation: { label: 'GENERATION' },
  seo: { label: 'SEO' },
  geo: { label: 'GEO' },
  analytics: { label: 'ANALYTICS' },
};

const CATEGORY_ORDER: NodeCategory[] = [
  'project', 'content', 'locale', 'generation', 'seo', 'geo', 'analytics',
];

interface NodeLabelsProps {
  className?: string;
}

export function NodeLabels({ className }: NodeLabelsProps) {
  const nodeTypeCounts = useGraphStore((state) => state.nodeTypeCounts);
  const executeQuery = useQueryStore((state) => state.executeQuery);
  const isExecuting = useQueryStore((state) => state.isExecuting);

  const [executingType, setExecutingType] = useState<NodeType | null>(null);

  // Group node types by category
  const categorizedTypes = useMemo(() => {
    return CATEGORY_ORDER.map((category) => {
      const types = NODE_CATEGORIES[category];
      const typesWithCounts = types.map((type) => ({
        type,
        config: NODE_TYPE_CONFIG[type],
        count: nodeTypeCounts[type] || 0,
      }));
      const typesWithNodes = typesWithCounts.filter((t) => t.count > 0);
      return {
        category,
        ...CATEGORY_CONFIG[category],
        types: typesWithCounts,
        typesWithNodes,
      };
    }).filter((cat) => cat.typesWithNodes.length > 0);
  }, [nodeTypeCounts]);

  // Handle click - execute query
  const handleClick = async (type: NodeType, event: MouseEvent) => {
    if (isExecuting) return;

    setExecutingType(type);
    try {
      const query = event.shiftKey
        ? queryByLabelWithRels(type)  // With relationships
        : queryByLabel(type);          // Just nodes
      await executeQuery(query);
    } finally {
      setExecutingType(null);
    }
  };

  return (
    <div className={cn('space-y-4', className)}>
      <h3 className="text-xs font-semibold text-white/50 uppercase tracking-wider px-1">
        Node Labels
      </h3>

      {categorizedTypes.map(({ category, label, types }) => (
        <div key={category} className="space-y-1">
          <div className="flex items-center gap-2 px-1 py-1">
            <span className="text-[10px] font-semibold text-white/40 uppercase tracking-widest">
              {label}
            </span>
            <div className="flex-1 h-px bg-white/[0.06]" />
          </div>

          <div className="space-y-0.5">
            {types.map(({ type, config, count }) => {
              const hasNodes = count > 0;
              const executing = executingType === type;

              return (
                <button
                  key={type}
                  onClick={(e) => handleClick(type, e)}
                  disabled={!hasNodes || isExecuting}
                  title={hasNodes ? `Click to query ${config.label}, Shift+click with relationships` : `No ${config.label} nodes`}
                  className={cn(
                    'w-full flex items-center justify-between px-2 py-1.5 rounded-md',
                    'text-sm transition-all duration-150 group',
                    hasNodes
                      ? 'text-white/60 hover:bg-white/[0.04] hover:text-white/80'
                      : 'text-white/20 cursor-not-allowed',
                    isExecuting && !executing && 'opacity-50'
                  )}
                >
                  <span className="flex items-center gap-2 min-w-0">
                    {executing ? (
                      <Loader2 className="w-4 h-4 animate-spin text-novanet-400" />
                    ) : (
                      <span className={cn('text-sm', !hasNodes && 'opacity-40')}>
                        {config.icon}
                      </span>
                    )}
                    <span className="truncate">{config.label}</span>
                  </span>

                  <span
                    className="px-1.5 py-0.5 rounded text-[10px] font-medium tabular-nums"
                    style={{
                      backgroundColor: hasNodes ? `${config.color}25` : 'transparent',
                      color: hasNodes ? config.color : 'inherit',
                    }}
                  >
                    {count}
                  </span>
                </button>
              );
            })}
          </div>
        </div>
      ))}

      {categorizedTypes.length === 0 && (
        <div className="flex flex-col items-center justify-center py-8 text-white/30">
          <span className="text-2xl mb-2">📭</span>
          <p className="text-xs">No nodes loaded</p>
        </div>
      )}

      {categorizedTypes.length > 0 && (
        <p className="text-[10px] text-white/30 text-center px-2">
          Click to query / Shift+click with relationships
        </p>
      )}
    </div>
  );
}
```

**Step 2: Commit**

```bash
git add src/components/sidebar/NodeLabels.tsx
git commit -m "feat(query): add NodeLabels query component"
```

---

## Task 4: Create QuerySidebar Component

**Files:**
- Create: `src/components/sidebar/QuerySidebar.tsx`

**Step 1: Write the component**

Replaces FilterSidebar - combines QueryLibrary + NodeLabels.

```typescript
'use client';

/**
 * QuerySidebar - Query launcher panel
 *
 * Replaces FilterSidebar with query-based approach:
 * - QueryLibrary: Pre-made workflow queries
 * - NodeLabels: Query by node type
 */

import { cn } from '@/lib/utils';
import { QueryLibrary } from './QueryLibrary';
import { NodeLabels } from './NodeLabels';

interface QuerySidebarProps {
  className?: string;
}

export function QuerySidebar({ className }: QuerySidebarProps) {
  return (
    <div className={cn('h-full flex flex-col', className)}>
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        <div className="space-y-6 p-4">
          {/* Query Library - Pre-made queries */}
          <QueryLibrary />

          {/* Divider */}
          <div className="border-t border-white/10 pt-4">
            {/* Node Labels - Query by type */}
            <NodeLabels />
          </div>
        </div>
      </div>
    </div>
  );
}
```

**Step 2: Commit**

```bash
git add src/components/sidebar/QuerySidebar.tsx
git commit -m "feat(query): add QuerySidebar combining QueryLibrary + NodeLabels"
```

---

## Task 5: Update SidebarTabs

**Files:**
- Modify: `src/components/sidebar/SidebarTabs.tsx`

**Step 1: Update imports and tabs**

Replace FilterSidebar with QuerySidebar, rename "Filters" to "Query".

```typescript
'use client';

/**
 * SidebarTabs - Tabbed sidebar with Query and Database panels
 */

import { useState, memo } from 'react';
import { Search, Database } from 'lucide-react';
import { cn } from '@/lib/utils';
import { QuerySidebar } from './QuerySidebar';
import { DatabaseInfoPanel } from './DatabaseInfoPanel';

type TabId = 'query' | 'database';

interface Tab {
  id: TabId;
  label: string;
  icon: React.ReactNode;
}

const TABS: Tab[] = [
  { id: 'database', label: 'Database', icon: <Database className="w-4 h-4" /> },
  { id: 'query', label: 'Query', icon: <Search className="w-4 h-4" /> },
];

export const SidebarTabs = memo(function SidebarTabs() {
  const [activeTab, setActiveTab] = useState<TabId>('database');

  return (
    <div className="h-full flex flex-col bg-black/40 backdrop-blur-xl">
      {/* Tab Navigation */}
      <div className="flex border-b border-white/[0.06]">
        {TABS.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={cn(
              'flex-1 flex items-center justify-center gap-2 px-4 py-3',
              'text-xs font-medium transition-all duration-200',
              'border-b-2 -mb-px',
              activeTab === tab.id
                ? 'text-white border-novanet-400 bg-white/[0.02]'
                : 'text-white/50 border-transparent hover:text-white/70 hover:bg-white/[0.02]'
            )}
          >
            <span className={cn('transition-colors', activeTab === tab.id ? 'text-novanet-400' : 'text-white/40')}>
              {tab.icon}
            </span>
            <span>{tab.label}</span>
          </button>
        ))}
      </div>

      {/* Tab Content */}
      <div className="flex-1 overflow-hidden">
        {activeTab === 'query' && <QuerySidebar className="h-full" />}
        {activeTab === 'database' && <DatabaseInfoPanel />}
      </div>
    </div>
  );
});
```

**Step 2: Commit**

```bash
git add src/components/sidebar/SidebarTabs.tsx
git commit -m "refactor(sidebar): replace Filters tab with Query tab"
```

---

## Task 6: Update sidebar index exports

**Files:**
- Modify: `src/components/sidebar/index.ts`

**Step 1: Update exports**

Add new components, keep backwards compatibility.

```typescript
export { SidebarTabs } from './SidebarTabs';
export { DatabaseInfoPanel } from './DatabaseInfoPanel';
export { QuerySidebar } from './QuerySidebar';
export { QueryLibrary } from './QueryLibrary';
export { NodeLabels } from './NodeLabels';
export { NodeDetailsPanel } from './NodeDetailsPanel';
export { EdgeDetailsPanel } from './EdgeDetailsPanel';

// Legacy exports (can be removed later)
export { FilterSidebar } from './FilterSidebar';
export { QuickViews } from './QuickViews';
export { LabelFilter } from './LabelFilter';
export { LocaleDropdown, useLocaleCycle } from './LocaleDropdown';
```

**Step 2: Commit**

```bash
git add src/components/sidebar/index.ts
git commit -m "refactor(sidebar): update exports for new Query components"
```

---

## Task 7: Clean up page.tsx

**Files:**
- Modify: `src/app/page.tsx`

**Step 1: Remove locale cycle from page.tsx**

The locale cycling keyboard shortcut (Shift+L) can be removed since LocaleDropdown is no longer in sidebar.

Find and remove:
- `import { useLocaleCycle } from '@/components/sidebar/LocaleDropdown'`
- `const { cycleLocale } = useLocaleCycle()`
- Keyboard handler for Shift+L
- `cycleLocale` from useEffect dependencies

**Step 2: Remove filter-related state from page.tsx**

Remove:
- `overviewFilterType` state
- `handleOverviewTypeFilter` callback
- Props passed to ResultsOverview (`selectedType`, `onTypeClick`)

Update ResultsOverview to just show stats without filter functionality.

**Step 3: Commit**

```bash
git add src/app/page.tsx
git commit -m "refactor(page): remove locale cycle and filter-related code"
```

---

## Task 8: Simplify ResultsOverview

**Files:**
- Modify: `src/components/query/ResultsOverview.tsx`

**Step 1: Remove filter props and click handlers**

The component should just show stats, not be interactive for filtering.

Remove:
- `selectedType` prop
- `onTypeClick` prop
- Click handlers on badges
- `ring` styles for selected state

Keep:
- Node type breakdown display
- Relationship type breakdown
- Collapsed/expanded states
- Floating bar design

**Step 2: Commit**

```bash
git add src/components/query/ResultsOverview.tsx
git commit -m "refactor(ResultsOverview): simplify to stats-only display"
```

---

## Task 9: Run tests and lint

**Step 1: Type check**

```bash
npm run type-check
```

Expected: No errors

**Step 2: Lint**

```bash
npm run lint
```

Expected: No errors

**Step 3: Fix any issues found**

---

## Task 10: Final verification

**Step 1: Start dev server**

```bash
npm run dev
```

**Step 2: Manual testing checklist**

- [ ] Sidebar shows "Database" and "Query" tabs
- [ ] Query tab shows QueryLibrary with categories
- [ ] Clicking a query executes and shows results in graph
- [ ] Node Labels section shows node types with counts
- [ ] Clicking node label queries that type
- [ ] Shift+click queries with relationships
- [ ] Loading states work correctly
- [ ] ResultsOverview shows breakdown without filter functionality
- [ ] No console errors

**Step 3: Commit final state**

```bash
git add -A
git commit -m "feat(sidebar): complete Query tab implementation

- Replace Filters tab with Query tab
- Add QueryLibrary with pre-made Cypher queries
- Add NodeLabels for querying by type
- Remove LocaleDropdown
- Simplify ResultsOverview to stats display

BREAKING CHANGE: FilterSidebar replaced by QuerySidebar"
```

---

## Summary

| Before | After |
|--------|-------|
| Client-side filtering | Real Cypher queries |
| QuickViews (filter) | QueryLibrary (execute) |
| LabelFilter (filter) | NodeLabels (query) |
| LocaleDropdown | Removed |
| Filters tab | Query tab |
