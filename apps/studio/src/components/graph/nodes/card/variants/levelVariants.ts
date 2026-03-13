/**
 * Level Variants - Visual encoding for 3-level graph architecture
 *
 * Based on research from 5 parallel agents analyzing:
 * - MOF 4-layer architecture (M3/M2/M1/M0)
 * - Graph visualization best practices (Neo4j, Protege, Gephi)
 * - UI component patterns (21st.dev, Raycast)
 * - NovaNet-specific requirements (ADR-005)
 *
 * 3 Levels:
 * - TAXONOMY (21 nodes): Realm, Layer, Trait, ArcFamily — the classification system
 * - SCHEMA (218 nodes): 59 NodeClass + 159 ArcClass — definitions (v0.18.0)
 * - DATA (∞ instances): Runtime data instances — actual content
 *
 * Visual Encoding Matrix:
 * | Channel       | TAXONOMY        | SCHEMA          | DATA            |
 * |---------------|-----------------|-----------------|-----------------|
 * | Border Width  | 4px             | 2px             | 1px             |
 * | Shadow Depth  | Floating (16px) | Elevated (8px)  | Flat (2px)      |
 * | Aura Rings    | Triple glow     | Single glow     | None            |
 * | Animation     | Always pulsing  | On interaction  | Hover only      |
 * | Width Range   | 200-280px       | 160-200px       | 120-180px       |
 * | Corner Radius | 16px (xl)       | 12px (lg)       | 8px (md)        |
 * | Badge Style   | Full banner [T] | Corner [S]      | Inline [D]      |
 */

import type { NodeLevel } from '../types';

// =============================================================================
// Level Visual Constants
// =============================================================================

export interface LevelVisualConfig {
  /** Border width in pixels */
  borderWidth: number;
  /** Border width Tailwind class */
  borderWidthClass: string;
  /** Box shadow depth (layer effect) */
  shadow: string;
  /** Glow/aura effect */
  glow: string;
  /** Corner radius in pixels */
  borderRadius: number;
  /** Corner radius Tailwind class */
  borderRadiusClass: string;
  /** Card width range [min, max] */
  widthRange: [number, number];
  /** Default card width */
  defaultWidth: number;
  /** Badge style variant */
  badgeStyle: 'banner' | 'corner' | 'inline';
  /** Level badge text */
  badgeText: string;
  /** Animation behavior */
  animation: 'always' | 'interaction' | 'hover';
  /** Opacity for the card (relative prominence) */
  opacity: number;
  /** Z-index boost for layering */
  zIndexBoost: number;
}

/**
 * Visual configuration for each abstraction level
 */
export const LEVEL_VISUALS: Record<NodeLevel, LevelVisualConfig> = {
  // ─────────────────────────────────────────────────────────────────────────
  // TAXONOMY: Meta-meta level (M2) — The classification system itself
  // Prominent, architectural look with heavy borders and floating effect
  // ─────────────────────────────────────────────────────────────────────────
  taxonomy: {
    borderWidth: 4,
    borderWidthClass: 'border-4',
    shadow: '0 16px 48px -8px rgba(0,0,0,0.5), 0 8px 24px -4px rgba(0,0,0,0.3)',
    glow: '0 0 32px rgba(var(--glow-rgb), 0.3), 0 0 64px rgba(var(--glow-rgb), 0.15), 0 0 96px rgba(var(--glow-rgb), 0.08)',
    borderRadius: 16,
    borderRadiusClass: 'rounded-2xl',
    widthRange: [200, 280],
    defaultWidth: 240,
    badgeStyle: 'banner',
    badgeText: 'TAXONOMY',
    animation: 'always',
    opacity: 1.0,
    zIndexBoost: 20,
  },

  // ─────────────────────────────────────────────────────────────────────────
  // SCHEMA: Meta level (M1) — Definitions of what CAN exist
  // Elevated, blueprint-like appearance
  // ─────────────────────────────────────────────────────────────────────────
  schema: {
    borderWidth: 2,
    borderWidthClass: 'border-2',
    shadow: '0 8px 24px -4px rgba(0,0,0,0.4), 0 4px 12px -2px rgba(0,0,0,0.2)',
    glow: '0 0 16px rgba(var(--glow-rgb), 0.2)',
    borderRadius: 12,
    borderRadiusClass: 'rounded-xl',
    widthRange: [160, 200],
    defaultWidth: 180,
    badgeStyle: 'corner',
    badgeText: 'SCHEMA',
    animation: 'interaction',
    opacity: 0.95,
    zIndexBoost: 10,
  },

  // ─────────────────────────────────────────────────────────────────────────
  // DATA: Instance level (M0) — Actual runtime instances
  // Flat, content-focused appearance
  // ─────────────────────────────────────────────────────────────────────────
  data: {
    borderWidth: 1,
    borderWidthClass: 'border',
    shadow: '0 2px 8px -1px rgba(0,0,0,0.3), 0 1px 4px -1px rgba(0,0,0,0.2)',
    glow: 'none',
    borderRadius: 8,
    borderRadiusClass: 'rounded-lg',
    widthRange: [120, 180],
    defaultWidth: 160,
    badgeStyle: 'inline',
    badgeText: 'DATA',
    animation: 'hover',
    opacity: 0.9,
    zIndexBoost: 0,
  },
};

