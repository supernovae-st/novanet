# Arc Animation System v2 - Design Document

**Date**: 2026-02-10
**Version**: v11.6.1
**Author**: Claude + Thibaut
**Status**: Ready for Implementation

## Overview

Enhance NovaNet Studio's arc (edge) visualization with:
1. **Family-specific animations** - Each ArcFamily gets a unique visual identity
2. **Multi-edge bundling** - Prevent overlap when multiple arcs connect same nodes
3. **Performance optimization** - Canvas hybrid for high edge counts

## Current State

### Existing System (v11.6.0)
- 8 effect primitives: `emit`, `particles`, `trail`, `impact`, `glow`, `zigzag`, `interference`, `scanline`
- AnimationBudget: 50 concurrent max
- LOD Controller: 4 tiers (high/medium/low/minimal)
- EdgeVisibilityManager: IntersectionObserver culling
- Theme registry per ArcFamily

### Limitations
- All families use similar particle effects (no visual differentiation)
- Multiple arcs between same nodes overlap
- No DNA/Matrix/Radar style effects
- Limited to ~50-80 animated edges before performance degrades

## Design Decisions

### 1. ArcFamily → Animation Mapping

| ArcFamily | Color | Animation Style | Metaphor |
|-----------|-------|-----------------|----------|
| **ownership** | #3b82f6 (blue) | ⚡ Energy Pulse | Power flows to children |
| **localization** | #22c55e (green) | 🧬 DNA Helix | Content DNA adapts |
| **semantic** | #f97316 (orange) | 🔗 Neural Sparks | Synapses firing |
| **generation** | #8b5cf6 (purple) | 💻 Matrix Code | AI processing data |
| **mining** | #ec4899 (pink) | 📡 Radar Sweep | Scanning for intel |

### 2. Multi-Edge Bundling Strategy

**Hybrid approach:**
- **2-3 arcs**: Curved fan-out (each arc visible with offset)
- **4+ arcs**: Collapse into bundle with count badge `[N]`
- **Hover**: Fan-out animation to show all arcs

```
2-3 arcs:                    4+ arcs:
    ╭─────────────╮              ════[4]════
[A]─┼─────────────┼─[B]      [A]────────────[B]
    ╰─────────────╯              (hover to expand)
```

### 3. New Effect Primitives (4)

| Primitive | Description | Key Technique |
|-----------|-------------|---------------|
| `DNAHelixPrimitive` | Double spiral pulsing along arc | `animateMotion` + oscillating Y offset |
| `MatrixCodePrimitive` | Characters flowing along arc | `<text>` + `animateMotion` + char cycling |
| `RadarSweepPrimitive` | Gradient sweep like radar | Animated `linearGradient` stops |
| `EnergyPulsePrimitive` | Bright packets with glow trail | Enhanced `particles` + intense glow |

**Total: 12 primitives** (8 existing + 4 new)

---

## Implementation Plan

### Phase 1: New Effect Primitives (2-3 days)

#### Task 1.1: EnergyPulsePrimitive
**File**: `apps/studio/src/components/graph/edges/effects/primitives/EnergyPulsePrimitive.tsx`

```typescript
interface EnergyPulseConfig {
  pulseCount: number;      // 3-5 pulses
  pulseSize: number;       // Larger than particles (8-12px)
  glowIntensity: number;   // 0.8-1.0 (very bright)
  speed: 'fast';           // 1.5s duration
  trailLength: number;     // 4-6 trail segments
}
```

**Visual layers:**
1. Outer glow (blur 12px, opacity 0.4)
2. Middle glow (blur 6px, opacity 0.6)
3. Core pulse (solid, opacity 0.9)
4. White hot center (2px, opacity 1.0)
5. Trail segments (decreasing size/opacity)

**Animation:**
```svg
<circle r="10" fill="#3b82f6">
  <animateMotion dur="1.5s" repeatCount="indefinite" path={edgePath}>
    <mpath href="#path" />
  </animateMotion>
</circle>
```

#### Task 1.2: DNAHelixPrimitive
**File**: `apps/studio/src/components/graph/edges/effects/primitives/DNAHelixPrimitive.tsx`

```typescript
interface DNAHelixConfig {
  strandCount: 2;          // Always double helix
  nucleotidesPerStrand: 8; // Points per strand
  amplitude: number;       // Oscillation height (15-25px)
  frequency: number;       // Waves per edge length
  rotationSpeed: number;   // 3D rotation illusion
}
```

