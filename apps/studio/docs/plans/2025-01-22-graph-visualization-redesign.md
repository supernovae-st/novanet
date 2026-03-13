# Graph Visualization Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform the graph visualization into a dynamic, physics-based system with differentiated node designs, organic edges, focus mode, and multiple view modes (ProjectView, ExplorationView).

**Architecture:** View Modes system with configurable layouts (dagre tree, d3-force organic), category-specific node components (InvariantNode, L10nNode, KnowledgeNode, etc.), organic animated edges with SVG filters/glow effects, and focus mode that dims unconnected nodes on selection. Inspired by Nika Studio's GlowingBorder and LabeledEdge patterns.

**Tech Stack:** @xyflow/react, d3-force, Tailwind CSS, SVG filters/animations

---

## Phase 1: Update Types to novanet-core v7.2.0

### Task 1.1: Add Prompts Category to NodeType

**Files:**
- Modify: `src/types/index.ts`
- Modify: `src/config/nodeTypes.ts`

**Step 1: Update NodeType union in types**

```typescript
// In src/types/index.ts - add to NodeType union:
| 'PagePrompt'
| 'BlockPrompt'
```

**Step 2: Run type-check to verify it compiles**

Run: `npm run type-check`
Expected: PASS (types are additive)

**Step 3: Add prompts category and new node configs**

```typescript
// In src/config/nodeTypes.ts - add to NodeTypeConfig interface:
category: 'invariant' | 'l10n' | 'output' | 'prompts' | 'knowledge' | 'seo' | 'geo' | 'mining' | 'metrics';

// Add after OUTPUT section (line ~193):
  // ==========================================================================
  // CATEGORY 4: PROMPTS (🔵) - AI instructions with versioning (v7.2.0)
  // ==========================================================================
  PagePrompt: {
    type: 'PagePrompt',
    label: 'Page Prompt',
    icon: '📝',
    color: '#3b82f6',
    colorClass: 'bg-blue-500',
    size: 14,
    category: 'prompts',
  },
  BlockPrompt: {
    type: 'BlockPrompt',
    label: 'Block Prompt',
    icon: '📝',
    color: '#60a5fa',
    colorClass: 'bg-blue-400',
    size: 12,
    category: 'prompts',
  },
  // v0.19.1: BlockRules removed (merged into BlockType.rules property)
  },
```

**Step 4: Add PROMPTS_TYPES export**

```typescript
// After METRICS_TYPES (line ~406):
/**
 * Prompts types (AI instructions) - v7.2.0
 */
export const PROMPTS_TYPES: NodeType[] = getNodeTypesByCategory('prompts');
```

**Step 5: Update FilterSidebar categories**

```typescript
// In src/components/sidebar/FilterSidebar.tsx - add to CATEGORIES array:
  {
    name: 'Prompts',
    types: PROMPTS_TYPES,
    color: '#3b82f6',
    icon: '🔵',
  },
```

**Step 6: Run type-check and verify**

Run: `npm run type-check`
Expected: PASS

**Step 7: Commit**

```bash
git add src/types/index.ts src/config/nodeTypes.ts src/components/sidebar/FilterSidebar.tsx
git commit -m "feat(types): add novanet-core v7.2.0 types (PagePrompt, BlockPrompt)"
```

---

## Phase 2: View Modes Architecture

### Task 2.1: Create View Modes Configuration

**Files:**
- Create: `src/config/viewModes.ts`
- Modify: `src/stores/uiStore.ts`

**Step 1: Create viewModes.ts with type definitions**

