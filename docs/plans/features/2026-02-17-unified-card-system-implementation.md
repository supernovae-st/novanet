# Unified Card System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement a unified card design system for NovaNet Studio's 3-level node architecture (Taxonomy → Schema → Data) with premium visual effects and performance optimization.

**Architecture:** CardShell + CardContent render props pattern with Framer Motion animations. Performance tiers (ULTRA/HIGH/MEDIUM/LOW/MINIMAL) control effect complexity based on visible node count. Visual encoding follows ADR-005: Fill=Layer, Border=Realm, Style=Trait.

**Tech Stack:** React 18, Framer Motion, TailwindCSS, React Flow, TypeScript

---

## Phase 1: Foundation Infrastructure (8 tasks)

### Task 1.1: Create Performance Context

**Files:**
- Create: `apps/studio/src/contexts/PerformanceContext.tsx`
- Create: `apps/studio/src/contexts/index.ts`
- Test: `apps/studio/src/contexts/__tests__/PerformanceContext.test.tsx`

**Step 1: Write the failing test**

```typescript
// apps/studio/src/contexts/__tests__/PerformanceContext.test.tsx
import { render, screen } from '@testing-library/react';
import { PerformanceProvider, usePerformance } from '../PerformanceContext';

const TestComponent = () => {
  const { tier, config } = usePerformance();
  return <div data-testid="tier">{tier}</div>;
};

describe('PerformanceContext', () => {
  it('provides default ULTRA tier when node count is low', () => {
    render(
      <PerformanceProvider nodeCount={10}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('ULTRA');
  });

  it('provides LOW tier when node count exceeds 100', () => {
    render(
      <PerformanceProvider nodeCount={150}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('LOW');
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cd apps/studio && pnpm test src/contexts/__tests__/PerformanceContext.test.tsx`
Expected: FAIL with "Cannot find module '../PerformanceContext'"

**Step 3: Write minimal implementation**

```typescript
// apps/studio/src/contexts/PerformanceContext.tsx
'use client';

import { createContext, useContext, useMemo, type ReactNode } from 'react';

export type PerformanceTier = 'ULTRA' | 'HIGH' | 'MEDIUM' | 'LOW' | 'MINIMAL';

export interface PerformanceConfig {
  tier: PerformanceTier;
  effects: {
    techCorners: boolean;
    scanlines: boolean;
    gridPattern: boolean;
    shimmer: boolean;
    matrixRain: boolean;
    outerGlow: boolean;
    particles: boolean;
    glassmorphism: boolean;
  };
  animation: {
    enabled: boolean;
    duration: 'fast' | 'normal' | 'slow' | 'none';
    spring: boolean;
  };
}

const TIER_THRESHOLDS = {
  ULTRA: 20,
  HIGH: 50,
  MEDIUM: 100,
  LOW: 200,
  MINIMAL: Infinity,
} as const;

const TIER_CONFIGS: Record<PerformanceTier, PerformanceConfig> = {
  ULTRA: {
    tier: 'ULTRA',
    effects: {
      techCorners: true, scanlines: true, gridPattern: true,
      shimmer: true, matrixRain: true, outerGlow: true,
      particles: true, glassmorphism: true,
    },
    animation: { enabled: true, duration: 'normal', spring: true },
  },
  HIGH: {
    tier: 'HIGH',
    effects: {
      techCorners: true, scanlines: true, gridPattern: true,
      shimmer: true, matrixRain: false, outerGlow: true,
      particles: false, glassmorphism: true,
    },
    animation: { enabled: true, duration: 'fast', spring: true },
  },
  MEDIUM: {
    tier: 'MEDIUM',
    effects: {
      techCorners: true, scanlines: false, gridPattern: true,
      shimmer: false, matrixRain: false, outerGlow: true,
      particles: false, glassmorphism: false,
    },
    animation: { enabled: true, duration: 'fast', spring: false },
  },
  LOW: {
    tier: 'LOW',
    effects: {
      techCorners: true, scanlines: false, gridPattern: false,
      shimmer: false, matrixRain: false, outerGlow: false,
      particles: false, glassmorphism: false,
    },
    animation: { enabled: false, duration: 'none', spring: false },
  },
  MINIMAL: {
    tier: 'MINIMAL',
    effects: {
      techCorners: false, scanlines: false, gridPattern: false,
      shimmer: false, matrixRain: false, outerGlow: false,
      particles: false, glassmorphism: false,
    },
    animation: { enabled: false, duration: 'none', spring: false },
  },
};

function getTierFromNodeCount(count: number): PerformanceTier {
  if (count <= TIER_THRESHOLDS.ULTRA) return 'ULTRA';
  if (count <= TIER_THRESHOLDS.HIGH) return 'HIGH';
  if (count <= TIER_THRESHOLDS.MEDIUM) return 'MEDIUM';
  if (count <= TIER_THRESHOLDS.LOW) return 'LOW';
  return 'MINIMAL';
}

interface PerformanceContextValue {
  tier: PerformanceTier;
  config: PerformanceConfig;
  nodeCount: number;
}

const PerformanceContext = createContext<PerformanceContextValue | null>(null);

interface PerformanceProviderProps {
  children: ReactNode;
  nodeCount: number;
  overrideTier?: PerformanceTier;
}

export function PerformanceProvider({
  children,
  nodeCount,
  overrideTier,
}: PerformanceProviderProps) {
  const value = useMemo(() => {
    const tier = overrideTier ?? getTierFromNodeCount(nodeCount);
    return {
      tier,
      config: TIER_CONFIGS[tier],
      nodeCount,
    };
  }, [nodeCount, overrideTier]);

  return (
    <PerformanceContext.Provider value={value}>
      {children}
    </PerformanceContext.Provider>
  );
}

export function usePerformance(): PerformanceContextValue {
  const context = useContext(PerformanceContext);
  if (!context) {
    throw new Error('usePerformance must be used within PerformanceProvider');
  }
  return context;
}

export { TIER_CONFIGS, getTierFromNodeCount };
```

