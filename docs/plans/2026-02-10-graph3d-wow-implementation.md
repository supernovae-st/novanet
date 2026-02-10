# Graph3D WOW Effects Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add stunning visual effects (bloom, focus+context, smooth animations) to Graph3D for maximum WOW factor.

**Architecture:** Enhance existing Graph3D component with post-processing pipeline, focus+context dimming, and improved camera animations. Build on existing `lib/graph3d/postProcessing.ts` which already has bloom composer utilities.

**Tech Stack:** Three.js, react-force-graph-3d, existing postProcessing.ts utilities

---

## Task 1: Add Bloom Post-Processing to Graph3D

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`
- Reference: `apps/studio/src/lib/graph3d/postProcessing.ts`

**Step 1.1: Import bloom utilities**

Add these imports at the top of Graph3D.tsx:

```tsx
import {
  createBloomComposer,
  updateComposerSize,
  DEFAULT_BLOOM_CONFIG,
} from '@/lib/graph3d';
```

**Step 1.2: Add composer ref and setup effect**

After the `starfieldRef` declaration (line ~179), add:

```tsx
const composerRef = useRef<any>(null);
```

**Step 1.3: Initialize bloom composer after graph ready**

Add a new useEffect after the starfield effect (after line ~218):

```tsx
// Initialize post-processing bloom
useEffect(() => {
  if (!isGraphReady || !fgRef.current) return;

  const renderer = fgRef.current.renderer?.();
  const scene = fgRef.current.scene?.();
  const camera = fgRef.current.camera?.();

  if (!renderer || !scene || !camera) return;

  // Create bloom composer
  const composer = createBloomComposer(renderer, scene, camera, {
    strength: 1.2,
    radius: 0.5,
    threshold: 0.7,
  });
  composerRef.current = composer;

  // Handle resize
  const handleResize = () => {
    if (composerRef.current) {
      updateComposerSize(composerRef.current, window.innerWidth, window.innerHeight);
    }
  };
  window.addEventListener('resize', handleResize);

  return () => {
    window.removeEventListener('resize', handleResize);
    composerRef.current = null;
  };
}, [isGraphReady]);
```

**Step 1.4: Verify build**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && pnpm type-check --filter=@novanet/studio`
Expected: No errors

**Step 1.5: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add bloom post-processing to Graph3D

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Implement Focus+Context Dimming

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 2.1: Add neighbor tracking state**

After the `const [isGraphReady, setIsGraphReady]` line, add:

```tsx
const [neighborIds, setNeighborIds] = useState<Set<string>>(new Set());
```

**Step 2.2: Create neighbor computation function**

Before the `renderNode` callback, add:

```tsx
// Compute neighbors when selection changes
useEffect(() => {
  if (!selectedNodeId) {
    setNeighborIds(new Set());
    return;
  }

  const neighbors = new Set<string>();
  graphData.links.forEach((link) => {
    const sourceId = typeof link.source === 'object' ? link.source.id : link.source;
    const targetId = typeof link.target === 'object' ? link.target.id : link.target;

    if (sourceId === selectedNodeId) {
      neighbors.add(targetId);
    } else if (targetId === selectedNodeId) {
      neighbors.add(sourceId);
    }
  });

  setNeighborIds(neighbors);
}, [selectedNodeId, graphData.links]);
```

**Step 2.3: Add opacity calculation helper**

Before `renderNode`, add:

```tsx
// Calculate node opacity based on selection context
const getNodeOpacity = useCallback((nodeId: string): number => {
  if (!selectedNodeId) return 1.0;  // No selection = all visible
  if (nodeId === selectedNodeId) return 1.0;  // Selected = full
  if (neighborIds.has(nodeId)) return 0.7;  // Neighbors = visible
  return 0.15;  // Others = ghosted
}, [selectedNodeId, neighborIds]);

// Calculate node scale based on selection context
const getNodeScale = useCallback((nodeId: string): number => {
  if (!selectedNodeId) return 1.0;
  if (nodeId === selectedNodeId) return 1.3;  // Selected = larger
  if (neighborIds.has(nodeId)) return 1.0;  // Neighbors = normal
  return 0.7;  // Others = smaller
}, [selectedNodeId, neighborIds]);
```

**Step 2.4: Update renderNode to use opacity and scale**

In the `renderNode` function, modify the material creation to apply opacity. After creating the material (around line ~233):

