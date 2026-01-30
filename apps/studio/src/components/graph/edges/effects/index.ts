/**
 * Effects Index
 *
 * Main export point for the edge effects system
 */

// Renderer
export { EffectRenderer, releaseEdgeAnimationSlot, shouldRenderEffect } from './EffectRenderer';
export type { EffectRendererProps, BatchEffectConfig } from './EffectRenderer';

// Primitives
export {
  EmitPrimitive,
  ImpactPrimitive,
  GlowPrimitive,
  ParticlesPrimitive,
  TrailPrimitive,
  ZigzagPrimitive,
  InterferencePrimitive,
  ScanlinePrimitive,
  PRIMITIVE_REGISTRY,
  getPrimitive,
  hasPrimitive,
} from './primitives';
