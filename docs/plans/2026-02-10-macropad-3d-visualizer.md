# Macropad 3D Visualizer - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a 3D macropad visualizer modal to Studio for configuring/viewing Work Louder Micro keybindings, with the same UX as controller configurators (Steam Input, Razer Synapse, VIA).

**Shortcut:** `Ctrl+K` or `Cmd+K` opens the modal (like command palette pattern)

**Config Location:** `~/Projects/work-louder/studio-integration/`

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         MACROPAD 3D VISUALIZER                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   ┌───────────────────────────────────────────────────────────────┐     │
│   │                                                               │     │
│   │                    [3D Model - Three.js]                      │     │
│   │                                                               │     │
│   │         ┌───────┐             ┌───────┐                       │     │
│   │         │   1   │  ┌───┐┌───┐ │   2   │                       │     │
│   │         │ Meta  │  │ K ││ ↑ │ │ Data  │  ← Clickable hotspots │     │
│   │         └───────┘  └───┘└───┘ └───────┘                       │     │
│   │                                                               │     │
│   │         ┌───┐┌───┐┌───┐┌───┐                                  │     │
│   │         │ H ││ ← ││SPC││ L │  │ → │                           │     │
│   │         └───┘└───┘└───┘└───┘                                  │     │
│   │                                                               │     │
│   │         ┌───┐┌───┐┌───┐┌───┐                                  │     │
│   │         │ 3 ││ J ││ ↓ ││ 4 │                                  │     │
│   │         │Ovr││   ││   ││Qry│                                  │     │
│   │         └───┘└───┘└───┘└───┘                                  │     │
│   │                                                               │     │
│   │                    ◎ Encoder (rotatable)                      │     │
│   │                                                               │     │
│   └───────────────────────────────────────────────────────────────┘     │
│                                                                         │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │ [ Layer 0 ] [ Layer 1 ] [ Layer 2 ]      Selected: [K] = ↑ Up  │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│   [?] Help    [Export JSON]    [Import]    [Apply to Device]   [Close] │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Task 1: Create Config Folder Structure

**Files:**
- Create: `~/Projects/work-louder/studio-integration/`

**Step 1.1: Create folder structure**

```bash
mkdir -p ~/Projects/work-louder/studio-integration/{models,configs,assets}
```

**Step 1.2: Create default config file**

Create `~/Projects/work-louder/studio-integration/configs/work-louder-micro.json`:

```json
{
  "device": {
    "name": "Work Louder Micro",
    "vendorId": "0x574C",
    "productId": "0xE6E3",
    "matrix": { "rows": 4, "cols": 4 },
    "encoder": true
  },
  "layers": [
    {
      "id": 0,
      "name": "Navigation",
      "color": "#00FFFF",
      "keys": {
        "0,0": { "key": "1", "label": "Meta", "action": "MODE_META" },
        "0,1": { "key": "K", "label": "↑", "action": "NAV_UP" },
        "0,2": { "key": "↑", "label": "↑", "action": "ARROW_UP" },
        "0,3": { "key": "2", "label": "Data", "action": "MODE_DATA" },
        "1,0": { "key": "H", "label": "←", "action": "NAV_LEFT" },
        "1,1": { "key": "←", "label": "←", "action": "ARROW_LEFT" },
        "1,2": { "key": "SPC", "label": "Toggle", "action": "TOGGLE" },
        "1,3": { "key": "L", "label": "→", "action": "NAV_RIGHT" },
        "2,0": { "key": "3", "label": "Overlay", "action": "MODE_OVERLAY" },
        "2,1": { "key": "J", "label": "↓", "action": "NAV_DOWN" },
        "2,2": { "key": "↓", "label": "↓", "action": "ARROW_DOWN" },
        "2,3": { "key": "4", "label": "Query", "action": "MODE_QUERY" },
        "3,0": { "key": "TAB", "label": "Cycle", "action": "CYCLE_FOCUS" },
        "3,1": { "key": "ENT", "label": "Enter", "action": "CONFIRM" },
        "3,2": { "key": "ESC", "label": "Esc", "action": "CLOSE" },
        "3,3": { "key": "MO1", "label": "Layer", "action": "LAYER_1" }
      },
      "encoder": {
        "cw": { "key": "K", "label": "Scroll Up" },
        "ccw": { "key": "J", "label": "Scroll Down" }
      }
    },
    {
      "id": 1,
      "name": "YAML & Overlays",
      "color": "#9945FF",
      "keys": {}
    },
    {
      "id": 2,
      "name": "System",
      "color": "#FF4545",
      "keys": {}
    }
  ]
}
```

