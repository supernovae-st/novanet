# Graph Performance Ultra-Optimization Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce memory from ~900MB to ~50MB, animations from 437k to <1k, achieve stable 60fps with 19k nodes/edges

**Architecture:** 6-phase progressive optimization - critical fixes first, then viewport culling, LOD system, node optimization, shared resources, and advanced techniques

**Tech Stack:** React 19 + React Flow + Zustand + IntersectionObserver + Web Workers + CSS Animations

---

## Current State (CRITICAL)

### Audit Findings - Edges

| Metric | Current | Problem |
|--------|---------|---------|
| Animations per edge | 16-72 (avg ~23) | Way too many |
| Total active animations | 437,000 | Browser can't handle |
| Blur filters per edge | 18 | GPU killer |
| Blur calculations/sec | 20.5 MILLION | Insane |
| Memory overhead | ~900 MB | Unacceptable |
| maxAnimatedEdges config | 100 | NEVER ENFORCED |

### Audit Findings - Nodes

| Metric | Current | Problem |
|--------|---------|---------|
| Inline style objects/node | 13 | Breaks memoization |
| Object allocations/frame | 247,000 | GC pressure |
| useMemo usage | None | Recomputes everything |

---

## Phase 1: Critical Fixes (Emergency Performance Recovery)

**Target:** 437k → 5k animations | Memory 900MB → 200MB

### Task 1.1: Create Animation Registry Store

**Files:**
- Create: `src/stores/animationStore.ts`
- Test: `src/stores/__tests__/animationStore.test.ts`

**Step 1: Write the failing test**

```typescript
// src/stores/__tests__/animationStore.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { useAnimationStore } from '../animationStore';

describe('animationStore', () => {
  beforeEach(() => {
    useAnimationStore.getState().reset();
  });

  it('should register edges up to maxAnimatedEdges limit', () => {
    const store = useAnimationStore.getState();

    // Register 150 edges (limit is 100)
    for (let i = 0; i < 150; i++) {
      store.registerEdge(`edge-${i}`);
    }

    expect(store.activeCount).toBe(100);
    expect(store.canAnimate('edge-50')).toBe(true);
    expect(store.canAnimate('edge-150')).toBe(false);
  });

  it('should allow animation after unregister', () => {
    const store = useAnimationStore.getState();

    for (let i = 0; i < 100; i++) {
      store.registerEdge(`edge-${i}`);
    }

    expect(store.canAnimate('edge-new')).toBe(false);

    store.unregisterEdge('edge-50');
    store.registerEdge('edge-new');

    expect(store.canAnimate('edge-new')).toBe(true);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/stores/__tests__/animationStore.test.ts`
Expected: FAIL with "Cannot find module '../animationStore'"

**Step 3: Write minimal implementation**

```typescript
// src/stores/animationStore.ts
import { create } from 'zustand';
import { PERFORMANCE_CONFIG } from '@/components/graph/edges/EdgeAnimationConfig';

interface AnimationStore {
  registeredEdges: Set<string>;
  activeCount: number;
  registerEdge: (id: string) => void;
  unregisterEdge: (id: string) => void;
  canAnimate: (id: string) => boolean;
  reset: () => void;
}

export const useAnimationStore = create<AnimationStore>((set, get) => ({
  registeredEdges: new Set(),
  activeCount: 0,

  registerEdge: (id: string) => {
    const { registeredEdges, activeCount } = get();
    if (registeredEdges.has(id)) return;
    if (activeCount >= PERFORMANCE_CONFIG.maxAnimatedEdges) return;

    set(state => ({
      registeredEdges: new Set(state.registeredEdges).add(id),
      activeCount: state.activeCount + 1,
    }));
  },

  unregisterEdge: (id: string) => {
    const { registeredEdges } = get();
    if (!registeredEdges.has(id)) return;

    const newSet = new Set(registeredEdges);
    newSet.delete(id);

    set({
      registeredEdges: newSet,
      activeCount: newSet.size,
    });
  },

  canAnimate: (id: string) => {
    return get().registeredEdges.has(id);
  },

  reset: () => {
    set({
      registeredEdges: new Set(),
      activeCount: 0,
    });
  },
}));
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/stores/__tests__/animationStore.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/stores/animationStore.ts src/stores/__tests__/animationStore.test.ts
git commit -m "feat(perf): add animation registry store with edge limit enforcement"
```

---

### Task 1.2: Integrate Animation Registry into FloatingEdge

**Files:**
- Modify: `src/components/graph/edges/FloatingEdge.tsx`
- Test: `src/components/graph/edges/__tests__/FloatingEdge.perf.test.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/graph/edges/__tests__/FloatingEdge.perf.test.tsx
import { describe, it, expect, beforeEach } from 'vitest';
import { render } from '@testing-library/react';
import { ReactFlowProvider } from '@xyflow/react';
import { FloatingEdge } from '../FloatingEdge';
import { useAnimationStore } from '@/stores/animationStore';

describe('FloatingEdge performance', () => {
  beforeEach(() => {
    useAnimationStore.getState().reset();
  });

  it('should not render particles when animation limit reached', () => {
    // Fill up animation slots
    const store = useAnimationStore.getState();
    for (let i = 0; i < 100; i++) {
      store.registerEdge(`other-edge-${i}`);
    }

    const { container } = render(
      <ReactFlowProvider>
        <svg>
          <FloatingEdge
            id="test-edge"
            source="node-1"
            target="node-2"
            data={{ relationType: 'HAS_TRANSLATION' }}
          />
        </svg>
      </ReactFlowProvider>
    );

    // Should NOT have animated particles (class or element)
    const particles = container.querySelectorAll('[class*="particle"], circle[r="3"]');
    expect(particles.length).toBe(0);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/edges/__tests__/FloatingEdge.perf.test.tsx`
Expected: FAIL (particles still render)

**Step 3: Update FloatingEdge implementation**

```typescript
// In FloatingEdge.tsx - add at top
import { useAnimationStore } from '@/stores/animationStore';

// Inside FloatingEdge component, after other hooks
const registerEdge = useAnimationStore(s => s.registerEdge);
const unregisterEdge = useAnimationStore(s => s.unregisterEdge);
const canAnimate = useAnimationStore(s => s.canAnimate);

// Register on mount, unregister on unmount
useEffect(() => {
  if (isAnimated && !isDimmed) {
    registerEdge(id);
  }
  return () => {
    unregisterEdge(id);
  };
}, [id, isAnimated, isDimmed, registerEdge, unregisterEdge]);

// Check if this edge can animate
const shouldAnimate = canAnimate(id) && isAnimated && !isDimmed;

// Replace isAnimated checks with shouldAnimate
// Line ~335: {isAnimated && !isDimmed && ( → {shouldAnimate && (
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/edges/__tests__/FloatingEdge.perf.test.tsx`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/edges/FloatingEdge.tsx src/components/graph/edges/__tests__/FloatingEdge.perf.test.tsx
git commit -m "feat(perf): integrate animation registry to enforce maxAnimatedEdges"
```

---

### Task 1.3: Reduce Particle Counts in EdgeAnimationConfig

**Files:**
- Modify: `src/components/graph/edges/EdgeAnimationConfig.ts`
- Test: `src/components/graph/edges/__tests__/EdgeAnimationConfig.test.ts`

**Step 1: Write the failing test**

```typescript
// src/components/graph/edges/__tests__/EdgeAnimationConfig.test.ts
import { describe, it, expect } from 'vitest';
import { PARTICLE_COUNTS, getParticleCount } from '../EdgeAnimationConfig';

