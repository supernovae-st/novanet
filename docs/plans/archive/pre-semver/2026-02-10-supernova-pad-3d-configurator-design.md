# SuperNovae Pad 3D Configurator Design

**Date**: 2026-02-10
**Status**: Draft
**Author**: Thibaut + Claude

## Overview

Interactive 3D macropad configurator displayed as a **Popup/Modal** in NovaNet Studio. Features a photorealistic Work Louder-style macropad with blueprint background, translucent LED-lit keycaps, and interactive rotary encoders.

**Branding**: "SuperNovae Pad - NovaNet" (replaces "Work Louder x Figma 2024")

## Visual Design

### Blueprint Style Background
```
┌─────────────────────────────────────────────────────────────────────────────┐
│  BLUEPRINT BACKGROUND                                                        │
│  • Dark blue grid pattern (#1a365d base, #2d4a6f grid lines)                │
│  • Subtle paper texture overlay                                              │
│  • Dimension lines with measurements (90.45mm × 90.45mm)                    │
│  • Technical annotations in monospace font                                   │
│  • EdgesGeometry + LineSegments for wireframe overlay                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Hardware Dimensions (from reference images)
```
                    ┌──────── 90.45mm ────────┐
                    │                         │
                    │   ┌───┐ ┌───┐ ┌───┐     │
                    │   │ 1 │ │ 2 │ │ 3 │     │  ← Row 1: Keys 1-3
              ▲     │   └───┘ └───┘ └───┘     │
              │     │   ┌───┐ ┌───┐ ┌───┐     │
           90.45mm  │   │ 4 │ │ 5 │ │ 6 │     │  ← Row 2: Keys 4-6
              │     │   └───┘ └───┘ └───┘     │
              ▼     │   ┌───┐ ┌───┐ ┌───┐     │
                    │   │ 7 │ │ 8 │ │ 9 │     │  ← Row 3: Keys 7-9
                    │                         │
                    │   ◉ Large   ○ Small    │  ← Rotary encoders
                    │   Encoder   Knob       │
                    │                         │
                    │   ═════════════════     │  ← Scroll wheel
                    │        USB-C            │
                    └─────────────────────────┘
                         36.00mm sections
```

### Key Colors (from visual-encoding.yaml)

Each key displays a Layer icon based on its binding:

| Layer | Icon | Default Color | Description |
|-------|------|---------------|-------------|
| config | ⚙ | Red (#ef4444) | System settings |
| locale | ⊕ | Purple (#a855f7) | Locale controls |
| geography | ⊙ | Green (#22c55e) | Geographic navigation |
| knowledge | ◈ | Cyan (#06b6d4) | Knowledge atoms |
| foundation | ▣ | Orange (#f97316) | Project base |
| structure | ▤ | White (#f5f5f5) | Pages, blocks |
| semantic | ◆ | Blue (#3b82f6) | Entities, content |
| instruction | ▧ | Yellow (#eab308) | Generation prompts |
| output | ● | Pink (#ec4899) | Generated artifacts |

### Material Properties

```typescript
// Chassis - Dark blue anodized aluminum
const chassisMaterial = {
  color: '#1e3a5f',
  metalness: 0.8,
  roughness: 0.3,
};

// Keycap - Translucent resin with LED glow
const keycapMaterial = {
  transmission: 0.6,        // Translucency
  thickness: 2,             // Refraction depth
  roughness: 0.1,           // Smooth surface
  chromaticAberration: 0.02,
  emissive: keyColor,       // LED color from binding
  emissiveIntensity: 1.5,   // HDR glow for Bloom
  toneMapped: false,        // Required for HDR
};

