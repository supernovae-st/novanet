/**
 * Post-Processing Effects for 3D Graph Visualization
 *
 * Provides bloom effects, glow, and other visual enhancements.
 * Uses Three.js post-processing pipeline.
 */

import * as THREE from 'three';
import { EffectComposer } from 'three/examples/jsm/postprocessing/EffectComposer.js';
import { RenderPass } from 'three/examples/jsm/postprocessing/RenderPass.js';
import { UnrealBloomPass } from 'three/examples/jsm/postprocessing/UnrealBloomPass.js';
import { ShaderPass } from 'three/examples/jsm/postprocessing/ShaderPass.js';

// Vignette shader for cinematic edges
const VignetteShader = {
  uniforms: {
    tDiffuse: { value: null },
    offset: { value: 0.5 },
    darkness: { value: 0.5 },
  },
  vertexShader: `
    varying vec2 vUv;
    void main() {
      vUv = uv;
      gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
  `,
  fragmentShader: `
    uniform sampler2D tDiffuse;
    uniform float offset;
    uniform float darkness;
    varying vec2 vUv;
    void main() {
      vec4 texel = texture2D(tDiffuse, vUv);
      vec2 uv = (vUv - vec2(0.5)) * vec2(offset);
      float vignette = 1.0 - dot(uv, uv);
      texel.rgb *= mix(1.0 - darkness, 1.0, vignette);
      gl_FragColor = texel;
    }
  `,
};

// Bloom configuration
export interface BloomConfig {
  strength: number;     // Intensity of bloom (0-3)
  radius: number;       // Bloom spread radius (0-1)
  threshold: number;    // Brightness threshold (0-1)
}

// Default bloom settings for galaxy-themed graph
export const DEFAULT_BLOOM_CONFIG: BloomConfig = {
  strength: 0.8,
  radius: 0.4,
  threshold: 0.6,
};

// High bloom for generated/output nodes
export const HIGH_BLOOM_CONFIG: BloomConfig = {
  strength: 1.5,
  radius: 0.6,
  threshold: 0.4,
};

/**
 * Create post-processing composer with bloom effect
 */
export function createBloomComposer(
  renderer: THREE.WebGLRenderer,
  scene: THREE.Scene,
  camera: THREE.Camera,
  config: BloomConfig = DEFAULT_BLOOM_CONFIG
): EffectComposer {
  const composer = new EffectComposer(renderer);

  // Render pass
  const renderPass = new RenderPass(scene, camera);
  composer.addPass(renderPass);

  // Bloom pass
  const bloomPass = new UnrealBloomPass(
    new THREE.Vector2(window.innerWidth, window.innerHeight),
    config.strength,
    config.radius,
    config.threshold
  );
  composer.addPass(bloomPass);

  return composer;
}

/**
 * Update bloom composer on resize
 */
export function updateComposerSize(
  composer: EffectComposer,
  width: number,
  height: number
): void {
  composer.setSize(width, height);
}

/**
 * Create post-processing composer with bloom + vignette
 */
export function createEnhancedComposer(
  renderer: THREE.WebGLRenderer,
  scene: THREE.Scene,
  camera: THREE.Camera,
  bloomConfig: BloomConfig = DEFAULT_BLOOM_CONFIG,
  vignetteConfig: { offset: number; darkness: number } = { offset: 0.5, darkness: 0.4 }
): EffectComposer {
  const composer = new EffectComposer(renderer);

  // Render pass
  const renderPass = new RenderPass(scene, camera);
  composer.addPass(renderPass);

  // Bloom pass
  const bloomPass = new UnrealBloomPass(
    new THREE.Vector2(window.innerWidth, window.innerHeight),
    bloomConfig.strength,
    bloomConfig.radius,
    bloomConfig.threshold
  );
  composer.addPass(bloomPass);

  // Vignette pass
  const vignettePass = new ShaderPass(VignetteShader);
  vignettePass.uniforms['offset'].value = vignetteConfig.offset;
  vignettePass.uniforms['darkness'].value = vignetteConfig.darkness;
  composer.addPass(vignettePass);

  return composer;
}

/**
 * Create glow material for emissive nodes
 */
export function createGlowMaterial(
  color: string | number,
  intensity: number = 0.5
): THREE.MeshStandardMaterial {
  const hexColor = typeof color === 'string' ? parseInt(color.replace('#', ''), 16) : color;

  return new THREE.MeshStandardMaterial({
    color: hexColor,
    emissive: hexColor,
    emissiveIntensity: intensity,
    transparent: true,
    opacity: 0.9,
  });
}

