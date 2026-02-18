'use client';

/**
 * StructuralNode - Premium structural node design (v0.13.1)
 *
 * Enhanced with:
 * 1. Hierarchy depth indication via visual markers
 * 2. Parent-child connection indicators
 * 3. Depth-based visual progression
 * 4. Smooth expand/collapse animations
 * 5. Premium hover states with layered effects
 *
 * Categories: project, content, locale, generation
 *
 * Handles ORG realm nodes across 6 layers:
 * - foundation: Project, Brand, BrandDesign, PromptStyle
 * - structure: Page, Block, ContentSlot
 * - semantic: Entity, EntityNative
 * - instruction: BlockInstruction, BlockType, BlockRules, PromptArtifact
 * - output: PageNative, BlockNative, OutputArtifact
 */

import { memo, useMemo, useState, useCallback } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { motion, AnimatePresence, type Variants } from 'motion/react';
import { getStructuralColors } from '@/design/nodeColors';
import { cn } from '@/lib/utils';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, getCardContentComponent, SPRING_CONFIGS, DURATIONS } from './card';

export type StructuralNodeType = Node<StructuralNodeData>;

/**
 * Extended node data with hierarchy information
 */
export interface StructuralNodeData extends BaseNodeData {
  /** Hierarchy depth (0 = root, 1 = child, 2 = grandchild, etc.) */
  depth?: number;
  /** Parent node key for visual connection */
  parentKey?: string;
  /** Number of child nodes */
  childCount?: number;
  /** Whether children are currently expanded */
  isExpanded?: boolean;
  /** Whether this node has children */
  hasChildren?: boolean;
  /** Arc relationship type to parent */
  parentArcType?: string;
}

/**
 * Get card width based on node type
 */
function getCardWidth(type: string): number {
  switch (type) {
    // Project: v0.13.1 "Mission Control" premium card with holographic/matrix effects
    case 'Project': return 440;
    case 'Page': return 320;
    // Semantic layer: Entity and EntityNative with "joined" visual design
    case 'Entity': return 360;
    case 'EntityNative': return 340;
    case 'Block': return 280;
    case 'BlockNative': return 300;
    case 'PageNative': return 320;
    case 'BlockType': return 200;
    case 'BlockInstruction': return 180;
    case 'Locale': return 220;
    case 'Brand': return 220;
    case 'BrandDesign': return 200;
    case 'BrandPrinciples': return 200;
    case 'PromptStyle': return 200;
    case 'ProjectNative': return 220;
    // v0.13.1: shared/config nodes (backup - normally routed to SharedLayerNode)
    case 'EntityCategory': return 420;
    case 'SEOKeywordFormat': return 175;
    default: return 200;
  }
}

/**
 * Get depth-based visual styles
 */
function getDepthStyles(depth: number, primaryColor: string) {
  // Progressive opacity and scale reduction for deeper nodes
  const depthOpacity = Math.max(0.7, 1 - depth * 0.08);
  const depthScale = Math.max(0.92, 1 - depth * 0.02);

  // Depth indicator width progression
  const indicatorWidth = depth > 0 ? Math.max(2, 4 - depth * 0.5) : 0;

  return {
    opacity: depthOpacity,
    scale: depthScale,
    indicatorWidth,
    // Softer glow for deeper nodes
    glowIntensity: Math.max(0.3, 0.8 - depth * 0.15),
    // Border thickness decreases with depth
    borderWidth: Math.max(1, 2 - depth * 0.3),
  };
}

// =============================================================================
// Hierarchy Indicator Component
// =============================================================================

interface HierarchyIndicatorProps {
  depth: number;
  color: string;
  isHovered: boolean;
  selected: boolean;
  hasParent: boolean;
}

