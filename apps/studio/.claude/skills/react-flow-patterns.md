---
name: react-flow-patterns
description: 2D graph visualization with @xyflow/react. Use when creating custom nodes, handling graph events, or implementing node type registration for the 2D view mode.
user-invocable: false
---

# React Flow Patterns

> NovaNet 2D visualization with @xyflow/react

## Use When

- Creating custom node components
- Registering node types for 33 NovaNet types
- Handling click/drag/connect events
- Implementing hooks (useNodesState, useReactFlow)
- Optimizing for 19k+ nodes

## Quick Reference

| Pattern | Usage |
|---------|-------|
| State hooks | `useNodesState`, `useEdgesState` |
| Instance | `useReactFlow()` for fitView, setCenter |
| Custom node | `memo()` + `forwardRef` for performance |
| Selection | `selected` prop on NodeProps |

---

## Custom Node Components

```typescript
import { Handle, Position, type NodeProps } from '@xyflow/react';

type NodeData = {
  label: string;
  type: string;
  properties: Record<string, unknown>;
};

export function CustomNode({ data, selected }: NodeProps<NodeData>) {
  return (
    <div className={cn(
      'px-4 py-2 rounded-lg border-2',
      selected ? 'border-novanet-500' : 'border-white/20'
    )}>
      <Handle type="target" position={Position.Top} />
      <div className="text-sm font-medium">{data.label}</div>
      <div className="text-xs text-white/60">{data.type}</div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}
```

## Hooks Usage

```typescript
import {
  useNodesState,
  useEdgesState,
  useReactFlow,
  type Node,
  type Edge,
} from '@xyflow/react';

// State management
const [nodes, setNodes, onNodesChange] = useNodesState<Node>(initialNodes);
const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>(initialEdges);

// Instance methods
const { fitView, setCenter, getZoom, setViewport } = useReactFlow();
```

## Node Types Registration

```typescript
const nodeTypes = {
  project: ProjectNode,
  locale: LocaleNode,
  source: SourceNode,
  translationUnit: TranslationUnitNode,
  // ... 17 types total
};

<ReactFlow
  nodes={nodes}
  edges={edges}
  nodeTypes={nodeTypes}
  onNodesChange={onNodesChange}
  onEdgesChange={onEdgesChange}
  fitView
/>
```

## Performance Tips

1. **Memoize node components** - Use `memo()` for all custom nodes
2. **Virtualization** - React Flow handles this automatically
3. **Batch updates** - Use `setNodes` with callback for bulk changes
4. **Limit visible nodes** - Filter before rendering for 19k nodes

## Event Handlers

```typescript
const onNodeClick = useCallback((event: React.MouseEvent, node: Node) => {
  setSelectedNode(node.id);
}, []);

const onPaneClick = useCallback(() => {
  setSelectedNode(null);
}, []);

const onConnect = useCallback((connection: Connection) => {
  setEdges((eds) => addEdge(connection, eds));
}, []);
```
