/**
 * Premium Card Effects
 *
 * Performance-aware visual effects for card components.
 * Inspired by MagicUI, Aceternity UI, and 21st.dev patterns.
 *
 * Effect Requirements by Tier:
 * - GlowEffect: MEDIUM+ tier
 * - ElectricBorder: ULTRA tier only
 * - Perspective3D: HIGH+ tier
 * - AuroraBackground: HIGH+ tier (TAXONOMY nodes)
 * - BorderBeam: MEDIUM+ tier
 * - HolographicOverlay: HIGH+ tier (TAXONOMY nodes)
 * - LiquidGlass: ULTRA tier only (SVG filters expensive)
 *
 * Usage:
 * ```tsx
 * import { GlowEffect, AuroraBackground, BorderBeam } from './effects';
 *
 * // Inside a card with performance context
 * {performanceTier !== 'LOW' && performanceTier !== 'MINIMAL' && (
 *   <GlowEffect color={colors.primary} intensity="medium" />
 * )}
 *
 * // TAXONOMY level (maximum wow)
 * <AuroraBackground primaryColor={color} selected={selected} />
 * <BorderBeam color={color} duration={6} />
 * <HolographicOverlay selected={selected} />
 * ```
 */

// Core effects
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

// Premium effects (v0.13.1 - wow factor)
export { AuroraBackground, type AuroraBackgroundProps } from './AuroraBackground';
export { BorderBeam, type BorderBeamProps } from './BorderBeam';
export { HolographicOverlay, type HolographicOverlayProps } from './HolographicOverlay';
export { LiquidGlass, type LiquidGlassProps } from './LiquidGlass';
export { MouseSpotlight, type MouseSpotlightProps } from './MouseSpotlight';

// Trait-specific animations (v0.13.1 - ADR-024)
export {
  TraitAnimation,
  TraitStatusDot,
  TraitBadgeAnimated,
  type TraitAnimationProps,
  type TraitStatusDotProps,
  type TraitBadgeAnimatedProps,
  type AnimatableTrait,
} from './TraitAnimation';

// Pattern backgrounds (Magic UI inspired)
export { GridPattern, DotPattern, type GridPatternProps, type DotPatternProps } from './GridPattern';

// ULTRA premium effects (v0.13.1 - maximum wow factor)
export { MatrixRain, type MatrixRainProps } from './MatrixRain';
export { Meteors, type MeteorsProps } from './Meteors';
export { LightRays, type LightRaysProps } from './LightRays';
export { ScanLines, type ScanLinesProps } from './ScanLines';