**Step 4: Run test to verify it passes**

Run: `cd apps/studio && pnpm test src/contexts/__tests__/PerformanceContext.test.tsx`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/src/contexts/
git commit -m "feat(studio): add PerformanceContext for card effect optimization"
```

---

### Task 1.2: Create Card Types

**Files:**
- Create: `apps/studio/src/components/graph/nodes/card/types.ts`
- Modify: `apps/studio/src/components/graph/nodes/card/index.ts`

**Step 1: Write the types file**

```typescript
// apps/studio/src/components/graph/nodes/card/types.ts
import type { PerformanceTier, PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Visual Encoding (ADR-005)
// =============================================================================

export type RealmKey = 'shared' | 'org';
export type LayerKey =
  | 'config' | 'locale' | 'geography' | 'knowledge'
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output';
export type TraitKey = 'defined' | 'authored' | 'imported' | 'generated' | 'retrieved';
export type ArcFamilyKey = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';

export interface TraitBorderStyle {
  style: 'solid' | 'dashed' | 'double' | 'dotted';
  width: number;
}

export const TRAIT_BORDERS: Record<TraitKey, TraitBorderStyle> = {
  defined: { style: 'solid', width: 2 },
  authored: { style: 'dashed', width: 2 },
  imported: { style: 'double', width: 3 },
  generated: { style: 'dotted', width: 2 },
  retrieved: { style: 'dotted', width: 1 },
};

// =============================================================================
// Base Node Data
// =============================================================================

export interface BaseNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  dimmed?: boolean;
  hoverDimmed?: boolean;
}

// =============================================================================
// Card Context (passed to CardContent via render props)
// =============================================================================

export interface CardContext {
  colors: {
    primary: string;    // Layer color (fill, accents)
    secondary: string;  // Realm color (border)
    accent?: string;    // Optional tertiary color
  };
  selected: boolean;
  isHovered: boolean;
  width?: number;
  performanceTier: PerformanceTier;
  performanceConfig: PerformanceConfig;
}

// =============================================================================
// Node Level Types (3-Level Architecture)
// =============================================================================

export type NodeLevel = 'taxonomy' | 'schema' | 'data';

// Level 1: Taxonomy (21 nodes)
export type TaxonomyVariant = 'realm' | 'layer' | 'trait' | 'arcFamily';

export interface TaxonomyNodeData extends BaseNodeData {
  level: 'taxonomy';
  variant: TaxonomyVariant;
  count: number; // Number of nodes/arcs in this category
  color: string;
}

// Level 2: Schema (239 nodes)
export type SchemaVariant = 'nodeClass' | 'arcClass';

export interface SchemaNodeData extends BaseNodeData {
  level: 'schema';
  variant: SchemaVariant;
  realm: RealmKey;
  layer: LayerKey;
  trait: TraitKey;
  propCount?: number;
  // For ArcClass
  source?: string;
  target?: string;
  family?: ArcFamilyKey;
  cardinality?: '1:1' | '1:N' | 'N:M';
}

// Level 3: Data (∞ instances)
export interface DataNodeData extends BaseNodeData {
  level: 'data';
  realm: RealmKey;
  layer: LayerKey;
  trait: TraitKey;
  className: string; // e.g., 'Page', 'Entity', 'Block'
  // Layer-specific data passed via layerData
  layerData?: Record<string, unknown>;
}

// =============================================================================
// Union Types
// =============================================================================

export type CardNodeData = TaxonomyNodeData | SchemaNodeData | DataNodeData;

// =============================================================================
// Card Shell Props
// =============================================================================

export interface CardShellProps {
  data: CardNodeData;
  children: (context: CardContext) => React.ReactNode;
  width?: number;
  className?: string;
}

// =============================================================================
// Animation Variants (Framer Motion)
// =============================================================================

export const CARD_ANIMATION_VARIANTS = {
  hidden: { opacity: 0, scale: 0.95, y: 10 },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: {
      type: 'spring',
      stiffness: 500,
      damping: 30,
      mass: 1,
    },
  },
  selected: {
    scale: 1.02,
    transition: { duration: 0.2, ease: [0.175, 0.885, 0.32, 2.275] },
  },
  hover: {
    y: -2,
    transition: { duration: 0.2 },
  },
} as const;
```

**Step 2: Update index.ts**

```typescript
// apps/studio/src/components/graph/nodes/card/index.ts
export * from './types';
export * from './CardShell';
export { ClassCardContent, type ClassNodeData } from './presets/ClassCardContent';
export { StructuralCardContent } from './presets/StructuralCardContent';
export { AttractorCardContent } from './presets/AttractorCardContent';
export { LocaleCardContent } from './presets/LocaleCardContent';
export { ProjectCardContent } from './presets/ProjectCardContent';
export { RealmOrbitalCardContent } from './presets/RealmOrbitalCardContent';
```

**Step 3: Commit**

```bash
git add apps/studio/src/components/graph/nodes/card/types.ts apps/studio/src/components/graph/nodes/card/index.ts
git commit -m "feat(studio): add unified card type system with 3-level architecture"
```

---

### Task 1.3: Enhance CardShell with Performance Awareness

**Files:**
- Modify: `apps/studio/src/components/graph/nodes/card/CardShell.tsx:1-150`
- Test: `apps/studio/src/components/graph/nodes/card/__tests__/CardShell.test.tsx`

**Step 1: Read current CardShell implementation**

Run: `cat apps/studio/src/components/graph/nodes/card/CardShell.tsx`

**Step 2: Write enhanced CardShell**

```typescript
// apps/studio/src/components/graph/nodes/card/CardShell.tsx
'use client';

import { memo, useMemo, useState, useCallback } from 'react';
import { motion, type Variants } from 'framer-motion';
import { cn } from '@/lib/utils';
import { usePerformance } from '@/contexts/PerformanceContext';
import {
  REALM_COLORS,
  LAYER_COLORS,
  type RealmKey,
  type LayerKey,
} from '@/design/colors/generated';
import type { CardShellProps, CardContext, TRAIT_BORDERS, TraitKey } from './types';

