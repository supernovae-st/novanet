# Graph3D WOW Enhancements Design

**Date**: 2026-02-10
**Status**: Draft
**Version**: 1.0

## Executive Summary

Research from 4 parallel agents (Perplexity trends, Context7 Three.js docs, Interactive patterns, react-force-graph advanced) reveals a clear path to making our 3D graph visualization significantly more impressive and interactive.

## Current State Analysis

Our Graph3D already has solid foundations:
- 9 geometries by Layer (sphere, box, octahedron, etc.)
- 5 materials by Trait (solid, wireframe, glass, emissive, points)
- Realm indicator rings
- Arc particles by ArcFamily
- Starfield background
- Basic hover scale + click zoom

**What's Missing**:
- Post-processing effects (no bloom, no depth)
- Focus+context visualization (everything at same opacity)
- Smooth camera transitions
- Semantic zooming (same detail at all distances)
- Custom shaders for unique effects

---

## Enhancement Roadmap

### Phase 1: Visual WOW (High Impact, Medium Effort)

#### 1.1 Post-Processing Pipeline

Add EffectComposer with bloom, SSAO, and vignette:

```tsx
// Integration approach for react-force-graph-3d
useEffect(() => {
  const renderer = fgRef.current.renderer();
  const scene = fgRef.current.scene();
  const camera = fgRef.current.camera();
  const composer = fgRef.current.postProcessingComposer();

  // Bloom for glowing nodes
  composer.addPass(new UnrealBloomPass(
    new THREE.Vector2(window.innerWidth, window.innerHeight),
    1.0,   // strength
    0.4,   // radius
    0.85   // threshold
  ));
}, []);
```

**Bloom Settings by Node Type**:
| Node State | Emissive Intensity | Effect |
|------------|-------------------|--------|
| Selected | 3.0 | Strong glow |
| Hovered | 1.5 | Medium glow |
| Generated trait | 1.0 | Subtle constant glow |
| Others | 0 | No glow |

#### 1.2 Depth of Field on Focus

When a node is selected, blur the background:

```tsx
const focusDistance = useMemo(() => {
  if (!selectedNode) return 0.5;
  const nodePos = new THREE.Vector3(selectedNode.x, selectedNode.y, selectedNode.z);
  const cameraPos = fgRef.current.camera().position;
  return nodePos.distanceTo(cameraPos) / 500; // Normalize
}, [selectedNode]);
```

#### 1.3 Vignette Framing

Subtle darkening at edges for cinematic feel:
- `offset`: 0.1
- `darkness`: 0.5

---

### Phase 2: Interactive WOW (High Impact, Low Effort)

#### 2.1 Focus+Context Dimming

When a node is selected, fade unrelated nodes:

```tsx
function getNodeOpacity(node: ForceGraphNode, selectedId: string | null, neighborIds: Set<string>) {
  if (!selectedId) return 1.0;  // No selection = all visible
  if (node.id === selectedId) return 1.0;  // Selected = full
  if (neighborIds.has(node.id)) return 0.8;  // Neighbors = visible
  return 0.15;  // Others = ghosted
}
```

**Visual Recipe**:
- Selected: 100% opacity, 1.5x scale, bloom
- 1-hop neighbors: 80% opacity, 1.0x scale
- 2-hop neighbors: 50% opacity, 0.8x scale (optional)
- Others: 15% opacity, 0.6x scale

#### 2.2 Zoom-to-Selection Animation

Improve camera animation with spring easing:

```tsx
const zoomToNode = useCallback((node: ForceGraphNode) => {
  const distance = 80;  // Fixed distance for consistency
  const duration = 1500;  // Smoother, longer animation

  fgRef.current.cameraPosition(
    {
      x: node.x + distance * 0.5,
      y: node.y + distance * 0.3,
      z: node.z + distance * 0.8,
    },
    { x: node.x, y: node.y, z: node.z },  // Look at node
    duration
  );
}, []);
```

#### 2.3 Highlight Propagation

When hovering, highlight connected arcs:

```tsx
const handleNodeHover = useCallback((node: ForceGraphNode | null) => {
  if (!node) {
    setHighlightedLinks(new Set());
    return;
  }

  const connectedLinks = graphData.links.filter(
    link => link.source.id === node.id || link.target.id === node.id
  );
  setHighlightedLinks(new Set(connectedLinks.map(l => l.id)));
}, [graphData.links]);

// In link rendering:
linkWidth={link => highlightedLinks.has(link.id) ? 4 : 1}
linkOpacity={link => highlightedLinks.has(link.id) ? 0.9 : 0.3}
```

#### 2.4 Constrained Orbit Camera

Prevent user disorientation:

```tsx
// Access controls after mount
const controls = fgRef.current.controls();
controls.minPolarAngle = Math.PI * 0.1;  // Don't go fully overhead
controls.maxPolarAngle = Math.PI * 0.9;  // Don't go fully underneath
controls.minDistance = 50;
controls.maxDistance = 500;
controls.enableDamping = true;
controls.dampingFactor = 0.05;
```

---

### Phase 3: Performance WOW (Medium Impact, High Effort)

#### 3.1 Semantic Zooming (LOD)

Show different detail levels based on camera distance:

