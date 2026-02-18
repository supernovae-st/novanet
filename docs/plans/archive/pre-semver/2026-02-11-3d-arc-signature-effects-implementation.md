# 3D Arc Signature Effects Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement 5 signature visual effects for 3D arcs with custom shaders and LOD-based performance optimization.

**Architecture:** Custom ShaderMaterial per arc family, managed by ArcLODManager that swaps effect quality based on camera distance. Effects replace the current react-force-graph-3d particle system.

**Tech Stack:** Three.js (ShaderMaterial, TubeGeometry, InstancedMesh), TypeScript, react-force-graph-3d linkThreeObject API

---

## Task 1: Create Arc Effects Directory Structure

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/index.ts`
- Create: `apps/studio/src/lib/graph3d/arcEffects/types.ts`

**Step 1: Create types file**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/types.ts
import * as THREE from 'three';

export type ArcFamily = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';

export type LODLevel = 'ultra' | 'high' | 'medium' | 'low';

export interface ArcEffectConfig {
  sourcePosition: THREE.Vector3;
  targetPosition: THREE.Vector3;
  family: ArcFamily;
  color: string;
  isSelected?: boolean;
  isHovered?: boolean;
}

export interface ArcEffect3D {
  /** The THREE.Group containing all effect meshes */
  group: THREE.Group;
  /** Current LOD level */
  lodLevel: LODLevel;
  /** Update positions when nodes move */
  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void;
  /** Update shader uniforms (call each frame) */
  updateUniforms(time: number, deltaTime: number): void;
  /** Switch to different LOD level */
  setLOD(level: LODLevel): void;
  /** Clean up resources */
  dispose(): void;
}

export interface ArcEffectFactory {
  create(config: ArcEffectConfig): ArcEffect3D;
}

export const LOD_THRESHOLDS = {
  ultra: 150,   // 0-150: full shader + particles + glow
  high: 400,    // 150-400: simplified shader + particles
  medium: 800,  // 400-800: tube glow only
  // 800+: low (simple line)
} as const;

export const ARC_FAMILY_COLORS: Record<ArcFamily, string> = {
  ownership: '#3b82f6',     // blue-500
  localization: '#22c55e',  // green-500
  semantic: '#f97316',      // orange-500
  generation: '#8b5cf6',    // violet-500
  mining: '#ec4899',        // pink-500
};
```

**Step 2: Create index file**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts
export * from './types';
```

**Step 3: Verify files exist**

Run: `ls -la apps/studio/src/lib/graph3d/arcEffects/`
Expected: `types.ts` and `index.ts` listed

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add arc effects types and directory structure"
```

---

## Task 2: Implement SimpleLine3D (LOD Fallback)

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/SimpleLine3D.ts`
- Modify: `apps/studio/src/lib/graph3d/arcEffects/index.ts`

**Step 1: Create SimpleLine3D**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/SimpleLine3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';

/**
 * SimpleLine3D - Minimal fallback for distant arcs (LOD: low)
 * Just a colored line, no effects, minimal GPU cost
 */
export class SimpleLine3D implements ArcEffect3D {
  public group: THREE.Group;
  public lodLevel: LODLevel = 'low';

  private line: THREE.Line;
  private geometry: THREE.BufferGeometry;
  private material: THREE.LineBasicMaterial;

  constructor(config: ArcEffectConfig) {
    this.group = new THREE.Group();

    // Create geometry with 2 points
    this.geometry = new THREE.BufferGeometry();
    const positions = new Float32Array(6); // 2 vertices * 3 coords
    this.geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));

    // Create material
    const color = new THREE.Color(config.color);
    this.material = new THREE.LineBasicMaterial({
      color,
      transparent: true,
      opacity: 0.6,
      linewidth: 1, // Note: linewidth > 1 only works on some platforms
    });

    // Create line
    this.line = new THREE.Line(this.geometry, this.material);
    this.group.add(this.line);

    // Set initial positions
    this.updatePositions(config.sourcePosition, config.targetPosition);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    const positions = this.geometry.attributes.position.array as Float32Array;
    positions[0] = source.x;
    positions[1] = source.y;
    positions[2] = source.z;
    positions[3] = target.x;
    positions[4] = target.y;
    positions[5] = target.z;
    this.geometry.attributes.position.needsUpdate = true;
  }

  updateUniforms(_time: number, _deltaTime: number): void {
    // No uniforms for simple line
  }

  setLOD(level: LODLevel): void {
    this.lodLevel = level;
    // Adjust opacity based on LOD
    this.material.opacity = level === 'low' ? 0.4 : 0.6;
  }

  dispose(): void {
    this.geometry.dispose();
    this.material.dispose();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts
export * from './types';
export * from './SimpleLine3D';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add SimpleLine3D fallback for distant arcs"
```

---

