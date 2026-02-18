# YAML Views Integration - Studio Single Source of Truth

**Date**: 2026-01-30
**Status**: Approved
**Author**: Claude + Thibaut

## Summary

Connect Studio UI to YAML view definitions in `@novanet/core`, establishing YAML as the single source of truth. Remove hardcoded `VIEW_PRESETS` and refactor to dynamic loading via API routes.

## Goals

1. **Single Source of Truth**: YAML views in `packages/core/models/views/` are THE reference
2. **Dynamic Loading**: Studio loads views via API routes, not hardcoded TypeScript
3. **URL Sync**: Bookmarkable/shareable URLs with view + params
4. **Clean UI**: Grid cards grouped by category with keyboard shortcuts

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│  @novanet/core (packages/core/)                                     │
│  models/views/                                                      │
│  ├── _registry.yaml    ← Master list (14 views + categories)        │
│  ├── complete-graph.yaml                                            │
│  └── ...                                                            │
│                                                                     │
│  src/filters/                                                       │
│  ├── ViewLoader.ts     ← YAML → NovaNetFilter                       │
│  └── CypherGenerator.ts ← Filter → Cypher                           │
└─────────────────────────────────────────────────────────────────────┘
                        │
                        ▼ import { ViewLoader } from '@novanet/core'
┌─────────────────────────────────────────────────────────────────────┐
│  @novanet/studio (apps/studio/)                                     │
│                                                                     │
│  API Routes (Server-side)                                           │
│  ├── /api/views          → GET registry + all views                 │
│  └── /api/views/[id]     → GET single view + generated Cypher       │
│                                                                     │
│  UI Components                                                      │
│  ├── ViewSelector.tsx    → Grid cards by category                   │
│  └── ViewParamsModal.tsx → Key/Locale/Project input                 │
│                                                                     │
│  State                                                              │
│  ├── viewStore.ts        → Selected view + params                   │
│  └── useUrlSync.ts       → ?view=X&locale=Y&project=Z               │
└─────────────────────────────────────────────────────────────────────┘
```

## Data Flow

```
1. User clicks ViewCard("block-generation")
       ↓
2. ViewParamsModal opens → user enters key="hero-pricing", locale="fr-FR"
       ↓
3. viewStore.selectView("block-generation", { key, locale })
       ↓
4. useUrlSync updates URL → ?view=block-generation&key=hero-pricing&locale=fr-FR
       ↓
5. fetch("/api/views/block-generation?key=hero-pricing&locale=fr-FR")
       ↓
6. API: ViewLoader.loadView() → ViewLoader.toFilter() → CypherGenerator.generate()
       ↓
7. Response: { view, cypher: { query, params } }
       ↓
8. graphStore.executeQuery(cypher) → Neo4j
       ↓
9. React Flow renders graph
```

## Implementation Phases

### Phase 1: Core Fixes (BLOCKER)

Must complete before any Studio work.

#### 1.1 Add Missing NovaNetFilter Methods

**File**: `packages/core/src/filters/NovaNetFilter.ts`

Add 4 methods that ViewLoader.ts references but don't exist:

```typescript
includeBlockType(): this {
  this.state.includes.push({
    relation: 'OF_TYPE',
    direction: 'outgoing',
  });
  return this;
}

includeSemanticLinks(opts?: { depth?: number }): this {
  this.state.includes.push({
    relation: 'SEMANTIC_LINK',
    direction: 'outgoing',
    depth: opts?.depth ?? 1,
  });
  return this;
}

includeBrandIdentity(): this {
  this.state.includes.push({
    relation: 'HAS_BRAND_IDENTITY',
    direction: 'outgoing',
  });
  return this;
}

includeProjectConcepts(opts?: { depth?: number }): this {
  this.state.includes.push({
    relation: 'HAS_CONCEPT',
    direction: 'outgoing',
    depth: opts?.depth ?? 1,
  });
  return this;
}
```

#### 1.2 Add Category to Registry

**File**: `packages/core/models/views/_registry.yaml`

Add `category` field to each view:

```yaml
views:
  - id: complete-graph
    file: complete-graph.yaml
    description: Full NovaNet graph
    category: scope  # NEW

  - id: global-layer
    file: global-layer.yaml
    description: Locale nodes
    category: scope

  - id: block-generation
    file: block-generation.yaml
    description: Sub-agent context
    category: generation
  # ... etc
