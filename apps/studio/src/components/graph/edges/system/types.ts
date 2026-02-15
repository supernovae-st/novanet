/**
 * Edge Animation System - Type Definitions
 *
 * Discriminated unions for complete type safety.
 * Zero magic strings - all values are typed.
 */

// =============================================================================
// Relation Types (from Neo4j schema)
// =============================================================================

/**
 * Neo4j relation categories - maps to visual themes
 */
export type RelationCategory =
  | 'structural'    // HAS_*, CONTAINS
  | 'localization'  // FOR_LOCALE, SUPPORTS, HAS_NATIVE
  | 'generation'    // HAS_NATIVE, HAS_INSTRUCTION, HAS_RULES
  | 'semantic'      // USES_ENTITY, SEMANTIC
  | 'seo'           // TARGETS_SEO, HAS_VARIATION
  | 'geo'           // TARGETS_GEO, HAS_CITATION
  | 'reference';    // USES, MODEL, PROVIDER, FALLBACK

/**
 * All known relation types (from Neo4j schema)
 */
export type RelationType =
  // Structural (v10.3: HAS_CONCEPT removed)
  | 'HAS_PAGE'
  | 'HAS_BLOCK'
  | 'HAS_AUDIENCE'
  | 'CONTAINS'
  // Localization
  | 'HAS_NATIVE'
  | 'SUPPORTS_LOCALE'
  | 'FOR_LOCALE'
  // Generation
  | 'HAS_NATIVE'
  | 'HAS_INSTRUCTION'
  | 'HAS_RULES'
  | 'GENERATED_BY'
  // Semantic
  | 'USES_ENTITY'
  | 'OF_TYPE'
  | 'SEMANTIC'
  // SEO
  | 'TARGETS_SEO'
  | 'HAS_VARIATION'
  | 'HAS_SNAPSHOT'
  // GEO
  | 'TARGETS_GEO'
  | 'HAS_CITATION'
  | 'HAS_REFORMULATION'
  // Reference
  | 'USES'
  | 'FALLBACK_TO'
  | 'MODEL'
  | 'PROVIDER';

// =============================================================================
// Effect Primitives
// =============================================================================

/**
 * Atomic effect building blocks - composable primitives
 *
 * v11.6.1: Added 4 new family-specific effects
 */
export type EffectPrimitive =
  // Core effects (v11.6.0)
  | 'emit'          // Pulse/burst at source
  | 'particles'     // Traveling data packets
  | 'trail'         // Comet tail behind particles
  | 'impact'        // Burst at target
  | 'glow'          // Edge glow layer
  | 'zigzag'        // Neural branching sparks (semantic family)
  | 'interference'  // Wave overlay pattern
  | 'scanline'      // Holographic scan effect
  // Family-specific effects (v11.6.1)
  | 'energyPulse'   // Intense power packets with glow trail (ownership)
  | 'dnaHelix'      // Double helix spiral animation (localization)
  | 'matrixCode'    // Flowing code characters (generation)
  | 'radarSweep';   // Gradient sweep like radar (mining)

/**
 * Animation speed presets
 */
export type AnimationSpeed = 'slow' | 'normal' | 'fast' | 'ultra';

/**
 * Line style variants
 */
export type LineStyle = 'solid' | 'dashed' | 'dotted' | 'double' | 'triple' | 'zigzag';

/**
 * Particle animation presets (composed from primitives)
 */
export type ParticlePreset =
  | 'flow'    // Simple directional flow
  | 'pulse'   // Pulsing rings
  | 'wave'    // Dense wave of particles
  | 'spark'   // Explosive sparks
  | 'plasma'  // Electric plasma discharge
  | 'helix'   // DNA double helix
  | 'orbit'   // Orbiting satellites
  | 'aurora'; // Color-shifting aurora

// =============================================================================
// Edge States
// =============================================================================

/**
 * Edge interaction states
 */
export type EdgeState = 'default' | 'selected' | 'highlighted' | 'muted';

/**
 * Edge priority for animation budget
 */
export type EdgePriority = 'selected' | 'highlighted' | 'connected' | 'default';

/**
 * LOD (Level of Detail) tiers
 */
export type LODTier = 'high' | 'medium' | 'low' | 'minimal';

// =============================================================================
// Configuration Interfaces
// =============================================================================

/**
 * Color palette for an edge theme
 */
export interface ColorPalette {
  /** Main color */
  primary: string;
  /** Secondary accent */
  secondary: string;
  /** Tertiary highlight */
  tertiary: string;
  /** Glow/blur color */
  glow: string;
}

/**
 * Timing configuration for animations
 */
