// apps/studio/src/lib/graph3d/arcEffects/dustParticles.ts
/**
 * Dust Particle System for 3D Arc Effects
 *
 * Particles TRAVEL along the arc from source to target while applying
 * perpendicular drift based on arc family.
 *
 * Shapes are aligned with 2D visual metaphors from visual-encoding.yaml:
 * - ownership (power_conduit) → diamonds (energy crystals)
 * - localization (dna_helix) → circles (base pairs)
 * - semantic (synaptic_firing) → stars (sparks)
 * - generation (matrix_code_rain) → triangles (data fragments)
 * - mining (sonar_pulse) → hexagons (data crystals)
 */

import * as THREE from 'three';
import type { ArcFamily } from '@/lib/graph3d/arcParticles';
import { calculateHelixOffset } from './shaderUtils';

// =============================================================================
// TYPES
// =============================================================================

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

// =============================================================================
// CONFIGURATION
// =============================================================================

/**
 * Dust configuration per arc family
 * Colors from taxonomy.yaml, shapes from visual-encoding.yaml metaphors
 */
export const DUST_CONFIG: Record<ArcFamily, DustConfig> = {
  ownership: {
    shape: 'diamond',
    count: 8,
    size: 0.15,
    drift: 'gravitational',
    opacity: 0.6,
    color: '#3b82f6', // blue
  },
  localization: {
    shape: 'circle',
    count: 10,
    size: 0.12,
    drift: 'orbital',
    opacity: 0.5,
    color: '#22c55e', // green
  },
  semantic: {
    shape: 'star',
    count: 12,
    size: 0.1,
    drift: 'sparkling',
    opacity: 0.7,
    color: '#f97316', // orange
  },
  generation: {
    shape: 'triangle',
    count: 15,
    size: 0.08,
    drift: 'cascade',
    opacity: 0.6,
    color: '#8b5cf6', // purple
  },
  mining: {
    shape: 'hexagon',
    count: 6,
    size: 0.18,
    drift: 'pulsing',
    opacity: 0.5,
    color: '#ec4899', // pink
  },
};

// =============================================================================
// GEOMETRY FACTORIES
// =============================================================================

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

// =============================================================================
// DUST PARTICLE SYSTEM CLASS
// =============================================================================

/**
 * DustParticleSystem - Manages dust particles that TRAVEL along an arc
 * Particles flow from source to target while applying perpendicular drift
 * Uses InstancedMesh for performance (one draw call)
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

      // Apply helix motion (zig-zag path) - subtle oscillation perpendicular to arc
      const helixAmplitude = 0.15;  // How far from center line
      const helixFrequency = 2.5;   // Number of oscillations along arc
      const helixOffset = calculateHelixOffset(t, helixAmplitude, helixFrequency, phase, time);
      basePos.add(helixOffset);

      // Calculate perpendicular offset based on drift type
      const perpOffset = new THREE.Vector3();
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
          {
            const orbitAngle = time * 2 + phase + t * Math.PI * 4;
            perpOffset.set(
              Math.cos(orbitAngle) * perpDist,
              Math.sin(orbitAngle) * perpDist,
              0
            );
          }
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
          {
            const pulseScale = 1 + Math.sin(time * 2 + phase) * 0.5;
            perpOffset.set(
              Math.cos(phase) * perpDist * pulseScale,
              Math.sin(phase) * perpDist * pulseScale,
              0
            );
          }
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