**Implementation:**
- Two parallel particle streams
- Oscillating Y offset using `animateTransform type="translate"`
- Phase offset between strands (180°)
- Connecting "rungs" between strands at intervals
- Opacity based on Z-position (3D depth illusion)

```tsx
// Strand 1: sin wave
const strand1Y = amplitude * Math.sin(phase);
// Strand 2: opposite phase
const strand2Y = amplitude * Math.sin(phase + Math.PI);
// Z-depth for opacity
const zDepth = Math.cos(phase);
const opacity = 0.5 + zDepth * 0.5;
```

#### Task 1.3: MatrixCodePrimitive
**File**: `apps/studio/src/components/graph/edges/effects/primitives/MatrixCodePrimitive.tsx`

```typescript
interface MatrixCodeConfig {
  charSet: string;         // 'アイウエオ01<>{}[]'
  charCount: number;       // 6-10 characters
  charSize: number;        // 10-12px font
  cycleSpeed: number;      // Character change interval (100ms)
  flowSpeed: number;       // Movement along path (2-3s)
  glowColor: string;       // #22c55e (matrix green) or family color
}
```

**Implementation:**
- Use `<text>` elements with `animateMotion`
- JavaScript interval to cycle characters randomly
- Staggered start times for wave effect
- Glow filter: `drop-shadow(0 0 4px ${color})`

```tsx
const MATRIX_CHARS = 'アイウエオカキクケコ01<>{}[]═══';

// Cycle character every 100ms
useEffect(() => {
  const interval = setInterval(() => {
    setChar(MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]);
  }, 100);
  return () => clearInterval(interval);
}, []);
```

#### Task 1.4: RadarSweepPrimitive
**File**: `apps/studio/src/components/graph/edges/effects/primitives/RadarSweepPrimitive.tsx`

```typescript
interface RadarSweepConfig {
  sweepWidth: number;      // Gradient width (20% of path)
  sweepSpeed: number;      // 2-3s per sweep
  fadeLength: number;      // Tail fade length
  pulseOnComplete: boolean; // Emit pulse at end
}
```

**Implementation:**
- Animated `linearGradient` with moving stops
- Gradient: transparent → color → bright → color → transparent
- Animate `offset` attribute from -0.3 to 1.3

```tsx
<linearGradient id={`radar-${id}`}>
  <stop offset="0%" stopOpacity="0">
    <animate attributeName="offset" values="-0.2;1" dur="2s" repeatCount="indefinite" />
  </stop>
  <stop offset="10%" stopColor={color} stopOpacity="0.8">
    <animate attributeName="offset" values="-0.1;1.1" dur="2s" repeatCount="indefinite" />
  </stop>
  <stop offset="20%" stopColor="#ffffff" stopOpacity="1">
    <animate attributeName="offset" values="0;1.2" dur="2s" repeatCount="indefinite" />
  </stop>
  <stop offset="30%" stopColor={color} stopOpacity="0.6">
    <animate attributeName="offset" values="0.1;1.3" dur="2s" repeatCount="indefinite" />
  </stop>
  <stop offset="40%" stopOpacity="0">
    <animate attributeName="offset" values="0.2;1.4" dur="2s" repeatCount="indefinite" />
  </stop>
</linearGradient>
```

#### Task 1.5: Update Theme Registry
**File**: `apps/studio/src/components/graph/edges/system/themes.ts`

```typescript
export const ARC_FAMILY_THEMES: Record<ArcFamily, ArcFamilyTheme> = {
  ownership: {
    effects: ['energyPulse', 'glow'],
    primaryColor: '#3b82f6',
    speed: 'fast',
    intensity: 0.9,
  },
  localization: {
    effects: ['dnaHelix', 'glow'],
    primaryColor: '#22c55e',
    speed: 'medium',
    intensity: 0.8,
  },
  semantic: {
    effects: ['particles', 'zigzag', 'glow'],  // Enhanced existing
    primaryColor: '#f97316',
    speed: 'medium',
    intensity: 0.75,
  },
  generation: {
    effects: ['matrixCode', 'scanline', 'glow'],
    primaryColor: '#8b5cf6',
    speed: 'medium',
    intensity: 0.85,
  },
  mining: {
    effects: ['radarSweep', 'emit', 'impact'],
    primaryColor: '#ec4899',
    speed: 'slow',
    intensity: 0.7,
  },
};
```

