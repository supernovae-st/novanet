'use client';

/**
 * FloatingEdge - Modular edge component with composable effects
 *
 * Architecture (v2.1):
 * - Uses new type system for discriminated unions
 * - Uses theme registry for relation-based styling
 * - Uses LOD controller for zoom-based quality
 * - Uses animation budget for performance limits
 * - Uses EffectRenderer for composable primitives
 *
 * v0.13.1 Enhancements:
 * - Gradient paths based on arc family colors
 * - Animated dash patterns for semantic/generation arcs
 * - Enhanced hover glow with blur and scale
 * - Improved arrow markers with proper sizing
 * - Label styling with frosted glass background
 */

import { memo, useMemo, useEffect, useRef } from 'react';
import { useInternalNode, useStore, type Edge, type EdgeProps } from '@xyflow/react';
import { useUIStore, selectHoveredEdgeId, selectHoveredNodeId, selectSelectedNodeId, selectSelectedEdgeId, selectSelectedEdgeData } from '@/stores/uiStore';
import { useEdgeVisibility, type EffectTier } from './EdgeVisibilityManager';

// New modular system
import { useEdgeTheme } from './hooks/useEdgeTheme';
import { useEdgeLOD } from './hooks/useEdgeLOD';
import { useAnimationBudget } from './hooks/useAnimationBudget';
import { releaseEdgeAnimationSlot } from './effects/EffectRenderer';
import { getSmartLabel, getNodeIntersection, generateCurvedPath, generateReversedPath, generateParallelPath, generateReversedParallelPath } from './EdgeUtils';
import type { EdgeState } from './system/types';

// Arc family detection from unified palette system (v11.7.0)
import { getArcFamily, getArcStroke, type ArcFamilyKey } from '@/design/colors/palette';

// Signature effects (v11.6.2 - Full Redesign)
import { PowerConduit, DNAHelix, SynapticFiring, MatrixCodeRain, SonarPulse, SelectionEffect } from './effects';

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
  /** Edge ID for deterministic random delay */
  edgeId: string;
}

/**
 * Generate a deterministic "random" delay based on edge ID.
 * Uses a simple hash function to convert string to number.
 * Returns a value between 0 and maxDelay seconds.
 */
function getRandomDelay(edgeId: string, maxDelay: number = 5): number {
  let hash = 0;
  for (let i = 0; i < edgeId.length; i++) {
    const char = edgeId.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32bit integer
  }
  // Normalize to 0-1 range, then multiply by maxDelay
  return Math.abs(hash % 1000) / 1000 * maxDelay;
}

/**
 * SimplifiedEdgeEffect - Lightweight atom-like effect for large graphs
 *
 * ATOM DESIGN v3 (v11.6.4 - PERFORMANCE OPTIMIZED):
 * - ULTRA simplified: Single animated particle with glow
 * - NO trails, NO wobble, NO multiple layers
 * - Just 2 SVG elements (core + glow) instead of 7
 * - v11.6.3: Random start delay based on edge ID for natural staggering
 *
 * Performance: ~2 SVG elements per edge (was 7)
 */
const SimplifiedEdgeEffect = memo(function SimplifiedEdgeEffect({
  edgePath,
  colors,
  state,
  edgeId,
}: Omit<InlineEdgeEffectsProps, 'relationType' | 'effectTier'>) {
  const isHighlighted = state === 'highlighted' || state === 'selected';
  const size = isHighlighted ? 10 : 8;
  const duration = 6; // Slow, steady movement
  const startDelay = getRandomDelay(edgeId, duration);

  return (
    <g className="effect-simplified-atom">
      {/* Glow */}
      <circle
        r={size * 1.5}
        fill={colors.glow}
        opacity={0.4}
      >
        <animateMotion dur={`${duration}s`} repeatCount="indefinite" begin={`${startDelay}s`} path={edgePath} />
      </circle>
      {/* Core particle */}
      <circle
        r={size}
        fill={colors.primary}
        opacity={0.9}
      >
        <animateMotion dur={`${duration}s`} repeatCount="indefinite" begin={`${startDelay}s`} path={edgePath} />
      </circle>
    </g>
  );
});