```typescript
// src/config/viewModes.ts
import type { NodeType } from '@/types';

export type ViewModeId = 'project' | 'exploration' | 'locale' | 'focus';

export type LayoutType = 'tree' | 'organic' | 'swimlane' | 'radial';

export interface PhysicsConfig {
  enabled: boolean;
  /** Repulsion strength between nodes */
  chargeStrength: number;
  /** Ideal distance for connected nodes */
  linkDistance: number;
  /** Center gravity strength */
  centerStrength: number;
  /** Collision radius multiplier */
  collisionRadius: number;
  /** Alpha decay rate (higher = faster stabilization) */
  alphaDecay: number;
}

export interface NodeStyleConfig {
  /** Scale multiplier */
  scale: number;
  /** Show full label */
  showLabel: boolean;
  /** Glow intensity (0-1) */
  glowIntensity: number;
}

export interface EdgeStyleConfig {
  /** Edge path type */
  pathType: 'bezier' | 'smoothstep' | 'straight';
  /** Curvature for bezier (0-1) */
  curvature: number;
  /** Show animated particles */
  animated: boolean;
  /** Edge width in pixels */
  strokeWidth: number;
}

export interface ViewMode {
  id: ViewModeId;
  name: string;
  description: string;
  icon: string;
  layout: LayoutType;
  direction: 'TB' | 'LR' | 'BT' | 'RL';
  physics: PhysicsConfig;
  defaultEdgeStyle: EdgeStyleConfig;
  /** Filter to specific node types (null = all) */
  nodeTypeFilter: NodeType[] | null;
}

/**
 * View mode configurations
 */
export const VIEW_MODES: Record<ViewModeId, ViewMode> = {
  project: {
    id: 'project',
    name: 'Project View',
    description: 'Hierarchical: Project → Pages → Blocks',
    icon: '🏗️',
    layout: 'tree',
    direction: 'TB',
    physics: {
      enabled: false,
      chargeStrength: 0,
      linkDistance: 150,
      centerStrength: 0,
      collisionRadius: 1.2,
      alphaDecay: 0.05,
    },
    defaultEdgeStyle: {
      pathType: 'smoothstep',
      curvature: 0,
      animated: false,
      strokeWidth: 2,
    },
    nodeTypeFilter: ['Project', 'Page', 'Block', 'BlockType', 'PageOutput', 'BlockOutput', 'PagePrompt', 'BlockPrompt'],
  },
  exploration: {
    id: 'exploration',
    name: 'Exploration',
    description: 'Force-directed organic layout',
    icon: '🔮',
    layout: 'organic',
    direction: 'LR',
    physics: {
      enabled: true,
      chargeStrength: -300,
      linkDistance: 150,
      centerStrength: 0.05,
      collisionRadius: 1.5,
      alphaDecay: 0.02,
    },
    defaultEdgeStyle: {
      pathType: 'bezier',
      curvature: 0.5,
      animated: true,
      strokeWidth: 1.5,
    },
    nodeTypeFilter: null,
  },
  locale: {
    id: 'locale',
    name: 'Locale View',
    description: 'Swimlanes by language',
    icon: '🌍',
    layout: 'swimlane',
    direction: 'TB',
    physics: {
      enabled: false,
      chargeStrength: 0,
      linkDistance: 100,
      centerStrength: 0,
      collisionRadius: 1.2,
      alphaDecay: 0.05,
    },
    defaultEdgeStyle: {
      pathType: 'smoothstep',
      curvature: 0,
      animated: false,
      strokeWidth: 1.5,
    },
    nodeTypeFilter: null,
  },
  focus: {
    id: 'focus',
    name: 'Focus Mode',
    description: 'Radial around selected node',
    icon: '🎯',
    layout: 'radial',
    direction: 'LR',
    physics: {
      enabled: true,
      chargeStrength: -200,
      linkDistance: 200,
      centerStrength: 0.1,
      collisionRadius: 1.3,
      alphaDecay: 0.03,
    },
    defaultEdgeStyle: {
      pathType: 'bezier',
      curvature: 0.6,
      animated: true,
      strokeWidth: 2,
    },
    nodeTypeFilter: null,
  },
};

export const DEFAULT_VIEW_MODE: ViewModeId = 'exploration';
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Add viewMode to uiStore**

```typescript
// In src/stores/uiStore.ts - add to interface and initial state:
import type { ViewModeId } from '@/config/viewModes';

// Add to UIState interface:
  viewModeId: ViewModeId;
  setViewMode: (mode: ViewModeId) => void;

// Add to initial state:
    viewModeId: 'exploration',

// Add action:
    setViewMode: (mode) => {
      set((state) => {
        state.viewModeId = mode;
      });
    },
```

**Step 4: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 5: Commit**

```bash
git add src/config/viewModes.ts src/stores/uiStore.ts
git commit -m "feat(viewModes): add view modes architecture with physics config"
```

---

## Phase 3: Differentiated Node Designs

### Task 3.1: Create Base Node Wrapper Component

**Files:**
- Create: `src/components/graph/nodes/BaseNodeWrapper.tsx`

**Step 1: Create BaseNodeWrapper with category-based styling**

```typescript
// src/components/graph/nodes/BaseNodeWrapper.tsx
'use client';