// =============================================================================
// Animation Variants
// =============================================================================

const createVariants = (springEnabled: boolean): Variants => ({
  hidden: { opacity: 0, scale: 0.95, y: 10 },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: springEnabled
      ? { type: 'spring', stiffness: 500, damping: 30, mass: 1 }
      : { duration: 0.15 },
  },
  hover: { y: -2, transition: { duration: 0.15 } },
  selected: { scale: 1.02, transition: { duration: 0.15 } },
});

// =============================================================================
// Border Style Helper
// =============================================================================

const getBorderStyle = (trait: TraitKey): React.CSSProperties => {
  const config = TRAIT_BORDERS[trait];
  return {
    borderStyle: config.style,
    borderWidth: config.width,
  };
};

// =============================================================================
// CardShell Component
// =============================================================================

export const CardShell = memo(function CardShell({
  data,
  children,
  width = 240,
  className,
}: CardShellProps) {
  const [isHovered, setIsHovered] = useState(false);
  const [isSelected, setIsSelected] = useState(false);
  const { tier, config: perfConfig } = usePerformance();

  // Extract realm/layer from data
  const realm: RealmKey = 'realm' in data ? (data.realm as RealmKey) : 'shared';
  const layer: LayerKey = 'layer' in data ? (data.layer as LayerKey) : 'config';
  const trait: TraitKey = 'trait' in data ? (data.trait as TraitKey) : 'defined';

  // Compute colors
  const colors = useMemo(() => ({
    primary: LAYER_COLORS[layer]?.color ?? '#64748b',
    secondary: REALM_COLORS[realm]?.color ?? '#2aa198',
  }), [realm, layer]);

  // Animation variants based on performance tier
  const variants = useMemo(
    () => createVariants(perfConfig.animation.spring),
    [perfConfig.animation.spring]
  );

  // Card context for children
  const context: CardContext = useMemo(() => ({
    colors,
    selected: isSelected,
    isHovered,
    width,
    performanceTier: tier,
    performanceConfig: perfConfig,
  }), [colors, isSelected, isHovered, width, tier, perfConfig]);

  // Event handlers
  const handleMouseEnter = useCallback(() => setIsHovered(true), []);
  const handleMouseLeave = useCallback(() => setIsHovered(false), []);
  const handleClick = useCallback(() => setIsSelected(prev => !prev), []);

  // Dimming style
  const opacityStyle = useMemo(() => ({
    opacity: data.dimmed ? 0.06 : data.hoverDimmed ? 0.25 : 1,
  }), [data.dimmed, data.hoverDimmed]);

  // Border style from trait
  const borderStyle = useMemo(() => ({
    ...getBorderStyle(trait),
    borderColor: isSelected ? colors.primary : `${colors.secondary}80`,
  }), [trait, isSelected, colors]);

  // Shadow/glow based on state and performance
  const shadowStyle = useMemo(() => {
    if (!perfConfig.effects.outerGlow) return {};
    if (isSelected) {
      return { boxShadow: `0 0 20px ${colors.primary}40, 0 0 40px ${colors.primary}20` };
    }
    if (isHovered) {
      return { boxShadow: `0 0 12px ${colors.primary}30` };
    }
    return {};
  }, [isSelected, isHovered, colors.primary, perfConfig.effects.outerGlow]);

  return (
    <motion.div
      className={cn(
        'relative rounded-xl overflow-hidden cursor-pointer',
        'transition-colors duration-200',
        className
      )}
      style={{
        width,
        ...opacityStyle,
        ...borderStyle,
        ...shadowStyle,
        background: 'rgba(0, 0, 0, 0.85)',
      }}
      variants={perfConfig.animation.enabled ? variants : undefined}
      initial={perfConfig.animation.enabled ? 'hidden' : undefined}
      animate={perfConfig.animation.enabled ? 'visible' : undefined}
      whileHover={perfConfig.animation.enabled ? 'hover' : undefined}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onClick={handleClick}
      aria-label={`Card: ${data.displayName}`}
      role="button"
      tabIndex={0}
    >
      {children(context)}
    </motion.div>
  );
});

export type { CardContext };
```

**Step 3: Update test**

```typescript
// apps/studio/src/components/graph/nodes/card/__tests__/CardShell.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { CardShell } from '../CardShell';
import { PerformanceProvider } from '@/contexts/PerformanceContext';

const mockData = {
  id: 'test-1',
  type: 'Page',
  key: 'test-page',
  displayName: 'Test Page',
  level: 'data' as const,
  realm: 'org' as const,
  layer: 'structure' as const,
  trait: 'defined' as const,
  className: 'Page',
};

const renderWithProvider = (ui: React.ReactElement, nodeCount = 10) => {
  return render(
    <PerformanceProvider nodeCount={nodeCount}>
      {ui}
    </PerformanceProvider>
  );
};

describe('CardShell', () => {
  it('renders children with context', () => {
    renderWithProvider(
      <CardShell data={mockData}>
        {(ctx) => <div data-testid="content">Tier: {ctx.performanceTier}</div>}
      </CardShell>
    );
    expect(screen.getByTestId('content')).toHaveTextContent('Tier: ULTRA');
  });

  it('provides hover state to children', () => {
    renderWithProvider(
      <CardShell data={mockData}>
        {(ctx) => <div data-testid="hover">{ctx.isHovered ? 'hovered' : 'not'}</div>}
      </CardShell>
    );

    const card = screen.getByRole('button');
    fireEvent.mouseEnter(card);
    expect(screen.getByTestId('hover')).toHaveTextContent('hovered');
  });

  it('applies correct border style for trait', () => {
    const { container } = renderWithProvider(
      <CardShell data={{ ...mockData, trait: 'authored' }}>
        {() => <div>Content</div>}
      </CardShell>
    );

    const card = container.firstChild as HTMLElement;
    expect(card.style.borderStyle).toBe('dashed');
  });
});
```

**Step 4: Run tests**

Run: `cd apps/studio && pnpm test src/components/graph/nodes/card/__tests__/CardShell.test.tsx`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/src/components/graph/nodes/card/
git commit -m "feat(studio): enhance CardShell with performance-aware animations"
```

