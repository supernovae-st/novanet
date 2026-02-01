# Matrix Transition: Data ↔ Schema Mode

**Date:** 2026-01-31
**Status:** Approved

## Overview

Smooth Matrix-style transition animation when switching between Data and Schema modes.

## Design Decisions

| Aspect | Choice |
|--------|--------|
| **Scope** | Stats Pill glow + Full canvas dissolve |
| **Style** | Matrix rain (800-1200ms total) |
| **Trigger** | ViewPicker → animationStore → fetch |
| **Tech** | tsparticles with WebGL renderer |

## State Flow

```
1. User clicks Data/Schema in ViewPicker
2. animationStore.startTransition('data' | 'schema')
3. Phase 1 - DISSOLVE (400ms):
   ├── Pill: glow=true, glowColor="novanet"
   ├── Canvas: MatrixRainOverlay visible, opacity 0→1
   └── Graph: nodes fade out (opacity 1→0)
4. Phase 2 - FETCH (variable):
   ├── fetchSchemaData() or fetchData() in background
   └── Matrix rain continues falling
5. Phase 3 - REFORM (400ms):
   ├── Canvas: MatrixRainOverlay opacity 1→0
   ├── Graph: new nodes fade in (opacity 0→1)
   └── Pill: glow=false
6. animationStore.endTransition()
```

## Components

### New Components

| Component | Purpose |
|-----------|---------|
| `MatrixRainOverlay.tsx` | tsparticles canvas overlay with Matrix rain effect |
| `animationStore.ts` | Zustand store for transition state |

### Modified Files

| File | Changes |
|------|---------|
| `ViewPicker.tsx` | onClick → animationStore.startTransition() |
| `page.tsx` | Add MatrixRainOverlay, wire Pill glow to isTransitioning |
| `Graph2D.tsx` | Add opacity transition based on phase |

## Animation Store

```typescript
interface AnimationState {
  isTransitioning: boolean;
  transitionPhase: 'dissolve' | 'fetch' | 'reform' | null;
  targetMode: 'data' | 'schema' | null;
  startTransition: (mode: 'data' | 'schema') => void;
  setPhase: (phase: 'dissolve' | 'fetch' | 'reform' | null) => void;
  endTransition: () => void;
}
```

## tsparticles Config

```typescript
{
  particles: {
    number: { value: 100, density: { enable: true, area: 800 } },
    color: { value: ["#10b981", "#34d399", "#6ee7b7"] },
    shape: {
      type: "char",
      character: {
        value: "アイウエオカキクケコサシスセソ0123456789".split(""),
        font: "monospace",
        weight: "400"
      }
    },
    opacity: { value: { min: 0.1, max: 0.8 }, animation: { enable: true } },
    size: { value: { min: 8, max: 16 } },
    move: {
      direction: "bottom",
      speed: { min: 5, max: 15 },
      straight: true,
      outModes: "out"
    }
  }
}
```

## Timing

```
0ms      400ms              fetch done        +400ms
 │────────│───────────────────│─────────────────│
 │DISSOLVE│      FETCH        │     REFORM      │
 │        │                   │                 │
 ▼        ▼                   ▼                 ▼
start   graph=0%           data ready      transition end
rain    rain=100%          begin reform    rain=0%, graph=100%
```

## Dependencies

```bash
pnpm add @tsparticles/react @tsparticles/slim --filter=@novanet/studio
```

Bundle impact: ~50KB gzipped

## Implementation Plan

1. Install tsparticles dependencies
2. Create animationStore.ts
3. Create MatrixRainOverlay.tsx component
4. Update ViewPicker.tsx to use animationStore
5. Update page.tsx to orchestrate the transition
6. Update Graph2D.tsx with opacity transitions
7. Test both directions (Data→Schema, Schema→Data)