import { memo, useState } from 'react';
import { Handle, Position, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { GlowingBorder } from '@/components/ui/GlowingBorder';
import type { NodeType } from '@/types';

export interface BaseNodeData extends Record<string, unknown> {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  icon?: string;
  description?: string;
  category?: string;
  /** Connection count for size scaling */
  connectionCount?: number;
  /** Whether this node is dimmed (focus mode) */
  dimmed?: boolean;
}

export interface BaseNodeWrapperProps {
  data: BaseNodeData;
  selected?: boolean;
  /** Primary color for glow */
  color: string;
  /** Secondary color for gradient */
  colorSecondary?: string;
  /** Node size variant */
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  /** Custom shape class */
  shapeClass?: string;
  children: React.ReactNode;
}

const SIZE_CLASSES = {
  xs: 'min-w-[100px] max-w-[140px]',
  sm: 'min-w-[120px] max-w-[180px]',
  md: 'min-w-[160px] max-w-[220px]',
  lg: 'min-w-[200px] max-w-[280px]',
  xl: 'min-w-[240px] max-w-[320px]',
};

export const BaseNodeWrapper = memo(function BaseNodeWrapper({
  data,
  selected = false,
  color,
  colorSecondary,
  size = 'md',
  shapeClass = 'rounded-xl',
  children,
}: BaseNodeWrapperProps) {
  const [isHovered, setIsHovered] = useState(false);
  const isDimmed = data.dimmed === true;

  return (
    <div
      className={cn(
        'relative transition-all duration-300',
        isDimmed && 'opacity-15 scale-90 grayscale pointer-events-none'
      )}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <GlowingBorder
        color={color}
        colorSecondary={colorSecondary || color}
        isSelected={selected}
        isHovered={isHovered && !isDimmed}
        animated={selected || isHovered}
        borderRadius={shapeClass.includes('full') ? 9999 : 14}
        glowIntensity={isDimmed ? 0 : selected ? 0.8 : 0.5}
      >
        <div
          className={cn(
            'relative transition-all duration-300',
            'bg-black/90 backdrop-blur-xl',
            SIZE_CLASSES[size],
            shapeClass,
            selected && 'bg-black/95'
          )}
        >
          {/* Target Handle */}
          <Handle
            type="target"
            position={Position.Top}
            className={cn(
              '!w-3 !h-3 !border-2 !rounded-full transition-all duration-300',
              '!bg-black/90 !border-white/30',
              'hover:!border-white/60 hover:!scale-125',
              selected && '!border-white/50'
            )}
            style={{
              boxShadow: (isHovered || selected) && !isDimmed
                ? `0 0 8px ${color}60`
                : undefined,
            }}
          />

          {/* Content */}
          <div className="px-4 py-3">
            {children}
          </div>

          {/* Source Handle */}
          <Handle
            type="source"
            position={Position.Bottom}
            className={cn(
              '!w-3 !h-3 !border-2 !rounded-full transition-all duration-300',
              '!bg-black/90 !border-white/30',
              'hover:!border-white/60 hover:!scale-125',
              selected && '!border-white/50'
            )}
            style={{
              boxShadow: (isHovered || selected) && !isDimmed
                ? `0 0 8px ${color}60`
                : undefined,
            }}
          />
        </div>
      </GlowingBorder>
    </div>
  );
});
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/graph/nodes/BaseNodeWrapper.tsx
git commit -m "feat(nodes): add BaseNodeWrapper with focus mode support"
```

### Task 3.2: Create InvariantNode (Project, Page, Block)

**Files:**
- Create: `src/components/graph/nodes/InvariantNode.tsx`

**Step 1: Create InvariantNode component**

```typescript
// src/components/graph/nodes/InvariantNode.tsx
'use client';

import { memo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { BaseNodeWrapper, type BaseNodeData } from './BaseNodeWrapper';

export type InvariantNodeType = Node<BaseNodeData>;

/**
 * Get gradient colors for invariant types
 */
function getInvariantColors(type: string): { primary: string; secondary: string } {
  switch (type) {
    case 'Project':
      return { primary: '#8b5cf6', secondary: '#6366f1' }; // Violet → Indigo
    case 'Page':
      return { primary: '#3b82f6', secondary: '#06b6d4' }; // Blue → Cyan
    case 'Block':
      return { primary: '#06b6d4', secondary: '#14b8a6' }; // Cyan → Teal
    case 'BlockType':
      return { primary: '#14b8a6', secondary: '#10b981' }; // Teal → Emerald
    case 'Concept':
      return { primary: '#f59e0b', secondary: '#f97316' }; // Amber → Orange
    case 'Audience':
      return { primary: '#7c3aed', secondary: '#8b5cf6' }; // Violet
    case 'Locale':
      return { primary: '#10b981', secondary: '#22c55e' }; // Emerald → Green
    case 'BrandIdentity':
      return { primary: '#6d28d9', secondary: '#7c3aed' }; // Purple
    default:
      return { primary: '#6366f1', secondary: '#8b5cf6' };
  }
}

/**
 * Get size based on node type importance
 */
function getNodeSize(type: string): 'xs' | 'sm' | 'md' | 'lg' | 'xl' {
  switch (type) {
    case 'Project':
      return 'xl';
    case 'Page':
    case 'Concept':
    case 'Locale':
      return 'lg';
    case 'Block':
    case 'BlockType':
    case 'Audience':
      return 'md';
    default:
      return 'md';
  }
}

/**
 * InvariantNode - Large cards for structural nodes
 */
export const InvariantNode = memo(function InvariantNode(props: NodeProps<InvariantNodeType>) {
  const { data, selected } = props;
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Project;
  const colors = getInvariantColors(data.type);
  const size = getNodeSize(data.type);

  return (
    <BaseNodeWrapper
      data={data}
      selected={selected}
      color={colors.primary}
      colorSecondary={colors.secondary}
      size={size}
    >
      {/* Header: Icon + Type badge */}
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <span
            className={cn(
              'text-2xl transition-all duration-300',
              selected && 'scale-110'
            )}
            style={{
              filter: `drop-shadow(0 0 ${selected ? '12px' : '8px'} ${config.color}80)`,
            }}
          >
            {data.icon || config.icon}
          </span>
          <span
            className="text-[10px] font-bold uppercase tracking-wider"
            style={{ color: config.color }}
          >
            {config.label}
          </span>
        </div>

        {/* Connection count badge */}
        {data.connectionCount && data.connectionCount > 0 && (
          <span
            className="px-1.5 py-0.5 rounded-full text-[9px] font-semibold"
            style={{
              background: `${colors.primary}20`,
              color: colors.primary,
            }}
          >
            {data.connectionCount}
          </span>
        )}
      </div>

      {/* Display Name */}
      <div
        className={cn(
          'text-sm font-semibold text-white/95 truncate',
          selected && 'text-white'
        )}
        title={data.displayName}
      >
        {data.displayName}
      </div>

      {/* Key */}
      {data.key !== data.displayName && (
        <div className="text-[10px] text-white/40 font-mono truncate mt-0.5">
          {data.key}
        </div>
      )}

      {/* Category badge */}
      <div
        className={cn(
          'mt-2 inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full',
          'text-[9px] font-semibold uppercase tracking-wider',
          'border transition-all duration-300'
        )}
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.secondary}15)`,
          borderColor: `${colors.primary}30`,
          color: colors.primary,
        }}
      >
        <span
          className="w-1.5 h-1.5 rounded-full animate-pulse"
          style={{ background: colors.primary }}
        />
        {config.category}
      </div>
    </BaseNodeWrapper>
  );
});
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/graph/nodes/InvariantNode.tsx
git commit -m "feat(nodes): add InvariantNode for structural types"
```

### Task 3.3: Create KnowledgeNode (Concepts, Expressions)

**Files:**
- Create: `src/components/graph/nodes/KnowledgeNode.tsx`

**Step 1: Create KnowledgeNode with circular design**

```typescript
// src/components/graph/nodes/KnowledgeNode.tsx
'use client';

import { memo, useState } from 'react';
import { Handle, Position, type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { GlowingBorder } from '@/components/ui/GlowingBorder';
import type { BaseNodeData } from './BaseNodeWrapper';

export type KnowledgeNodeType = Node<BaseNodeData>;

/**
 * Get colors for knowledge types
 */
function getKnowledgeColors(type: string): { primary: string; secondary: string } {
  switch (type) {
    case 'LocaleIdentity':
      return { primary: '#22c55e', secondary: '#10b981' };
    case 'LocaleVoice':
      return { primary: '#4ade80', secondary: '#22c55e' };
    case 'LocaleCulture':
      return { primary: '#86efac', secondary: '#4ade80' };
    case 'LocaleMarket':
      return { primary: '#6ee7b7', secondary: '#34d399' };
    case 'LocaleLexicon':
      return { primary: '#34d399', secondary: '#10b981' };
    case 'Expression':
      return { primary: '#ec4899', secondary: '#f472b6' };
    default:
      return { primary: '#10b981', secondary: '#06b6d4' };
  }
}

/**
 * Get size based on connection count (importance)
 */
function getCircleSize(connectionCount?: number): number {
  const count = connectionCount || 0;
  if (count > 10) return 80;
  if (count > 5) return 70;
  if (count > 2) return 60;
  return 50;
}

/**
 * KnowledgeNode - Circular nodes for semantic/knowledge types
 */
export const KnowledgeNode = memo(function KnowledgeNode(props: NodeProps<KnowledgeNodeType>) {
  const { data, selected } = props;
  const [isHovered, setIsHovered] = useState(false);
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Expression;
  const colors = getKnowledgeColors(data.type);
  const size = getCircleSize(data.connectionCount);
  const isDimmed = data.dimmed === true;

  return (
    <div
      className={cn(
        'relative transition-all duration-300',
        isDimmed && 'opacity-15 scale-75 grayscale pointer-events-none'
      )}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <GlowingBorder
        color={colors.primary}
        colorSecondary={colors.secondary}
        isSelected={selected}
        isHovered={isHovered && !isDimmed}
        animated={selected}
        borderRadius={9999}
        glowIntensity={isDimmed ? 0 : selected ? 0.7 : 0.4}
      >
        <div
          className={cn(
            'relative flex flex-col items-center justify-center',
            'bg-black/90 backdrop-blur-xl rounded-full',
            'transition-all duration-300',
            selected && 'bg-black/95'
          )}
          style={{
            width: size,
            height: size,
          }}
        >
          {/* Handle Top */}
          <Handle
            type="target"
            position={Position.Top}
            className="!w-2 !h-2 !bg-black/90 !border !border-white/30 !rounded-full"
          />

          {/* Icon */}
          <span
            className={cn(
              'text-xl transition-all duration-300',
              (isHovered || selected) && 'scale-110'
            )}
            style={{
              filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${config.color}80)`,
            }}
          >
            {data.icon || config.icon}
          </span>

          {/* Label (truncated) */}
          <span
            className="text-[9px] text-white/80 font-medium text-center truncate max-w-[90%] mt-0.5"
            title={data.displayName}
          >
            {data.displayName.length > 12
              ? data.displayName.slice(0, 10) + '...'
              : data.displayName}
          </span>

          {/* Handle Bottom */}
          <Handle
            type="source"
            position={Position.Bottom}
            className="!w-2 !h-2 !bg-black/90 !border !border-white/30 !rounded-full"
          />
        </div>
      </GlowingBorder>

      {/* Orbiting particle effect for selected */}
      {selected && !isDimmed && (
        <div
          className="absolute inset-0 pointer-events-none animate-spin-slow"
          style={{ animationDuration: '8s' }}
        >
          <div
            className="absolute w-2 h-2 rounded-full"
            style={{
              background: colors.primary,
              boxShadow: `0 0 8px ${colors.primary}`,
              top: -4,
              left: '50%',
              transform: 'translateX(-50%)',
            }}
          />
        </div>
      )}
    </div>
  );
});
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/graph/nodes/KnowledgeNode.tsx
git commit -m "feat(nodes): add KnowledgeNode with circular design"
```

### Task 3.4: Create nodes index and register types

**Files:**
- Create: `src/components/graph/nodes/index.ts`
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Create nodes index**

```typescript
// src/components/graph/nodes/index.ts
export { BaseNodeWrapper, type BaseNodeData } from './BaseNodeWrapper';
export { InvariantNode, type InvariantNodeType } from './InvariantNode';
export { KnowledgeNode, type KnowledgeNodeType } from './KnowledgeNode';