const HierarchyIndicator = memo(function HierarchyIndicator({
  depth,
  color,
  isHovered,
  selected,
  hasParent,
}: HierarchyIndicatorProps) {
  if (depth === 0 || !hasParent) return null;

  const variants: Variants = {
    initial: { scaleY: 0, opacity: 0 },
    animate: { scaleY: 1, opacity: 1 },
    hover: { scaleY: 1.1, opacity: 1 },
    selected: { scaleY: 1, opacity: 1 },
  };

  const state = selected ? 'selected' : isHovered ? 'hover' : 'animate';

  return (
    <motion.div
      className="absolute -left-3 top-1/2 flex items-center gap-0.5 pointer-events-none"
      style={{ transform: 'translateY(-50%)' }}
      initial="initial"
      animate={state}
      variants={variants}
      transition={SPRING_CONFIGS.smooth}
    >
      {/* Vertical depth line */}
      <motion.div
        className="h-10 rounded-full"
        style={{
          width: Math.max(2, 4 - depth * 0.5),
          background: `linear-gradient(to bottom, transparent, ${color}40, ${color}80, ${color}40, transparent)`,
          boxShadow: selected ? `0 0 8px ${color}60` : undefined,
        }}
        variants={{
          initial: { opacity: 0, height: 0 },
          animate: { opacity: 0.6, height: 40 },
          hover: { opacity: 0.8, height: 44 },
          selected: { opacity: 1, height: 48 },
        }}
      />

      {/* Connection dot */}
      <motion.div
        className="w-1.5 h-1.5 rounded-full"
        style={{
          backgroundColor: color,
          boxShadow: `0 0 6px ${color}`,
        }}
        variants={{
          initial: { scale: 0 },
          animate: { scale: 1 },
          hover: { scale: 1.3 },
          selected: { scale: 1.5 },
        }}
      />
    </motion.div>
  );
});

// =============================================================================
// Child Count Badge Component
// =============================================================================

interface ChildCountBadgeProps {
  count: number;
  isExpanded: boolean;
  color: string;
  onToggle?: () => void;
}

const ChildCountBadge = memo(function ChildCountBadge({
  count,
  isExpanded,
  color,
  onToggle,
}: ChildCountBadgeProps) {
  if (count === 0) return null;

  return (
    <motion.button
      className={cn(
        'absolute -bottom-2.5 left-1/2 z-10',
        'flex items-center justify-center gap-1',
        'px-2 py-0.5 rounded-full',
        'text-[9px] font-bold uppercase tracking-wider',
        'border backdrop-blur-sm',
        'transition-colors duration-150',
        'cursor-pointer hover:scale-105 active:scale-95'
      )}
      style={{
        transform: 'translateX(-50%)',
        backgroundColor: `${color}20`,
        borderColor: `${color}50`,
        color: color,
        boxShadow: `0 2px 8px ${color}30`,
      }}
      onClick={(e) => {
        e.stopPropagation();
        onToggle?.();
      }}
      whileHover={{
        backgroundColor: `${color}35`,
        borderColor: color,
        boxShadow: `0 4px 12px ${color}50`,
      }}
      whileTap={{ scale: 0.95 }}
    >
      {/* Expand/Collapse chevron */}
      <motion.svg
        width="8"
        height="8"
        viewBox="0 0 8 8"
        fill="none"
        animate={{ rotate: isExpanded ? 180 : 0 }}
        transition={SPRING_CONFIGS.snappy}
      >
        <path
          d="M1 2.5L4 5.5L7 2.5"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </motion.svg>
      <span>{count}</span>
    </motion.button>
  );
});

// =============================================================================
// Depth Glow Effect Component
// =============================================================================

interface DepthGlowProps {
  depth: number;
  color: string;
  isHovered: boolean;
  selected: boolean;
}

const DepthGlow = memo(function DepthGlow({
  depth,
  color,
  isHovered,
  selected,
}: DepthGlowProps) {
  const { glowIntensity } = getDepthStyles(depth, color);

  const variants: Variants = {
    idle: {
      opacity: 0,
      scale: 1,
    },
    hover: {
      opacity: glowIntensity * 0.6,
      scale: 1.02,
      transition: { duration: DURATIONS.fast },
    },
    selected: {
      opacity: glowIntensity,
      scale: 1.03,
      transition: SPRING_CONFIGS.gentle,
    },
  };

  const state = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  return (
    <motion.div
      className="absolute inset-0 rounded-2xl pointer-events-none"
      style={{
        background: `radial-gradient(ellipse at center, ${color}25 0%, transparent 70%)`,
        filter: 'blur(8px)',
      }}
      variants={variants}
      initial="idle"
      animate={state}
    />
  );
});

// =============================================================================
// Premium Hover Overlay Component
// =============================================================================

interface HoverOverlayProps {
  isHovered: boolean;
  selected: boolean;
  color: string;
  width: number;
}

