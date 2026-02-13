/**
 * Effects Index
 *
 * Main export point for the edge effects system.
 * v11.6.2: Added 5 signature effects for arc families.
 *
 * @see docs/plans/2026-02-11-arc-effects-redesign.md
 */

// ============================================================================
// SIGNATURE EFFECTS (v11.6.2 - Full Redesign)
// ============================================================================

// Each arc family has a unique, signature effect that conveys semantic meaning
export { PowerConduit } from './PowerConduit';       // Ownership: High-voltage power conduit
export { DNAHelix } from './DNAHelix';               // Localization: DNA helix adaptation
export { SynapticFiring } from './SynapticFiring';   // Semantic: Neural synaptic firing
export { MatrixCodeRain } from './MatrixCodeRain';   // Generation: Matrix code rain
export { SonarPulse } from './SonarPulse';           // Mining: Sonar pulse discovery

// ============================================================================
// SELECTION EFFECT (v11.6.5 - Selection UX Polish)
// ============================================================================

export { SelectionEffect } from './SelectionEffect'; // Direction, differentiation, transitions

// ============================================================================
// EFFECT RENDERER
// ============================================================================

export { EffectRenderer, releaseEdgeAnimationSlot, shouldRenderEffect } from './EffectRenderer';
export type { EffectRendererProps, BatchEffectConfig } from './EffectRenderer';

// ============================================================================
// EFFECT PRIMITIVES
// ============================================================================

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
