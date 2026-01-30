# Edge Animation System - Design Document

**Date:** 2026-01-30
**Status:** Approved
**Migration:** Big Bang refactor

---

## Overview

Complete redesign of the edge animation system for NovaNet Studio. Goals:
- **Type Safety:** Discriminated unions, zero magic strings
- **Modularity:** Composable effect primitives
- **Scalability:** 19k edges @ 60fps via LOD + budget + pooling
- **Extensibility:** Easy to add new effects and relation variants

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      FloatingEdge.tsx                           │
│                    (Simplified Orchestrator)                    │
└─────────────────────────────┬───────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌───────────────┐   ┌─────────────────┐   ┌─────────────────┐
│    system/    │   │    effects/     │   │  performance/   │
│  Theme Resolver│   │ EffectRenderer │   │  LOD + Budget   │
└───────┬───────┘   └────────┬────────┘   └────────┬────────┘
        │                    │                     │
        ▼                    ▼                     ▼
┌───────────────┐   ┌─────────────────┐   ┌─────────────────┐
│ Category Theme│   │   Primitives    │   │  Effect Pool    │
│ + Overrides   │   │ Emit│Trail│...  │   │  (Lazy + Pool)  │
└───────────────┘   └─────────────────┘   └─────────────────┘
```

---

## 1. Type System (`system/types.ts`)

### Relation Categories

```typescript
/**
 * Neo4j relation categories - maps to visual themes
 */
export type RelationCategory =
  | 'structural'    // HAS_*, CONTAINS
  | 'localization'  // FOR_LOCALE, SUPPORTS, HAS_L10N
  | 'generation'    // OUTPUT, GENERATED, HAS_PROMPT, HAS_RULES
  | 'semantic'      // USES_CONCEPT, SEMANTIC
  | 'seo'           // TARGETS_SEO, HAS_VARIATION
  | 'geo'           // TARGETS_GEO, HAS_CITATION
  | 'reference';    // USES, MODEL, PROVIDER

/**
 * All known relation types (from Neo4j schema)
 */
export type RelationType =
  | 'HAS_CONCEPT' | 'HAS_PAGE' | 'HAS_BLOCK' | 'HAS_AUDIENCE'
  | 'HAS_L10N' | 'HAS_OUTPUT' | 'HAS_PROMPT' | 'HAS_RULES'
  | 'SUPPORTS_LOCALE' | 'FOR_LOCALE'
  | 'USES_CONCEPT' | 'OF_TYPE'
  | 'TARGETS_SEO' | 'TARGETS_GEO'
  | 'HAS_VARIATION' | 'HAS_CITATION'
  | 'USES' | 'FALLBACK_TO';
```

### Effect Primitives

```typescript
/**
 * Atomic effect building blocks
 */
export type EffectPrimitive =
  | 'emit'          // Pulse/burst at source
  | 'particles'     // Traveling data packets
  | 'trail'         // Comet tail behind particles
  | 'impact'        // Burst at target
  | 'glow'          // Edge glow layer
  | 'zigzag'        // Neural branching sparks
  | 'interference'  // Wave overlay pattern
  | 'scanline';     // Holographic scan effect

/**
 * Animation speed presets
 */
export type AnimationSpeed = 'slow' | 'normal' | 'fast' | 'ultra';

/**
 * Line style variants
 */
export type LineStyle = 'solid' | 'dashed' | 'dotted' | 'double' | 'triple' | 'zigzag';

/**
 * Particle animation presets (built from primitives)
 */
export type ParticlePreset =
  | 'flow'    // Simple directional flow
  | 'pulse'   // Pulsing rings
  | 'wave'    // Dense wave of particles
  | 'spark'   // Explosive sparks
  | 'plasma'  // Electric plasma discharge
  | 'helix'   // DNA double helix
  | 'orbit'   // Orbiting satellites
  | 'aurora'; // Color-shifting aurora
```

### Configuration Interfaces

```typescript
/**
 * Color palette for an edge theme
 */
export interface ColorPalette {
  primary: string;
  secondary: string;
  tertiary: string;
  glow: string;
}

/**
 * Timing configuration
 */
