// apps/studio/src/lib/graph3d/arcEffects/SynapticFiring3D.ts
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
