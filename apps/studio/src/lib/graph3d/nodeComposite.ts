/**
 * Composite Node Builder for 3D Graph Visualization
 *
 * ATOMIC/GALAXY STYLE — Core + Orbital Rings + Particle Cloud
 *
 * Structure:
 * - Core: Layer geometry + color (solid, emissive)
 * - Rings: Realm indicator (shared=1 ring, org=2 rings)
 * - Particles: Stardust cloud orbiting the node
 *
 * Encoding:
 * - Core Shape = Layer (9 geometries)
 * - Core Color = Layer palette
 * - Ring Count = Realm (1 or 2)
 * - Ring Speed = Trait (slow→fast)
 * - Glow Intensity = Trait
 */

import * as THREE from 'three';
import type { Layer, Realm, Trait } from '@novanet/core/types';

// Trait animation speeds (ring rotation rad/s)
export const TRAIT_RING_SPEEDS: Record<Trait, number> = {
  invariant: 0.2,    // Slow, stable
  localized: 0.4,    // Medium
  knowledge: 0.3,    // Gentle
  generated: 0.8,    // Fast, active
  aggregated: 0.5,   // Medium-fast
};

// Trait glow intensities
export const TRAIT_GLOW_INTENSITY: Record<Trait, number> = {
  invariant: 0.2,    // Subtle
  localized: 0.4,    // Medium
  knowledge: 0.6,    // Higher (knowledge glows)
  generated: 0.8,    // Bright (output)
  aggregated: 0.5,   // Medium
};

// Particle count by importance (can be based on connection count)
export const BASE_PARTICLE_COUNT = 12;
export const PARTICLE_PER_CONNECTION = 2;

export interface CompositeNodeConfig {
  layer: Layer;
  realm: Realm;
  trait: Trait;
  layerColor: string;
  realmColor: string;
  connectionCount?: number;
  baseSize?: number;
}

export interface CompositeNodeMeshes {
  core: THREE.Mesh;
  rings: THREE.Mesh[];
  particles: THREE.Points;
  group: THREE.Group;
}

/**
 * Create core geometry based on layer
 * Simplified set of geometries for cleaner look
 */
function createCoreGeometry(layer: Layer, size: number): THREE.BufferGeometry {
  switch (layer) {
    case 'config':
      return new THREE.OctahedronGeometry(size * 0.9);
    case 'locale':
      return new THREE.SphereGeometry(size * 0.85, 24, 24);
    case 'geography':
      return new THREE.IcosahedronGeometry(size * 0.9);
    case 'knowledge':
      return new THREE.IcosahedronGeometry(size * 0.85);
    case 'foundation':
      return new THREE.BoxGeometry(size * 1.2, size * 1.2, size * 1.2);
    case 'structure':
      return new THREE.TetrahedronGeometry(size);
    case 'semantic':
      return new THREE.DodecahedronGeometry(size * 0.9);
    case 'instruction':
      return new THREE.ConeGeometry(size * 0.7, size * 1.3, 8);
    case 'output':
      return new THREE.SphereGeometry(size, 32, 32);
    default:
      return new THREE.SphereGeometry(size, 24, 24);
  }
}

/**
 * Create core material with emissive glow
 */
function createCoreMaterial(color: string, glowIntensity: number): THREE.MeshPhysicalMaterial {
  const hexColor = parseInt(color.replace('#', ''), 16);

  return new THREE.MeshPhysicalMaterial({
    color: hexColor,
    metalness: 0.3,
    roughness: 0.2,
    emissive: hexColor,
    emissiveIntensity: glowIntensity,
    transparent: true,
    opacity: 0.95,
  });
}

/**
 * Create orbital ring geometry
 */
function createRingGeometry(radius: number, tubeRadius: number): THREE.TorusGeometry {
  return new THREE.TorusGeometry(radius, tubeRadius, 8, 64);
}

/**
 * Create ring material (additive blending for glow)
 */
function createRingMaterial(color: string, opacity: number = 0.6): THREE.MeshBasicMaterial {
  const hexColor = parseInt(color.replace('#', ''), 16);

  return new THREE.MeshBasicMaterial({
    color: hexColor,
    transparent: true,
    opacity,
    side: THREE.DoubleSide,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
  });
}

/**
 * Create particle cloud geometry
 */
