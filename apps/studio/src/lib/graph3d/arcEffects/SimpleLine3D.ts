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
    // Defensive: skip if positions are invalid
    if (!source || !target) return;
    if (typeof source.x !== 'number' || typeof target.x !== 'number') return;

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