```

**Categories**:
- `scope` - Layer views (complete, global, project, shared)
- `generation` - Orchestrator/agent context views
- `knowledge` - Locale and concept views
- `project` - Project structure views
- `mining` - SEO/GEO pipeline views

#### 1.3 Add ViewCategory Type

**File**: `packages/core/src/filters/types.ts`

```typescript
export type ViewCategory = 'scope' | 'generation' | 'knowledge' | 'project' | 'mining';

export interface ViewRegistryEntry {
  id: string;
  file: string;
  description: string;
  category: ViewCategory;
}

export interface ViewRegistry {
  views: ViewRegistryEntry[];
}
```

#### 1.4 Verify

```bash
pnpm build --filter=@novanet/core
```

### Phase 2: API Routes

#### 2.1 Registry Endpoint

**File**: `apps/studio/src/app/api/views/route.ts` (CREATE)

```typescript
import { NextResponse } from 'next/server';
import { ViewLoader } from '@novanet/core';
import path from 'path';

const viewsDir = path.resolve(process.cwd(), '../../packages/core/models/views');

let registryCache: ViewRegistry | null = null;

export async function GET() {
  try {
    if (!registryCache) {
      registryCache = await ViewLoader.loadRegistry(viewsDir);
    }

    // Group by category
    const categories = ['scope', 'generation', 'knowledge', 'project', 'mining'];
    const grouped = categories.map(cat => ({
      id: cat,
      name: cat.charAt(0).toUpperCase() + cat.slice(1),
      views: registryCache!.views.filter(v => v.category === cat),
    }));

    return NextResponse.json({
      success: true,
      data: { registry: registryCache, categories: grouped }
    });
  } catch (error) {
    return NextResponse.json(
      { success: false, error: String(error) },
      { status: 500 }
    );
  }
}
```

#### 2.2 Single View Endpoint

**File**: `apps/studio/src/app/api/views/[id]/route.ts` (CREATE)

```typescript
import { NextRequest, NextResponse } from 'next/server';
import { ViewLoader, CypherGenerator } from '@novanet/core';
import path from 'path';

const viewsDir = path.resolve(process.cwd(), '../../packages/core/models/views');
const viewIdRegex = /^[a-z0-9-]+$/;

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  const { id } = await params;

  // Validate view ID
  if (!viewIdRegex.test(id)) {
    return NextResponse.json(
      { success: false, error: 'Invalid view ID' },
      { status: 400 }
    );
  }

  try {
    const searchParams = request.nextUrl.searchParams;
    const viewParams = {
      key: searchParams.get('key') || undefined,
      locale: searchParams.get('locale') || undefined,
      project: searchParams.get('project') || undefined,
    };

    // Load view
    const view = await ViewLoader.loadView(id, viewsDir);

    // Convert to filter and generate Cypher
    const filter = ViewLoader.toFilter(view, viewParams);
    const cypher = CypherGenerator.generate(filter);

    return NextResponse.json({
      success: true,
      data: { view, cypher, params: viewParams }
    });
  } catch (error: any) {
    if (error.code === 'ENOENT') {
      return NextResponse.json(
        { success: false, error: `View '${id}' not found` },
        { status: 404 }
      );
    }
    return NextResponse.json(
      { success: false, error: String(error) },
      { status: 500 }
    );
  }
}
```

#### 2.3 Verify

```bash
curl http://localhost:3000/api/views
curl "http://localhost:3000/api/views/block-generation?key=hero-pricing&locale=fr-FR"
```

### Phase 3: State Layer

#### 3.1 View Store

**File**: `apps/studio/src/stores/viewStore.ts` (CREATE)

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

interface ViewParams {
  key?: string;
  locale?: string;
  project?: string;
}

interface ViewCategory {
  id: string;
  name: string;
  views: ViewRegistryEntry[];
}

interface ViewState {
  // Data
  categories: ViewCategory[];
  activeViewId: string | null;
  params: ViewParams;
  loading: boolean;
  error: string | null;

  // Actions
  loadRegistry: () => Promise<void>;
  selectView: (id: string, params?: ViewParams) => void;
  setParams: (params: Partial<ViewParams>) => void;
  clearView: () => void;

  // URL sync
  syncFromURL: (searchParams: URLSearchParams) => void;
  toURLParams: () => URLSearchParams;
}

export const useViewStore = create<ViewState>()(
  persist(
    immer((set, get) => ({
      categories: [],
      activeViewId: null,
      params: {},
      loading: false,
      error: null,

      loadRegistry: async () => {
        set({ loading: true, error: null });
        try {
          const res = await fetch('/api/views');
          const json = await res.json();
          if (json.success) {
            set({ categories: json.data.categories, loading: false });
          } else {
            set({ error: json.error, loading: false });
          }
        } catch (e) {
          set({ error: String(e), loading: false });
        }
      },

      selectView: (id, params) => {
        set((state) => {
          state.activeViewId = id;
          if (params) state.params = params;
        });
      },

      setParams: (params) => {
        set((state) => {
          state.params = { ...state.params, ...params };
        });
      },

      clearView: () => {
        set({ activeViewId: null, params: {} });
      },

      syncFromURL: (searchParams) => {
        const view = searchParams.get('view');
        if (view) {
          set({
            activeViewId: view,
            params: {
              key: searchParams.get('key') || undefined,
              locale: searchParams.get('locale') || undefined,
              project: searchParams.get('project') || undefined,
            }
          });
        }
      },

      toURLParams: () => {
        const { activeViewId, params } = get();
        const urlParams = new URLSearchParams();
        if (activeViewId) {
          urlParams.set('view', activeViewId);
          if (params.key) urlParams.set('key', params.key);
          if (params.locale) urlParams.set('locale', params.locale);
          if (params.project) urlParams.set('project', params.project);
        }
        return urlParams;
      },
    })),
    {
      name: 'novanet-view-store',
      partialize: (state) => ({
        activeViewId: state.activeViewId,
        params: state.params,
      }),
    }
  )
);
```