// Re-export TurboNode for backwards compatibility
export { TurboNode, TurboNodeCompact, type TurboNodeData, type TurboNodeType } from '../TurboNode';
```

**Step 2: Update Graph2D.tsx to use new node types**

```typescript
// In src/components/graph/Graph2D.tsx - update nodeTypes:
import { TurboNode, TurboNodeCompact, InvariantNode, KnowledgeNode } from './nodes';

const nodeTypes = {
  turbo: TurboNode,
  turboCompact: TurboNodeCompact,
  invariant: InvariantNode,
  knowledge: KnowledgeNode,
} as const;

// Update toTurboNode function to select node type based on category:
function toTurboNode(node: GraphNodeType, compact: boolean): TurboNodeType {
  const config = NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project;

  // Select node type based on category
  let nodeType: string = compact ? 'turboCompact' : 'turbo';
  if (!compact) {
    switch (config.category) {
      case 'invariant':
        nodeType = 'invariant';
        break;
      case 'knowledge':
        nodeType = 'knowledge';
        break;
      // Default to turbo for other categories
    }
  }

  return {
    id: node.id,
    type: nodeType,
    position: { x: 0, y: 0 },
    data: {
      id: node.id,
      type: node.type,
      key: node.key,
      displayName: node.displayName,
      icon: node.icon,
      description: node.description,
      category: config.category,
    },
  };
}
```

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/components/graph/nodes/index.ts src/components/graph/Graph2D.tsx
git commit -m "feat(nodes): register differentiated node types"
```