/**
 * Create outer glow mesh for hover/selection effects
 */
export function createOuterGlow(
  geometry: THREE.BufferGeometry,
  color: string | number,
  scale: number = 1.3
): THREE.Mesh {
  const hexColor = typeof color === 'string' ? parseInt(color.replace('#', ''), 16) : color;

  const glowMaterial = new THREE.MeshBasicMaterial({
    color: hexColor,
    transparent: true,
    opacity: 0.15,
    side: THREE.BackSide,
  });

  // Clone and scale geometry for glow
  const glowGeometry = geometry.clone();
  const glowMesh = new THREE.Mesh(glowGeometry, glowMaterial);
  glowMesh.scale.setScalar(scale);

  return glowMesh;
}

/**
 * Create pulsing glow animation
 */
export function createPulseAnimation(
  mesh: THREE.Mesh,
  minScale: number = 1.0,
  maxScale: number = 1.2,
  duration: number = 1000
): () => void {
  const startTime = Date.now();

  const animate = () => {
    const elapsed = Date.now() - startTime;
    const phase = (elapsed % duration) / duration;
    const scale = minScale + (maxScale - minScale) * (0.5 + 0.5 * Math.sin(phase * Math.PI * 2));
    mesh.scale.setScalar(scale);
  };

  return animate;
}

/**
 * Spring physics configuration for hover effects
 */
export interface SpringConfig {
  stiffness: number;    // Spring stiffness (higher = faster)
  damping: number;      // Damping ratio (higher = less bounce)
  mass: number;         // Virtual mass
}

export const DEFAULT_SPRING: SpringConfig = {
  stiffness: 180,
  damping: 12,
  mass: 1,
};

/**
 * Simple spring physics for smooth transitions
 */
export class SpringValue {
  private target: number;
  private current: number;
  private velocity: number;
  private config: SpringConfig;

  constructor(initial: number, config: SpringConfig = DEFAULT_SPRING) {
    this.target = initial;
    this.current = initial;
    this.velocity = 0;
    this.config = config;
  }

  setTarget(value: number): void {
    this.target = value;
  }

  update(deltaTime: number): number {
    const { stiffness, damping, mass } = this.config;

    // Spring force
    const springForce = (this.target - this.current) * stiffness;
    const dampingForce = -this.velocity * damping;
    const acceleration = (springForce + dampingForce) / mass;

    // Update velocity and position
    this.velocity += acceleration * deltaTime;
    this.current += this.velocity * deltaTime;

    return this.current;
  }

  getValue(): number {
    return this.current;
  }

  isAtRest(threshold: number = 0.001): boolean {
    return Math.abs(this.target - this.current) < threshold &&
           Math.abs(this.velocity) < threshold;
  }
}

/**
 * Create edge outline geometry for wireframe effects
 */
export function createEdgeOutline(
  geometry: THREE.BufferGeometry,
  color: string | number,
  opacity: number = 0.5
): THREE.LineSegments {
  const hexColor = typeof color === 'string' ? parseInt(color.replace('#', ''), 16) : color;

  const edges = new THREE.EdgesGeometry(geometry);
  const material = new THREE.LineBasicMaterial({
    color: hexColor,
    transparent: true,
    opacity,
  });

  return new THREE.LineSegments(edges, material);
}

/**
 * Background gradient colors for space theme
 */
export const SPACE_BACKGROUND = {
  top: '#0a0a1a',      // Deep space blue
  bottom: '#000000',   // Pure black
  stars: '#ffffff',    // Star color
};

/**
 * Create starfield points for background
 */
export function createStarfield(count: number = 1000, radius: number = 500): THREE.Points {
  const geometry = new THREE.BufferGeometry();
  const positions = new Float32Array(count * 3);
  const sizes = new Float32Array(count);

  for (let i = 0; i < count; i++) {
    const theta = Math.random() * Math.PI * 2;
    const phi = Math.acos(2 * Math.random() - 1);
    const r = radius * (0.5 + 0.5 * Math.random());

    positions[i * 3] = r * Math.sin(phi) * Math.cos(theta);
    positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta);
    positions[i * 3 + 2] = r * Math.cos(phi);

    sizes[i] = Math.random() * 2;
  }

  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute('size', new THREE.BufferAttribute(sizes, 1));

  const material = new THREE.PointsMaterial({
    color: 0xffffff,
    size: 1,
    transparent: true,
    opacity: 0.6,
    sizeAttenuation: true,
  });

  return new THREE.Points(geometry, material);
}
