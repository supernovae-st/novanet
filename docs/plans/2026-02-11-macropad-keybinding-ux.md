# Macropad Key Binding UX Design

**Date:** 2026-02-11
**Status:** Approved
**Author:** Thibaut + Claude

## Overview

Console-style controller remapping UX for the Work Louder Micro macropad, with real-time 3D feedback and WebHID integration.

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Edit trigger | Hybrid (presets + capture) | Best of both: quick presets + full customization |
| UI paradigm | Action-centric | Users think "Navigate Down" not "KC_J" |
| Edit location | Drawer (bottom) | More immersive, gaming-style |
| Feedback | 3D label billboard | Direct visual on the key itself |
| Physical input | Mirror in 3D | Press physical key → 3D animates |

## User Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│  1. User clicks key on 3D pad                                       │
│              ↓                                                      │
│  2. Key glows + floating label appears below                        │
│     ╭─────────────╮                                                 │
│     │ Navigate ↓  │                                                 │
│     │   [Edit]    │                                                 │
│     ╰─────────────╯                                                 │
│              ↓                                                      │
│  3. User clicks [Edit] → Drawer slides up from bottom               │
│              ↓                                                      │
│  4. User either:                                                    │
│     a) Clicks preset action → Bound immediately                     │
│     b) Clicks "Press a key..." → Listening mode                     │
│     c) Presses physical pad key → Captured + bound                  │
│              ↓                                                      │
│  5. Success flash (green) + drawer closes + auto-save               │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Architecture

```
MacropadVisualizer.tsx (orchestrator)
├── SuperNovaePad3D.tsx
│   ├── Keycap components
│   └── KeyLabel3D.tsx (NEW - billboard text)
├── ActionPickerDrawer.tsx (NEW)
│   ├── ActionGrid (presets by category)
│   └── CaptureButton ("Press a key...")
└── useWebHID.ts (modified - onKeyPress callback)
```

## New Files

### 1. ActionPickerDrawer.tsx

Drawer component that slides up from bottom when editing a key.

```typescript
interface ActionPickerDrawerProps {
  isOpen: boolean;
  selectedKey: string | null;
  currentAction: string | null;
  onSelectAction: (action: ActionPreset) => void;
  onStartCapture: () => void;
  onClose: () => void;
  isCapturing: boolean;
}
```

**Layout:**
- Header: "What should this key do?" + close button
- Grid sections: Navigation, Modes, Scroll, Actions (color-coded)
- Capture button: "Press a key on your pad..."
- Footer: [Clear] [Cancel]

### 2. KeyLabel3D.tsx

Three.js billboard text component that floats below selected key.

```typescript
interface KeyLabel3DProps {
  position: [number, number, number];
  label: string;
  sublabel?: string; // keycode
  isEditing: boolean;
  onEditClick: () => void;
}
```

**Behavior:**
- Always faces camera (billboard)
- Shows current action label + small keycode
- [Edit] button appears on hover/select
- Preview mode: shows ghost label when hovering presets

### 3. actions.ts

Action presets with semantic labels.

```typescript
export interface ActionPreset {
  id: string;
  category: 'navigation' | 'modes' | 'scroll' | 'actions' | 'system';
  label: string;        // "Navigate Down"
  shortLabel: string;   // "↓ Down"
  key: string;          // "J"
  keycode: string;      // "KC_J"
  icon: string;         // "ArrowDown"
  color: string;        // category color
}

export const ACTION_PRESETS: Record<string, ActionPreset[]> = {
  navigation: [
    { id: 'nav-up', category: 'navigation', label: 'Navigate Up', shortLabel: '↑ Up', key: 'K', keycode: 'KC_K', icon: 'ArrowUp', color: '#00FFFF' },
    { id: 'nav-down', category: 'navigation', label: 'Navigate Down', shortLabel: '↓ Down', key: 'J', keycode: 'KC_J', icon: 'ArrowDown', color: '#00FFFF' },
    { id: 'nav-left', category: 'navigation', label: 'Collapse/Back', shortLabel: '← Back', key: 'H', keycode: 'KC_H', icon: 'ArrowLeft', color: '#00FFFF' },
    { id: 'nav-right', category: 'navigation', label: 'Expand/Enter', shortLabel: '→ Enter', key: 'L', keycode: 'KC_L', icon: 'ArrowRight', color: '#00FFFF' },
    { id: 'nav-toggle', category: 'navigation', label: 'Toggle', shortLabel: '␣ Toggle', key: 'SPACE', keycode: 'KC_SPC', icon: 'ToggleLeft', color: '#00FFFF' },
  ],
  modes: [
    { id: 'mode-meta', category: 'modes', label: 'Meta Mode', shortLabel: 'META', key: '1', keycode: 'KC_1', icon: 'Database', color: '#9945FF' },
    { id: 'mode-data', category: 'modes', label: 'Data Mode', shortLabel: 'DATA', key: '2', keycode: 'KC_2', icon: 'FileText', color: '#9945FF' },
    { id: 'mode-overlay', category: 'modes', label: 'Overlay Mode', shortLabel: 'OVERLAY', key: '3', keycode: 'KC_3', icon: 'Layers', color: '#9945FF' },
    { id: 'mode-query', category: 'modes', label: 'Query Mode', shortLabel: 'QUERY', key: '4', keycode: 'KC_4', icon: 'Search', color: '#9945FF' },
  ],
  scroll: [
    { id: 'scroll-page-up', category: 'scroll', label: 'Page Up', shortLabel: 'PgUp', key: 'U', keycode: 'KC_U', icon: 'ChevronsUp', color: '#22C55E' },
    { id: 'scroll-page-down', category: 'scroll', label: 'Page Down', shortLabel: 'PgDn', key: 'D', keycode: 'KC_D', icon: 'ChevronsDown', color: '#22C55E' },
    { id: 'scroll-top', category: 'scroll', label: 'Go to Top', shortLabel: 'Top', key: 'g', keycode: 'KC_G', icon: 'ArrowUpToLine', color: '#22C55E' },
    { id: 'scroll-bottom', category: 'scroll', label: 'Go to Bottom', shortLabel: 'Bottom', key: 'G', keycode: 'S(KC_G)', icon: 'ArrowDownToLine', color: '#22C55E' },
  ],
  actions: [
    { id: 'action-search', category: 'actions', label: 'Search', shortLabel: 'Search', key: '/', keycode: 'KC_SLSH', icon: 'Search', color: '#F97316' },
    { id: 'action-help', category: 'actions', label: 'Help', shortLabel: 'Help', key: '?', keycode: 'S(KC_SLSH)', icon: 'HelpCircle', color: '#F97316' },
    { id: 'action-refresh', category: 'actions', label: 'Refresh', shortLabel: 'Refresh', key: 'R', keycode: 'KC_R', icon: 'RefreshCw', color: '#F97316' },
    { id: 'action-quit', category: 'actions', label: 'Quit', shortLabel: 'Quit', key: 'Q', keycode: 'KC_Q', icon: 'X', color: '#F97316' },
  ],
};

export const CATEGORY_COLORS = {
  navigation: '#00FFFF',
  modes: '#9945FF',
  scroll: '#22C55E',
  actions: '#F97316',
  system: '#FF4545',
};
```