export interface TimingConfig {
  /** Base duration in seconds */
  duration: number;
  /** Speed preset */
  speed: AnimationSpeed;
  /** Delay between particles (0-1 ratio of duration) */
  stagger: number;
  /** CSS easing or SVG spline */
  easing: string;
}

/**
 * Size configuration for effects
 */
export interface SizeConfig {
  /** Base particle size in pixels */
  particleSize: number;
  /** Number of particles */
  particleCount: number;
  /** Glow radius multiplier */
  glowMultiplier: number;
  /** Trail length multiplier */
  trailLength: number;
}

/**
 * Complete edge theme configuration
 */
export interface EdgeTheme {
  /** Color palette */
  palette: ColorPalette;
  /** Effect primitives to render */
  effects: EffectPrimitive[];
  /** Line stroke style */
  lineStyle: LineStyle;
  /** Stroke width in pixels */
  strokeWidth: number;
  /** Particle behavior preset */
  particlePreset: ParticlePreset;
  /** Animation speed */
  speed: AnimationSpeed;
  /** Glow intensity (0-1) */
  glowIntensity: number;
}

/**
 * Arc family classification (v9.5)
 */
export type ArcFamilyType =
  | 'ownership'      // Structural parent→child containment
  | 'localization'   // Invariant↔locale-specific bridges
  | 'semantic'       // Concept connections + spreading activation
  | 'generation'     // LLM pipeline: prompts → outputs
  | 'mining';        // SEO/GEO targeting and metrics

/**
 * Resolved theme (after merging category base + relation overrides)
 */
export interface ResolvedEdgeTheme extends EdgeTheme {
  /** Source category */
  category: RelationCategory;
  /** Arc family (v9.5 - primary classification) */
  arcFamily?: ArcFamilyType;
  /** Original relation type */
  relationType: string;
  /** Computed timing config */
  timing: TimingConfig;
  /** Computed size config */
  sizes: SizeConfig;
  /** Resolved colors (alias for palette for easier access) */
  colors: ColorPalette;
}

// =============================================================================
// Effect Props Interfaces
// =============================================================================

/**
 * Position in 2D space
 */
export interface Position {
  x: number;
  y: number;
}

/**
 * Common props for all effect primitives
 */
export interface EffectPrimitiveProps {
  /** SVG path ID to animate along */
  pathId: string;
  /** Reversed path ID (for bidirectional effects) */
  reversedPathId?: string;
  /** Color palette */
  colors: ColorPalette;
  /** Timing configuration */
  timing: TimingConfig;
  /** Intensity 0-1 (controlled by LOD) */
  intensity: number;
  /** Current interaction state */
  state: EdgeState;
  /** Source node position */
  sourcePosition: Position;
  /** Target node position */
  targetPosition: Position;
}

/**
 * Props for the EffectRenderer component
 */
export interface EffectRendererProps extends Omit<EffectPrimitiveProps, 'pathId'> {
  /** Effect stack to render */
  effects: EffectPrimitive[];
  /** Primary path ID */
  pathId: string;
  /** Reversed path ID */
  reversedPathId: string;
  /** Particle preset for particle-based effects */
  particlePreset: ParticlePreset;
  /** Size configuration */
  sizes: SizeConfig;
}

// =============================================================================
// Performance Interfaces
// =============================================================================

/**
 * LOD configuration per tier
 */
export interface LODConfig {
  /** Which effects to render (or preset) */
  effects: EffectPrimitive[] | 'ALL' | 'CORE' | 'GLOW' | 'NONE';
  /** Maximum particles to render */
  maxParticles: number;
  /** Whether to enable glow effects */
  enableGlow: boolean;
  /** Target FPS for this tier */
  targetFPS: number;
}

/**
 * Animation budget configuration
 */
export interface AnimationBudgetConfig {
  /** Maximum edges animated concurrently */
  maxConcurrent: number;
  /** Priority values per edge state */
  priorities: Record<EdgePriority, number>;
}

// =============================================================================
// Registry Types
// =============================================================================

/**
 * Primitive component type
 */
export type PrimitiveComponent = React.ComponentType<EffectPrimitiveProps>;

/**
 * Primitive registry mapping
 */
export type PrimitiveRegistry = Record<EffectPrimitive, PrimitiveComponent>;

/**
 * Theme override (partial - only specify what differs)
 */
export type ThemeOverride = Partial<Omit<EdgeTheme, 'palette'>> & {
  palette?: Partial<ColorPalette>;
};

/**
 * Relation overrides registry
 */
export type RelationOverrides = Partial<Record<RelationType, ThemeOverride>>;