**Step 1.3: Create README**

Create `~/Projects/work-louder/studio-integration/README.md`:

```markdown
# Work Louder Studio Integration

Configuration files for the Macropad 3D Visualizer in Novanet Studio.

## Structure

```
studio-integration/
├── configs/           # Device keybinding configs (JSON)
│   └── work-louder-micro.json
├── models/            # 3D models (GLTF/GLB)
│   └── work-louder-micro.glb
└── assets/            # Textures, icons
    └── keycap-font.woff2
```

## Usage

1. Open Studio (`pnpm dev`)
2. Press `Cmd+K` / `Ctrl+K` to open Macropad Visualizer
3. Select device from dropdown
4. Click keys to view/edit bindings
5. Switch layers with tabs
6. Export/Import configs as JSON
```

**Step 1.4: Commit**

```bash
cd ~/Projects/work-louder
git add studio-integration/
git commit -m "feat: add Studio integration config structure

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Create MacropadVisualizer Component Shell

**Files:**
- Create: `apps/studio/src/components/macropad/MacropadVisualizer.tsx`
- Create: `apps/studio/src/components/macropad/index.ts`

**Step 2.1: Create component directory**

```bash
mkdir -p apps/studio/src/components/macropad
```

**Step 2.2: Create MacropadVisualizer.tsx**

```tsx
'use client';

/**
 * MacropadVisualizer - 3D Work Louder Micro Configuration Modal
 *
 * Features:
 * - 3D model of macropad with clickable key hotspots
 * - Layer tabs (Navigation, YAML, System)
 * - Key binding display and editing
 * - Import/Export JSON configs
 * - Device connection status
 */

import { memo, useState, useCallback } from 'react';
import { X, Keyboard, Download, Upload, Check, HelpCircle } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';

export interface MacropadVisualizerProps {
  /** Whether the modal is open */
  open: boolean;
  /** Callback when modal should close */
  onOpenChange: (open: boolean) => void;
}

// Layer configuration
const LAYERS = [
  { id: 0, name: 'Navigation', color: '#00FFFF' },
  { id: 1, name: 'YAML', color: '#9945FF' },
  { id: 2, name: 'System', color: '#FF4545' },
];