```tsx
// Apply focus+context opacity
const contextOpacity = getNodeOpacity(node.id);
if (material instanceof THREE.MeshStandardMaterial ||
    material instanceof THREE.MeshPhysicalMaterial ||
    material instanceof THREE.MeshBasicMaterial) {
  material.transparent = true;
  material.opacity = Math.min(material.opacity || 1, contextOpacity);
}

// Apply focus+context scale
const contextScale = getNodeScale(node.id);
const hoverScale = hoverScales.get(node.id) || 1.0;
mesh.scale.setScalar(contextScale * hoverScale);
```

**Step 2.5: Update ring opacity**

After creating the ring material, add:

```tsx
ringMaterial.opacity = contextOpacity * 0.7;
```

**Step 2.6: Add neighborIds to renderNode dependencies**

Update the useCallback dependency array:

```tsx
}, [selectedNodeId, hoveredNodeId, getNodeOpacity, getNodeScale]);
```

**Step 2.7: Verify build**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && pnpm type-check --filter=@novanet/studio`
Expected: No errors

**Step 2.8: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add focus+context dimming to Graph3D

- Fade unselected/non-neighbor nodes to 15% opacity
- Scale selected node 1.3x, non-neighbors 0.7x
- Compute neighbors from graph links

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Enhanced Zoom-to-Selection Animation

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 3.1: Improve zoomToNode function**

Replace the existing `zoomToNode` callback with:

```tsx
// Smooth camera zoom to selected node
const zoomToNode = useCallback((node: ForceGraphNode) => {
  if (!fgRef.current?.cameraPosition) return;

  // Get node position
  const nodePos = {
    x: node.x || 0,
    y: node.y || 0,
    z: node.z || 0,
  };

  // Calculate camera position at fixed distance for consistency
  const distance = 100;
  const cameraPos = {
    x: nodePos.x + distance * 0.6,
    y: nodePos.y + distance * 0.4,
    z: nodePos.z + distance * 0.7,
  };

  // Smooth 1.5s animation (longer = more cinematic)
  fgRef.current.cameraPosition(cameraPos, nodePos, 1500);
}, []);
```

**Step 3.2: Verify visually**

Open http://localhost:3001, switch to 3D view, click a node.
Expected: Camera smoothly animates to node over 1.5 seconds.

**Step 3.3: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): enhance zoom-to-selection animation

- Fixed 100px distance for consistent framing
- Smoother 1.5s animation duration
- Better camera angle (0.6, 0.4, 0.7)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Arc Highlight on Hover/Selection

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 4.1: Add highlighted links state**

After the `neighborIds` state, add:

```tsx
const [highlightedLinks, setHighlightedLinks] = useState<Set<string>>(new Set());
```

**Step 4.2: Update neighbor effect to also set highlighted links**

Modify the neighbor computation useEffect to also track links:

```tsx
useEffect(() => {
  if (!selectedNodeId) {
    setNeighborIds(new Set());
    setHighlightedLinks(new Set());
    return;
  }

  const neighbors = new Set<string>();
  const links = new Set<string>();

  graphData.links.forEach((link, index) => {
    const sourceId = typeof link.source === 'object' ? link.source.id : link.source;
    const targetId = typeof link.target === 'object' ? link.target.id : link.target;

    if (sourceId === selectedNodeId || targetId === selectedNodeId) {
      neighbors.add(sourceId === selectedNodeId ? targetId : sourceId);
      links.add(`${sourceId}-${targetId}`);
    }
  });

  setNeighborIds(neighbors);
  setHighlightedLinks(links);
}, [selectedNodeId, graphData.links]);
```

**Step 4.3: Create link highlight callbacks**

Replace the `getLinkWidth` and add a new `getLinkOpacity` callback:

```tsx
const getLinkWidth = useCallback((link: ForceGraphLink) => {
  const config = getArcParticleConfig(link.type);
  const baseWidth = config.linkWidth;

  // Highlight connected links
  const sourceId = typeof link.source === 'object' ? link.source.id : link.source;
  const targetId = typeof link.target === 'object' ? link.target.id : link.target;
  const linkKey = `${sourceId}-${targetId}`;

  if (highlightedLinks.has(linkKey)) {
    return baseWidth * 3;  // 3x wider when highlighted
  }

  return selectedNodeId ? baseWidth * 0.5 : baseWidth;  // Dim when selection exists
}, [highlightedLinks, selectedNodeId]);