describe('EdgeAnimationConfig particle counts', () => {
  it('should have reduced base particle count', () => {
    expect(PARTICLE_COUNTS.base).toBeLessThanOrEqual(3);
  });

  it('should have minimal tier with 1 particle', () => {
    expect(PARTICLE_COUNTS.minimal).toBe(1);
  });

  it('should return correct count for animation types', () => {
    expect(getParticleCount('pulse')).toBeLessThanOrEqual(4);
    expect(getParticleCount('flow')).toBeLessThanOrEqual(3);
    expect(getParticleCount('sparkle')).toBeLessThanOrEqual(2);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/edges/__tests__/EdgeAnimationConfig.test.ts`
Expected: FAIL (current base is 6-12)

**Step 3: Update EdgeAnimationConfig**

```typescript
// In EdgeAnimationConfig.ts - update PARTICLE_COUNTS
export const PARTICLE_COUNTS = {
  minimal: 1,      // NEW: Ultra-low for LOD
  base: 3,         // REDUCED from 6
  selected: 5,     // REDUCED from 12
  hovered: 4,      // REDUCED from 8
} as const;

// Update getParticleCount function
export function getParticleCount(animationType: AnimationType, isSelected?: boolean): number {
  if (isSelected) return PARTICLE_COUNTS.selected;

  const counts: Record<AnimationType, number> = {
    pulse: 4,      // REDUCED
    flow: 3,       // REDUCED
    sparkle: 2,    // REDUCED
    orbit: 3,      // REDUCED
    wave: 2,       // REDUCED
    burst: 4,      // REDUCED
    helix: 3,      // REDUCED
    comet: 2,      // REDUCED
  };

  return counts[animationType] ?? PARTICLE_COUNTS.base;
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/edges/__tests__/EdgeAnimationConfig.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/edges/EdgeAnimationConfig.ts src/components/graph/edges/__tests__/EdgeAnimationConfig.test.ts
git commit -m "perf(edges): reduce particle counts by 60% for better performance"
```

---

### Task 1.4: Reduce Blur Layers in EnergyEffects

**Files:**
- Modify: `src/components/graph/edges/EnergyEffects.tsx`
- Modify: `src/components/graph/edges/EdgeGlow.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/graph/edges/__tests__/EnergyEffects.perf.test.tsx
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import { ElectricEffect, FireEffect, PlasmaEffect, AuroraEffect } from '../EnergyEffects';

describe('EnergyEffects performance', () => {
  const mockProps = {
    edgePath: 'M0,0 L100,100',
    duration: 3,
    primaryColor: '#5E6AD2',
    secondaryColor: '#3b82f6',
    tertiaryColor: '#06b6d4',
    intensity: 1,
  };

  it('should have max 4 blur elements per effect', () => {
    const effects = [ElectricEffect, FireEffect, PlasmaEffect, AuroraEffect];

    effects.forEach(Effect => {
      const { container } = render(
        <svg><Effect {...mockProps} /></svg>
      );

      // Count elements with blur in style
      const blurElements = container.querySelectorAll('[style*="blur"]');
      expect(blurElements.length).toBeLessThanOrEqual(4);
    });
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/edges/__tests__/EnergyEffects.perf.test.tsx`
Expected: FAIL (current has 4-6 blur elements per effect)

**Step 3: Simplify EnergyEffects**

```typescript
// In EnergyEffects.tsx - simplify each effect to max 2 blur layers

// ElectricEffect - reduce from 4 blur paths to 2
export const ElectricEffect = memo(function ElectricEffect({...}) {
  return (
    <g className="energy-effect-electric">
      {/* Single glow layer instead of 2 */}
      <path
        d={edgePath}
        fill="none"
        stroke={primaryColor}
        strokeWidth={arcWidth * 2}
        opacity={0.4 * intensity}
        style={{ filter: `blur(${config.blurRadius}px)` }}
      />
      {/* Electric arcs - no blur, just dash animation */}
      <path
        d={edgePath}
        fill="none"
        stroke={primaryColor}
        strokeWidth={arcWidth}
        strokeDasharray="3,10"
        opacity={0.8 * intensity}
      >
        <animate ... />
      </path>
      {/* Spark bursts - no blur */}
      <path
        d={edgePath}
        fill="none"
        stroke="white"
        strokeWidth={sparkWidth}
        strokeDasharray="2,16"
        opacity={0.95 * intensity}
      >
        <animate ... />
      </path>
    </g>
  );
});

// Similar simplification for Fire, Plasma, Aurora
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/edges/__tests__/EnergyEffects.perf.test.tsx`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/edges/EnergyEffects.tsx src/components/graph/edges/__tests__/EnergyEffects.perf.test.tsx
git commit -m "perf(edges): reduce blur layers from 18 to 4 max per edge"
```

---

### Task 1.5: Reduce Glow Layers in EdgeGlow

**Files:**
- Modify: `src/components/graph/edges/EdgeGlow.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/graph/edges/__tests__/EdgeGlow.perf.test.tsx
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import { EdgeGlowLayers } from '../EdgeGlow';

describe('EdgeGlow performance', () => {
  it('should render max 2 glow layers by default', () => {
    const { container } = render(
      <svg>
        <EdgeGlowLayers
          edgePath="M0,0 L100,100"
          strokeWidth={5}
          glowColor="#5E6AD2"
          primaryColor="#3b82f6"
          glowIntensity={1}
          isSelected={false}
          isHovered={false}
          animated={true}
        />
      </svg>
    );

    const paths = container.querySelectorAll('path');
    expect(paths.length).toBeLessThanOrEqual(2);
  });

  it('should render max 3 layers when selected', () => {
    const { container } = render(
      <svg>
        <EdgeGlowLayers
          edgePath="M0,0 L100,100"
          strokeWidth={5}
          glowColor="#5E6AD2"
          primaryColor="#3b82f6"
          glowIntensity={1}
          isSelected={true}
          isHovered={false}
          animated={true}
        />
      </svg>
    );

    const paths = container.querySelectorAll('path');
    expect(paths.length).toBeLessThanOrEqual(3);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/edges/__tests__/EdgeGlow.perf.test.tsx`
Expected: FAIL (current renders 4+ layers)

**Step 3: Simplify EdgeGlow**

```typescript
// In EdgeGlow.tsx - simplify to 2 layers max (3 when selected)

export const EdgeGlowLayers = memo(function EdgeGlowLayers({
  edgePath,
  strokeWidth,
  glowColor,
  primaryColor,
  glowIntensity,
  isSelected,
  isHovered,
  animated,
}: EdgeGlowProps) {
  // Only 1 outer glow + 1 inner glow normally
  // Add 1 extra layer when selected
  const layers = isSelected ? 3 : isHovered ? 2 : 2;

  return (
    <g className="edge-glow-layers">
      {/* Outer glow - always present */}
      <OuterGlow
        edgePath={edgePath}
        strokeWidth={strokeWidth}
        glowColor={glowColor}
        intensity={glowIntensity}
        animated={animated}
      />

      {/* Inner glow - always present */}
      <InnerGlow
        edgePath={edgePath}
        strokeWidth={strokeWidth}
        primaryColor={primaryColor}
        intensity={glowIntensity}
      />

      {/* Extra bright layer only when selected */}
      {isSelected && (
        <path
          d={edgePath}
          fill="none"
          stroke="white"
          strokeWidth={strokeWidth * 0.5}
          opacity={0.3}
          style={{ filter: 'blur(2px)' }}
        />
      )}
    </g>
  );
});
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/edges/__tests__/EdgeGlow.perf.test.tsx`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/edges/EdgeGlow.tsx src/components/graph/edges/__tests__/EdgeGlow.perf.test.tsx
git commit -m "perf(edges): reduce glow layers from 4+ to 2-3 max"
```

---

## Phase 2: Viewport Culling

**Target:** Only visible edges animate (50-200 vs 19,000)

### Task 2.1: Create EdgeVisibilityManager

**Files:**
- Create: `src/components/graph/edges/EdgeVisibilityManager.tsx`
- Test: `src/components/graph/edges/__tests__/EdgeVisibilityManager.test.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/graph/edges/__tests__/EdgeVisibilityManager.test.tsx
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useEdgeVisibility, EdgeVisibilityProvider } from '../EdgeVisibilityManager';

// Mock IntersectionObserver
const mockObserve = vi.fn();
const mockUnobserve = vi.fn();
const mockDisconnect = vi.fn();

beforeEach(() => {
  global.IntersectionObserver = vi.fn().mockImplementation((callback) => ({
    observe: mockObserve,
    unobserve: mockUnobserve,
    disconnect: mockDisconnect,
    callback,
  }));
});

describe('EdgeVisibilityManager', () => {
  it('should track visible edges', () => {
    const { result } = renderHook(() => useEdgeVisibility(), {
      wrapper: EdgeVisibilityProvider,
    });

    act(() => {
      result.current.setVisible('edge-1', true);
      result.current.setVisible('edge-2', true);
    });

    expect(result.current.isVisible('edge-1')).toBe(true);
    expect(result.current.isVisible('edge-2')).toBe(true);
    expect(result.current.isVisible('edge-3')).toBe(false);
  });

  it('should remove edge from visible set', () => {
    const { result } = renderHook(() => useEdgeVisibility(), {
      wrapper: EdgeVisibilityProvider,
    });

    act(() => {
      result.current.setVisible('edge-1', true);
      result.current.setVisible('edge-1', false);
    });

    expect(result.current.isVisible('edge-1')).toBe(false);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/edges/__tests__/EdgeVisibilityManager.test.tsx`
Expected: FAIL (module not found)

**Step 3: Implement EdgeVisibilityManager**

```typescript
// src/components/graph/edges/EdgeVisibilityManager.tsx
'use client';

import { createContext, useContext, useCallback, useMemo, useRef, ReactNode } from 'react';
import { create } from 'zustand';

interface EdgeVisibilityState {
  visibleEdges: Set<string>;
  setVisible: (id: string, visible: boolean) => void;
  isVisible: (id: string) => boolean;
  clear: () => void;
}

const useEdgeVisibilityStore = create<EdgeVisibilityState>((set, get) => ({
  visibleEdges: new Set(),

  setVisible: (id: string, visible: boolean) => {
    set(state => {
      const newSet = new Set(state.visibleEdges);
      if (visible) {
        newSet.add(id);
      } else {
        newSet.delete(id);
      }
      return { visibleEdges: newSet };
    });
  },

  isVisible: (id: string) => get().visibleEdges.has(id),

  clear: () => set({ visibleEdges: new Set() }),
}));

interface EdgeVisibilityContextValue {
  observerRef: React.MutableRefObject<IntersectionObserver | null>;
  registerEdge: (id: string, element: Element) => void;
  unregisterEdge: (id: string, element: Element) => void;
}

const EdgeVisibilityContext = createContext<EdgeVisibilityContextValue | null>(null);

export function EdgeVisibilityProvider({ children }: { children: ReactNode }) {
  const observerRef = useRef<IntersectionObserver | null>(null);
  const elementMap = useRef<Map<Element, string>>(new Map());
  const { setVisible } = useEdgeVisibilityStore();

  // Create observer on mount
  if (!observerRef.current && typeof window !== 'undefined') {
    observerRef.current = new IntersectionObserver(
      (entries) => {
        entries.forEach(entry => {
          const edgeId = elementMap.current.get(entry.target);
          if (edgeId) {
            setVisible(edgeId, entry.isIntersecting);
          }
        });
      },
      { rootMargin: '100px' } // Buffer for smooth transitions
    );
  }

  const registerEdge = useCallback((id: string, element: Element) => {
    elementMap.current.set(element, id);
    observerRef.current?.observe(element);
  }, []);

  const unregisterEdge = useCallback((id: string, element: Element) => {
    elementMap.current.delete(element);
    observerRef.current?.unobserve(element);
    setVisible(id, false);
  }, [setVisible]);

  const value = useMemo(() => ({
    observerRef,
    registerEdge,
    unregisterEdge,
  }), [registerEdge, unregisterEdge]);

  return (
    <EdgeVisibilityContext.Provider value={value}>
      {children}
    </EdgeVisibilityContext.Provider>
  );
}

export function useEdgeVisibility() {
  const context = useContext(EdgeVisibilityContext);
  const store = useEdgeVisibilityStore();

  return {
    ...store,
    registerEdge: context?.registerEdge ?? (() => {}),
    unregisterEdge: context?.unregisterEdge ?? (() => {}),
  };
}

export { useEdgeVisibilityStore };
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/edges/__tests__/EdgeVisibilityManager.test.tsx`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/edges/EdgeVisibilityManager.tsx src/components/graph/edges/__tests__/EdgeVisibilityManager.test.tsx
git commit -m "feat(perf): add EdgeVisibilityManager with IntersectionObserver"
```

---

### Task 2.2: Integrate EdgeVisibilityProvider into Graph2D

**Files:**
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/graph/__tests__/Graph2D.visibility.test.tsx
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import { Graph2D } from '../Graph2D';

describe('Graph2D visibility integration', () => {
  it('should wrap ReactFlow with EdgeVisibilityProvider', () => {
    const { container } = render(
      <Graph2D nodes={[]} edges={[]} />
    );

    // Provider should be in tree (check via context)
    expect(container.querySelector('[data-testid="react-flow-wrapper"]')).toBeTruthy();
  });
});
```

**Step 2: Update Graph2D**

```typescript
// In Graph2D.tsx
import { EdgeVisibilityProvider } from './edges/EdgeVisibilityManager';

// Wrap the ReactFlow component
return (
  <EdgeVisibilityProvider>
    <div ref={containerRef} className="..." data-testid="react-flow-wrapper">
      <ReactFlow ...>
        {/* existing content */}
      </ReactFlow>
    </div>
  </EdgeVisibilityProvider>
);
```

**Step 3: Commit**

```bash
git add src/components/graph/Graph2D.tsx src/components/graph/__tests__/Graph2D.visibility.test.tsx
git commit -m "feat(perf): integrate EdgeVisibilityProvider into Graph2D"
```

---

### Task 2.3: Use Visibility in FloatingEdge

**Files:**
- Modify: `src/components/graph/edges/FloatingEdge.tsx`

**Step 1: Write the failing test**

```typescript
// Add to FloatingEdge.perf.test.tsx
it('should not render effects when not visible', () => {
  // Mock visibility to false
  vi.mock('../EdgeVisibilityManager', () => ({
    useEdgeVisibility: () => ({
      isVisible: () => false,
      registerEdge: vi.fn(),
      unregisterEdge: vi.fn(),
    }),
  }));

  const { container } = render(
    <ReactFlowProvider>
      <svg>
        <FloatingEdge
          id="test-edge"
          source="node-1"
          target="node-2"
          data={{ relationType: 'HAS_TRANSLATION' }}
        />
      </svg>
    </ReactFlowProvider>
  );

  // Should only have static path, no effects
  const effects = container.querySelectorAll('.energy-effect-*, .edge-glow-*');
  expect(effects.length).toBe(0);
});
```

**Step 2: Update FloatingEdge**

```typescript
// In FloatingEdge.tsx
import { useEdgeVisibility } from './EdgeVisibilityManager';

// Inside component
const { isVisible, registerEdge, unregisterEdge } = useEdgeVisibility();
const pathRef = useRef<SVGPathElement>(null);

// Register with visibility observer
useEffect(() => {
  const element = pathRef.current;
  if (element) {
    registerEdge(id, element);
    return () => unregisterEdge(id, element);
  }
}, [id, registerEdge, unregisterEdge]);

const isEdgeVisible = isVisible(id);

// Only animate if visible AND within animation limit
const shouldAnimate = isEdgeVisible && canAnimate(id) && isAnimated && !isDimmed;
const shouldRenderEffects = isEdgeVisible && !isDimmed;

// Update JSX to use shouldRenderEffects
{shouldRenderEffects && (
  <EnergyEffectRenderer ... />
)}
{shouldRenderEffects && (
  <EdgeGlowLayers ... />
)}
{shouldAnimate && (
  <AnimatedParticles ... />
)}
```

**Step 3: Commit**

```bash
git add src/components/graph/edges/FloatingEdge.tsx
git commit -m "feat(perf): integrate visibility culling in FloatingEdge"
```

---

## Phase 3: Level of Detail (LOD) System

**Target:** Appropriate detail per zoom level

### Task 3.1: Create LOD Configuration

**Files:**
- Create: `src/components/graph/config/LODConfig.ts`
- Test: `src/components/graph/config/__tests__/LODConfig.test.ts`

**Step 1: Write the failing test**

```typescript
// src/components/graph/config/__tests__/LODConfig.test.ts
import { describe, it, expect } from 'vitest';
import { LOD_TIERS, getLODTier } from '../LODConfig';

describe('LODConfig', () => {
  it('should return ULTRA for zoom > 1.5', () => {
    expect(getLODTier(2.0)).toBe(LOD_TIERS.ULTRA);
    expect(getLODTier(1.6)).toBe(LOD_TIERS.ULTRA);
  });

  it('should return HIGH for zoom 0.8-1.5', () => {
    expect(getLODTier(1.0)).toBe(LOD_TIERS.HIGH);
    expect(getLODTier(0.9)).toBe(LOD_TIERS.HIGH);
  });

  it('should return MEDIUM for zoom 0.4-0.8', () => {
    expect(getLODTier(0.6)).toBe(LOD_TIERS.MEDIUM);
  });

  it('should return LOW for zoom 0.2-0.4', () => {
    expect(getLODTier(0.3)).toBe(LOD_TIERS.LOW);
  });

  it('should return MINIMAL for zoom < 0.2', () => {
    expect(getLODTier(0.1)).toBe(LOD_TIERS.MINIMAL);
  });

  it('MINIMAL tier should disable all effects', () => {
    expect(LOD_TIERS.MINIMAL.particles).toBe(false);
    expect(LOD_TIERS.MINIMAL.energyEffects).toBe(false);
    expect(LOD_TIERS.MINIMAL.glowLayers).toBe(0);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/config/__tests__/LODConfig.test.ts`
Expected: FAIL (module not found)

**Step 3: Implement LODConfig**

```typescript
// src/components/graph/config/LODConfig.ts
export interface LODTier {
  particles: boolean;
  energyEffects: boolean | 'simplified';
  glowLayers: number;
  labels: boolean;
  blurQuality: 'high' | 'medium' | 'low' | 'none';
}

export const LOD_TIERS = {
  ULTRA: {
    particles: true,
    energyEffects: true,
    glowLayers: 3,
    labels: true,
    blurQuality: 'high',
  },
  HIGH: {
    particles: true,
    energyEffects: true,
    glowLayers: 2,
    labels: true,
    blurQuality: 'medium',
  },
  MEDIUM: {
    particles: false,
    energyEffects: 'simplified',
    glowLayers: 1,
    labels: false,
    blurQuality: 'low',
  },
  LOW: {
    particles: false,
    energyEffects: false,
    glowLayers: 0,
    labels: false,
    blurQuality: 'none',
  },
  MINIMAL: {
    particles: false,
    energyEffects: false,
    glowLayers: 0,
    labels: false,
    blurQuality: 'none',
  },
} as const satisfies Record<string, LODTier>;

export function getLODTier(zoom: number): LODTier {
  if (zoom > 1.5) return LOD_TIERS.ULTRA;
  if (zoom > 0.8) return LOD_TIERS.HIGH;
  if (zoom > 0.4) return LOD_TIERS.MEDIUM;
  if (zoom > 0.2) return LOD_TIERS.LOW;
  return LOD_TIERS.MINIMAL;
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/config/__tests__/LODConfig.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/config/LODConfig.ts src/components/graph/config/__tests__/LODConfig.test.ts
git commit -m "feat(perf): add LOD configuration with 5 zoom-based tiers"
```

---

### Task 3.2: Create useLOD Hook

**Files:**
- Create: `src/hooks/useLOD.ts`
- Test: `src/hooks/__tests__/useLOD.test.ts`

**Step 1: Write the failing test**

```typescript
// src/hooks/__tests__/useLOD.test.ts
import { describe, it, expect, vi } from 'vitest';
import { renderHook } from '@testing-library/react';
import { useLOD } from '../useLOD';
import { LOD_TIERS } from '@/components/graph/config/LODConfig';

// Mock React Flow store
vi.mock('@xyflow/react', () => ({
  useStore: vi.fn((selector) => selector({ transform: [0, 0, 1.0] })),
}));

describe('useLOD', () => {
  it('should return correct LOD tier based on zoom', () => {
    const { result } = renderHook(() => useLOD());
    expect(result.current).toEqual(LOD_TIERS.HIGH); // zoom 1.0
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/hooks/__tests__/useLOD.test.ts`
Expected: FAIL (module not found)

**Step 3: Implement useLOD**

```typescript
// src/hooks/useLOD.ts
import { useMemo } from 'react';
import { useStore } from '@xyflow/react';
import { getLODTier, type LODTier } from '@/components/graph/config/LODConfig';

export function useLOD(): LODTier {
  const zoom = useStore(state => state.transform[2]);

  return useMemo(() => getLODTier(zoom), [zoom]);
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/hooks/__tests__/useLOD.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/hooks/useLOD.ts src/hooks/__tests__/useLOD.test.ts
git commit -m "feat(perf): add useLOD hook for zoom-based detail levels"
```

---

### Task 3.3: Integrate LOD into FloatingEdge

**Files:**
- Modify: `src/components/graph/edges/FloatingEdge.tsx`

**Step 1: Write the failing test**

```typescript
// Add to FloatingEdge.perf.test.tsx
it('should not render particles at LOW LOD', () => {
  // Mock zoom to 0.3 (LOW tier)
  vi.mock('@xyflow/react', () => ({
    useStore: vi.fn((selector) => selector({ transform: [0, 0, 0.3] })),
    useInternalNode: vi.fn(() => mockNode),
  }));

  const { container } = render(
    <svg>
      <FloatingEdge id="test" source="a" target="b" data={{ relationType: 'TEST' }} />
    </svg>
  );

  expect(container.querySelectorAll('circle').length).toBe(0);
});
```

**Step 2: Update FloatingEdge**

```typescript
// In FloatingEdge.tsx
import { useLOD } from '@/hooks/useLOD';

// Inside component
const lod = useLOD();

// Update conditionals to use LOD
const shouldRenderParticles = shouldAnimate && lod.particles;
const shouldRenderEnergyEffects = shouldRenderEffects && lod.energyEffects;
const glowLayerCount = Math.min(lod.glowLayers, 3);
const shouldRenderLabels = shouldShowLabel && lod.labels;

// Update JSX
{shouldRenderEnergyEffects && (
  <EnergyEffectRenderer
    type={lod.energyEffects === 'simplified' ? 'aurora' : energyEffect}
    ...
  />
)}
{glowLayerCount > 0 && (
  <EdgeGlowLayers layers={glowLayerCount} ... />
)}
{shouldRenderParticles && (
  <AnimatedParticles ... />
)}
{shouldRenderLabels && (
  // Label JSX
)}
```

**Step 3: Commit**

```bash
git add src/components/graph/edges/FloatingEdge.tsx
git commit -m "feat(perf): integrate LOD system into FloatingEdge"
```

---

## Phase 4: Node Optimization

**Target:** 247k → 5k allocations/frame

### Task 4.1: Create Node Style Factory

**Files:**
- Create: `src/components/graph/nodes/NodeStyles.ts`
- Test: `src/components/graph/nodes/__tests__/NodeStyles.test.ts`

**Step 1: Write the failing test**

```typescript
// src/components/graph/nodes/__tests__/NodeStyles.test.ts
import { describe, it, expect } from 'vitest';
import { getNodeContainerStyle, getNodeHeaderStyle } from '../NodeStyles';

describe('NodeStyles factory', () => {
  it('should return same object for same inputs (memoized)', () => {
    const style1 = getNodeContainerStyle(200, 100, false, false);
    const style2 = getNodeContainerStyle(200, 100, false, false);

    expect(style1).toBe(style2); // Same reference, not just equal
  });

  it('should return different objects for different inputs', () => {
    const style1 = getNodeContainerStyle(200, 100, false, false);
    const style2 = getNodeContainerStyle(200, 100, true, false);

    expect(style1).not.toBe(style2);
  });

  it('should apply dimmed opacity', () => {
    const style = getNodeContainerStyle(200, 100, true, false);
    expect(style.opacity).toBe(0.3);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/nodes/__tests__/NodeStyles.test.ts`
Expected: FAIL (module not found)

**Step 3: Implement NodeStyles**

```typescript
// src/components/graph/nodes/NodeStyles.ts
import type { CSSProperties } from 'react';

// Style caches
const containerCache = new Map<string, CSSProperties>();
const headerCache = new Map<string, CSSProperties>();
const contentCache = new Map<string, CSSProperties>();

function cacheKey(...args: (string | number | boolean)[]): string {
  return args.join('-');
}

export function getNodeContainerStyle(
  width: number,
  height: number,
  isDimmed: boolean,
  isSelected: boolean
): CSSProperties {
  const key = cacheKey(width, height, isDimmed, isSelected);

  if (!containerCache.has(key)) {
    containerCache.set(key, {
      width,
      height,
      opacity: isDimmed ? 0.3 : 1,
      transform: isSelected ? 'scale(1.02)' : undefined,
      transition: 'opacity 0.2s ease, transform 0.2s ease',
    });
  }

  return containerCache.get(key)!;
}

export function getNodeHeaderStyle(
  primaryColor: string,
  isHovered: boolean
): CSSProperties {
  const key = cacheKey(primaryColor, isHovered);

  if (!headerCache.has(key)) {
    headerCache.set(key, {
      backgroundColor: primaryColor,
      opacity: isHovered ? 1 : 0.9,
    });
  }

  return headerCache.get(key)!;
}

export function getNodeContentStyle(
  isDimmed: boolean
): CSSProperties {
  const key = cacheKey(isDimmed);

  if (!contentCache.has(key)) {
    contentCache.set(key, {
      opacity: isDimmed ? 0.5 : 1,
    });
  }

  return contentCache.get(key)!;
}

// Clear caches (for testing or memory management)
export function clearStyleCaches(): void {
  containerCache.clear();
  headerCache.clear();
  contentCache.clear();
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/nodes/__tests__/NodeStyles.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/nodes/NodeStyles.ts src/components/graph/nodes/__tests__/NodeStyles.test.ts
git commit -m "feat(perf): add memoized NodeStyles factory to reduce allocations"
```

---

### Task 4.2: Create Node Configuration Lookup Tables

**Files:**
- Create: `src/components/graph/nodes/NodeConfig.ts`
- Test: `src/components/graph/nodes/__tests__/NodeConfig.test.ts`

**Step 1: Write the failing test**

```typescript
// src/components/graph/nodes/__tests__/NodeConfig.test.ts
import { describe, it, expect } from 'vitest';
import { NODE_SIZES, NODE_COLORS, getNodeConfig } from '../NodeConfig';

describe('NodeConfig lookup tables', () => {
  it('should have predefined sizes for all node types', () => {
    expect(NODE_SIZES.Project).toBeDefined();
    expect(NODE_SIZES.TranslationUnit).toBeDefined();
    expect(NODE_SIZES.Locale).toBeDefined();
  });

  it('should have predefined colors for all node types', () => {
    expect(NODE_COLORS.Project.primary).toBeDefined();
    expect(NODE_COLORS.Project.secondary).toBeDefined();
  });

  it('getNodeConfig should return combined config', () => {
    const config = getNodeConfig('Project');
    expect(config.size).toEqual(NODE_SIZES.Project);
    expect(config.colors).toEqual(NODE_COLORS.Project);
  });

  it('should return default for unknown type', () => {
    const config = getNodeConfig('Unknown' as any);
    expect(config.size).toBeDefined();
    expect(config.colors).toBeDefined();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/components/graph/nodes/__tests__/NodeConfig.test.ts`
Expected: FAIL (module not found)

**Step 3: Implement NodeConfig**

```typescript
// src/components/graph/nodes/NodeConfig.ts
import type { NodeType } from '@/types';

export interface NodeSize {
  width: number;
  height: number;
}

export interface NodeColors {
  primary: string;
  secondary: string;
  tertiary: string;
  glow: string;
}

export const NODE_SIZES: Record<NodeType, NodeSize> = {
  Project: { width: 280, height: 140 },
  Locale: { width: 180, height: 80 },
  Source: { width: 220, height: 100 },
  TranslationUnit: { width: 240, height: 110 },
  AITranslation: { width: 220, height: 100 },
  HumanTranslation: { width: 220, height: 100 },
  PlatformOutput: { width: 200, height: 90 },
  Concept: { width: 160, height: 80 },
  Expression: { width: 200, height: 90 },
  Term: { width: 160, height: 70 },
  Glossary: { width: 200, height: 90 },
  TranslationMemory: { width: 220, height: 100 },
  StyleGuide: { width: 200, height: 90 },
  QARule: { width: 180, height: 80 },
  Provider: { width: 180, height: 80 },
  Model: { width: 180, height: 80 },
  Pipeline: { width: 240, height: 110 },
};

export const NODE_COLORS: Record<NodeType, NodeColors> = {
  Project: { primary: '#5E6AD2', secondary: '#3b82f6', tertiary: '#06b6d4', glow: '#5E6AD2' },
  Locale: { primary: '#10b981', secondary: '#22c55e', tertiary: '#6ee7b7', glow: '#10b981' },
  Source: { primary: '#f59e0b', secondary: '#fbbf24', tertiary: '#fde047', glow: '#f59e0b' },
  TranslationUnit: { primary: '#8b5cf6', secondary: '#a78bfa', tertiary: '#c4b5fd', glow: '#8b5cf6' },
  AITranslation: { primary: '#06b6d4', secondary: '#22d3ee', tertiary: '#67e8f9', glow: '#06b6d4' },
  HumanTranslation: { primary: '#ec4899', secondary: '#f472b6', tertiary: '#f9a8d4', glow: '#ec4899' },
  PlatformOutput: { primary: '#f97316', secondary: '#fb923c', tertiary: '#fdba74', glow: '#f97316' },
  Concept: { primary: '#9333ea', secondary: '#a855f7', tertiary: '#c084fc', glow: '#9333ea' },
  Expression: { primary: '#14b8a6', secondary: '#2dd4bf', tertiary: '#5eead4', glow: '#14b8a6' },
  Term: { primary: '#eab308', secondary: '#facc15', tertiary: '#fde047', glow: '#eab308' },
  Glossary: { primary: '#84cc16', secondary: '#a3e635', tertiary: '#bef264', glow: '#84cc16' },
  TranslationMemory: { primary: '#0ea5e9', secondary: '#38bdf8', tertiary: '#7dd3fc', glow: '#0ea5e9' },
  StyleGuide: { primary: '#d946ef', secondary: '#e879f9', tertiary: '#f0abfc', glow: '#d946ef' },
  QARule: { primary: '#ef4444', secondary: '#f87171', tertiary: '#fca5a5', glow: '#ef4444' },
  Provider: { primary: '#64748b', secondary: '#94a3b8', tertiary: '#cbd5e1', glow: '#64748b' },
  Model: { primary: '#6366f1', secondary: '#818cf8', tertiary: '#a5b4fc', glow: '#6366f1' },
  Pipeline: { primary: '#22c55e', secondary: '#4ade80', tertiary: '#86efac', glow: '#22c55e' },
};

const DEFAULT_CONFIG = {
  size: { width: 200, height: 100 },
  colors: { primary: '#64748b', secondary: '#94a3b8', tertiary: '#cbd5e1', glow: '#64748b' },
};

export interface NodeConfig {
  size: NodeSize;
  colors: NodeColors;
}

export function getNodeConfig(type: NodeType): NodeConfig {
  return {
    size: NODE_SIZES[type] ?? DEFAULT_CONFIG.size,
    colors: NODE_COLORS[type] ?? DEFAULT_CONFIG.colors,
  };
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/components/graph/nodes/__tests__/NodeConfig.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/nodes/NodeConfig.ts src/components/graph/nodes/__tests__/NodeConfig.test.ts
git commit -m "feat(perf): add NodeConfig lookup tables for O(1) access"
```

---

### Task 4.3: Update TurboNode to Use Style Factory

**Files:**
- Modify: `src/components/graph/TurboNode.tsx`

**Step 1: Write the failing test**

```typescript
// src/components/graph/__tests__/TurboNode.perf.test.tsx
import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import { TurboNode } from '../TurboNode';
import * as NodeStyles from '../nodes/NodeStyles';

describe('TurboNode performance', () => {
  it('should use memoized style factory', () => {
    const spy = vi.spyOn(NodeStyles, 'getNodeContainerStyle');

    const { rerender } = render(
      <TurboNode
        id="test"
        data={{ label: 'Test', nodeType: 'Project' }}
        type="turbo"
        positionAbsoluteX={0}
        positionAbsoluteY={0}
      />
    );

    // Rerender with same props
    rerender(
      <TurboNode
        id="test"
        data={{ label: 'Test', nodeType: 'Project' }}
        type="turbo"
        positionAbsoluteX={0}
        positionAbsoluteY={0}
      />
    );

    // Should have called factory, which returns cached result
    expect(spy).toHaveBeenCalled();
  });
});
```

**Step 2: Update TurboNode**

```typescript
// In TurboNode.tsx
import { getNodeContainerStyle, getNodeHeaderStyle } from './nodes/NodeStyles';
import { getNodeConfig } from './nodes/NodeConfig';

// Replace inline styles with factory calls
const TurboNode = memo(function TurboNode({ data, selected, ... }) {
  const config = getNodeConfig(data.nodeType);

  // Use factory for container style
  const containerStyle = getNodeContainerStyle(
    config.size.width,
    config.size.height,
    data.dimmed ?? false,
    selected ?? false
  );

  // Use factory for header style
  const headerStyle = getNodeHeaderStyle(
    config.colors.primary,
    data.hovered ?? false
  );

  return (
    <div style={containerStyle} className="...">
      <div style={headerStyle} className="...">
        {/* header content */}
      </div>
      {/* rest of node */}
    </div>
  );
});
```

**Step 3: Commit**

```bash
git add src/components/graph/TurboNode.tsx src/components/graph/__tests__/TurboNode.perf.test.tsx
git commit -m "perf(nodes): use memoized style factory in TurboNode"
```

---

## Phase 5: Shared Resources (Complete the Fix)

### Task 5.1: Add Shared Marker Definitions

**Files:**
- Modify: `src/components/graph/edges/SharedSVGDefs.tsx`

**Step 1: Implement SharedMarkerDefinitions**

```typescript
// In SharedSVGDefs.tsx - add shared markers

const MARKER_TYPES = ['arrow', 'diamond', 'circle', 'square', 'triangle'] as const;

const SharedMarkerDefinitions = memo(function SharedMarkerDefinitions() {
  return (
    <>
      {MARKER_TYPES.map(type => (
        Object.entries(RELATIONSHIP_COLORS).map(([scheme, colors]) => (
          <marker
            key={`marker-${type}-${scheme}`}
            id={`marker-${type}-${scheme}`}
            viewBox="0 0 10 10"
            refX="5"
            refY="5"
            markerWidth="6"
            markerHeight="6"
            orient="auto"
          >
            {type === 'arrow' && (
              <path d="M0,0 L10,5 L0,10 z" fill={colors.primary} />
            )}
            {type === 'diamond' && (
              <path d="M5,0 L10,5 L5,10 L0,5 z" fill={colors.primary} />
            )}
            {type === 'circle' && (
              <circle cx="5" cy="5" r="4" fill={colors.primary} />
            )}
            {type === 'square' && (
              <rect x="1" y="1" width="8" height="8" fill={colors.primary} />
            )}
            {type === 'triangle' && (
              <path d="M5,0 L10,10 L0,10 z" fill={colors.primary} />
            )}
          </marker>
        ))
      ))}
    </>
  );
});

// Add to SharedSVGDefs component
export const SharedSVGDefs = memo(function SharedSVGDefs() {
  return (
    <defs>
      <SharedNeonFilters />
      <SharedMarkerDefinitions />
      {/* ... existing gradients ... */}
    </defs>
  );
});
```

**Step 2: Commit**

```bash
git add src/components/graph/edges/SharedSVGDefs.tsx
git commit -m "feat(perf): add shared marker definitions to SharedSVGDefs"
```

---

### Task 5.2: Add CSS Keyframe Animations

**Files:**
- Modify: `src/components/graph/edges/SharedSVGDefs.tsx`

**Step 1: Add shared animations**

```typescript
// In SharedSVGDefs.tsx - add CSS animations

const SharedAnimations = memo(function SharedAnimations() {
  return (
    <style>{`
      @keyframes edge-dash-flow {
        from { stroke-dashoffset: 0; }
        to { stroke-dashoffset: 50; }
      }

      @keyframes edge-glow-pulse {
        0%, 100% { opacity: 0.4; stroke-width: var(--base-width); }
        50% { opacity: 0.7; stroke-width: calc(var(--base-width) * 1.2); }
      }

      @keyframes particle-move {
        from { offset-distance: 0%; }
        to { offset-distance: 100%; }
      }

      @keyframes spark-flicker {
        0%, 100% { opacity: 1; }
        25% { opacity: 0.3; }
        50% { opacity: 0.8; }
        75% { opacity: 0.2; }
      }

      .animate-dash-flow {
        animation: edge-dash-flow 3s linear infinite;
      }

      .animate-glow-pulse {
        animation: edge-glow-pulse 2s ease-in-out infinite;
      }

      .animate-particle {
        offset-path: path(var(--edge-path));
        animation: particle-move var(--duration, 3s) linear infinite;
      }

      .animate-spark {
        animation: spark-flicker 0.5s ease-in-out infinite;
      }
    `}</style>
  );
});

// Add to SharedSVGDefs
export const SharedSVGDefs = memo(function SharedSVGDefs() {
  return (
    <defs>
      <SharedAnimations />
      <SharedNeonFilters />
      <SharedMarkerDefinitions />
      {/* ... */}
    </defs>
  );
});
```

**Step 2: Commit**

```bash
git add src/components/graph/edges/SharedSVGDefs.tsx
git commit -m "feat(perf): add shared CSS keyframe animations"
```

---

## Phase 6: Advanced Optimizations

### Task 6.1: Add Web Worker for Path Calculations

**Files:**
- Create: `public/workers/pathWorker.js`
- Create: `src/hooks/usePathWorker.ts`
- Test: `src/hooks/__tests__/usePathWorker.test.ts`

**Step 1: Create worker**

```javascript
// public/workers/pathWorker.js
self.onmessage = function(e) {
  const { sourceNode, targetNode, edgePadding } = e.data;

  // Calculate intersection points
  const sourcePoint = getNodeIntersection(
    sourceNode.center,
    sourceNode.width,
    sourceNode.height,
    targetNode.center,
    edgePadding
  );

  const targetPoint = getNodeIntersection(
    targetNode.center,
    targetNode.width,
    targetNode.height,
    sourceNode.center,
    edgePadding
  );

  // Generate curved path
  const dx = targetPoint.x - sourcePoint.x;
  const dy = targetPoint.y - sourcePoint.y;
  const distance = Math.sqrt(dx * dx + dy * dy);
  const curvature = Math.min(distance * 0.3, 100);

  const midX = (sourcePoint.x + targetPoint.x) / 2;
  const midY = (sourcePoint.y + targetPoint.y) / 2;

  const perpX = -dy / distance;
  const perpY = dx / distance;

  const controlX = midX + perpX * curvature;
  const controlY = midY + perpY * curvature;

  const edgePath = `M${sourcePoint.x},${sourcePoint.y} Q${controlX},${controlY} ${targetPoint.x},${targetPoint.y}`;
  const reversedPath = `M${targetPoint.x},${targetPoint.y} Q${controlX},${controlY} ${sourcePoint.x},${sourcePoint.y}`;

  self.postMessage({
    edgePath,
    reversedPath,
    edgeLength: distance,
    sourcePoint,
    targetPoint,
  });
};

function getNodeIntersection(nodeCenter, nodeWidth, nodeHeight, targetCenter, padding) {
  const dx = targetCenter.x - nodeCenter.x;
  const dy = targetCenter.y - nodeCenter.y;

  const halfWidth = nodeWidth / 2 + padding;
  const halfHeight = nodeHeight / 2 + padding;

  const absDx = Math.abs(dx);
  const absDy = Math.abs(dy);

  let intersectX, intersectY;

  if (absDx * halfHeight > absDy * halfWidth) {
    const ratio = halfWidth / absDx;
    intersectX = nodeCenter.x + dx * ratio;
    intersectY = nodeCenter.y + dy * ratio;
  } else {
    const ratio = halfHeight / absDy;
    intersectX = nodeCenter.x + dx * ratio;
    intersectY = nodeCenter.y + dy * ratio;
  }

  return { x: intersectX, y: intersectY };
}
```

**Step 2: Create hook**

```typescript
// src/hooks/usePathWorker.ts
import { useEffect, useRef, useState, useCallback } from 'react';

interface PathWorkerInput {
  sourceNode: { center: { x: number; y: number }; width: number; height: number };
  targetNode: { center: { x: number; y: number }; width: number; height: number };
  edgePadding?: number;
}

interface PathWorkerOutput {
  edgePath: string;
  reversedPath: string;
  edgeLength: number;
  sourcePoint: { x: number; y: number };
  targetPoint: { x: number; y: number };
}

export function usePathWorker() {
  const workerRef = useRef<Worker | null>(null);
  const [result, setResult] = useState<PathWorkerOutput | null>(null);
  const pendingRef = useRef<((value: PathWorkerOutput) => void) | null>(null);

  useEffect(() => {
    workerRef.current = new Worker('/workers/pathWorker.js');

    workerRef.current.onmessage = (e: MessageEvent<PathWorkerOutput>) => {
      setResult(e.data);
      pendingRef.current?.(e.data);
      pendingRef.current = null;
    };

    return () => {
      workerRef.current?.terminate();
    };
  }, []);

  const calculatePath = useCallback((input: PathWorkerInput): Promise<PathWorkerOutput> => {
    return new Promise((resolve) => {
      pendingRef.current = resolve;
      workerRef.current?.postMessage(input);
    });
  }, []);

  return { calculatePath, result };
}
```

**Step 3: Commit**

```bash
git add public/workers/pathWorker.js src/hooks/usePathWorker.ts
git commit -m "feat(perf): add Web Worker for edge path calculations"
```

---

### Task 6.2: Add Debounced Batch Updates

**Files:**
- Create: `src/hooks/useDebouncedBatch.ts`
- Test: `src/hooks/__tests__/useDebouncedBatch.test.ts`

**Step 1: Write the failing test**

```typescript
// src/hooks/__tests__/useDebouncedBatch.test.ts
import { describe, it, expect, vi } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useDebouncedBatch } from '../useDebouncedBatch';

describe('useDebouncedBatch', () => {
  vi.useFakeTimers();

  it('should batch multiple updates', () => {
    const callback = vi.fn();
    const { result } = renderHook(() => useDebouncedBatch(callback, 16));

    act(() => {
      result.current.add({ id: '1', x: 10 });
      result.current.add({ id: '2', x: 20 });
      result.current.add({ id: '3', x: 30 });
    });

    expect(callback).not.toHaveBeenCalled();

    act(() => {
      vi.advanceTimersByTime(20);
    });

    expect(callback).toHaveBeenCalledTimes(1);
    expect(callback).toHaveBeenCalledWith([
      { id: '1', x: 10 },
      { id: '2', x: 20 },
      { id: '3', x: 30 },
    ]);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- src/hooks/__tests__/useDebouncedBatch.test.ts`
Expected: FAIL (module not found)

**Step 3: Implement useDebouncedBatch**

```typescript
// src/hooks/useDebouncedBatch.ts
import { useRef, useCallback, useEffect } from 'react';

export function useDebouncedBatch<T>(
  callback: (items: T[]) => void,
  delay: number = 16 // One frame at 60fps
) {
  const batchRef = useRef<T[]>([]);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const callbackRef = useRef(callback);

  // Keep callback ref up to date
  useEffect(() => {
    callbackRef.current = callback;
  }, [callback]);

  const add = useCallback((item: T) => {
    batchRef.current.push(item);

    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = setTimeout(() => {
      if (batchRef.current.length > 0) {
        callbackRef.current([...batchRef.current]);
        batchRef.current = [];
      }
      timeoutRef.current = null;
    }, delay);
  }, [delay]);

  const flush = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    if (batchRef.current.length > 0) {
      callbackRef.current([...batchRef.current]);
      batchRef.current = [];
    }
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  return { add, flush };
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- src/hooks/__tests__/useDebouncedBatch.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/hooks/useDebouncedBatch.ts src/hooks/__tests__/useDebouncedBatch.test.ts
git commit -m "feat(perf): add useDebouncedBatch hook for batched updates"
```

---

## Summary

### Expected Results

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Active animations | 437,000 | <1,000 | 99.8% |
| Memory usage | ~900 MB | ~50 MB | 94% |
| Blur operations/sec | 20.5M | ~100k | 99.5% |
| Object allocations/frame | 247,000 | ~5,000 | 98% |
| FPS | ~5 fps | 60 fps | 12x |

### Implementation Order

1. **Phase 1** (Day 1-2): Critical fixes - immediate impact
2. **Phase 2** (Day 3-4): Viewport culling - major gain
3. **Phase 3** (Day 5-6): LOD system - zoom optimization
4. **Phase 4** (Day 7-8): Node optimization - allocation reduction
5. **Phase 5** (Day 9-10): Shared resources - complete the fix
6. **Phase 6** (Day 11-14): Advanced optimizations - polish

### Testing Strategy

Each task includes:
- Unit tests for new modules
- Performance tests verifying improvements
- Integration tests for system behavior

Run full test suite after each phase:
```bash
npm test
npm run build
```

### Rollback Plan

Each phase is independent. If issues arise:
1. Revert phase-specific commits
2. Previous phases remain functional
3. Feature flags can disable optimizations
