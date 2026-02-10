/**
 * Geometry Factory for 3D Graph Visualization
 *
 * Creates Three.js geometries based on node classification:
 * - Layer determines shape (9 unique geometries)
 * - Trait determines material style
 * - Realm determines outline color
 */

import * as THREE from 'three';
import type { Layer } from '@novanet/core/types';

// Layer to geometry mapping (9 unique shapes)
export type GeometryType =
  | 'octahedron'    // config
  | 'sphereRings'   // locale (sphere with torus rings)
  | 'icosahedron'   // geography
  | 'torus'         // knowledge
  | 'box'           // foundation
  | 'tetrahedron'   // structure
  | 'dodecahedron'  // semantic
  | 'cone'          // instruction
  | 'sphereGlow';   // output (sphere with bloom)

// Map layer to geometry type
export const LAYER_GEOMETRY_MAP: Record<Layer, GeometryType> = {
  config: 'octahedron',
  locale: 'sphereRings',
  geography: 'icosahedron',
  knowledge: 'torus',
  foundation: 'box',
  structure: 'tetrahedron',
  semantic: 'dodecahedron',
  instruction: 'cone',
  output: 'sphereGlow',
};

// Geometry size multipliers for visual balance
const SIZE_MULTIPLIERS: Record<GeometryType, number> = {
  octahedron: 1.0,
  sphereRings: 1.0,
  icosahedron: 1.0,
  torus: 0.8,
  box: 0.9,
  tetrahedron: 1.1,
  dodecahedron: 0.95,
  cone: 1.0,
  sphereGlow: 1.0,
};

/**
 * Create a Three.js geometry based on layer classification
 */
export function createGeometryForLayer(layer: Layer, baseSize = 5): THREE.BufferGeometry {
  const geometryType = LAYER_GEOMETRY_MAP[layer];
  const size = baseSize * (SIZE_MULTIPLIERS[geometryType] || 1.0);

  switch (geometryType) {
    case 'octahedron':
      return new THREE.OctahedronGeometry(size);

    case 'sphereRings':
      // Base sphere - torus rings added in renderer
      return new THREE.SphereGeometry(size * 0.8, 16, 16);

    case 'icosahedron':
      return new THREE.IcosahedronGeometry(size);

    case 'torus':
      return new THREE.TorusGeometry(size * 0.7, size * 0.25, 8, 24);

    case 'box':
      return new THREE.BoxGeometry(size * 1.4, size * 1.4, size * 1.4);

    case 'tetrahedron':
      return new THREE.TetrahedronGeometry(size * 1.2);

    case 'dodecahedron':
      return new THREE.DodecahedronGeometry(size);

    case 'cone':
      return new THREE.ConeGeometry(size * 0.7, size * 1.4, 6);

    case 'sphereGlow':
      // Base sphere - glow effect added in renderer
      return new THREE.SphereGeometry(size, 16, 16);

    default:
      return new THREE.SphereGeometry(size, 16, 16);
  }
}

/**
 * Create torus rings for locale nodes (globe-like appearance)
 */
export function createTorusRings(baseSize = 5): THREE.BufferGeometry[] {
  const rings: THREE.BufferGeometry[] = [];

  // Equator ring
  rings.push(new THREE.TorusGeometry(baseSize * 0.85, baseSize * 0.05, 4, 32));

  // Latitude rings (30° and 60°)
  const latitudes = [0.5, 0.866]; // sin(30°) and sin(60°)
  latitudes.forEach((lat) => {
    rings.push(
      new THREE.TorusGeometry(baseSize * 0.85 * lat, baseSize * 0.03, 4, 24)
    );
  });

  return rings;
}

/**
 * Get geometry type description for UI
 */
export function getGeometryDescription(layer: Layer): string {
  const descriptions: Record<Layer, string> = {
    config: 'Octahedron (8-faced crystal)',
    locale: 'Globe with latitude rings',
    geography: 'Icosahedron (20-faced)',
    knowledge: 'Torus (wisdom ring)',
    foundation: 'Cube (stable base)',
    structure: 'Tetrahedron (pyramid)',
    semantic: 'Dodecahedron (12-faced)',
    instruction: 'Cone (directional)',
    output: 'Sphere with glow',
  };

  return descriptions[layer] || 'Sphere';
}