// Encoder knob - Brushed silver
const encoderMaterial = {
  color: '#c0c0c0',
  metalness: 0.9,
  roughness: 0.4,
};
```

## Technical Stack

### Dependencies
```json
{
  "@react-three/fiber": "^8.x",
  "@react-three/drei": "^9.x",
  "@react-three/postprocessing": "^2.x",
  "three": "^0.160.x"
}
```

### Component Architecture

```
apps/studio/src/components/configurator/
├── SuperNovaePadModal.tsx       # Modal wrapper with trigger
├── SuperNovaePad3D.tsx          # Main Canvas component
├── components/
│   ├── Chassis.tsx              # Metal body with blueprint edges
│   ├── Keycap.tsx               # Interactive translucent key
│   ├── RotaryEncoder.tsx        # Draggable encoder
│   ├── ScrollWheel.tsx          # Horizontal scroll element
│   ├── UsbPort.tsx              # USB-C connector
│   └── BlueprintBackground.tsx  # Grid + dimension lines
├── hooks/
│   ├── useKeyBindings.ts        # Fetch bindings from store
│   ├── useEncoderDrag.ts        # Encoder rotation logic
│   └── useKeyPress.ts           # Key press animation
├── utils/
│   ├── geometry.ts              # Procedural mesh generation
│   └── visualEncoding.ts        # Map bindings to colors/icons
└── types.ts                     # TypeScript interfaces
```

## React Three Fiber Implementation

### Canvas Setup

```tsx
import { Canvas } from '@react-three/fiber';
import { Environment, PresentationControls } from '@react-three/drei';
import { EffectComposer, Bloom } from '@react-three/postprocessing';

function SuperNovaePad3D() {
  return (
    <Canvas
      camera={{ position: [0, 3, 5], fov: 45 }}
      shadows
      dpr={[1, 2]}
      gl={{ antialias: true }}
    >
      <Environment preset="studio" />

      <PresentationControls
        global
        polar={[-Math.PI / 4, Math.PI / 4]}
        azimuth={[-Math.PI / 4, Math.PI / 4]}
        config={{ mass: 1, tension: 170, friction: 26 }}
      >
        <group position={[0, 0, 0]}>
          <BlueprintBackground />
          <Chassis />
          <KeyGrid />
          <RotaryEncoders />
          <ScrollWheel />
          <UsbPort />
        </group>
      </PresentationControls>

      <EffectComposer>
        <Bloom
          intensity={0.5}
          luminanceThreshold={1}
          luminanceSmoothing={0.9}
        />
      </EffectComposer>
    </Canvas>
  );
}
```

### Interactive Keycap Component

```tsx
import { useState, useRef } from 'react';
import { useFrame } from '@react-three/fiber';
import { MeshTransmissionMaterial, RoundedBox, Text } from '@react-three/drei';
import { useSpring, animated } from '@react-spring/three';

interface KeycapProps {
  position: [number, number, number];
  binding: KeyBinding;
  onPress: () => void;
}

function Keycap({ position, binding, onPress }: KeycapProps) {
  const [hovered, setHovered] = useState(false);
  const [pressed, setPressed] = useState(false);
  const meshRef = useRef<THREE.Mesh>(null);

  // Get color/icon from visual-encoding
  const { color, icon } = getVisualEncoding(binding.layer);

  // Spring animation for press effect
  const { scale, positionY } = useSpring({
    scale: pressed ? 0.95 : hovered ? 1.05 : 1,
    positionY: pressed ? -0.05 : 0,
    config: { tension: 400, friction: 20 },
  });

  return (
    <animated.group
      position-x={position[0]}
      position-y={positionY.to(y => position[1] + y)}
      position-z={position[2]}
      scale={scale}
    >
      <RoundedBox
        ref={meshRef}
        args={[0.9, 0.4, 0.9]}
        radius={0.08}
        smoothness={4}
        onClick={(e) => {
          e.stopPropagation();
          setPressed(true);
          onPress();
          setTimeout(() => setPressed(false), 150);
        }}
        onPointerOver={(e) => {
          e.stopPropagation();
          setHovered(true);
          document.body.style.cursor = 'pointer';
        }}
        onPointerOut={() => {
          setHovered(false);
          document.body.style.cursor = 'default';
        }}
      >
        <MeshTransmissionMaterial
          transmission={0.6}
          thickness={2}
          roughness={0.1}
          chromaticAberration={0.02}
          emissive={color}
          emissiveIntensity={hovered ? 2 : 1.5}
          toneMapped={false}
        />
      </RoundedBox>

      {/* Icon on keycap */}
      <Text
        position={[0, 0.21, 0]}
        rotation={[-Math.PI / 2, 0, 0]}
        fontSize={0.3}
        color={color}
        anchorX="center"
        anchorY="middle"
      >
        {icon}
      </Text>
    </animated.group>
  );
}
```

### Rotary Encoder with Drag

```tsx
import { useRef, useState } from 'react';
import { useThree } from '@react-three/fiber';
import { useDrag } from '@use-gesture/react';

