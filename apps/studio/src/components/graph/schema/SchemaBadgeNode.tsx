'use client';

/**
 * SchemaBadgeNode - "Holographic Matrix" design for Realm & Layer badges
 *
 * Visual Encoding (ADR-005, visual-encoding.yaml):
 * - Primary color → from Realm (shared=cyan, org=violet) or Layer
 * - Scanline overlay effect (horizontal animated lines)
 * - Grid pattern background (blueprint/matrix paper)
 * - Holographic shimmer on hover
 * - Corner tech decorations
 * - Matrix rain effect on selection
 *
 * Layout:
 * ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐
 * ╎ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ╎  ← Scanlines + Grid
 * ╎  ┌───┐                      ┌─────────┐  ╎
 * ╎  │ ⬡ │                      │ REALM   │  ╎  ← Badge type
 * ╎  └───┘                      │ SHARED  │  ╎  ← Badge value
 * ╎                             └─────────┘  ╎
 * ╎  ════════════════════════════════════    ╎  ← Double line
 * ╎  Shared                                  ╎  ← Label (glow)
 * ╎  40 node types                           ╎  ← Subtitle
 * └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { LayerIcon, RealmIcon } from '@/components/ui/CategoryIcon';
import type { Realm, Layer } from '@novanet/core/types';
import { REALM_COLORS, LAYER_COLORS } from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export interface SchemaBadgeNodeData extends Record<string, unknown> {
  label: string;
  description: string;
  metaType: 'realm' | 'layer';
  color: string;
  typeCount?: number;
  realmKey?: Realm;
  layerKey?: Layer;
}

export type SchemaBadgeNodeType = Node<SchemaBadgeNodeData, 'schemaBadge'>;

// =============================================================================
// Helper
// =============================================================================

/** Convert hex to RGB string for rgba usage */
const hexToRgb = (hex: string): string => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}`
    : '42, 161, 152';
};

// =============================================================================
// Subcomponents
// =============================================================================

/**
 * Scanline overlay effect - horizontal lines that drift slowly
 */
const ScanlineOverlay = memo(function ScanlineOverlay({
  intensity,
  color,
}: {
  intensity: 'idle' | 'hover' | 'selected';
  color: string;
}) {
  const opacity = intensity === 'selected' ? 0.15 : intensity === 'hover' ? 0.1 : 0.05;
  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 pointer-events-none overflow-hidden rounded-xl"
      style={{
        background: `repeating-linear-gradient(
          0deg,
          transparent,
          transparent 3px,
          rgba(${rgb}, ${opacity}) 3px,
          rgba(${rgb}, ${opacity}) 6px
        )`,
        animation: intensity !== 'idle' ? 'scanline-drift 6s linear infinite' : undefined,
      }}
    />
  );
});

/**
 * Grid pattern background - matrix/blueprint paper effect
 */
const GridPattern = memo(function GridPattern({
  intensity,
  color,
}: {
  intensity: 'idle' | 'hover' | 'selected';
  color: string;
}) {
  const rgb = hexToRgb(color);
  const opacity = intensity === 'selected' ? 0.08 : intensity === 'hover' ? 0.05 : 0.03;

  return (
    <div
      className="absolute inset-0 pointer-events-none rounded-xl transition-opacity duration-500"
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
const HolographicShimmer = memo(function HolographicShimmer({
  active,
  color,
}: {
  active: boolean;
  color: string;
}) {
  if (!active) return null;

  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 pointer-events-none rounded-xl"
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
        animation: 'shimmer-slide 2.5s ease-in-out infinite',
      }}
    />
  );
});

/**
 * Matrix rain effect for selected state
 */
const MatrixRain = memo(function MatrixRain({
  active,
  color,
}: {
  active: boolean;
  color: string;
}) {
  if (!active) return null;

  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 pointer-events-none rounded-xl overflow-hidden"
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
          animation: 'matrix-rain 1.5s linear infinite',
        }}
      />
    </div>
  );
});

/**
 * Corner tech decorations
 */
const CornerDecorations = memo(function CornerDecorations({
  color,
  selected,
}: {
  color: string;
  selected: boolean;
}) {
  const opacity = selected ? 0.8 : 0.4;

  return (
    <>
      {/* Top-left corner */}
      <div
        className="absolute top-2 left-2 pointer-events-none"
        style={{ color, opacity }}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M0 8L0 0L8 0" stroke="currentColor" strokeWidth="1.5" />
          <circle cx="0" cy="0" r="2" fill="currentColor" />
        </svg>
      </div>

      {/* Top-right corner */}
      <div
        className="absolute top-2 right-2 pointer-events-none"
        style={{ color, opacity }}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M16 8L16 0L8 0" stroke="currentColor" strokeWidth="1.5" />
          <circle cx="16" cy="0" r="2" fill="currentColor" />
        </svg>
      </div>

      {/* Bottom-left corner */}
      <div
        className="absolute bottom-2 left-2 pointer-events-none"
        style={{ color, opacity }}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M0 8L0 16L8 16" stroke="currentColor" strokeWidth="1.5" />
          <circle cx="0" cy="16" r="2" fill="currentColor" />
        </svg>
      </div>

      {/* Bottom-right corner */}
      <div
        className="absolute bottom-2 right-2 pointer-events-none"
        style={{ color, opacity }}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M16 8L16 16L8 16" stroke="currentColor" strokeWidth="1.5" />
          <circle cx="16" cy="16" r="2" fill="currentColor" />
        </svg>
      </div>
    </>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const SchemaBadgeNode = memo(function SchemaBadgeNode({
  data,
  selected = false,
}: NodeProps<SchemaBadgeNodeType>) {
  const { metaType, label, typeCount = 0, realmKey = 'shared', layerKey = 'foundation' } = data;

  // Get design system color based on meta type
  const isRealm = metaType === 'realm';
  const primaryColor = isRealm
    ? REALM_COLORS[realmKey]?.color || '#2aa198'
    : LAYER_COLORS[layerKey]?.color || '#64748b';

  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
  } = useNodeInteractions({ selected });

  // Intensity level for effects
  const intensity = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style for label
  const labelGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 20px ${primaryColor}80, 0 0 40px ${primaryColor}40`
        : isHovered
          ? `0 0 12px ${primaryColor}60`
          : `0 0 6px ${primaryColor}30`,
    }),
    [primaryColor, selected, isHovered]
  );

  // Border glow animation
  const containerGlowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `0 0 30px ${primaryColor}50, 0 0 60px ${primaryColor}25, inset 0 0 30px ${primaryColor}15`
        : isHovered
          ? `0 0 20px ${primaryColor}35, inset 0 0 15px ${primaryColor}10`
          : `0 0 12px ${primaryColor}20`,
    }),
    [primaryColor, selected, isHovered]
  );

  // Container opacity for dimming
  const containerStyle = useMemo(() => ({
    opacity: 1,
    transition: 'all 0.3s ease-out',
    transform: selected ? 'scale(1.02)' : 'scale(1)',
  }), [selected]);

  return (
    <div
      className="relative"
      style={{ width: 280, ...containerStyle }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={`${metaType} node: ${label}`}
    >
      {/* Hidden handles for edges */}
      <Handle type="target" position={Position.Left} className="!opacity-0 !w-1 !h-1" />
      <Handle type="source" position={Position.Right} className="!opacity-0 !w-1 !h-1" />

      {/* Outer glow layer */}
      <div
        className="absolute -inset-1 rounded-[16px] transition-all duration-300"
        style={containerGlowStyle}
      />

      {/* Main container */}
      <div
        className={cn(
          'relative overflow-hidden rounded-xl transition-all duration-300',
          selected && 'ring-2',
        )}
        style={{
          minHeight: 160,
          background: 'rgba(0, 0, 0, 0.85)',
          backdropFilter: 'blur(12px)',
          border: `2px solid ${selected ? primaryColor : `${primaryColor}50`}`,
          // Use CSS variable for ring color (Tailwind ring-2 reads --tw-ring-color)
          '--tw-ring-color': primaryColor,
        } as React.CSSProperties}
      >
        {/* Background effects */}
        <GridPattern intensity={intensity} color={primaryColor} />
        <ScanlineOverlay intensity={intensity} color={primaryColor} />
        <HolographicShimmer active={isHovered && !selected} color={primaryColor} />
        <MatrixRain active={selected} color={primaryColor} />
        <CornerDecorations color={primaryColor} selected={selected} />

        {/* Content */}
        <div className="relative z-10 px-5 py-4">
          {/* Top row: Icon left, Badges right */}
          <div className="flex justify-between items-start mb-4">
            {/* Hexagonal icon container */}
            <div
              className={cn(
                'flex items-center justify-center w-14 h-14 rounded-lg transition-all duration-300',
                selected && 'animate-pulse'
              )}
              style={{
                background: `linear-gradient(135deg, ${primaryColor}30, ${primaryColor}10)`,
                border: `2px solid ${primaryColor}60`,
                boxShadow: `0 0 20px ${primaryColor}30, inset 0 0 15px ${primaryColor}15`,
              }}
            >
              {isRealm ? (
                <RealmIcon
                  realm={realmKey}
                  size={28}
                  strokeWidth={1.5}
                  style={{
                    color: primaryColor,
                    filter: `drop-shadow(0 0 6px ${primaryColor})`,
                  }}
                />
              ) : (
                <LayerIcon
                  layer={layerKey}
                  size={28}
                  strokeWidth={1.5}
                  style={{
                    color: primaryColor,
                    filter: `drop-shadow(0 0 6px ${primaryColor})`,
                  }}
                />
              )}
            </div>

            {/* Stacked badges */}
            <div className="flex flex-col gap-1.5 items-end">
              {/* Type badge */}
              <span
                className="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-widest px-2.5 py-1 rounded font-mono"
                style={{
                  background: `${primaryColor}20`,
                  color: primaryColor,
                  border: `1px solid ${primaryColor}40`,
                  boxShadow: `0 0 10px ${primaryColor}20`,
                }}
              >
                {isRealm ? (
                  <RealmIcon realm={realmKey} size={10} strokeWidth={2} style={{ color: primaryColor }} />
                ) : (
                  <LayerIcon layer={layerKey} size={10} strokeWidth={2} style={{ color: primaryColor }} />
                )}
                {isRealm ? 'REALM' : 'LAYER'}
              </span>

              {/* Value badge */}
              <span
                className="text-[10px] font-bold uppercase tracking-wide px-2.5 py-1 rounded font-mono"
                style={{
                  background: `${primaryColor}15`,
                  color: primaryColor,
                  border: `1px solid ${primaryColor}30`,
                }}
              >
                {isRealm ? realmKey.toUpperCase() : layerKey.toUpperCase()}
              </span>
            </div>
          </div>

          {/* Double line separator */}
          <div className="mb-3">
            <div
              className="h-[2px] mb-[2px]"
              style={{
                background: `linear-gradient(90deg, ${primaryColor}60, ${primaryColor}20, transparent)`,
              }}
            />
            <div
              className="h-[1px]"
              style={{
                background: `linear-gradient(90deg, ${primaryColor}40, transparent)`,
              }}
            />
          </div>

          {/* Label - hero element */}
          <h3
            className={cn(
              'text-xl font-bold text-white mb-1',
              'transition-all duration-200'
            )}
            style={labelGlowStyle}
          >
            {label}
          </h3>

          {/* Subtitle - type count */}
          <p
            className="text-sm font-mono"
            style={{ color: `${primaryColor}cc` }}
          >
            {isRealm ? `${typeCount} node types` : `${typeCount} types`}
          </p>
        </div>
      </div>

      {/* CSS for animations */}
      <style jsx>{`
        @keyframes scanline-drift {
          0% { background-position: 0 0; }
          100% { background-position: 0 120px; }
        }
        @keyframes shimmer-slide {
          0% { background-position: 250% 0; }
          100% { background-position: -250% 0; }
        }
        @keyframes matrix-rain {
          0% { transform: translateY(-8px); }
          100% { transform: translateY(0); }
        }
      `}</style>
    </div>
  );
});