---

## Phase 4: Organic Edges

### Task 4.1: Create OrganicEdge Component

**Files:**
- Create: `src/components/graph/edges/OrganicEdge.tsx`

**Step 1: Create OrganicEdge with glow and animation**

```typescript
// src/components/graph/edges/OrganicEdge.tsx
'use client';

import { memo, useMemo } from 'react';
import {
  BaseEdge,
  getBezierPath,
  getSmoothStepPath,
  type Edge,
  type EdgeProps,
} from '@xyflow/react';
import { cn } from '@/lib/utils';

export interface OrganicEdgeData extends Record<string, unknown> {
  relationType: string;
  animated?: boolean;
  dimmed?: boolean;
  selected?: boolean;
}

export type OrganicEdgeType = Edge<OrganicEdgeData>;

/**
 * Get edge colors based on relation type
 */
function getEdgeColors(relationType: string): { primary: string; secondary: string } {
  // Structural
  if (relationType.includes('HAS_') || relationType.includes('CONTAINS')) {
    return { primary: '#3b82f6', secondary: '#06b6d4' };
  }
  // Localization
  if (relationType.includes('FOR_LOCALE') || relationType.includes('SUPPORTS')) {
    return { primary: '#10b981', secondary: '#22c55e' };
  }
  // Output
  if (relationType.includes('OUTPUT') || relationType.includes('GENERATED')) {
    return { primary: '#f97316', secondary: '#ef4444' };
  }
  // Semantic
  if (relationType.includes('USES_CONCEPT') || relationType.includes('SEMANTIC')) {
    return { primary: '#ec4899', secondary: '#a855f7' };
  }
  // Prompts (v7.2.0)
  if (relationType.includes('PROMPT') || relationType.includes('RULES')) {
    return { primary: '#3b82f6', secondary: '#60a5fa' };
  }
  // Default
  return { primary: '#6366f1', secondary: '#8b5cf6' };
}

/**
 * OrganicEdge - Bezier edge with glow effects
 */
export const OrganicEdge = memo(function OrganicEdge({
  id,
  data,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  selected,
}: EdgeProps<OrganicEdgeType>) {
  const colors = getEdgeColors(data?.relationType || '');
  const isDimmed = data?.dimmed === true;
  const isAnimated = data?.animated !== false;
  const isSelected = selected || data?.selected;

  const [edgePath] = useMemo(() => {
    return getBezierPath({
      sourceX,
      sourceY,
      sourcePosition,
      targetX,
      targetY,
      targetPosition,
      curvature: 0.5,
    });
  }, [sourceX, sourceY, sourcePosition, targetX, targetY, targetPosition]);

  const strokeWidth = isSelected ? 3 : isDimmed ? 1 : 2;
  const opacity = isDimmed ? 0.1 : 1;

  return (
    <>
      {/* Glow layer */}
      {!isDimmed && (
        <path
          d={edgePath}
          fill="none"
          stroke={colors.primary}
          strokeWidth={strokeWidth + 6}
          strokeOpacity={isSelected ? 0.4 : 0.15}
          style={{
            filter: `blur(${isSelected ? 6 : 4}px)`,
          }}
        />
      )}

      {/* Main edge */}
      <BaseEdge
        id={id}
        path={edgePath}
        style={{
          stroke: `url(#edge-gradient-${id})`,
          strokeWidth,
          opacity,
          strokeLinecap: 'round',
        }}
      />

      {/* Animated particle */}
      {isAnimated && !isDimmed && (
        <circle r={isSelected ? 4 : 3} fill={colors.primary}>
          <animateMotion
            dur={isSelected ? '1.5s' : '3s'}
            repeatCount="indefinite"
            path={edgePath}
          />
        </circle>
      )}

      {/* Gradient definition */}
      <defs>
        <linearGradient id={`edge-gradient-${id}`} x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stopColor={colors.primary} />
          <stop offset="100%" stopColor={colors.secondary} />
        </linearGradient>
      </defs>
    </>
  );
});
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/graph/edges/OrganicEdge.tsx
git commit -m "feat(edges): add OrganicEdge with glow and animation"
```

### Task 4.2: Create edges index and register

**Files:**
- Create: `src/components/graph/edges/index.ts`
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Create edges index**

```typescript
// src/components/graph/edges/index.ts
export { OrganicEdge, type OrganicEdgeData, type OrganicEdgeType } from './OrganicEdge';

