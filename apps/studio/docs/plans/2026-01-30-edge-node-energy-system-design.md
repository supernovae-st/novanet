# Edge & Node Energy System Design

> Brainstorm session: 2026-01-30
> Status: Ready for implementation

## Overview

A comprehensive energy flow visualization system for NovaNet graph edges and nodes, inspired by neural synapse firing and knowledge transmission metaphors.

## Core Concept: The Triplet

```
SOURCE              TRAVEL                    TARGET
┌─────┐                                       ┌─────┐
│     │   ◎                                   │     │
│  ◎══╪══╱│╲══► ○╲╲╲ ════════════════► 💥≋≋ │  ◎  │
│     │  ╲│╱    comet                         │     │
└─────┘   │      trail                        └─────┘
      EMIT                                   IMPACT
   charge+ring                          flash+burst+ripple
                                          +node bounce
```

### 1. EMIT Effect (Source Node)

**Phase 1: Charge** (0.3s)
- Glow builds up at emission point
- Energy accumulates before release

**Phase 2: Release**
- Pulse expands outward
- Ring effect at spawn point
- Particle ejected with momentum

### 2. TRAVEL Effect (Edge Path)

- **Particles**: Multi-layered with comet trail
- **Trail**: 3-4 segments that fade progressively
- **Motion**: Slight sinusoidal oscillation
- **Easing**: Acceleration progressive (ease-in toward target)
- **Direction**: Always source → target (follows arrow)

### 3. IMPACT Effect (Target Node)

**Phase 1: Flash** (0.05s)
- Immediate bright response
- "Signal received!"

**Phase 2: Micro-burst**
- Small particle scatter
- Activation effect

**Phase 3: Ripples**
- Concentric waves expanding
- Multiple layers fading out

**Phase 4: Node Bounce**
- Target node scale: 1 → 1.005 → 1
- Subtle physical feedback

## Direction Rules

| Relationship Type | Direction | Reason |
|-------------------|-----------|--------|
| All relationships | source → target | Follow the arrow |
| SEMANTIC_LINK | Bidirectional | Peer relationship |

## Node Effects

### On Hover
- Scale: 1 → 1.02 → 1 (bounce with spring)
- Border glow intensifies + pulse
- Drop shadow increases
- Connected edges highlight

### On Selected
- Breathing glow (opacity oscillates)
- Orbiting particles (4-6, varied speeds/sizes)
- Ring pulse expanding periodically

### On Edge Hover (Connected Node)
- Subtle bounce (scale 1.01)
- Glow matches edge color

### On Impact Receive
- Flash burst at impact point
- Micro-bounce (scale 1.005)
- Ripple from impact point

## Edge Hover Effects

1. **Sweeping Light**: Scan effect traveling along edge
2. **Glow Intensify**: Ambient glow x2, blur increases
3. **Stroke Thickening**: +2px stroke width
4. **Particle Density**: x2 particles, faster

## Adaptive Timing

Based on edge temperature/intensity:

| Intensity | Cycle Time | Behavior |
|-----------|------------|----------|
| High (>0.7) | ~1s | Continuous, overlapping, fast |
| Medium | ~2s | Balanced |
| Low (<0.3) | ~3s | Sequential, contemplative |

## Performance & LOD

### Level of Detail (Zoom-based)
- **Close**: Full effects, all details
- **Medium**: Reduced particles, simpler effects
- **Far**: Minimal effects, ambient glow only

### Optimizations
- Viewport culling: Only animate visible edges
- Max animated edges limit
- GPU acceleration (transform, will-change)
- Respect `prefers-reduced-motion`

## Text Orientation Fix

Problem: Text appears upside-down when path goes right-to-left.

Solution:
```typescript
if (sourcePoint.x > targetPoint.x) {
  // Use reversed path for textPath
  // Text always reads left-to-right
}
```

## Color System

Effects use edge colors for visual coherence:
- Particles: Edge primary color
- Glow: Edge glow color
- Flash/Impact: White + edge color mix

## Settings & Controls

### Keyboard Shortcut
- `⇧E` (Shift+E): Cycle Full → Reduced → Off → Full

### Settings Panel
```
┌─────────────────────────────────────────────────┐
│ ✨ Animation Effects                      [ON]  │
├─────────────────────────────────────────────────┤
│ Mode:  ○ Full   ○ Reduced   ○ Off               │
│ Intensity: ──────●────────  [70%]               │
│ ☑ Edge particles (EMIT → TRAVEL → IMPACT)       │
│ ☑ Node effects (hover, selected, orbit)         │
│ ☑ Ambient glow                                  │
│ ☐ Respect reduced-motion (auto-detect)          │
└─────────────────────────────────────────────────┘
```

## 2D/3D Consistency

- Shared logic between React Flow (2D) and force-graph-3d (3D)
- Effects adapted to each renderer's capabilities
- Toggle to enable/disable per view mode

## Components to Create/Modify

1. **EmitEffect** - Charge + pulse + ring at source
2. **TravelEffect** - Enhanced DataPackets with comet trail
3. **ImpactEffect** - Flash + burst + ripple at target
4. **NodeOrbitEffect** - Orbiting particles for selected nodes
5. **NodeHoverEffect** - Bounce + glow for hovered nodes
6. **SmartLabel** - Auto-flip text orientation
7. **AnimationSettingsStore** - Zustand store for settings
8. **AnimationSettingsPanel** - UI for settings
9. **useAnimationShortcut** - ⇧E keyboard handler

## Accessibility

- Respect `prefers-reduced-motion` media query
- Provide "Off" mode for users who need it
- Settings persist across sessions
