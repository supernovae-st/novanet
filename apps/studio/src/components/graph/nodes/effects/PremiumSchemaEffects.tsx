'use client';

/**
 * PremiumSchemaEffects - Shared premium visual effects for schema nodes
 *
 * Provides consistent "hacker/cyberpunk" styling across all schema nodes:
 * - Realm nodes (shared, org)
 * - Layer nodes (config, locale, semantic, etc.)
 * - Class nodes (NodeClass, ArcClass)
 * - Trait nodes (defined, authored, imported, generated, retrieved)
 *
 * Effects included:
 * - L-shape tech corners (SVG-based)
 * - Scanline overlay (drifting horizontal lines)
 * - Grid pattern background (blueprint paper)
 * - Holographic shimmer on hover
 * - Matrix rain effect on selection
 * - Animated glow border
 *
 * @example
 * ```tsx
 * <PremiumSchemaEffects
 *   color={primaryColor}
 *   selected={selected}
 *   isHovered={isHovered}
 *   borderRadius={12}
 * />
 * ```
 */

import { memo, useMemo } from 'react';

// =============================================================================
// Types
// =============================================================================

export interface PremiumSchemaEffectsProps {
  /** Primary color for all effects */
  color: string;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is being hovered */
  isHovered: boolean;
  /** Border radius of the container */
  borderRadius?: number;
  /** Show corner decorations (default: true) */
  showCorners?: boolean;
  /** Show scanlines (default: true) */
  showScanlines?: boolean;
  /** Show grid pattern (default: true) */
  showGrid?: boolean;
  /** Show holographic shimmer on hover (default: true) */
  showShimmer?: boolean;
  /** Show matrix rain on selection (default: true) */
  showMatrixRain?: boolean;
  /** Intensity multiplier (default: 1) */
  intensity?: number;
}

// =============================================================================
// Helper
// =============================================================================

