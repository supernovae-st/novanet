# 3D Graph Visualization Design

**Date**: 2026-02-10
**Status**: Ready for Implementation
**Version**: NovaNet v11.6
**Research**: 10 parallel agents completed

## Overview

Replace the Mermaid diagram placeholder in GraphTab with a React Flow mini-graph, and add a 2D/3D toggle to the main canvas for immersive 3D graph visualization using Three.js force-graph.

**Key Feature**: Custom 3D geometries for each node classification (Layer → Shape, Trait → Material, Realm → Outline)

## Architecture

### Component Structure

```
apps/studio/src/
├── components/
│   └── graph/
│       ├── Graph2D.tsx              # Existing React Flow canvas
│       ├── Graph3D.tsx              # NEW: Three.js force graph wrapper
│       ├── GraphViewToggle.tsx      # NEW: 2D/3D segmented control
│       ├── GraphCanvas.tsx          # NEW: View switcher wrapper
│       ├── 3d/
│       │   ├── ForceGraph3DWrapper.tsx  # react-force-graph-3d setup
│       │   ├── Node3DRenderer.tsx       # Custom node geometries
│       │   ├── Link3DRenderer.tsx       # Arc particles/effects
│       │   └── geometries/
│       │       ├── LayerGeometries.tsx  # 9 unique shapes
│       │       ├── TraitMaterials.tsx   # 5 material effects
│       │       └── ArcParticles.tsx     # 5 particle systems
│       └── mini/
│           ├── EgoMiniGraph.tsx     # Sidebar React Flow preview
│           ├── MiniNode.tsx         # Simplified node
│           └── MiniEdge.tsx         # Simplified edge
│
├── lib/
│   └── graph3d/
│       ├── dataTransform.ts         # React Flow → ForceGraph3D
│       ├── colorPalette.ts          # From taxonomy.yaml
│       └── geometryFactory.ts       # Create geometries by classification
│
└── stores/
    └── uiStore.ts                   # ADD: graphViewMode: '2d' | '3d'
```

### Store Changes

```typescript
// uiStore.ts
interface UIState {
  // ... existing
  graphViewMode: '2d' | '3d';
  setGraphViewMode: (mode: '2d' | '3d') => void;
}
```

---

## Visual Encoding System (from taxonomy.yaml + visual-encoding.yaml)

### Design Principle

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  3D NODE APPEARANCE = f(Layer, Trait, Realm)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SHAPE (primary)     → Layer    → 9 unique Three.js geometries              │
│  MATERIAL (effect)   → Trait    → 5 material types (solid/wire/glass/etc)   │
│  OUTLINE COLOR       → Realm    → cyan (shared) / purple (org)              │
│  FILL COLOR          → Layer    → from taxonomy.yaml layer colors           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1. Node Shapes by Layer (9 unique geometries)

| Layer | Three.js Geometry | Rationale | Icon |
|-------|-------------------|-----------|------|
| **config** | `OctahedronGeometry` | 8-faced crystal = system settings | ⚙ |
| **locale** | `SphereGeometry` + `TorusGeometry` (rings) | Globe with latitude rings | ⊕ |
| **geography** | `IcosahedronGeometry` | 20-faced polyhedron = territory/map | ⊙ |
| **knowledge** | `TorusGeometry` | Ring = continuous wisdom flow | ◈ |
| **foundation** | `BoxGeometry` | Cube = stable base | ▣ |
| **structure** | `TetrahedronGeometry` | 4-faced pyramid = hierarchy | ▤ |
| **semantic** | `DodecahedronGeometry` | 12-faced = complex meaning | ◆ |
| **instruction** | `ConeGeometry` | Arrow pointing = direction | ▧ |
| **output** | `SphereGeometry` + Bloom | Glowing orb = generated | ● |

### 2. Node Materials by Trait (5 effects)

| Trait | Material Type | Properties | Visual |
|-------|---------------|------------|--------|
| **invariant** | `MeshStandardMaterial` | Solid, metalness: 0.3 | Solid opaque mesh |
| **localized** | Wireframe | `wireframe: true` | See-through structure |
| **knowledge** | `MeshPhysicalMaterial` | transmission: 0.7, ior: 1.5 | Glass/crystal effect |
| **generated** | Particle System | Points + animation | Floating particle cloud |
| **aggregated** | `PointsMaterial` | Static points, size: 2 | Point cloud |

