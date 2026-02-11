/**
 * Graph 3D Utilities
 *
 * Utilities for 3D graph visualization with react-force-graph-3d.
 */

export * from './colorPalette';
export * from './geometryFactory';
export * from './arcParticles';
export * from './dataTransform';
export * from './postProcessing';
export * from './nodeComposite';

// Arc effects - explicit exports to avoid naming conflicts with arcParticles
export {
  // Classes
  ArcLODManager,
  SimpleLine3D,
  PowerConduit3D,
  SynapticFiring3D,
  DNAHelix3D,
  MatrixRain3D,
  SonarPulse3D,
  // Constants
  LOD_THRESHOLDS,
  // Shader utilities
  COMMON_GLSL,
  createArcTubeGeometry,
  createCurvedArcPath,
  parseColor,
  createBaseUniforms,
  // Types
  type ArcEffect3D,
  type ArcEffectConfig,
  type ArcEffectFactory,
  type LODLevel,
  // Note: ArcFamily type is from arcParticles (which re-exports from palette)
  // ARC_FAMILY_COLORS is from colorPalette (which re-exports from palette)
} from './arcEffects';