function createParticleGeometry(count: number, radius: number): THREE.BufferGeometry {
  const positions = new Float32Array(count * 3);
  const sizes = new Float32Array(count);

  for (let i = 0; i < count; i++) {
    // Distribute in spherical shell around node
    const theta = Math.random() * Math.PI * 2;
    const phi = Math.acos(2 * Math.random() - 1);
    const r = radius * (0.8 + Math.random() * 0.4); // 80-120% of radius

    positions[i * 3] = r * Math.sin(phi) * Math.cos(theta);
    positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta);
    positions[i * 3 + 2] = r * Math.cos(phi);

    sizes[i] = 0.5 + Math.random() * 1.5; // Varying sizes
  }

  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute('size', new THREE.BufferAttribute(sizes, 1));

  return geometry;
}

/**
 * Create particle material
 */
function createParticleMaterial(color: string): THREE.PointsMaterial {
  const hexColor = parseInt(color.replace('#', ''), 16);

  return new THREE.PointsMaterial({
    color: hexColor,
    size: 1.5,
    transparent: true,
    opacity: 0.7,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
    sizeAttenuation: true,
  });
}

/**
 * Build complete composite node with core, rings, and particles
 */
export function createCompositeNode(config: CompositeNodeConfig): CompositeNodeMeshes {
  const {
    layer,
    realm,
    trait,
    layerColor,
    realmColor,
    connectionCount = 0,
    baseSize = 8,
  } = config;

  const group = new THREE.Group();

  // 1. Create core
  const coreGeometry = createCoreGeometry(layer, baseSize);
  const glowIntensity = TRAIT_GLOW_INTENSITY[trait] || 0.3;
  const coreMaterial = createCoreMaterial(layerColor, glowIntensity);
  const core = new THREE.Mesh(coreGeometry, coreMaterial);
  group.add(core);

  // 2. Create rings based on realm
  const rings: THREE.Mesh[] = [];
  const ringCount = realm === 'org' ? 2 : 1;
  const ringRadius = baseSize * 1.4;
  const tubeRadius = baseSize * 0.08;

  for (let i = 0; i < ringCount; i++) {
    const ringGeometry = createRingGeometry(
      ringRadius * (1 - i * 0.15), // Slightly smaller inner ring
      tubeRadius
    );
    const ringMaterial = createRingMaterial(realmColor, 0.5 - i * 0.1);
    const ring = new THREE.Mesh(ringGeometry, ringMaterial);

    // Different orientations for multiple rings
    if (i === 0) {
      ring.rotation.x = Math.PI / 2; // Horizontal (equator)
    } else {
      ring.rotation.x = Math.PI / 3; // Tilted
      ring.rotation.z = Math.PI / 4;
    }

    rings.push(ring);
    group.add(ring);
  }

  // 3. Create particle cloud
  const particleCount = BASE_PARTICLE_COUNT + connectionCount * PARTICLE_PER_CONNECTION;
  const particleRadius = baseSize * 1.8;
  const particleGeometry = createParticleGeometry(particleCount, particleRadius);
  const particleMaterial = createParticleMaterial(layerColor);
  const particles = new THREE.Points(particleGeometry, particleMaterial);
  group.add(particles);

  // Store trait for animation updates
  group.userData = { trait, ringSpeed: TRAIT_RING_SPEEDS[trait] || 0.3 };

  return { core, rings, particles, group };
}

/**
 * Animate composite node (call in render loop)
 */
export function animateCompositeNode(
  meshes: CompositeNodeMeshes,
  deltaTime: number,
  elapsedTime: number
): void {
  const { rings, particles, group } = meshes;
  const ringSpeed = (group.userData.ringSpeed as number) || 0.3;

  // Rotate rings
  rings.forEach((ring, i) => {
    const direction = i % 2 === 0 ? 1 : -1;
    ring.rotation.z += ringSpeed * deltaTime * direction;
  });

  // Rotate particle cloud slowly
  particles.rotation.y += 0.1 * deltaTime;
  particles.rotation.x += 0.05 * deltaTime;

  // Pulse effect (subtle scale oscillation)
  const pulse = 1 + Math.sin(elapsedTime * 2) * 0.03;
  meshes.core.scale.setScalar(pulse);
}

/**
 * Create a simple preview node for sidebar (smaller, optimized)
 */
export function createPreviewNode(config: CompositeNodeConfig): THREE.Group {
  const previewConfig = {
    ...config,
    baseSize: (config.baseSize || 8) * 0.6, // Smaller for preview
  };

  const { group } = createCompositeNode(previewConfig);
  return group;
}

/**
 * Dispose of composite node meshes
 */
export function disposeCompositeNode(meshes: CompositeNodeMeshes): void {
  meshes.core.geometry.dispose();
  (meshes.core.material as THREE.Material).dispose();

  meshes.rings.forEach((ring) => {
    ring.geometry.dispose();
    (ring.material as THREE.Material).dispose();
  });

  meshes.particles.geometry.dispose();
  (meshes.particles.material as THREE.Material).dispose();
}
