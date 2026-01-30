/**
 * Effect Primitives Index
 *
 * Exports all atomic effect primitives and the primitive registry
 */

import type { ComponentType } from 'react';
import type { EffectPrimitive, EffectPrimitiveProps } from '../../system/types';

// Export all primitives
export { EmitPrimitive } from './EmitPrimitive';
export { ImpactPrimitive } from './ImpactPrimitive';
export { GlowPrimitive } from './GlowPrimitive';
export { ParticlesPrimitive } from './ParticlesPrimitive';
export { TrailPrimitive } from './TrailPrimitive';
export { ZigzagPrimitive } from './ZigzagPrimitive';
export { InterferencePrimitive } from './InterferencePrimitive';
export { ScanlinePrimitive } from './ScanlinePrimitive';

// Import for registry
import { EmitPrimitive } from './EmitPrimitive';
import { ImpactPrimitive } from './ImpactPrimitive';
import { GlowPrimitive } from './GlowPrimitive';
import { ParticlesPrimitive } from './ParticlesPrimitive';
import { TrailPrimitive } from './TrailPrimitive';
import { ZigzagPrimitive } from './ZigzagPrimitive';
import { InterferencePrimitive } from './InterferencePrimitive';
import { ScanlinePrimitive } from './ScanlinePrimitive';

/**
 * Registry mapping primitive types to their React components
 */
export const PRIMITIVE_REGISTRY: Record<EffectPrimitive, ComponentType<EffectPrimitiveProps>> = {
  emit: EmitPrimitive,
  particles: ParticlesPrimitive,
  trail: TrailPrimitive,
  impact: ImpactPrimitive,
  glow: GlowPrimitive,
  zigzag: ZigzagPrimitive,
  interference: InterferencePrimitive,
  scanline: ScanlinePrimitive,
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