#### Task 1.6: Update EffectRenderer
**File**: `apps/studio/src/components/graph/edges/effects/EffectRenderer.tsx`

Add new primitives to the effect map:
```typescript
const EFFECT_COMPONENTS: Record<EffectType, React.ComponentType<EffectPrimitiveProps>> = {
  // Existing
  emit: EmitPrimitive,
  particles: ParticlesPrimitive,
  trail: TrailPrimitive,
  impact: ImpactPrimitive,
  glow: GlowPrimitive,
  zigzag: ZigzagPrimitive,
  interference: InterferencePrimitive,
  scanline: ScanlinePrimitive,
  // New (Phase 1)
  energyPulse: EnergyPulsePrimitive,
  dnaHelix: DNAHelixPrimitive,
  matrixCode: MatrixCodePrimitive,
  radarSweep: RadarSweepPrimitive,
};
```

---

### Phase 2: Multi-Edge Bundling (1-2 days)

#### Task 2.1: Edge Grouping Logic
**File**: `apps/studio/src/hooks/useParallelEdges.ts`

```typescript
interface ParallelEdgeGroup {
  key: string;              // 'nodeA::nodeB' (sorted)
  edges: Edge[];
  count: number;
  isBundled: boolean;       // true if count >= 4
}

export function useParallelEdges(edges: Edge[]): Map<string, ParallelEdgeGroup> {
  return useMemo(() => {
    const groups = new Map<string, Edge[]>();

    for (const edge of edges) {
      // Canonical key (sorted to handle both directions)
      const key = [edge.source, edge.target].sort().join('::');
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key)!.push(edge);
    }

    return new Map(
      Array.from(groups.entries()).map(([key, groupEdges]) => [
        key,
        {
          key,
          edges: groupEdges,
          count: groupEdges.length,
          isBundled: groupEdges.length >= 4,
        },
      ])
    );
  }, [edges]);
}
```

#### Task 2.2: Curved Offset Path Generation
**File**: `apps/studio/src/components/graph/edges/EdgeUtils.ts`

```typescript
export function generateParallelPath(
  source: Point,
  target: Point,
  index: number,
  total: number
): string {
  if (total === 1) {
    return generateCurvedPath(source, target);
  }

  // Calculate perpendicular offset
  const dx = target.x - source.x;
  const dy = target.y - source.y;
  const length = Math.sqrt(dx * dx + dy * dy);

  // Perpendicular unit vector
  const perpX = -dy / length;
  const perpY = dx / length;

  // Offset from center (-1 to +1 normalized)
  const normalizedOffset = (index - (total - 1) / 2) / Math.max(total - 1, 1);
  const offsetPixels = normalizedOffset * 25 * Math.min(total, 3); // Max 75px spread

  // Control point with offset
  const midX = (source.x + target.x) / 2 + perpX * offsetPixels;
  const midY = (source.y + target.y) / 2 + perpY * offsetPixels;

  // Additional curve based on offset magnitude
  const curveOffset = Math.abs(offsetPixels) * 0.5;
  const curveX = midX + perpX * curveOffset;
  const curveY = midY + perpY * curveOffset;

  return `M ${source.x} ${source.y} Q ${curveX} ${curveY} ${target.x} ${target.y}`;
}
```

#### Task 2.3: Bundled Edge Component
**File**: `apps/studio/src/components/graph/edges/BundledEdge.tsx`

```typescript
interface BundledEdgeProps {
  edges: Edge[];
  source: Point;
  target: Point;
  isExpanded: boolean;
  onHover: (expanded: boolean) => void;
}

export function BundledEdge({ edges, source, target, isExpanded, onHover }: BundledEdgeProps) {
  const count = edges.length;

  if (isExpanded || count <= 3) {
    // Render individual edges with offset
    return (
      <g onMouseLeave={() => onHover(false)}>
        {edges.map((edge, i) => (
          <FloatingEdge
            key={edge.id}
            {...edge}
            path={generateParallelPath(source, target, i, count)}
          />
        ))}
      </g>
    );
  }

  // Render collapsed bundle
  const bundlePath = generateCurvedPath(source, target);
  const midPoint = getPathMidpoint(bundlePath);

  return (
    <g onMouseEnter={() => onHover(true)}>
      {/* Bundle line */}
      <path
        d={bundlePath}
        stroke="#64748b"
        strokeWidth={3 + count * 0.5}
        strokeDasharray="8 4"
        fill="none"
        opacity={0.6}
      />

      {/* Count badge */}
      <g transform={`translate(${midPoint.x}, ${midPoint.y})`}>
        <rect
          x={-12}
          y={-10}
          width={24}
          height={20}
          rx={4}
          fill="#1e293b"
          stroke="#64748b"
        />
        <text
          textAnchor="middle"
          dominantBaseline="middle"
          fill="#e2e8f0"
          fontSize={11}
          fontWeight="bold"
        >
          {count}
        </text>
      </g>
    </g>
  );
}
```

