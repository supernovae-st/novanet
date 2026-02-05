---
name: react-best-practices
description: React and Next.js 15 performance patterns. Use when writing components, data fetching, or optimizing bundle size.
user-invocable: false
---

# React Best Practices for NovaNet Studio

> Adapted from Vercel agent-skills (57 rules) for Next.js 15 + React 19

## 1. Avoid Async Request Waterfalls

### Problem: Sequential Fetches

```typescript
// BAD - Sequential fetches (waterfall)
async function GraphPage({ params }: Props) {
  const project = await getProject(params.id);
  const nodes = await getNodes(project.id);  // waits for project
  const edges = await getEdges(project.id);  // waits for nodes
  return <Graph nodes={nodes} edges={edges} />;
}
```

### Solution: Parallel Fetches

```typescript
// GOOD - Parallel fetches
async function GraphPage({ params }: Props) {
  const [project, nodes, edges] = await Promise.all([
    getProject(params.id),
    getNodes(params.id),
    getEdges(params.id),
  ]);
  return <Graph nodes={nodes} edges={edges} />;
}
```

### Solution: Suspense Boundaries

```typescript
// BETTER - Suspense for streaming
export default function GraphPage({ params }: Props) {
  return (
    <Suspense fallback={<GraphSkeleton />}>
      <GraphLoader id={params.id} />
    </Suspense>
  );
}
```

---

## 2. Optimize Bundle Size

### Use Dynamic Imports

```typescript
// BAD - Bloats initial bundle
import { ForceGraph3D } from 'react-force-graph-3d';

// GOOD - Load on demand
const ForceGraph3D = dynamic(
  () => import('react-force-graph-3d').then(mod => mod.ForceGraph3D),
  { ssr: false, loading: () => <GraphSkeleton /> }
);
```

### Tree Shake Icons

```typescript
// BAD - Imports entire library
import * as Icons from 'lucide-react';

// GOOD - Named imports
import { Copy, Check, ChevronDown } from 'lucide-react';
```

### Analyze Bundle

```bash
# Add to package.json scripts
"analyze": "ANALYZE=true next build"
```

---

## 3. Server Components (Default)

### Keep Server Components Pure

```typescript
// page.tsx - Server Component (default in Next.js 15)
export default async function GraphPage({ params }: Props) {
  const nodes = await fetchNodes(params.projectId);

  return (
    <main>
      <h1>Project Graph</h1>
      <GraphClient initialNodes={nodes} />
    </main>
  );
}
```

### Push Client Boundary Down

```typescript
// BAD - Entire page is client
'use client';
export default function GraphPage() { /* ... */ }

// GOOD - Only interactive part is client
// page.tsx (server)
export default async function GraphPage() {
  const data = await fetchData();
  return <InteractiveGraph data={data} />;  // client boundary
}

// InteractiveGraph.tsx
'use client';
export function InteractiveGraph({ data }: Props) { /* ... */ }
```

---

## 4. Client Components Best Practices

### Minimize 'use client' Scope

```typescript
// components/sidebar/NodeDetails.tsx
'use client';

import { useGraphStore } from '@/stores/graphStore';
import { NodeDetailsPanel } from './NodeDetailsPanel';  // can be server

export function NodeDetails() {
  const selectedNode = useGraphStore((s) => s.selectedNode);
  if (!selectedNode) return null;
  return <NodeDetailsPanel node={selectedNode} />;
}
```

### Prefer Composition

```typescript
// BAD - Static content inside client component
'use client';
export function Sidebar({ children }: Props) {
  const [open, setOpen] = useState(true);
  return (
    <aside>
      <h2>Node Details</h2>  {/* Static - should be server */}
      {children}
    </aside>
  );
}

// GOOD - Server content passed as children
// SidebarWrapper.tsx (client)
'use client';
export function SidebarWrapper({ children }: Props) {
  const [open, setOpen] = useState(true);
  return <aside className={open ? 'w-80' : 'w-0'}>{children}</aside>;
}

// page.tsx (server)
<SidebarWrapper>
  <h2>Node Details</h2>
  <NodeDetails />
</SidebarWrapper>
```

---

## 5. Prevent Unnecessary Re-renders

### Selector Pattern (Zustand)

```typescript
// BAD - Re-renders on ANY store change
const store = useGraphStore();
return <div>{store.selectedNode?.id}</div>;

// GOOD - Only re-renders when selectedNode changes
const selectedNode = useGraphStore((s) => s.selectedNode);
return <div>{selectedNode?.id}</div>;

// BETTER - Granular selection
const nodeId = useGraphStore((s) => s.selectedNode?.id);
return <div>{nodeId}</div>;
```

### Memoize Expensive Components

```typescript
import { memo } from 'react';

// Memoize custom nodes for React Flow
export const EntityNode = memo(function EntityNode({ data }: NodeProps) {
  return (
    <div className="entity-node">
      <Handle type="target" position={Position.Top} />
      <span>{data.label}</span>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
});
```

### Stable Callbacks

```typescript
// BAD - New function every render
<ReactFlow
  onNodeClick={(e, node) => handleNodeClick(node)}
/>

// GOOD - Stable reference
const onNodeClick = useCallback((e: MouseEvent, node: Node) => {
  setSelectedNode(node.id);
}, []);

<ReactFlow onNodeClick={onNodeClick} />
```