export interface TimingConfig {
  duration: number;      // Base duration in seconds
  speed: AnimationSpeed;
  stagger: number;       // Delay between particles
  easing: string;        // CSS easing function
}

/**
 * Complete edge theme configuration
 */
export interface EdgeTheme {
  palette: ColorPalette;
  effects: EffectPrimitive[];
  lineStyle: LineStyle;
  strokeWidth: number;
  particlePreset: ParticlePreset;
  speed: AnimationSpeed;
  glowIntensity: number;
}

/**
 * Resolved theme (after merging category + overrides)
 */
export interface ResolvedEdgeTheme extends EdgeTheme {
  category: RelationCategory;
  relationType: RelationType;
}
```

---

## 2. Theme System (`system/themes.ts`)

### Color Palettes

```typescript
export const PALETTES: Record<RelationCategory, ColorPalette> = {
  structural: {
    primary: '#5E6AD2',
    secondary: '#3b82f6',
    tertiary: '#06b6d4',
    glow: '#5E6AD2',
  },
  localization: {
    primary: '#10b981',
    secondary: '#22c55e',
    tertiary: '#6ee7b7',
    glow: '#10b981',
  },
  generation: {
    primary: '#F2994A',
    secondary: '#f97316',
    tertiary: '#fbbf24',
    glow: '#F2994A',
  },
  semantic: {
    primary: '#9B51E0',
    secondary: '#8b5cf6',
    tertiary: '#a78bfa',
    glow: '#9B51E0',
  },
  seo: {
    primary: '#ec4899',
    secondary: '#f472b6',
    tertiary: '#f9a8d4',
    glow: '#ec4899',
  },
  geo: {
    primary: '#06b6d4',
    secondary: '#22d3ee',
    tertiary: '#67e8f9',
    glow: '#06b6d4',
  },
  reference: {
    primary: '#14b8a6',
    secondary: '#2dd4bf',
    tertiary: '#5eead4',
    glow: '#14b8a6',
  },
};
```

### Category Base Themes

```typescript
export const CATEGORY_THEMES: Record<RelationCategory, EdgeTheme> = {
  structural: {
    palette: PALETTES.structural,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow'],
    lineStyle: 'solid',
    strokeWidth: 3,
    particlePreset: 'plasma',
    speed: 'normal',
    glowIntensity: 0.8,
  },
  localization: {
    palette: PALETTES.localization,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow', 'interference'],
    lineStyle: 'double',
    strokeWidth: 2.5,
    particlePreset: 'helix',
    speed: 'slow',
    glowIntensity: 0.75,
  },
  generation: {
    palette: PALETTES.generation,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow', 'scanline'],
    lineStyle: 'triple',
    strokeWidth: 4,
    particlePreset: 'spark',
    speed: 'fast',
    glowIntensity: 0.85,
  },
  semantic: {
    palette: PALETTES.semantic,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow', 'interference'],
    lineStyle: 'dotted',
    strokeWidth: 3,
    particlePreset: 'aurora',
    speed: 'slow',
    glowIntensity: 0.75,
  },
  seo: {
    palette: PALETTES.seo,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow'],
    lineStyle: 'solid',
    strokeWidth: 3.5,
    particlePreset: 'wave',
    speed: 'normal',
    glowIntensity: 0.75,
  },
  geo: {
    palette: PALETTES.geo,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow', 'scanline'],
    lineStyle: 'zigzag',
    strokeWidth: 2.5,
    particlePreset: 'orbit',
    speed: 'normal',
    glowIntensity: 0.7,
  },
  reference: {
    palette: PALETTES.reference,
    effects: ['emit', 'particles', 'trail', 'impact', 'glow'],
    lineStyle: 'solid',
    strokeWidth: 3,
    particlePreset: 'flow',
    speed: 'normal',
    glowIntensity: 0.8,
  },
};
```

### Relation Overrides

```typescript
/**
 * Sparse overrides - only specify what differs from category base
 */