export const MacropadVisualizer = memo(function MacropadVisualizer({
  open,
  onOpenChange,
}: MacropadVisualizerProps) {
  const [activeLayer, setActiveLayer] = useState(0);
  const [selectedKey, setSelectedKey] = useState<string | null>(null);

  const handleKeyClick = useCallback((keyId: string) => {
    setSelectedKey(keyId);
  }, []);

  const handleExport = useCallback(() => {
    // TODO: Export config to JSON
    console.log('Export config');
  }, []);

  const handleImport = useCallback(() => {
    // TODO: Import config from JSON
    console.log('Import config');
  }, []);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl h-[80vh] flex flex-col bg-slate-900 border-slate-700">
        <DialogHeader className="flex-shrink-0">
          <div className="flex items-center justify-between">
            <DialogTitle className="flex items-center gap-2 text-white">
              <Keyboard className="w-5 h-5" />
              Macropad Configuration
            </DialogTitle>
            <div className="flex items-center gap-2">
              <span className="text-xs text-emerald-400 flex items-center gap-1">
                <span className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                Work Louder Micro Connected
              </span>
            </div>
          </div>
        </DialogHeader>

        {/* Layer Tabs */}
        <div className="flex-shrink-0 border-b border-slate-700 pb-2">
          <Tabs value={String(activeLayer)} onValueChange={(v) => setActiveLayer(Number(v))}>
            <TabsList className="bg-slate-800">
              {LAYERS.map((layer) => (
                <TabsTrigger
                  key={layer.id}
                  value={String(layer.id)}
                  className="data-[state=active]:bg-slate-700"
                  style={{
                    borderBottomColor: activeLayer === layer.id ? layer.color : 'transparent',
                    borderBottomWidth: 2,
                  }}
                >
                  <span
                    className="w-2 h-2 rounded-full mr-2"
                    style={{ backgroundColor: layer.color }}
                  />
                  {layer.name}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
        </div>

        {/* 3D Visualization Area */}
        <div className="flex-1 flex gap-4 min-h-0">
          {/* 3D Model Container */}
          <div className="flex-1 bg-slate-950 rounded-lg border border-slate-700 relative">
            {/* TODO: Three.js 3D model here */}
            <div className="absolute inset-0 flex items-center justify-center">
              <div className="text-slate-500 text-center">
                <Keyboard className="w-16 h-16 mx-auto mb-4 opacity-50" />
                <p className="text-sm">3D Model Loading...</p>
                <p className="text-xs text-slate-600 mt-1">Click keys to view bindings</p>
              </div>
            </div>

            {/* Placeholder ASCII representation */}
            <pre className="absolute inset-0 flex items-center justify-center text-[10px] text-cyan-400/50 font-mono pointer-events-none">
{`
        ┌───────┐             ┌───────┐
        │   1   │  ┌───┐┌───┐ │   2   │
        │ Meta  │  │ K ││ ↑ │ │ Data  │
        └───────┘  └───┘└───┘ └───────┘

        ┌───┐┌───┐┌───┐┌───┐
        │ H ││ ← ││SPC││ L │
        └───┘└───┘└───┘└───┘

        ┌───┐┌───┐┌───┐┌───┐
        │ 3 ││ J ││ ↓ ││ 4 │
        │Ovr││   ││   ││Qry│
        └───┘└───┘└───┘└───┘

        ┌───────┐  ┌───┐┌───┐  ┌───────┐
        │  TAB  │  │ENT││ESC│  │  MO1  │
        └───────┘  └───┘└───┘  └───────┘

                    ◎ Encoder
`}
            </pre>
          </div>

          {/* Key Info Panel */}
          <div className="w-64 bg-slate-800 rounded-lg border border-slate-700 p-4 flex flex-col">
            <h3 className="text-sm font-medium text-white mb-3">Key Details</h3>

            {selectedKey ? (
              <div className="space-y-3">
                <div>
                  <label className="text-xs text-slate-400">Position</label>
                  <p className="text-sm text-white font-mono">{selectedKey}</p>
                </div>
                <div>
                  <label className="text-xs text-slate-400">Current Binding</label>
                  <p className="text-sm text-white">K (Up)</p>
                </div>
                <div>
                  <label className="text-xs text-slate-400">Action</label>
                  <p className="text-sm text-cyan-400">NAV_UP</p>
                </div>
              </div>
            ) : (
              <p className="text-xs text-slate-500">
                Click a key on the macropad to view its configuration.
              </p>
            )}

            <div className="mt-auto pt-4 border-t border-slate-700">
              <h4 className="text-xs text-slate-400 mb-2">Layer {activeLayer} Stats</h4>
              <div className="text-xs text-slate-500 space-y-1">
                <p>Keys bound: 16/16</p>
                <p>Encoder: Configured</p>
              </div>
            </div>
          </div>
        </div>

        {/* Footer Actions */}
        <div className="flex-shrink-0 flex items-center justify-between pt-4 border-t border-slate-700">
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="sm" className="text-slate-400">
              <HelpCircle className="w-4 h-4 mr-1" />
              Help
            </Button>
          </div>

          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={handleExport}>
              <Download className="w-4 h-4 mr-1" />
              Export
            </Button>
            <Button variant="outline" size="sm" onClick={handleImport}>
              <Upload className="w-4 h-4 mr-1" />
              Import
            </Button>
            <Button size="sm" className="bg-cyan-600 hover:bg-cyan-700">
              <Check className="w-4 h-4 mr-1" />
              Apply to Device
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
});

export default MacropadVisualizer;
```

**Step 2.3: Create index.ts**

```tsx
export { MacropadVisualizer } from './MacropadVisualizer';
export type { MacropadVisualizerProps } from './MacropadVisualizer';
```

**Step 2.4: Verify build**

```bash
pnpm type-check --filter=@novanet/studio
```

**Step 2.5: Commit**

```bash
git add apps/studio/src/components/macropad/
git commit -m "feat(studio): add MacropadVisualizer component shell

- Dialog-based modal with layer tabs
- Placeholder for 3D model
- Key details panel
- Export/Import buttons

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Add Keyboard Shortcut Hook

**Files:**
- Modify: `apps/studio/src/app/layout.tsx` or create `apps/studio/src/hooks/useMacropadShortcut.ts`
- Create: `apps/studio/src/stores/macropadStore.ts`

**Step 3.1: Create macropad store**

Create `apps/studio/src/stores/macropadStore.ts`:

```tsx
import { create } from 'zustand';

interface MacropadStore {
  isOpen: boolean;
  openMacropad: () => void;
  closeMacropad: () => void;
  toggleMacropad: () => void;
}

export const useMacropadStore = create<MacropadStore>((set) => ({
  isOpen: false,
  openMacropad: () => set({ isOpen: true }),
  closeMacropad: () => set({ isOpen: false }),
  toggleMacropad: () => set((state) => ({ isOpen: !state.isOpen })),
}));
```

**Step 3.2: Create keyboard shortcut hook**

Create `apps/studio/src/hooks/useMacropadShortcut.ts`:

```tsx
import { useEffect } from 'react';
import { useMacropadStore } from '@/stores/macropadStore';

/**
 * Hook to handle Cmd+K / Ctrl+K keyboard shortcut for macropad
 */
export function useMacropadShortcut() {
  const toggleMacropad = useMacropadStore((state) => state.toggleMacropad);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      // Cmd+K (Mac) or Ctrl+K (Windows/Linux)
      if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
        event.preventDefault();
        toggleMacropad();
      }
    }

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [toggleMacropad]);
}
```

**Step 3.3: Export hook**

Add to `apps/studio/src/hooks/index.ts`:

```tsx
export { useMacropadShortcut } from './useMacropadShortcut';
```

**Step 3.4: Integrate in layout**

Modify the main layout or App component to use the shortcut and render the modal.

**Step 3.5: Commit**

```bash
git add apps/studio/src/stores/macropadStore.ts apps/studio/src/hooks/useMacropadShortcut.ts
git commit -m "feat(studio): add Cmd+K shortcut for macropad visualizer

- macropadStore with Zustand
- useMacropadShortcut hook

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Add 3D Model with Three.js

**Files:**
- Create: `apps/studio/src/components/macropad/Macropad3D.tsx`
- Modify: `MacropadVisualizer.tsx`

**Step 4.1: Create Macropad3D component**

```tsx
'use client';

import { useRef, useCallback, Suspense } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls, RoundedBox, Text } from '@react-three/drei';
import * as THREE from 'three';

interface KeyProps {
  position: [number, number, number];
  size?: [number, number, number];
  label: string;
  sublabel?: string;
  color: string;
  isSelected: boolean;
  onClick: () => void;
}

function Key({ position, size = [1, 0.6, 1], label, sublabel, color, isSelected, onClick }: KeyProps) {
  const meshRef = useRef<THREE.Mesh>(null);

  useFrame(() => {
    if (meshRef.current) {
      // Subtle hover animation
      meshRef.current.scale.y = isSelected ? 0.8 : 1;
    }
  });

  return (
    <group position={position}>
      <RoundedBox
        ref={meshRef}
        args={size}
        radius={0.1}
        smoothness={4}
        onClick={onClick}
      >
        <meshStandardMaterial
          color={isSelected ? '#00FFFF' : color}
          emissive={isSelected ? '#00FFFF' : '#000000'}
          emissiveIntensity={isSelected ? 0.3 : 0}
        />
      </RoundedBox>
      <Text
        position={[0, 0.35, 0]}
        fontSize={0.3}
        color="white"
        anchorX="center"
        anchorY="middle"
      >
        {label}
      </Text>
      {sublabel && (
        <Text
          position={[0, 0.1, 0]}
          fontSize={0.15}
          color="#888888"
          anchorX="center"
          anchorY="middle"
        >
          {sublabel}
        </Text>
      )}
    </group>
  );
}

function Encoder({ position }: { position: [number, number, number] }) {
  const meshRef = useRef<THREE.Mesh>(null);

  useFrame((state) => {
    if (meshRef.current) {
      meshRef.current.rotation.y = state.clock.elapsedTime * 0.2;
    }
  });

  return (
    <group position={position}>
      <mesh ref={meshRef}>
        <cylinderGeometry args={[0.6, 0.6, 0.4, 32]} />
        <meshStandardMaterial color="#333333" metalness={0.8} roughness={0.2} />
      </mesh>
      {/* Knurling indicator lines */}
      {[...Array(12)].map((_, i) => (
        <mesh key={i} rotation={[0, (i * Math.PI * 2) / 12, 0]} position={[0.55, 0, 0]}>
          <boxGeometry args={[0.05, 0.3, 0.05]} />
          <meshStandardMaterial color="#555555" />
        </mesh>
      ))}
    </group>
  );
}

interface Macropad3DProps {
  layerColor: string;
  selectedKey: string | null;
  onKeyClick: (keyId: string) => void;
}

export function Macropad3D({ layerColor, selectedKey, onKeyClick }: Macropad3DProps) {
  // Key layout for Work Louder Micro (4x4 + encoder)
  const keys = [
    // Row 0
    { id: '0,0', pos: [-2.5, 0, -2], label: '1', sublabel: 'Meta' },
    { id: '0,1', pos: [-0.5, 0, -2], label: 'K', sublabel: '↑' },
    { id: '0,2', pos: [0.5, 0, -2], label: '↑' },
    { id: '0,3', pos: [2.5, 0, -2], label: '2', sublabel: 'Data' },
    // Row 1
    { id: '1,0', pos: [-2.5, 0, -0.5], label: 'H', sublabel: '←' },
    { id: '1,1', pos: [-0.5, 0, -0.5], label: '←' },
    { id: '1,2', pos: [0.5, 0, -0.5], label: 'SPC' },
    { id: '1,3', pos: [2.5, 0, -0.5], label: 'L', sublabel: '→' },
    // Row 2
    { id: '2,0', pos: [-2.5, 0, 1], label: '3', sublabel: 'Ovr' },
    { id: '2,1', pos: [-0.5, 0, 1], label: 'J', sublabel: '↓' },
    { id: '2,2', pos: [0.5, 0, 1], label: '↓' },
    { id: '2,3', pos: [2.5, 0, 1], label: '4', sublabel: 'Qry' },
    // Row 3
    { id: '3,0', pos: [-2.5, 0, 2.5], label: 'TAB', size: [1.5, 0.6, 1] as [number, number, number] },
    { id: '3,1', pos: [-0.5, 0, 2.5], label: 'ENT' },
    { id: '3,2', pos: [0.5, 0, 2.5], label: 'ESC' },
    { id: '3,3', pos: [2.5, 0, 2.5], label: 'MO1', size: [1.5, 0.6, 1] as [number, number, number] },
  ];

  return (
    <Canvas camera={{ position: [0, 8, 8], fov: 50 }}>
      <color attach="background" args={['#0a0a0f']} />
      <ambientLight intensity={0.4} />
      <pointLight position={[10, 10, 10]} intensity={1} />
      <pointLight position={[-10, 10, -10]} intensity={0.5} color={layerColor} />

      {/* Base plate */}
      <RoundedBox args={[8, 0.5, 8]} position={[0, -0.5, 0]} radius={0.3}>
        <meshStandardMaterial color="#1a1a1a" metalness={0.3} roughness={0.7} />
      </RoundedBox>

      {/* Keys */}
      {keys.map((key) => (
        <Key
          key={key.id}
          position={key.pos as [number, number, number]}
          size={key.size}
          label={key.label}
          sublabel={key.sublabel}
          color="#2a2a2a"
          isSelected={selectedKey === key.id}
          onClick={() => onKeyClick(key.id)}
        />
      ))}

      {/* Encoder */}
      <Encoder position={[0, 0.3, 4.5]} />

      {/* RGB underglow */}
      <mesh position={[0, -0.7, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[7.5, 7.5]} />
        <meshBasicMaterial color={layerColor} transparent opacity={0.3} />
      </mesh>

      <OrbitControls
        enablePan={false}
        minDistance={8}
        maxDistance={20}
        minPolarAngle={Math.PI * 0.1}
        maxPolarAngle={Math.PI * 0.45}
      />
    </Canvas>
  );
}

export default Macropad3D;
```

**Step 4.2: Integrate in MacropadVisualizer**

Replace the placeholder with:

```tsx
import dynamic from 'next/dynamic';

const Macropad3D = dynamic(() => import('./Macropad3D'), {
  ssr: false,
  loading: () => (
    <div className="flex items-center justify-center h-full">
      <div className="animate-spin w-8 h-8 border-2 border-cyan-400 border-t-transparent rounded-full" />
    </div>
  ),
});

// In the render, replace the placeholder:
<Macropad3D
  layerColor={LAYERS[activeLayer].color}
  selectedKey={selectedKey}
  onKeyClick={handleKeyClick}
/>
```

**Step 4.3: Commit**

```bash
git add apps/studio/src/components/macropad/Macropad3D.tsx
git commit -m "feat(studio): add 3D macropad model with Three.js

- Interactive key meshes with click handlers
- Rotating encoder knob
- RGB underglow per layer
- Orbit controls with constraints

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Wire Up to Main App

**Files:**
- Modify: `apps/studio/src/app/page.tsx` or main layout

**Step 5.1: Import and render MacropadVisualizer**

Add to main page/layout:

```tsx
import { MacropadVisualizer } from '@/components/macropad';
import { useMacropadStore } from '@/stores/macropadStore';
import { useMacropadShortcut } from '@/hooks';

// In component:
useMacropadShortcut();
const { isOpen, closeMacropad } = useMacropadStore();

// In render:
<MacropadVisualizer open={isOpen} onOpenChange={closeMacropad} />
```

**Step 5.2: Add keyboard shortcut hint in UI**

Add a small hint in the bottom-left stats area:

```tsx
<span className="text-xs text-slate-500">
  <kbd className="px-1 py-0.5 bg-slate-800 rounded text-[10px]">⌘K</kbd> Macropad
</span>
```

**Step 5.3: Verify everything works**

1. Run `pnpm dev`
2. Press `Cmd+K` / `Ctrl+K`
3. Modal should open with 3D macropad
4. Click keys to select
5. Switch layers with tabs

**Step 5.4: Final commit**

```bash
git add -A
git commit -m "feat(studio): complete macropad 3D visualizer integration

- Cmd+K shortcut opens modal
- 3D model with clickable keys
- Layer tabs with RGB colors
- Connected to main layout

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Summary

| Task | Feature | Files |
|------|---------|-------|
| 1 | Config folder structure | `~/Projects/work-louder/studio-integration/` |
| 2 | MacropadVisualizer component | `components/macropad/MacropadVisualizer.tsx` |
| 3 | Keyboard shortcut | `stores/macropadStore.ts`, `hooks/useMacropadShortcut.ts` |
| 4 | 3D model with Three.js | `components/macropad/Macropad3D.tsx` |
| 5 | Wire up to app | `app/page.tsx` integration |

---

## Dependencies

Already installed in Studio:
- `three`
- `@react-three/fiber`
- `@react-three/drei`
- `zustand`

---

## Future Enhancements

- [ ] WebHID device connection (read real keymaps)
- [ ] Live keymap editing
- [ ] QMK firmware compilation
- [ ] Multiple device support
- [ ] Profile presets
