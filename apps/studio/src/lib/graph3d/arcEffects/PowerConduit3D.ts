// apps/studio/src/lib/graph3d/arcEffects/PowerConduit3D.ts
import * as THREE from 'three';
import type { ArcEffect3D, ArcEffectConfig, LODLevel } from './types';
import { createArcTubeGeometry, createBaseUniforms, COMMON_GLSL } from './shaderUtils';
import { DustParticleSystem, DUST_CONFIG } from './dustParticles';

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
  private dustSystem: DustParticleSystem;

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

    // Add dust particle system (diamond shapes with gravitational drift)
    this.dustSystem = new DustParticleSystem(DUST_CONFIG.ownership);
    this.dustSystem.updateArcPositions(this.sourcePos, this.targetPos);
    this.group.add(this.dustSystem.mesh);
  }

  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void {
    // Defensive: skip if positions are invalid
    if (!source || !target) return;
    if (typeof source.x !== 'number' || typeof target.x !== 'number') return;

    // Only update if position changed significantly (threshold: 0.5 units)
    const threshold = 0.5;
    const sourceMoved = this.sourcePos.distanceTo(source) > threshold;
    const targetMoved = this.targetPos.distanceTo(target) > threshold;

    if (!sourceMoved && !targetMoved) return;

    this.sourcePos.copy(source);
    this.targetPos.copy(target);

    // Recreate geometry with new positions
    this.geometry.dispose();
    this.geometry = createArcTubeGeometry(this.sourcePos, this.targetPos, 0.8, 64, 12);
    this.tube.geometry = this.geometry;

    // Update dust particle positions
    this.dustSystem.updateArcPositions(this.sourcePos, this.targetPos);
  }

  updateUniforms(time: number, deltaTime: number): void {
    this.uniforms.time.value = time;

    // Animate dust particles
    this.dustSystem.update(time, deltaTime);
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
    this.dustSystem.dispose();
  }
}
