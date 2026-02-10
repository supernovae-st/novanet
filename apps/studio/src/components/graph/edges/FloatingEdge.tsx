'use client';

/**
 * FloatingEdge - Modular edge component with composable effects
 *
 * Architecture (v2):
 * - Uses new type system for discriminated unions
 * - Uses theme registry for relation-based styling
 * - Uses LOD controller for zoom-based quality
 * - Uses animation budget for performance limits
 * - Uses EffectRenderer for composable primitives
 */

import { memo, useMemo, useEffect, useRef } from 'react';
import { useInternalNode, useStore, type Edge, type EdgeProps } from '@xyflow/react';
import { useUIStore, selectHoveredEdgeId, selectHoveredNodeId, selectSelectedNodeId, selectSelectedEdgeId } from '@/stores/uiStore';
import { useEdgeVisibility } from './EdgeVisibilityManager';

// New modular system
import { useEdgeTheme } from './hooks/useEdgeTheme';
import { useEdgeLOD } from './hooks/useEdgeLOD';
import { useAnimationBudget } from './hooks/useAnimationBudget';
import { releaseEdgeAnimationSlot } from './effects/EffectRenderer';
import { getSmartLabel, getNodeIntersection, generateCurvedPath, generateReversedPath, generateParallelPath } from './EdgeUtils';
import type { EdgeState } from './system/types';

// Arc family detection with comprehensive mapping (60+ relation types)
import { getArcFamily } from './system/arcFamilyPalettes';

// Note: EffectRenderer disabled for now - using InlineEdgeEffects instead (working pattern)

// =============================================================================
// Inline Edge Effects (Working Pattern)
// =============================================================================

interface InlineEdgeEffectsProps {
  edgePath: string;
  relationType: string;
  colors: { primary: string; secondary: string; glow: string };
  state: EdgeState;
  /** Use simplified 2-element rendering for performance */
  simplified?: boolean;
}

/**
 * SimplifiedEdgeEffect - Lightweight atom-like effect for large graphs
 *
 * ATOM DESIGN v2:
 * - Slower movement (4s duration) for better visibility
 * - Larger particles (14px) that are easy to track
 * - Multi-layer glow for "energy orb" effect
 * - Trail with size/opacity decay for direction
 * - Organic wobble via perpendicular oscillation
 *
 * Performance: ~6 SVG elements per edge (optimized from 11-20)
 */
const SimplifiedEdgeEffect = memo(function SimplifiedEdgeEffect({
  edgePath,
  colors,
  state,
}: Omit<InlineEdgeEffectsProps, 'relationType' | 'simplified'>) {
  const isHighlighted = state === 'highlighted' || state === 'selected';
  // LARGER particles for visibility
  const size = isHighlighted ? 16 : 12;
  // SLOWER movement - 4x slower than before
  const duration = isHighlighted ? 3.5 : 5;

  return (
    <g className="effect-simplified-atom">
      {/* === LEADER ATOM === */}
      {/* Outer glow - wide, soft, creates "energy field" */}
      <circle
        r={size * 2}
        fill={colors.glow}
        opacity={0.25}
        style={{ filter: 'blur(8px)' }}
      >
        <animateMotion dur={`${duration}s`} repeatCount="indefinite" path={edgePath} />
      </circle>

      {/* Middle glow - core energy */}
      <circle
        r={size * 1.3}
        fill={colors.primary}
        opacity={0.5}
        style={{ filter: 'blur(4px)' }}
      >
        <animateMotion dur={`${duration}s`} repeatCount="indefinite" path={edgePath} />
        {/* Organic wobble - perpendicular oscillation */}
        <animate attributeName="cy" values="-3;3;-3" dur="0.8s" repeatCount="indefinite" />
      </circle>

      {/* Core - solid visible particle */}
      <circle
        r={size}
        fill={colors.primary}
        opacity={0.95}
        style={{ filter: `drop-shadow(0 0 ${size}px ${colors.glow})` }}
      >
        <animateMotion dur={`${duration}s`} repeatCount="indefinite" path={edgePath} />
        {/* Subtle wobble */}
        <animate attributeName="cy" values="-2;2;-2" dur="0.6s" repeatCount="indefinite" />
      </circle>

      {/* White hot center - brightest point */}
      <circle r={size * 0.35} fill="#ffffff" opacity={1}>
        <animateMotion dur={`${duration}s`} repeatCount="indefinite" path={edgePath} />
      </circle>

      {/* === TRAIL SEGMENTS (3) - show direction via decay === */}
      {[1, 2, 3].map((i) => (
        <circle
          key={`trail-${i}`}
          r={size * (1 - i * 0.2)}
          fill={colors.glow}
          opacity={0.7 - i * 0.15}
          style={{ filter: `drop-shadow(0 0 ${Math.max(2, size - i * 3)}px ${colors.glow})` }}
        >
          <animateMotion
            dur={`${duration}s`}
            repeatCount="indefinite"
            begin={`${i * 0.15}s`}
            path={edgePath}
          />
        </circle>
      ))}
    </g>
  );
});