// Re-export TurboEdge for backwards compatibility
export { TurboEdge, type TurboEdgeData, type TurboEdgeType } from '../TurboEdge';
```

**Step 2: Update Graph2D.tsx to use OrganicEdge**

```typescript
// In src/components/graph/Graph2D.tsx:
import { OrganicEdge } from './edges';

const edgeTypes = {
  turbo: TurboEdge,
  organic: OrganicEdge,
} as const;

// Update toTurboEdge to use organic:
function toTurboEdge(edge: GraphEdgeType): OrganicEdgeType {
  return {
    id: edge.id,
    source: edge.source,
    target: edge.target,
    type: 'organic',
    data: {
      relationType: edge.type,
      animated: true,
    },
  };
}
```

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/components/graph/edges/index.ts src/components/graph/Graph2D.tsx
git commit -m "feat(edges): register OrganicEdge as default edge type"
```

---

## Phase 5: Focus Mode

### Task 5.1: Create useFocusMode Hook

**Files:**
- Create: `src/hooks/useFocusMode.ts`

**Step 1: Create useFocusMode hook**

```typescript
// src/hooks/useFocusMode.ts
import { useMemo } from 'react';
import { useGraphStore } from '@/stores/graphStore';
import { useUIStore } from '@/stores/uiStore';

export interface FocusModeState {
  /** Currently selected node ID */
  selectedId: string | null;
  /** Set of node IDs that are directly connected (1-hop) */
  connectedIds: Set<string>;
  /** Set of node IDs that are 2-hops away */
  secondHopIds: Set<string>;
  /** Check if a node should be dimmed */
  isNodeDimmed: (nodeId: string) => boolean;
  /** Check if an edge should be dimmed */
  isEdgeDimmed: (sourceId: string, targetId: string) => boolean;
  /** Get opacity level for a node */
  getNodeOpacity: (nodeId: string) => number;
}

/**
 * Hook for focus mode state
 * When a node is selected, calculates which nodes/edges should be dimmed
 */
export function useFocusMode(): FocusModeState {
  const selectedNodeId = useUIStore((state) => state.selectedNodeId);
  const adjacencyMap = useGraphStore((state) => state.adjacencyMap);
  const edgesBySource = useGraphStore((state) => state.edgesBySource);
  const edgesByTarget = useGraphStore((state) => state.edgesByTarget);

  const { connectedIds, secondHopIds } = useMemo(() => {
    if (!selectedNodeId) {
      return { connectedIds: new Set<string>(), secondHopIds: new Set<string>() };
    }

    // Get 1-hop connections
    const firstHop = adjacencyMap.get(selectedNodeId) || new Set<string>();
    const connected = new Set(firstHop);

    // Get 2-hop connections
    const secondHop = new Set<string>();
    for (const nodeId of firstHop) {
      const neighbors = adjacencyMap.get(nodeId) || new Set<string>();
      for (const neighbor of neighbors) {
        if (neighbor !== selectedNodeId && !connected.has(neighbor)) {
          secondHop.add(neighbor);
        }
      }
    }

    return { connectedIds: connected, secondHopIds: secondHop };
  }, [selectedNodeId, adjacencyMap]);

  const isNodeDimmed = (nodeId: string): boolean => {
    if (!selectedNodeId) return false;
    if (nodeId === selectedNodeId) return false;
    if (connectedIds.has(nodeId)) return false;
    if (secondHopIds.has(nodeId)) return false;
    return true;
  };

  const isEdgeDimmed = (sourceId: string, targetId: string): boolean => {
    if (!selectedNodeId) return false;
    // Edge is visible if it connects to selected node or between connected nodes
    const involvesSelected = sourceId === selectedNodeId || targetId === selectedNodeId;
    const bothConnected = connectedIds.has(sourceId) && connectedIds.has(targetId);
    return !involvesSelected && !bothConnected;
  };

  const getNodeOpacity = (nodeId: string): number => {
    if (!selectedNodeId) return 1;
    if (nodeId === selectedNodeId) return 1;
    if (connectedIds.has(nodeId)) return 1;
    if (secondHopIds.has(nodeId)) return 0.6;
    return 0.15;
  };

  return {
    selectedId: selectedNodeId,
    connectedIds,
    secondHopIds,
    isNodeDimmed,
    isEdgeDimmed,
    getNodeOpacity,
  };
}
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/hooks/useFocusMode.ts
git commit -m "feat(hooks): add useFocusMode for selection-based dimming"
```

