# Arc Effects Redesign - Full Signature Effects

**Date**: 2026-02-11
**Status**: Design Approved
**Effort**: 12-20h estimated

## Overview

Complete redesign of arc visual effects to give each of the 5 arc families a unique, signature effect that conveys semantic meaning through animation.

## Design Decisions

### Research Synthesis

Research conducted via 4 parallel agents:
1. **Codebase Analysis**: Current 5 effects, tier-based scaling, hub node detection
2. **Web Research**: WebGL particles, physics-based motion, layered glow
3. **Context7/React Flow**: `animateMotion` > `stroke-dasharray`, GSAP/Framer Motion
4. **Visual Coherence**: Metaphor mismatches, speed duplication, accessibility

### Key Findings

- Current effects are functional but lack "wow factor"
- Generation/Semantic metaphors don't match implementations
- Tier-based performance scaling works well (keep it)
- Hub node detection prevents particle explosion (keep it)

---

## 5 Signature Effects

### 1. Ownership (Blue #3b82f6) - Power Conduit

**Semantic**: Parent-child hierarchy, structural authority, data containment

**Visual Concept**:
- **Main arc**: Thick glowing cable (4px stroke + 3-layer glow)
- **Inner core**: Bright white-blue "hot wire" (1.5px, 100% opacity)
- **Energy packets**: 3 large orbs (20px) traveling in convoy
- **Ambient hum**: Subtle pulsing glow (0.98-1.02 scale, 3s cycle)

**Animation**:
```
Duration: 8s (slow, authoritative)
Orbs: Spaced evenly, calcMode="spline" for smooth acceleration
Cable pulse: opacity 0.7→0.9→0.7 (4s cycle)
Trail: Subtle afterglow behind each orb
```

**Metaphor**: High-voltage power line delivering authority

---

### 2. Localization (Green #22c55e) - True DNA Helix

**Semantic**: Invariant↔Locale bridge, content adaptation, cultural DNA

**Visual Concept**:
- **Strand 1**: Primary wave (±25px amplitude, phase 0°)
- **Strand 2**: Secondary wave (±25px amplitude, phase 180°)
- **Base pairs**: White connectors (3-4 per arc) appearing at crossings
- **3D illusion**: Opacity gradient (closer strand brighter)
- **Gene markers**: Small bright dots at strand intersections

**Animation**:
```
Duration: 10s (slow, methodical adaptation)
Strands: Perpendicular oscillation values="-25;25;-25"
Connectors: Fade in/out as strands cross (0.3s transition)
Gene markers: Pulse brighter at intersections
```

**Metaphor**: Content DNA unwinding and adapting to locale

---

### 3. Semantic (Orange #f97316) - Synaptic Firing

**Semantic**: Meaning connections, entity relationships, concept links

**Visual Concept**:
- **Baseline**: Dim, dormant arc (30% opacity, subtle glow)
- **Firing pulse**: Bright wave travels source→target (0.3s)
- **Signal propagation**: Leading edge white-hot, trailing decay
- **Spark shower**: 3-5 tiny particles ejected at target
- **Residual glow**: Arc stays brighter for 1s after firing

**Animation**:
```
Firing interval: Random 2-4s (organic timing)
Pulse travel: 0.3s (fast signal)
Spark physics: Gravity + scatter (0.5s lifetime)
Residual: Linear fade 0.8→0.3 over 1s
```

**Metaphor**: Neurons firing and propagating signals

---

### 4. Generation (Purple #8b5cf6) - Matrix Code Rain

**Semantic**: LLM generation pipeline, AI processing, prompt→output flow

**Visual Concept**:
- **Data stream**: Characters ("01" or "▓▒░") falling along arc
- **Variable speed**: Some fast, some slow (parallax depth)
- **Processing burst**: Periodic bright flash at midpoint
- **Output glow**: Target pulses when data "arrives"
- **Scanline**: Horizontal line sweeping across arc

**Animation**:
```
Character fall: 1-3s varied (parallax effect)
Processing burst: Every 2s (0.2s duration)
Scanline: 4s sweep cycle
Characters: 6-10 simultaneously
```

**Metaphor**: Data being processed through AI neural network

---

### 5. Mining (Pink #ec4899) - Sonar Pulse

**Semantic**: SEO/GEO data extraction, discovery, market intelligence

**Visual Concept**:
- **Ping source**: Bright pulse emitted from source node
- **Wave propagation**: 4 concentric rings expanding along arc
- **Echo return**: Faint reflection wave traveling back
- **Data blip**: Small bright dot at target when wave arrives
- **Ambient scan**: Slow sweeping highlight (10s cycle)

**Animation**:
```
Ping interval: Every 3s (methodical scanning)
Ring expansion: 1.5s source→target
Echo return: 2s target→source (slower)
Blip: 0.5s bright flash then fade
```

**Metaphor**: Sonar pinging and discovering data in the depths

---

## Implementation Plan

### Phase 1: Foundation (2-3h)
1. Create `ArcEffectRenderer` component system
2. Extract shared utilities (glow layers, path utils)
3. Update effect selection in `InlineEdgeEffects`

### Phase 2: Individual Effects (8-12h)
4. Implement Power Conduit (ownership)
5. Implement DNA Helix (localization)
6. Implement Synaptic Firing (semantic)
7. Implement Matrix Code Rain (generation)
8. Implement Sonar Pulse (mining)

### Phase 3: Polish (2-3h)
9. Update `visual-encoding.yaml` metaphors
10. Performance testing with hub nodes
11. Tier-adaptive scaling for new effects
12. Commit and document

---

## Technical Considerations

### Performance
- Keep tier-based scaling (ULTRA/HIGH/MEDIUM/LOW/MINIMAL)
- Hub node detection forces LOW tier
- Use `animateMotion` for path traversal (not stroke-dasharray)
- Limit particle count per tier

### Accessibility
- Animation provides redundancy for colorblind users
- Respect `prefers-reduced-motion` (fallback to LOW tier)
- Each effect has distinct motion pattern

### File Changes

```
apps/studio/src/components/graph/edges/
├── FloatingEdge.tsx          # Update InlineEdgeEffects switch
├── effects/
│   ├── PowerConduit.tsx      # NEW: Ownership effect
│   ├── DNAHelix.tsx          # NEW: Localization effect
│   ├── SynapticFiring.tsx    # NEW: Semantic effect
│   ├── MatrixCodeRain.tsx    # NEW: Generation effect
│   └── SonarPulse.tsx        # NEW: Mining effect

packages/core/models/
└── visual-encoding.yaml      # Update metaphor descriptions
```

---

## Approval

- [x] User approved all 5 concepts
- [x] Matrix Code Rain selected for Generation
- [ ] Implementation started
