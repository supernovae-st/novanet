---
name: novanet-design-system
description: NovaNet colors, typography, glass morphism, and node styling. Use when styling components or graph nodes.
user-invocable: false
---

# NovaNet Design System

> Dark-first, glass morphism, graph visualization optimized

## Design Principles

1. **Dark First** - Black background (`#000`) optimized for graph visualization
2. **Glass Morphism** - Blur + transparency for depth without distraction
3. **Node Colors** - Semantic colors for 33 graph node types
4. **DX First** - Every property is copyable, every action has a shortcut

---

## Color System

### Brand Colors (novanet-*)

```css
/* Sky blue palette */
--novanet-50: #f0f9ff;
--novanet-100: #e0f2fe;
--novanet-200: #bae6fd;
--novanet-300: #7dd3fc;
--novanet-400: #38bdf8;
--novanet-500: #0ea5e9;  /* Primary */
--novanet-600: #0284c7;
--novanet-700: #0369a1;
--novanet-800: #075985;
--novanet-900: #0c4a6e;
--novanet-950: #082f49;
```

### Node Type Colors

```typescript
// Semantic colors for graph node categories
const NODE_COLORS = {
  // Project category (violet)
  Project: '#8b5cf6',
  BrandIdentity: '#a78bfa',
  Audience: '#8b5cf6',

  // Content category (blue/cyan)
  Page: '#3b82f6',
  Block: '#06b6d4',
  BlockType: '#14b8a6',

  // Entity category (amber)
  Entity: '#f59e0b',
  EntityL10n: '#fbbf24',

  // Locale category (emerald)
  Locale: '#10b981',
  LocaleIdentity: '#22c55e',
  LocaleVoice: '#22c55e',
  LocaleCulture: '#22c55e',
  LocaleMarket: '#22c55e',
  LocaleLexicon: '#22c55e',
  Expression: '#ec4899',

  // Generation (indigo)
  PagePrompt: '#6366f1',
  BlockPrompt: '#818cf8',
  PageOutput: '#a5b4fc',
  BlockOutput: '#c7d2fe',

  // SEO (orange)
  SEOKeyword: '#f97316',
  SEOVariation: '#fb923c',

  // GEO (rose)
  GEOSeed: '#f43f5e',
  GEOReformulation: '#fb7185',
};
```

### Surface Levels

```css
/* Background hierarchy */
--surface-0: #000000;              /* Base background */
--surface-1: hsl(240, 8%, 5%);     /* Card default */
--surface-2: hsl(240, 6%, 8%);     /* Card elevated */
--surface-3: hsl(240, 5%, 12%);    /* Card hover */

/* Usage in Tailwind */
.surface-0 { background-color: #000000; }
.surface-1 { background-color: hsl(240, 8%, 5%); }
.surface-2 { background-color: hsl(240, 6%, 8%); }
.surface-3 { background-color: hsl(240, 5%, 12%); }
```

### Border & Transparency

```css
/* White with opacity for borders */
--border-subtle: rgba(255, 255, 255, 0.06);  /* 6% */
--border-default: rgba(255, 255, 255, 0.12); /* 12% */
--border-strong: rgba(255, 255, 255, 0.18);  /* 18% */
--border-focus: rgba(255, 255, 255, 0.25);   /* 25% */

/* Tailwind usage */
border-white/6   /* Subtle dividers */
border-white/12  /* Default borders */
border-white/18  /* Hover state */
border-white/25  /* Focus state */
```

### Text Colors

```css
/* White with opacity for text */
--text-primary: #ffffff;           /* 100% - Headings */
--text-secondary: rgba(255, 255, 255, 0.65);  /* 65% - Body */
--text-muted: rgba(255, 255, 255, 0.40);      /* 40% - Hints */
--text-disabled: rgba(255, 255, 255, 0.25);   /* 25% - Disabled */

/* Tailwind usage */
text-white        /* Primary */
text-white/65     /* Secondary */
text-white/40     /* Muted */
text-white/25     /* Disabled */
```

---

## Glass Morphism

### Glass Card