### Task 5.2: Integrate Focus Mode into Graph2D

**Files:**
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Update Graph2D to use focus mode**

```typescript
// In src/components/graph/Graph2D.tsx - add import:
import { useFocusMode } from '@/hooks/useFocusMode';

// Inside Graph2DInner, add:
const { isNodeDimmed, isEdgeDimmed } = useFocusMode();

// Update toTurboNode to include dimmed state:
function toTurboNode(node: GraphNodeType, compact: boolean, dimmed: boolean): TurboNodeType {
  // ... existing code ...
  return {
    // ... existing fields ...
    data: {
      // ... existing data ...
      dimmed,
    },
  };
}

// Update initialNodes memo:
const initialNodes = useMemo(() => {
  const turboNodes = graphNodes.map((n) =>
    toTurboNode(n, compact, isNodeDimmed(n.id))
  );
  // ... rest of layout code
}, [graphNodes, graphEdges, compact, isNodeDimmed]);

// Update toTurboEdge:
function toTurboEdge(edge: GraphEdgeType, dimmed: boolean): OrganicEdgeType {
  return {
    // ... existing fields ...
    data: {
      relationType: edge.type,
      animated: !dimmed,
      dimmed,
    },
  };
}

// Update initialEdges:
const initialEdges = useMemo(() => {
  return graphEdges.map((e) => toTurboEdge(e, isEdgeDimmed(e.source, e.target)));
}, [graphEdges, isEdgeDimmed]);
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/graph/Graph2D.tsx
git commit -m "feat(graph): integrate focus mode dimming"
```

---

## Phase 6: d3-force Physics

### Task 6.1: Install d3-force

**Step 1: Install d3-force package**

Run: `npm install d3-force @types/d3-force`
Expected: Package installed

**Step 2: Commit**

```bash
git add package.json package-lock.json
git commit -m "chore(deps): add d3-force for physics simulation"
```

### Task 6.2: Create useGraphPhysics Hook

**Files:**
- Create: `src/hooks/useGraphPhysics.ts`

**Step 1: Create physics simulation hook**