## Task 3: Implement Base Shader Utilities

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/shaderUtils.ts`

**Step 1: Create shader utilities**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/shaderUtils.ts
import * as THREE from 'three';

/**
 * Common GLSL functions shared across arc shaders
 */
export const COMMON_GLSL = {
  // Fresnel effect for edge glow
  fresnel: `
    float fresnel(vec3 viewDir, vec3 normal, float power) {
      return pow(1.0 - abs(dot(viewDir, normal)), power);
    }
  `,

  // Smooth pulse function
  pulse: `
    float pulse(float time, float speed, float minVal, float maxVal) {
      return minVal + (maxVal - minVal) * (0.5 + 0.5 * sin(time * speed));
    }
  `,

  // Flow along UV.x (0 to 1)
  flow: `
    float flow(float uv, float time, float speed) {
      return fract(uv - time * speed);
    }
  `,

  // Noise function (simple)
  noise: `
    float hash(float n) { return fract(sin(n) * 43758.5453123); }
    float noise(float x) {
      float i = floor(x);
      float f = fract(x);
      return mix(hash(i), hash(i + 1.0), smoothstep(0.0, 1.0, f));
    }
  `,
};

/**
 * Create a TubeGeometry following a straight path between two points
 */
export function createArcTubeGeometry(
  source: THREE.Vector3,
  target: THREE.Vector3,
  radius: number = 0.5,
  tubularSegments: number = 32,
  radialSegments: number = 8
): THREE.TubeGeometry {
  const path = new THREE.LineCurve3(source, target);
  return new THREE.TubeGeometry(path, tubularSegments, radius, radialSegments, false);
}

/**
 * Create a curved arc path with optional curvature
 */
export function createCurvedArcPath(
  source: THREE.Vector3,
  target: THREE.Vector3,
  curvature: number = 0.2
): THREE.CatmullRomCurve3 {
  const midpoint = new THREE.Vector3()
    .addVectors(source, target)
    .multiplyScalar(0.5);

  // Offset midpoint perpendicular to the line
  const direction = new THREE.Vector3().subVectors(target, source);
  const length = direction.length();
  const perpendicular = new THREE.Vector3(-direction.y, direction.x, direction.z).normalize();

  midpoint.add(perpendicular.multiplyScalar(length * curvature));

  return new THREE.CatmullRomCurve3([source, midpoint, target]);
}

/**
 * Parse hex color to THREE.Color with fallback
 */
export function parseColor(hex: string, fallback: string = '#60a5fa'): THREE.Color {
  try {
    return new THREE.Color(hex);
  } catch {
    return new THREE.Color(fallback);
  }
}

/**
 * Create base uniforms for arc shaders
 */
export function createBaseUniforms(color: string) {
  return {
    time: { value: 0 },
    color: { value: parseColor(color) },
    opacity: { value: 1.0 },
    selected: { value: 0.0 },
    hovered: { value: 0.0 },
  };
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts
export * from './types';
export * from './SimpleLine3D';
export * from './shaderUtils';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add shader utilities for arc effects"
```

---

## Task 4: Implement PowerConduit3D (ownership)

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/PowerConduit3D.ts`

**Step 1: Create PowerConduit3D**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/PowerConduit3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { createArcTubeGeometry, createBaseUniforms, COMMON_GLSL } from './shaderUtils';

const VERTEX_SHADER = `
  varying vec2 vUv;
  varying vec3 vNormal;
  varying vec3 vViewPosition;

  void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);
    vViewPosition = -mvPosition.xyz;
    gl_Position = projectionMatrix * mvPosition;
  }
`;

const FRAGMENT_SHADER = `
  uniform float time;
  uniform vec3 color;
  uniform float opacity;
  uniform float selected;
  uniform float hovered;

  varying vec2 vUv;
  varying vec3 vNormal;
  varying vec3 vViewPosition;

  ${COMMON_GLSL.fresnel}
  ${COMMON_GLSL.pulse}
  ${COMMON_GLSL.flow}

  void main() {
    // Fresnel glow at edges
    vec3 viewDir = normalize(vViewPosition);
    float fresnelTerm = fresnel(viewDir, vNormal, 2.0);

    // Flow effect along the tube
    float flowValue = flow(vUv.x, time, 0.12);

    // 3 orbs traveling along the tube (convoy)
    float orb1 = smoothstep(0.05, 0.0, abs(flowValue - 0.0));
    float orb2 = smoothstep(0.05, 0.0, abs(flowValue - 0.33));
    float orb3 = smoothstep(0.05, 0.0, abs(flowValue - 0.66));
    float orbs = orb1 + orb2 + orb3;

    // Pulsing glow
    float pulseValue = pulse(time, 2.0, 0.3, 0.6);

    // Combine effects
    float intensity = fresnelTerm * 0.5 + orbs * 1.5 + pulseValue * 0.3;

    // Boost for selection/hover
    intensity *= 1.0 + selected * 0.5 + hovered * 0.3;

    vec3 finalColor = color * intensity;
    float finalOpacity = opacity * (0.4 + intensity * 0.6);

    gl_FragColor = vec4(finalColor, finalOpacity);
  }
`;

/**
 * PowerConduit3D - ownership arc effect
 * Visual: Glowing tube with 3 orbs traveling in convoy
 */
export class PowerConduit3D implements ArcEffect3D {
  public group: THREE.Group;
  public lodLevel: LODLevel = 'ultra';

  private tube: THREE.Mesh;
  private geometry: THREE.TubeGeometry;
  private material: THREE.ShaderMaterial;
  private uniforms: ReturnType<typeof createBaseUniforms>;

  private sourcePos: THREE.Vector3;
  private targetPos: THREE.Vector3;

  constructor(config: ArcEffectConfig) {
    this.group = new THREE.Group();
    this.sourcePos = config.sourcePosition.clone();
    this.targetPos = config.targetPosition.clone();

    // Create geometry
    this.geometry = createArcTubeGeometry(
      this.sourcePos,
      this.targetPos,
      0.8,  // radius
      64,   // tubular segments (high for smooth flow)
      12    // radial segments
    );

    // Create shader material
    this.uniforms = createBaseUniforms(config.color);
    this.uniforms.selected.value = config.isSelected ? 1.0 : 0.0;
    this.uniforms.hovered.value = config.isHovered ? 1.0 : 0.0;

    this.material = new THREE.ShaderMaterial({
      uniforms: this.uniforms,
      vertexShader: VERTEX_SHADER,
      fragmentShader: FRAGMENT_SHADER,
      transparent: true,
      blending: THREE.AdditiveBlending,
      side: THREE.DoubleSide,
      depthWrite: false,
    });

    // Create mesh
    this.tube = new THREE.Mesh(this.geometry, this.material);
    this.group.add(this.tube);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    this.sourcePos.copy(source);
    this.targetPos.copy(target);

    // Recreate geometry with new positions
    this.geometry.dispose();
    this.geometry = createArcTubeGeometry(this.sourcePos, this.targetPos, 0.8, 64, 12);
    this.tube.geometry = this.geometry;
  }

  updateUniforms(time: number, _deltaTime: number): void {
    this.uniforms.time.value = time;
  }

  setLOD(level: LODLevel): void {
    this.lodLevel = level;

    // Adjust quality based on LOD
    switch (level) {
      case 'ultra':
        this.material.uniforms.opacity.value = 1.0;
        break;
      case 'high':
        this.material.uniforms.opacity.value = 0.8;
        break;
      case 'medium':
        this.material.uniforms.opacity.value = 0.6;
        break;
      default:
        this.material.uniforms.opacity.value = 0.4;
    }
  }

  setSelected(selected: boolean): void {
    this.uniforms.selected.value = selected ? 1.0 : 0.0;
  }

  setHovered(hovered: boolean): void {
    this.uniforms.hovered.value = hovered ? 1.0 : 0.0;
  }

  dispose(): void {
    this.geometry.dispose();
    this.material.dispose();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts
export * from './types';
export * from './SimpleLine3D';
export * from './shaderUtils';
export * from './PowerConduit3D';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add PowerConduit3D effect for ownership arcs"
```

