# Graph3D Atomic Redesign

**Date**: 2026-02-10
**Status**: Approved
**Style**: Atomic/Galaxy/Biological — Westworld-inspired

## Executive Summary

Complete redesign of 3D graph visualization with atomic-style nodes (core + orbital rings + particle clouds) and neural/fiber-optic arc effects. Includes 3D preview in detail sidebar.

---

## Problems Addressed

1. **Node shapes incoherent** — Too many random primitives, looks messy
2. **Arc particles invisible** — Energy transfer not visible (too big, too slow, too few)
3. **Materials look cheap** — Not premium, no sophistication
4. **No wow factor** — Missing atmospheric effects

---

## Design: Nodes

### Structure: Core + Orbital Rings + Particle Cloud

```
      ✦  ·    ·  ✦
    ·    ╭─────╮    ·    ← Particle cloud (stardust)
   ·   ╱ ╭───╮ ╲   ·
  ✦   │  │ ◉ │  │   ✦    ← Core: Layer geometry + color
   ·   ╲ ╰───╯ ╱   ·     ← Inner ring: always present
    ·    ╰─────╯    ·    ← Outer ring: org realm only
      ✦  ·    ·  ✦
```

### Encoding

| Element | Encodes | Values |
|---------|---------|--------|
| Core Shape | Layer | 9 simplified geometries |
| Core Color | Layer | Existing palette |
| Ring Count | Realm | shared=1, org=2 |
| Ring Speed | Trait | invariant=slow, generated=fast |
| Glow Intensity | Trait | knowledge=high, invariant=low |
| Particle Count | Connections | More connections = more particles |

### Core Shapes (Simplified)

| Layer | Geometry | Rationale |
|-------|----------|-----------|
| config | Octahedron | Crystal/settings |
| locale | Sphere | Globe |
| geography | Icosahedron | Geodesic |
| knowledge | Icosahedron | Complex knowledge |
| foundation | Box | Stable base |
| structure | Tetrahedron | Building block |
| semantic | Dodecahedron | Rich meaning |
| instruction | Cone | Directional |
| output | Sphere | Result/emission |

### Materials

```typescript
// Core
new THREE.MeshPhysicalMaterial({
  color: layerColor,
  metalness: 0.3,
  roughness: 0.2,
  emissive: layerColor,
  emissiveIntensity: 0.3,
});

// Rings
new THREE.MeshBasicMaterial({
  color: realmColor,
  transparent: true,
  opacity: 0.6,
  blending: THREE.AdditiveBlending,
});

// Particles
new THREE.PointsMaterial({
  size: 1.5,
  color: 0xffffff,
  transparent: true,
  opacity: 0.8,
  blending: THREE.AdditiveBlending,
});
```

### Ring Animation

```typescript
// Trait determines rotation speed
const TRAIT_RING_SPEEDS = {
  invariant: 0.2,    // Slow, stable
  localized: 0.4,    // Medium
  knowledge: 0.3,    // Gentle
  generated: 0.8,    // Fast, active
  aggregated: 0.5,   // Medium-fast
};
```

---

## Design: Arcs (Energy Flow)

### Style: Neural + Fiber Optic + Electric

```
◉━━━━•━━•━━•━━•━━•━━•━━•━━•━━•━━•━━•━━━━━━━━━━━━━━━━━━━━━━◉
     ↑
Micro-particles rapides, flux continu visible
```

### Particle Configuration (NEW vs OLD)

| Property | OLD | NEW | Reason |
|----------|-----|-----|--------|
| Size | 15-24px | 2-4px | Subtle, not chunky |
| Count | 5-12 | 30-50 | Continuous stream |
| Speed | 0.003-0.012 | 0.02-0.04 | Fast, visible flow |
| Trail | None | Motion blur | Energy feel |

### Arc Family Styles

| Family | Color | Behavior | Curvature |
|--------|-------|----------|-----------|
| ownership | Blue #60a5fa | Regular flow | 0 (straight) |
| localization | Green #4ade80 | Undulating | 0.2 (gentle curve) |
| semantic | Orange #fb923c | Random sparks | 0.1 (slight) |
| generation | Violet #a78bfa | Fast cascade | 0 (straight) |
| mining | Pink #f472b6 | Radar pulse | 0.3 (curved) |

### Line Base