---

## 6. Efficient Rendering

### Virtualize Large Lists

```typescript
// For 19k nodes - use virtualization
import { useVirtualizer } from '@tanstack/react-virtual';

export function NodeList({ nodes }: { nodes: NodeType[] }) {
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: nodes.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 48,
    overscan: 5,
  });

  return (
    <div ref={parentRef} className="h-full overflow-auto">
      <div style={{ height: virtualizer.getTotalSize() }}>
        {virtualizer.getVirtualItems().map((item) => (
          <NodeRow key={nodes[item.index].id} node={nodes[item.index]} />
        ))}
      </div>
    </div>
  );
}
```

### Debounce Search/Filter

```typescript
import { useDebouncedCallback } from 'use-debounce';

export function NodeSearch() {
  const setFilter = useFilterStore((s) => s.setSearchQuery);

  const debouncedFilter = useDebouncedCallback((value: string) => {
    setFilter(value);
  }, 300);

  return <input onChange={(e) => debouncedFilter(e.target.value)} />;
}
```

---

## 7. JavaScript Optimizations

### Avoid Inline Objects/Arrays

```typescript
// BAD - New object every render
<Button style={{ marginTop: 10 }} />

// GOOD - Static styles
const buttonStyle = { marginTop: 10 };
<Button style={buttonStyle} />

// BEST - Tailwind classes
<Button className="mt-2.5" />
```

### Use CSS for Animations

```typescript
// BAD - JavaScript animation (blocks main thread)
useEffect(() => {
  const interval = setInterval(() => {
    setOpacity((o) => o + 0.1);
  }, 16);
  return () => clearInterval(interval);
}, []);

// GOOD - CSS animation (GPU accelerated)
<div className="animate-fade-in" />

// tailwind.config.ts
animation: {
  'fade-in': 'fadeIn 0.3s ease-in-out',
}
```

---

## 8. Advanced Patterns

### Optimistic Updates

```typescript
// For graph mutations
async function updateNode(nodeId: string, data: Partial<NodeData>) {
  // Optimistic update
  const previousNodes = useGraphStore.getState().nodes;
  useGraphStore.setState((s) => ({
    nodes: s.nodes.map((n) =>
      n.id === nodeId ? { ...n, data: { ...n.data, ...data } } : n
    ),
  }));

  try {
    await api.updateNode(nodeId, data);
  } catch (error) {
    // Rollback on error
    useGraphStore.setState({ nodes: previousNodes });
    throw error;
  }
}
```

### Suspense with Error Boundaries

```typescript
import { ErrorBoundary } from 'react-error-boundary';

export function GraphSection() {
  return (
    <ErrorBoundary fallback={<GraphError />}>
      <Suspense fallback={<GraphSkeleton />}>
        <Graph />
      </Suspense>
    </ErrorBoundary>
  );
}
```

### Server Actions for Mutations

```typescript
// actions/graph.ts
'use server';

export async function createNode(formData: FormData) {
  const type = formData.get('type') as string;
  const label = formData.get('label') as string;

  const node = await db.node.create({
    data: { type, label },
  });

  revalidatePath('/graph');
  return node;
}

// Client usage
<form action={createNode}>
  <input name="type" />
  <input name="label" />
  <button type="submit">Create Node</button>
</form>
```

---

## NovaNet-Specific Patterns

### Graph Store Selectors

```typescript
// stores/graphStore.ts
export const useGraphStore = create<GraphState>((set, get) => ({
  nodes: [],
  edges: [],
  selectedNodeId: null,

  // Derived state as getter
  getSelectedNode: () => {
    const { nodes, selectedNodeId } = get();
    return nodes.find((n) => n.id === selectedNodeId);
  },
}));

// Usage with selector
const selectedNode = useGraphStore((s) => s.getSelectedNode());
```

### Filter Performance

```typescript
// Pre-compute filtered nodes
const filteredNodes = useMemo(() => {
  return nodes.filter((node) => {
    if (activeTypes.length && !activeTypes.includes(node.type)) return false;
    if (activeLocale && node.locale !== activeLocale) return false;
    if (searchQuery && !node.label.toLowerCase().includes(searchQuery)) return false;
    return true;
  });
}, [nodes, activeTypes, activeLocale, searchQuery]);
```

---

## Quick Reference

| Pattern | When | Example |
|---------|------|---------|
| `Promise.all()` | Multiple independent fetches | Graph data loading |
| `dynamic()` | Heavy components | ForceGraph3D |
| `memo()` | Expensive pure components | Custom React Flow nodes |
| `useCallback()` | Event handlers | onNodeClick |
| `useMemo()` | Derived computations | filteredNodes |
| Selector pattern | Zustand subscriptions | `(s) => s.selectedNode` |
| Virtualization | Lists > 100 items | NodeList, EdgeList |
| Debounce | User input | Search, filter |
| Suspense | Async boundaries | Data loading |

---

## Checklist Before PR

- [ ] No request waterfalls (use `Promise.all`)
- [ ] Heavy imports are dynamic
- [ ] Client boundary pushed down
- [ ] Zustand selectors are granular
- [ ] Callbacks are memoized
- [ ] Large lists are virtualized
- [ ] No inline objects in JSX
- [ ] Animations use CSS/Tailwind