export const RELATION_OVERRIDES: Partial<Record<RelationType, Partial<EdgeTheme>>> = {
  // Structural overrides
  HAS_CONCEPT: {
    effects: ['emit', 'particles', 'trail', 'impact', 'glow', 'zigzag'],
    speed: 'fast',
  },
  HAS_PAGE: {
    strokeWidth: 4,
  },

  // Localization overrides
  SUPPORTS_LOCALE: {
    particlePreset: 'helix',
    glowIntensity: 0.9,
  },
  HAS_L10N: {
    speed: 'normal',
  },

  // Generation overrides
  HAS_OUTPUT: {
    effects: ['emit', 'particles', 'trail', 'impact', 'glow', 'scanline', 'zigzag'],
    speed: 'ultra',
  },
  HAS_PROMPT: {
    lineStyle: 'zigzag',
    particlePreset: 'orbit',
  },

  // Add more as needed...
};
```

### Theme Resolver

```typescript
/**
 * Map relation type to category
 */
export function getCategory(relationType: RelationType): RelationCategory {
  if (relationType.startsWith('HAS_') || relationType === 'CONTAINS') return 'structural';
  if (relationType.includes('LOCALE') || relationType === 'SUPPORTS_LOCALE') return 'localization';
  if (relationType.includes('OUTPUT') || relationType.includes('PROMPT') || relationType.includes('RULES')) return 'generation';
  if (relationType.includes('CONCEPT') || relationType === 'SEMANTIC') return 'semantic';
  if (relationType.includes('SEO')) return 'seo';
  if (relationType.includes('GEO')) return 'geo';
  return 'reference';
}

/**
 * Resolve complete theme for a relation type
 */
export function resolveTheme(relationType: RelationType): ResolvedEdgeTheme {
  const category = getCategory(relationType);
  const base = CATEGORY_THEMES[category];
  const override = RELATION_OVERRIDES[relationType] ?? {};

  return {
    ...base,
    ...override,
    palette: { ...base.palette, ...override.palette },
    effects: override.effects ?? base.effects,
    category,
    relationType,
  };
}
```

---

## 3. Effect Primitives (`effects/primitives/`)

### Primitive Interface

```typescript
/**
 * Common props for all effect primitives
 */