---

### Task 1.4: Create Framer Motion Animation Presets

**Files:**
- Create: `apps/studio/src/components/graph/nodes/card/animations.ts`

**Step 1: Write animation presets**

```typescript
// apps/studio/src/components/graph/nodes/card/animations.ts
import type { Variants, Transition } from 'framer-motion';

// =============================================================================
// Timing Constants
// =============================================================================

export const TIMING = {
  fast: 0.15,
  normal: 0.3,
  slow: 0.5,
} as const;

export const SPRING_PRESETS = {
  snappy: { type: 'spring', stiffness: 500, damping: 30, mass: 1 },
  bouncy: { type: 'spring', stiffness: 400, damping: 25, mass: 0.8 },
  smooth: { type: 'spring', stiffness: 300, damping: 30, mass: 1.2 },
} as const;

// =============================================================================
// Card Variants
// =============================================================================

export const cardEnterVariants: Variants = {
  hidden: { opacity: 0, scale: 0.95, y: 10 },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: SPRING_PRESETS.snappy,
  },
};

export const cardHoverVariants: Variants = {
  rest: { y: 0, scale: 1 },
  hover: { y: -2, scale: 1.01, transition: { duration: TIMING.fast } },
};

export const cardSelectVariants: Variants = {
  unselected: { scale: 1 },
  selected: {
    scale: 1.02,
    transition: { duration: TIMING.fast, ease: [0.175, 0.885, 0.32, 2.275] },
  },
};

// =============================================================================
// Effect Variants
// =============================================================================

export const pulseVariants: Variants = {
  idle: { opacity: 0.5, scale: 1 },
  active: {
    opacity: [0.5, 1, 0.5],
    scale: [1, 1.1, 1],
    transition: { duration: 2, repeat: Infinity, ease: 'easeInOut' },
  },
};

export const shimmerVariants: Variants = {
  idle: { backgroundPosition: '200% 0' },
  active: {
    backgroundPosition: '-200% 0',
    transition: { duration: 2.5, repeat: Infinity, ease: 'linear' },
  },
};

export const glowVariants: Variants = {
  idle: { boxShadow: '0 0 0 rgba(0,0,0,0)' },
  hover: (color: string) => ({
    boxShadow: `0 0 20px ${color}40`,
    transition: { duration: TIMING.normal },
  }),
  selected: (color: string) => ({
    boxShadow: `0 0 30px ${color}60, 0 0 60px ${color}30`,
    transition: { duration: TIMING.normal },
  }),
};

// =============================================================================
// Taxonomy-Specific Variants
// =============================================================================

export const orbitalRotateVariants: Variants = {
  idle: { rotate: 0 },
  active: {
    rotate: 360,
    transition: { duration: 20, repeat: Infinity, ease: 'linear' },
  },
};

export const stackedPlanesVariants: Variants = {
  rest: { z: 0 },
  hover: { z: -8, transition: { duration: TIMING.normal } },
};

export const borderMorphVariants: Variants = {
  solid: { borderStyle: 'solid' },
  dashed: { borderStyle: 'dashed' },
  double: { borderStyle: 'double' },
  dotted: { borderStyle: 'dotted' },
};

// =============================================================================
// Particle Flow (for ArcClass)
// =============================================================================

export const createParticleVariants = (delay: number): Variants => ({
  hidden: { offsetDistance: '0%', opacity: 0 },
  visible: {
    offsetDistance: '100%',
    opacity: [0, 1, 1, 0],
    transition: {
      duration: 1.5,
      repeat: Infinity,
      ease: 'linear',
      delay,
    },
  },
});

// =============================================================================
// Stagger Children
// =============================================================================

export const staggerContainerVariants: Variants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.05,
      delayChildren: 0.1,
    },
  },
};

export const staggerItemVariants: Variants = {
  hidden: { opacity: 0, y: 10 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: TIMING.fast },
  },
};
```

**Step 2: Commit**

```bash
git add apps/studio/src/components/graph/nodes/card/animations.ts
git commit -m "feat(studio): add Framer Motion animation presets for cards"
```

---

### Task 1.5: Create Enhanced TechCorners Effect

**Files:**
- Modify: `apps/studio/src/components/graph/nodes/effects/PremiumSchemaEffects.tsx:79-140`

**Step 1: Enhance TechCorners with Framer Motion glow**

```typescript
// Add to apps/studio/src/components/graph/nodes/effects/PremiumSchemaEffects.tsx
// Replace the existing TechCorners component

import { motion } from 'framer-motion';

export const TechCorners = memo(function TechCorners({
  color,
  selected,
  size = 16,
  animated = true,
}: {
  color: string;
  selected: boolean;
  size?: number;
  animated?: boolean;
}) {
  const positions = [
    { key: 'tl', top: 8, left: 8, transform: 'rotate(0deg)' },
    { key: 'tr', top: 8, right: 8, transform: 'scaleX(-1)' },
    { key: 'bl', bottom: 8, left: 8, transform: 'scaleY(-1)' },
    { key: 'br', bottom: 8, right: 8, transform: 'scale(-1)' },
  ];

  const cornerPath = `M0 ${size}L0 0L${size} 0`;

  return (
    <>
      {positions.map(({ key, transform, ...pos }) => (
        <motion.div
          key={key}
          className="absolute pointer-events-none z-20"
          style={{ ...pos, color }}
          initial={{ opacity: 0.3 }}
          animate={{
            opacity: selected ? 0.9 : 0.5,
            filter: selected
              ? `drop-shadow(0 0 6px ${color}) drop-shadow(0 0 12px ${color})`
              : 'none',
          }}
          transition={{ duration: 0.3 }}
        >
          <svg
            width={size}
            height={size}
            viewBox={`0 0 ${size} ${size}`}
            fill="none"
            style={{ transform }}
          >
            <motion.path
              d={cornerPath}
              stroke="currentColor"
              strokeWidth="1.5"
              initial={{ pathLength: 0 }}
              animate={{ pathLength: 1 }}
              transition={animated ? { duration: 0.5, delay: 0.2 } : undefined}
            />
            <motion.circle
              cx="0"
              cy="0"
              r="2"
              fill="currentColor"
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={animated ? { duration: 0.3, delay: 0.5 } : undefined}
            />
          </svg>
        </motion.div>
      ))}
    </>
  );
});
```

