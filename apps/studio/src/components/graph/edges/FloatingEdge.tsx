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
import { useEdgeVisibility, type EffectTier } from './EdgeVisibilityManager';

// New modular system
import { useEdgeTheme } from './hooks/useEdgeTheme';
import { useEdgeLOD } from './hooks/useEdgeLOD';
import { useAnimationBudget } from './hooks/useAnimationBudget';
import { releaseEdgeAnimationSlot } from './effects/EffectRenderer';
import { getSmartLabel, getNodeIntersection, generateCurvedPath, generateReversedPath, generateParallelPath } from './EdgeUtils';
import type { EdgeState } from './system/types';

// Arc family detection with comprehensive mapping (60+ relation types)
import { getArcFamily } from './system/arcFamilyPalettes';

// Signature effects (v11.6.2 - Full Redesign)
import { PowerConduit, DNAHelix, SynapticFiring, MatrixCodeRain, SonarPulse } from './effects';

// Note: EffectRenderer disabled for now - using InlineEdgeEffects instead (working pattern)

// =============================================================================
// Inline Edge Effects (Working Pattern)
// =============================================================================

interface InlineEdgeEffectsProps {
  edgePath: string;
  relationType: string;
  colors: { primary: string; secondary: string; glow: string };
  state: EdgeState;
  /** Effect quality tier based on total edge count */
  effectTier: EffectTier;
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
}: Omit<InlineEdgeEffectsProps, 'relationType' | 'effectTier'>) {
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
 * InlineEdgeEffects - SIGNATURE EFFECTS for NovaNet arcs (v4)
 *
 * TIER-BASED RENDERING - Effect richness scales with graph size:
 * - ULTRA (0-30 arcs):   Maximum wow - full signature effects
 * - HIGH (30-100 arcs):  Full signature effects
 * - MEDIUM (100-250):    Reduced signature effects
 * - LOW (250-500):       SimplifiedEdgeEffect (fallback)
 * - MINIMAL (500+):      No animation (handled outside)
 *
 * Each arc family has a UNIQUE SIGNATURE EFFECT:
 * - ownership: ⚡ POWER CONDUIT - High-voltage cable with orb convoy
 * - localization: 🧬 DNA HELIX - Double strand with base pairs
 * - semantic: 🔗 SYNAPTIC FIRING - Neural firing pulses
 * - generation: 💻 MATRIX CODE RAIN - Falling characters + scanline
 * - mining: 📡 SONAR PULSE - Ping + expanding rings + echo
 *
 * @see docs/plans/2026-02-11-arc-effects-redesign.md
 */
const InlineEdgeEffects = memo(function InlineEdgeEffects({
  edgePath,
  relationType,
  colors,
  state,
  effectTier,
}: InlineEdgeEffectsProps) {
  // LOW tier: Use simplified 2-element effect
  if (effectTier === 'low') {
    return <SimplifiedEdgeEffect edgePath={edgePath} colors={colors} state={state} />;
  }

  const family = getArcFamily(relationType);

  // Route to signature effects based on arc family
  switch (family) {
    case 'ownership':
      return <PowerConduit edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} />;

    case 'localization':
      return <DNAHelix edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} />;

    case 'semantic':
      return <SynapticFiring edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} />;

    case 'generation':
      return <MatrixCodeRain edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} />;

    case 'mining':
      return <SonarPulse edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} />;

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
  const { isVisible, registerEdge, unregisterEdge, registerEdgeMeta, getEffectiveTier, disableAnimations } = useEdgeVisibility();

  // Register edge metadata for hub node detection
  useEffect(() => {
    registerEdgeMeta(id, source, target);
  }, [id, source, target, registerEdgeMeta]);

  // Register for viewport visibility tracking
  useEffect(() => {
    const element = pathRef.current;
    if (element) {
      registerEdge(id, element);
      return () => unregisterEdge(id, element);
    }
  }, [id, registerEdge, unregisterEdge]);

  const isEdgeVisible = isVisible(id);
  // Get effective tier (may be downgraded if connected to hub node)
  const effectTier = getEffectiveTier(id);

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
          effectTier={effectTier}
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