#### Task 2.4: Integration with Graph2D
**File**: `apps/studio/src/components/graph/Graph2D.tsx`

Update edge rendering to use bundling:
```typescript
const parallelEdgeGroups = useParallelEdges(edges);
const [expandedBundles, setExpandedBundles] = useState<Set<string>>(new Set());

// In render:
{Array.from(parallelEdgeGroups.values()).map((group) => {
  if (group.isBundled) {
    return (
      <BundledEdge
        key={group.key}
        edges={group.edges}
        isExpanded={expandedBundles.has(group.key)}
        onHover={(expanded) => {
          setExpandedBundles((prev) => {
            const next = new Set(prev);
            expanded ? next.add(group.key) : next.delete(group.key);
            return next;
          });
        }}
      />
    );
  }

  return group.edges.map((edge, i) => (
    <FloatingEdge
      key={edge.id}
      {...edge}
      parallelIndex={i}
      parallelTotal={group.count}
    />
  ));
})}
```

---

### Phase 3: Canvas Performance Optimization (Future)

#### Task 3.1: Canvas Overlay Layer
**File**: `apps/studio/src/components/graph/edges/CanvasEffectsLayer.tsx`

Architecture:
```
React Flow (SVG)
├── Edge paths (stroke only, no effects)
├── Labels
└── Hit areas

Canvas Overlay (positioned absolute)
├── All particle effects
├── Glow effects
└── Trail effects
```

```typescript
export function CanvasEffectsLayer({ edges, viewport }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number>(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');
    if (!ctx) return;

    const animate = (timestamp: number) => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      for (const edge of edges) {
        if (!edge.isVisible) continue;
        drawEdgeEffects(ctx, edge, timestamp, viewport);
      }

      animationRef.current = requestAnimationFrame(animate);
    };

    animationRef.current = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animationRef.current);
  }, [edges, viewport]);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 pointer-events-none"
      style={{ width: '100%', height: '100%' }}
    />
  );
}
```

#### Task 3.2: Particle System (Canvas)
```typescript
interface Particle {
  x: number;
  y: number;
  progress: number;  // 0-1 along path
  size: number;
  opacity: number;
  color: string;
}

function drawParticle(ctx: CanvasRenderingContext2D, p: Particle) {
  // Outer glow
  ctx.beginPath();
  ctx.arc(p.x, p.y, p.size * 2, 0, Math.PI * 2);
  ctx.fillStyle = `${p.color}40`;
  ctx.filter = 'blur(4px)';
  ctx.fill();

  // Core
  ctx.beginPath();
  ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
  ctx.fillStyle = p.color;
  ctx.filter = 'none';
  ctx.globalAlpha = p.opacity;
  ctx.fill();

  // Hot center
  ctx.beginPath();
  ctx.arc(p.x, p.y, p.size * 0.3, 0, Math.PI * 2);
  ctx.fillStyle = '#ffffff';
  ctx.fill();

  ctx.globalAlpha = 1;
}
```

#### Task 3.3: Path Interpolation (Canvas)
```typescript
// Pre-compute path points for Canvas animation
function pathToPoints(pathData: string, segments: number = 100): Point[] {
  const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
  path.setAttribute('d', pathData);

  const length = path.getTotalLength();
  const points: Point[] = [];

  for (let i = 0; i <= segments; i++) {
    const point = path.getPointAtLength((i / segments) * length);
    points.push({ x: point.x, y: point.y });
  }

  return points;
}

// Get position along path by progress (0-1)
function getPointAtProgress(points: Point[], progress: number): Point {
  const index = Math.min(
    Math.floor(progress * (points.length - 1)),
    points.length - 2
  );
  const t = (progress * (points.length - 1)) - index;

  return {
    x: points[index].x + (points[index + 1].x - points[index].x) * t,
    y: points[index].y + (points[index + 1].y - points[index].y) * t,
  };
}
```