// =============================================================================
// Level-Specific Styling Functions
// =============================================================================

/**
 * Get visual configuration for a node level
 */
export function getLevelVisuals(level: NodeLevel): LevelVisualConfig {
  return LEVEL_VISUALS[level];
}

/**
 * Get shadow style for a level (inline CSS)
 */
export function getLevelShadow(level: NodeLevel, glowColor?: string): string {
  const config = LEVEL_VISUALS[level];
  let shadow = config.shadow;

  // Add glow effect if color provided
  if (glowColor && config.glow !== 'none') {
    const glowWithColor = config.glow.replace(/var\(--glow-rgb\)/g, glowColor);
    shadow = `${shadow}, ${glowWithColor}`;
  }

  return shadow;
}

/**
 * Get Tailwind classes for level-based styling
 */
export function getLevelClasses(level: NodeLevel): string {
  const config = LEVEL_VISUALS[level];
  return `${config.borderWidthClass} ${config.borderRadiusClass}`;
}

// =============================================================================
// Level Badge Components
// =============================================================================

export interface LevelBadgeConfig {
  /** Badge position classes */
  position: string;
  /** Badge size classes */
  size: string;
  /** Badge background */
  background: string;
  /** Badge text color */
  textColor: string;
  /** Font size */
  fontSize: string;
  /** Letter spacing */
  letterSpacing: string;
  /** Icon (Unicode) */
  icon: string;
}

export const LEVEL_BADGES: Record<NodeLevel, LevelBadgeConfig> = {
  taxonomy: {
    position: 'absolute -top-2 left-1/2 -translate-x-1/2',
    size: 'px-3 py-1',
    background: 'bg-gradient-to-r from-violet-600/90 to-purple-600/90 backdrop-blur-sm',
    textColor: 'text-white',
    fontSize: 'text-[10px]',
    letterSpacing: 'tracking-widest',
    icon: '◈', // Diamond
  },
  schema: {
    position: 'absolute top-2 right-2',
    size: 'px-2 py-0.5',
    background: 'bg-slate-700/80 backdrop-blur-sm',
    textColor: 'text-slate-200',
    fontSize: 'text-[9px]',
    letterSpacing: 'tracking-wide',
    icon: '◇', // Hollow diamond
  },
  data: {
    position: '', // Inline with header
    size: 'px-1.5 py-0.5',
    background: 'bg-white/10',
    textColor: 'text-white/60',
    fontSize: 'text-[8px]',
    letterSpacing: 'tracking-normal',
    icon: '○', // Circle
  },
};

/**
 * Get badge configuration for a level
 */
export function getLevelBadgeConfig(level: NodeLevel): LevelBadgeConfig {
  return LEVEL_BADGES[level];
}

// =============================================================================
// Animation Presets per Level
// =============================================================================

export interface LevelAnimationConfig {
  /** Initial state (hidden) */
  initial: Record<string, number | string>;
  /** Visible state */
  visible: Record<string, number | string>;
  /** Hover state */
  hover: Record<string, number | string>;
  /** Selected state */
  selected: Record<string, number | string>;
  /** Idle animation (for taxonomy: always pulsing) */
  idle?: Record<string, unknown>;
  /** Transition config */
  transition: {
    type: string;
    stiffness: number;
    damping: number;
  };
}

// CRITICAL: All animation states MUST include opacity to prevent nodes disappearing
// when transitioning between states. If opacity is missing, Framer Motion may
// interpolate from the initial opacity: 0 state.
export const LEVEL_ANIMATIONS: Record<NodeLevel, LevelAnimationConfig> = {
  taxonomy: {
    initial: { opacity: 1, scale: 1, y: 0 }, // Start visible to prevent flicker
    visible: { opacity: 1, scale: 1, y: 0 },
    hover: { opacity: 1, scale: 1, y: -2 }, // No scale to prevent layout shift
    selected: { opacity: 1, scale: 1, y: -3 },
    idle: {
      boxShadow: [
        '0 0 20px rgba(139, 92, 246, 0.2)',
        '0 0 40px rgba(139, 92, 246, 0.3)',
        '0 0 20px rgba(139, 92, 246, 0.2)',
      ],
    },
    transition: { type: 'spring', stiffness: 400, damping: 25 },
  },
  schema: {
    initial: { opacity: 1, scale: 1, y: 0 }, // Start visible
    visible: { opacity: 1, scale: 1, y: 0 },
    hover: { opacity: 1, scale: 1, y: -2 },
    selected: { opacity: 1, scale: 1, y: -3 },
    transition: { type: 'spring', stiffness: 500, damping: 30 },
  },
  data: {
    initial: { opacity: 1, scale: 1, y: 0 }, // Start visible
    visible: { opacity: 1, scale: 1, y: 0 },
    hover: { opacity: 1, scale: 1, y: -1 },
    selected: { opacity: 1, scale: 1, y: -2 },
    transition: { type: 'spring', stiffness: 600, damping: 35 },
  },
};

