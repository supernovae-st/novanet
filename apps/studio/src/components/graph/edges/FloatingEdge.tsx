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
import { getArcFamily, type ArcFamily } from './system/arcFamilyPalettes';

// Note: EffectRenderer disabled for now - using InlineEdgeEffects instead (working pattern)

// =============================================================================
// Inline Edge Effects (Working Pattern)
// =============================================================================

interface InlineEdgeEffectsProps {
  edgePath: string;
  relationType: string;
  colors: { primary: string; secondary: string; glow: string };
  state: EdgeState;
}

/**
 * InlineEdgeEffects - Advanced animated effects with high visibility
 *
 * Each arc family has a distinct, elaborate visual style:
 * - ownership: ⚡ BLUE Energy Pulse - Multi-layer glow with trail (power flows to children)
 * - localization: 🧬 GREEN DNA Helix - Double spiral oscillation (content DNA adapts)
 * - semantic: 🔗 ORANGE Neural Sparks - Fast zigzag synapses (meaning connections)
 * - generation: 💻 PURPLE Matrix Code - Flowing characters (AI processing data)
 * - mining: 📡 PINK Radar Sweep - Scanning gradient (discovery)
 *
 * Colors come from taxonomy.yaml via arcFamilyPalettes.ts
 */
const InlineEdgeEffects = memo(function InlineEdgeEffects({
  edgePath,
  relationType,
  colors,
  state,
}: InlineEdgeEffectsProps) {
  const family = getArcFamily(relationType);
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // MUCH larger sizes for visibility - bigger when highlighted
  const baseSize = isHighlighted ? 16 : 12;
  const baseDuration = isHighlighted ? 1.0 : 1.6;

  // Debug: uncomment to see arc family detection in console
  // console.log(`[ArcEffect] ${relationType} → ${family} (${colors.primary})`);


  switch (family) {
    case 'ownership':
      // ⚡ ENERGY PULSE - Multi-layer intense glow with trailing segments
      // Visual: Bright power packets flowing from parent to children
      return (
        <g className="effect-ownership">
          {/* Pulse group 1 - Leader pulse */}
          <g>
            {/* Outer glow layer (blur 12px) */}
            <circle r={baseSize * 2.5} fill={colors.glow} opacity={0.4} style={{ filter: 'blur(8px)' }}>
              <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
            {/* Middle glow layer (blur 6px) */}
            <circle r={baseSize * 1.8} fill={colors.primary} opacity={0.6} style={{ filter: 'blur(4px)' }}>
              <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
            {/* Core pulse (solid) */}
            <circle r={baseSize} fill={colors.primary} opacity={0.95} style={{ filter: `drop-shadow(0 0 8px ${colors.glow})` }}>
              <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
            {/* White hot center */}
            <circle r={baseSize * 0.4} fill="#ffffff" opacity={1}>
              <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" path={edgePath} />
            </circle>
          </g>

          {/* Trail segments - 5 decreasing particles behind leader */}
          {[1, 2, 3, 4, 5].map((i) => (
            <circle
              key={`trail-${i}`}
              r={baseSize * (1 - i * 0.15)}
              fill={colors.glow}
              opacity={0.8 - i * 0.12}
              style={{ filter: `drop-shadow(0 0 ${6 - i}px ${colors.glow})` }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${i * 0.08}s`}
                path={edgePath}
              />
            </circle>
          ))}

          {/* Secondary pulse - offset timing */}
          <circle r={baseSize * 0.7} fill={colors.secondary} opacity={0.85} style={{ filter: `drop-shadow(0 0 6px ${colors.glow})` }}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" begin={`${baseDuration / 2}s`} path={edgePath} />
          </circle>
          <circle r={baseSize * 0.25} fill="#ffffff" opacity={0.9}>
            <animateMotion dur={`${baseDuration}s`} repeatCount="indefinite" begin={`${baseDuration / 2}s`} path={edgePath} />
          </circle>
        </g>
      );

    case 'localization':
      // 🧬 DNA HELIX - Double strand with oscillating offset
      // Visual: Two intertwining streams representing content adaptation
      return (
        <g className="effect-localization">
          {/* Strand 1 - Primary color */}
          {[0, 1, 2, 3, 4, 5, 6, 7].map((i) => (
            <circle
              key={`strand1-${i}`}
              r={baseSize * 0.6}
              fill={colors.primary}
              opacity={0.9}
              style={{ filter: `drop-shadow(0 0 4px ${colors.glow})` }}
            >
              <animateMotion dur={`${baseDuration * 1.5}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.18}s`} path={edgePath} />
              {/* Oscillate perpendicular to path */}
              <animate attributeName="cy" values="-12;12;-12" dur="0.6s" repeatCount="indefinite" begin={`${i * 0.075}s`} />
            </circle>
          ))}

          {/* Strand 2 - Secondary color (opposite phase) */}
          {[0, 1, 2, 3, 4, 5, 6, 7].map((i) => (
            <circle
              key={`strand2-${i}`}
              r={baseSize * 0.5}
              fill={colors.secondary}
              opacity={0.85}
              style={{ filter: `drop-shadow(0 0 3px ${colors.secondary})` }}
            >
              <animateMotion dur={`${baseDuration * 1.5}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.18}s`} path={edgePath} />
              {/* Opposite oscillation */}
              <animate attributeName="cy" values="12;-12;12" dur="0.6s" repeatCount="indefinite" begin={`${i * 0.075}s`} />
            </circle>
          ))}

          {/* Connecting "rungs" - white dots between strands */}
          {[0, 1, 2, 3].map((i) => (
            <circle
              key={`rung-${i}`}
              r={baseSize * 0.25}
              fill="#ffffff"
              opacity={0.7}
            >
              <animateMotion dur={`${baseDuration * 1.5}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.36}s`} path={edgePath} />
            </circle>
          ))}
        </g>
      );

    case 'semantic':
      // 🔗 NEURAL SPARKS - Fast zigzag synapses firing
      // Visual: Rapid electrical impulses representing meaning connections
      return (
        <g className="effect-semantic">
          {/* Fast primary sparks */}
          {[0, 1, 2, 3, 4, 5, 6, 7].map((i) => (
            <g key={`spark-${i}`}>
              {/* Spark core */}
              <circle
                r={baseSize * 0.45}
                fill={colors.primary}
                opacity={0.95}
                style={{ filter: `drop-shadow(0 0 6px ${colors.glow})` }}
              >
                <animateMotion dur={`${baseDuration * 0.6}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.075}s`} path={edgePath} />
              </circle>
              {/* Spark trail */}
              <circle
                r={baseSize * 0.3}
                fill={colors.glow}
                opacity={0.6}
              >
                <animateMotion dur={`${baseDuration * 0.6}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.075 + 0.05}s`} path={edgePath} />
              </circle>
            </g>
          ))}

          {/* Zigzag glow particles */}
          {[0, 1, 2, 3].map((i) => (
            <circle
              key={`glow-${i}`}
              r={baseSize * 0.7}
              fill={colors.glow}
              opacity={0.4}
              style={{ filter: 'blur(3px)' }}
            >
              <animateMotion dur={`${baseDuration * 0.6}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.15}s`} path={edgePath} />
              {/* Random oscillation for zigzag effect */}
              <animate attributeName="cx" values="-8;8;-8" dur="0.2s" repeatCount="indefinite" />
              <animate attributeName="cy" values="8;-8;8" dur="0.15s" repeatCount="indefinite" />
            </circle>
          ))}
        </g>
      );

    case 'generation':
      // 💻 MATRIX CODE - Flowing data characters
      // Visual: Digital rain effect representing AI/LLM processing
      return (
        <g className="effect-generation">
          {/* Matrix character blocks */}
          {[0, 1, 2, 3, 4, 5].map((i) => (
            <g key={`char-${i}`}>
              {/* Character glow background */}
              <rect
                x={-baseSize * 0.4}
                y={-baseSize * 0.6}
                width={baseSize * 0.8}
                height={baseSize * 1.2}
                fill={colors.glow}
                opacity={0.5}
                rx={2}
                style={{ filter: 'blur(2px)' }}
              >
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.15}s`} path={edgePath} rotate="auto" />
              </rect>
              {/* Character block */}
              <rect
                x={-baseSize * 0.3}
                y={-baseSize * 0.5}
                width={baseSize * 0.6}
                height={baseSize}
                fill={colors.primary}
                opacity={0.95}
                rx={1}
                style={{ filter: `drop-shadow(0 0 4px ${colors.glow})` }}
              >
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.15}s`} path={edgePath} rotate="auto" />
              </rect>
              {/* Bright center line (simulates character) */}
              <rect
                x={-baseSize * 0.1}
                y={-baseSize * 0.35}
                width={baseSize * 0.2}
                height={baseSize * 0.7}
                fill="#ffffff"
                opacity={0.9}
              >
                <animateMotion dur={`${baseDuration * 0.9}s`} repeatCount="indefinite" begin={`${i * baseDuration * 0.15}s`} path={edgePath} rotate="auto" />
                {/* Flickering effect */}
                <animate attributeName="opacity" values="0.9;0.5;0.9;0.7;0.9" dur="0.3s" repeatCount="indefinite" begin={`${i * 0.1}s`} />
              </rect>
            </g>
          ))}

          {/* Scanline effect */}
          <rect
            x={-baseSize * 1.5}
            y={-1}
            width={baseSize * 3}
            height={2}
            fill={colors.glow}
            opacity={0.7}
          >
            <animateMotion dur={`${baseDuration * 1.2}s`} repeatCount="indefinite" path={edgePath} rotate="auto" />
          </rect>
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

  // Visibility culling
  const { isVisible, registerEdge, unregisterEdge } = useEdgeVisibility();
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

  const { tier: lodTier, shouldRender: shouldRenderEffects, intensityMultiplier } = useEdgeLOD({
    distanceFromCenter,
    zoom,
    isSelected: !!isSelected,
    isHighlighted: isHovered,
    isConnectedToSelected: connectsToHoveredNode,
  });

  // Animation budget
  const priority = isSelected ? 'selected' : isHovered ? 'highlighted' : connectsToHoveredNode ? 'connected' : 'default';
  const { canAnimate } = useAnimationBudget({
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

  // Simplified: always animate if not dimmed (LOD and budget optimizations can be re-enabled later)
  // Original: const shouldAnimate = isEdgeVisible && canAnimate && isAnimated && !effectiveDimmed;
  const shouldAnimate = isAnimated && !effectiveDimmed;

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