#### 3.2 URL Sync Hook

**File**: `apps/studio/src/hooks/useUrlSync.ts` (CREATE)

```typescript
'use client';

import { useEffect, useRef } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useViewStore } from '@/stores/viewStore';

export function useUrlSync() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const { activeViewId, params, syncFromURL, toURLParams } = useViewStore();
  const initialized = useRef(false);

  // Hydrate from URL on mount
  useEffect(() => {
    if (!initialized.current) {
      syncFromURL(searchParams);
      initialized.current = true;
    }
  }, []);

  // Sync store → URL (debounced)
  useEffect(() => {
    if (!initialized.current) return;

    const timeout = setTimeout(() => {
      const newParams = toURLParams();
      const currentParams = new URLSearchParams(searchParams);

      if (newParams.toString() !== currentParams.toString()) {
        router.replace(`?${newParams.toString()}`, { scroll: false });
      }
    }, 300);

    return () => clearTimeout(timeout);
  }, [activeViewId, params]);
}
```

#### 3.3 Simplify filterStore

**File**: `apps/studio/src/stores/filterStore.ts` (MODIFY)

Remove all `VIEW_PRESETS` logic. Keep only:
- `enabledNodeTypes` for client-side filtering
- `selectedProject`, `selectedLocale` (now driven by viewStore.params)

### Phase 4: UI Components

#### 4.1 ViewCard

**File**: `apps/studio/src/components/views/ViewCard.tsx` (CREATE)

```typescript
'use client';

import { cn } from '@/lib/utils';
import { Layers, Zap, BookOpen, FolderOpen, Search } from 'lucide-react';

const categoryIcons = {
  scope: Layers,
  generation: Zap,
  knowledge: BookOpen,
  project: FolderOpen,
  mining: Search,
};

interface ViewCardProps {
  view: { id: string; description: string; category: string };
  shortcut?: string;
  isActive: boolean;
  onClick: () => void;
}

export function ViewCard({ view, shortcut, isActive, onClick }: ViewCardProps) {
  const Icon = categoryIcons[view.category as keyof typeof categoryIcons] || Layers;

  return (
    <button
      onClick={onClick}
      className={cn(
        'flex flex-col items-center justify-center p-3 rounded-lg border',
        'hover:bg-accent transition-colors',
        isActive && 'bg-accent border-primary'
      )}
    >
      <Icon className="h-5 w-5 mb-1" />
      <span className="text-xs font-medium truncate w-full text-center">
        {view.id.split('-').slice(0, 2).join(' ')}
      </span>
      {shortcut && (
        <kbd className="mt-1 text-[10px] px-1 bg-muted rounded">{shortcut}</kbd>
      )}
    </button>
  );
}
```

#### 4.2 ViewCategory

**File**: `apps/studio/src/components/views/ViewCategory.tsx` (CREATE)