/**
 * InlineEdgeEffects - ATOM-LIKE animated effects (v2)
 *
 * REDESIGNED for better visibility and direction perception:
 * - SLOWER: 4-8s duration (was 0.6-1.8s) - easy to track
 * - FEWER: 3-5 particles (was 8-30) - cleaner, less noise
 * - LARGER: 14-20px (was 4-12px) - highly visible atoms
 * - ORGANIC: Perpendicular wobble for natural movement
 * - DIRECTIONAL: Trail decay shows flow direction
 *
 * Each arc family has DISTINCT movement patterns:
 * - ownership: ⚡ STEADY PULSE - Consistent flow with trail (data ownership)
 * - localization: 🧬 GENTLE WAVE - Sinusoidal oscillation (content adapts)
 * - semantic: 🔗 ORBITING SPARKS - Slow circular drift (meaning links)
 * - generation: 💻 CASCADE PULSE - Rhythmic bursts (AI processing)
 * - mining: 📡 RADAR PING - Slow sweep + expanding rings (discovery)
 */
const InlineEdgeEffects = memo(function InlineEdgeEffects({
  edgePath,
  relationType,
  colors,
  state,
  simplified = false,
}: InlineEdgeEffectsProps) {
  // PERFORMANCE: Use simplified atom effect for large graphs
  if (simplified) {
    return <SimplifiedEdgeEffect edgePath={edgePath} colors={colors} state={state} />;
  }

  const family = getArcFamily(relationType);
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // LARGER particles for visibility
  const baseSize = isHighlighted ? 18 : 14;
  // SLOWER movement - 4x slower than before for trackability
  const baseDuration = isHighlighted ? 4 : 6;


  switch (family) {
    case 'ownership':
      // ⚡ STEADY PULSE - Consistent energy flow with visible trail
      // Visual: Data ownership flowing from parent to children
      // OPTIMIZED: 2 atoms + 4 trail = 6 elements (was 13)
      return (
        <g className="effect-ownership-atom">
          {/* === LEADER ATOM === */}
          {/* Wide glow field */}
          <circle r={baseSize * 2.2} fill={colors.glow} opacity={0.3} style={{ filter: 'blur(10px)' }}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
          </circle>
          {/* Core glow */}
          <circle r={baseSize * 1.4} fill={colors.primary} opacity={0.6} style={{ filter: 'blur(5px)' }}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
            {/* Gentle organic wobble */}
            <animate attributeName="cy" values="-4;4;-4" dur="1.2s" repeatCount="indefinite" />
          </circle>
          {/* Solid core */}
          <circle r={baseSize} fill={colors.primary} opacity={0.95} style={{ filter: `drop-shadow(0 0 ${baseSize}px ${colors.glow})` }}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
            <animate attributeName="cy" values="-2;2;-2" dur="0.9s" repeatCount="indefinite" />
          </circle>
          {/* White hot center */}
          <circle r={baseSize * 0.35} fill="#ffffff" opacity={1}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
          </circle>

          {/* === TRAIL (4 segments) - shows direction === */}
          {[1, 2, 3, 4].map((i) => (
            <circle
              key={`trail-${i}`}
              r={baseSize * (1 - i * 0.18)}
              fill={colors.glow}
              opacity={0.75 - i * 0.15}
              style={{ filter: `drop-shadow(0 0 ${Math.max(3, baseSize - i * 4)}px ${colors.glow})` }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${i * 0.2}s`}
                path={edgePath}
              />
            </circle>
          ))}

          {/* === SECONDARY ATOM (offset) === */}
          <circle r={baseSize * 0.8} fill={colors.secondary} opacity={0.85} style={{ filter: `drop-shadow(0 0 8px ${colors.glow})` }}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" begin={`${baseDuration / 2}s`} path={edgePath} />
            <animate attributeName="cy" values="3;-3;3" dur="1s" repeatCount="indefinite" />
          </circle>
          <circle r={baseSize * 0.3} fill="#ffffff" opacity={0.9}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" begin={`${baseDuration / 2}s`} path={edgePath} />
          </circle>
        </g>
      );

    case 'localization':
      // 🧬 GENTLE WAVE - Sinusoidal oscillation for content adaptation
      // Visual: Flowing wave of atoms adapting to locale
      // OPTIMIZED: 2 strands × 3 atoms + 2 connectors = 8 elements (was 20)
      return (
        <g className="effect-localization-atom">
          {/* === STRAND 1 (3 atoms) - Primary wave === */}
          {[0, 1, 2].map((i) => (
            <g key={`strand1-${i}`}>
              {/* Glow field */}
              <circle
                r={baseSize * 1.6}
                fill={colors.glow}
                opacity={0.25}
                style={{ filter: 'blur(6px)' }}
              >
                <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.4}s`} path={edgePath} />
                {/* Wide sinusoidal wave - SLOWER oscillation */}
                <animate attributeName="cy" values="-18;18;-18" dur="2s" repeatCount="indefinite" begin={`${i * 0.3}s`} />
              </circle>
              {/* Core atom */}
              <circle
                r={baseSize * 0.9}
                fill={colors.primary}
                opacity={0.9}
                style={{ filter: `drop-shadow(0 0 ${baseSize}px ${colors.glow})` }}
              >
                <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.4}s`} path={edgePath} />
                <animate attributeName="cy" values="-16;16;-16" dur="2s" repeatCount="indefinite" begin={`${i * 0.3}s`} />
              </circle>
              {/* White center */}
              <circle r={baseSize * 0.3} fill="#ffffff" opacity={0.95}>
                <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.4}s`} path={edgePath} />
                <animate attributeName="cy" values="-16;16;-16" dur="2s" repeatCount="indefinite" begin={`${i * 0.3}s`} />
              </circle>
            </g>
          ))}

          {/* === STRAND 2 (3 atoms) - Secondary wave (opposite phase) === */}
          {[0, 1, 2].map((i) => (
            <g key={`strand2-${i}`}>
              <circle
                r={baseSize * 1.3}
                fill={colors.secondary}
                opacity={0.2}
                style={{ filter: 'blur(5px)' }}
              >
                <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.4}s`} path={edgePath} />
                <animate attributeName="cy" values="18;-18;18" dur="2s" repeatCount="indefinite" begin={`${i * 0.3}s`} />
              </circle>
              <circle
                r={baseSize * 0.7}
                fill={colors.secondary}
                opacity={0.85}
                style={{ filter: `drop-shadow(0 0 8px ${colors.secondary})` }}
              >
                <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.4}s`} path={edgePath} />
                <animate attributeName="cy" values="16;-16;16" dur="2s" repeatCount="indefinite" begin={`${i * 0.3}s`} />
              </circle>
            </g>
          ))}

          {/* === CONNECTORS (2) - white dots between strands === */}
          {[0, 1].map((i) => (
            <circle
              key={`conn-${i}`}
              r={baseSize * 0.35}
              fill="#ffffff"
              opacity={0.8}
              style={{ filter: `drop-shadow(0 0 4px #ffffff)` }}
            >
              <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.6}s`} path={edgePath} />
            </circle>
          ))}
        </g>
      );

    case 'semantic':
      // 🔗 ORBITING SPARKS - Slow circular drift for meaning connections
      // Visual: Atoms orbiting around path showing semantic links
      // OPTIMIZED: 4 atoms with orbital wobble = 8 elements (was 16)
      return (
        <g className="effect-semantic-atom">
          {/* === ORBITING ATOMS (4) === */}
          {[0, 1, 2, 3].map((i) => (
            <g key={`orbit-${i}`}>
              {/* Glow field */}
              <circle
                r={baseSize * 1.5}
                fill={colors.glow}
                opacity={0.25}
                style={{ filter: 'blur(6px)' }}
              >
                <animateMotion dur={`${baseDuration * 0.8}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.2}s`} path={edgePath} />
                {/* Orbital wobble - circular motion around path */}
                <animate attributeName="cx" values="-10;10;-10" dur="1.5s" repeatCount="indefinite" begin={`${i * 0.4}s`} />
                <animate attributeName="cy" values="10;-10;10" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.4}s`} />
              </circle>
              {/* Core atom */}
              <circle
                r={baseSize * 0.8}
                fill={colors.primary}
                opacity={0.95}
                style={{ filter: `drop-shadow(0 0 ${baseSize}px ${colors.glow})` }}
              >
                <animateMotion dur={`${baseDuration * 0.8}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.2}s`} path={edgePath} />
                <animate attributeName="cx" values="-8;8;-8" dur="1.5s" repeatCount="indefinite" begin={`${i * 0.4}s`} />
                <animate attributeName="cy" values="8;-8;8" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.4}s`} />
              </circle>
              {/* White spark center */}
              <circle r={baseSize * 0.25} fill="#ffffff" opacity={1}>
                <animateMotion dur={`${baseDuration * 0.8}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.2}s`} path={edgePath} />
                <animate attributeName="cx" values="-8;8;-8" dur="1.5s" repeatCount="indefinite" begin={`${i * 0.4}s`} />
                <animate attributeName="cy" values="8;-8;8" dur="1.2s" repeatCount="indefinite" begin={`${i * 0.4}s`} />
              </circle>
            </g>
          ))}

          {/* === SUBTLE GLOW TRAIL (2) === */}
          {[0, 1].map((i) => (
            <circle
              key={`glow-${i}`}
              r={baseSize * 1.2}
              fill={colors.glow}
              opacity={0.3}
              style={{ filter: 'blur(8px)' }}
            >
              <animateMotion dur={`${baseDuration * 0.8}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.4}s`} path={edgePath} />
            </circle>
          ))}
        </g>
      );

    case 'generation':
      // 💻 CASCADE PULSE - Rhythmic bursts for AI processing
      // Visual: Data packets cascading through the generation pipeline
      // OPTIMIZED: 3 cascading atoms + 3 trail = 9 elements (was 19)
      return (
        <g className="effect-generation-atom">
          {/* === CASCADING ATOMS (3) === */}
          {[0, 1, 2].map((i) => (
            <g key={`cascade-${i}`}>
              {/* Wide energy field */}
              <circle
                r={baseSize * 2}
                fill={colors.glow}
                opacity={0.2}
                style={{ filter: 'blur(10px)' }}
              >
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.3}s`} path={edgePath} />
              </circle>
              {/* Core glow */}
              <circle
                r={baseSize * 1.2}
                fill={colors.primary}
                opacity={0.5}
                style={{ filter: 'blur(4px)' }}
              >
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.3}s`} path={edgePath} />
                {/* Pulsing effect */}
                <animate attributeName="r" values={`${baseSize * 1.2};${baseSize * 1.5};${baseSize * 1.2}`} dur="1.5s" repeatCount="indefinite" begin={`${i * 0.5}s`} />
              </circle>
              {/* Solid atom */}
              <circle
                r={baseSize * 0.9}
                fill={colors.primary}
                opacity={0.95}
                style={{ filter: `drop-shadow(0 0 ${baseSize}px ${colors.glow})` }}
              >
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.3}s`} path={edgePath} />
                {/* Subtle wobble */}
                <animate attributeName="cy" values="-3;3;-3" dur="0.8s" repeatCount="indefinite" />
              </circle>
              {/* White center with flicker */}
              <circle r={baseSize * 0.35} fill="#ffffff">
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.3}s`} path={edgePath} />
                <animate attributeName="opacity" values="1;0.6;1;0.8;1" dur="0.8s" repeatCount="indefinite" begin={`${i * 0.2}s`} />
              </circle>
            </g>
          ))}

          {/* === TRAIL SEGMENTS (3) === */}
          {[1, 2, 3].map((i) => (
            <circle
              key={`trail-${i}`}
              r={baseSize * (0.7 - i * 0.15)}
              fill={colors.glow}
              opacity={0.6 - i * 0.15}
              style={{ filter: `drop-shadow(0 0 ${Math.max(2, baseSize - i * 3)}px ${colors.glow})` }}
            >
              <animateMotion
                dur={`${baseDuration * 0.9}s`}
                repeatCount="indefinite"
                begin={`${i * 0.15}s`}
                path={edgePath}
              />
            </circle>
          ))}
        </g>
      );

    case 'mining':
      // 📡 RADAR SWEEP - Scanning pulse discovering data
      // Visual: Radar-like sweep with expanding detection rings
      return (
        <g className="effect-mining">
          {/* Main radar pulse */}
          <g>
            {/* Wide sweep glow */}
            <circle r={baseSize * 2} fill={colors.glow} opacity={0.3} style={{ filter: 'blur(6px)' }}>
              <animateMotion dur={`${baseDuration * 1.8}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
            {/* Core scanner */}
            <circle r={baseSize} fill={colors.primary} opacity={0.95} style={{ filter: `drop-shadow(0 0 10px ${colors.glow})` }}>
              <animateMotion dur={`${baseDuration * 1.8}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
            {/* Hot center */}
            <circle r={baseSize * 0.35} fill="#ffffff" opacity={1}>
              <animateMotion dur={`${baseDuration * 1.8}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
          </g>

          {/* Expanding detection rings */}
          {[0, 1, 2].map((i) => (
            <circle
              key={`ring-${i}`}
              r={baseSize}
              fill="none"
              stroke={colors.glow}
              strokeWidth={2 - i * 0.5}
              opacity={0}
            >
              <animateMotion dur={`${baseDuration * 1.8}s`} repeatCount="indefinite" begin={`${i * 0.4}s`} path={edgePath} />
              {/* Expanding ring animation */}
              <animate attributeName="r" values={`${baseSize};${baseSize * 3.5};${baseSize * 3.5}`} dur="1.2s" repeatCount="indefinite" />
              <animate attributeName="opacity" values="0.8;0.3;0" dur="1.2s" repeatCount="indefinite" />
              <animate attributeName="stroke-width" values="2;0.5;0" dur="1.2s" repeatCount="indefinite" />
            </circle>
          ))}

          {/* Secondary ping - offset */}
          <circle r={baseSize * 0.6} fill={colors.secondary} opacity={0.8} style={{ filter: `drop-shadow(0 0 5px ${colors.glow})` }}>
            <animateMotion dur={`${baseDuration * 1.8}s`} repeatCount="indefinite" begin={`${baseDuration * 0.9}s`} path={edgePath} />
          </circle>
        </g>
      );

    default:
      return null;
  }
});

