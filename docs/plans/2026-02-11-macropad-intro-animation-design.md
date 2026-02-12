# Macropad Intro Animation Design

**Date**: 2026-02-11
**Status**: Approved
**Version**: 1.0

## Overview

Stylish intro animation for the Work Louder macropad 3D visualizer with "SuperNovae Pad" title.

## Architecture Decision

### Problem
- drei `Text` component caused Worker module error in Next.js 16.1.6 Turbopack
- Need performant particle system without excessive draw calls

### Solution
- **HTML Overlay** for text (framer-motion) - avoids WebGL text rendering issues
- **InstancedMesh** for particles - single draw call for hundreds of particles
- **CSS animations** + **useFrame** for smooth physics

## Animation Sequence

```
Timeline (3.5s total):
├── 0.0s - 0.5s: Camera starts further back, fog dense
├── 0.5s - 1.5s: "SuperNovae Pad" text appears (staggered letters)
├── 1.5s - 2.0s: Text holds, particles start spawning
├── 2.0s - 2.5s: Text explodes into particles
├── 2.5s - 3.0s: Particles disperse, pad fades in
├── 3.0s - 3.5s: Camera settles to final position
└── 3.5s+: Continuous slow rotation begins
```

## Components

### 1. IntroAnimation (HTML Overlay)

```typescript
// Location: apps/studio/src/components/macropad/IntroAnimation.tsx
interface IntroAnimationProps {
  onComplete: () => void;
  duration?: number;
}
```

**Responsibilities**:
- Render "SuperNovae Pad" with staggered letter animation
- Handle explosion effect with framer-motion
- Call onComplete when animation finishes

### 2. ParticleField (Three.js)

```typescript
// Inside CreatorBoardLowPoly.tsx
const ParticleField = memo(function ParticleField({
  active: boolean,
  count?: number // default 200
})
```

**Responsibilities**:
- InstancedMesh with 200 particles (single draw call)
- Particles spawn from center, disperse outward
- Glow effect via emissive material

### 3. Camera Animation

```typescript
// Spring-based camera position interpolation
const cameraAnimation = {
  initial: [0, 8, 12],  // Further back
  final: [0, 5, 8],     // Original position
  duration: 3000
}
```

## Technical Specifications

### Performance Budget
- **Draw calls**: < 100 (InstancedMesh for particles)
- **Frame budget**: 16ms (60fps target)
- **Memory**: Reuse geometries/materials (existing pattern)

### Text Animation (framer-motion)
```typescript
const letterVariants = {
  hidden: { opacity: 0, y: 20 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.05, type: 'spring' }
  }),
  explode: (i: number) => ({
    opacity: 0,
    scale: 0,
    x: (Math.random() - 0.5) * 200,
    y: (Math.random() - 0.5) * 200,
    transition: { duration: 0.5 }
  })
}
```

### Particle System (InstancedMesh)
```typescript
// Single geometry, single material, 200 instances
const particleGeometry = new THREE.SphereGeometry(0.05, 8, 8);
const particleMaterial = new THREE.MeshBasicMaterial({
  color: '#00ffff',
  transparent: true,
  opacity: 0.8
});

// useFrame updates instance matrices for animation
useFrame((state, delta) => {
  // Update each instance position based on velocity
});
```

## File Structure

```
apps/studio/src/components/macropad/
├── CreatorBoardLowPoly.tsx  # Main component (modified)
├── IntroAnimation.tsx       # NEW: HTML overlay text animation
└── ...existing files
```

## Integration Points

1. **CreatorBoardLowPoly.tsx**
   - Add `showIntro` state (default: true)
   - Render IntroAnimation overlay when showIntro
   - Add ParticleField inside Canvas
   - Animate camera position with useFrame

2. **IntroAnimation.tsx**
   - Absolute positioned overlay on Canvas
   - framer-motion AnimatePresence
   - Callback when animation completes

## Testing Strategy (TDD)

1. **Unit**: IntroAnimation renders correctly
2. **Unit**: Animation phases complete in sequence
3. **Integration**: Camera interpolates smoothly
4. **Visual**: No jarring transitions, 60fps maintained

## Rollback Plan

If issues arise, the animation can be disabled via:
```typescript
<CreatorBoardLowPoly skipIntro={true} {...props} />
```