**Step 2: Commit**

```bash
git add apps/studio/src/components/graph/nodes/effects/PremiumSchemaEffects.tsx
git commit -m "feat(studio): enhance TechCorners with Framer Motion animations"
```

---

### Task 1.6: Create NeonBorderGlow Effect

**Files:**
- Create: `apps/studio/src/components/graph/nodes/effects/NeonBorderGlow.tsx`
- Modify: `apps/studio/src/components/graph/nodes/effects/index.ts`

**Step 1: Write NeonBorderGlow component**

```typescript
// apps/studio/src/components/graph/nodes/effects/NeonBorderGlow.tsx
'use client';

import { memo, useMemo } from 'react';
import { motion } from 'framer-motion';

export interface NeonBorderGlowProps {
  color: string;
  selected: boolean;
  isHovered: boolean;
  borderRadius?: number;
  intensity?: 'low' | 'medium' | 'high';
}

export const NeonBorderGlow = memo(function NeonBorderGlow({
  color,
  selected,
  isHovered,
  borderRadius = 12,
  intensity = 'medium',
}: NeonBorderGlowProps) {
  const glowIntensity = {
    low: { spread: 10, blur: 20 },
    medium: { spread: 15, blur: 30 },
    high: { spread: 20, blur: 40 },
  }[intensity];

  const boxShadow = useMemo(() => {
    if (selected) {
      return `
        0 0 ${glowIntensity.spread}px ${color}60,
        0 0 ${glowIntensity.blur}px ${color}40,
        inset 0 0 ${glowIntensity.spread}px ${color}20
      `;
    }
    if (isHovered) {
      return `0 0 ${glowIntensity.spread}px ${color}40`;
    }
    return `0 0 5px ${color}20`;
  }, [selected, isHovered, color, glowIntensity]);

  return (
    <motion.div
      className="absolute -inset-0.5 rounded-xl pointer-events-none z-0"
      style={{ borderRadius: borderRadius + 2 }}
      animate={{ boxShadow }}
      transition={{ duration: 0.3, ease: 'easeOut' }}
    />
  );
});
```

**Step 2: Update effects index**

```typescript
// apps/studio/src/components/graph/nodes/effects/index.ts
export { SelectionPulseRing, type SelectionPulseRingProps } from './SelectionPulseRing';
export { GlassmorphismEffects, type GlassmorphismEffectsProps } from './GlassmorphismEffects';
export { SmartHandle, NodeHandles, type SmartHandleProps, type NodeHandlesProps } from './SmartHandle';
export { GlowBadge, type GlowBadgeProps } from './GlowBadge';
export { NeonBorderGlow, type NeonBorderGlowProps } from './NeonBorderGlow';
export {
  PremiumSchemaEffects,
  TechCorners,
  Scanlines,
  GridPattern,
  HolographicShimmer,
  MatrixRain,
  OuterGlow,
  PremiumSchemaKeyframes,
  type PremiumSchemaEffectsProps,
} from './PremiumSchemaEffects';
```

**Step 3: Commit**

```bash
git add apps/studio/src/components/graph/nodes/effects/
git commit -m "feat(studio): add NeonBorderGlow effect component"
```

---

### Task 1.7: Create FlowingParticles Effect (for ArcClass)

**Files:**
- Create: `apps/studio/src/components/graph/nodes/effects/FlowingParticles.tsx`
- Modify: `apps/studio/src/components/graph/nodes/effects/index.ts`

**Step 1: Write FlowingParticles component**

```typescript
// apps/studio/src/components/graph/nodes/effects/FlowingParticles.tsx
'use client';

import { memo, useMemo } from 'react';
import { motion } from 'framer-motion';

export interface FlowingParticlesProps {
  color: string;
  active: boolean;
  particleCount?: number;
  duration?: number;
  pathId?: string;
}

export const FlowingParticles = memo(function FlowingParticles({
  color,
  active,
  particleCount = 3,
  duration = 1.5,
  pathId = 'arcFlowPath',
}: FlowingParticlesProps) {
  const particles = useMemo(() =>
    Array.from({ length: particleCount }, (_, i) => ({
      id: i,
      delay: (i * duration) / particleCount,
    })),
    [particleCount, duration]
  );

  if (!active) return null;

  return (
    <g>
      {particles.map(({ id, delay }) => (
        <motion.circle
          key={id}
          r="3"
          fill={color}
          style={{
            offsetPath: `url(#${pathId})`,
            filter: `drop-shadow(0 0 4px ${color})`,
          }}
          initial={{ offsetDistance: '0%', opacity: 0 }}
          animate={{
            offsetDistance: '100%',
            opacity: [0, 1, 1, 0],
          }}
          transition={{
            duration,
            repeat: Infinity,
            ease: 'linear',
            delay,
          }}
        />
      ))}
    </g>
  );
});

// Arc path with flowing particles visualization
export interface ArcFlowVisualizationProps {
  sourceLabel: string;
  targetLabel: string;
  color: string;
  isHovered: boolean;
  cardinality: '1:1' | '1:N' | 'N:M';
}

