# 3D Arc Signature Effects Design

**Date**: 2026-02-11
**Status**: Approved
**Author**: Thibaut + Claude

## Summary

Implement 5 signature visual effects for 3D arcs in NovaNet Studio, matching the 2D metaphors (PowerConduit, DNAHelix, SynapticFiring, MatrixRain, SonarPulse) with custom Three.js shaders and LOD-based performance optimization.

## Problem

Current 3D arc rendering has poor visibility:
- Links are hair-thin (0.2-0.35 width) and nearly transparent (15% opacity)
- Particles are small (4-7px) and sparse (4-5 per arc)
- No visual distinction between arc families (unlike 2D which has 5 signature effects)
- Energy flow effects lack quality and impact

## Solution

Replace the basic particle system with custom ShaderMaterial-based effects, one per arc family, with LOD management for performance.

## Architecture

```
apps/studio/src/lib/graph3d/
├── arcEffects/                      # NEW: 3D effects by family
│   ├── index.ts                     # Factory + exports
│   ├── types.ts                     # ArcEffect3D interface
│   ├── shaders/                     # GLSL shaders
│   │   ├── powerConduit.glsl        # ownership
│   │   ├── dnaHelix.glsl            # localization
│   │   ├── synapticFiring.glsl      # semantic
│   │   ├── matrixRain.glsl          # generation
│   │   └── sonarPulse.glsl          # mining
│   ├── PowerConduit3D.ts            # Shader + geometry setup
│   ├── DNAHelix3D.ts
│   ├── SynapticFiring3D.ts
│   ├── MatrixRain3D.ts
│   ├── SonarPulse3D.ts
│   └── SimpleLine3D.ts              # LOD fallback
├── ArcLODManager.ts                 # NEW: Distance-based LOD
└── Graph3D.tsx                      # Integration
```

## Signature Effects

### ownership - PowerConduit3D

- **Base**: TubeGeometry (radius 0.8) with glow shader
- **Effect**: 3 emissive spheres in convoy (regular spacing)
- **Animation**: Slow linear flow (8-10s cycle)
- **Shader**: Fresnel glow + subtle pulsation
- **Color**: #3b82f6 (blue-500)

### localization - DNAHelix3D

- **Base**: 2 particle spirals (InstancedMesh)
- **Effect**: Helical rotation around arc axis
- **Animation**: Slow rotation + traveling particles
- **Shader**: Luminous points with soft glow
- **Color**: #22c55e (green-500)

### semantic - SynapticFiring3D

- **Base**: Thin line + spark bursts
- **Effect**: Bright leading pulse + trail of 3-5 small sparks
- **Animation**: Fast and irregular (2-4s, random delay)
- **Shader**: Point sprites with fast fade
- **Color**: #f97316 (orange-500)

### generation - MatrixRain3D

- **Base**: Tube with scanline shader
- **Effect**: Descending light band + characters (or points)
- **Animation**: Fast cascade (1-2s) with parallax
- **Shader**: Scanline + noise for digital effect
- **Color**: #8b5cf6 (violet-500)

### mining - SonarPulse3D

- **Base**: Line + expanding rings
- **Effect**: Ping wave from source + concentric rings
- **Animation**: Periodic pulse (3-5s) with fade out
- **Shader**: Ring geometry with opacity decay
- **Color**: #ec4899 (pink-500)

## LOD System

### Distance Thresholds

| Distance | LOD Level | Rendering |
|----------|-----------|-----------|
| 0 - 150 | ULTRA | Full shader + particles + glow |
| 150 - 400 | HIGH | Simplified shader + particles |
| 400 - 800 | MEDIUM | TubeGeometry glow (no custom shader) |
| 800+ | LOW | Simple line with color |

### ArcLODManager

```typescript
class ArcLODManager {
  private arcs: Map<string, ArcLODState>;
  private effectPool: Map<ArcFamily, EffectPool>;

  // Called each frame in render loop
  update(camera: THREE.Camera) {
    for (const arc of this.arcs.values()) {
      const distance = camera.position.distanceTo(arc.midpoint);
      const newLOD = this.calculateLOD(distance);
      if (newLOD !== arc.currentLOD) {
        this.swapEffect(arc, newLOD);  // Hot-swap mesh
      }
    }
  }

  // Update shader uniforms (batched)
  updateUniforms(time: number) {
    // Single update per frame, not per arc
  }
}
```

### Optimizations

- **Object pooling**: Pre-create meshes, reuse on LOD swap
- **Shader uniforms batching**: 1 update per frame, not per arc
- **Frustum culling**: Skip arcs outside camera view
- **Hysteresis**: Avoid flip-flop at thresholds (±20 units buffer)

## Integration

### Graph3D.tsx Changes

```tsx
// Replace current particle system with custom rendering
<ForceGraph3D
  linkThreeObject={arcLODManager.createArcObject}
  linkThreeObjectExtend={false}
  linkPositionUpdate={arcLODManager.updatePosition}
  // Remove: linkDirectionalParticles, linkDirectionalParticleSpeed, etc.
/>

// In render loop (onEngineTick or useFrame)
arcLODManager.update(camera);  // LOD + animations
```

### Removed Code

- `linkDirectionalParticles={5}`
- `linkDirectionalParticleSpeed={0.004}`
- `linkDirectionalParticleWidth={4}`
- `linkDirectionalParticleColor={...}`
- `linkDirectionalParticleThreeObject={...}`

## Implementation Plan

### Phase 1: Infrastructure (2h)

1. Create `arcEffects/` directory structure
2. Define types & interfaces (`ArcEffect3D`, `LODLevel`)
3. Implement `ArcLODManager` base class
4. Create `SimpleLine3D` fallback

### Phase 2: Signature Effects (4-6h)

1. `PowerConduit3D` (ownership) - simplest, good baseline
2. `SynapticFiring3D` (semantic) - spark system
3. `DNAHelix3D` (localization) - helical particles
4. `MatrixRain3D` (generation) - scanline shader
5. `SonarPulse3D` (mining) - ring expansion

### Phase 3: Integration (2h)

1. Wire into `Graph3D.tsx`
2. Remove old particle system
3. Visual testing & parameter tuning

**Total estimated**: 8-10h

## Success Criteria

- [ ] All 5 arc families have distinct visual effects
- [ ] Effects are clearly visible at default zoom
- [ ] Smooth 60fps with 100+ arcs
- [ ] LOD transitions are not jarring
- [ ] Bloom post-processing enhances effects
- [ ] Colors match 2D signature effects

## References

- 2D signature effects: `apps/studio/src/components/graph/edges/effects/`
- Current 3D config: `apps/studio/src/lib/graph3d/arcParticles.ts`
- Three.js ShaderMaterial docs: https://threejs.org/docs/#api/en/materials/ShaderMaterial
