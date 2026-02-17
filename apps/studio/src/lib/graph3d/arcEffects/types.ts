// apps/studio/src/lib/graph3d/arcEffects/types.ts
import * as THREE from 'three';

export type ArcFamily = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining' | 'schema';

export type LODLevel = 'ultra' | 'high' | 'medium' | 'low';

export interface ArcEffectConfig {
  sourcePosition: THREE.Vector3;
  targetPosition: THREE.Vector3;
  family: ArcFamily;
  color: string;
  isSelected?: boolean;
  isHovered?: boolean;
}

export interface ArcEffect3D {
  /** The THREE.Group containing all effect meshes */
  group: THREE.Group;
  /** Current LOD level */
  lodLevel: LODLevel;
  /** Update positions when nodes move */
  updatePositions(source: THREE.Vector3, target: THREE.Vector3): void;
  /** Update shader uniforms (call each frame) */
  updateUniforms(time: number, deltaTime: number): void;
  /** Switch to different LOD level */
  setLOD(level: LODLevel): void;
  /** Clean up resources */
  dispose(): void;
}

export interface ArcEffectFactory {
  create(config: ArcEffectConfig): ArcEffect3D;
}

export const LOD_THRESHOLDS = {
  ultra: 150,   // 0-150: full shader + particles + glow
  high: 400,    // 150-400: simplified shader + particles
  medium: 800,  // 400-800: tube glow only
  // 800+: low (simple line)
} as const;

export const ARC_FAMILY_COLORS: Record<ArcFamily, string> = {
  ownership: '#3b82f6',     // blue-500
  localization: '#22c55e',  // green-500
  semantic: '#f97316',      // orange-500
  generation: '#8b5cf6',    // violet-500
  mining: '#ec4899',        // pink-500
  schema: '#6366f1',        // indigo-500
};