#### Task 3.4: Performance Thresholds
```typescript
const PERFORMANCE_CONFIG = {
  // Switch to Canvas when exceeding thresholds
  svgMaxEdges: 80,
  svgMaxAnimatedEdges: 50,

  // Canvas settings
  canvasMaxParticlesPerEdge: 6,
  canvasTargetFPS: 60,

  // Auto-detection
  detectSlowdown: true,
  slowdownThreshold: 45, // fps

  // Fallback
  disableEffectsThreshold: 30, // fps
};
```

---

## File Structure

```
apps/studio/src/components/graph/edges/
├── effects/
│   ├── primitives/
│   │   ├── EmitPrimitive.tsx           # Existing
│   │   ├── ParticlesPrimitive.tsx      # Existing
│   │   ├── TrailPrimitive.tsx          # Existing
│   │   ├── ImpactPrimitive.tsx         # Existing
│   │   ├── GlowPrimitive.tsx           # Existing
│   │   ├── ZigzagPrimitive.tsx         # Existing
│   │   ├── InterferencePrimitive.tsx   # Existing
│   │   ├── ScanlinePrimitive.tsx       # Existing
│   │   ├── EnergyPulsePrimitive.tsx    # NEW (Phase 1)
│   │   ├── DNAHelixPrimitive.tsx       # NEW (Phase 1)
│   │   ├── MatrixCodePrimitive.tsx     # NEW (Phase 1)
│   │   └── RadarSweepPrimitive.tsx     # NEW (Phase 1)
│   └── EffectRenderer.tsx              # UPDATE (Phase 1)
├── system/
│   ├── themes.ts                       # UPDATE (Phase 1)
│   ├── types.ts                        # UPDATE (Phase 1)
│   └── constants.ts                    # UPDATE (Phase 1)
├── FloatingEdge.tsx                    # UPDATE (Phase 2)
├── BundledEdge.tsx                     # NEW (Phase 2)
├── EdgeUtils.ts                        # UPDATE (Phase 2)
├── CanvasEffectsLayer.tsx              # NEW (Phase 3)
└── index.ts                            # UPDATE (all phases)

apps/studio/src/hooks/
└── useParallelEdges.ts                 # NEW (Phase 2)
```

---

## Testing Strategy

### Unit Tests
- Each new primitive: render, animation timing, props
- Path generation: offset calculations, edge cases
- Bundling logic: grouping, threshold detection

### Visual Tests
- Storybook stories for each primitive in isolation
- Combined effects per ArcFamily
- Multi-edge scenarios (2, 3, 4, 5+ edges)
- Bundle expand/collapse animation

### Performance Tests
- Benchmark: 50, 100, 200 edges
- FPS monitoring during pan/zoom
- Memory usage over time

---

## Success Criteria

### Phase 1
- [ ] 4 new primitives rendering correctly
- [ ] Each ArcFamily has distinct visual identity
- [ ] No regression in existing effects
- [ ] 60fps with 50 animated edges

### Phase 2
- [ ] Parallel edges don't overlap
- [ ] Bundle badge shows correct count
- [ ] Hover expands bundled edges smoothly
- [ ] Works with all edge types

### Phase 3
- [ ] Canvas layer syncs with SVG edges
- [ ] 60fps with 200 animated edges
- [ ] Graceful fallback when performance degrades
- [ ] No visual difference from SVG mode

---

## Timeline

| Phase | Duration | Deliverables |
|-------|----------|--------------|
| Phase 1 | 2-3 days | 4 new primitives, family themes |
| Phase 2 | 1-2 days | Multi-edge bundling, hover expand |
| Phase 3 | 3-5 days | Canvas hybrid (future sprint) |

**Total: 3-5 days** for Phases 1-2 (immediate value)

---

## References

- Current implementation: `apps/studio/src/components/graph/edges/`
- Animation budget: `apps/studio/src/stores/animationStore.ts`
- LOD system: `apps/studio/src/components/graph/edges/system/constants.ts`
- ADR-012: 2-Realm Architecture (ArcFamily definitions)
- React Flow docs: https://reactflow.dev/docs/api/edges/custom-edges/