function RotaryEncoder({ position, onChange }: EncoderProps) {
  const [rotation, setRotation] = useState(0);
  const meshRef = useRef<THREE.Mesh>(null);
  const { size, viewport } = useThree();
  const aspect = size.width / viewport.width;

  const bind = useDrag(({ movement: [mx], memo }) => {
    const initial = memo ?? rotation;
    const newRotation = initial + mx / (aspect * 50);
    setRotation(newRotation);
    onChange(newRotation);
    return initial;
  });

  return (
    <group position={position}>
      {/* Knurled knob */}
      <mesh ref={meshRef} rotation-y={rotation} {...bind()}>
        <cylinderGeometry args={[0.5, 0.5, 0.3, 32]} />
        <meshStandardMaterial
          color="#c0c0c0"
          metalness={0.9}
          roughness={0.4}
        />
      </mesh>

      {/* Knurling pattern (EdgesGeometry) */}
      <lineSegments>
        <edgesGeometry attach="geometry" args={[knurlGeometry]} />
        <lineBasicMaterial color="#888888" />
      </lineSegments>
    </group>
  );
}
```

### Blueprint Background

```tsx
function BlueprintBackground() {
  return (
    <group position={[0, -0.5, 0]} rotation={[-Math.PI / 2, 0, 0]}>
      {/* Grid plane */}
      <mesh>
        <planeGeometry args={[10, 10]} />
        <meshBasicMaterial color="#1a365d" />
      </mesh>

      {/* Grid lines */}
      <gridHelper args={[10, 20, '#2d4a6f', '#2d4a6f']} rotation={[Math.PI / 2, 0, 0]} />

      {/* Dimension lines */}
      <DimensionLine start={[-2, 0, 2.5]} end={[2, 0, 2.5]} label="90.45mm" />
      <DimensionLine start={[2.5, 0, -2]} end={[2.5, 0, 2]} label="90.45mm" vertical />

      {/* Technical annotations */}
      <Text position={[-3, 0, 3]} fontSize={0.15} color="#4a7a9f">
        SuperNovae Pad - NovaNet
      </Text>
    </group>
  );
}
```

## Modal Integration

### Using Radix Dialog (from Magic MCP research)

```tsx
import * as Dialog from '@radix-ui/react-dialog';
import { motion, AnimatePresence } from 'motion/react';

export function SuperNovaePadModal() {
  const [open, setOpen] = useState(false);

  return (
    <Dialog.Root open={open} onOpenChange={setOpen}>
      <Dialog.Trigger asChild>
        <button className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg">
          <KeyboardIcon className="w-4 h-4" />
          Configure Pad
        </button>
      </Dialog.Trigger>

      <AnimatePresence>
        {open && (
          <Dialog.Portal forceMount>
            <Dialog.Overlay asChild>
              <motion.div
                className="fixed inset-0 bg-black/50 backdrop-blur-sm z-50"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
              />
            </Dialog.Overlay>

            <Dialog.Content asChild>
              <motion.div
                className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                           w-[800px] h-[600px] bg-slate-900 rounded-2xl border
                           border-slate-700 shadow-2xl z-50 overflow-hidden"
                initial={{ opacity: 0, scale: 0.95, y: 20 }}
                animate={{ opacity: 1, scale: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.95, y: 20 }}
              >
                <Dialog.Title className="sr-only">
                  SuperNovae Pad Configuration
                </Dialog.Title>

                <SuperNovaePad3D />

                <Dialog.Close asChild>
                  <button className="absolute top-4 right-4 p-2 text-slate-400 hover:text-white">
                    <XIcon className="w-5 h-5" />
                  </button>
                </Dialog.Close>
              </motion.div>
            </Dialog.Content>
          </Dialog.Portal>
        )}
      </AnimatePresence>
    </Dialog.Root>
  );
}
```

## Visual Encoding Integration

### Mapping Function

```typescript
// utils/visualEncoding.ts
import { visualEncoding } from '@novanet/core/models/visual-encoding';

interface VisualProps {
  color: string;
  icon: string;
  animation?: {
    duration: number;
    easing: string;
  };
}

export function getVisualEncoding(layer: string): VisualProps {
  const layerConfig = visualEncoding.layers[layer];

  return {
    color: layerConfig?.color ?? '#ffffff',
    icon: layerConfig?.icon ?? '●',
    animation: visualEncoding.animation?.presets?.standard,
  };
}

// Layer icon mapping from visual-encoding.yaml
const LAYER_ICONS = {
  config: '⚙',
  locale: '⊕',
  geography: '⊙',
  knowledge: '◈',
  foundation: '▣',
  structure: '▤',
  semantic: '◆',
  instruction: '▧',
  output: '●',
};