const HoverOverlay = memo(function HoverOverlay({
  isHovered,
  selected,
  color,
  width,
}: HoverOverlayProps) {
  return (
    <AnimatePresence>
      {(isHovered || selected) && (
        <motion.div
          className="absolute inset-0 rounded-2xl pointer-events-none overflow-hidden"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.15 }}
        >
          {/* Gradient sweep effect */}
          <motion.div
            className="absolute inset-0"
            style={{
              background: `linear-gradient(135deg, transparent 30%, ${color}10 50%, transparent 70%)`,
            }}
            initial={{ x: -width }}
            animate={{ x: width }}
            transition={{
              duration: 1.5,
              repeat: selected ? Infinity : 0,
              repeatDelay: 2,
              ease: 'easeInOut',
            }}
          />

          {/* Top highlight */}
          <motion.div
            className="absolute top-0 left-0 right-0 h-px"
            style={{
              background: `linear-gradient(90deg, transparent, ${color}60, transparent)`,
            }}
            animate={{
              opacity: selected ? [0.4, 0.8, 0.4] : 0.6,
            }}
            transition={{
              duration: 2,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Corner accents */}
          {selected && (
            <>
              <motion.div
                className="absolute top-2 left-2 w-3 h-3 rounded-tl-lg border-t-2 border-l-2"
                style={{ borderColor: color }}
                initial={{ opacity: 0, scale: 0.5 }}
                animate={{ opacity: 0.8, scale: 1 }}
                transition={SPRING_CONFIGS.smooth}
              />
              <motion.div
                className="absolute top-2 right-2 w-3 h-3 rounded-tr-lg border-t-2 border-r-2"
                style={{ borderColor: color }}
                initial={{ opacity: 0, scale: 0.5 }}
                animate={{ opacity: 0.8, scale: 1 }}
                transition={{ ...SPRING_CONFIGS.smooth, delay: 0.05 }}
              />
              <motion.div
                className="absolute bottom-2 left-2 w-3 h-3 rounded-bl-lg border-b-2 border-l-2"
                style={{ borderColor: color }}
                initial={{ opacity: 0, scale: 0.5 }}
                animate={{ opacity: 0.8, scale: 1 }}
                transition={{ ...SPRING_CONFIGS.smooth, delay: 0.1 }}
              />
              <motion.div
                className="absolute bottom-2 right-2 w-3 h-3 rounded-br-lg border-b-2 border-r-2"
                style={{ borderColor: color }}
                initial={{ opacity: 0, scale: 0.5 }}
                animate={{ opacity: 0.8, scale: 1 }}
                transition={{ ...SPRING_CONFIGS.smooth, delay: 0.15 }}
              />
            </>
          )}
        </motion.div>
      )}
    </AnimatePresence>
  );
});

// =============================================================================
// Parent Connection Line Component
// =============================================================================

interface ParentConnectionProps {
  hasParent: boolean;
  parentArcType?: string;
  color: string;
  isHovered: boolean;
  selected: boolean;
}