### 3. Colors (from taxonomy.yaml)

```typescript
// lib/graph3d/colorPalette.ts

// Layer colors (fill)
export const LAYER_COLORS = {
  config: '#64748b',     // slate-500
  locale: '#64748b',     // slate-500
  geography: '#10b981',  // emerald-500
  knowledge: '#8b5cf6',  // violet-500
  foundation: '#3b82f6', // blue-500
  structure: '#06b6d4',  // cyan-500
  semantic: '#f97316',   // orange-500
  instruction: '#eab308', // yellow-500
  output: '#22c55e',     // green-500
};

// Realm colors (outline/border)
export const REALM_COLORS = {
  shared: '#2aa198',     // solarized cyan
  org: '#6c71c4',        // solarized purple
};

// Trait colors
export const TRAIT_COLORS = {
  invariant: '#3b82f6',  // blue-500
  localized: '#22c55e',  // green-500
  knowledge: '#8b5cf6',  // violet-500
  generated: '#b58900',  // solarized yellow
  aggregated: '#6c71c4', // solarized purple
};

// Arc family colors
export const ARC_FAMILY_COLORS = {
  ownership: '#3b82f6',    // blue
  localization: '#22c55e', // green
  semantic: '#f97316',     // orange
  generation: '#8b5cf6',   // purple
  mining: '#ec4899',       // pink
};
```

---

## Arc Particle Effects (5 families)

| Arc Family | Line Style | Particle Type | Behavior | Color |
|------------|------------|---------------|----------|-------|
| **ownership** | Solid, 2px | Pulse dots | Flow source→target, steady | `#3b82f6` |
| **localization** | Dashed | Globe orbs | Slow orbit, scatter | `#22c55e` |
| **semantic** | Dotted | Sparks | Random zigzag, fade | `#f97316` |
| **generation** | Solid, 3px | Matrix rain | Fast cascade, glow | `#8b5cf6` |
| **mining** | Dashed, 1px | Radar sweep | Circular scan, pulse | `#ec4899` |

### react-force-graph-3d Link Configuration

```typescript
// lib/graph3d/arcParticles.ts

export const ARC_PARTICLE_CONFIG = {
  ownership: {
    particles: 4,
    particleSpeed: 0.005,
    particleWidth: 2,
    particleColor: '#3b82f6',
  },
  localization: {
    particles: 2,
    particleSpeed: 0.002,
    particleWidth: 3,
    particleColor: '#22c55e',
  },
  semantic: {
    particles: 6,
    particleSpeed: 0.008,
    particleWidth: 1.5,
    particleColor: '#f97316',
  },
  generation: {
    particles: 8,
    particleSpeed: 0.01,
    particleWidth: 2.5,
    particleColor: '#8b5cf6',
  },
  mining: {
    particles: 3,
    particleSpeed: 0.003,
    particleWidth: 1,
    particleColor: '#ec4899',
  },
};
```

---

## Implementation Phases

### Phase 1: Sidebar Mini-Graph (React Flow)

**Files to create/modify:**
- `components/graph/mini/EgoMiniGraph.tsx`
- `components/graph/mini/MiniNode.tsx`
- `components/graph/mini/MiniEdge.tsx`
- `components/sidebar/tabs/GraphTab.tsx` (update)

**Features:**
- Compact React Flow instance in sidebar
- Simplified nodes (40x40px shapes by layer)
- Color by layer, shape outline by realm
- Click to navigate in main canvas
- Auto-fit to container

### Phase 2: GraphViewToggle

**Files to create/modify:**
- `components/graph/GraphViewToggle.tsx`
- `components/graph/GraphCanvas.tsx`
- `stores/uiStore.ts` (add graphViewMode)

**Features:**
- Segmented control at bottom of canvas
- Smooth transition animation (fade)
- Keyboard shortcuts: `2` for 2D, `3` for 3D
- Persist preference in localStorage

### Phase 3: Graph3D Component

**Dependencies to install:**
```bash
pnpm add three @react-three/fiber @react-three/drei react-force-graph-3d
pnpm add -D @types/three
```

**Files to create:**
- `components/graph/Graph3D.tsx`
- `components/graph/3d/ForceGraph3DWrapper.tsx`
- `components/graph/3d/Node3DRenderer.tsx`
- `components/graph/3d/Link3DRenderer.tsx`
- `components/graph/3d/geometries/*.tsx`
- `lib/graph3d/*.ts`

