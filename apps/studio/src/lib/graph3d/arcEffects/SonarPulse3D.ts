// apps/studio/src/lib/graph3d/arcEffects/SonarPulse3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { parseColor } from './shaderUtils';
import { DustParticleSystem, DUST_CONFIG } from './dustParticles';

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
  private dustSystem: DustParticleSystem;

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

    // Add dust particle system (hexagon shapes with pulsing drift)
    this.dustSystem = new DustParticleSystem(DUST_CONFIG.mining);
    this.dustSystem.updateArcPositions(this.sourcePos, this.targetPos);
    this.group.add(this.dustSystem.mesh);

    this.updatePositions(this.sourcePos, this.targetPos);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    // Defensive: skip if positions are invalid
    if (!source || !target) return;
    if (typeof source.x !== 'number' || typeof target.x !== 'number') return;

    // Only update if position changed significantly
    const threshold = 0.5;
    if (this.sourcePos.distanceTo(source) < threshold && this.targetPos.distanceTo(target) < threshold) return;

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

    // Update dust particle positions
    this.dustSystem.updateArcPositions(this.sourcePos, this.targetPos);
  }

  updateUniforms(time: number, deltaTime: number): void {
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

    // Animate dust particles
    this.dustSystem.update(time, deltaTime);
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
    this.dustSystem.dispose();
  }
}