// =============================================================================
// Types
// =============================================================================

export interface FloatingEdgeData extends Record<string, unknown> {
  relationType: string;
  animated?: boolean;
  dimmed?: boolean;
  selected?: boolean;
  showLabel?: boolean;
  /** Parallel edge support (v11.6.1) */
  parallelIndex?: number;
  parallelTotal?: number;
}

export type FloatingEdgeType = Edge<FloatingEdgeData>;

// =============================================================================
// Equality Function
// =============================================================================

function arePropsEqual(
  prev: EdgeProps<FloatingEdgeType>,
  next: EdgeProps<FloatingEdgeType>
): boolean {
  if (
    prev.id !== next.id ||
    prev.source !== next.source ||
    prev.target !== next.target ||
    prev.selected !== next.selected
  ) {
    return false;
  }

  const prevData = prev.data;
  const nextData = next.data;

  if (!prevData && !nextData) return true;
  if (!prevData || !nextData) return false;

  return (
    prevData.dimmed === nextData.dimmed &&
    prevData.animated === nextData.animated &&
    prevData.showLabel === nextData.showLabel &&
    prevData.relationType === nextData.relationType &&
    prevData.parallelIndex === nextData.parallelIndex &&
    prevData.parallelTotal === nextData.parallelTotal
  );
}