**Features:**
- ForceGraph3D with NovaNet data
- Custom node geometries by Layer
- Materials by Trait
- Particles on arcs by ArcFamily
- OrbitControls for navigation
- Click/hover interactions synced with 2D

### Phase 4: Polish & Sync

**Features:**
- Selection sync between 2D/3D views
- Camera position approximation when switching
- LOD for 3D (simplify distant nodes)
- Performance: instanced meshes for nodes
- Bloom/glow effects for selected nodes
- Edge highlighting on node hover

---

## Data Transformation

```typescript
// lib/graph3d/dataTransform.ts

import { LAYER_COLORS, REALM_COLORS, ARC_FAMILY_COLORS } from './colorPalette';
import { getGeometryForLayer, getMaterialForTrait } from './geometryFactory';

interface ForceGraphNode {
  id: string;
  name: string;
  group: string;      // layer
  color: string;      // from LAYER_COLORS
  val: number;        // node size (by importance)
  realm: 'shared' | 'org';
  layer: NodeLayer;
  trait: NodeTrait;
  // Custom 3D properties
  __threeObj?: THREE.Object3D;
}

interface ForceGraphLink {
  source: string;
  target: string;
  color: string;      // from ARC_FAMILY_COLORS
  family: ArcFamily;
  // Particle config
  particles: number;
  particleSpeed: number;
  particleWidth: number;
  particleColor: string;
}

export function transformToForceGraph(
  nodes: GraphNode[],
  edges: Edge[]
): { nodes: ForceGraphNode[]; links: ForceGraphLink[] } {
  const forceNodes = nodes.map(node => ({
    id: node.id,
    name: node.data.label,
    group: node.data.layer,
    color: LAYER_COLORS[node.data.layer],
    val: getNodeSize(node),
    realm: node.data.realm,
    layer: node.data.layer,
    trait: node.data.trait,
  }));

  const forceLinks = edges.map(edge => ({
    source: edge.source,
    target: edge.target,
    color: ARC_FAMILY_COLORS[edge.data.family],
    family: edge.data.family,
    ...ARC_PARTICLE_CONFIG[edge.data.family],
  }));

  return { nodes: forceNodes, links: forceLinks };
}
```

---

## Node 3D Renderer

```typescript
// components/graph/3d/Node3DRenderer.tsx

import * as THREE from 'three';
import { LAYER_COLORS, REALM_COLORS } from '@/lib/graph3d/colorPalette';

const LAYER_GEOMETRIES: Record<NodeLayer, THREE.BufferGeometry> = {
  config: new THREE.OctahedronGeometry(5),
  locale: new THREE.SphereGeometry(5, 16, 16),
  geography: new THREE.IcosahedronGeometry(5),
  knowledge: new THREE.TorusGeometry(4, 1.5, 8, 24),
  foundation: new THREE.BoxGeometry(8, 8, 8),
  structure: new THREE.TetrahedronGeometry(6),
  semantic: new THREE.DodecahedronGeometry(5),
  instruction: new THREE.ConeGeometry(4, 8, 16),
  output: new THREE.SphereGeometry(5, 32, 32),
};

function createNodeMaterial(trait: NodeTrait, color: string): THREE.Material {
  switch (trait) {
    case 'invariant':
      return new THREE.MeshStandardMaterial({
        color,
        metalness: 0.3,
        roughness: 0.7,
      });
    case 'localized':
      return new THREE.MeshBasicMaterial({
        color,
        wireframe: true,
      });
    case 'knowledge':
      return new THREE.MeshPhysicalMaterial({
        color,
        transmission: 0.7,
        thickness: 2,
        roughness: 0.1,
        ior: 1.5,
        toneMapped: false,
      });
    case 'generated':
      // Return point material for particle effect
      return new THREE.PointsMaterial({
        color,
        size: 2,
        transparent: true,
        opacity: 0.8,
      });
    case 'aggregated':
      return new THREE.PointsMaterial({
        color,
        size: 1.5,
        transparent: true,
        opacity: 0.6,
      });
    default:
      return new THREE.MeshStandardMaterial({ color });
  }
}

export function createNode3D(node: ForceGraphNode): THREE.Object3D {
  const geometry = LAYER_GEOMETRIES[node.layer];
  const material = createNodeMaterial(node.trait, node.color);

  let mesh: THREE.Object3D;

  if (node.trait === 'generated' || node.trait === 'aggregated') {
    // Create point cloud for generated/aggregated
    const points = new THREE.Points(geometry, material);
    mesh = points;
  } else {
    mesh = new THREE.Mesh(geometry, material);

    // Add realm outline
    const outlineGeometry = new THREE.EdgesGeometry(geometry);
    const outlineMaterial = new THREE.LineBasicMaterial({
      color: REALM_COLORS[node.realm],
      linewidth: 2,
    });
    const outline = new THREE.LineSegments(outlineGeometry, outlineMaterial);
    mesh.add(outline);
  }

  // Special case: locale gets rings
  if (node.layer === 'locale') {
    const ringGeometry = new THREE.TorusGeometry(6, 0.3, 8, 32);
    const ringMaterial = new THREE.MeshBasicMaterial({
      color: REALM_COLORS[node.realm],
      transparent: true,
      opacity: 0.6,
    });
    const ring = new THREE.Mesh(ringGeometry, ringMaterial);
    ring.rotation.x = Math.PI / 2;
    mesh.add(ring);
  }

  // Special case: output gets glow
  if (node.layer === 'output') {
    // Glow handled by Bloom postprocessing
    (mesh as THREE.Mesh).material = new THREE.MeshStandardMaterial({
      color: node.color,
      emissive: node.color,
      emissiveIntensity: 0.5,
    });
  }

  return mesh;
}
```