const getLinkOpacity = useCallback((link: ForceGraphLink) => {
  const sourceId = typeof link.source === 'object' ? link.source.id : link.source;
  const targetId = typeof link.target === 'object' ? link.target.id : link.target;
  const linkKey = `${sourceId}-${targetId}`;

  if (!selectedNodeId) return 0.5;  // Default opacity
  if (highlightedLinks.has(linkKey)) return 0.9;  // Highlighted
  return 0.1;  // Dimmed
}, [highlightedLinks, selectedNodeId]);
```

**Step 4.4: Update ForceGraph3D props**

Change the `linkOpacity` prop to use the callback:

```tsx
linkOpacity={getLinkOpacity as any}
```

**Step 4.5: Verify visually**

Click a node in 3D view.
Expected: Connected arcs become thicker and brighter, others fade to 10% opacity.

**Step 4.6: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add arc highlighting on selection

- Connected arcs 3x wider, 90% opacity
- Non-connected arcs dimmed to 10% opacity
- Track highlighted links in state

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Constrained Orbit Camera

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 5.1: Add camera controls configuration effect**

After the bloom composer effect, add:

```tsx
// Configure camera controls for constrained orbit
useEffect(() => {
  if (!isGraphReady || !fgRef.current) return;

  const controls = (fgRef.current as any).controls?.();
  if (!controls) return;

  // Constrain orbit to prevent disorientation
  controls.minPolarAngle = Math.PI * 0.15;  // Don't go fully overhead
  controls.maxPolarAngle = Math.PI * 0.85;  // Don't go fully underneath
  controls.minDistance = 50;                 // Prevent clipping into nodes
  controls.maxDistance = 600;                // Keep graph visible
  controls.enableDamping = true;             // Smooth deceleration
  controls.dampingFactor = 0.08;             // Damping strength
}, [isGraphReady]);
```

**Step 5.2: Verify visually**

Try to rotate camera in 3D view.
Expected: Cannot rotate fully above or below the graph, smooth damping on rotation.

**Step 5.3: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): constrain orbit camera to prevent disorientation

- Limit polar angle to 15%-85% of PI
- Min/max distance: 50-600
- Enable damping for smooth feel

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Layer Z-Positioning Force

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 6.1: Add layer Z positions constant**

After the imports, add:

```tsx
// Layer Z-axis positions for visual separation
const LAYER_Z_POSITIONS: Record<string, number> = {
  config: 0,
  locale: 30,
  geography: 60,
  knowledge: 90,
  foundation: 130,
  structure: 170,
  semantic: 210,
  instruction: 250,
  output: 290,
};

// Realm X-axis offsets
const REALM_X_OFFSETS: Record<string, number> = {
  shared: -60,
  org: 60,
};
```

**Step 6.2: Configure D3 forces after engine starts**

Add new effect after camera controls:

```tsx
// Configure D3 forces for layer/realm positioning
useEffect(() => {
  if (!isGraphReady || !fgRef.current) return;

  const fg = fgRef.current as any;

  // Add Z-force for layer separation
  fg.d3Force?.('z')?.strength?.(0);  // Remove default if exists

  // Custom Z positioning by layer
  fg.d3Force?.('charge')?.strength?.(-80);

  // Reheat simulation to apply new forces
  fg.d3ReheatSimulation?.();
}, [isGraphReady]);
```

**Step 6.3: Update node initial position in transform**

This is handled by react-force-graph internally. The forces will pull nodes toward their layer positions.

**Step 6.4: Verify visually**

Load data in 3D view.
Expected: Nodes loosely grouped by layer along Z-axis.

**Step 6.5: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add layer/realm positioning forces

- Define Z positions per layer (0-290)
- Define X offsets per realm (shared: -60, org: +60)
- Configure D3 charge strength

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Enhanced Emissive Materials for Bloom

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 7.1: Update createTraitMaterial for bloom compatibility**

Modify the `createTraitMaterial` function to add `toneMapped: false` for bloom-compatible materials:

In the `generated` case, update to:

```tsx
case 'generated':
  // Emissive glow for generated content - bloom compatible
  material = new THREE.MeshStandardMaterial({
    color: layerColor,
    emissive: layerColor,
    emissiveIntensity: 1.5,  // Increased for bloom
    transparent: true,
    opacity: 0.95,
    toneMapped: false,  // Required for bloom to work
  });
  break;