```typescript
const cardVariants = cva('rounded-xl border transition-all', {
  variants: {
    variant: {
      // Default surface-1
      default:
        'bg-[hsl(240,8%,5%)] border-white/12 hover:bg-[hsl(240,6%,8%)] hover:border-white/18',

      // Glass with backdrop blur
      glass:
        'bg-[hsl(240,6%,8%)] backdrop-blur-xl border-white/12 shadow-lg shadow-black/40',

      // Floating premium card
      floating:
        'bg-[hsl(240,6%,8%)] backdrop-blur-xl border-white/15 rounded-2xl shadow-2xl shadow-black/50 ring-1 ring-white/4 ring-inset',

      // Interactive with hover feedback
      interactive:
        'bg-[hsl(240,8%,5%)] border-white/12 hover:bg-[hsl(240,5%,12%)] hover:border-white/20 hover:shadow-lg hover:shadow-black/40 cursor-pointer',
    },
  },
});
```

### Glass Gradient

```css
.glass-gradient {
  background: linear-gradient(
    135deg,
    rgba(255, 255, 255, 0.1) 0%,
    rgba(255, 255, 255, 0.05) 100%
  );
}
```

---

## Typography

### Font Family

```css
/* Geist font (variable) */
--font-geist-sans: 'Geist', system-ui, sans-serif;
--font-geist-mono: 'Geist Mono', 'SF Mono', monospace;
```

### Type Scale

```css
/* Headings */
.text-xl  { font-size: 1.25rem; }   /* 20px - Section title */
.text-lg  { font-size: 1.125rem; }  /* 18px - Card title */
.text-base { font-size: 1rem; }     /* 16px - Body */
.text-sm  { font-size: 0.875rem; }  /* 14px - UI elements */
.text-xs  { font-size: 0.75rem; }   /* 12px - Captions */

/* Font weights */
.font-semibold { font-weight: 600; }  /* Headings */
.font-medium { font-weight: 500; }    /* UI elements */
.font-normal { font-weight: 400; }    /* Body text */
```

---

## Spacing & Layout

### Spacing Scale

```css
/* 4px base unit */
--space-1: 0.25rem;   /* 4px */
--space-2: 0.5rem;    /* 8px */
--space-3: 0.75rem;   /* 12px */
--space-4: 1rem;      /* 16px */
--space-5: 1.25rem;   /* 20px */
--space-6: 1.5rem;    /* 24px */
--space-8: 2rem;      /* 32px */
```

### Border Radius

```css
--radius-sm: calc(var(--radius) - 4px);  /* 4px - Small elements */
--radius-md: calc(var(--radius) - 2px);  /* 6px - Buttons */
--radius-lg: var(--radius);              /* 8px - Cards */
--radius-xl: 0.75rem;                    /* 12px - Large cards */
--radius-2xl: 1rem;                      /* 16px - Modals */
```

---

## Component Patterns

### Card

```tsx
<Card variant="glass">
  <CardHeader>
    <CardTitle>Node Details</CardTitle>
    <CardDescription>View and edit node properties</CardDescription>
  </CardHeader>
  <CardContent>
    {/* Content */}
  </CardContent>
</Card>
```

### Button

```tsx
// Primary action
<Button variant="default">Save Changes</Button>

// Secondary action
<Button variant="outline">Cancel</Button>

// Destructive
<Button variant="destructive">Delete</Button>

// Ghost (icon buttons)
<Button variant="ghost" size="icon">
  <CopyIcon />
</Button>
```

### Input

```tsx
<div className="space-y-2">
  <label className="text-sm text-white/65">Label</label>
  <input
    className={cn(
      'w-full rounded-lg border border-white/12 bg-white/5',
      'px-3 py-2 text-sm text-white',
      'placeholder:text-white/40',
      'focus:border-novanet-500 focus:outline-none focus:ring-1 focus:ring-novanet-500',
      'disabled:opacity-50'
    )}
    placeholder="Enter value..."
  />
</div>
```

---

## Graph Node Styling

### React Flow Custom Node