---

## Graph3D Component

```typescript
// components/graph/Graph3D.tsx

'use client';

import { useCallback, useMemo, useRef } from 'react';
import ForceGraph3D from 'react-force-graph-3d';
import { useGraphStore } from '@/stores/graphStore';
import { transformToForceGraph } from '@/lib/graph3d/dataTransform';
import { createNode3D } from './3d/Node3DRenderer';
import { ARC_PARTICLE_CONFIG } from '@/lib/graph3d/arcParticles';

export function Graph3D() {
  const fgRef = useRef<any>();
  const { nodes, edges, selectedNodeId, setSelectedNodeId } = useGraphStore();

  const graphData = useMemo(
    () => transformToForceGraph(nodes, edges),
    [nodes, edges]
  );

  const handleNodeClick = useCallback((node: any) => {
    setSelectedNodeId(node.id);
    // Zoom to node
    const distance = 100;
    const distRatio = 1 + distance / Math.hypot(node.x, node.y, node.z);
    fgRef.current?.cameraPosition(
      { x: node.x * distRatio, y: node.y * distRatio, z: node.z * distRatio },
      node,
      2000
    );
  }, [setSelectedNodeId]);

  return (
    <ForceGraph3D
      ref={fgRef}
      graphData={graphData}
      nodeThreeObject={createNode3D}
      nodeThreeObjectExtend={false}
      linkColor={(link: any) => link.color}
      linkWidth={(link: any) => link.particleWidth}
      linkDirectionalParticles={(link: any) => link.particles}
      linkDirectionalParticleSpeed={(link: any) => link.particleSpeed}
      linkDirectionalParticleWidth={(link: any) => link.particleWidth}
      linkDirectionalParticleColor={(link: any) => link.particleColor}
      onNodeClick={handleNodeClick}
      backgroundColor="rgba(0,0,0,0)"
      showNavInfo={false}
    />
  );
}
```

---

## Interactions

### 2D/3D Sync

| Action in 2D | Effect in 3D |
|--------------|--------------|
| Select node | Zoom to node, highlight |
| Hover node | Highlight connected arcs |
| Pan/zoom | Approximate camera position |
| Filter facets | Same nodes visible |

| Action in 3D | Effect in 2D |
|--------------|--------------|
| Click node | Select in React Flow |
| Orbit camera | No direct mapping |
| Double-click | Reset camera + fit view |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `2` | Switch to 2D view |
| `3` | Switch to 3D view |
| `R` | Reset camera (3D) |
| `F` | Fit view (both) |

---

## Performance Considerations