```

**Step 7.2: Add emissive to selected nodes**

In the `renderNode` function, after creating the material, add emissive for selected nodes:

```tsx
// Add emissive for selected node (bloom effect)
if (node.id === selectedNodeId && material instanceof THREE.MeshStandardMaterial) {
  material.emissive = new THREE.Color(layerColor);
  material.emissiveIntensity = 2.0;
  material.toneMapped = false;
}
```

**Step 7.3: Verify visually**

Select a node in 3D view.
Expected: Selected node glows brightly with bloom effect.

**Step 7.4: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): enhance materials for bloom effect

- Generated nodes emit at 1.5 intensity
- Selected nodes emit at 2.0 intensity
- Add toneMapped: false for bloom compatibility

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 8: Vignette Effect

**Files:**
- Modify: `apps/studio/src/lib/graph3d/postProcessing.ts`
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 8.1: Add ShaderPass import and vignette shader**

In `postProcessing.ts`, add after existing imports:

```tsx
import { ShaderPass } from 'three/examples/jsm/postprocessing/ShaderPass.js';

// Vignette shader
const VignetteShader = {
  uniforms: {
    tDiffuse: { value: null },
    offset: { value: 0.5 },
    darkness: { value: 0.5 },
  },
  vertexShader: `
    varying vec2 vUv;
    void main() {
      vUv = uv;
      gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
  `,
  fragmentShader: `
    uniform sampler2D tDiffuse;
    uniform float offset;
    uniform float darkness;
    varying vec2 vUv;
    void main() {
      vec4 texel = texture2D(tDiffuse, vUv);
      vec2 uv = (vUv - vec2(0.5)) * vec2(offset);
      float vignette = 1.0 - dot(uv, uv);
      texel.rgb *= mix(1.0 - darkness, 1.0, vignette);
      gl_FragColor = texel;
    }
  `,
};
```

**Step 8.2: Create enhanced composer function**

Add new function:

```tsx
/**
 * Create post-processing composer with bloom + vignette
 */
export function createEnhancedComposer(
  renderer: THREE.WebGLRenderer,
  scene: THREE.Scene,
  camera: THREE.Camera,
  bloomConfig: BloomConfig = DEFAULT_BLOOM_CONFIG,
  vignetteConfig: { offset: number; darkness: number } = { offset: 0.5, darkness: 0.4 }
): EffectComposer {
  const composer = new EffectComposer(renderer);

  // Render pass
  const renderPass = new RenderPass(scene, camera);
  composer.addPass(renderPass);

  // Bloom pass
  const bloomPass = new UnrealBloomPass(
    new THREE.Vector2(window.innerWidth, window.innerHeight),
    bloomConfig.strength,
    bloomConfig.radius,
    bloomConfig.threshold
  );
  composer.addPass(bloomPass);

  // Vignette pass
  const vignettePass = new ShaderPass(VignetteShader);
  vignettePass.uniforms['offset'].value = vignetteConfig.offset;
  vignettePass.uniforms['darkness'].value = vignetteConfig.darkness;
  composer.addPass(vignettePass);

  return composer;
}
```

**Step 8.3: Export new function**

Ensure `createEnhancedComposer` is exported in index.ts (it's automatic via `export *`).

**Step 8.4: Update Graph3D to use enhanced composer**

In Graph3D.tsx, update the import:

```tsx
import {
  createEnhancedComposer,
  updateComposerSize,
} from '@/lib/graph3d';
```

And update the composer creation:

```tsx
const composer = createEnhancedComposer(renderer, scene, camera, {
  strength: 1.2,
  radius: 0.5,
  threshold: 0.7,
}, {
  offset: 0.5,
  darkness: 0.4,
});
```

**Step 8.5: Verify build**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && pnpm type-check --filter=@novanet/studio`
Expected: No errors

**Step 8.6: Verify visually**

Look at edges of 3D view.
Expected: Subtle darkening at screen edges (cinematic vignette).

**Step 8.7: Commit**

