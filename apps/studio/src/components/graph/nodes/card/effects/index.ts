/**
 * Premium Card Effects
 *
 * Performance-aware visual effects for card components.
 * Inspired by MagicUI patterns (GlassCard, ElectricCard).
 *
 * Effect Requirements:
 * - GlowEffect: MEDIUM+ tier
 * - ElectricBorder: ULTRA tier only
 * - Perspective3D: HIGH+ tier
 *
 * Usage:
 * ```tsx
 * import { GlowEffect, ElectricBorder, Perspective3D } from './effects';
 *
 * // Inside a card with performance context
 * {performanceTier !== 'LOW' && performanceTier !== 'MINIMAL' && (
 *   <GlowEffect color={colors.primary} intensity="medium" />
 * )}
 * ```
 */

export { GlowEffect, type GlowEffectProps, type GlowIntensity } from './GlowEffect';
export { ElectricBorder, type ElectricBorderProps, type ElectricStyle } from './ElectricBorder';
export { Perspective3D, type Perspective3DProps } from './Perspective3D';
export {
  TraitGlow,
  TraitIndicatorAnimated,
  getGlowMode,
  type TraitGlowProps,
  type TraitIndicatorAnimatedProps,
  type GlowMode,
} from './TraitGlow';