```typescript
{
  width: 1.5,           // Thin, elegant
  opacity: 0.3,         // Base opacity
  opacityActive: 0.6,   // When hovered/selected
  glow: true,           // Subtle halo
  breathing: true,      // Sinusoidal opacity variation
  breathingSpeed: 0.5,  // Slow, organic
}
```

---

## Design: Sidebar 3D Preview

### Layout

```
┌─────────────────────────────────────────┐
│  ┌─────────────────────────────────┐    │
│  │                                 │    │
│  │      ✦  ·    ·  ✦              │    │  ← Three.js canvas 200x200
│  │    ·   ╭───╮   ·               │    │
│  │   ✦    │ ◉ │    ✦              │    │  ← Slow auto-rotate
│  │    ·   ╰───╯   ·               │    │
│  │      ✦  ·    ·  ✦              │    │  ← All effects active
│  │                                 │    │
│  └─────────────────────────────────┘    │
│                                         │
│  Entity                                 │
│  ─────────────────────                  │
│  Layer: semantic  Realm: org            │
│  Trait: invariant                       │
│  ...                                    │
└─────────────────────────────────────────┘
```

### Features

- **Auto-rotate**: 0.5 rad/s, pauses on hover
- **Full effects**: Rings, particles, glow all active
- **Background**: Dark gradient or transparent
- **For Arcs**: Shows 2 mini-nodes + animated arc between them
- **Isolation**: Separate Three.js scene, doesn't affect main graph

### Component Structure

```typescript
// New component: NodePreview3D
interface NodePreview3DProps {
  node: GraphNode;
  size?: number;        // Default 200
  autoRotate?: boolean; // Default true
  showEffects?: boolean; // Default true
}

// New component: ArcPreview3D
interface ArcPreview3DProps {
  arc: GraphEdge;
  size?: number;
  showFlow?: boolean;
}
```

---

## Design: Global Effects

### Post-Processing Pipeline

```typescript
const composer = new EffectComposer(renderer);

// 1. Render pass
composer.addPass(new RenderPass(scene, camera));

// 2. Bloom (atmospheric glow)
composer.addPass(new UnrealBloomPass(
  resolution,
  strength: 1.2,    // Strong but not overwhelming
  radius: 0.5,      // Medium spread
  threshold: 0.4    // Catch emissive materials
));

// 3. Vignette (cinematic edges)
composer.addPass(new ShaderPass(VignetteShader, {
  offset: 0.5,
  darkness: 0.4
}));

// 4. Color grading (space feel)
// Subtle blue/violet shift for cosmic atmosphere
```

### Pulse Animation (Heartbeat)

```typescript
// All nodes pulse subtly
const pulseAnimation = (node, time) => {
  const pulse = 1 + Math.sin(time * 2) * 0.05; // ±5% scale
  node.scale.setScalar(baseScale * pulse);

  // Glow intensity also pulses
  node.material.emissiveIntensity = baseIntensity * (0.8 + pulse * 0.4);
};
```

---

## Implementation Files

| File | Changes |
|------|---------|
| `lib/graph3d/geometryFactory.ts` | Add ring + particle geometry creators |
| `lib/graph3d/arcParticles.ts` | New particle config (smaller, faster, more) |
| `lib/graph3d/nodeComposite.ts` | NEW: Composite node builder (core + rings + particles) |
| `lib/graph3d/postProcessing.ts` | Update bloom settings, add color grading |
| `components/graph/Graph3D.tsx` | Use new composite nodes, new arc config |
| `components/graph/NodePreview3D.tsx` | NEW: Sidebar 3D preview component |
| `components/graph/ArcPreview3D.tsx` | NEW: Arc preview with mini-nodes |
| `components/sidebar/TabbedDetailPanel.tsx` | Integrate NodePreview3D |
| `components/sidebar/TabbedArcPanel.tsx` | Integrate ArcPreview3D |

---

## Success Metrics

- [ ] Nodes visually encode Realm (rings), Layer (shape+color), Trait (animation)
- [ ] Arc energy flow clearly visible (stream of particles)
- [ ] Premium feel (glow, materials, post-processing)
- [ ] Sidebar preview shows interactive 3D with all effects
- [ ] 60fps with 100 nodes
- [ ] No "messy" or "cheap" appearance

---

## References

- Westworld host visualization
- Atom/molecular diagrams
- Galaxy/nebula imagery
- CERN particle physics visualizations
