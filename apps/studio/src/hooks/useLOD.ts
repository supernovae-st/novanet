/**
 * useLOD - Level of Detail Hook for React Flow
 *
 * Provides zoom-based LOD tier selection for graph visualization.
 * Returns the appropriate LODTier based on current viewport zoom.
 *
 * Part of Phase 3: Advanced Performance Optimizations (Task 3.2).
 *
 * @example
 * const lod = useLOD();
 * if (lod.particles) {
 *   renderParticles();
 * }
 *
 * @example
 * // Conditional rendering based on LOD
 * const lod = useLOD();
 * return (
 *   <div>
 *     {lod.labels && <Label>{node.data.label}</Label>}
 *     {lod.energyEffects && <EnergyRing />}
 *   </div>
 * );
 */

import { useMemo } from 'react';
import { useStore } from '@xyflow/react';
import { getLODTier, type LODTier } from '@/components/graph/config/LODConfig';

/**
 * Hook that returns the current LOD tier based on React Flow's zoom level.
 *
 * @returns The LOD tier configuration for the current zoom level
 */
export function useLOD(): LODTier {
  const zoom = useStore((state) => state.transform[2]);
  return useMemo(() => getLODTier(zoom), [zoom]);
}