const ParentConnection = memo(function ParentConnection({
  hasParent,
  parentArcType,
  color,
  isHovered,
  selected,
}: ParentConnectionProps) {
  if (!hasParent) return null;

  const arcLabel = parentArcType?.replace(/_/g, ' ').toLowerCase() || 'child of';

  return (
    <motion.div
      className="absolute -top-6 left-1/2 transform -translate-x-1/2 flex flex-col items-center pointer-events-none"
      initial={{ opacity: 0, y: 4 }}
      animate={{
        opacity: isHovered || selected ? 1 : 0.5,
        y: 0,
      }}
      transition={SPRING_CONFIGS.gentle}
    >
      {/* Vertical connector */}
      <motion.div
        className="w-0.5 h-4 rounded-full"
        style={{
          background: `linear-gradient(to bottom, ${color}60, ${color})`,
        }}
        animate={{
          height: isHovered || selected ? 20 : 16,
        }}
      />

      {/* Arc type label (shown on hover/select) */}
      <AnimatePresence>
        {(isHovered || selected) && (
          <motion.div
            className="absolute -top-4 text-[8px] font-medium uppercase tracking-wider whitespace-nowrap"
            style={{ color: `${color}90` }}
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.9 }}
            transition={{ duration: 0.15 }}
          >
            {arcLabel}
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
});

// =============================================================================
// Main StructuralNode Component
// =============================================================================

/**
 * StructuralNode - Premium design with hierarchy visualization
 *
 * Features:
 * - Depth-based visual progression (opacity, scale, glow)
 * - Hierarchy indicators (vertical lines, connection dots)
 * - Child count badge with expand/collapse toggle
 * - Premium hover states with corner accents and sweep effects
 * - Parent connection visualization
 * - Smooth spring animations
 */
export const StructuralNode = memo(function StructuralNode(props: NodeProps<StructuralNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getStructuralColors(data.type), [data.type]);
  const width = getCardWidth(data.type);

  // Local hover state for premium effects
  const [isHovered, setIsHovered] = useState(false);

  // Hierarchy data
  const depth = data.depth ?? 0;
  const childCount = data.childCount ?? 0;
  const hasChildren = data.hasChildren ?? childCount > 0;
  const isExpanded = data.isExpanded ?? false;
  const hasParent = depth > 0 && !!data.parentKey;

  // Get depth-based styles
  const depthStyles = useMemo(
    () => getDepthStyles(depth, colors.primary),
    [depth, colors.primary]
  );

  // Get the specialized card content component for this node type
  const CardContent = useMemo(() => getCardContentComponent(data.type), [data.type]);

  // Prepare data for card content
  // Neo4j transformation puts extra properties in data.data, flatten them for card components
  // Priority order: data.data (raw) < data (computed) < explicit overrides
  const contentData = useMemo(() => ({
    // Raw Neo4j properties (lowest priority)
    ...(data.data || {}),
    // Computed top-level properties (higher priority)
    ...data,
    // Explicit overrides (highest priority) - ensure these are never shadowed
    id: data.id,
    type: data.type,
    key: data.key,
    displayName: data.displayName,
  }), [data]);

  // Handle expand/collapse toggle
  const handleToggleExpand = useCallback(() => {
    // This would typically dispatch an action to the store
    // For now, it's a placeholder for the expand/collapse functionality
    console.log('Toggle expand:', data.key, !isExpanded);
  }, [data.key, isExpanded]);

  // Animation variants for the container
  // CRITICAL: All variants MUST include opacity to prevent disappearing on state change
  const containerVariants: Variants = useMemo(() => ({
    initial: {
      opacity: depthStyles.opacity, // Start visible, not at 0
      scale: depthStyles.scale,
      y: 0,
    },
    animate: {
      opacity: depthStyles.opacity,
      scale: depthStyles.scale,
      y: 0,
      transition: SPRING_CONFIGS.smooth,
    },
    hover: {
      opacity: depthStyles.opacity, // MUST include opacity
      scale: depthStyles.scale, // No scale change to prevent layout shift
      y: -2,
      transition: {
        duration: DURATIONS.fast,
        ease: [0.22, 1, 0.36, 1],
      },
    },
    selected: {
      opacity: depthStyles.opacity, // MUST include opacity
      scale: depthStyles.scale, // No scale change to prevent layout shift
      y: -3,
      transition: SPRING_CONFIGS.snappy,
    },
  }), [depthStyles]);

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'animate';

  return (
    <motion.div
      className="relative"
      variants={containerVariants}
      initial="initial"
      animate={animationState}
      onHoverStart={() => setIsHovered(true)}
      onHoverEnd={() => setIsHovered(false)}
    >
      {/* Parent connection visualization */}
      <ParentConnection
        hasParent={hasParent}
        parentArcType={data.parentArcType}
        color={colors.primary}
        isHovered={isHovered}
        selected={selected}
      />

      {/* Hierarchy depth indicator */}
      <HierarchyIndicator
        depth={depth}
        color={colors.primary}
        isHovered={isHovered}
        selected={selected}
        hasParent={hasParent}
      />

      {/* Depth-based glow effect */}
      <DepthGlow
        depth={depth}
        color={colors.primary}
        isHovered={isHovered}
        selected={selected}
      />

      {/* Main card */}
      <CardShell
        colors={colors}
        selected={selected}
        width={width}
        isDimmed={data.dimmed === true}
        isHoverDimmed={data.hoverDimmed === true}
        isSchemaMode={data.isSchemaMode === true}
        ariaLabel={`${data.type} node: ${data.displayName}`}
        renderContent={(ctx) => (
          <>
            {/* Premium hover overlay */}
            <HoverOverlay
              isHovered={isHovered}
              selected={selected}
              color={colors.primary}
              width={width}
            />

            {/* Card content */}
            <CardContent data={contentData} {...ctx} />
          </>
        )}
      />

      {/* Child count badge with expand/collapse */}
      {hasChildren && (
        <ChildCountBadge
          count={childCount}
          isExpanded={isExpanded}
          color={colors.primary}
          onToggle={handleToggleExpand}
        />
      )}

      {/* Depth level indicator (small badge) */}
      {depth > 0 && (
        <motion.div
          className="absolute -right-1 -top-1 w-4 h-4 rounded-full flex items-center justify-center text-[8px] font-bold"
          style={{
            backgroundColor: `${colors.primary}30`,
            color: colors.primary,
            border: `1px solid ${colors.primary}50`,
          }}
          initial={{ scale: 0, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ delay: 0.2, ...SPRING_CONFIGS.bouncy }}
        >
          {depth}
        </motion.div>
      )}
    </motion.div>
  );
});