export const ArcFlowVisualization = memo(function ArcFlowVisualization({
  sourceLabel,
  targetLabel,
  color,
  isHovered,
  cardinality,
}: ArcFlowVisualizationProps) {
  const particleCount = cardinality === 'N:M' ? 4 : cardinality === '1:N' ? 3 : 2;
  const pathId = `arcPath-${sourceLabel}-${targetLabel}`;

  return (
    <div className="relative flex items-center justify-between gap-2 py-3 px-2">
      {/* Source Node */}
      <motion.div
        className="flex items-center justify-center px-2.5 py-1 rounded-lg border text-[10px] font-mono"
        style={{
          borderColor: `${color}50`,
          backgroundColor: `${color}10`,
          color: `${color}`,
        }}
        whileHover={{ scale: 1.05, borderColor: color }}
      >
        {sourceLabel}
      </motion.div>

      {/* Connection Path */}
      <div className="relative flex-1 h-6 mx-1">
        <svg className="absolute inset-0 w-full h-full overflow-visible">
          <defs>
            <path
              id={pathId}
              d="M 0,12 C 30,12 50,12 80,12"
              fill="none"
            />
          </defs>

          {/* Base path */}
          <use
            href={`#${pathId}`}
            stroke={`${color}40`}
            strokeWidth="2"
            strokeDasharray={cardinality === '1:1' ? 'none' : '4,4'}
            fill="none"
          />

          {/* Flowing particles */}
          <FlowingParticles
            color={color}
            active={isHovered}
            particleCount={particleCount}
            pathId={pathId}
          />
        </svg>

        {/* Cardinality badge */}
        <div
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     text-[8px] font-mono px-1.5 py-0.5 rounded-full"
          style={{ backgroundColor: `${color}20`, color }}
        >
          {cardinality}
        </div>
      </div>

      {/* Target Node */}
      <motion.div
        className="flex items-center justify-center px-2.5 py-1 rounded-lg border text-[10px] font-mono"
        style={{
          borderColor: `${color}50`,
          backgroundColor: `${color}10`,
          color: `${color}`,
        }}
        whileHover={{ scale: 1.05, borderColor: color }}
      >
        {targetLabel}
      </motion.div>
    </div>
  );
});
```

**Step 2: Update effects index**

Add to `apps/studio/src/components/graph/nodes/effects/index.ts`:
```typescript
export { FlowingParticles, ArcFlowVisualization, type FlowingParticlesProps, type ArcFlowVisualizationProps } from './FlowingParticles';
```

**Step 3: Commit**

```bash
git add apps/studio/src/components/graph/nodes/effects/
git commit -m "feat(studio): add FlowingParticles effect for ArcClass cards"
```

---

### Task 1.8: Create Taxonomy-Specific Effects (OrbitalRings, StackedPlanes, BorderMorph)

**Files:**
- Create: `apps/studio/src/components/graph/nodes/effects/TaxonomyEffects.tsx`
- Modify: `apps/studio/src/components/graph/nodes/effects/index.ts`

**Step 1: Write TaxonomyEffects**

```typescript
// apps/studio/src/components/graph/nodes/effects/TaxonomyEffects.tsx
'use client';

import { memo, useState, useEffect } from 'react';
import { motion, useMotionValue, useTransform } from 'framer-motion';

// =============================================================================
// OrbitalRings - For Realm nodes
// =============================================================================

export interface OrbitalRingsProps {
  color: string;
  isHovered: boolean;
  selected: boolean;
}

export const OrbitalRings = memo(function OrbitalRings({
  color,
  isHovered,
  selected,
}: OrbitalRingsProps) {
  const rings = [
    { rx: 30, ry: 12, duration: 8, opacity: 0.4 },
    { rx: 45, ry: 18, duration: 12, opacity: 0.3 },
    { rx: 60, ry: 24, duration: 16, opacity: 0.2 },
  ];

  return (
    <svg className="absolute inset-0 w-full h-full overflow-visible pointer-events-none">
      <g transform="translate(50%, 50%)">
        {rings.map((ring, i) => (
          <motion.ellipse
            key={i}
            cx="0"
            cy="0"
            rx={ring.rx}
            ry={ring.ry}
            fill="none"
            stroke={color}
            strokeWidth={selected ? 1.5 : 1}
            strokeOpacity={ring.opacity}
            initial={{ rotate: i * 30 }}
            animate={{
              rotate: [i * 30, i * 30 + 360],
              scale: isHovered ? 1.1 : 1,
            }}
            transition={{
              rotate: { duration: ring.duration, repeat: Infinity, ease: 'linear' },
              scale: { duration: 0.3 },
            }}
            style={{ transformOrigin: 'center' }}
          />
        ))}
        {/* Central dot */}
        <motion.circle
          cx="0"
          cy="0"
          r={selected ? 6 : 4}
          fill={color}
          animate={{
            scale: selected ? [1, 1.2, 1] : 1,
            opacity: selected ? 1 : 0.7,
          }}
          transition={{ duration: 1, repeat: selected ? Infinity : 0 }}
        />
      </g>
    </svg>
  );
});

// =============================================================================
// StackedPlanes - For Layer nodes
// =============================================================================

export interface StackedPlanesProps {
  color: string;
  isHovered: boolean;
  planeCount?: number;
}