// =============================================================================
// Component
// =============================================================================

export const FloatingEdge = memo(function FloatingEdge({
  id,
  source,
  target,
  data,
  selected,
}: EdgeProps<FloatingEdgeType>) {
  const sourceNode = useInternalNode(source);
  const targetNode = useInternalNode(target);
  const pathRef = useRef<SVGPathElement>(null);

  // Get viewport transform for LOD and label scaling
  // transform = [panX, panY, zoom], width/height = viewport dimensions
  const transform = useStore((state) => state.transform);
  const viewportWidth = useStore((state) => state.width);
  const viewportHeight = useStore((state) => state.height);
  const zoom = transform[2];

  // Direct store subscriptions for hover and selection state
  const hoveredEdgeId = useUIStore(selectHoveredEdgeId);
  const hoveredNodeId = useUIStore(selectHoveredNodeId);
  const selectedNodeId = useUIStore(selectSelectedNodeId);
  const selectedEdgeId = useUIStore(selectSelectedEdgeId);

  // Compute local state
  const isDimmed = data?.dimmed === true;
  const isAnimated = data?.animated !== false;
  // Use store-based selection for reliable state (React Flow's selected prop can be unreliable)
  const isSelected = selected || data?.selected || selectedEdgeId === id;
  const showLabel = data?.showLabel !== false;
  const relationType = data?.relationType || '';
  // Parallel edge info (v11.6.1)
  const parallelIndex = data?.parallelIndex;
  const parallelTotal = data?.parallelTotal;

  // Hover state computation
  const isHovered = hoveredEdgeId === id;
  const isEdgeHoverDimmed = hoveredEdgeId !== null && hoveredEdgeId !== id;
  const connectsToHoveredNode = source === hoveredNodeId || target === hoveredNodeId;
  const isNodeHoverDimmed = hoveredNodeId !== null && !connectsToHoveredNode;
  const isHoverDimmed = !selectedNodeId && (isEdgeHoverDimmed || (!hoveredEdgeId && isNodeHoverDimmed));
  const effectiveDimmed = isDimmed || isHoverDimmed;

  // Determine edge state for theme
  const edgeState: EdgeState = isSelected
    ? 'selected'
    : isHovered
    ? 'highlighted'
    : effectiveDimmed
    ? 'muted'
    : 'default';

  // Use new hooks
  const { theme } = useEdgeTheme(relationType, {
    state: edgeState,
    isSelected: !!isSelected,
    isHighlighted: isHovered,
  });

  // Visibility culling and performance mode
  const { isVisible, registerEdge, unregisterEdge, useSimplifiedEffects, disableAnimations } = useEdgeVisibility();
  useEffect(() => {
    const element = pathRef.current;
    if (element) {
      registerEdge(id, element);
      return () => unregisterEdge(id, element);
    }
  }, [id, registerEdge, unregisterEdge]);
  const isEdgeVisible = isVisible(id);

  // Extract node positions
  const sourceX = sourceNode?.internals.positionAbsolute.x ?? 0;
  const sourceY = sourceNode?.internals.positionAbsolute.y ?? 0;
  const sourceWidth = sourceNode?.measured?.width ?? 200;
  const sourceHeight = sourceNode?.measured?.height ?? 100;
  const targetX = targetNode?.internals.positionAbsolute.x ?? 0;
  const targetY = targetNode?.internals.positionAbsolute.y ?? 0;
  const targetWidth = targetNode?.measured?.width ?? 200;
  const targetHeight = targetNode?.measured?.height ?? 100;

  // Calculate edge path (with parallel offset support v11.6.1)
  const { edgePath, reversedPath, edgeLength, sourcePoint, targetPoint } = useMemo(() => {
    if (!sourceNode || !targetNode) {
      return { edgePath: '', reversedPath: '', edgeLength: 0, sourcePoint: { x: 0, y: 0 }, targetPoint: { x: 0, y: 0 } };
    }

    const sourceCenter = { x: sourceX + sourceWidth / 2, y: sourceY + sourceHeight / 2 };
    const targetCenter = { x: targetX + targetWidth / 2, y: targetY + targetHeight / 2 };

    const sourcePt = getNodeIntersection(sourceCenter, sourceWidth, sourceHeight, targetCenter, 16);
    const targetPt = getNodeIntersection(targetCenter, targetWidth, targetHeight, sourceCenter, 20);

    // Use parallel path offset when edge is part of a parallel group
    const hasParallelInfo = typeof parallelIndex === 'number' && typeof parallelTotal === 'number' && parallelTotal > 1;
    const path = hasParallelInfo
      ? generateParallelPath(sourcePt, targetPt, parallelIndex, parallelTotal)
      : generateCurvedPath(sourcePt, targetPt);
    // Note: reversedPath doesn't use parallel offset (animation direction stays consistent)
    const revPath = generateReversedPath(sourcePt, targetPt);

    const dx = targetPt.x - sourcePt.x;
    const dy = targetPt.y - sourcePt.y;
    const length = Math.sqrt(dx * dx + dy * dy);

    return { edgePath: path, reversedPath: revPath, edgeLength: length, sourcePoint: sourcePt, targetPoint: targetPt };
  }, [sourceNode, targetNode, sourceX, sourceY, sourceWidth, sourceHeight, targetX, targetY, targetWidth, targetHeight, parallelIndex, parallelTotal]);

  // LOD calculation - distance from VIEWPORT CENTER (not world origin)
  // This ensures edges visible on screen get high LOD regardless of graph position
  const distanceFromCenter = useMemo(() => {
    // Edge midpoint in world coordinates
    const edgeMidX = (sourcePoint.x + targetPoint.x) / 2;
    const edgeMidY = (sourcePoint.y + targetPoint.y) / 2;

    // Viewport center in world coordinates
    // transform = [panX, panY, zoom] where pan values shift the viewport
    const viewportCenterX = (viewportWidth / 2 - transform[0]) / zoom;
    const viewportCenterY = (viewportHeight / 2 - transform[1]) / zoom;

    // Distance from viewport center to edge midpoint
    const dx = edgeMidX - viewportCenterX;
    const dy = edgeMidY - viewportCenterY;
    return Math.sqrt(dx * dx + dy * dy);
  }, [sourcePoint, targetPoint, transform, viewportWidth, viewportHeight, zoom]);

  const { tier: _lodTier, shouldRender: _shouldRenderEffects, intensityMultiplier: _intensityMultiplier } = useEdgeLOD({
    distanceFromCenter,
    zoom,
    isSelected: !!isSelected,
    isHighlighted: isHovered,
    isConnectedToSelected: connectsToHoveredNode,
  });

  // Animation budget
  const priority = isSelected ? 'selected' : isHovered ? 'highlighted' : connectsToHoveredNode ? 'connected' : 'default';
  const { canAnimate: _canAnimate } = useAnimationBudget({
    edgeId: id,
    priority,
    enabled: isAnimated && !effectiveDimmed && isEdgeVisible,
  });

  // Cleanup on unmount
  useEffect(() => {
    return () => releaseEdgeAnimationSlot(id);
  }, [id]);

  if (!sourceNode || !targetNode || !edgePath) {
    return null;
  }

  // Visual calculations
  const baseStrokeWidth = 6;
  const finalStrokeWidth = isSelected || isHovered
    ? baseStrokeWidth + 3
    : effectiveDimmed
    ? baseStrokeWidth * 0.5
    : baseStrokeWidth;

  const groupOpacity = isDimmed ? 0.06 : isHoverDimmed ? 0.25 : 1;

  // Label
  const smartLabel = getSmartLabel(relationType, edgeLength);
  const labelText = smartLabel.text ? `${smartLabel.icon} ${smartLabel.text}` : smartLabel.icon;
  const shouldShowLabel = showLabel && !effectiveDimmed && (edgeLength > 30 || isHovered || isSelected);
  const isTextFlipped = sourcePoint.x > targetPoint.x;
  const labelPathId = isTextFlipped ? `edge-path-label-${id}` : `edge-path-${id}`;
  const labelScale = Math.min(2.5, Math.max(0.6, 1 / zoom));
  const baseFontSize = isHovered || isSelected ? 13 : 10;
  const scaledFontSize = baseFontSize * labelScale;

  // PERFORMANCE: Disable animations when edge count exceeds threshold
  // Simplified effects are handled by passing the simplified prop
  // Original: const shouldAnimate = isEdgeVisible && canAnimate && isAnimated && !effectiveDimmed;
  const shouldAnimate = isAnimated && !effectiveDimmed && !disableAnimations;

  return (
    <g className="floating-edge" style={{ opacity: groupOpacity, transition: 'opacity 0.15s ease-out' }}>
      {/* Interaction path */}
      <path d={edgePath} fill="none" stroke="transparent" strokeWidth={40} strokeLinecap="round" style={{ cursor: 'pointer' }} />

      {/* Visibility tracking path */}
      <path ref={pathRef} d={edgePath} fill="none" stroke="transparent" strokeWidth={20} pointerEvents="none" aria-hidden="true" />

      {/* Definitions */}
      <defs>
        <path id={`edge-path-${id}`} d={edgePath} />
        <path id={`edge-path-reversed-${id}`} d={reversedPath} />
        {isTextFlipped && <path id={`edge-path-label-${id}`} d={reversedPath} />}
      </defs>

      {/* Base glow layer */}
      <path
        d={edgePath}
        fill="none"
        stroke={theme.colors.primary}
        strokeWidth={finalStrokeWidth + 4}
        strokeLinecap="round"
        opacity={effectiveDimmed ? 0.2 : 0.35}
      />

      {/* Main edge stroke */}
      <path
        d={edgePath}
        fill="none"
        stroke={theme.colors.primary}
        strokeWidth={finalStrokeWidth}
        strokeLinecap="round"
        opacity={effectiveDimmed ? 0.5 : 0.8}
      />

      {/* Bright center line */}
      {!effectiveDimmed && (
        <path
          d={edgePath}
          fill="none"
          stroke="#ffffff"
          strokeWidth={Math.max(1.5, finalStrokeWidth * 0.25)}
          strokeLinecap="round"
          opacity={0.9}
          style={{ mixBlendMode: 'screen' }}
        />
      )}

      {/* Selection pulse effect - animated glow when edge is selected */}
      {isSelected && (
        <>
          {/* Wide outer glow - creates dramatic "energy field" effect */}
          <path
            d={edgePath}
            fill="none"
            stroke={theme.colors.glow}
            strokeWidth={finalStrokeWidth + 20}
            strokeLinecap="round"
            style={{
              opacity: 0.25,
              filter: 'blur(12px)',
              animation: 'edgeGlowPulse 2s ease-in-out infinite',
            }}
          />
          {/* Main pulse glow */}
          <path
            d={edgePath}
            fill="none"
            stroke={theme.colors.glow}
            strokeWidth={finalStrokeWidth + 12}
            strokeLinecap="round"
            style={{
              opacity: 0.5,
              filter: 'blur(8px)',
              animation: 'edgePulse 1.5s ease-in-out infinite',
            }}
          />
          {/* Inner bright pulse */}
          <path
            d={edgePath}
            fill="none"
            stroke="#ffffff"
            strokeWidth={finalStrokeWidth + 4}
            strokeLinecap="round"
            style={{
              opacity: 0.7,
              filter: 'blur(3px)',
              animation: 'edgePulse 1.5s ease-in-out infinite',
              animationDelay: '0.1s',
            }}
          />
          {/* Source endpoint ring */}
          <circle
            cx={sourcePoint.x}
            cy={sourcePoint.y}
            r={12}
            fill="none"
            stroke={theme.colors.glow}
            strokeWidth={3}
            style={{
              opacity: 0.8,
              filter: `drop-shadow(0 0 8px ${theme.colors.glow})`,
              animation: 'endpointPulse 1.5s ease-in-out infinite',
            }}
          />
          {/* Target endpoint ring */}
          <circle
            cx={targetPoint.x}
            cy={targetPoint.y}
            r={12}
            fill="none"
            stroke={theme.colors.glow}
            strokeWidth={3}
            style={{
              opacity: 0.8,
              filter: `drop-shadow(0 0 8px ${theme.colors.glow})`,
              animation: 'endpointPulse 1.5s ease-in-out infinite',
              animationDelay: '0.75s',
            }}
          />
          {/* Source endpoint dot */}
          <circle
            cx={sourcePoint.x}
            cy={sourcePoint.y}
            r={5}
            fill={theme.colors.primary}
            style={{
              filter: `drop-shadow(0 0 6px ${theme.colors.glow})`,
            }}
          />
          {/* Target endpoint dot */}
          <circle
            cx={targetPoint.x}
            cy={targetPoint.y}
            r={5}
            fill={theme.colors.primary}
            style={{
              filter: `drop-shadow(0 0 6px ${theme.colors.glow})`,
            }}
          />
          {/* CSS keyframes injected via style tag */}
          <style>
            {`
              @keyframes edgePulse {
                0%, 100% { opacity: 0.4; }
                50% { opacity: 0.8; }
              }
              @keyframes edgeGlowPulse {
                0%, 100% { opacity: 0.15; }
                50% { opacity: 0.35; }
              }
              @keyframes endpointPulse {
                0%, 100% { r: 10; opacity: 0.6; }
                50% { r: 16; opacity: 1; }
              }
            `}
          </style>
        </>
      )}

      {/* INLINE ANIMATED EFFECTS - working pattern using inline path attribute */}
      {/* Each arc family gets different visual treatment */}
      {shouldAnimate && (
        <InlineEdgeEffects
          edgePath={edgePath}
          relationType={relationType}
          colors={theme.colors}
          state={edgeState}
          simplified={useSimplifiedEffects}
        />
      )}

      {/* Label */}
      {shouldShowLabel && (
        <g style={{ pointerEvents: 'none' }}>
          <text
            style={{
              fontSize: `${scaledFontSize}px`,
              fontWeight: 600,
              fill: 'transparent',
              fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
              letterSpacing: '0.05em',
              textTransform: 'uppercase',
              stroke: 'rgba(0, 0, 0, 0.95)',
              strokeWidth: scaledFontSize * 0.15,
              strokeLinejoin: 'round',
              paintOrder: 'stroke fill',
            }}
          >
            <textPath href={`#${labelPathId}`} startOffset="50%" textAnchor="middle">{labelText}</textPath>
          </text>
          <text
            style={{
              fontSize: `${scaledFontSize}px`,
              fontWeight: 600,
              fill: theme.colors.glow,
              fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
              letterSpacing: '0.05em',
              textTransform: 'uppercase',
              filter: `blur(${isHovered || isSelected ? 3 : 1.5}px)`,
              opacity: isHovered || isSelected ? 0.5 : 0.25,
            }}
          >
            <textPath href={`#${labelPathId}`} startOffset="50%" textAnchor="middle">{labelText}</textPath>
          </text>
          <text
            style={{
              fontSize: `${scaledFontSize}px`,
              fontWeight: 600,
              fill: 'rgba(255, 255, 255, 0.98)',
              fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
              letterSpacing: '0.05em',
              textTransform: 'uppercase',
            }}
          >
            <textPath href={`#${labelPathId}`} startOffset="50%" textAnchor="middle">{labelText}</textPath>
          </text>
        </g>
      )}
    </g>
  );
}, arePropsEqual);