---

## Task 5: Implement SynapticFiring3D (semantic)

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/SynapticFiring3D.ts`

**Step 1: Create SynapticFiring3D**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/SynapticFiring3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { createArcTubeGeometry, createBaseUniforms, COMMON_GLSL, parseColor } from './shaderUtils';

const VERTEX_SHADER = `
  varying vec2 vUv;
  varying vec3 vNormal;
  varying vec3 vViewPosition;

  void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);
    vViewPosition = -mvPosition.xyz;
    gl_Position = projectionMatrix * mvPosition;
  }
`;

const FRAGMENT_SHADER = `
  uniform float time;
  uniform vec3 color;
  uniform float opacity;
  uniform float selected;
  uniform float hovered;
  uniform float firePhase;

  varying vec2 vUv;
  varying vec3 vNormal;
  varying vec3 vViewPosition;

  ${COMMON_GLSL.fresnel}
  ${COMMON_GLSL.noise}

  void main() {
    vec3 viewDir = normalize(vViewPosition);
    float fresnelTerm = fresnel(viewDir, vNormal, 3.0);

    // Fast firing pulse (leading bright point + trail)
    float cycleTime = mod(time * 0.4 + firePhase, 1.0);

    // Leading pulse
    float leadingPulse = smoothstep(0.08, 0.0, abs(vUv.x - cycleTime));

    // Trail of sparks behind the leading pulse
    float trailDist = vUv.x - cycleTime;
    float trail = 0.0;
    if (trailDist < 0.0 && trailDist > -0.3) {
      // Multiple sparks in the trail
      float sparkNoise = noise(vUv.x * 20.0 + time * 5.0);
      trail = smoothstep(0.3, 0.0, abs(trailDist)) * sparkNoise * 0.8;
    }

    // Random spark flickers
    float flicker = noise(time * 10.0 + vUv.x * 30.0) * 0.3;

    // Combine
    float intensity = fresnelTerm * 0.2 + leadingPulse * 2.0 + trail + flicker * leadingPulse;
    intensity *= 1.0 + selected * 0.6 + hovered * 0.3;

    vec3 finalColor = color * intensity;

    // White hot center for leading pulse
    finalColor = mix(finalColor, vec3(1.0), leadingPulse * 0.5);

    float finalOpacity = opacity * (0.2 + intensity * 0.8);

    gl_FragColor = vec4(finalColor, finalOpacity);
  }
`;

/**
 * SynapticFiring3D - semantic arc effect
 * Visual: Fast firing pulses with spark shower trail
 */
export class SynapticFiring3D implements ArcEffect3D {
  public group: THREE.Group;
  public lodLevel: LODLevel = 'ultra';

  private tube: THREE.Mesh;
  private geometry: THREE.TubeGeometry;
  private material: THREE.ShaderMaterial;
  private uniforms: ReturnType<typeof createBaseUniforms> & { firePhase: { value: number } };

  private sourcePos: THREE.Vector3;
  private targetPos: THREE.Vector3;

  constructor(config: ArcEffectConfig) {
    this.group = new THREE.Group();
    this.sourcePos = config.sourcePosition.clone();
    this.targetPos = config.targetPosition.clone();

    // Thinner tube for synaptic effect
    this.geometry = createArcTubeGeometry(this.sourcePos, this.targetPos, 0.4, 48, 8);

    // Random phase offset for varied timing
    const baseUniforms = createBaseUniforms(config.color);
    this.uniforms = {
      ...baseUniforms,
      firePhase: { value: Math.random() },
    };

    this.material = new THREE.ShaderMaterial({
      uniforms: this.uniforms,
      vertexShader: VERTEX_SHADER,
      fragmentShader: FRAGMENT_SHADER,
      transparent: true,
      blending: THREE.AdditiveBlending,
      side: THREE.DoubleSide,
      depthWrite: false,
    });

    this.tube = new THREE.Mesh(this.geometry, this.material);
    this.group.add(this.tube);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    this.sourcePos.copy(source);
    this.targetPos.copy(target);
    this.geometry.dispose();
    this.geometry = createArcTubeGeometry(this.sourcePos, this.targetPos, 0.4, 48, 8);
    this.tube.geometry = this.geometry;
  }

  updateUniforms(time: number, _deltaTime: number): void {
    this.uniforms.time.value = time;
  }

  setLOD(level: LODLevel): void {
    this.lodLevel = level;
    this.material.uniforms.opacity.value = level === 'ultra' ? 1.0 : level === 'high' ? 0.8 : 0.5;
  }

  dispose(): void {
    this.geometry.dispose();
    this.material.dispose();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts (add line)
export * from './SynapticFiring3D';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add SynapticFiring3D effect for semantic arcs"
```

---