1. **Instanced Meshes**: Use `THREE.InstancedMesh` for nodes (60+ nodes)
2. **LOD**: Simplify geometry for distant nodes
3. **Culling**: Don't render nodes behind camera
4. **Throttle**: Debounce force simulation updates
5. **Web Workers**: Offload physics to worker (force-graph supports this)
6. **Geometry Reuse**: Cache geometries per layer (9 total)
7. **Material Reuse**: Cache materials per trait (5 total)

---

## Dependencies

```json
{
  "three": "^0.170.0",
  "@react-three/fiber": "^8.17.0",
  "@react-three/drei": "^9.117.0",
  "@react-three/postprocessing": "^2.16.0",
  "react-force-graph-3d": "^1.24.0"
}
```

---

## Success Criteria

1. [ ] Sidebar mini-graph renders ego network
2. [ ] 2D/3D toggle works smoothly with fade transition
3. [ ] 3D view shows all 60 nodes with correct shapes by Layer
4. [ ] Node materials change by Trait (solid/wireframe/glass/particles)
5. [ ] Node outlines colored by Realm (cyan/purple)
6. [ ] Arc particles animate by family (5 distinct styles)
7. [ ] Selection syncs between views
8. [ ] Performance: 60fps on M1 MacBook
9. [ ] Keyboard shortcuts work (2, 3, R, F)
10. [ ] Locale nodes have ring decorations
11. [ ] Output nodes glow with Bloom effect

---

## References

- [react-force-graph-3d](https://github.com/vasturiano/react-force-graph)
- [react-three-fiber](https://docs.pmnd.rs/react-three-fiber)
- [drei](https://github.com/pmndrs/drei)
- NovaNet `packages/core/models/taxonomy.yaml`
- NovaNet `packages/core/models/visual-encoding.yaml`
- SuperNovae Pad 3D: `apps/studio/src/components/configurator/` (existing R3F reference)

---

## ASCII Mockups

### 2D/3D Toggle Position

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                         GRAPH CANVAS (2D or 3D)                             │
│                                                                             │
│     ◆───────◈                    OR           ◆                             │
│     │       │                              ╱   ╲                            │
│     ●───────▣───────▤                    ◈──────●                          │
│             │                              ╲   ╱                            │
│             ⚙                                ▣                              │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                   [████ 2D Graph ████|     3D Graph     ]                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Node Shape Gallery (9 Layers)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  LAYER SHAPES (3D View)                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   ◇ config         ⊕ locale          ⬡ geography                           │
│   Octahedron       Sphere+Ring       Icosahedron                           │
│                                                                             │
│   ○ knowledge      □ foundation      △ structure                           │
│   Torus            Box               Tetrahedron                           │
│                                                                             │
│   ⬟ semantic       ▲ instruction     ● output                              │
│   Dodecahedron     Cone              Sphere+Glow                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Trait Materials (5 Types)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  TRAIT MATERIALS                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  invariant    localized    knowledge    generated    aggregated             │
│  ┌───────┐    ┌───────┐    ┌───────┐    ·  · · ·     · · · · ·             │
│  │███████│    │ ╱ ╲ ╱ │    │░░░░░░░│    ·· · ·  ·    · · · · ·             │
│  │███████│    │╱ ╳ ╲╱ │    │░░░░░░░│    · ·  ·· ·    ·  ·  ·  ·            │
│  │███████│    │╲ ╳ ╱╲ │    │░░░░░░░│    ·· ·  · ·    · · · · ·             │
│  └───────┘    └───────┘    └───────┘    · ·  ·· ·    · · · · ·             │
│   Solid       Wireframe    Glass        Particles    Points                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Arc Particle Effects (5 Families)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  ARC PARTICLES BY FAMILY                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ownership      ○──●──○──●──○──●──○──●→     Steady pulse flow               │
│  (blue)                                                                     │
│                                                                             │
│  localization   ◎- - - ◎- - - ◎- - -◎→     Slow orbiting globs             │
│  (green)                                                                    │
│                                                                             │
│  semantic       ✦~✦~✦~✦~✦~✦~✦~✦~✦~→     Random sparking zigzag            │
│  (orange)                                                                   │
│                                                                             │
│  generation     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓→     Fast matrix cascade               │
│  (purple)                                                                   │
│                                                                             │
│  mining         ◠ ◡ ◠ ◡ ◠ ◡ ◠ ◡→     Radar sweep pulse                    │
│  (pink)                                                                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