```tsx
type LODLevel = 'far' | 'medium' | 'close' | 'detail';

function getLODLevel(cameraDistance: number): LODLevel {
  if (cameraDistance > 400) return 'far';
  if (cameraDistance > 200) return 'medium';
  if (cameraDistance > 80) return 'close';
  return 'detail';
}

function renderNodeAtLOD(node: ForceGraphNode, lod: LODLevel) {
  switch (lod) {
    case 'far':
      // Point sprite (fastest)
      return createPointSprite(node);
    case 'medium':
      // Simple sphere (fast)
      return createSimpleSphere(node);
    case 'close':
      // Current full geometry
      return renderFullNode(node);
    case 'detail':
      // Full geometry + label billboard
      return (
        <>
          {renderFullNode(node)}
          <Html position={[node.x, node.y + 10, node.z]}>
            <div className="node-label">{node.name}</div>
          </Html>
        </>
      );
  }
}
```

#### 3.2 Layer Z-Positioning

Use Z-axis to separate layers visually:

```tsx
const LAYER_Z_POSITIONS: Record<string, number> = {
  config: 0,
  locale: 40,
  geography: 80,
  knowledge: 120,
  foundation: 160,
  structure: 200,
  semantic: 240,
  instruction: 280,
  output: 320,
};

// In D3 force configuration
fgRef.current.d3Force('z', d3.forceZ(
  (node: any) => LAYER_Z_POSITIONS[node.layer] || 0
).strength(0.3));
```

#### 3.3 Realm X-Positioning

Separate shared (left) from org (right):

```tsx
const REALM_X_OFFSET = {
  shared: -80,
  org: 80,
};

fgRef.current.d3Force('x', d3.forceX(
  (node: any) => REALM_X_OFFSET[node.realm] || 0
).strength(0.2));
```

---

### Phase 4: Polish WOW (Low Impact, Fun)

#### 4.1 Galaxy Boot Animation

Nodes spiral in from center on initial load:

```tsx
const [bootPhase, setBootPhase] = useState<'loading' | 'spawning' | 'ready'>('loading');

useEffect(() => {
  if (graphData.nodes.length > 0 && bootPhase === 'loading') {
    setBootPhase('spawning');

    // Animate nodes from center over 2 seconds
    graphData.nodes.forEach((node, i) => {
      const delay = i * 20;  // Stagger
      setTimeout(() => {
        node.fx = undefined;  // Release from center
        node.fy = undefined;
        node.fz = undefined;
      }, delay);
    });

    setTimeout(() => setBootPhase('ready'), 2500);
  }
}, [graphData.nodes, bootPhase]);
```

#### 4.2 Supernova Selection Effect

Brief bloom burst when selecting important nodes:

```tsx
const [selectionBurst, setSelectionBurst] = useState<string | null>(null);

const handleNodeClick = useCallback((node: ForceGraphNode) => {
  setSelectedNode(node.id);

  // Trigger burst effect
  setSelectionBurst(node.id);
  setTimeout(() => setSelectionBurst(null), 500);
}, []);

// In material creation
const burstIntensity = selectionBurst === node.id ? 5.0 : 0;
```

#### 4.3 Idle Oscillation

Subtle movement when not interacting:

```tsx
useFrame((state) => {
  if (!isUserInteracting && bootPhase === 'ready') {
    graphData.nodes.forEach(node => {
      const wobble = Math.sin(state.clock.elapsedTime + node.id.charCodeAt(0)) * 0.02;
      node.y += wobble;
    });
  }
});
```

---

## Implementation Priority Matrix

| Enhancement | Impact | Effort | Priority |
|-------------|--------|--------|----------|
| Bloom on selection | High | Low | P0 |
| Focus+Context dimming | High | Low | P0 |
| Zoom-to-selection smooth | Medium | Low | P0 |
| Arc highlight propagation | Medium | Low | P0 |
| Vignette | Medium | Low | P1 |
| SSAO | Medium | Medium | P1 |
| Constrained orbit | Medium | Low | P1 |
| Depth of Field | Medium | Medium | P2 |
| Layer Z-positioning | Medium | Medium | P2 |
| Semantic zoom LOD | High | High | P2 |
| Boot animation | Low | Medium | P3 |
| Supernova burst | Low | Low | P3 |
| Idle oscillation | Low | Low | P3 |

---

## Technical Dependencies

```
@react-three/postprocessing  → Bloom, SSAO, DoF, Vignette
three (already installed)    → EffectComposer, passes
d3-force-3d (already used)   → Custom forces (Z/X positioning)
```

**Note**: react-force-graph-3d exposes `postProcessingComposer()` method - we can add passes directly without switching to @react-three/fiber.

---

## Questions for User

1. **Boot animation**: Should nodes spawn from center or fade in place?
2. **Layer separation**: Fixed Z-positions or let force layout decide?
3. **LOD labels**: Show node names at close zoom level?
4. **Sound**: Audio feedback on selection? (Future consideration)

---

## Success Metrics

- [ ] First impression "wow" within 2 seconds of load
- [ ] Clear visual hierarchy (selection vs context)
- [ ] 60fps with 100 nodes
- [ ] 30fps with 500 nodes
- [ ] No user disorientation (constrained camera)

---

## Next Steps

1. Create feature branch: `feat/graph3d-wow`
2. Implement P0 enhancements (1-2 hours)
3. Test with real NovaNet data
4. Iterate on P1-P2 based on feedback

---

## References

- [Three.js Post-Processing](https://threejs.org/docs/#examples/en/postprocessing/EffectComposer)
- [react-force-graph-3d API](https://github.com/vasturiano/react-force-graph)
- [Obsidian Graph View](https://help.obsidian.md/Plugins/Graph+view)
- [Neo4j Bloom](https://neo4j.com/bloom/)