## Task 6: Implement DNAHelix3D (localization)

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/DNAHelix3D.ts`

**Step 1: Create DNAHelix3D**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/DNAHelix3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { parseColor } from './shaderUtils';

const HELIX_PARTICLE_COUNT = 40; // 20 per strand

/**
 * DNAHelix3D - localization arc effect
 * Visual: Two intertwined helical particle spirals
 */
export class DNAHelix3D implements ArcEffect3D {
  public group: THREE.Group;
  public lodLevel: LODLevel = 'ultra';

  private particles: THREE.Points;
  private geometry: THREE.BufferGeometry;
  private material: THREE.PointsMaterial;

  private sourcePos: THREE.Vector3;
  private targetPos: THREE.Vector3;
  private color: THREE.Color;

  private basePositions: Float32Array;

  constructor(config: ArcEffectConfig) {
    this.group = new THREE.Group();
    this.sourcePos = config.sourcePosition.clone();
    this.targetPos = config.targetPosition.clone();
    this.color = parseColor(config.color);

    // Create particle geometry
    this.geometry = new THREE.BufferGeometry();
    this.basePositions = new Float32Array(HELIX_PARTICLE_COUNT * 3);

    this.initializeHelixPositions();

    this.geometry.setAttribute('position', new THREE.BufferAttribute(this.basePositions, 3));

    // Glowing particle material
    this.material = new THREE.PointsMaterial({
      color: this.color,
      size: 3,
      transparent: true,
      opacity: 0.9,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
      sizeAttenuation: true,
    });

    this.particles = new THREE.Points(this.geometry, this.material);
    this.group.add(this.particles);

    // Add thin connecting line
    const lineGeometry = new THREE.BufferGeometry().setFromPoints([this.sourcePos, this.targetPos]);
    const lineMaterial = new THREE.LineBasicMaterial({
      color: this.color,
      transparent: true,
      opacity: 0.2,
    });
    const line = new THREE.Line(lineGeometry, lineMaterial);
    this.group.add(line);
  }

  private initializeHelixPositions(): void {
    const direction = new THREE.Vector3().subVectors(this.targetPos, this.sourcePos);
    const length = direction.length();
    direction.normalize();

    // Create perpendicular vectors for helix
    const up = new THREE.Vector3(0, 1, 0);
    const perp1 = new THREE.Vector3().crossVectors(direction, up).normalize();
    if (perp1.length() < 0.1) {
      perp1.crossVectors(direction, new THREE.Vector3(1, 0, 0)).normalize();
    }
    const perp2 = new THREE.Vector3().crossVectors(direction, perp1).normalize();

    const helixRadius = 2.0;
    const turns = 3;

    for (let i = 0; i < HELIX_PARTICLE_COUNT; i++) {
      const t = i / (HELIX_PARTICLE_COUNT / 2); // 0 to 2 (two strands)
      const strand = i < HELIX_PARTICLE_COUNT / 2 ? 0 : 1;
      const localT = strand === 0 ? t : t - 1;

      // Position along the arc
      const pos = new THREE.Vector3().copy(this.sourcePos)
        .addScaledVector(direction, localT * length);

      // Helix offset
      const angle = localT * Math.PI * 2 * turns + strand * Math.PI; // Offset second strand
      pos.addScaledVector(perp1, Math.cos(angle) * helixRadius);
      pos.addScaledVector(perp2, Math.sin(angle) * helixRadius);

      this.basePositions[i * 3] = pos.x;
      this.basePositions[i * 3 + 1] = pos.y;
      this.basePositions[i * 3 + 2] = pos.z;
    }
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    this.sourcePos.copy(source);
    this.targetPos.copy(target);
    this.initializeHelixPositions();
    this.geometry.attributes.position.needsUpdate = true;
  }

  updateUniforms(time: number, _deltaTime: number): void {
    // Animate helix rotation
    const positions = this.geometry.attributes.position.array as Float32Array;
    const direction = new THREE.Vector3().subVectors(this.targetPos, this.sourcePos);
    const length = direction.length();
    direction.normalize();

    const up = new THREE.Vector3(0, 1, 0);
    const perp1 = new THREE.Vector3().crossVectors(direction, up).normalize();
    if (perp1.length() < 0.1) {
      perp1.crossVectors(direction, new THREE.Vector3(1, 0, 0)).normalize();
    }
    const perp2 = new THREE.Vector3().crossVectors(direction, perp1).normalize();

    const helixRadius = 2.0;
    const turns = 3;
    const rotationSpeed = 0.5;

    for (let i = 0; i < HELIX_PARTICLE_COUNT; i++) {
      const strand = i < HELIX_PARTICLE_COUNT / 2 ? 0 : 1;
      const localT = strand === 0
        ? i / (HELIX_PARTICLE_COUNT / 2)
        : (i - HELIX_PARTICLE_COUNT / 2) / (HELIX_PARTICLE_COUNT / 2);

      const pos = new THREE.Vector3().copy(this.sourcePos)
        .addScaledVector(direction, localT * length);

      const angle = localT * Math.PI * 2 * turns + strand * Math.PI + time * rotationSpeed;
      pos.addScaledVector(perp1, Math.cos(angle) * helixRadius);
      pos.addScaledVector(perp2, Math.sin(angle) * helixRadius);

      positions[i * 3] = pos.x;
      positions[i * 3 + 1] = pos.y;
      positions[i * 3 + 2] = pos.z;
    }

    this.geometry.attributes.position.needsUpdate = true;
  }

  setLOD(level: LODLevel): void {
    this.lodLevel = level;
    this.material.opacity = level === 'ultra' ? 0.9 : level === 'high' ? 0.7 : 0.4;
    this.material.size = level === 'ultra' ? 3 : level === 'high' ? 2 : 1;
  }

  dispose(): void {
    this.geometry.dispose();
    this.material.dispose();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts (add line)
export * from './DNAHelix3D';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add DNAHelix3D effect for localization arcs"
```