/** Convert hex to RGB string for rgba usage */
const hexToRgb = (hex: string): string => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}`
    : '42, 161, 152'; // cyan fallback
};

// =============================================================================
// Sub-Effects
// =============================================================================

/**
 * L-shape tech corner decorations - cyberpunk/hacker aesthetic
 */
export const TechCorners = memo(function TechCorners({
  color,
  selected,
  size = 16,
}: {
  color: string;
  selected: boolean;
  size?: number;
}) {
  const opacity = selected ? 0.9 : 0.5;

  // Corner path: L-shape with optional dot
  const cornerPath = `M0 ${size}L0 0L${size} 0`;

  return (
    <>
      {/* Top-left */}
      <div
        className="absolute pointer-events-none z-20"
        style={{ top: 8, left: 8, color, opacity }}
      >
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} fill="none">
          <path d={cornerPath} stroke="currentColor" strokeWidth="1.5" />
          <circle cx="0" cy="0" r="2" fill="currentColor" />
        </svg>
      </div>

      {/* Top-right */}
      <div
        className="absolute pointer-events-none z-20"
        style={{ top: 8, right: 8, color, opacity, transform: 'scaleX(-1)' }}
      >
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} fill="none">
          <path d={cornerPath} stroke="currentColor" strokeWidth="1.5" />
          <circle cx="0" cy="0" r="2" fill="currentColor" />
        </svg>
      </div>

      {/* Bottom-left */}
      <div
        className="absolute pointer-events-none z-20"
        style={{ bottom: 8, left: 8, color, opacity, transform: 'scaleY(-1)' }}
      >
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} fill="none">
          <path d={cornerPath} stroke="currentColor" strokeWidth="1.5" />
          <circle cx="0" cy="0" r="2" fill="currentColor" />
        </svg>
      </div>

      {/* Bottom-right */}
      <div
        className="absolute pointer-events-none z-20"
        style={{ bottom: 8, right: 8, color, opacity, transform: 'scale(-1)' }}
      >
        <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} fill="none">
          <path d={cornerPath} stroke="currentColor" strokeWidth="1.5" />
          <circle cx="0" cy="0" r="2" fill="currentColor" />
        </svg>
      </div>
    </>
  );
});

/**
 * Scanline overlay effect - horizontal lines that drift slowly
 */
export const Scanlines = memo(function Scanlines({
  color,
  intensity,
}: {
  color: string;
  intensity: 'idle' | 'hover' | 'selected';
}) {
  const opacity = intensity === 'selected' ? 0.15 : intensity === 'hover' ? 0.1 : 0.05;
  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 pointer-events-none overflow-hidden rounded-xl z-10"
      style={{
        background: `repeating-linear-gradient(
          0deg,
          transparent,
          transparent 3px,
          rgba(${rgb}, ${opacity}) 3px,
          rgba(${rgb}, ${opacity}) 6px
        )`,
        animation: intensity !== 'idle' ? 'schema-scanline-drift 6s linear infinite' : undefined,
      }}
    />
  );
});

/**
 * Grid pattern background - matrix/blueprint paper effect
 */
export const GridPattern = memo(function GridPattern({
  color,
  intensity,
}: {
  color: string;
  intensity: 'idle' | 'hover' | 'selected';
}) {
  const rgb = hexToRgb(color);
  const opacity = intensity === 'selected' ? 0.08 : intensity === 'hover' ? 0.05 : 0.03;

  return (
    <div
      className="absolute inset-0 pointer-events-none rounded-xl transition-opacity duration-500 z-10"
      style={{
        backgroundImage: `
          linear-gradient(rgba(${rgb}, ${opacity}) 1px, transparent 1px),
          linear-gradient(90deg, rgba(${rgb}, ${opacity}) 1px, transparent 1px)
        `,
        backgroundSize: '16px 16px',
      }}
    />
  );
});

/**
 * Holographic shimmer effect on hover
 */
export const HolographicShimmer = memo(function HolographicShimmer({
  color,
  active,
}: {
  color: string;
  active: boolean;
}) {
  if (!active) return null;

  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 pointer-events-none rounded-xl z-10"
      style={{
        background: `linear-gradient(
          105deg,
          transparent 35%,
          rgba(${rgb}, 0.15) 42%,
          rgba(${rgb}, 0.25) 50%,
          rgba(${rgb}, 0.15) 58%,
          transparent 65%
        )`,
        backgroundSize: '250% 100%',
        animation: 'schema-shimmer-slide 2.5s ease-in-out infinite',
      }}
    />
  );
});

/**
 * Matrix rain effect for selected state - falling digital rain
 */
export const MatrixRain = memo(function MatrixRain({
  color,
  active,
}: {
  color: string;
  active: boolean;
}) {
  if (!active) return null;

  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 pointer-events-none rounded-xl overflow-hidden z-10"
      style={{
        background: `
          linear-gradient(180deg,
            rgba(${rgb}, 0.1) 0%,
            transparent 20%,
            transparent 80%,
            rgba(${rgb}, 0.1) 100%
          )
        `,
      }}
    >
      {/* Vertical falling lines */}
      <div
        className="absolute inset-0"
        style={{
          backgroundImage: `
            linear-gradient(0deg, transparent 50%, rgba(${rgb}, 0.3) 50%)
          `,
          backgroundSize: '4px 8px',
          animation: 'schema-matrix-rain 1.5s linear infinite',
        }}
      />
    </div>
  );
});

/**
 * Outer glow layer for the container
 */
export const OuterGlow = memo(function OuterGlow({
  color,
  selected,
  isHovered,
  borderRadius = 16,
}: {
  color: string;
  selected: boolean;
  isHovered: boolean;
  borderRadius?: number;
}) {
  const glowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `0 0 30px ${color}50, 0 0 60px ${color}25, inset 0 0 30px ${color}15`
        : isHovered
          ? `0 0 20px ${color}35, inset 0 0 15px ${color}10`
          : `0 0 12px ${color}20`,
    }),
    [color, selected, isHovered]
  );

  return (
    <div
      className="absolute -inset-1 transition-all duration-300 z-0"
      style={{
        borderRadius: borderRadius + 4,
        ...glowStyle,
      }}
    />
  );
});

// =============================================================================
// Main Combined Component
// =============================================================================

export const PremiumSchemaEffects = memo(function PremiumSchemaEffects({
  color,
  selected,
  isHovered,
  borderRadius = 12,
  showCorners = true,
  showScanlines = true,
  showGrid = true,
  showShimmer = true,
  showMatrixRain = true,
}: PremiumSchemaEffectsProps) {
  const intensity = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  return (
    <>
      {/* Outer glow layer */}
      <OuterGlow
        color={color}
        selected={selected}
        isHovered={isHovered}
        borderRadius={borderRadius}
      />

      {/* Grid pattern */}
      {showGrid && <GridPattern color={color} intensity={intensity} />}

      {/* Scanlines */}
      {showScanlines && <Scanlines color={color} intensity={intensity} />}

      {/* Holographic shimmer (hover only) */}
      {showShimmer && <HolographicShimmer color={color} active={isHovered && !selected} />}

      {/* Matrix rain (selected only) */}
      {showMatrixRain && <MatrixRain color={color} active={selected} />}

      {/* Tech corners */}
      {showCorners && <TechCorners color={color} selected={selected} />}
    </>
  );
});

// =============================================================================
// CSS Keyframes (to be injected in parent or via global styles)
// =============================================================================

export const PremiumSchemaKeyframes = `
  @keyframes schema-scanline-drift {
    0% { background-position: 0 0; }
    100% { background-position: 0 120px; }
  }
  @keyframes schema-shimmer-slide {
    0% { background-position: 250% 0; }
    100% { background-position: -250% 0; }
  }
  @keyframes schema-matrix-rain {
    0% { transform: translateY(-8px); }
    100% { transform: translateY(0); }
  }
`;
