# 3D Arc Signature Effects Enhancement - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance existing 3D arc effects with increased tube opacity (+35%), universal Double Helix motion, and family-specific dust particles that travel along edges.

**Architecture:** Layer enhancements onto existing ArcEffect3D system. Add helix utilities to shaderUtils.ts, create DustParticleSystem class where particles flow from source to target with perpendicular drift. Update each effect to use helix + dust. Preserve existing LOD system.

**Tech Stack:** Three.js ShaderMaterial, GLSL shaders, InstancedMesh for dust particles, existing ArcLODManager integration.

---

## Prerequisites

- Neo4j running (`pnpm infra:up`)
- Studio dev server (`pnpm dev` in apps/studio)
- Existing arc effects working (verify at http://localhost:3000)

---

## Task 1: Increase Tube Opacity (+35%)

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcParticles.ts`

**Step 1: Update opacity values**

Open `apps/studio/src/lib/graph3d/arcParticles.ts` and update `linkOpacity` for all families:

```typescript
// BEFORE
ownership: { linkOpacity: 0.15, ... }
localization: { linkOpacity: 0.15, ... }
semantic: { linkOpacity: 0.15, ... }
generation: { linkOpacity: 0.20, ... }
mining: { linkOpacity: 0.15, ... }

// AFTER (+35% increase)
ownership: { linkOpacity: 0.20, ... }
localization: { linkOpacity: 0.20, ... }
semantic: { linkOpacity: 0.20, ... }
generation: { linkOpacity: 0.27, ... }
mining: { linkOpacity: 0.20, ... }
```

**Step 2: Verify visually**

1. Open Studio at http://localhost:3000
2. Load a graph with arcs
3. Toggle to 3D view
4. Verify tubes are more visible but particles still prominent

**Step 3: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcParticles.ts
git commit -m "feat(studio): increase 3D arc tube opacity by 35%

- ownership/localization/semantic/mining: 0.15 → 0.20
- generation: 0.20 → 0.27
- Improves tube visibility while preserving particle prominence

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Create Helix Utility Functions

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/shaderUtils.ts`

**Step 1: Add helix GLSL functions**

Add to the `COMMON_GLSL` object in `shaderUtils.ts`:

```typescript
export const COMMON_GLSL = {
  // ... existing functions (fresnel, pulse, flow, noise, hash)

  // NEW: Double helix utilities
  helix: `
    // Calculate helix offset perpendicular to path direction
    vec3 helixOffset(float t, float amplitude, float frequency, float phase) {
      float angle = t * frequency * 6.28318 + phase;
      // Perpendicular offset in local space (will be transformed by view matrix)
      return vec3(cos(angle) * amplitude, sin(angle) * amplitude, 0.0);
    }

    // Dual helix for two intertwined spirals
    vec2 dualHelixPhases(float t, float frequency) {
      float base = t * frequency * 6.28318;
      return vec2(base, base + 3.14159); // 180° phase offset
    }
  `,
};
```

**Step 2: Add TypeScript helix helper**

Add below the GLSL constant:

```typescript
/**
 * Calculate helix position offset at parameter t
 * @param t - Position along arc (0-1)
 * @param amplitude - Distance from center
 * @param frequency - Rotations per arc length
 * @param phase - Phase offset (0 or PI for dual helix)
 * @param time - Animation time for rotation
 */
export function calculateHelixOffset(
  t: number,
  amplitude: number,
  frequency: number,
  phase: number,
  time: number
): THREE.Vector3 {
  const angle = t * frequency * Math.PI * 2 + phase + time * 0.5;
  return new THREE.Vector3(
    Math.cos(angle) * amplitude,
    Math.sin(angle) * amplitude,
    0
  );
}
```

**Step 3: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/shaderUtils.ts
git commit -m "feat(studio): add helix GLSL and TypeScript utilities

- helixOffset(): single helix perpendicular offset
- dualHelixPhases(): dual helix with 180° phase
- calculateHelixOffset(): TypeScript helper for CPU calculations

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Create Dust Particle Types and Config

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts`

**Step 1: Create the dust particle configuration file**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts
import * as THREE from 'three';
import type { ArcFamily } from '@/types';

/**
 * Dust particle shapes per arc family
 * Aligned with 2D visual metaphors from visual-encoding.yaml
 */
export type DustShape = 'diamond' | 'circle' | 'star' | 'triangle' | 'hexagon';

export interface DustConfig {
  shape: DustShape;
  count: number;        // particles per arc
  size: number;         // base size in world units
  drift: 'gravitational' | 'orbital' | 'sparkling' | 'cascade' | 'pulsing';
  opacity: number;      // 0-1
  color: string;        // hex color (from arc family)
}

/**
 * Dust configuration per arc family
 * Shapes match 2D effect metaphors:
 * - ownership (power_conduit) → diamonds (energy crystals)
 * - localization (dna_helix) → circles (base pairs)
 * - semantic (synaptic_firing) → stars (sparks)
 * - generation (matrix_code_rain) → triangles (data fragments)
 * - mining (sonar_pulse) → hexagons (data crystals)
 */
export const DUST_CONFIG: Record<ArcFamily, DustConfig> = {
  ownership: {
    shape: 'diamond',
    count: 8,
    size: 0.15,
    drift: 'gravitational',
    opacity: 0.6,
    color: '#3b82f6',
  },
  localization: {
    shape: 'circle',
    count: 10,
    size: 0.12,
    drift: 'orbital',
    opacity: 0.5,
    color: '#22c55e',
  },
  semantic: {
    shape: 'star',
    count: 12,
    size: 0.1,
    drift: 'sparkling',
    opacity: 0.7,
    color: '#f97316',
  },
  generation: {
    shape: 'triangle',
    count: 15,
    size: 0.08,
    drift: 'cascade',
    opacity: 0.6,
    color: '#8b5cf6',
  },
  mining: {
    shape: 'hexagon',
    count: 6,
    size: 0.18,
    drift: 'pulsing',
    opacity: 0.5,
    color: '#ec4899',
  },
};
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts
git commit -m "feat(studio): add dust particle types and configuration

- DustShape type: diamond, circle, star, triangle, hexagon
- DustConfig interface with shape, count, size, drift, opacity
- DUST_CONFIG mapping per arc family
- Shapes aligned with 2D visual metaphors

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Create Dust Particle Geometries

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts`

**Step 1: Add geometry factory functions**

Add to `dustParticles.ts`:

```typescript
/**
 * Create geometry for each dust shape
 * All shapes are flat (2D) and will be billboarded toward camera
 */
export function createDustGeometry(shape: DustShape): THREE.BufferGeometry {
  switch (shape) {
    case 'diamond':
      return createDiamondGeometry();
    case 'circle':
      return createCircleGeometry();
    case 'star':
      return createStarGeometry();
    case 'triangle':
      return createTriangleGeometry();
    case 'hexagon':
      return createHexagonGeometry();
    default:
      return createCircleGeometry();
  }
}

function createDiamondGeometry(): THREE.BufferGeometry {
  const shape = new THREE.Shape();
  shape.moveTo(0, 0.5);
  shape.lineTo(0.4, 0);
  shape.lineTo(0, -0.5);
  shape.lineTo(-0.4, 0);
  shape.closePath();
  return new THREE.ShapeGeometry(shape);
}

function createCircleGeometry(): THREE.BufferGeometry {
  return new THREE.CircleGeometry(0.5, 16);
}

function createStarGeometry(): THREE.BufferGeometry {
  const shape = new THREE.Shape();
  const points = 5;
  const outerRadius = 0.5;
  const innerRadius = 0.2;

  for (let i = 0; i < points * 2; i++) {
    const radius = i % 2 === 0 ? outerRadius : innerRadius;
    const angle = (i * Math.PI) / points - Math.PI / 2;
    const x = Math.cos(angle) * radius;
    const y = Math.sin(angle) * radius;
    if (i === 0) shape.moveTo(x, y);
    else shape.lineTo(x, y);
  }
  shape.closePath();
  return new THREE.ShapeGeometry(shape);
}

function createTriangleGeometry(): THREE.BufferGeometry {
  const shape = new THREE.Shape();
  shape.moveTo(0, 0.5);
  shape.lineTo(0.433, -0.25);
  shape.lineTo(-0.433, -0.25);
  shape.closePath();
  return new THREE.ShapeGeometry(shape);
}

function createHexagonGeometry(): THREE.BufferGeometry {
  const shape = new THREE.Shape();
  for (let i = 0; i < 6; i++) {
    const angle = (i * Math.PI) / 3;
    const x = Math.cos(angle) * 0.5;
    const y = Math.sin(angle) * 0.5;
    if (i === 0) shape.moveTo(x, y);
    else shape.lineTo(x, y);
  }
  shape.closePath();
  return new THREE.ShapeGeometry(shape);
}
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts
git commit -m "feat(studio): add dust particle geometry factories

- createDustGeometry() factory with 5 shapes
- Diamond: 4-point rhombus for ownership
- Circle: 16-segment for localization
- Star: 5-point for semantic
- Triangle: equilateral for generation
- Hexagon: 6-point for mining

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Create DustParticleSystem Class

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts`

**Step 1: Add the particle system class**

Add to `dustParticles.ts`:

```typescript
/**
 * DustParticleSystem - Manages dust particles that TRAVEL along an arc
 * Particles flow from source to target while applying perpendicular drift
 * Uses InstancedMesh for performance
 */
export class DustParticleSystem {
  public mesh: THREE.InstancedMesh;
  private config: DustConfig;
  private progress: number[];      // 0-1 position along arc
  private phases: number[];        // Random phase for drift variation
  private speeds: number[];        // Individual particle speeds
  private dummy: THREE.Object3D;
  private sourcePos: THREE.Vector3;
  private targetPos: THREE.Vector3;

  constructor(config: DustConfig) {
    this.config = config;
    this.dummy = new THREE.Object3D();
    this.sourcePos = new THREE.Vector3();
    this.targetPos = new THREE.Vector3();

    // Create geometry and material
    const geometry = createDustGeometry(config.shape);
    const material = new THREE.MeshBasicMaterial({
      color: config.color,
      transparent: true,
      opacity: config.opacity,
      side: THREE.DoubleSide,
      depthWrite: false,
      blending: THREE.AdditiveBlending,
    });

    // Create instanced mesh
    this.mesh = new THREE.InstancedMesh(geometry, material, config.count);
    this.mesh.frustumCulled = false;

    // Initialize particle state - staggered along arc
    this.progress = [];
    this.phases = [];
    this.speeds = [];

    for (let i = 0; i < config.count; i++) {
      // Stagger initial positions evenly along arc
      this.progress.push(i / config.count);
      this.phases.push(Math.random() * Math.PI * 2);
      // Vary speed slightly per particle (0.8-1.2x base speed)
      this.speeds.push(0.8 + Math.random() * 0.4);
    }
  }

  /**
   * Update arc endpoints (called when nodes move)
   */
  updateArcPositions(source: THREE.Vector3, target: THREE.Vector3): void {
    this.sourcePos.copy(source);
    this.targetPos.copy(target);
  }

  /**
   * Animate particles - they TRAVEL along the edge with perpendicular drift
   */
  update(time: number, deltaTime: number): void {
    // Base travel speed (complete arc in ~4-8 seconds depending on drift type)
    const baseSpeed = this.config.drift === 'cascade' ? 0.25 :
                      this.config.drift === 'sparkling' ? 0.15 :
                      this.config.drift === 'pulsing' ? 0.1 :
                      0.12; // gravitational, orbital

    for (let i = 0; i < this.config.count; i++) {
      const phase = this.phases[i];
      const speed = this.speeds[i];

      // Advance position along arc (loop back to start)
      this.progress[i] += deltaTime * baseSpeed * speed;
      if (this.progress[i] > 1) {
        this.progress[i] -= 1; // Loop back to source
      }

      const t = this.progress[i];

      // Base position along arc
      const basePos = new THREE.Vector3().lerpVectors(
        this.sourcePos,
        this.targetPos,
        t
      );

      // Calculate perpendicular offset based on drift type
      let perpOffset = new THREE.Vector3();
      const perpDist = 0.25 + Math.sin(phase) * 0.15; // 0.1-0.4 units from tube

      switch (this.config.drift) {
        case 'gravitational':
          // Slow sway while traveling
          perpOffset.set(
            Math.sin(time * 0.5 + phase) * perpDist,
            Math.cos(time * 0.3 + phase) * perpDist * 0.5 - perpDist * 0.3,
            0
          );
          break;

        case 'orbital':
          // Spiral around tube while traveling
          const orbitAngle = time * 2 + phase + t * Math.PI * 4;
          perpOffset.set(
            Math.cos(orbitAngle) * perpDist,
            Math.sin(orbitAngle) * perpDist,
            0
          );
          break;

        case 'sparkling':
          // Erratic movement while traveling
          perpOffset.set(
            Math.sin(time * 5 + phase * 10) * perpDist,
            Math.cos(time * 7 + phase * 8) * perpDist,
            Math.sin(time * 3 + phase * 6) * perpDist * 0.5
          );
          break;

        case 'cascade':
          // Zigzag descent pattern while traveling fast
          perpOffset.set(
            Math.sin(t * Math.PI * 6 + phase) * perpDist,
            -Math.abs(Math.sin(t * Math.PI * 3)) * perpDist,
            0
          );
          break;

        case 'pulsing':
          // Expand/contract while traveling slowly
          const pulseScale = 1 + Math.sin(time * 2 + phase) * 0.5;
          perpOffset.set(
            Math.cos(phase) * perpDist * pulseScale,
            Math.sin(phase) * perpDist * pulseScale,
            0
          );
          break;
      }

      // Final position = base + perpendicular offset
      const finalPos = basePos.add(perpOffset);

      // Update instance matrix
      this.dummy.position.copy(finalPos);
      this.dummy.scale.setScalar(this.config.size);
      this.dummy.lookAt(finalPos.clone().add(new THREE.Vector3(0, 0, 1))); // Billboard
      this.dummy.updateMatrix();
      this.mesh.setMatrixAt(i, this.dummy.matrix);
    }

    this.mesh.instanceMatrix.needsUpdate = true;
  }

  dispose(): void {
    this.mesh.geometry.dispose();
    if (this.mesh.material instanceof THREE.Material) {
      this.mesh.material.dispose();
    }
  }
}
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts
git commit -m "feat(studio): add DustParticleSystem class

- InstancedMesh for performance (one draw call)
- 5 drift behaviors: gravitational, orbital, sparkling, cascade, pulsing
- Billboard particles face camera
- Additive blending for glow effect
- Dispose method for cleanup

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Export Dust Particle Module

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/index.ts`

**Step 1: Add exports**

Add to `index.ts`:

```typescript
// Dust particle system
export {
  DustParticleSystem,
  DUST_CONFIG,
  createDustGeometry,
  type DustConfig,
  type DustShape,
} from './dustParticles';
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/index.ts
git commit -m "feat(studio): export dust particle system from arcEffects

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Integrate Dust into PowerConduit3D

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/PowerConduit3D.ts`

**Step 1: Add dust particle system**

Update `PowerConduit3D.ts`:

```typescript
import { DustParticleSystem, DUST_CONFIG } from './dustParticles';

export class PowerConduit3D implements ArcEffect3D {
  // ... existing properties
  private dustSystem: DustParticleSystem;

  constructor(config: ArcEffectConfig) {
    // ... existing initialization

    // Add dust particle system
    this.dustSystem = new DustParticleSystem(DUST_CONFIG.ownership);
    this.dustSystem.updateArcPositions(this.sourcePos, this.targetPos);
    this.group.add(this.dustSystem.mesh);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    // ... existing position update

    // Update dust positions
    this.dustSystem.updateArcPositions(source, target);
  }

  updateUniforms(time: number, deltaTime: number): void {
    // ... existing uniform update

    // Animate dust
    this.dustSystem.update(time, deltaTime);
  }

  dispose(): void {
    // ... existing disposal
    this.dustSystem.dispose();
  }
}
```

**Step 2: Verify visually**

1. Open Studio, load graph with ownership arcs (HAS_PAGE, HAS_BLOCK)
2. Toggle 3D view
3. Verify diamond-shaped dust particles floating around blue tubes

**Step 3: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/PowerConduit3D.ts
git commit -m "feat(studio): add dust particles to PowerConduit3D (ownership)

- Diamond shapes with gravitational drift
- Integrated with existing tube effect
- Updates on position change and animation frame

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 8: Integrate Dust into DNAHelix3D

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/DNAHelix3D.ts`

**Step 1: Add dust particle system**

Same pattern as Task 7, but with localization config:

```typescript
import { DustParticleSystem, DUST_CONFIG } from './dustParticles';

// In constructor:
this.dustSystem = new DustParticleSystem(DUST_CONFIG.localization);
this.dustSystem.updateArcPositions(this.sourcePos, this.targetPos);
this.group.add(this.dustSystem.mesh);

// In updatePositions:
this.dustSystem.updateArcPositions(source, target);

// In updateUniforms:
this.dustSystem.update(time, deltaTime);

// In dispose:
this.dustSystem.dispose();
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/DNAHelix3D.ts
git commit -m "feat(studio): add dust particles to DNAHelix3D (localization)

- Circle shapes with orbital drift
- Complements helix strands effect

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 9: Integrate Dust into SynapticFiring3D

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/SynapticFiring3D.ts`

**Step 1: Add dust particle system**

Same pattern, with semantic config:

```typescript
this.dustSystem = new DustParticleSystem(DUST_CONFIG.semantic);
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/SynapticFiring3D.ts
git commit -m "feat(studio): add dust particles to SynapticFiring3D (semantic)

- Star shapes with sparkling drift
- Enhances neural firing visual

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 10: Integrate Dust into MatrixRain3D

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/MatrixRain3D.ts`

**Step 1: Add dust particle system**

Same pattern, with generation config:

```typescript
this.dustSystem = new DustParticleSystem(DUST_CONFIG.generation);
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/MatrixRain3D.ts
git commit -m "feat(studio): add dust particles to MatrixRain3D (generation)

- Triangle shapes with cascade drift
- Enhances matrix rain effect

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 11: Integrate Dust into SonarPulse3D

**Files:**
- Modify: `apps/studio/src/lib/graph3d/arcEffects/SonarPulse3D.ts`

**Step 1: Add dust particle system**

Same pattern, with mining config:

```typescript
this.dustSystem = new DustParticleSystem(DUST_CONFIG.mining);
```

**Step 2: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/SonarPulse3D.ts
git commit -m "feat(studio): add dust particles to SonarPulse3D (mining)

- Hexagon shapes with pulsing drift
- Enhances radar ping visual

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 12: Add Helix Motion to All Effects

**Files:**
- Modify: All 5 effect files

**Step 1: Update each effect's vertex shader**

Add helix motion to particle positions in each effect. Example for PowerConduit3D:

```glsl
// In vertex shader
uniform float time;
uniform float helixAmplitude;
uniform float helixFrequency;

void main() {
  vec3 pos = position;

  // Add helix motion
  float t = uv.x; // Position along arc (0-1)
  float helixAngle = t * helixFrequency * 6.28318 + time * 0.5;
  pos.x += cos(helixAngle) * helixAmplitude;
  pos.y += sin(helixAngle) * helixAmplitude;

  // ... rest of vertex shader
}
```

**Step 2: Add helix uniforms**

In each effect constructor:

```typescript
// Add to uniforms
helixAmplitude: { value: 0.15 },  // Subtle helix
helixFrequency: { value: 2.0 },   // 2 rotations per arc
```

**Step 3: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/*.ts
git commit -m "feat(studio): add helix motion to all 3D arc effects

- Subtle helix amplitude (0.15) for tube rotation
- 2 rotations per arc length
- Animated via time uniform

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 13: Update visual-encoding.yaml

**Files:**
- Modify: `packages/core/models/visual-encoding.yaml`

**Step 1: Add 3D effect configuration section**

Add after `arc_animation_effects`:

```yaml
# =============================================================================
# ARC 3D EFFECTS (Studio 3D view) — v11.6.3
# =============================================================================
# 3D enhancements layered on top of base arc effects
# Implementation: apps/studio/src/lib/graph3d/arcEffects/

arc_3d_effects:
  common:
    tube_opacity: 0.20          # +35% from 0.15 baseline
    helix_amplitude: 0.15       # Subtle perpendicular motion
    helix_frequency: 2.0        # Rotations per arc length

  dust_particles:
    ownership:
      shape: diamond
      count: 8
      drift: gravitational
      description: "Energy crystals drifting down around power conduit"

    localization:
      shape: circle
      count: 10
      drift: orbital
      description: "Base pairs orbiting around DNA helix"

    semantic:
      shape: star
      count: 12
      drift: sparkling
      description: "Sparks flickering around synaptic connection"

    generation:
      shape: triangle
      count: 15
      drift: cascade
      description: "Data fragments cascading through matrix stream"

    mining:
      shape: hexagon
      count: 6
      drift: pulsing
      description: "Data crystals pulsing around sonar beam"
```

**Step 2: Commit**

```bash
git add packages/core/models/visual-encoding.yaml
git commit -m "docs(core): add 3D arc effect configuration to visual-encoding.yaml

- Common settings: tube_opacity, helix parameters
- Dust particle config per family with shapes and drift
- Documents v11.6.3 3D enhancements

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 14: Visual QA and Performance Test

**Files:** None (manual testing)

**Step 1: Visual verification checklist**

Open Studio at http://localhost:3000 and verify:

- [ ] Tubes are visibly more opaque (but particles still prominent)
- [ ] All 5 arc types have correct dust shapes:
  - [ ] Ownership (blue): Diamond particles, gravitational drift
  - [ ] Localization (green): Circle particles, orbital drift
  - [ ] Semantic (orange): Star particles, sparkling drift
  - [ ] Generation (purple): Triangle particles, cascade drift
  - [ ] Mining (pink): Hexagon particles, pulsing drift
- [ ] Helix motion visible on tube rotation
- [ ] LOD transitions smooth (zoom in/out)
- [ ] No console errors

**Step 2: Performance verification**

1. Load graph with 100+ arcs
2. Open browser DevTools → Performance tab
3. Record 10 seconds of animation
4. Verify: Frame rate ≥ 55 FPS average

**Step 3: Document results**

If issues found, create tasks to fix them before final commit.

---

## Task 15: Final Integration Commit

**Step 1: Verify all tests pass**

```bash
cd apps/studio
pnpm type-check
pnpm lint
```

**Step 2: Final commit**

```bash
git add -A
git commit -m "feat(studio): complete 3D arc signature effects enhancement

Summary:
- Tube opacity increased by 35% (0.15 → 0.20)
- Double helix motion added to all arc effects
- Dust particle system traveling along edges with 5 family-specific shapes:
  - ownership: diamond (gravitational drift while flowing)
  - localization: circle (orbital spiral while flowing)
  - semantic: star (sparkling while flowing)
  - generation: triangle (zigzag cascade while flowing fast)
  - mining: hexagon (pulsing while flowing slow)

Closes #XXX

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Summary

| Task | Description | Est. Time |
|------|-------------|-----------|
| 1 | Increase tube opacity | 5 min |
| 2 | Create helix utilities | 10 min |
| 3 | Create dust types/config | 10 min |
| 4 | Create dust geometries | 15 min |
| 5 | Create DustParticleSystem | 20 min |
| 6 | Export dust module | 2 min |
| 7-11 | Integrate dust (5 effects) | 25 min |
| 12 | Add helix motion | 15 min |
| 13 | Update visual-encoding.yaml | 5 min |
| 14 | Visual QA | 10 min |
| 15 | Final commit | 5 min |

**Total: ~2 hours**

---

## References

- Design doc: `docs/plans/2026-02-11-3d-arc-signature-effects-design.md`
- 2D effects: `apps/studio/src/components/graph/edges/effects/`
- 3D effects: `apps/studio/src/lib/graph3d/arcEffects/`
- Visual encoding: `packages/core/models/visual-encoding.yaml`
- Taxonomy colors: `packages/core/models/taxonomy.yaml`