```typescript
'use client';

import { useState } from 'react';
import { ChevronDown, ChevronRight } from 'lucide-react';
import { cn } from '@/lib/utils';

interface ViewCategoryProps {
  name: string;
  count: number;
  defaultOpen?: boolean;
  children: React.ReactNode;
}

export function ViewCategory({ name, count, defaultOpen = true, children }: ViewCategoryProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  return (
    <div className="space-y-2">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 w-full text-sm font-medium text-muted-foreground hover:text-foreground"
      >
        {isOpen ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
        <span className="uppercase tracking-wider">{name}</span>
        <span className="ml-auto text-xs">[{count}]</span>
      </button>
      {isOpen && (
        <div className="grid grid-cols-3 gap-2 pl-6">
          {children}
        </div>
      )}
    </div>
  );
}
```

#### 4.3 ViewSelector

**File**: `apps/studio/src/components/views/ViewSelector.tsx` (CREATE)

```typescript
'use client';

import { useEffect } from 'react';
import { useViewStore } from '@/stores/viewStore';
import { ViewCard } from './ViewCard';
import { ViewCategory } from './ViewCategory';
import { ViewParamsModal } from './ViewParamsModal';

export function ViewSelector() {
  const { categories, activeViewId, loading, loadRegistry, selectView } = useViewStore();
  const [pendingView, setPendingView] = useState<string | null>(null);

  useEffect(() => {
    loadRegistry();
  }, []);

  // Keyboard shortcuts
  useEffect(() => {
    const allViews = categories.flatMap(c => c.views);

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement) return;
      const num = parseInt(e.key);
      if (num >= 1 && num <= 9 && allViews[num - 1]) {
        setPendingView(allViews[num - 1].id);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [categories]);

  if (loading) return <div className="p-4 text-sm text-muted-foreground">Loading views...</div>;

  let shortcutIndex = 1;

  return (
    <div className="space-y-4 p-4">
      {categories.map((category) => (
        <ViewCategory
          key={category.id}
          name={category.name}
          count={category.views.length}
          defaultOpen={['scope', 'generation'].includes(category.id)}
        >
          {category.views.map((view) => (
            <ViewCard
              key={view.id}
              view={view}
              shortcut={shortcutIndex <= 9 ? String(shortcutIndex++) : undefined}
              isActive={activeViewId === view.id}
              onClick={() => setPendingView(view.id)}
            />
          ))}
        </ViewCategory>
      ))}

      {pendingView && (
        <ViewParamsModal
          viewId={pendingView}
          onClose={() => setPendingView(null)}
          onSubmit={(params) => {
            selectView(pendingView, params);
            setPendingView(null);
          }}
        />
      )}
    </div>
  );
}
```

#### 4.4 ViewParamsModal

**File**: `apps/studio/src/components/views/ViewParamsModal.tsx` (CREATE)

```typescript
'use client';

import { useState } from 'react';
import * as Dialog from '@radix-ui/react-dialog';
import { useViewStore } from '@/stores/viewStore';

interface ViewParamsModalProps {
  viewId: string;
  onClose: () => void;
  onSubmit: (params: { key?: string; locale?: string; project?: string }) => void;
}

export function ViewParamsModal({ viewId, onClose, onSubmit }: ViewParamsModalProps) {
  const { params: currentParams } = useViewStore();
  const [key, setKey] = useState(currentParams.key || '');
  const [locale, setLocale] = useState(currentParams.locale || 'fr-FR');
  const [project, setProject] = useState(currentParams.project || 'qrcode-ai');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({
      key: key || undefined,
      locale: locale || undefined,
      project: project || undefined,
    });
  };

  return (
    <Dialog.Root open onOpenChange={onClose}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50" />
        <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-background p-6 rounded-lg shadow-xl w-96">
          <Dialog.Title className="text-lg font-semibold mb-4">
            Configure View: {viewId}
          </Dialog.Title>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Key (optional)</label>
              <input
                type="text"
                value={key}
                onChange={(e) => setKey(e.target.value)}
                placeholder="e.g., hero-pricing"
                className="w-full px-3 py-2 border rounded-md"
              />
            </div>

            <div>
              <label className="block text-sm font-medium mb-1">Locale</label>
              <select
                value={locale}
                onChange={(e) => setLocale(e.target.value)}
                className="w-full px-3 py-2 border rounded-md"
              >
                <option value="fr-FR">French (fr-FR)</option>
                <option value="en-US">English (en-US)</option>
                <option value="es-MX">Spanish (es-MX)</option>
                <option value="ja-JP">Japanese (ja-JP)</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium mb-1">Project</label>
              <input
                type="text"
                value={project}
                onChange={(e) => setProject(e.target.value)}
                placeholder="e.g., qrcode-ai"
                className="w-full px-3 py-2 border rounded-md"
              />
            </div>

            <div className="flex gap-2 justify-end">
              <button type="button" onClick={onClose} className="px-4 py-2 border rounded-md">
                Cancel
              </button>
              <button type="submit" className="px-4 py-2 bg-primary text-primary-foreground rounded-md">
                Apply View
              </button>
            </div>
          </form>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

### Phase 5: Integration

#### 5.1 Update SidebarTabs

**File**: `apps/studio/src/components/sidebar/SidebarTabs.tsx` (MODIFY)

Replace `ViewPresetSelector` import with `ViewSelector`:

```typescript
import { ViewSelector } from '@/components/views/ViewSelector';