```typescript
// src/hooks/useGraphPhysics.ts
import { useEffect, useRef, useCallback } from 'react';
import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCenter,
  forceCollide,
  type Simulation,
  type SimulationNodeDatum,
  type SimulationLinkDatum,
} from 'd3-force';
import type { Node, Edge } from '@xyflow/react';
import type { PhysicsConfig } from '@/config/viewModes';

interface PhysicsNode extends SimulationNodeDatum {
  id: string;
  x?: number;
  y?: number;
  fx?: number | null;
  fy?: number | null;
}

interface PhysicsLink extends SimulationLinkDatum<PhysicsNode> {
  source: string | PhysicsNode;
  target: string | PhysicsNode;
}

interface UseGraphPhysicsOptions {
  nodes: Node[];
  edges: Edge[];
  config: PhysicsConfig;
  enabled: boolean;
  onTick: (nodes: Node[]) => void;
}

/**
 * Hook for d3-force physics simulation
 */
export function useGraphPhysics({
  nodes,
  edges,
  config,
  enabled,
  onTick,
}: UseGraphPhysicsOptions) {
  const simulationRef = useRef<Simulation<PhysicsNode, PhysicsLink> | null>(null);

  // Initialize simulation
  useEffect(() => {
    if (!enabled || nodes.length === 0) {
      if (simulationRef.current) {
        simulationRef.current.stop();
        simulationRef.current = null;
      }
      return;
    }

    // Convert nodes to physics nodes
    const physicsNodes: PhysicsNode[] = nodes.map((n) => ({
      id: n.id,
      x: n.position.x,
      y: n.position.y,
    }));

    // Convert edges to physics links
    const physicsLinks: PhysicsLink[] = edges.map((e) => ({
      source: e.source,
      target: e.target,
    }));

    // Create simulation
    const simulation = forceSimulation<PhysicsNode>(physicsNodes)
      .force(
        'link',
        forceLink<PhysicsNode, PhysicsLink>(physicsLinks)
          .id((d) => d.id)
          .distance(config.linkDistance)
          .strength(0.5)
      )
      .force('charge', forceManyBody().strength(config.chargeStrength))
      .force('center', forceCenter(0, 0).strength(config.centerStrength))
      .force(
        'collision',
        forceCollide<PhysicsNode>().radius(50 * config.collisionRadius)
      )
      .alphaDecay(config.alphaDecay)
      .velocityDecay(0.4)
      .on('tick', () => {
        // Update node positions
        const updatedNodes = nodes.map((node, i) => ({
          ...node,
          position: {
            x: physicsNodes[i].x || 0,
            y: physicsNodes[i].y || 0,
          },
        }));
        onTick(updatedNodes);
      });

    simulationRef.current = simulation;

    return () => {
      simulation.stop();
    };
  }, [nodes.length, edges.length, enabled, config]);

  // Reheat simulation
  const reheat = useCallback(() => {
    if (simulationRef.current) {
      simulationRef.current.alpha(0.3).restart();
    }
  }, []);

  // Fix node position (for dragging)
  const fixNode = useCallback((nodeId: string, x: number, y: number) => {
    if (simulationRef.current) {
      const node = simulationRef.current.nodes().find((n) => n.id === nodeId);
      if (node) {
        node.fx = x;
        node.fy = y;
      }
    }
  }, []);

  // Release node (after drag)
  const releaseNode = useCallback((nodeId: string) => {
    if (simulationRef.current) {
      const node = simulationRef.current.nodes().find((n) => n.id === nodeId);
      if (node) {
        node.fx = null;
        node.fy = null;
      }
    }
  }, []);

  return {
    reheat,
    fixNode,
    releaseNode,
  };
}
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/hooks/useGraphPhysics.ts
git commit -m "feat(hooks): add useGraphPhysics for d3-force simulation"
```

---

## Phase 7: Final Integration

### Task 7.1: Add View Mode Selector UI

**Files:**
- Create: `src/components/ui/ViewModeSelector.tsx`
- Modify: `src/app/page.tsx`

**Step 1: Create ViewModeSelector component**

```typescript
// src/components/ui/ViewModeSelector.tsx
'use client';

import { cn } from '@/lib/utils';
import { VIEW_MODES, type ViewModeId } from '@/config/viewModes';
import { useUIStore } from '@/stores/uiStore';

export function ViewModeSelector() {
  const viewModeId = useUIStore((state) => state.viewModeId);
  const setViewMode = useUIStore((state) => state.setViewMode);

  return (
    <div className="flex items-center gap-1 p-1 rounded-xl bg-black/60 backdrop-blur-xl border border-white/10">
      {Object.values(VIEW_MODES).map((mode) => (
        <button
          key={mode.id}
          onClick={() => setViewMode(mode.id)}
          className={cn(
            'flex items-center gap-2 px-3 py-2 rounded-lg text-xs font-medium transition-all',
            viewModeId === mode.id
              ? 'bg-white/10 text-white'
              : 'text-white/50 hover:text-white/70 hover:bg-white/5'
          )}
          title={mode.description}
        >
          <span>{mode.icon}</span>
          <span className="hidden sm:inline">{mode.name}</span>
        </button>
      ))}
    </div>
  );
}
```

**Step 2: Add to page.tsx**

```typescript
// In src/app/page.tsx - add import:
import { ViewModeSelector } from '@/components/ui/ViewModeSelector';

// Add inside the main graph area, as a Panel:
<Panel position="top-left" className="!m-3">
  <ViewModeSelector />
</Panel>
```

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/components/ui/ViewModeSelector.tsx src/app/page.tsx
git commit -m "feat(ui): add ViewModeSelector component"
```

### Task 7.2: Final Type-Check and Build

**Step 1: Run full type-check**

Run: `npm run type-check`
Expected: PASS

**Step 2: Run build**

Run: `npm run build`
Expected: BUILD SUCCESS

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat(graph): complete graph visualization redesign v1.0"
```

---

## Summary

| Phase | Tasks | Key Files |
|-------|-------|-----------|
| 1. Types v7.2.0 | 1 task | `types/index.ts`, `config/nodeTypes.ts` |
| 2. View Modes | 1 task | `config/viewModes.ts`, `stores/uiStore.ts` |
| 3. Node Designs | 4 tasks | `nodes/BaseNodeWrapper.tsx`, `InvariantNode.tsx`, `KnowledgeNode.tsx` |
| 4. Organic Edges | 2 tasks | `edges/OrganicEdge.tsx` |
| 5. Focus Mode | 2 tasks | `hooks/useFocusMode.ts` |
| 6. Physics | 2 tasks | `hooks/useGraphPhysics.ts` |
| 7. Integration | 2 tasks | `ui/ViewModeSelector.tsx` |

**Total:** 14 tasks, ~25-30 commits
