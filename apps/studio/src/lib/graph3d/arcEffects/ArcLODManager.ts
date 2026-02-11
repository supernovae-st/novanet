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
