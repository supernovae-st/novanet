# Context7 Documentation Reference

Quick reference for the main libraries used in NovaNet Visualizer.
Full docs available via Context7 MCP tool.

---

## @xyflow/react (React Flow)

**Library ID:** `/xyflow/react-flow`

### Key Imports

```typescript
import {
  ReactFlow,
  useNodesState,
  useEdgesState,
  useReactFlow,
  Handle,
  Position,
  type Node,
  type Edge,
  type NodeProps,
  type EdgeProps,
} from '@xyflow/react';
```

### Custom Node Pattern

```typescript
function CustomNode({ data, selected }: NodeProps) {
  return (
    <div className={cn('node', selected && 'selected')}>
      <Handle type="target" position={Position.Top} />
      {data.label}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

const nodeTypes = { custom: CustomNode };
```

### Fit View

```typescript
const { fitView } = useReactFlow();
fitView({ padding: 0.2, duration: 200 });
```

---

## react-force-graph-3d

**Library ID:** `/vasturiano/react-force-graph`

### Basic Usage

```typescript
import ForceGraph3D from 'react-force-graph-3d';

<ForceGraph3D
  graphData={{ nodes, links }}
  nodeLabel="name"
  nodeColor={node => getColor(node.type)}
  linkDirectionalArrowLength={3.5}
  onNodeClick={handleNodeClick}
/>
```

### Camera Control

```typescript
const fgRef = useRef();

// Focus on node
fgRef.current.cameraPosition(
  { x: node.x * 1.5, y: node.y * 1.5, z: node.z * 1.5 },
  node,
  2000
);

// Fit all
fgRef.current.zoomToFit(400);
```

---

## Zustand 5

**Library ID:** `/pmndrs/zustand`

### Store with Middleware

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

const useStore = create(
  persist(
    immer((set, get) => ({
      count: 0,
      increment: () => set((state) => { state.count += 1; }),
    })),
    { name: 'store-key' }
  )
);
```

### Selector Pattern

```typescript
// Fine-grained subscription
const count = useStore((state) => state.count);

// Multiple values
const { count, increment } = useStore((state) => ({
  count: state.count,
  increment: state.increment,
}));
```

---

## neo4j-driver

**Library ID:** `/neo4j/docs`

### Connection

```typescript
import neo4j from 'neo4j-driver';

const driver = neo4j.driver(
  'bolt://localhost:7687',
  neo4j.auth.basic('neo4j', 'password')
);
```

### Read Query

```typescript
const session = driver.session();
try {
  const result = await session.executeRead(async (tx) => {
    const res = await tx.run('MATCH (n) RETURN n LIMIT $limit', { limit: 10 });
    return res.records.map(r => r.get('n').properties);
  });
} finally {
  await session.close();
}
```

### Write Query

```typescript
await session.executeWrite(async (tx) => {
  await tx.run(
    'CREATE (n:Node {name: $name}) RETURN n',
    { name: 'Test' }
  );
});
```

---

## Quick Context7 Queries

To get more details, use the Context7 MCP tool:

```
resolve-library-id: "@xyflow/react"
query-docs: { libraryId: "/xyflow/react-flow", query: "custom nodes typescript" }
```