/**
 * InlineEdgeEffects - SIGNATURE EFFECTS for NovaNet arcs (v4)
 *
 * TIER-BASED RENDERING - Effect richness scales with graph size:
 * - ULTRA (0-10 arcs):   Maximum wow - full signature effects
 * - HIGH (10-25 arcs):   Full signature effects
 * - MEDIUM (25-40):      SimplifiedEdgeEffect (v11.6.4: downgraded for perf)
 * - LOW (40-60):         SimplifiedEdgeEffect
 * - MINIMAL (60+):       No animation (handled outside)
 *
 * Each arc family has a UNIQUE SIGNATURE EFFECT (ULTRA/HIGH only):
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
  edgeId,
}: InlineEdgeEffectsProps) {
  // v11.6.4: MEDIUM and LOW tiers use simplified 2-element effect for performance
  // Only ULTRA and HIGH get full signature effects
  if (effectTier === 'low' || effectTier === 'medium') {
    return <SimplifiedEdgeEffect edgePath={edgePath} colors={colors} state={state} edgeId={edgeId} />;
  }

  const family = getArcFamily(relationType);

  // Route to signature effects based on arc family (ULTRA/HIGH only)
  // v11.6.3: Pass edgeId for random delay staggering
  switch (family) {
    case 'ownership':
      return <PowerConduit edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} edgeId={edgeId} />;

    case 'localization':
      return <DNAHelix edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} edgeId={edgeId} />;

    case 'semantic':
      return <SynapticFiring edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} edgeId={edgeId} />;

    case 'generation':
      return <MatrixCodeRain edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} edgeId={edgeId} />;

    case 'mining':
      return <SonarPulse edgePath={edgePath} colors={colors} state={state} effectTier={effectTier} edgeId={edgeId} />;

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
  /** Cardinality from arc definition (v0.13.1: ADR-027) */
  cardinality?: '1:1' | '1:N' | 'N:1' | 'N:M' | string;
  /** Arc family (ownership, localization, semantic, generation, mining) */
  arcFamily?: string;
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
  const selectedEdgeData = useUIStore(selectSelectedEdgeData);

  // Check if this edge is a sibling of the selected edge (same source+target but different edge)
  // Handles both directions: A→B and B→A are considered siblings
  const isSiblingOfSelected = selectedEdgeData !== null &&
    selectedEdgeId !== id &&
    ((source === selectedEdgeData.source && target === selectedEdgeData.target) ||
     (source === selectedEdgeData.target && target === selectedEdgeData.source));

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
  // Cardinality info (v0.13.1: ADR-027)
  const cardinality = data?.cardinality;

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
  // v11.6.4: FORCE ULTRA tier when edge is selected (one edge = full wow effect)
  const baseTier = getEffectiveTier(id);
  const effectTier = isSelected ? 'ultra' : baseTier;

  // Extract node positions (defensive: internals/positionAbsolute may be undefined during mount)
  const sourceX = sourceNode?.internals?.positionAbsolute?.x ?? 0;
  const sourceY = sourceNode?.internals?.positionAbsolute?.y ?? 0;
  const sourceWidth = sourceNode?.measured?.width ?? 200;
  const sourceHeight = sourceNode?.measured?.height ?? 100;
  const targetX = targetNode?.internals?.positionAbsolute?.x ?? 0;
  const targetY = targetNode?.internals?.positionAbsolute?.y ?? 0;
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
    // v11.6.1: Reversed path also uses parallel offset for correct label positioning
    const revPath = hasParallelInfo
      ? generateReversedParallelPath(sourcePt, targetPt, parallelIndex, parallelTotal)
      : generateReversedPath(sourcePt, targetPt);

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

  // Get arc family for styling (v0.13.1)
  const arcFamily = getArcFamily(relationType);
  const arcStroke = getArcStroke(relationType);

  // Visual calculations
  const baseStrokeWidth = 6;
  const finalStrokeWidth = isSelected || isHovered
    ? baseStrokeWidth + 3
    : effectiveDimmed
    ? baseStrokeWidth * 0.5
    : baseStrokeWidth;

  // Opacity hierarchy: focus dimmed (6%) < hover dimmed (25%) < sibling dimmed (50%) < full (100%)
  const groupOpacity = isDimmed ? 0.06 : isHoverDimmed ? 0.25 : isSiblingOfSelected ? 0.5 : 1;

  // Animated dash patterns for semantic/generation arcs (v0.13.1)
  const useAnimatedDash = !effectiveDimmed && (arcFamily === 'semantic' || arcFamily === 'generation' || arcFamily === 'localization');
  const dashArray = arcStroke.dashArray || undefined;

  // Unique gradient/marker IDs for this edge
  const gradientId = `edge-gradient-${id}`;
  const markerId = `edge-marker-${id}`;
  const glowFilterId = `edge-glow-${id}`;

  // Label
  const smartLabel = getSmartLabel(relationType, edgeLength, cardinality);
  // v11.6.9: Flip arrow direction when text path is reversed (so arrow still points source→target)
  const isTextFlipped = sourcePoint.x > targetPoint.x;
  const directionIcon = isTextFlipped ? '←' : '→';
  const labelText = smartLabel.text ? `${directionIcon} ${smartLabel.text}` : directionIcon;
  // v0.13.1: Cardinality badge (shown when edge is hovered/selected)
  const showCardinality = smartLabel.cardinality && (isHovered || isSelected);
  const shouldShowLabel = showLabel && !effectiveDimmed && (edgeLength > 30 || isHovered || isSelected);
  const labelPathId = isTextFlipped ? `edge-path-label-${id}` : `edge-path-${id}`;
  const labelScale = Math.min(2.5, Math.max(0.6, 1 / zoom));
  const baseFontSize = isHovered || isSelected ? 13 : 10;
  const scaledFontSize = baseFontSize * labelScale;

  // v11.6.1: Stagger label positions for parallel edges so they don't overlap
  // Single edge: 50%, Two edges: 40%/60%, Three edges: 35%/50%/65%
  const hasParallelEdges = typeof parallelIndex === 'number' && typeof parallelTotal === 'number' && parallelTotal > 1;
  let labelStartOffset = '50%';
  if (hasParallelEdges) {
    // Spread labels from 35% to 65% of the path
    const spread = 30; // Total spread in percentage points
    const step = spread / (parallelTotal - 1);
    const offsetPercent = 35 + (parallelIndex * step);
    labelStartOffset = `${offsetPercent}%`;
  }

  // PERFORMANCE: Disable animations when edge count exceeds threshold
  // Simplified effects are handled by passing the simplified prop
  // v11.6.4: ALWAYS animate selected edge (even when global animations disabled)
  // This gives the "wow" effect when focusing on a single edge
  // v11.6.10: Disable animation for sibling edges (same endpoints) to emphasize selected edge
  const shouldAnimate = isAnimated && !effectiveDimmed && !isSiblingOfSelected && (!disableAnimations || isSelected);

  // PERFORMANCE: Disable opacity transition for large graphs to prevent flash effect
  // When many edges update simultaneously, the transition causes a visible "blink"
  // Only apply smooth transitions for small graphs (ultra/high tier = < 100 edges)
  const useOpacityTransition = effectTier === 'ultra' || effectTier === 'high';

  return (
    <g className="floating-edge" style={{ opacity: groupOpacity, transition: useOpacityTransition ? 'opacity 0.15s ease-out' : 'none' }}>
      {/* Interaction path */}
      <path d={edgePath} fill="none" stroke="transparent" strokeWidth={40} strokeLinecap="round" style={{ cursor: 'pointer' }} />

      {/* Visibility tracking path */}
      <path ref={pathRef} d={edgePath} fill="none" stroke="transparent" strokeWidth={20} pointerEvents="none" aria-hidden="true" />

      {/* Definitions - Gradients, Markers, Filters (v0.13.1) */}
      <defs>
        {/* Path definitions */}
        <path id={`edge-path-${id}`} d={edgePath} />
        <path id={`edge-path-reversed-${id}`} d={reversedPath} />
        {isTextFlipped && <path id={`edge-path-label-${id}`} d={reversedPath} />}

        {/* Gradient along path based on arc family colors */}
        <linearGradient
          id={gradientId}
          gradientUnits="userSpaceOnUse"
          x1={sourcePoint.x}
          y1={sourcePoint.y}
          x2={targetPoint.x}
          y2={targetPoint.y}
        >
          <stop offset="0%" stopColor={theme.colors.secondary} stopOpacity={0.7} />
          <stop offset="35%" stopColor={theme.colors.primary} stopOpacity={1} />
          <stop offset="65%" stopColor={theme.colors.primary} stopOpacity={1} />
          <stop offset="100%" stopColor={theme.colors.tertiary} stopOpacity={0.8} />
        </linearGradient>

        {/* Arrow marker with proper sizing */}
        <marker
          id={markerId}
          markerWidth={isHovered || isSelected ? 14 : 10}
          markerHeight={isHovered || isSelected ? 14 : 10}
          refX={isHovered || isSelected ? 11 : 8}
          refY={isHovered || isSelected ? 7 : 5}
          orient="auto"
          markerUnits="userSpaceOnUse"
        >
          <path
            d={isHovered || isSelected
              ? "M 0 0 L 14 7 L 0 14 L 3 7 Z"
              : "M 0 0 L 10 5 L 0 10 L 2 5 Z"
            }
            fill={isHovered || isSelected ? '#ffffff' : theme.colors.primary}
            style={{
              filter: isHovered || isSelected ? `drop-shadow(0 0 3px ${theme.colors.glow})` : 'none',
            }}
          />
        </marker>

        {/* Glow filter for hover/selection effect */}
        <filter id={glowFilterId} x="-50%" y="-50%" width="200%" height="200%">
          <feGaussianBlur in="SourceGraphic" stdDeviation={isHovered ? 6 : isSelected ? 8 : 3} result="blur" />
          <feColorMatrix
            in="blur"
            type="matrix"
            values={`1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 ${isHovered ? 0.8 : isSelected ? 1 : 0.5} 0`}
          />
          <feMerge>
            <feMergeNode />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>

        {/* Frosted glass filter for labels (v0.13.1) */}
        <filter id={`label-glass-${id}`} x="-20%" y="-30%" width="140%" height="160%">
          <feGaussianBlur in="SourceGraphic" stdDeviation="4" result="blur" />
          <feColorMatrix
            in="blur"
            type="matrix"
            values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 0.85 0"
          />
        </filter>
      </defs>

      {/* Outer glow layer (v0.13.1 - enhanced with filter) */}
      {(isHovered || isSelected) && (
        <path
          d={edgePath}
          fill="none"
          stroke={theme.colors.glow}
          strokeWidth={finalStrokeWidth + 20}
          strokeLinecap="round"
          opacity={0.3}
          style={{ filter: `blur(12px)` }}
        />
      )}

      {/* Base glow layer */}
      <path
        d={edgePath}
        fill="none"
        stroke={theme.colors.primary}
        strokeWidth={finalStrokeWidth + 6}
        strokeLinecap="round"
        opacity={effectiveDimmed ? 0.15 : isHovered ? 0.5 : 0.3}
        style={{
          filter: isHovered || isSelected ? `url(#${glowFilterId})` : 'none',
          transition: 'opacity 0.2s ease-out',
        }}
      />

      {/* Main edge stroke with gradient (v0.13.1) */}
      <path
        d={edgePath}
        fill="none"
        stroke={effectiveDimmed ? theme.colors.primary : `url(#${gradientId})`}
        strokeWidth={finalStrokeWidth}
        strokeLinecap="round"
        strokeDasharray={useAnimatedDash ? dashArray : undefined}
        markerEnd={!effectiveDimmed ? `url(#${markerId})` : undefined}
        opacity={effectiveDimmed ? 0.4 : 0.9}
        style={{
          transition: 'stroke-width 0.2s ease-out, opacity 0.15s ease-out',
        }}
      >
        {/* Animated dash for semantic/generation/localization arcs */}
        {useAnimatedDash && dashArray && (
          <animate
            attributeName="stroke-dashoffset"
            values={arcFamily === 'generation' ? '0;-20' : '0;20'}
            dur={arcFamily === 'generation' ? '0.8s' : arcFamily === 'semantic' ? '1.2s' : '1.5s'}
            repeatCount="indefinite"
          />
        )}
      </path>

      {/* Bright center line - enhanced glow on hover */}
      {!effectiveDimmed && (
        <path
          d={edgePath}
          fill="none"
          stroke="#ffffff"
          strokeWidth={Math.max(1.5, finalStrokeWidth * (isHovered || isSelected ? 0.35 : 0.25))}
          strokeLinecap="round"
          opacity={isHovered || isSelected ? 0.95 : 0.85}
          style={{
            mixBlendMode: 'screen',
            filter: isHovered || isSelected ? 'blur(0.5px)' : 'none',
          }}
        />
      )}

      {/* Secondary highlight line for depth (v0.13.1) */}
      {!effectiveDimmed && (isHovered || isSelected) && (
        <path
          d={edgePath}
          fill="none"
          stroke={theme.colors.tertiary}
          strokeWidth={Math.max(2, finalStrokeWidth * 0.4)}
          strokeLinecap="round"
          opacity={0.6}
          style={{
            filter: 'blur(1px)',
            mixBlendMode: 'overlay',
          }}
        />
      )}

      {/* Selection effect - v11.6.5: Selection UX Polish */}
      {/* Includes: direction indicator, source/target differentiation, smooth transitions */}
      {isSelected && (
        <SelectionEffect
          edgePath={edgePath}
          sourcePoint={sourcePoint}
          targetPoint={targetPoint}
          colors={{ primary: theme.colors.primary, glow: theme.colors.glow }}
          strokeWidth={finalStrokeWidth}
          edgeId={id}
        />
      )}

      {/* INLINE ANIMATED EFFECTS - working pattern using inline path attribute */}
      {/* Each arc family gets different visual treatment */}
      {/* v11.6.3: Pass edgeId for random start delay staggering */}
      {/* v11.6.8: Disabled when selected - SelectionEffect has its own particle system */}
      {shouldAnimate && !isSelected && (
        <InlineEdgeEffects
          edgePath={edgePath}
          relationType={relationType}
          colors={theme.colors}
          state={edgeState}
          effectTier={effectTier}
          edgeId={id}
        />
      )}

      {/* Label - v0.13.1: Enhanced with frosted glass background and improved styling */}
      {shouldShowLabel && (
        <g style={{ pointerEvents: 'none' }}>
          {/* Frosted glass background (v0.13.1) */}
          {(isHovered || isSelected) && (
            <>
              {/* Background blur pill */}
              <text
                style={{
                  fontSize: `${scaledFontSize}px`,
                  fontWeight: 700,
                  fill: 'none',
                  stroke: 'rgba(10, 10, 20, 0.75)',
                  strokeWidth: scaledFontSize * 0.8,
                  strokeLinecap: 'round',
                  strokeLinejoin: 'round',
                  fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
                  letterSpacing: '0.05em',
                  textTransform: 'uppercase',
                  filter: 'blur(3px)',
                }}
              >
                <textPath href={`#${labelPathId}`} startOffset={labelStartOffset} textAnchor="middle">{labelText}</textPath>
              </text>
              {/* Glass edge highlight */}
              <text
                style={{
                  fontSize: `${scaledFontSize}px`,
                  fontWeight: 700,
                  fill: 'none',
                  stroke: `rgba(255, 255, 255, 0.15)`,
                  strokeWidth: scaledFontSize * 0.6,
                  strokeLinecap: 'round',
                  strokeLinejoin: 'round',
                  fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
                  letterSpacing: '0.05em',
                  textTransform: 'uppercase',
                }}
              >
                <textPath href={`#${labelPathId}`} startOffset={labelStartOffset} textAnchor="middle">{labelText}</textPath>
              </text>
            </>
          )}
          {/* Dark stroke outline for readability */}
          <text
            style={{
              fontSize: `${scaledFontSize}px`,
              fontWeight: 700,
              fill: 'transparent',
              fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
              letterSpacing: '0.05em',
              textTransform: 'uppercase',
              stroke: 'rgba(0, 0, 0, 0.9)',
              strokeWidth: scaledFontSize * 0.22,
              strokeLinejoin: 'round',
              paintOrder: 'stroke fill',
            }}
          >
            <textPath href={`#${labelPathId}`} startOffset={labelStartOffset} textAnchor="middle">{labelText}</textPath>
          </text>
          {/* Glow layer matching edge color - enhanced for hover */}
          <text
            style={{
              fontSize: `${scaledFontSize}px`,
              fontWeight: 700,
              fill: theme.colors.glow,
              fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
              letterSpacing: '0.05em',
              textTransform: 'uppercase',
              filter: `blur(${isHovered || isSelected ? 5 : 2}px)`,
              opacity: isHovered || isSelected ? 0.85 : 0.45,
            }}
          >
            <textPath href={`#${labelPathId}`} startOffset={labelStartOffset} textAnchor="middle">{labelText}</textPath>
          </text>
          {/* Main label - white on hover/select, colored otherwise */}
          <text
            style={{
              fontSize: `${scaledFontSize}px`,
              fontWeight: 700,
              fill: isHovered || isSelected ? '#ffffff' : theme.colors.primary,
              fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
              letterSpacing: '0.05em',
              textTransform: 'uppercase',
              textShadow: isHovered || isSelected ? `0 0 8px ${theme.colors.glow}` : 'none',
            }}
          >
            <textPath href={`#${labelPathId}`} startOffset={labelStartOffset} textAnchor="middle">{labelText}</textPath>
          </text>
        </g>
      )}

      {/* Cardinality badge - v0.13.1: ADR-027 visual encoding with enhanced styling */}
      {/* Shows cardinality (1:1, 1:N, N:1, N:M) near target when edge is hovered/selected */}
      {showCardinality && (
        <g
          style={{ pointerEvents: 'none' }}
          transform={`translate(${targetPoint.x}, ${targetPoint.y})`}
        >
          {/* Badge glow */}
          <rect
            x={-22 * labelScale}
            y={-30 * labelScale}
            width={44 * labelScale}
            height={22 * labelScale}
            rx={6 * labelScale}
            fill={theme.colors.glow}
            opacity={0.3}
            style={{ filter: 'blur(4px)' }}
          />
          {/* Badge background with gradient border */}
          <rect
            x={-20 * labelScale}
            y={-28 * labelScale}
            width={40 * labelScale}
            height={18 * labelScale}
            rx={4 * labelScale}
            fill="rgba(5, 5, 15, 0.9)"
            stroke={`url(#${gradientId})`}
            strokeWidth={1.5}
          />
          {/* Inner highlight */}
          <rect
            x={-18 * labelScale}
            y={-26 * labelScale}
            width={36 * labelScale}
            height={14 * labelScale}
            rx={3 * labelScale}
            fill="none"
            stroke="rgba(255, 255, 255, 0.1)"
            strokeWidth={0.5}
          />
          {/* Badge text */}
          <text
            x={0}
            y={-16 * labelScale}
            textAnchor="middle"
            dominantBaseline="middle"
            style={{
              fontSize: `${10 * labelScale}px`,
              fontWeight: 700,
              fill: '#ffffff',
              fontFamily: 'JetBrains Mono, monospace',
              letterSpacing: '0.1em',
              textShadow: `0 0 6px ${theme.colors.glow}`,
            }}
          >
            {smartLabel.cardinality}
          </text>
        </g>
      )}
    </g>
  );
}, arePropsEqual);
