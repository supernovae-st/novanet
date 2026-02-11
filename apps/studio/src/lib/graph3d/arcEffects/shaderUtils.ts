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
