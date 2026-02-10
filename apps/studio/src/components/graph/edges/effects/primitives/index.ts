/**
 * Effect Primitives Index
 *
 * Exports all atomic effect primitives and the primitive registry
 *
 * v11.6.1: Added 4 family-specific effect primitives
 */

import type { ComponentType } from 'react';
import type { EffectPrimitive, EffectPrimitiveProps } from '../../system/types';

// Export all primitives - Core (v11.6.0)
export { EmitPrimitive } from './EmitPrimitive';
export { ImpactPrimitive } from './ImpactPrimitive';
export { GlowPrimitive } from './GlowPrimitive';
export { ParticlesPrimitive } from './ParticlesPrimitive';
export { TrailPrimitive } from './TrailPrimitive';
export { ZigzagPrimitive } from './ZigzagPrimitive';
export { InterferencePrimitive } from './InterferencePrimitive';
export { ScanlinePrimitive } from './ScanlinePrimitive';

// Export all primitives - Family-specific (v11.6.1)
export { EnergyPulsePrimitive } from './EnergyPulsePrimitive';
export { DNAHelixPrimitive } from './DNAHelixPrimitive';
export { MatrixCodePrimitive } from './MatrixCodePrimitive';
export { RadarSweepPrimitive } from './RadarSweepPrimitive';

// Import for registry - Core
import { EmitPrimitive } from './EmitPrimitive';
import { ImpactPrimitive } from './ImpactPrimitive';
import { GlowPrimitive } from './GlowPrimitive';
import { ParticlesPrimitive } from './ParticlesPrimitive';
import { TrailPrimitive } from './TrailPrimitive';
import { ZigzagPrimitive } from './ZigzagPrimitive';
import { InterferencePrimitive } from './InterferencePrimitive';
import { ScanlinePrimitive } from './ScanlinePrimitive';

// Import for registry - Family-specific
import { EnergyPulsePrimitive } from './EnergyPulsePrimitive';
import { DNAHelixPrimitive } from './DNAHelixPrimitive';
import { MatrixCodePrimitive } from './MatrixCodePrimitive';
import { RadarSweepPrimitive } from './RadarSweepPrimitive';

/**
 * Registry mapping primitive types to their React components
 *
 * Note: Family-specific primitives have extended props but conform
 * to EffectPrimitiveProps base interface for the registry.
 */
export const PRIMITIVE_REGISTRY: Record<EffectPrimitive, ComponentType<EffectPrimitiveProps>> = {
  // Core primitives
  emit: EmitPrimitive,
  particles: ParticlesPrimitive,
  trail: TrailPrimitive,
  impact: ImpactPrimitive,
  glow: GlowPrimitive,
  zigzag: ZigzagPrimitive,
  interference: InterferencePrimitive,
  scanline: ScanlinePrimitive,
  // Family-specific primitives (v11.6.1)
  energyPulse: EnergyPulsePrimitive,
  dnaHelix: DNAHelixPrimitive,
  matrixCode: MatrixCodePrimitive,
  radarSweep: RadarSweepPrimitive,
};

/**
 * Get a primitive component by type
 */
export function getPrimitive(type: EffectPrimitive): ComponentType<EffectPrimitiveProps> {
  return PRIMITIVE_REGISTRY[type];
}

/**
 * Check if a primitive type exists
 */
export function hasPrimitive(type: string): type is EffectPrimitive {
  return type in PRIMITIVE_REGISTRY;
}