---

## Task 7: Implement MatrixRain3D (generation)

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/MatrixRain3D.ts`

**Step 1: Create MatrixRain3D**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/MatrixRain3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { createArcTubeGeometry, createBaseUniforms, COMMON_GLSL } from './shaderUtils';

const VERTEX_SHADER = `
  varying vec2 vUv;
  varying vec3 vNormal;
  varying vec3 vViewPosition;

  void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);
    vViewPosition = -mvPosition.xyz;
    gl_Position = projectionMatrix * mvPosition;
  }
`;

const FRAGMENT_SHADER = `
  uniform float time;
  uniform vec3 color;
  uniform float opacity;
  uniform float selected;
  uniform float hovered;

  varying vec2 vUv;
  varying vec3 vNormal;
  varying vec3 vViewPosition;

  ${COMMON_GLSL.fresnel}
  ${COMMON_GLSL.noise}

  void main() {
    vec3 viewDir = normalize(vViewPosition);
    float fresnelTerm = fresnel(viewDir, vNormal, 2.5);

    // Scanline effect (fast moving band)
    float scanSpeed = 0.8;
    float scanPos = fract(time * scanSpeed);
    float scanline = smoothstep(0.1, 0.0, abs(vUv.x - scanPos));

    // Digital rain drops (multiple falling elements)
    float rainSpeed = 1.2;
    float rainDensity = 8.0;

    float rain = 0.0;
    for (float i = 0.0; i < 5.0; i++) {
      float offset = i * 0.2;
      float dropPos = fract(time * rainSpeed * (0.8 + i * 0.1) + offset);
      float dropX = fract(vUv.y * rainDensity + i * 0.37);

      // Each drop is a vertical streak
      float drop = smoothstep(0.15, 0.0, abs(vUv.x - dropPos))
                 * smoothstep(0.1, 0.0, abs(dropX - 0.5));
      rain += drop * (0.3 + i * 0.1);
    }

    // Digital noise overlay
    float digitalNoise = noise(vUv.x * 50.0 + time * 20.0) * noise(vUv.y * 30.0 + time * 15.0);

    // Output pulse at the end
    float outputPulse = smoothstep(0.95, 1.0, vUv.x) * (0.5 + 0.5 * sin(time * 8.0));

    // Combine
    float intensity = fresnelTerm * 0.3 + scanline * 1.5 + rain * 0.8 + digitalNoise * 0.2 + outputPulse;
    intensity *= 1.0 + selected * 0.5 + hovered * 0.3;

    vec3 finalColor = color * intensity;

    // Bright white for scanline
    finalColor = mix(finalColor, vec3(1.0), scanline * 0.6);

    float finalOpacity = opacity * (0.3 + intensity * 0.7);

    gl_FragColor = vec4(finalColor, finalOpacity);
  }
`;

/**
 * MatrixRain3D - generation arc effect
 * Visual: Digital rain cascade with scanline and output pulse
 */
export class MatrixRain3D implements ArcEffect3D {
  public group: THREE.Group;
  public lodLevel: LODLevel = 'ultra';

  private tube: THREE.Mesh;
  private geometry: THREE.TubeGeometry;
  private material: THREE.ShaderMaterial;
  private uniforms: ReturnType<typeof createBaseUniforms>;

  private sourcePos: THREE.Vector3;
  private targetPos: THREE.Vector3;

  constructor(config: ArcEffectConfig) {
    this.group = new THREE.Group();
    this.sourcePos = config.sourcePosition.clone();
    this.targetPos = config.targetPosition.clone();

    // Slightly thicker tube for generation
    this.geometry = createArcTubeGeometry(this.sourcePos, this.targetPos, 0.6, 64, 10);

    this.uniforms = createBaseUniforms(config.color);

    this.material = new THREE.ShaderMaterial({
      uniforms: this.uniforms,
      vertexShader: VERTEX_SHADER,
      fragmentShader: FRAGMENT_SHADER,
      transparent: true,
      blending: THREE.AdditiveBlending,
      side: THREE.DoubleSide,
      depthWrite: false,
    });

    this.tube = new THREE.Mesh(this.geometry, this.material);
    this.group.add(this.tube);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    this.sourcePos.copy(source);
    this.targetPos.copy(target);
    this.geometry.dispose();
    this.geometry = createArcTubeGeometry(this.sourcePos, this.targetPos, 0.6, 64, 10);
    this.tube.geometry = this.geometry;
  }

  updateUniforms(time: number, _deltaTime: number): void {
    this.uniforms.time.value = time;
  }

  setLOD(level: LODLevel): void {
    this.lodLevel = level;
    this.material.uniforms.opacity.value = level === 'ultra' ? 1.0 : level === 'high' ? 0.8 : 0.5;
  }

  dispose(): void {
    this.geometry.dispose();
    this.material.dispose();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts (add line)
export * from './MatrixRain3D';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add MatrixRain3D effect for generation arcs"
```

---