## Modified Files

### SuperNovaePad3D.tsx

New props:

```typescript
interface SuperNovaePad3DProps {
  // existing...
  pressedKeys?: Set<string>;        // Physical keys currently pressed
  previewLabel?: string | null;     // Ghost label when hovering preset
  showLabels?: boolean;             // Show labels on all keys
  editingKey?: string | null;       // Key currently being edited
  onEditKey?: (keyId: string) => void;
}
```

### useWebHID.ts

Add key press callback:

```typescript
interface UseWebHIDReturn {
  // existing...
  onKeyPress?: (callback: (row: number, col: number, pressed: boolean) => void) => void;
}
```

### MacropadVisualizer.tsx

New state for edit mode:

```typescript
const [editMode, setEditMode] = useState(false);
const [editingKey, setEditingKey] = useState<string | null>(null);
const [isCapturing, setIsCapturing] = useState(false);
const [previewAction, setPreviewAction] = useState<string | null>(null);
const [pressedKeys, setPressedKeys] = useState<Set<string>>(new Set());
```

## Interaction States

### 3D Key States

| State | Visual |
|-------|--------|
| idle | Normal color, no label |
| hovered | Slight glow, label appears on hover |
| selected | Cyan glow + floating label + bounce |
| editing | Bright glow + label with [Edit] button |
| preview | Ghost label showing hovered preset |
| pressed | Y position -= 0.08, press animation |
| bound | Green flash ✓ for 600ms |

### Drawer States

| State | Visual |
|-------|--------|
| closed | Hidden below viewport |
| open | Slide up 200ms ease-out, backdrop blur |
| capturing | "Press a key..." button pulses cyan |
| success | Flash green, auto-close after 400ms |

## WebHID Integration

When physical pad is connected:

1. User presses key on physical pad
2. `useWebHID.onKeyPress(row, col, true)` fires
3. `pressedKeys.add(\`${row},${col}\`)`
4. 3D key animates (press down)
5. If `isCapturing`:
   - Capture the key position
   - Map to current layer's binding
   - Show success feedback
   - Close drawer

When no pad connected (preview mode):
- Click 3D key to select
- Use keyboard to capture (if in capture mode)
- Or click presets

## Animations

```typescript
// Key press animation (useFrame)
const pressDepth = 0.08;
const pressSpeed = 20; // lerp factor

// Label appear
const labelAnimation = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  transition: { duration: 0.2 }
};

// Drawer slide
const drawerAnimation = {
  initial: { y: '100%' },
  animate: { y: 0 },
  transition: { duration: 0.2, ease: 'easeOut' }
};

// Success flash
const successAnimation = {
  scale: [1, 1.1, 1],
  backgroundColor: ['current', '#22C55E', 'current'],
  transition: { duration: 0.6 }
};
```

## File Structure

```
apps/studio/src/components/macropad/
├── MacropadVisualizer.tsx      (modified)
├── SuperNovaePad3D.tsx         (modified)
├── KeyLabel3D.tsx              (NEW)
├── ActionPickerDrawer.tsx      (NEW)
├── MacropadTutorial.tsx        (existing)
└── CreatorBoardLowPoly.tsx     (existing)

apps/studio/src/config/
├── keybindings.ts              (existing)
└── actions.ts                  (NEW)

apps/studio/src/hooks/
└── useWebHID.ts                (modified)
```

## Success Criteria

- [ ] Click key → floating label appears
- [ ] Click [Edit] → drawer slides up
- [ ] Click preset → binding updates + success flash
- [ ] "Press a key" → capture mode works
- [ ] Physical pad press → 3D mirrors + captures
- [ ] Hover preset → ghost label preview on 3D key
- [ ] Escape closes drawer
- [ ] Auto-save after binding change