export const StackedPlanes = memo(function StackedPlanes({
  color,
  isHovered,
  planeCount = 3,
}: StackedPlanesProps) {
  const mouseX = useMotionValue(0);
  const mouseY = useMotionValue(0);

  const rotateX = useTransform(mouseY, [-50, 50], [5, -5]);
  const rotateY = useTransform(mouseX, [-50, 50], [-5, 5]);

  const planes = Array.from({ length: planeCount }, (_, i) => ({
    z: -i * 8,
    opacity: 0.3 - i * 0.08,
    scale: 1 - i * 0.05,
  }));

  return (
    <motion.div
      className="absolute inset-4 perspective-500"
      style={{ perspective: 500 }}
      onMouseMove={(e) => {
        const rect = e.currentTarget.getBoundingClientRect();
        mouseX.set(e.clientX - rect.left - rect.width / 2);
        mouseY.set(e.clientY - rect.top - rect.height / 2);
      }}
      onMouseLeave={() => {
        mouseX.set(0);
        mouseY.set(0);
      }}
    >
      {planes.map((plane, i) => (
        <motion.div
          key={i}
          className="absolute inset-0 rounded-lg border"
          style={{
            borderColor: color,
            opacity: plane.opacity,
            transformStyle: 'preserve-3d',
          }}
          animate={{
            z: isHovered ? plane.z - 4 : plane.z,
            scale: isHovered ? plane.scale * 1.02 : plane.scale,
            rotateX: isHovered ? rotateX.get() : 0,
            rotateY: isHovered ? rotateY.get() : 0,
          }}
          transition={{ duration: 0.3, ease: 'easeOut' }}
        />
      ))}
    </motion.div>
  );
});

// =============================================================================
// BorderMorph - For Trait nodes
// =============================================================================

export interface BorderMorphProps {
  color: string;
  animated?: boolean;
  currentStyle?: 'solid' | 'dashed' | 'double' | 'dotted';
}

export const BorderMorph = memo(function BorderMorph({
  color,
  animated = true,
  currentStyle,
}: BorderMorphProps) {
  const styles: Array<'solid' | 'dashed' | 'double' | 'dotted'> = [
    'solid', 'dashed', 'double', 'dotted', 'dotted',
  ];
  const [index, setIndex] = useState(
    currentStyle ? styles.indexOf(currentStyle) : 0
  );

  useEffect(() => {
    if (!animated) return;
    const interval = setInterval(() => {
      setIndex((prev) => (prev + 1) % styles.length);
    }, 1500);
    return () => clearInterval(interval);
  }, [animated]);

  const style = animated ? styles[index] : (currentStyle ?? 'solid');
  const width = style === 'double' ? 3 : 2;

  return (
    <motion.div
      className="absolute inset-4 rounded-lg pointer-events-none"
      style={{
        borderWidth: width,
        borderStyle: style,
        borderColor: color,
      }}
      animate={{ opacity: [0.6, 1, 0.6] }}
      transition={{ duration: 1.5, repeat: Infinity }}
    />
  );
});

// =============================================================================
// RadiatingPulse - For ArcFamily nodes
// =============================================================================

export interface RadiatingPulseProps {
  color: string;
  isHovered: boolean;
  selected: boolean;
  rayCount?: number;
}

export const RadiatingPulse = memo(function RadiatingPulse({
  color,
  isHovered,
  selected,
  rayCount = 6,
}: RadiatingPulseProps) {
  const rays = Array.from({ length: rayCount }, (_, i) => ({
    angle: (360 / rayCount) * i,
    delay: (i * 0.1),
  }));

  return (
    <svg className="absolute inset-0 w-full h-full overflow-visible pointer-events-none">
      <g transform="translate(50%, 50%)">
        {/* Central node */}
        <motion.circle
          cx="0"
          cy="0"
          r="8"
          fill={color}
          animate={{
            scale: selected ? [1, 1.2, 1] : 1,
          }}
          transition={{ duration: 1, repeat: selected ? Infinity : 0 }}
        />

        {/* Radiating rays */}
        {rays.map(({ angle, delay }) => (
          <motion.g
            key={angle}
            style={{ rotate: angle }}
          >
            <motion.line
              x1="12"
              y1="0"
              x2="35"
              y2="0"
              stroke={color}
              strokeWidth={selected ? 2 : 1.5}
              strokeOpacity={0.6}
              initial={{ pathLength: 0 }}
              animate={{
                pathLength: isHovered || selected ? 1 : 0.5,
              }}
              transition={{ duration: 0.5, delay }}
            />
            {/* Arrow head */}
            <motion.polygon
              points="35,0 30,-3 30,3"
              fill={color}
              fillOpacity={0.8}
              animate={{
                scale: isHovered ? 1.2 : 1,
              }}
              transition={{ duration: 0.3, delay }}
            />
          </motion.g>
        ))}
      </g>
    </svg>
  );
});
```

**Step 2: Update effects index**

Add to `apps/studio/src/components/graph/nodes/effects/index.ts`:
```typescript
export {
  OrbitalRings,
  StackedPlanes,
  BorderMorph,
  RadiatingPulse,
  type OrbitalRingsProps,
  type StackedPlanesProps,
  type BorderMorphProps,
  type RadiatingPulseProps,
} from './TaxonomyEffects';
```

**Step 3: Commit**

```bash
git add apps/studio/src/components/graph/nodes/effects/
git commit -m "feat(studio): add Taxonomy-specific effects (OrbitalRings, StackedPlanes, BorderMorph, RadiatingPulse)"
```

---

## Phase 2: Taxonomy Cards (4 tasks)

### Task 2.1: Create TaxonomyCard Base Component

**Files:**
- Create: `apps/studio/src/components/graph/nodes/taxonomy/TaxonomyCard.tsx`
- Create: `apps/studio/src/components/graph/nodes/taxonomy/index.ts`

**Step 1: Write TaxonomyCard**

```typescript
// apps/studio/src/components/graph/nodes/taxonomy/TaxonomyCard.tsx
'use client';

import { memo, useMemo } from 'react';
import { motion } from 'framer-motion';
import { Handle, Position, type NodeProps, type Node } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { CardShell } from '../card/CardShell';
import type { TaxonomyNodeData, TaxonomyVariant, CardContext } from '../card/types';
import {
  OrbitalRings,
  StackedPlanes,
  BorderMorph,
  RadiatingPulse,
  TechCorners,
} from '../effects';

// =============================================================================
// Types
// =============================================================================

export type TaxonomyNodeType = Node<TaxonomyNodeData>;

interface TaxonomyContentProps {
  data: TaxonomyNodeData;
  context: CardContext;
}