export interface EffectPrimitiveProps {
  /** SVG path ID to animate along */
  pathId: string;
  /** Color palette */
  colors: ColorPalette;
  /** Timing configuration */
  timing: TimingConfig;
  /** Intensity 0-1 (controlled by LOD) */
  intensity: number;
  /** Current state */
  state: 'idle' | 'active' | 'highlighted';
  /** Source position */
  sourcePosition: { x: number; y: number };
  /** Target position */
  targetPosition: { x: number; y: number };
}
```

### Example: EmitPrimitive

```typescript
// effects/primitives/EmitPrimitive.tsx
export const EmitPrimitive = memo(function EmitPrimitive({
  colors,
  timing,
  intensity,
  state,
  sourcePosition,
}: EffectPrimitiveProps) {
  const scale = state === 'highlighted' ? 1.5 : 1;
  const baseSize = 8 * intensity * scale;

  return (
    <g className="emit-primitive">
      {/* Outer pulse ring */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize * 2}
        fill="none"
        stroke={colors.glow}
        strokeWidth={2}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${baseSize};${baseSize * 4};${baseSize}`}
          dur={`${timing.duration}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values="0.6;0;0.6"
          dur={`${timing.duration}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Core glow */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize}
        fill={colors.primary}
        opacity={0.8 * intensity}
      >
        <animate
          attributeName="r"
          values={`${baseSize * 0.8};${baseSize * 1.2};${baseSize * 0.8}`}
          dur={`${timing.duration * 0.5}s`}
          repeatCount="indefinite"
        />
      </circle>
    </g>
  );
});
```

### Primitive Registry

```typescript
// effects/primitives/index.ts
import { EmitPrimitive } from './EmitPrimitive';
import { ParticlesPrimitive } from './ParticlesPrimitive';
import { TrailPrimitive } from './TrailPrimitive';
import { ImpactPrimitive } from './ImpactPrimitive';
import { GlowPrimitive } from './GlowPrimitive';
import { ZigzagPrimitive } from './ZigzagPrimitive';
import { InterferencePrimitive } from './InterferencePrimitive';
import { ScanlinePrimitive } from './ScanlinePrimitive';

export const PRIMITIVE_REGISTRY: Record<EffectPrimitive, React.ComponentType<EffectPrimitiveProps>> = {
  emit: EmitPrimitive,
  particles: ParticlesPrimitive,
  trail: TrailPrimitive,
  impact: ImpactPrimitive,
  glow: GlowPrimitive,
  zigzag: ZigzagPrimitive,
  interference: InterferencePrimitive,
  scanline: ScanlinePrimitive,
};
```

---

## 4. Effect Renderer (`effects/EffectRenderer.tsx`)

```typescript
interface EffectRendererProps extends Omit<EffectPrimitiveProps, 'pathId'> {
  /** Effect stack to render */
  effects: EffectPrimitive[];
  /** Path IDs */
  pathId: string;
  reversedPathId: string;
}

export const EffectRenderer = memo(function EffectRenderer({
  effects,
  pathId,
  reversedPathId,
  ...primitiveProps
}: EffectRendererProps) {
  // Filter effects based on intensity (LOD)
  const activeEffects = useMemo(() => {
    if (primitiveProps.intensity < 0.3) {
      // Minimal - only glow
      return effects.filter(e => e === 'glow');
    }
    if (primitiveProps.intensity < 0.6) {
      // Reduced - core effects only
      return effects.filter(e => ['emit', 'particles', 'impact', 'glow'].includes(e));
    }
    // Full
    return effects;
  }, [effects, primitiveProps.intensity]);

  return (
    <g className="effect-renderer">
      {activeEffects.map((primitive) => {
        const Component = PRIMITIVE_REGISTRY[primitive];
        const effectPathId = primitive === 'trail' ? reversedPathId : pathId;

        return (
          <Component
            key={primitive}
            pathId={effectPathId}
            {...primitiveProps}
          />
        );
      })}
    </g>
  );
});
```

---

## 5. Performance System (`system/performance/`)

### LOD Controller

```typescript
// system/performance/LODController.ts
export type LODTier = 'full' | 'reduced' | 'minimal' | 'static' | 'hidden';

export interface LODConfig {
  effects: EffectPrimitive[] | 'ALL' | 'CORE' | 'GLOW' | 'NONE';
  maxParticles: number;
  enableGlow: boolean;
  targetFPS: number;
}

export const LOD_CONFIGS: Record<LODTier, LODConfig> = {
  full:    { effects: 'ALL',  maxParticles: 6, enableGlow: true,  targetFPS: 60 },
  reduced: { effects: 'CORE', maxParticles: 3, enableGlow: true,  targetFPS: 30 },
  minimal: { effects: 'GLOW', maxParticles: 0, enableGlow: true,  targetFPS: 15 },
  static:  { effects: 'NONE', maxParticles: 0, enableGlow: false, targetFPS: 0  },
  hidden:  { effects: 'NONE', maxParticles: 0, enableGlow: false, targetFPS: 0  },
};

export function calculateLODTier(
  distance: number,
  zoom: number,
  isSelected: boolean,
  isHovered: boolean,
): LODTier {
  // Selected/hovered always full
  if (isSelected) return 'full';
  if (isHovered) return 'full';

  // Distance-based (adjusted for zoom)
  const effectiveDistance = distance / zoom;

  if (effectiveDistance < 200) return 'full';
  if (effectiveDistance < 500) return 'reduced';
  if (effectiveDistance < 1000) return 'minimal';
  if (effectiveDistance < 2000) return 'static';
  return 'hidden';
}
```

### Animation Budget

```typescript
// system/performance/AnimationBudget.ts
export interface AnimationBudgetConfig {
  maxConcurrent: number;
  priorities: Record<EdgeState, number>;
}

export const DEFAULT_BUDGET: AnimationBudgetConfig = {
  maxConcurrent: 50,
  priorities: {
    selected: 100,
    hovered: 80,
    connected: 60,
    visible: 40,
    offscreen: 0,
  },
};

export class AnimationBudgetManager {
  private config: AnimationBudgetConfig;
  private activeEdges: Map<string, number> = new Map(); // edgeId -> priority

  constructor(config = DEFAULT_BUDGET) {
    this.config = config;
  }

  canAnimate(edgeId: string, state: EdgeState): boolean {
    const priority = this.config.priorities[state];

    // Always allow high priority
    if (priority >= 80) return true;

    // Check budget
    if (this.activeEdges.size < this.config.maxConcurrent) {
      return true;
    }

    // Check if we can evict a lower priority edge
    const lowestActive = Math.min(...this.activeEdges.values());
    return priority > lowestActive;
  }

  register(edgeId: string, priority: number): void {
    this.activeEdges.set(edgeId, priority);
    this.evictIfNeeded();
  }

  unregister(edgeId: string): void {
    this.activeEdges.delete(edgeId);
  }

  private evictIfNeeded(): void {
    while (this.activeEdges.size > this.config.maxConcurrent) {
      // Evict lowest priority
      let lowestId: string | null = null;
      let lowestPriority = Infinity;

      for (const [id, priority] of this.activeEdges) {
        if (priority < lowestPriority) {
          lowestPriority = priority;
          lowestId = id;
        }
      }

      if (lowestId) {
        this.activeEdges.delete(lowestId);
      }
    }
  }
}
```

### Lazy Effect Pool

```typescript
// system/performance/LazyEffectPool.ts
export class EffectPool {
  private pools: Map<EffectPrimitive, SVGElement[]> = new Map();
  private active: Map<string, Set<SVGElement>> = new Map();

  constructor() {
    // Initialize empty pools for each primitive type
    for (const primitive of Object.keys(PRIMITIVE_REGISTRY)) {
      this.pools.set(primitive as EffectPrimitive, []);
    }
  }

  /**
   * Pre-warm pools for smooth initial render
   */
  prewarm(counts: Partial<Record<EffectPrimitive, number>>): void {
    for (const [primitive, count] of Object.entries(counts)) {
      const pool = this.pools.get(primitive as EffectPrimitive)!;
      while (pool.length < count) {
        pool.push(this.createElement(primitive as EffectPrimitive));
      }
    }
  }

  /**
   * Acquire an effect element (from pool or create new)
   */
  acquire(edgeId: string, primitive: EffectPrimitive): SVGElement {
    const pool = this.pools.get(primitive)!;
    const element = pool.pop() ?? this.createElement(primitive);

    if (!this.active.has(edgeId)) {
      this.active.set(edgeId, new Set());
    }
    this.active.get(edgeId)!.add(element);

    return element;
  }

  /**
   * Release all effects for an edge back to pool
   */
  release(edgeId: string): void {
    const elements = this.active.get(edgeId);
    if (!elements) return;

    for (const element of elements) {
      const primitive = element.dataset.primitive as EffectPrimitive;
      this.pools.get(primitive)!.push(element);
    }

    this.active.delete(edgeId);
  }

  private createElement(primitive: EffectPrimitive): SVGElement {
    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.dataset.primitive = primitive;
    return g;
  }
}
```

---

## 6. File Structure

```
src/components/graph/edges/
├── system/                          # Core type-safe system
│   ├── types.ts                    # Discriminated unions, interfaces
│   ├── themes.ts                   # Palettes, category themes, overrides
│   ├── registry.ts                 # Theme resolver, relation→category mapping
│   ├── constants.ts                # Timing, sizing constants
│   └── performance/
│       ├── LODController.ts        # Distance-based detail levels
│       ├── AnimationBudget.ts      # Priority queue, max concurrent
│       ├── LazyEffectPool.ts       # Object pooling
│       ├── PerformanceMonitor.ts   # FPS tracking, auto-throttle
│       └── index.ts
│
├── effects/                         # Modular effect system
│   ├── primitives/
│   │   ├── EmitPrimitive.tsx
│   │   ├── ParticlesPrimitive.tsx
│   │   ├── TrailPrimitive.tsx
│   │   ├── ImpactPrimitive.tsx
│   │   ├── GlowPrimitive.tsx
│   │   ├── ZigzagPrimitive.tsx
│   │   ├── InterferencePrimitive.tsx
│   │   ├── ScanlinePrimitive.tsx
│   │   └── index.ts                # PRIMITIVE_REGISTRY
│   │
│   ├── presets/
│   │   ├── PlasmaPreset.ts
│   │   ├── NeuralPreset.ts
│   │   ├── AuroraPreset.ts
│   │   ├── FlowPreset.ts
│   │   └── index.ts                # PRESET_REGISTRY
│   │
│   └── EffectRenderer.tsx          # Stack orchestrator
│
├── components/                      # Edge UI components
│   ├── EdgePath.tsx                # SVG path rendering
│   ├── EdgeLabel.tsx               # Curved text label
│   ├── EdgeMarkers.tsx             # Orb markers
│   └── EdgeGlow.tsx                # Glow layers
│
├── hooks/                           # Edge-specific hooks
│   ├── useEdgeTheme.ts             # Theme resolution
│   ├── useEdgeLOD.ts               # LOD calculation
│   ├── useAnimationBudget.ts       # Budget management
│   └── useEffectPool.ts            # Pool access
│
├── FloatingEdge.tsx                 # Simplified orchestrator (~200 LOC)
├── SharedSVGDefs.tsx                # Shared gradients/filters
├── index.ts                         # Public API
│
└── __tests__/                       # Tests
    ├── themes.test.ts
    ├── primitives.test.tsx
    ├── performance.test.ts
    └── integration.test.tsx
```

---

## 7. Migration Plan (Big Bang)

### Step 1: Create Type System
- [ ] Create `system/types.ts` with all discriminated unions
- [ ] Create `system/constants.ts` with timing/sizing values

### Step 2: Create Theme System
- [ ] Create `system/themes.ts` with palettes and category themes
- [ ] Create `system/registry.ts` with resolver and overrides

### Step 3: Extract Primitives
- [ ] Extract `EmitPrimitive` from `EmitEffect.tsx`
- [ ] Extract `ParticlesPrimitive` from `EnergyEffects.tsx`
- [ ] Extract `TrailPrimitive` from `EnergyEffects.tsx`
- [ ] Extract `ImpactPrimitive` from `ImpactEffect.tsx`
- [ ] Extract `GlowPrimitive` from `EdgeGlow.tsx`
- [ ] Extract `ZigzagPrimitive` from `NeuralZigzagEffect.tsx`
- [ ] Extract `InterferencePrimitive` from `EdgeAnimations.tsx`
- [ ] Extract `ScanlinePrimitive` from `EdgeAnimations.tsx`
- [ ] Create `PRIMITIVE_REGISTRY`

### Step 4: Create Presets
- [ ] Create preset compositions from primitives
- [ ] Create `PRESET_REGISTRY`

### Step 5: Create Performance System
- [ ] Create `LODController.ts`
- [ ] Create `AnimationBudget.ts`
- [ ] Create `LazyEffectPool.ts`
- [ ] Create hooks: `useEdgeLOD`, `useAnimationBudget`, `useEffectPool`

### Step 6: Create EffectRenderer
- [ ] Create `EffectRenderer.tsx` that composes primitives
- [ ] Integrate with LOD system

### Step 7: Rewrite FloatingEdge
- [ ] Simplify to ~200 LOC orchestrator
- [ ] Use new theme system
- [ ] Use new effect system
- [ ] Use new performance system

### Step 8: Update Exports
- [ ] Update `index.ts` with new public API
- [ ] Ensure backward compatibility for external imports

### Step 9: Delete Legacy
- [ ] Remove `EdgeStyles.ts`
- [ ] Remove `EdgeAnimationConfig.ts`
- [ ] Remove `EdgeAnimations.tsx`
- [ ] Remove `EnergyEffects.tsx`
- [ ] Remove old effect files

### Step 10: Testing
- [ ] Run all existing tests
- [ ] Add new tests for theme resolution
- [ ] Add new tests for primitives
- [ ] Add performance benchmarks
- [ ] Visual regression testing

---

## Success Criteria

1. **Type Safety:** Zero `any` types, full IntelliSense support
2. **Performance:** 19k edges, 50+ animated, 60fps maintained
3. **Modularity:** Adding new effect = 1 file + registry entry
4. **Maintainability:** FloatingEdge.tsx < 250 LOC
5. **Extensibility:** New relation variant = 1 override entry