// Trait icons for special states
const TRAIT_ICONS = {
  invariant: '■',
  localized: '□',
  knowledge: '◊',
  generated: '★',
  aggregated: '▪',
};
```

## Interactions

### Key Press
1. **Hover**: Scale 1.05, emissiveIntensity 2
2. **Click**: Scale 0.95, Y-offset -0.05, elastic bounce back
3. **Bound action**: Execute keyboard shortcut

### Rotary Encoder
1. **Drag horizontal**: Rotate encoder, trigger action
2. **Click**: Toggle mode / execute default action

### Scroll Wheel
1. **Drag horizontal**: Scroll action
2. **Visual feedback**: Slight rotation animation

## Performance Optimizations

1. **Geometry reuse**: Share RoundedBoxGeometry across all keycaps
2. **Material instancing**: Single MeshTransmissionMaterial with color uniforms
3. **On-demand rendering**: `frameloop="demand"` when not animating
4. **Resolution scaling**: Lower Bloom resolution on mobile
5. **Suspense boundaries**: Lazy load 3D scene

```tsx
<Canvas frameloop="demand" performance={{ min: 0.5 }}>
  <Suspense fallback={<LoadingSpinner />}>
    <SuperNovaePadScene />
  </Suspense>
</Canvas>
```

## File Structure

```
apps/studio/src/
├── components/
│   └── configurator/
│       ├── SuperNovaePadModal.tsx
│       ├── SuperNovaePad3D.tsx
│       └── components/
│           ├── Chassis.tsx
│           ├── Keycap.tsx
│           ├── RotaryEncoder.tsx
│           ├── ScrollWheel.tsx
│           ├── UsbPort.tsx
│           └── BlueprintBackground.tsx
├── hooks/
│   └── configurator/
│       ├── useKeyBindings.ts
│       ├── useEncoderDrag.ts
│       └── useKeyPress.ts
└── utils/
    └── configurator/
        ├── geometry.ts
        └── visualEncoding.ts
