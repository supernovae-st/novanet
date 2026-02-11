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