```bash
git add apps/studio/src/lib/graph3d/postProcessing.ts apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add vignette post-processing effect

- Custom vignette shader with offset/darkness control
- createEnhancedComposer combines bloom + vignette
- Cinematic edges at 40% darkness

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 9: Galaxy Boot Animation

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 9.1: Add boot phase state**

After the `highlightedLinks` state, add:

```tsx
const [bootPhase, setBootPhase] = useState<'loading' | 'spawning' | 'ready'>('loading');
```

**Step 9.2: Add boot animation effect**

After the D3 forces effect, add:

```tsx
// Galaxy boot animation - nodes spiral in from center
useEffect(() => {
  if (!isGraphReady || bootPhase !== 'loading' || graphData.nodes.length === 0) return;

  setBootPhase('spawning');

  // Animate nodes from center with stagger
  graphData.nodes.forEach((node, index) => {
    // Start all nodes at center
    node.fx = 0;
    node.fy = 0;
    node.fz = 0;

    // Release with stagger
    const delay = index * 30;  // 30ms stagger
    setTimeout(() => {
      node.fx = undefined;
      node.fy = undefined;
      node.fz = undefined;
    }, delay);
  });

  // Mark as ready after all nodes released
  const totalDelay = graphData.nodes.length * 30 + 1500;
  setTimeout(() => setBootPhase('ready'), totalDelay);
}, [isGraphReady, bootPhase, graphData.nodes]);
```

**Step 9.3: Verify visually**

Refresh page with 3D view active.
Expected: Nodes explode outward from center in sequence.

**Step 9.4: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add galaxy boot animation

- Nodes start at center (0,0,0)
- Staggered release every 30ms
- Spawning → ready phase transition

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 10: Selection Burst Effect

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`

**Step 10.1: Add burst state**

After `bootPhase` state, add:

```tsx
const [selectionBurst, setSelectionBurst] = useState<string | null>(null);
```

**Step 10.2: Update handleNodeClick to trigger burst**

Modify the `handleNodeClick` callback:

```tsx
const handleNodeClick = useCallback(
  (node: ForceGraphNode) => {
    setSelectedNode(node.id);
    zoomToNode(node);

    // Trigger selection burst effect
    setSelectionBurst(node.id);
    setTimeout(() => setSelectionBurst(null), 400);

    onNodeClick?.(node.id);
  },
  [setSelectedNode, zoomToNode, onNodeClick]
);
```

**Step 10.3: Add burst emissive in renderNode**

In the selection emissive section, update to:

```tsx
// Add emissive for selected/burst node (bloom effect)
if (material instanceof THREE.MeshStandardMaterial) {
  if (node.id === selectionBurst) {
    // Burst effect - extra bright
    material.emissive = new THREE.Color(layerColor);
    material.emissiveIntensity = 5.0;
    material.toneMapped = false;
  } else if (node.id === selectedNodeId) {
    // Selected - bright
    material.emissive = new THREE.Color(layerColor);
    material.emissiveIntensity = 2.0;
    material.toneMapped = false;
  }
}
```

**Step 10.4: Add selectionBurst to renderNode dependencies**

```tsx
}, [selectedNodeId, hoveredNodeId, selectionBurst, getNodeOpacity, getNodeScale]);
```

**Step 10.5: Verify visually**

Click a node in 3D view.
Expected: Brief bright flash (burst) on click, then settles to normal selection glow.

**Step 10.6: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx
git commit -m "feat(studio): add selection burst supernova effect

- 400ms emissive burst at 5.0 intensity on click
- Settles to 2.0 for sustained selection
- Visual feedback for user interaction

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 11: Final Type Check and Integration Test

**Files:**
- All modified files

**Step 11.1: Run full type check**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && pnpm type-check --filter=@novanet/studio`
Expected: No errors

**Step 11.2: Visual integration test**

Open http://localhost:3001
1. Switch to 3D view
2. Observe boot animation (nodes spiral out)
3. Click a node (burst effect + zoom)
4. Verify dimming (non-neighbors fade)
5. Verify arc highlighting
6. Rotate camera (constrained orbit)
7. Check vignette at edges

**Step 11.3: Final commit**

```bash
git add -A
git commit -m "feat(studio): complete Graph3D WOW enhancements

Phase 1: Visual (bloom, vignette)
Phase 2: Interactive (focus+context, arc highlights)
Phase 3: Performance (layer positioning)
Phase 4: Polish (boot animation, burst effect)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Summary

| Task | Feature | Estimated Time |
|------|---------|----------------|
| 1 | Bloom post-processing | 10 min |
| 2 | Focus+Context dimming | 15 min |
| 3 | Enhanced zoom animation | 5 min |
| 4 | Arc highlight on selection | 10 min |
| 5 | Constrained orbit camera | 5 min |
| 6 | Layer Z-positioning | 10 min |
| 7 | Enhanced emissive materials | 10 min |
| 8 | Vignette effect | 15 min |
| 9 | Galaxy boot animation | 10 min |
| 10 | Selection burst effect | 5 min |
| 11 | Integration test | 10 min |

**Total: ~105 minutes**

---

## Dependencies

All dependencies are already installed:
- `three` (Three.js)
- `react-force-graph-3d`

No new packages needed.