```

## Testing Requirements

Must pass all existing tests in:
- `apps/studio/src/stores/__tests__/`
- `apps/studio/src/components/__tests__/`
- `apps/studio/src/hooks/__tests__/`
- `apps/studio/src/lib/__tests__/keyboard.test.ts`

New tests to create:
- `SuperNovaePadModal.test.tsx` - Modal open/close
- `Keycap.test.tsx` - Click/hover interactions
- `useKeyBindings.test.ts` - Binding resolution
- `visualEncoding.test.ts` - Color/icon mapping

## Success Criteria

- [ ] Modal opens with smooth animation
- [ ] 3D pad renders with correct materials
- [ ] All 9 keys display correct layer icons/colors
- [ ] Keys respond to hover (scale + glow)
- [ ] Keys respond to click (press animation)
- [ ] Rotary encoders can be dragged
- [ ] Blueprint background with dimension lines
- [ ] Bloom glow effect on keycaps
- [ ] PresentationControls for orbit
- [ ] All existing tests pass
- [ ] Performance: 60fps on modern hardware
- [ ] Accessibility: keyboard navigation

## Research Sources

### Magic MCP Components
- **Keyboard Keys**: Interactive keycap with press animation
- **Animated Keyboard**: Full keyboard with RGB lighting
- **Animated Modal**: Spring-based modal with backdrop blur
- **Dialog**: Radix-based settings dialog

### Context7 Documentation
- **React Three Fiber**: Canvas setup, events, raycasting
- **Drei**: Environment, MeshTransmissionMaterial, PresentationControls
- **Postprocessing**: Bloom effect configuration

### Perplexity Research
- Hall-effect keyboard rapid trigger technology
- React Three Fiber bloom + emissive patterns
- Magnetic switch thermal calibration

## ASCII Mockups

### View 1: Modal Closed (Trigger Button)
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NOVANET STUDIO - MAIN VIEW                                                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │                         GRAPH CANVAS                                     │   │
│  │                                                                          │   │
│  │     ◈───────◆                                                           │   │
│  │     │       │                                                            │   │
│  │     ●───────▣───────▤                                                   │   │
│  │             │                                                            │   │
│  │             ⚙                                                            │   │
│  │                                                                          │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌──────────────────────────────┐                                              │
│  │  ⌨️  Configure Pad           │  ← TRIGGER BUTTON                            │
│  └──────────────────────────────┘                                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### View 2: Modal Open with 3D View
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ░░░░░░░░░░░░░░░░░░░░░░ BACKDROP BLUR ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
│  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
│  ░░░  ╔═══════════════════════════════════════════════════════════════╗  ░░░  │
│  ░░░  ║  SuperNovae Pad - NovaNet                              [X]   ║  ░░░  │
│  ░░░  ╠═══════════════════════════════════════════════════════════════╣  ░░░  │
│  ░░░  ║                                                               ║  ░░░  │
│  ░░░  ║          ┌─────────────────────────────────────┐              ║  ░░░  │
│  ░░░  ║          │   ╭───╮  ╭───╮  ╭───╮               │              ║  ░░░  │
│  ░░░  ║          │   │ ⚙ │  │ ⊕ │  │ ⊙ │               │              ║  ░░░  │
│  ░░░  ║          │   ╰───╯  ╰───╯  ╰───╯               │              ║  ░░░  │
│  ░░░  ║          │   ╭───╮  ╭───╮  ╭───╮               │              ║  ░░░  │
│  ░░░  ║          │   │ ◈ │  │ ▣ │  │ ▤ │    ◎ ENCODER  │              ║  ░░░  │
│  ░░░  ║          │   ╰───╯  ╰───╯  ╰───╯               │              ║  ░░░  │
│  ░░░  ║          │   ╭───╮  ╭───╮  ╭───╮    ○ knob     │              ║  ░░░  │
│  ░░░  ║          │   │ ◆ │  │ ▧ │  │ ● │               │              ║  ░░░  │
│  ░░░  ║          │   ╰───╯  ╰───╯  ╰───╯               │              ║  ░░░  │
│  ░░░  ║          │   ═══════════════════  USB-C       │              ║  ░░░  │
│  ░░░  ║          └─────────────────────────────────────┘              ║  ░░░  │
│  ░░░  ║                    90.45mm × 90.45mm                          ║  ░░░  │
│  ░░░  ║                                                               ║  ░░░  │
│  ░░░  ╚═══════════════════════════════════════════════════════════════╝  ░░░  │
│  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### View 3: Keycap Hover State
```
                    NORMAL              HOVER               PRESSED
                  ╭───────╮           ╭───────╮           ╭───────╮
                  │       │           │░░░░░░░│           │▓▓▓▓▓▓▓│
                  │   ⚙   │    →      │░░ ⚙ ░░│    →      │▓▓ ⚙ ▓▓│
                  │       │           │░░░░░░░│           │▓▓▓▓▓▓▓│
                  ╰───────╯           ╰───────╯           ╰───────╯
                   scale:1            scale:1.05          scale:0.95
                  glow: 1.5x          glow: 2.0x          glow: 2.5x
                                      cursor:pointer       Y: -0.05
```

### View 4: Isometric 3D View
```
                         _______________
                       /               /│
                      /   ⚙   ⊕   ⊙  / │
                     /_______________/  │
                    │               │   │
                    │   ◈   ▣   ▤  │   │  ◎ ← Large encoder
                    │               │   │  ○ ← Small knob
                    │   ◆   ▧   ●  │   │
                    │               │  /
                    │═══════════════│ /   ← Scroll wheel
                    └───────────────┘/
                         USB-C ▼