## Task 8: Implement SonarPulse3D (mining)

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/SonarPulse3D.ts`

**Step 1: Create SonarPulse3D**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/SonarPulse3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { parseColor } from './shaderUtils';

const RING_COUNT = 3;

/**
 * SonarPulse3D - mining arc effect
 * Visual: Ping wave with expanding concentric rings
 */
export class SonarPulse3D implements ArcEffect3D {
  public group: THREE.Group;
  public lodLevel: LODLevel = 'ultra';

  private line: THREE.Line;
  private lineGeometry: THREE.BufferGeometry;
  private lineMaterial: THREE.LineBasicMaterial;

  private rings: THREE.Mesh[];
  private ringGeometries: THREE.RingGeometry[];
  private ringMaterials: THREE.MeshBasicMaterial[];

  private pingParticle: THREE.Mesh;
  private pingGeometry: THREE.SphereGeometry;
  private pingMaterial: THREE.MeshBasicMaterial;

  private sourcePos: THREE.Vector3;
  private targetPos: THREE.Vector3;
  private color: THREE.Color;

  private pulsePhase: number;

  constructor(config: ArcEffectConfig) {
    this.group = new THREE.Group();
    this.sourcePos = config.sourcePosition.clone();
    this.targetPos = config.targetPosition.clone();
    this.color = parseColor(config.color);
    this.pulsePhase = Math.random(); // Random start phase

    // Base line
    this.lineGeometry = new THREE.BufferGeometry().setFromPoints([this.sourcePos, this.targetPos]);
    this.lineMaterial = new THREE.LineBasicMaterial({
      color: this.color,
      transparent: true,
      opacity: 0.3,
    });
    this.line = new THREE.Line(this.lineGeometry, this.lineMaterial);
    this.group.add(this.line);

    // Expanding rings
    this.rings = [];
    this.ringGeometries = [];
    this.ringMaterials = [];

    for (let i = 0; i < RING_COUNT; i++) {
      const ringGeometry = new THREE.RingGeometry(0.5, 1.0, 32);
      const ringMaterial = new THREE.MeshBasicMaterial({
        color: this.color,
        transparent: true,
        opacity: 0.6,
        side: THREE.DoubleSide,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      });
      const ring = new THREE.Mesh(ringGeometry, ringMaterial);

      this.ringGeometries.push(ringGeometry);
      this.ringMaterials.push(ringMaterial);
      this.rings.push(ring);
      this.group.add(ring);
    }

    // Ping particle (traveling dot)
    this.pingGeometry = new THREE.SphereGeometry(1.5, 16, 16);
    this.pingMaterial = new THREE.MeshBasicMaterial({
      color: this.color,
      transparent: true,
      opacity: 0.9,
      blending: THREE.AdditiveBlending,
    });
    this.pingParticle = new THREE.Mesh(this.pingGeometry, this.pingMaterial);
    this.group.add(this.pingParticle);

    this.updatePositions(this.sourcePos, this.targetPos);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    this.sourcePos.copy(source);
    this.targetPos.copy(target);

    // Update line
    const positions = this.lineGeometry.attributes.position?.array as Float32Array;
    if (positions) {
      positions[0] = source.x;
      positions[1] = source.y;
      positions[2] = source.z;
      positions[3] = target.x;
      positions[4] = target.y;
      positions[5] = target.z;
      this.lineGeometry.attributes.position.needsUpdate = true;
    }

    // Orient rings perpendicular to arc direction
    const direction = new THREE.Vector3().subVectors(target, source).normalize();
    const quaternion = new THREE.Quaternion().setFromUnitVectors(
      new THREE.Vector3(0, 0, 1),
      direction
    );

    this.rings.forEach(ring => {
      ring.quaternion.copy(quaternion);
    });
  }

  updateUniforms(time: number, _deltaTime: number): void {
    const cycleTime = (time * 0.25 + this.pulsePhase) % 1.0; // 4 second cycle
    const direction = new THREE.Vector3().subVectors(this.targetPos, this.sourcePos);
    const length = direction.length();

    // Animate ping particle along the arc
    const pingPos = new THREE.Vector3().copy(this.sourcePos)
      .addScaledVector(direction.normalize(), cycleTime * length);
    this.pingParticle.position.copy(pingPos);

    // Pulse size for ping
    const pulseScale = 0.8 + 0.4 * Math.sin(time * 8);
    this.pingParticle.scale.setScalar(pulseScale);

    // Animate expanding rings from ping position
    this.rings.forEach((ring, i) => {
      const ringPhase = (cycleTime + i * 0.15) % 1.0;
      const ringScale = 1 + ringPhase * 8; // Expand from 1 to 9
      const ringOpacity = Math.max(0, 0.6 - ringPhase * 0.8); // Fade out

      ring.position.copy(pingPos);
      ring.scale.setScalar(ringScale);
      this.ringMaterials[i].opacity = ringOpacity;
    });
  }

  setLOD(level: LODLevel): void {
    this.lodLevel = level;

    const visible = level !== 'low';
    this.rings.forEach(ring => ring.visible = visible);

    this.pingMaterial.opacity = level === 'ultra' ? 0.9 : level === 'high' ? 0.7 : 0.4;
  }

  dispose(): void {
    this.lineGeometry.dispose();
    this.lineMaterial.dispose();
    this.ringGeometries.forEach(g => g.dispose());
    this.ringMaterials.forEach(m => m.dispose());
    this.pingGeometry.dispose();
    this.pingMaterial.dispose();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts (add line)
export * from './SonarPulse3D';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add SonarPulse3D effect for mining arcs"
```

---

## Task 9: Implement ArcLODManager

**Files:**
- Create: `apps/studio/src/lib/graph3d/arcEffects/ArcLODManager.ts`

**Step 1: Create ArcLODManager**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/ArcLODManager.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, ArcFamily, LODLevel } from './types';
import { LOD_THRESHOLDS, ARC_FAMILY_COLORS } from './types';
import { SimpleLine3D } from './SimpleLine3D';
import { PowerConduit3D } from './PowerConduit3D';
import { SynapticFiring3D } from './SynapticFiring3D';
import { DNAHelix3D } from './DNAHelix3D';
import { MatrixRain3D } from './MatrixRain3D';
import { SonarPulse3D } from './SonarPulse3D';

interface ManagedArc {
  id: string;
  effect: ArcEffect3D;
  family: ArcFamily;
  sourceNodeId: string;
  targetNodeId: string;
  midpoint: THREE.Vector3;
  currentLOD: LODLevel;
}

/**
 * ArcLODManager - Manages arc effects with distance-based LOD
 */
export class ArcLODManager {
  private arcs: Map<string, ManagedArc> = new Map();
  private scene: THREE.Group;
  private hysteresis = 20; // Buffer zone to prevent flip-flop