// =============================================================================
// Variant-Specific Content Renderers
// =============================================================================

const RealmContent = memo(function RealmContent({ data, context }: TaxonomyContentProps) {
  return (
    <>
      <OrbitalRings
        color={context.colors.primary}
        isHovered={context.isHovered}
        selected={context.selected}
      />
      <div className="relative z-10 p-4 text-center">
        <div className="text-[10px] font-mono uppercase tracking-widest text-white/50 mb-2">
          ◉ REALM
        </div>
        <h3 className="text-lg font-bold text-white mb-1">{data.displayName}</h3>
        <div
          className="inline-block px-2 py-0.5 rounded-full text-xs font-mono"
          style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
        >
          {data.count} nodes
        </div>
      </div>
    </>
  );
});

const LayerContent = memo(function LayerContent({ data, context }: TaxonomyContentProps) {
  return (
    <>
      <StackedPlanes
        color={context.colors.primary}
        isHovered={context.isHovered}
      />
      <div className="relative z-10 p-4 text-center">
        <div className="text-[10px] font-mono uppercase tracking-widest text-white/50 mb-2">
          ◈ LAYER
        </div>
        <h3 className="text-lg font-bold text-white mb-1">{data.displayName}</h3>
        <div
          className="inline-block px-2 py-0.5 rounded-full text-xs font-mono"
          style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
        >
          {data.count} nodes
        </div>
      </div>
    </>
  );
});

const TraitContent = memo(function TraitContent({ data, context }: TaxonomyContentProps) {
  return (
    <>
      <BorderMorph color={context.colors.primary} animated={context.isHovered} />
      <div className="relative z-10 p-4 text-center">
        <div className="text-[10px] font-mono uppercase tracking-widest text-white/50 mb-2">
          ◆ TRAIT
        </div>
        <h3 className="text-lg font-bold text-white mb-1">{data.displayName}</h3>
        <div
          className="inline-block px-2 py-0.5 rounded-full text-xs font-mono"
          style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
        >
          {data.count} nodes
        </div>
      </div>
    </>
  );
});

const ArcFamilyContent = memo(function ArcFamilyContent({ data, context }: TaxonomyContentProps) {
  return (
    <>
      <RadiatingPulse
        color={context.colors.primary}
        isHovered={context.isHovered}
        selected={context.selected}
      />
      <div className="relative z-10 p-4 text-center">
        <div className="text-[10px] font-mono uppercase tracking-widest text-white/50 mb-2">
          → ARC FAMILY
        </div>
        <h3 className="text-lg font-bold text-white mb-1">{data.displayName}</h3>
        <div
          className="inline-block px-2 py-0.5 rounded-full text-xs font-mono"
          style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
        >
          {data.count} arcs
        </div>
      </div>
    </>
  );
});

// =============================================================================
// Content Selector
// =============================================================================

const VARIANT_CONTENT: Record<TaxonomyVariant, React.FC<TaxonomyContentProps>> = {
  realm: RealmContent,
  layer: LayerContent,
  trait: TraitContent,
  arcFamily: ArcFamilyContent,
};

// =============================================================================
// Main Component
// =============================================================================

export const TaxonomyCard = memo(function TaxonomyCard(props: NodeProps<TaxonomyNodeType>) {
  const { data, selected = false } = props;

  const { isHovered, handleMouseEnter, handleMouseLeave } = useNodeInteractions({
    selected,
    isDimmed: data.dimmed === true,
    isHoverDimmed: data.hoverDimmed === true,
  });

  const ContentComponent = VARIANT_CONTENT[data.variant];

  return (
    <div
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      <Handle type="target" position={Position.Top} className="!w-3 !h-3 !opacity-0" />

      <CardShell data={data} width={200}>
        {(context) => (
          <div className="relative min-h-[140px]">
            {context.performanceConfig.effects.techCorners && (
              <TechCorners
                color={context.colors.primary}
                selected={context.selected}
                size={12}
              />
            )}
            <ContentComponent data={data} context={context} />
          </div>
        )}
      </CardShell>

      <Handle type="source" position={Position.Bottom} className="!w-3 !h-3 !opacity-0" />
    </div>
  );
});
```

**Step 2: Create index**

```typescript
// apps/studio/src/components/graph/nodes/taxonomy/index.ts
export { TaxonomyCard, type TaxonomyNodeType } from './TaxonomyCard';
```

**Step 3: Commit**

```bash
git add apps/studio/src/components/graph/nodes/taxonomy/
git commit -m "feat(studio): add TaxonomyCard with 4 variant-specific effects"
```

---

## Phase 3: Schema Cards (2 tasks)

### Task 3.1: Enhance ClassCardContent (NodeClass)

**Files:**
- Modify: `apps/studio/src/components/graph/nodes/card/presets/ClassCardContent.tsx`

(Continue with existing ClassCardContent enhancements...)

---

### Task 3.2: Create ArcClassCard

**Files:**
- Create: `apps/studio/src/components/graph/nodes/schema/ArcClassCard.tsx`

(Create ArcClass card with FlowingParticles effect...)

---

## Phase 4-6: Data Layer Cards, Integration, Testing

(Additional tasks for each layer: Foundation, Structure, Semantic, Instruction, Output, Knowledge, Locale/Geo)

---

## Summary

| Phase | Tasks | Est. Time |
|-------|-------|-----------|
| Phase 1: Foundation | 8 tasks | 4h |
| Phase 2: Taxonomy | 4 tasks | 2h |
| Phase 3: Schema | 2 tasks | 2h |
| Phase 4: Data Layers | 14 tasks | 6h |
| Phase 5: Integration | 4 tasks | 2h |
| Phase 6: Testing | 6 tasks | 3h |
| **Total** | **38 tasks** | **~19h** |

---

## Execution

Plan complete and saved to `docs/plans/2026-02-17-unified-card-system-implementation.md`.

**Two execution options:**

1. **Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

2. **Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**
