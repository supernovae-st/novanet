---
name: force-graph-patterns
description: 3D graph visualization with react-force-graph-3d. Use when implementing 3D view, camera controls, node styling, or performance optimization for large graphs (19k+ nodes).
user-invocable: false
---

# Force Graph 3D Patterns

> NovaNet 3D visualization with react-force-graph-3d

## Use When

- Implementing 3D graph view mode
- Adding camera controls (focus, zoom, fit)
- Styling nodes by type with colors
- Optimizing performance for large graphs
- Handling click/hover events in 3D space

## Quick Reference

| Pattern | Method |
|---------|--------|
| Focus node | `fgRef.current.cameraPosition()` |
| Fit view | `fgRef.current.zoomToFit(400)` |
| Node color | `NODE_COLORS[node.type]` |
| Large graph | Disable drag for >1000 nodes |

---

## Basic Setup

```typescript
import ForceGraph3D from 'react-force-graph-3d';

interface GraphData {
  nodes: { id: string; label: string; type: string; color: string }[];
  links: { source: string; target: string; type: string }[];
}

export function Graph3D({ data }: { data: GraphData }) {
  const fgRef = useRef<any>(null);

  return (
    <ForceGraph3D
      ref={fgRef}
      graphData={data}
      nodeLabel="label"
      nodeColor="color"
      linkDirectionalArrowLength={3.5}
      linkDirectionalArrowRelPos={1}
      onNodeClick={handleNodeClick}
    />
  );
}
```

## Node Styling by Type

```typescript
const NODE_COLORS: Record<string, string> = {
  Project: '#10b981',      // emerald
  Locale: '#3b82f6',       // blue
  Source: '#8b5cf6',       // violet
  TranslationUnit: '#f59e0b', // amber
  AITranslation: '#ec4899', // pink
  HumanTranslation: '#14b8a6', // teal
  // ... etc
};

const nodeColor = useCallback((node: GraphNode) => {
  return NODE_COLORS[node.type] || '#6b7280';
}, []);

const nodeVal = useCallback((node: GraphNode) => {
  // Size based on connections
  return Math.max(1, node.connections?.length || 1);
}, []);
```

## Camera Controls

```typescript
// Focus on a specific node
const focusOnNode = useCallback((node: GraphNode) => {
  const distance = 100;
  const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z);

  fgRef.current?.cameraPosition(
    { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio },
    node, // lookAt
    2000  // transition duration ms
  );
}, []);

// Fit all nodes in view
const fitToView = useCallback(() => {
  fgRef.current?.zoomToFit(400);
}, []);
```

## Click Events

```typescript
const handleNodeClick = useCallback((node: GraphNode) => {
  setSelectedNode(node.id);
  focusOnNode(node);
}, [focusOnNode]);

const handleBackgroundClick = useCallback(() => {
  setSelectedNode(null);
}, []);

const handleNodeRightClick = useCallback((node: GraphNode) => {
  // Show context menu
  openContextMenu(node);
}, []);
```

## Performance for Large Graphs

```typescript
<ForceGraph3D
  // Reduce physics iterations
  d3AlphaDecay={0.02}
  d3VelocityDecay={0.3}
  // Simplify rendering
  nodeRelSize={4}
  // Disable expensive features for large graphs
  enableNodeDrag={nodes.length < 1000}
  enablePointerInteraction={nodes.length < 5000}
  // Use simpler geometries
  nodeThreeObject={null} // Use default spheres
/>
```

## Link Styling

```typescript
const linkColor = useCallback((link: GraphLink) => {
  const colors: Record<string, string> = {
    USES_ENTITY: '#10b981',
    HAS_OUTPUT: '#3b82f6',
    DERIVES_FROM: '#8b5cf6',
    CONTAINS: '#f59e0b',
  };
  return colors[link.type] || '#6b7280';
}, []);

const linkWidth = useCallback((link: GraphLink) => {
  return link.type === 'CONTAINS' ? 2 : 1;
}, []);
```