  constructor() {
    this.scene = new THREE.Group();
  }

  getScene(): THREE.Group {
    return this.scene;
  }

  /**
   * Create effect for arc family
   */
  private createEffect(family: ArcFamily, config: ArcEffectConfig): ArcEffect3D {
    switch (family) {
      case 'ownership':
        return new PowerConduit3D(config);
      case 'localization':
        return new DNAHelix3D(config);
      case 'semantic':
        return new SynapticFiring3D(config);
      case 'generation':
        return new MatrixRain3D(config);
      case 'mining':
        return new SonarPulse3D(config);
      default:
        return new SimpleLine3D(config);
    }
  }

  /**
   * Add a new arc
   */
  addArc(
    id: string,
    family: ArcFamily,
    sourceNodeId: string,
    targetNodeId: string,
    sourcePosition: THREE.Vector3,
    targetPosition: THREE.Vector3
  ): void {
    // Remove existing if present
    if (this.arcs.has(id)) {
      this.removeArc(id);
    }

    const config: ArcEffectConfig = {
      sourcePosition,
      targetPosition,
      family,
      color: ARC_FAMILY_COLORS[family],
    };

    const effect = this.createEffect(family, config);

    const midpoint = new THREE.Vector3()
      .addVectors(sourcePosition, targetPosition)
      .multiplyScalar(0.5);

    const managedArc: ManagedArc = {
      id,
      effect,
      family,
      sourceNodeId,
      targetNodeId,
      midpoint,
      currentLOD: 'ultra',
    };

    this.arcs.set(id, managedArc);
    this.scene.add(effect.group);
  }

  /**
   * Remove an arc
   */
  removeArc(id: string): void {
    const arc = this.arcs.get(id);
    if (arc) {
      this.scene.remove(arc.effect.group);
      arc.effect.dispose();
      this.arcs.delete(id);
    }
  }

  /**
   * Update arc positions (when nodes move)
   */
  updateArcPositions(
    id: string,
    sourcePosition: THREE.Vector3,
    targetPosition: THREE.Vector3
  ): void {
    const arc = this.arcs.get(id);
    if (arc) {
      arc.effect.updatePositions(sourcePosition, targetPosition);
      arc.midpoint.addVectors(sourcePosition, targetPosition).multiplyScalar(0.5);
    }
  }

  /**
   * Calculate LOD level based on distance
   */
  private calculateLOD(distance: number, currentLOD: LODLevel): LODLevel {
    // Apply hysteresis based on direction of change
    const hysteresis = this.hysteresis;

    if (distance < LOD_THRESHOLDS.ultra - (currentLOD === 'ultra' ? 0 : hysteresis)) {
      return 'ultra';
    } else if (distance < LOD_THRESHOLDS.high - (currentLOD === 'high' ? 0 : hysteresis)) {
      return 'high';
    } else if (distance < LOD_THRESHOLDS.medium - (currentLOD === 'medium' ? 0 : hysteresis)) {
      return 'medium';
    }
    return 'low';
  }

  /**
   * Update all arcs (call each frame)
   */
  update(camera: THREE.Camera, time: number, deltaTime: number): void {
    const cameraPosition = camera.position;

    for (const arc of this.arcs.values()) {
      // Calculate distance to camera
      const distance = cameraPosition.distanceTo(arc.midpoint);

      // Check LOD transition
      const newLOD = this.calculateLOD(distance, arc.currentLOD);
      if (newLOD !== arc.currentLOD) {
        arc.effect.setLOD(newLOD);
        arc.currentLOD = newLOD;
      }

      // Update shader uniforms
      arc.effect.updateUniforms(time, deltaTime);
    }
  }

  /**
   * Clear all arcs
   */
  clear(): void {
    for (const arc of this.arcs.values()) {
      this.scene.remove(arc.effect.group);
      arc.effect.dispose();
    }
    this.arcs.clear();
  }