/**
 * Get animation config for a level
 */
export function getLevelAnimation(level: NodeLevel): LevelAnimationConfig {
  return LEVEL_ANIMATIONS[level];
}

// =============================================================================
// Typography Scale per Level
// =============================================================================

export interface LevelTypographyConfig {
  /** Main title size class */
  title: string;
  /** Subtitle/description size class */
  subtitle: string;
  /** Meta info size class */
  meta: string;
  /** Badge text size class */
  badge: string;
  /** Icon size in pixels */
  iconSize: number;
}

export const LEVEL_TYPOGRAPHY: Record<NodeLevel, LevelTypographyConfig> = {
  taxonomy: {
    title: 'text-xl font-bold',
    subtitle: 'text-sm font-medium',
    meta: 'text-xs',
    badge: 'text-[10px]',
    iconSize: 24,
  },
  schema: {
    title: 'text-lg font-semibold',
    subtitle: 'text-sm',
    meta: 'text-[11px]',
    badge: 'text-[9px]',
    iconSize: 20,
  },
  data: {
    title: 'text-base font-medium',
    subtitle: 'text-sm',
    meta: 'text-[10px]',
    badge: 'text-[8px]',
    iconSize: 16,
  },
};

/**
 * Get typography config for a level
 */
export function getLevelTypography(level: NodeLevel): LevelTypographyConfig {
  return LEVEL_TYPOGRAPHY[level];
}

// =============================================================================
// Combined Style Generator
// =============================================================================

export interface LevelStyles {
  /** Inline CSS styles */
  style: React.CSSProperties;
  /** Tailwind classes */
  className: string;
  /** Animation config */
  animation: LevelAnimationConfig;
  /** Typography config */
  typography: LevelTypographyConfig;
  /** Badge config */
  badge: LevelBadgeConfig;
}

/**
 * Get all styles for a level with optional glow color
 */
export function getLevelStyles(level: NodeLevel, glowColor?: string): LevelStyles {
  const visuals = getLevelVisuals(level);

  return {
    style: {
      borderWidth: visuals.borderWidth,
      borderRadius: visuals.borderRadius,
      boxShadow: getLevelShadow(level, glowColor),
      opacity: visuals.opacity,
      zIndex: visuals.zIndexBoost,
    },
    className: getLevelClasses(level),
    animation: getLevelAnimation(level),
    typography: getLevelTypography(level),
    badge: getLevelBadgeConfig(level),
  };
}

// =============================================================================
// Utility: Determine Level from Node Data
// =============================================================================

/** Taxonomy node types (21) */
export const TAXONOMY_TYPES = new Set([
  // Realms (2)
  'Realm', 'NodeRealm',
  // Layers (10) — includes both "Layer" node and specific layer attractors
  'Layer', 'NodeLayer',
  'ConfigLayer', 'LocaleLayer', 'GeographyLayer', 'KnowledgeLayer',
  'FoundationLayer', 'StructureLayer', 'SemanticLayer', 'InstructionLayer', 'OutputLayer',
  // Traits (5)
  'Trait', 'NodeTrait',
  // Arc Families (5)
  'ArcFamily',
]);

/** Schema node types (59 NodeClass + 159 ArcClass) v0.18.0 */
export const SCHEMA_TYPES = new Set([
  'NodeClass', 'ArcClass',
  // Additional schema-level labels
  'Class', 'Schema',
]);

/**
 * Determine the abstraction level of a node
 *
 * Uses type name to classify:
 * - Taxonomy: Realm, Layer, Trait, ArcFamily
 * - Schema: NodeClass, ArcClass (or any type ending in "Class")
 * - Data: Everything else (actual instances)
 */
export function getNodeLevel(type: string): NodeLevel {
  // Check for taxonomy nodes
  if (TAXONOMY_TYPES.has(type)) {
    return 'taxonomy';
  }

  // Check for schema nodes (Class definitions)
  if (SCHEMA_TYPES.has(type) || type.endsWith('Class')) {
    return 'schema';
  }

  // Everything else is data
  return 'data';
}

/**
 * Check if a node type is a taxonomy node
 */
export function isTaxonomyType(type: string): boolean {
  return getNodeLevel(type) === 'taxonomy';
}

/**
 * Check if a node type is a schema node
 */
export function isSchemaType(type: string): boolean {
  return getNodeLevel(type) === 'schema';
}

/**
 * Check if a node type is a data node
 */
export function isDataType(type: string): boolean {
  return getNodeLevel(type) === 'data';
}