// In JSX, replace:
// <ViewPresetSelector />
// with:
<ViewSelector />
```

#### 5.2 Delete Old Files

```bash
rm apps/studio/src/components/sidebar/ViewPresetSelector.tsx
```

Remove `VIEW_PRESETS` from `apps/studio/src/lib/filterAdapter.ts`.

#### 5.3 Connect to Graph

**File**: `apps/studio/src/hooks/useGraphData.ts` (MODIFY)

Add effect to execute view when activeViewId changes:

```typescript
import { useViewStore } from '@/stores/viewStore';

// Inside hook:
const { activeViewId, params } = useViewStore();

useEffect(() => {
  if (!activeViewId) return;

  const fetchView = async () => {
    const query = new URLSearchParams();
    if (params.key) query.set('key', params.key);
    if (params.locale) query.set('locale', params.locale);
    if (params.project) query.set('project', params.project);

    const res = await fetch(`/api/views/${activeViewId}?${query}`);
    const json = await res.json();

    if (json.success) {
      executeQuery(json.data.cypher.query, json.data.cypher.params);
    }
  };

  fetchView();
}, [activeViewId, params]);
```

### Phase 6: Cleanup & Test

#### 6.1 Test Checklist

- [ ] `pnpm build` passes
- [ ] All 14 YAML views load via `/api/views`
- [ ] Each view generates valid Cypher
- [ ] Keyboard shortcuts 1-9 work
- [ ] URL sync: navigate to `?view=block-generation&key=hero&locale=fr-FR`
- [ ] Graph renders correctly for each view

#### 6.2 Files Summary

| Action | File | Description |
|--------|------|-------------|
| MODIFY | `packages/core/src/filters/NovaNetFilter.ts` | +4 methods |
| MODIFY | `packages/core/models/views/_registry.yaml` | +category field |
| MODIFY | `packages/core/src/filters/types.ts` | +ViewCategory type |
| CREATE | `apps/studio/src/app/api/views/route.ts` | Registry endpoint |
| CREATE | `apps/studio/src/app/api/views/[id]/route.ts` | Single view endpoint |
| CREATE | `apps/studio/src/stores/viewStore.ts` | Zustand store |
| CREATE | `apps/studio/src/hooks/useUrlSync.ts` | URL sync hook |
| MODIFY | `apps/studio/src/stores/filterStore.ts` | Remove presets |
| CREATE | `apps/studio/src/components/views/ViewCard.tsx` | Card component |
| CREATE | `apps/studio/src/components/views/ViewCategory.tsx` | Category component |
| CREATE | `apps/studio/src/components/views/ViewSelector.tsx` | Main selector |
| CREATE | `apps/studio/src/components/views/ViewParamsModal.tsx` | Params dialog |
| MODIFY | `apps/studio/src/components/sidebar/SidebarTabs.tsx` | Use ViewSelector |
| MODIFY | `apps/studio/src/hooks/useGraphData.ts` | Connect to viewStore |
| DELETE | `apps/studio/src/components/sidebar/ViewPresetSelector.tsx` | Old component |
| MODIFY | `apps/studio/src/lib/filterAdapter.ts` | Remove VIEW_PRESETS |

**Total**: 12 CREATE/MODIFY, 2 DELETE

## Success Criteria

1. YAML views in `@novanet/core` are the single source of truth
2. No hardcoded view definitions in Studio
3. All 14 views accessible via grid UI with categories
4. URL sharing works: `?view=X&key=Y&locale=Z`
5. Keyboard shortcuts (1-9) for quick access