```

### View 5: Blueprint Background Detail
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│ │     │     │     │     │     │     │     │     │     │     │     │     │ │   │
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│ │     │     │     │     │     │     │     │     │     │     │     │     │ │   │
│ ─ ─ ─ ─ ─ ─◄────────── 90.45mm ──────────►─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│ │     │     │  ╭───╮  ╭───╮  ╭───╮  │     │     │     │     │     │     │ │   │
│ ─ ─ ─ ─ ─ ─ ─  │ 1 │  │ 2 │  │ 3 │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│ │     │     │  ╰───╯  ╰───╯  ╰───╯  │     │     │     │     │     │     │ │ ▲ │
│ ─ ─ ─ ─ ─ ─ ─  ╭───╮  ╭───╮  ╭───╮  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │ │
│ │     │     │  │ 4 │  │ 5 │  │ 6 │  │     │     │     │     │     │     │ │ │ │
│ ─ ─ ─ ─ ─ ─ ─  ╰───╯  ╰───╯  ╰───╯  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─90.45mm
│ │     │     │  ╭───╮  ╭───╮  ╭───╮  │     │     │     │     │     │     │ │ │ │
│ ─ ─ ─ ─ ─ ─ ─  │ 7 │  │ 8 │  │ 9 │  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │ │
│ │     │     │  ╰───╯  ╰───╯  ╰───╯  │     │     │     │     │     │     │ │ ▼ │
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─  │
│                                                                                 │
│  SuperNovae Pad - NovaNet                                    REV 1.0 2026-02  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### View 6: Rotary Encoder Interaction
```
                    IDLE                DRAG LEFT           DRAG RIGHT

                    ╭───╮               ╭───╮               ╭───╮
                   ╱│   │╲             ╱│ ↺ │╲             ╱│ ↻ │╲
                  │ │ ● │ │     ←     │ │ ● │ │     →     │ │ ● │ │
                   ╲│   │╱             ╲│   │╱             ╲│   │╱
                    ╰───╯               ╰───╯               ╰───╯

                  rotation: 0°       rotation: -15°      rotation: +15°
                  cursor: grab       cursor: grabbing    cursor: grabbing
                                    action: decrement    action: increment
```

### View 7: Key Colors by Layer (visual-encoding.yaml)
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  KEY COLOR MAPPING FROM VISUAL-ENCODING.YAML                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐                                     │
│  │  CONFIG   │ │  LOCALE   │ │ GEOGRAPHY │                                     │
│  │    ⚙      │ │    ⊕      │ │    ⊙      │                                     │
│  │ #ef4444   │ │ #a855f7   │ │ #22c55e   │                                     │
│  │   RED     │ │  PURPLE   │ │  GREEN    │                                     │
│  └───────────┘ └───────────┘ └───────────┘                                     │
│                                                                                 │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐                                     │
│  │ KNOWLEDGE │ │ FOUNDATION│ │ STRUCTURE │                                     │
│  │    ◈      │ │    ▣      │ │    ▤      │                                     │
│  │ #06b6d4   │ │ #f97316   │ │ #f5f5f5   │                                     │
│  │   CYAN    │ │  ORANGE   │ │  WHITE    │                                     │
│  └───────────┘ └───────────┘ └───────────┘                                     │
│                                                                                 │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐                                     │
│  │ SEMANTIC  │ │INSTRUCTION│ │  OUTPUT   │                                     │
│  │    ◆      │ │    ▧      │ │    ●      │                                     │
│  │ #3b82f6   │ │ #eab308   │ │ #ec4899   │                                     │
│  │   BLUE    │ │  YELLOW   │ │   PINK    │                                     │
│  └───────────┘ └───────────┘ └───────────┘                                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### View 8: Animation Flow
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ANIMATION SEQUENCE                                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. MODAL OPEN                                                                  │
│     ┌───┐         ┌─────────────────┐                                          │
│     │ ▪ │   →     │                 │   opacity: 0→1, scale: 0.95→1            │
│     └───┘         │    3D CANVAS    │   duration: 300ms, ease: spring          │
│                   │                 │                                          │
│                   └─────────────────┘                                          │
│                                                                                 │
│  2. KEYCAP HOVER                                                               │
│     ╭───╮         ╭─────╮                                                      │
│     │ ⚙ │   →     │░░⚙░░│   scale: 1→1.05, emissive: 1.5→2.0                  │
│     ╰───╯         ╰─────╯   duration: 150ms, ease: easeOut                     │
│                                                                                 │
│  3. KEYCAP PRESS                                                               │
│     ╭─────╮       ╭───╮                                                        │
│     │░░⚙░░│  →    │▓⚙▓│   scale: 1.05→0.95, Y: 0→-0.05                        │
│     ╰─────╯       ╰───╯   duration: 100ms + elastic bounce                     │
│                                                                                 │
│  4. ENCODER ROTATE                                                             │
│       ●             ●                                                          │
│      ╱│╲    →      ╱│╲    rotation: += delta, particles emit                   │
│     │ │ │        │ │↻│    continuous while dragging                            │
│      ╲│╱           ╲│╱                                                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Next Steps

1. Run existing tests to verify baseline
2. Create procedural geometry for chassis/keycaps
3. Implement Keycap component with MeshTransmissionMaterial
4. Add RotaryEncoder with drag gesture
5. Integrate visual-encoding.yaml mappings
6. Add Bloom postprocessing
7. Create modal wrapper
8. Write new component tests
9. Performance optimization pass