```tsx
export function EntityNode({ data, selected }: NodeProps) {
  return (
    <div
      className={cn(
        'px-4 py-2 rounded-lg border-2 transition-all',
        'bg-[hsl(240,8%,5%)]',
        selected
          ? 'border-amber-500 shadow-lg shadow-amber-500/20'
          : 'border-amber-500/50 hover:border-amber-500'
      )}
    >
      <Handle type="target" position={Position.Top} />
      <div className="flex items-center gap-2">
        <div
          className="w-3 h-3 rounded-full"
          style={{ backgroundColor: NODE_COLORS.Entity }}
        />
        <span className="text-sm font-medium text-white">{data.label}</span>
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}
```

### Node Badge

```tsx
export function NodeBadge({ label }: { label: string }) {
  const color = NODE_COLORS[label] || '#71717a';

  return (
    <span
      className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium"
      style={{
        backgroundColor: `${color}20`,  // 12% opacity
        color: color,
        border: `1px solid ${color}40`, // 25% opacity
      }}
    >
      <span
        className="w-2 h-2 rounded-full"
        style={{ backgroundColor: color }}
      />
      {label}
    </span>
  );
}
```

---

## Animation

### Keyframes

```css
@keyframes glow {
  0% { box-shadow: 0 0 5px currentColor, 0 0 10px currentColor; }
  100% { box-shadow: 0 0 10px currentColor, 0 0 20px currentColor, 0 0 30px currentColor; }
}

@keyframes shake {
  10%, 90% { transform: translate3d(-1px, 0, 0); }
  20%, 80% { transform: translate3d(2px, 0, 0); }
  30%, 50%, 70% { transform: translate3d(-4px, 0, 0); }
  40%, 60% { transform: translate3d(4px, 0, 0); }
}

@keyframes toast-enter {
  0% { opacity: 0; transform: translateY(10px) scale(0.95); }
  100% { opacity: 1; transform: translateY(0) scale(1); }
}
```

### Tailwind Classes

```css
.animate-pulse-slow { animation: pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
.animate-glow { animation: glow 2s ease-in-out infinite alternate; }
.animate-shake { animation: shake 0.5s cubic-bezier(.36,.07,.19,.97) both; }
.animate-toast-enter { animation: toast-enter 0.3s ease-out; }
```

---

## Icons

### Lucide React

```tsx
import { Copy, Check, ChevronDown, Search, X } from 'lucide-react';

// Standard size
<CopyIcon className="h-4 w-4" />

// With color
<CheckIcon className="h-4 w-4 text-green-500" />

// In button
<Button variant="ghost" size="icon">
  <SearchIcon className="h-4 w-4" />
</Button>
```

### Category Icons

```tsx
const CATEGORY_ICONS = {
  project: Building2,
  content: FileText,
  locale: Globe,
  generation: Sparkles,
  seo: Search,
  geo: MapPin,
  analytics: BarChart3,
};
```

---

## Dark Mode Considerations

### Images

```tsx
// Invert light logos
<img src="/logo.svg" className="dark:invert" alt="Logo" />

// Or use dark variant
<picture>
  <source srcSet="/logo-dark.svg" media="(prefers-color-scheme: dark)" />
  <img src="/logo-light.svg" alt="Logo" />
</picture>
```

### Code Blocks

```tsx
// Use dark theme for code highlighting
import { Highlight, themes } from 'prism-react-renderer';

<Highlight theme={themes.nightOwl} code={code} language="typescript">
  {/* ... */}
</Highlight>
```

---

## Checklist

### Colors

- [ ] Use `novanet-500` for primary actions
- [ ] Use semantic node colors consistently
- [ ] Maintain contrast ratios (4.5:1 for text)
- [ ] Use white/opacity for text hierarchy

### Components

- [ ] All cards use surface levels
- [ ] Buttons have hover/focus states
- [ ] Inputs have focus rings
- [ ] Modals have backdrop blur

### Graph

- [ ] Node colors match category
- [ ] Selected state is visible
- [ ] Edges use appropriate opacity
- [ ] Labels are readable on dark bg

### Motion

- [ ] Respect prefers-reduced-motion
- [ ] Keep animations subtle (<300ms)
- [ ] Use ease-out for entrances
- [ ] Use ease-in for exits