  /**
   * Dispose manager
   */
  dispose(): void {
    this.clear();
  }
}
```

**Step 2: Export from index**

```typescript
// apps/studio/src/lib/graph3d/arcEffects/index.ts (add line)
export * from './ArcLODManager';
```

**Step 3: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 4: Commit**

```bash
git add apps/studio/src/lib/graph3d/arcEffects/
git commit -m "feat(studio): add ArcLODManager for distance-based LOD"
```

---

## Task 10: Integrate into Graph3D.tsx

**Files:**
- Modify: `apps/studio/src/components/graph/Graph3D.tsx`
- Modify: `apps/studio/src/lib/graph3d/index.ts`

**Step 1: Update graph3d index exports**

```typescript
// apps/studio/src/lib/graph3d/index.ts (add line at end)
export * from './arcEffects';
```

**Step 2: Import ArcLODManager in Graph3D.tsx**

Add at top of file after other imports:

```typescript
import { ArcLODManager, type ArcFamily } from '@/lib/graph3d/arcEffects';
import { detectArcFamily } from '@/lib/graph3d/arcParticles';
```

**Step 3: Add ArcLODManager ref**

After `composerRef` declaration (~line 186):

```typescript
const arcLODManagerRef = useRef<ArcLODManager | null>(null);
```

**Step 4: Initialize ArcLODManager**

Add new useEffect after the starfield effect (~line 328):

```typescript
// Initialize arc effects manager
useEffect(() => {
  if (!isGraphReady || !fgRef.current) return;

  const scene = fgRef.current.scene?.();
  if (!scene) return;

  // Create manager if not exists
  if (!arcLODManagerRef.current) {
    arcLODManagerRef.current = new ArcLODManager();
    scene.add(arcLODManagerRef.current.getScene());
  }

  // Populate arcs
  const manager = arcLODManagerRef.current;
  manager.clear();

  graphData.links.forEach((link) => {
    if (!isValidForceGraphLink(link)) return;

    const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
    const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
    const sourceNode = graphData.nodes.find(n => n.id === sourceId);
    const targetNode = graphData.nodes.find(n => n.id === targetId);

    if (!sourceNode || !targetNode) return;

    const sourcePos = new THREE.Vector3(
      sourceNode.x ?? 0,
      sourceNode.y ?? 0,
      sourceNode.z ?? 0
    );
    const targetPos = new THREE.Vector3(
      targetNode.x ?? 0,
      targetNode.y ?? 0,
      targetNode.z ?? 0
    );

    const family = detectArcFamily(link.type ?? '') as ArcFamily;
    manager.addArc(link.id, family, sourceId, targetId, sourcePos, targetPos);
  });

  return () => {
    if (arcLODManagerRef.current) {
      scene.remove(arcLODManagerRef.current.getScene());
      arcLODManagerRef.current.dispose();
      arcLODManagerRef.current = null;
    }
  };
}, [isGraphReady, graphData.links, graphData.nodes]);
```

**Step 5: Update arc positions in engine tick**

Modify `handleEngineTick` callback (~line 822):

```typescript
const handleEngineTick = useCallback(() => {
  if (!isGraphReady) {
    setIsGraphReady(true);
  }

  // Update arc positions and LOD
  if (arcLODManagerRef.current && fgRef.current) {
    const camera = fgRef.current.camera?.();
    if (camera) {
      const time = performance.now() * 0.001;
      const deltaTime = 0.016; // ~60fps

      // Update arc positions from simulation
      graphData.links.forEach((link) => {
        if (!isValidForceGraphLink(link)) return;

        const sourceId = getNodeIdFromLinkEndpoint(link.source as LinkEndpoint);
        const targetId = getNodeIdFromLinkEndpoint(link.target as LinkEndpoint);
        const sourceNode = graphData.nodes.find(n => n.id === sourceId);
        const targetNode = graphData.nodes.find(n => n.id === targetId);

        if (!sourceNode || !targetNode) return;

        const sourcePos = new THREE.Vector3(
          sourceNode.x ?? 0,
          sourceNode.y ?? 0,
          sourceNode.z ?? 0
        );
        const targetPos = new THREE.Vector3(
          targetNode.x ?? 0,
          targetNode.y ?? 0,
          targetNode.z ?? 0
        );

        arcLODManagerRef.current?.updateArcPositions(link.id, sourcePos, targetPos);
      });

      // Update LOD and animations
      arcLODManagerRef.current.update(camera, time, deltaTime);
    }
  }
}, [isGraphReady, graphData.links, graphData.nodes]);
```

**Step 6: Disable built-in link rendering**

In ForceGraph3D component props (~line 969-977), change:

```typescript
// BEFORE:
linkDirectionalParticles={5}
linkDirectionalParticleSpeed={0.004}
linkDirectionalParticleWidth={4}
linkDirectionalParticleColor={getLinkParticleColor as any}
linkDirectionalParticleResolution={32}
linkDirectionalParticleThreeObject={getParticleThreeObject as any}

// AFTER:
linkDirectionalParticles={0}  // Disable built-in particles
linkVisibility={false}        // Hide default links (we render custom)
```

**Step 7: Verify TypeScript compiles**

Run: `cd apps/studio && pnpm tsc --noEmit`
Expected: No errors

**Step 8: Test in browser**

Run: `pnpm dev`
Navigate to: http://localhost:3000
Expected: 3D graph shows custom arc effects instead of basic lines

**Step 9: Commit**

```bash
git add apps/studio/src/components/graph/Graph3D.tsx apps/studio/src/lib/graph3d/index.ts
git commit -m "feat(studio): integrate ArcLODManager into Graph3D

- Replace built-in particle system with custom arc effects
- Initialize ArcLODManager on graph ready
- Update arc positions from force simulation
- Update LOD and animations each frame"
```

---

## Task 11: Visual Testing & Parameter Tuning

**Files:**
- Potentially modify any arcEffect file for tuning

**Step 1: Test each arc family**

Open Studio at http://localhost:3000, switch to 3D view, and verify:

- [ ] ownership arcs: Blue tubes with 3 traveling orbs
- [ ] localization arcs: Green double helix particles
- [ ] semantic arcs: Orange fast-firing sparks
- [ ] generation arcs: Violet scanline/rain effect
- [ ] mining arcs: Pink expanding ring pulses

**Step 2: Test LOD transitions**

Zoom in/out and verify:
- [ ] Close (< 150): Full effects visible
- [ ] Medium (150-400): Simplified but visible
- [ ] Far (400-800): Basic glow tube
- [ ] Very far (800+): Simple line

**Step 3: Test performance**

Open browser DevTools, check:
- [ ] FPS stays above 30 with all arcs visible
- [ ] No memory leaks (memory stable over time)
- [ ] Smooth LOD transitions (no flickering)

**Step 4: Adjust parameters if needed**

Common tuning:
- Shader intensity values
- Particle counts
- Animation speeds
- LOD thresholds

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat(studio): finalize 3D arc signature effects

Visual testing complete:
- 5 signature effects working (ownership, localization, semantic, generation, mining)
- LOD system functioning (ULTRA → LOW based on distance)
- Performance acceptable (60fps target met)"
```

---

## Summary

| Task | Description | Est. Time |
|------|-------------|-----------|
| 1 | Directory structure & types | 15 min |
| 2 | SimpleLine3D fallback | 15 min |
| 3 | Shader utilities | 20 min |
| 4 | PowerConduit3D (ownership) | 30 min |
| 5 | SynapticFiring3D (semantic) | 30 min |
| 6 | DNAHelix3D (localization) | 30 min |
| 7 | MatrixRain3D (generation) | 30 min |
| 8 | SonarPulse3D (mining) | 30 min |
| 9 | ArcLODManager | 30 min |
| 10 | Graph3D integration | 45 min |
| 11 | Visual testing & tuning | 30 min |

**Total: ~5h** (optimistic) to **8h** (with debugging)
