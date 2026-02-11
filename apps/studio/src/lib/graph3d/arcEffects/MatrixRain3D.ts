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
