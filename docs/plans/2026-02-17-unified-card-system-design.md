# NovaNet Studio - Unified Card System Design

**Version**: v0.13.1
**Date**: 2026-02-17
**Status**: Design Phase - Research Complete
**Research Sources**: 21st.dev components, Framer Motion, Context7 docs

---

## Executive Summary

This document synthesizes 10 parallel brainstorming sessions and enhanced research into a unified card design system for NovaNet Studio's 3-level node architecture:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  3-LEVEL NODE ARCHITECTURE                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  LEVEL 1: TAXONOMY (21 nodes)                                                   │
│  ├── 2 Realms (shared, org)                                                     │
│  ├── 10 Layers (config, locale, geography, knowledge, foundation...)            │
│  ├── 5 Traits (defined, authored, imported, generated, retrieved)               │
│  └── 5 Arc Families (ownership, localization, semantic, generation, mining)     │
│                                                                                 │
│  LEVEL 2: SCHEMA (239 nodes)                                                    │
│  ├── 61 NodeClass definitions                                                   │
│  └── 178 ArcClass definitions                                                   │
│                                                                                 │
│  LEVEL 3: DATA (∞ instances)                                                    │
│  └── Runtime instances (Page, Entity, Block, Locale, etc.)                      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Part 1: Visual Encoding System (ADR-005)

### Color Channels

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VISUAL ENCODING CHANNELS                                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Fill Color    → Layer (10 distinct colors)                                     │
│  Border Color  → Realm (shared=#2aa198 cyan, org=#6c71c4 violet)                │
│  Border Style  → Trait (see below)                                              │
│  Arc Stroke    → ArcFamily (5 families)                                         │
│  Glow/Effects  → Selection State + Hover                                        │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  TRAIT BORDER STYLES                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  defined     ████████████  solid 2px       (human creates ONCE)                 │
│  authored    ▬ ▬ ▬ ▬ ▬ ▬  dashed 2px      (human writes PER locale)            │
│  imported    ═══════════  double 3px      (external data)                       │
│  generated   ............  dotted 2px      (LLM produces)                       │
│  retrieved   · · · · · ·  dotted 1px thin (external APIs)                       │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  LAYER COLORS (from taxonomy.yaml)                                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  config       #64748b  slate-500     ●                                          │
│  locale       #22c55e  green-500     ●                                          │
│  geography    #f97316  orange-500    ●                                          │
│  knowledge    #eab308  yellow-500    ●                                          │
│  foundation   #8b5cf6  violet-500    ●                                          │
│  structure    #3b82f6  blue-500      ●                                          │
│  semantic     #06b6d4  cyan-500      ●                                          │
│  instruction  #ec4899  pink-500      ●                                          │
│  output       #10b981  emerald-500   ●                                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Part 2: Card Architecture (Enhanced)

### Base Component System

```typescript
// ═══════════════════════════════════════════════════════════════════════════════
// CARD ARCHITECTURE: Shell + Content Pattern (Render Props)
// Enhanced with Framer Motion integration
// ═══════════════════════════════════════════════════════════════════════════════

import { motion, AnimatePresence, type Variants } from 'framer-motion';

interface CardContext {
  colors: { primary: string; secondary: string; accent?: string };
  selected: boolean;
  isHovered: boolean;
  width?: number;
  performanceTier: PerformanceTier;
}

interface CardShellProps {
  data: BaseNodeData;
  children: (context: CardContext) => React.ReactNode;
  width?: number;
  performanceTier?: PerformanceTier;
  variants?: CardVariants;
}

// Animation variants for consistent motion design
const cardVariants: Variants = {
  hidden: { opacity: 0, scale: 0.95, y: 10 },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: {
      type: 'spring',
      stiffness: 500,
      damping: 30,
      mass: 1
    }
  },
  selected: {
    scale: 1.02,
    transition: { duration: 0.2, ease: [0.175, 0.885, 0.32, 2.2] }
  },
  hover: {
    y: -2,
    transition: { duration: 0.2 }
  }
};
```

### Enhanced Shared Base Components

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SHARED CARD COMPONENTS (Enhanced with 21st.dev patterns)                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  CardHeader (ShineBorder pattern)                                               │
│  ├── icon (from visual-encoding.yaml) + glow effect                             │
│  ├── label (CLASS, ARC, REALM, etc.) + gradient shimmer                         │
│  ├── statusIndicator (animated pulse dot)                                       │
│  └── Optional: Badge with AnimatedBeam                                          │
│                                                                                 │
│  CardBody (BentoGrid inspired)                                                  │
│  ├── title (displayName) + text-shadow glow                                     │
│  ├── subtitle (optional) + typewriter effect                                    │
│  ├── MetadataGrid (3-column pill layout)                                        │
│  └── children (variant-specific content)                                        │
│                                                                                 │
│  CardFooter (Gauge/Badge patterns)                                              │
│  ├── badges (realm, layer, trait) with hover tooltips                           │
│  ├── metrics (propCount, childCount) with animated counters                     │
│  └── progress indicators (coverage bars)                                        │
│                                                                                 │
│  CardEffects (GPU-accelerated transforms)                                       │
│  ├── TechCorners (SVG with drop-shadow glow)                                    │
│  ├── Scanlines (CSS animation, reduced motion support)                          │
│  ├── GridPattern (SVG pattern)                                                  │
│  ├── HolographicShimmer (gradient animation)                                    │
│  ├── MatrixRain (canvas-based for performance)                                  │
│  ├── OuterGlow (box-shadow animation)                                           │
│  ├── GlassmorphicBlur (backdrop-filter)                                         │
│  └── NeonBorderGlow (animated border gradient)                                  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Part 3: Level 1 - Taxonomy Cards (21 nodes) - ENHANCED

### TaxonomyNode - 4 Variants with Unique Effects

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TAXONOMY NODE VARIANTS - Premium "Classification" Design                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌──────────────────────┐  ┌──────────────────────┐                             │
│  │ ◉ REALM              │  │ ◈ LAYER              │                             │
│  │ ═══════════════════  │  │ ═══════════════════  │                             │
│  │      ╭───╮           │  │    ┌───┐             │                             │
│  │    ╭─┤ ○ ├─╮         │  │    │ ▬ │ ← Plane 1   │   ← Stacked 3D planes       │
│  │    │ ╰───╯ │         │  │    ├───┤             │     with parallax           │
│  │    ╰───────╯         │  │    │ ▬ │ ← Plane 2   │                             │
│  │      orbits          │  │    ├───┤             │                             │
│  │                      │  │    │ ▬ │ ← Plane 3   │                             │
│  │  shared              │  │    └───┘             │                             │
│  │  ● cyan #2aa198      │  │  semantic            │                             │
│  │                      │  │  ● cyan-500          │                             │
│  │  [40 nodes]          │  │  [4 nodes]           │                             │
│  └──────────────────────┘  └──────────────────────┘                             │
│                                                                                 │
│  ┌──────────────────────┐  ┌──────────────────────┐                             │
│  │ ◆ TRAIT              │  │ → ARC FAMILY         │                             │
│  │ ═══════════════════  │  │ ═══════════════════  │                             │
│  │ ┌────────────────┐   │  │                      │                             │
│  │ │ ████ solid     │ ← │  │    ○ ──→── ○        │   ← Animated particles      │
│  │ │ ▬ ▬ dashed    │   │  │      ∿∿∿∿∿∿          │     flowing outward         │
│  │ │ ═══ double    │   │  │    ↗   ↑   ↖         │                             │
│  │ │ ··· dotted    │   │  │  ○ ← ─ ● ─ → ○       │                             │
│  │ └────────────────┘   │  │    ↙   ↓   ↘         │                             │
│  │ Morphing preview     │  │  ownership           │                             │
│  │                      │  │  ━━━ solid           │                             │
│  │  defined             │  │                      │                             │
│  │  ▬▬▬▬▬ solid         │  │  [43 arcs]           │                             │
│  │  [31 nodes]          │  └──────────────────────┘                             │
│  └──────────────────────┘                                                       │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  UNIQUE EFFECTS PER VARIANT (GPU-accelerated)                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Realm:      OrbitalRings                                                       │
│              - 2-3 concentric ellipses with rotation                            │
│              - Pulse animation on hover                                         │
│              - CSS: transform-style: preserve-3d                                │
│                                                                                 │
│  Layer:      StackedPlanes                                                      │
│              - 3 translucent rectangles with Z-depth                            │
│              - Parallax effect on mouse move                                    │
│              - Framer Motion: useMotionValue + useTransform                     │
│                                                                                 │
│  Trait:      BorderMorph                                                        │
│              - Animated border cycling through all 5 styles                     │
│              - CSS clip-path animation                                          │
│              - Reduced motion: static sample                                    │
│                                                                                 │
│  ArcFamily:  RadiatingPulse                                                     │
│              - Central node with 4-6 outward arrows                             │
│              - Particle flow effect (offset-path)                               │
│              - SVG with animate/animateMotion                                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Enhanced TypeScript Interface

```typescript
type TaxonomyVariant = 'realm' | 'layer' | 'trait' | 'arcFamily';

interface TaxonomyNodeData extends BaseNodeData {
  variant: TaxonomyVariant;
  variantData: RealmData | LayerData | TraitData | ArcFamilyData;
}

// OrbitalRings Effect Component
const OrbitalRings: React.FC<{ color: string; isHovered: boolean }> = ({
  color,
  isHovered,
}) => {
  const rings = [
    { rx: 30, ry: 12, duration: 8, delay: 0 },
    { rx: 45, ry: 18, duration: 12, delay: -4 },
    { rx: 60, ry: 24, duration: 16, delay: -8 },
  ];

  return (
    <svg className="absolute inset-0 w-full h-full overflow-visible">
      {rings.map((ring, i) => (
        <motion.ellipse
          key={i}
          cx="50%"
          cy="50%"
          rx={ring.rx}
          ry={ring.ry}
          fill="none"
          stroke={color}
          strokeWidth={1}
          strokeOpacity={0.4 - i * 0.1}
          initial={{ rotate: 0 }}
          animate={{
            rotate: 360,
            scale: isHovered ? 1.1 : 1,
          }}
          transition={{
            rotate: { duration: ring.duration, repeat: Infinity, ease: 'linear' },
            scale: { duration: 0.3 }
          }}
          style={{ transformOrigin: 'center' }}
        />
      ))}
    </svg>
  );
};

// StackedPlanes Effect Component
const StackedPlanes: React.FC<{ color: string; isHovered: boolean }> = ({
  color,
  isHovered,
}) => {
  const planes = [
    { z: 0, opacity: 0.3, scale: 1 },
    { z: -8, opacity: 0.2, scale: 0.95 },
    { z: -16, opacity: 0.1, scale: 0.9 },
  ];

  return (
    <div className="absolute inset-0 perspective-500">
      {planes.map((plane, i) => (
        <motion.div
          key={i}
          className="absolute inset-4 rounded-lg border"
          style={{
            borderColor: color,
            opacity: plane.opacity,
            transformStyle: 'preserve-3d',
          }}
          animate={{
            z: isHovered ? plane.z - 4 : plane.z,
            scale: isHovered ? plane.scale * 1.02 : plane.scale,
          }}
          transition={{ duration: 0.3, ease: 'easeOut' }}
        />
      ))}
    </div>
  );
};

// BorderMorph Effect Component
const BorderMorph: React.FC<{ color: string }> = ({ color }) => {
  const styles = ['solid', 'dashed', 'double', 'dotted', 'dotted'];
  const [index, setIndex] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setIndex((prev) => (prev + 1) % styles.length);
    }, 1500);
    return () => clearInterval(interval);
  }, []);

  return (
    <motion.div
      className="absolute inset-4 rounded-lg"
      style={{
        borderWidth: styles[index] === 'double' ? 3 : 2,
        borderStyle: styles[index],
        borderColor: color,
      }}
      animate={{ opacity: [0.8, 1, 0.8] }}
      transition={{ duration: 1.5, repeat: Infinity }}
    />
  );
};
```

---

## Part 4: Level 2 - Schema Cards (239 nodes) - ENHANCED

### 4.1 NodeClass Card (61 nodes) - "Blueprint Holographic"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NODECLASS CARD - Enhanced "Blueprint Holographic" Design                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐  Width: 240px                         │
│  ╎ L─                               ─L ╎  Min-height: 140px                     │
│  ╎ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ╎                                        │
│  ╎  ◈ CLASS                        ●   ╎  ← Layer color header + neon glow     │
│  ╎  ════════════════════════════       ╎  ← Double separator (animated)        │
│  ╎  Entity                             ╎  ← Class name (text-shadow glow)      │
│  ╎  ┌─────────────────────────────────┐╎                                        │
│  ╎  │ ●org │ ●semantic │ ▬defined    │╎  ← 3-column metadata pills             │
│  ╎  ├─────────────────────────────────┤╎                                        │
│  ╎  │ ⊞ 12 props │ →8 out │ ←3 in   │╎  ← Metrics row with counts             │
│  ╎  └─────────────────────────────────┘╎                                        │
│  ╎ L─                               ─L ╎  ← Animated corners with glow         │
│  └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                                        │
│                                                                                 │
│  Border: 2px realm color with neon glow animation                               │
│  Background: rgba(0,0,0,0.85) + glassmorphic blur                               │
│  Effects: Enhanced TechCorners, Scanlines, GridPattern, Shimmer/MatrixRain      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Enhanced TechCorner with Glow

```typescript
const TechCorner: React.FC<{
  position: 'tl' | 'tr' | 'bl' | 'br';
  color: string;
  isHovered: boolean;
  selected: boolean;
}> = ({ position, color, isHovered, selected }) => {
  const transforms = {
    tl: 'rotate(0)',
    tr: 'rotate(90deg)',
    br: 'rotate(180deg)',
    bl: 'rotate(270deg)',
  };

  return (
    <motion.svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      className={cn('absolute pointer-events-none', {
        'top-2 left-2': position === 'tl',
        'top-2 right-2': position === 'tr',
        'bottom-2 right-2': position === 'br',
        'bottom-2 left-2': position === 'bl',
      })}
      style={{ transform: transforms[position], color }}
      initial={{ opacity: 0.5 }}
      animate={{
        opacity: selected ? 1 : isHovered ? 0.8 : 0.5,
        filter: selected
          ? `drop-shadow(0 0 6px ${color}) drop-shadow(0 0 12px ${color})`
          : isHovered
            ? `drop-shadow(0 0 4px ${color})`
            : 'none',
      }}
      transition={{ duration: 0.3, ease: [0.175, 0.885, 0.32, 2.2] }}
    >
      <path d="M0 16L0 0L16 0" stroke="currentColor" strokeWidth="2" fill="none" />
      <circle cx="0" cy="0" r="2.5" fill="currentColor" />
    </motion.svg>
  );
};
```

### 4.2 ArcClass Card (178 nodes) - "Connection Flow"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ARCCLASS CARD - Enhanced "Connection Flow" Design                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐  Width: 260px                         │
│  ╎ L─                               ─L ╎  Min-height: 160px                     │
│  ╎ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ╎                                        │
│  ╎  → ARC CLASS                    ●   ╎  ← Family color header                │
│  ╎  ════════════════════════════       ╎                                        │
│  ╎  HAS_ENTITY                         ╎  ← Arc name (mono, glow)              │
│  ╎  ┌─────────────────────────────────┐╎                                        │
│  ╎  │ ●ownership │ ●intra │ N:M      │╎  ← Family/Scope/Cardinality pills      │
│  ╎  ├─────────────────────────────────┤╎                                        │
│  ╎  │ ┌───────┐         ┌───────┐    │╎                                        │
│  ╎  │ │ Page  │ ══●══●══│Entity │    │╎  ← Animated particle flow              │
│  ╎  │ └───────┘   ∿∿∿∿  └───────┘    │╎                                        │
│  ╎  └─────────────────────────────────┘╎                                        │
│  ╎ L─                               ─L ╎                                        │
│  └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘                                        │
│                                                                                 │
│  Unique: FlowingParticles effect (CSS offset-path or SVG animateMotion)        │
│  Border: 2px family color (ownership=blue, semantic=cyan, etc.)                 │
│  Cardinality Badge: 1:1 / 1:N / N:M with icon                                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### FlowingParticles Effect

```typescript
const FlowingParticles: React.FC<{
  sourceLabel: string;
  targetLabel: string;
  color: string;
  isHovered: boolean;
  cardinality: '1:1' | '1:N' | 'N:M';
}> = ({ sourceLabel, targetLabel, color, isHovered, cardinality }) => {
  const particleCount = cardinality === 'N:M' ? 4 : cardinality === '1:N' ? 3 : 2;

  return (
    <div className="relative flex items-center justify-between gap-3 py-3 px-2">
      {/* Source Node */}
      <motion.div
        className="flex items-center justify-center px-3 py-1.5 rounded-lg border text-xs font-mono"
        style={{ borderColor: `${color}60`, backgroundColor: `${color}10` }}
        whileHover={{ scale: 1.05 }}
      >
        {sourceLabel}
      </motion.div>

      {/* Connection Path with Particles */}
      <div className="relative flex-1 h-8 mx-2">
        <svg className="absolute inset-0 w-full h-full overflow-visible">
          {/* Base path */}
          <path
            id="arcPath"
            d="M0,16 C40,16 60,16 100,16"
            fill="none"
            stroke={`${color}40`}
            strokeWidth="2"
            strokeDasharray={cardinality === '1:1' ? 'none' : '4,4'}
          />

          {/* Flowing particles */}
          {isHovered && Array.from({ length: particleCount }).map((_, i) => (
            <motion.circle
              key={i}
              r="3"
              fill={color}
              initial={{ offsetDistance: '0%' }}
              animate={{ offsetDistance: '100%' }}
              transition={{
                duration: 1.5,
                repeat: Infinity,
                ease: 'linear',
                delay: i * (1.5 / particleCount),
              }}
              style={{
                offsetPath: "path('M0,16 C40,16 60,16 100,16')",
                filter: `drop-shadow(0 0 4px ${color})`,
              }}
            />
          ))}
        </svg>

        {/* Cardinality indicator */}
        <div
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     text-[9px] font-mono px-1.5 py-0.5 rounded-full"
          style={{ backgroundColor: `${color}20`, color }}
        >
          {cardinality}
        </div>
      </div>

      {/* Target Node */}
      <motion.div
        className="flex items-center justify-center px-3 py-1.5 rounded-lg border text-xs font-mono"
        style={{ borderColor: `${color}60`, backgroundColor: `${color}10` }}
        whileHover={{ scale: 1.05 }}
      >
        {targetLabel}
      </motion.div>
    </div>
  );
};
```

---

## Part 5: Level 3 - Data Instance Cards by Layer - ENHANCED

### 5.1 Foundation Layer (6 nodes) - "Brand Identity"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  FOUNDATION LAYER CARDS - Violet #8b5cf6 - Enhanced with Brand Components       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ProjectNode (defined)                    BrandNode (defined)                   │
│  ┌────────────────────────────┐           ┌────────────────────────────────┐    │
│  │ 📁 PROJECT                 │           │ ◉ BRAND             ●active   │    │
│  │ ═══════════════════════    │           │ ══════════════════════════     │    │
│  │ QRCode AI                  │           │ QRCode AI Brand                │    │
│  │ ┌────────────────────────┐ │           │ ┌──────────────────────────┐   │    │
│  │ │ ┌──┬──┬──┬──┬──┐       │ │           │ │ SOUL     purpose/vision │   │    │
│  │ │ │██│██│░░│░░│░░│ 40%   │ │ ← KPI     │ │ PITCH    what + whom   │   │    │
│  │ │ └──┴──┴──┴──┴──┘       │ │   gauge   │ │ VOICE    tone + formal │   │    │
│  │ │ pages 24/60 │ loc 5/200│ │           │ └──────────────────────────┘   │    │
│  │ └────────────────────────┘ │           │ ┌──────────────────────────┐   │    │
│  └────────────────────────────┘           │ │ ● confidence  ● friendly │   │    │
│                                           │ │ ● professional           │   │    │
│                                           │ └──────────────────────────┘   │    │
│                                           └────────────────────────────────┘    │
│                                                                                 │
│  BrandDesignNode (defined)               PromptStyleNode (defined)              │
│  ┌────────────────────────────┐           ┌────────────────────────────────┐    │
│  │ 🎨 DESIGN                  │           │ ✨ PROMPT STYLE                 │    │
│  │ ═══════════════════════    │           │ ══════════════════════════     │    │
│  │ QRCode Design System       │           │ hero-illustration              │    │
│  │ ┌────────────────────────┐ │           │ ┌──────────────────────────┐   │    │
│  │ │ ●Primary   #6366f1     │ │ ← Color   │ │ 🖼️ cinematic            │   │    │
│  │ │ ●Secondary #8b5cf6     │ │   swatches│ │ 🎭 confident, minimal   │   │    │
│  │ │ ●Accent    #06b6d4     │ │           │ │ 🌍 EA (Eastern Asia)    │   │    │
│  │ ├────────────────────────┤ │           │ ├──────────────────────────┤   │    │
│  │ │ Aa Inter 16/24         │ │ ← Typo    │ │ platforms: MJ, DALL-E   │   │    │
│  │ │ Aa JetBrains Mono      │ │   preview │ └──────────────────────────┘   │    │
│  │ └────────────────────────┘ │           └────────────────────────────────┘    │
│  └────────────────────────────┘                                                 │
│                                                                                 │
│  ProjectNativeNode (authored - dashed border)                                   │
│  ┌▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬┐                                                 │
│  │ 🇫🇷 PROJECT NATIVE         │  ← Locale flag + dashed border (authored)      │
│  │ ═══════════════════════    │                                                 │
│  │ QRCode AI France           │                                                 │
│  │ ┌────────────────────────┐ │                                                 │
│  │ │ tagline "Créez vos..." │ │                                                 │
│  │ │ description 245 words  │ │                                                 │
│  │ └────────────────────────┘ │                                                 │
│  └▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬ ▬┘                                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Foundation Card Components

```typescript
// Color Swatch Component
const ColorSwatch: React.FC<{ color: string; label: string }> = ({ color, label }) => (
  <div className="flex items-center gap-2">
    <motion.div
      className="w-4 h-4 rounded-full border border-white/20"
      style={{ backgroundColor: color }}
      whileHover={{ scale: 1.2, boxShadow: `0 0 12px ${color}` }}
    />
    <span className="text-xs text-white/70">{label}</span>
    <span className="text-[9px] font-mono text-white/40">{color}</span>
  </div>
);

// KPI Gauge Component (21st.dev Gauge pattern)
const KPIGauge: React.FC<{
  current: number;
  max: number;
  label: string;
  color: string;
}> = ({ current, max, label, color }) => {
  const percentage = (current / max) * 100;

  return (
    <div className="flex flex-col gap-1">
      <div className="flex justify-between text-[9px]">
        <span className="text-white/60">{label}</span>
        <span style={{ color }}>{current}/{max}</span>
      </div>
      <div className="h-1.5 rounded-full bg-white/10 overflow-hidden">
        <motion.div
          className="h-full rounded-full"
          style={{ backgroundColor: color }}
          initial={{ width: 0 }}
          animate={{ width: `${percentage}%` }}
          transition={{ duration: 0.5, ease: 'easeOut' }}
        />
      </div>
    </div>
  );
};

// Typography Preview Component
const TypographyPreview: React.FC<{
  fontFamily: string;
  fontSize: number;
  lineHeight: number;
}> = ({ fontFamily, fontSize, lineHeight }) => (
  <div className="flex items-baseline gap-2 px-2 py-1 rounded bg-white/5">
    <span
      className="text-lg text-white/80"
      style={{ fontFamily }}
    >
      Aa
    </span>
    <span className="text-[9px] font-mono text-white/40">
      {fontFamily} {fontSize}/{lineHeight}
    </span>
  </div>
);
```

### 5.2 Structure Layer (3 nodes) - "Page Architecture"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  STRUCTURE LAYER CARDS - Blue #3b82f6 - Enhanced with URL & Order Preview       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  PageNode (defined)                       BlockNode (defined)                   │
│  ┌──────────────────────────────┐         ┌────────────────────────┐            │
│  │ 📄 PAGE              ★pillar │         │ 🧱 BLOCK          #1   │            │
│  │ ══════════════════════════   │         │ ════════════════════   │            │
│  │ qr-code-generator            │         │ homepage:hero:1        │            │
│  │ ┌────────────────────────┐   │         │ ┌────────────────────┐ │            │
│  │ │ 🌐 /qr-code-generator  │   │ ← URL   │ │ type  HeroBlock   │ │            │
│  │ │ ↳ /fr/generateur-qr    │   │   paths │ │ ┌────────────────┐ │ │ ← Order   │
│  │ │ ↳ /es/generador-qr     │   │         │ │ │ 1 │ 2 │ 3 │ 4 │ │ │   visual  │
│  │ ├────────────────────────┤   │         │ │ └────────────────┘ │ │            │
│  │ │ blocks 8 │ SEO ●●●○○   │   │ ← Stats │ │ ⊞ 5 props        │ │            │
│  │ │ 🔗 12 internal links   │   │         │ └────────────────────┘ │            │
│  │ └────────────────────────┘   │         └────────────────────────┘            │
│  └──────────────────────────────┘                                               │
│                                                                                 │
│  ContentSlotNode (defined - dashed outline as "placeholder")                    │
│  ┌┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┐                                                 │
│  │ ┌ ─ ─ ┐ SLOT                │  ← Dashed border + interior dashed box         │
│  │         ═══════════════     │    (visual "empty placeholder" feel)           │
│  │ │     │ sidebar-cta         │                                                │
│  │         ┌────────────────┐  │                                                │
│  │ │     │ │ allowed: [CTA] │  │  ← Allowed block types as pills               │
│  │         └────────────────┘  │                                                │
│  │ └ ─ ─ ┘                     │                                                │
│  └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘                                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Structure Card Components

```typescript
// URL Path Display Component
const URLPathDisplay: React.FC<{
  basePath: string;
  localePaths: Array<{ locale: string; path: string }>;
}> = ({ basePath, localePaths }) => (
  <div className="flex flex-col gap-1 p-2 rounded bg-white/5 font-mono text-[10px]">
    <div className="flex items-center gap-1">
      <span className="text-white/40">🌐</span>
      <span className="text-white/80">{basePath}</span>
    </div>
    {localePaths.slice(0, 2).map(({ locale, path }) => (
      <div key={locale} className="flex items-center gap-1 pl-3">
        <span className="text-white/30">↳</span>
        <span className="text-blue-400/60">/{locale}</span>
        <span className="text-white/50">{path}</span>
      </div>
    ))}
    {localePaths.length > 2 && (
      <span className="text-white/30 pl-3">+{localePaths.length - 2} more</span>
    )}
  </div>
);

// Block Order Indicator
const BlockOrderIndicator: React.FC<{
  order: number;
  total: number;
  color: string;
}> = ({ order, total, color }) => (
  <div className="flex gap-0.5">
    {Array.from({ length: Math.min(total, 8) }).map((_, i) => (
      <motion.div
        key={i}
        className="w-3 h-3 rounded-sm border"
        style={{
          backgroundColor: i + 1 === order ? color : 'transparent',
          borderColor: i + 1 === order ? color : `${color}30`,
        }}
        whileHover={{ scale: 1.2 }}
      />
    ))}
    {total > 8 && <span className="text-[9px] text-white/40">+{total - 8}</span>}
  </div>
);

// SEO Score Display
const SEOScoreDisplay: React.FC<{ score: number }> = ({ score }) => {
  const getColor = (s: number) => {
    if (s >= 80) return '#10b981';
    if (s >= 60) return '#eab308';
    return '#ef4444';
  };

  return (
    <div className="flex items-center gap-1">
      <span className="text-[9px] text-white/40">SEO</span>
      <div className="flex gap-0.5">
        {[1, 2, 3, 4, 5].map((i) => (
          <div
            key={i}
            className="w-1.5 h-3 rounded-sm"
            style={{
              backgroundColor: i <= Math.ceil(score / 20)
                ? getColor(score)
                : 'rgba(255,255,255,0.1)',
            }}
          />
        ))}
      </div>
    </div>
  );
};
```

### 5.3 Semantic Layer (4 nodes) - "Knowledge Graph"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SEMANTIC LAYER CARDS - Cyan #06b6d4 - Enhanced with Entity Relations          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  EntityNode (defined)                     EntityNativeNode (authored)           │
│  ┌──────────────────────────────┐         ┌────────────────────────────┐        │
│  │ ◇ ENTITY              ●tree │         │ 🇫🇷 NATIVE      ▬ ▬ ▬ ▬    │        │
│  │ ══════════════════════════   │         │ ════════════════════════   │        │
│  │ qr-code-generator            │         │ entity:qr-code@fr-FR       │        │
│  │ ┌────────────────────────┐   │         │ ┌──────────────────────┐   │        │
│  │ │   ◆                    │   │ ← Dia-  │ │ ← qr-code-generator  │   │        │
│  │ │  ╱│╲                   │   │   mond  │ │                      │   │        │
│  │ │ ◇─┼─◇                  │   │   tree  │ │ Générateur de        │   │        │
│  │ │  ╲│╱                   │   │   icon  │ │ Code QR Gratuit      │   │        │
│  │ │   ◇                    │   │         │ │                      │   │        │
│  │ ├────────────────────────┤   │         │ │ 2,450 words          │   │        │
│  │ │ category  product      │   │         │ │ keywords: 5          │   │        │
│  │ │ children  12           │   │         │ └──────────────────────┘   │        │
│  │ │ keywords  5            │   │         └────────────────────────────┘        │
│  │ └────────────────────────┘   │                                               │
│  └──────────────────────────────┘                                               │
│                                                                                 │
│  AudiencePersonaNode (defined)            ChannelSurfaceNode (defined)          │
│  ┌──────────────────────────────┐         ┌────────────────────────────┐        │
│  │ 👤 PERSONA                   │         │ 📱 CHANNEL                 │        │
│  │ ══════════════════════════   │         │ ════════════════════════   │        │
│  │ developers                   │         │ mobile-app                 │        │
│  │ ┌────────────────────────┐   │         │ ┌──────────────────────┐   │        │
│  │ │ ╭─────╮                │   │ ← Avatar│ │ ┌────┬────┬────┬────┐│   │        │
│  │ │ │ 👨‍💻 │ 28-45 yrs     │   │   + age │ │ │ iOS│Andr│ Web│ API││   │ ← Chan │
│  │ │ ╰─────╯ Male/Female    │   │         │ │ │ ●  │ ●  │ ○  │ ●  ││   │   matrix│
│  │ ├────────────────────────┤   │         │ │ └────┴────┴────┴────┘│   │        │
│  │ │ 😤 Pain: manual QR...  │   │ ← Pain/ │ ├──────────────────────┤   │        │
│  │ │ 🎯 Goal: automate...   │   │   goal  │ │ caps: push, AR, geo  │   │        │
│  │ │ 💡 Tech-savvy, API-    │   │         │ │ limits: no video     │   │        │
│  │ │    first mindset       │   │         │ └──────────────────────┘   │        │
│  │ └────────────────────────┘   │         └────────────────────────────┘        │
│  └──────────────────────────────┘                                               │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Semantic Card Components

```typescript
// Entity Tree Icon (Diamond with connections)
const EntityTreeIcon: React.FC<{ color: string; hasChildren: boolean }> = ({
  color,
  hasChildren,
}) => (
  <svg width="40" height="40" viewBox="0 0 40 40" className="opacity-60">
    {/* Central diamond */}
    <motion.polygon
      points="20,4 36,20 20,36 4,20"
      fill="none"
      stroke={color}
      strokeWidth="1.5"
      initial={{ scale: 0.8 }}
      animate={{ scale: 1 }}
    />
    {hasChildren && (
      <>
        {/* Child connections */}
        <line x1="20" y1="36" x2="10" y2="46" stroke={color} strokeOpacity="0.4" />
        <line x1="20" y1="36" x2="20" y2="46" stroke={color} strokeOpacity="0.4" />
        <line x1="20" y1="36" x2="30" y2="46" stroke={color} strokeOpacity="0.4" />
        {/* Child diamonds (smaller) */}
        <polygon points="10,46 14,50 10,54 6,50" fill={color} fillOpacity="0.3" />
        <polygon points="20,46 24,50 20,54 16,50" fill={color} fillOpacity="0.3" />
        <polygon points="30,46 34,50 30,54 26,50" fill={color} fillOpacity="0.3" />
      </>
    )}
  </svg>
);

// Persona Avatar Component
const PersonaAvatar: React.FC<{
  ageRange: string;
  gender: string;
}> = ({ ageRange, gender }) => (
  <div className="flex items-center gap-3">
    <div className="w-12 h-12 rounded-full bg-gradient-to-br from-cyan-500/30 to-violet-500/30
                    flex items-center justify-center text-2xl border border-white/10">
      👨‍💻
    </div>
    <div className="text-[10px] text-white/60">
      <div>{ageRange}</div>
      <div>{gender}</div>
    </div>
  </div>
);

// Channel Matrix Component
const ChannelMatrix: React.FC<{
  channels: Array<{ name: string; supported: boolean }>;
}> = ({ channels }) => (
  <div className="grid grid-cols-4 gap-1">
    {channels.map(({ name, supported }) => (
      <div
        key={name}
        className={cn(
          'px-1.5 py-1 rounded text-center text-[8px] font-mono',
          supported ? 'bg-emerald-500/20 text-emerald-400' : 'bg-white/5 text-white/30'
        )}
      >
        <div>{name}</div>
        <div>{supported ? '●' : '○'}</div>
      </div>
    ))}
  </div>
);
```

### 5.4 Instruction Layer (4 nodes) - "LLM Pipeline"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  INSTRUCTION LAYER CARDS - Pink #ec4899 - Enhanced with JSON Preview           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  BlockTypeNode (defined)                  BlockInstructionNode (defined)        │
│  ┌──────────────────────────────┐         ┌────────────────────────────┐        │
│  │ {} BLOCK TYPE                │         │ 📝 INSTRUCTION             │        │
│  │ ══════════════════════════   │         │ ════════════════════════   │        │
│  │ HeroBlock                    │         │ hero-instruction           │        │
│  │ ┌────────────────────────┐   │         │ ┌──────────────────────┐   │        │
│  │ │ {                      │   │ ← JSON  │ │ @ References:        │   │        │
│  │ │   "title": string,     │   │   prev  │ │ ├─ @entity: 3        │   │        │
│  │ │   "subtitle": string,  │   │   iew   │ │ ├─ @term: 5          │   │        │
│  │ │   "cta": {             │   │   (syn  │ │ └─ @page: 2          │   │        │
│  │ │     "label": ...,      │   │   tax   │ ├──────────────────────┤   │        │
│  │ │     "url": ...         │   │   high  │ │ [TRANSLATE] title    │   │        │
│  │ │   }                    │   │   light │ │ [FIXED] cta_url      │   │        │
│  │ │ }                      │   │   ed)   │ │ [DERIVE] meta_desc   │   │        │
│  │ └────────────────────────┘   │         │ └──────────────────────────┘│        │
│  │ ⊞ 8 required │ 12 total     │         │ 245 words                   │        │
│  └──────────────────────────────┘         └────────────────────────────┘        │
│                                                                                 │
│  BlockRulesNode (defined)                 PromptArtifactNode (generated)        │
│  ┌──────────────────────────────┐         ┌····························┐        │
│  │ ⚠️ RULES                     │         │ 📋 ARTIFACT     ........   │        │
│  │ ══════════════════════════   │         │ ════════════════════════   │        │
│  │ hero-rules                   │         │ prompt-hero-fr-FR          │        │
│  │ ┌────────────────────────┐   │         │ ┌──────────────────────┐   │        │
│  │ │ 🔴 max_length: 150     │   │ ← Sever │ │ tokens  4,230        │   │ ← Gen  │
│  │ │ 🔴 required: title     │   │   ity   │ │ compiled  2h ago     │   │   dot  │
│  │ │ 🟡 prefer: active_voice │   │   color │ │ entities  8          │   │   ted  │
│  │ │ 🔵 tip: include CTA    │   │   coded │ │ terms  23            │   │   bord │
│  │ └────────────────────────┘   │         │ └──────────────────────┘   │        │
│  └──────────────────────────────┘         └····························┘        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Instruction Card Components

```typescript
// JSON Schema Preview Component
const JSONSchemaPreview: React.FC<{
  schema: Record<string, unknown>;
  maxLines?: number;
}> = ({ schema, maxLines = 6 }) => {
  const formatted = JSON.stringify(schema, null, 2).split('\n').slice(0, maxLines);

  return (
    <div className="font-mono text-[9px] p-2 rounded bg-black/40 overflow-hidden">
      {formatted.map((line, i) => (
        <div key={i} className="whitespace-pre">
          {highlightJSON(line)}
        </div>
      ))}
      {JSON.stringify(schema, null, 2).split('\n').length > maxLines && (
        <div className="text-white/30">...</div>
      )}
    </div>
  );
};

// JSON syntax highlighting helper
const highlightJSON = (line: string): React.ReactNode => {
  return line
    .replace(/"([^"]+)":/g, '<span class="text-pink-400">"$1"</span>:')
    .replace(/: "([^"]+)"/g, ': <span class="text-emerald-400">"$1"</span>')
    .replace(/: (string|number|boolean)/g, ': <span class="text-cyan-400">$1</span>');
};

// @ Reference Counter Component
const RefCounter: React.FC<{
  refs: Array<{ type: string; count: number }>;
}> = ({ refs }) => (
  <div className="flex flex-col gap-0.5 text-[9px]">
    <div className="text-white/50 mb-1">@ References:</div>
    {refs.map(({ type, count }) => (
      <div key={type} className="flex items-center gap-1 pl-2">
        <span className="text-pink-400">├─</span>
        <span className="text-white/40">@{type}:</span>
        <span className="text-white/70">{count}</span>
      </div>
    ))}
  </div>
);

// Rule Severity Badge
const RuleSeverityBadge: React.FC<{
  severity: 'error' | 'warning' | 'info';
  text: string;
}> = ({ severity, text }) => {
  const colors = {
    error: { bg: 'bg-red-500/20', text: 'text-red-400', icon: '🔴' },
    warning: { bg: 'bg-amber-500/20', text: 'text-amber-400', icon: '🟡' },
    info: { bg: 'bg-blue-500/20', text: 'text-blue-400', icon: '🔵' },
  };

  const { bg, text: textColor, icon } = colors[severity];

  return (
    <div className={cn('flex items-center gap-1.5 px-2 py-0.5 rounded text-[9px]', bg)}>
      <span>{icon}</span>
      <span className={textColor}>{text}</span>
    </div>
  );
};
```

### 5.5 Output Layer (3 nodes) - "AI Generated Content"

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  OUTPUT LAYER CARDS - Emerald #10b981 (dotted borders per ADR-005 generated)   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  PageNativeNode (generated)               BlockNativeNode (generated)           │
│  ┌··································┐     ┌····························┐        │
│  │ ✨ PAGE NATIVE     🇫🇷 fr-FR    │     │ ✨ BLOCK        ........ │        │
│  │ ════════════════════════════    │     │ ════════════════════════ │        │
│  │ page:homepage@fr-FR             │     │ block:hero:1@fr-FR       │        │
│  │ ┌──────────────────────────┐    │     │ ┌──────────────────────┐ │        │
│  │ │ 🔍 SERP Preview:         │    │ SEO │ │ "Créez votre QR..." │ │ ← Con  │
│  │ │ ┌────────────────────────┐│    │ pre │ │                      │ │   tent│
│  │ │ │Créateur QR Code - Site ││    │ view│ │ ✨ AI confidence:    │ │   pre │
│  │ │ │qrcode-ai.com/fr/...    ││    │     │ │ ●●●●○ 82%           │ │   view│
│  │ │ │Générateur de code QR   ││    │     │ │                      │ │       │
│  │ │ │gratuit avec analytics. ││    │     │ │ 156 words            │ │       │
│  │ │ └────────────────────────┘│    │     │ └──────────────────────┘ │       │
│  │ └──────────────────────────┘    │     └····························┘       │
│  │ words 3,240 │ blocks 8          │                                          │
│  └··································┘                                          │
│                                                                                 │
│  OutputArtifactNode (generated)                                                 │
│  ┌··································┐                                          │
│  │ 📦 OUTPUT          ✨ ........   │  ← AI sparkles + dotted border           │
│  │ ════════════════════════════    │                                          │
│  │ deploy-2026-02-17-fr            │                                          │
│  │ ┌──────────────────────────┐    │                                          │
│  │ │ pages  24   │ locales 5  │    │                                          │
│  │ │ bundle 1.2MB│ ● deployed │    │ ← Status indicator (animated pulse)      │
│  │ │ ┌────────────────────┐   │    │                                          │
│  │ │ │ ■■■■■■■■■░░ 85%    │   │    │ ← Build progress bar                     │
│  │ │ └────────────────────┘   │    │                                          │
│  │ └──────────────────────────┘    │                                          │
│  └··································┘                                          │
│                                                                                 │
│  Note: Dotted borders indicate "generated" trait per ADR-005                   │
│        Sparkles (✨) icon indicates AI-generated content                        │
│        Dashed borders for "authored" trait (human-written)                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Output Card Components

```typescript
// SERP Preview Component (SEO preview card)
const SERPPreview: React.FC<{
  title: string;
  url: string;
  description: string;
}> = ({ title, url, description }) => (
  <div className="p-2 rounded bg-white/5 border border-white/10">
    <div className="text-[11px] text-blue-400 font-medium line-clamp-1">{title}</div>
    <div className="text-[9px] text-emerald-400/80 font-mono">{url}</div>
    <div className="text-[9px] text-white/60 line-clamp-2 mt-0.5">{description}</div>
  </div>
);

// AI Confidence Gauge
const AIConfidenceGauge: React.FC<{
  confidence: number; // 0-100
}> = ({ confidence }) => {
  const getColor = (c: number) => {
    if (c >= 80) return '#10b981';
    if (c >= 60) return '#eab308';
    return '#ef4444';
  };

  const filled = Math.round(confidence / 20);

  return (
    <div className="flex items-center gap-1">
      <span className="text-[9px] text-white/40">✨ AI confidence:</span>
      <div className="flex gap-0.5">
        {[1, 2, 3, 4, 5].map((i) => (
          <div
            key={i}
            className="w-1.5 h-3 rounded-sm"
            style={{
              backgroundColor: i <= filled ? getColor(confidence) : 'rgba(255,255,255,0.1)',
            }}
          />
        ))}
      </div>
      <span className="text-[9px]" style={{ color: getColor(confidence) }}>
        {confidence}%
      </span>
    </div>
  );
};

// Sparkles Animation (for AI-generated indicator)
const AISparkles: React.FC<{ active: boolean }> = ({ active }) => {
  if (!active) return <span className="opacity-50">✨</span>;

  return (
    <motion.span
      animate={{
        opacity: [0.5, 1, 0.5],
        scale: [1, 1.1, 1],
      }}
      transition={{
        duration: 2,
        repeat: Infinity,
        ease: 'easeInOut',
      }}
    >
      ✨
    </motion.span>
  );
};

// Deploy Status Indicator
const DeployStatus: React.FC<{
  status: 'building' | 'deployed' | 'failed' | 'pending';
}> = ({ status }) => {
  const configs = {
    building: { color: '#eab308', label: 'Building', animate: true },
    deployed: { color: '#10b981', label: 'Deployed', animate: false },
    failed: { color: '#ef4444', label: 'Failed', animate: false },
    pending: { color: '#64748b', label: 'Pending', animate: true },
  };

  const { color, label, animate } = configs[status];

  return (
    <div className="flex items-center gap-1.5">
      <motion.div
        className="w-2 h-2 rounded-full"
        style={{ backgroundColor: color }}
        animate={animate ? { opacity: [1, 0.4, 1] } : undefined}
        transition={animate ? { duration: 1.5, repeat: Infinity } : undefined}
      />
      <span className="text-[9px]" style={{ color }}>
        {label}
      </span>
    </div>
  );
};
```

### 5.6 Knowledge Layer - Container + Atom Pattern (Enhanced)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  KNOWLEDGE LAYER - Yellow #eab308 - Enhanced with Coverage Meters & Chips      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  CONTAINERS (Set nodes) - Full cards with coverage meters                       │
│  ┌════════════════════════════════════┐   ← Double border (imported trait)      │
│  ║ 📚 TERM SET                        ║                                         │
│  ║ ══════════════════════════════     ║                                         │
│  ║ domain: technical                  ║                                         │
│  ║ ┌──────────────────────────────┐   ║                                         │
│  ║ │ terms  2,450                 │   ║                                         │
│  ║ │ locale  🇫🇷 fr-FR            │   ║                                         │
│  ║ │ ┌────────────────────────┐   │   ║ ← Coverage meter (21st.dev Gauge)       │
│  ║ │ │■■■■■■■■░░░░░░░│ 65%   │   ║                                         │
│  ║ │ └────────────────────────┘   │   ║                                         │
│  ║ │ used: 1,592 │ unused: 858    │   ║                                         │
│  ║ └──────────────────────────────┘   ║                                         │
│  └════════════════════════════════════┘                                         │
│                                                                                 │
│  ATOMS - Compact pill chips with hover expansion                                │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐                    │
│  │ 💬 abonnement   │ │ 💬 générateur   │ │ 💬 code QR      │                    │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘                    │
│         ↓ hover expands                                                         │
│  ┌─────────────────────────────────────────┐                                    │
│  │ 💬 abonnement                           │                                    │
│  │ "monthly subscription payment model"    │                                    │
│  │ used by: 24 blocks │ domain: pricing    │                                    │
│  └─────────────────────────────────────────┘                                    │
│                                                                                 │
│  SEO/GEO ATOMS - With difficulty/platform indicators                           │
│  ┌─────────────────────────────────┐ ┌─────────────────────────────────┐        │
│  │ 🔍 qr code generator            │ │ 🌐 "best qr code maker"         │        │
│  │    vol 12K                      │ │    🤖 GPT-4  💎 Perplexity      │ ← AI   │
│  │    diff ●●●○○ medium            │ │    ✓ Claude  ○ Gemini           │   plat │
│  │    trend ↗ +15%                 │ │    rank #3 │ last: 2h ago       │   forms│
│  └─────────────────────────────────┘ └─────────────────────────────────┘        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Knowledge Card Components

```typescript
// Coverage Meter (21st.dev Gauge inspired)
const CoverageMeter: React.FC<{
  used: number;
  total: number;
  color: string;
}> = ({ used, total, color }) => {
  const percentage = Math.round((used / total) * 100);

  return (
    <div className="flex flex-col gap-1">
      <div className="flex justify-between text-[9px]">
        <span className="text-white/50">coverage</span>
        <span style={{ color }}>{percentage}%</span>
      </div>
      <div className="h-2 rounded-full bg-white/10 overflow-hidden">
        <motion.div
          className="h-full rounded-full"
          style={{ backgroundColor: color }}
          initial={{ width: 0 }}
          animate={{ width: `${percentage}%` }}
          transition={{ duration: 0.8, ease: 'easeOut' }}
        />
      </div>
      <div className="flex justify-between text-[8px] text-white/40">
        <span>used: {used.toLocaleString()}</span>
        <span>unused: {(total - used).toLocaleString()}</span>
      </div>
    </div>
  );
};

// Atom Chip with hover expansion
const AtomChip: React.FC<{
  term: string;
  definition?: string;
  usedBy?: number;
  domain?: string;
}> = ({ term, definition, usedBy, domain }) => {
  const [expanded, setExpanded] = useState(false);

  return (
    <motion.div
      className="relative"
      onHoverStart={() => setExpanded(true)}
      onHoverEnd={() => setExpanded(false)}
    >
      <motion.div
        className="px-2.5 py-1 rounded-full bg-yellow-500/10 border border-yellow-500/30
                   text-[10px] text-yellow-400 cursor-default"
        whileHover={{ scale: 1.05 }}
      >
        💬 {term}
      </motion.div>

      <AnimatePresence>
        {expanded && definition && (
          <motion.div
            className="absolute z-50 top-full left-0 mt-1 p-2 rounded-lg bg-black/95
                       border border-yellow-500/30 min-w-[200px] shadow-lg"
            initial={{ opacity: 0, y: -5 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -5 }}
          >
            <div className="text-[10px] text-yellow-400 font-medium">💬 {term}</div>
            <div className="text-[9px] text-white/70 mt-1">{definition}</div>
            <div className="flex gap-2 mt-1 text-[8px] text-white/40">
              {usedBy && <span>used by: {usedBy} blocks</span>}
              {domain && <span>domain: {domain}</span>}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
};

// SEO Difficulty Gauge
const SEODifficultyGauge: React.FC<{
  difficulty: number; // 0-100
}> = ({ difficulty }) => {
  const level = difficulty <= 30 ? 'easy' : difficulty <= 60 ? 'medium' : 'hard';
  const colors = {
    easy: { fill: '#10b981', text: 'Easy' },
    medium: { fill: '#eab308', text: 'Medium' },
    hard: { fill: '#ef4444', text: 'Hard' },
  };

  const filledDots = Math.ceil(difficulty / 20);

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[9px] text-white/40">diff</span>
      <div className="flex gap-0.5">
        {[1, 2, 3, 4, 5].map((i) => (
          <div
            key={i}
            className="w-2 h-2 rounded-full"
            style={{
              backgroundColor: i <= filledDots ? colors[level].fill : 'rgba(255,255,255,0.1)',
            }}
          />
        ))}
      </div>
      <span className="text-[8px]" style={{ color: colors[level].fill }}>
        {colors[level].text}
      </span>
    </div>
  );
};

// AI Platform Status Grid
const AIPlatformStatus: React.FC<{
  platforms: Array<{ name: string; icon: string; status: 'active' | 'inactive' | 'unknown' }>;
}> = ({ platforms }) => (
  <div className="flex flex-wrap gap-1">
    {platforms.map(({ name, icon, status }) => (
      <div
        key={name}
        className={cn(
          'flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[8px]',
          status === 'active' && 'bg-emerald-500/20 text-emerald-400',
          status === 'inactive' && 'bg-white/5 text-white/30',
          status === 'unknown' && 'bg-amber-500/10 text-amber-400/60'
        )}
      >
        <span>{icon}</span>
        <span>{status === 'active' ? '✓' : status === 'inactive' ? '○' : '?'}</span>
      </div>
    ))}
  </div>
);
```

### 5.7 Locale + Geography Layers (Enhanced)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  LOCALE LAYER - Green #22c55e | GEOGRAPHY LAYER - Orange #f97316               │
│  Enhanced with Flag Integration, Hofstede Gauges, and Passport Style           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  LocaleNode (defined)                     CultureNode (imported)                │
│  ┌──────────────────────────────┐         ┌════════════════════════════┐        │
│  │ 🌐 LOCALE                    │         ║ 🧭 CULTURE                 ║ ← Dbl  │
│  │ ══════════════════════════   │         ║ ════════════════════════   ║   bord│
│  │ ┌────────────────────┐       │ ← Large │ high-context               ║        │
│  │ │                    │       │   flag  │ ┌──────────────────────┐   ║        │
│  │ │       🇫🇷          │       │   (40px)│ │ Hofstede Dimensions: │   ║        │
│  │ │                    │       │         │ │ ┌─────────────────┐  │   ║        │
│  │ └────────────────────┘       │         │ │ │PDI ■■■■■■░░ 71 │  │   ║ ← Hof  │
│  │ fr-FR                        │         │ │ │IDV ■■■■░░░░ 43 │  │   ║   stede│
│  │ French (France)              │         │ │ │MAS ■■■■░░░░ 45 │  │   ║   gauge│
│  │ ┌────────────────────────┐   │         │ │ │UAI ■■■■■■■░ 86 │  │   ║        │
│  │ │ terms 2,450 │ expr 890 │   │         │ │ └─────────────────┘  │   ║        │
│  │ │ coverage ●●●●●●●○○○ 70%│   │         │ │ ◐ direct   ◕ formal  │   ║        │
│  │ └────────────────────────┘   │         │ └──────────────────────┘   ║        │
│  └──────────────────────────────┘         └════════════════════════════┘        │
│                                                                                 │
│  CountryNode (imported)                   RegionNode (imported)                 │
│  ┌════════════════════════════════┐       ┌════════════════════════════┐        │
│  ║ 🛂 COUNTRY                     ║ ← Pass║ 🗺️ REGION                  ║        │
│  ║ ══════════════════════════     ║   port║ ════════════════════════   ║        │
│  ║ ┌────────────────────────┐     ║   style║ Western Europe             ║        │
│  ║ │  ╭─────────────────╮   │     ║       │ ┌──────────────────────┐   ║        │
│  ║ │  │     FRANCE      │   │     ║       │ │ ┌──────────────────┐ │   ║ ← Mini│
│  ║ │  │                 │   │     ║       │ │ │ 🇫🇷🇩🇪🇪🇸🇮🇹🇳🇱 │ │   ║   flags│
│  ║ │  │    ● Paris      │   │     ║       │ │ └──────────────────┘ │   ║        │
│  ║ │  │      🇫🇷        │   │     ║       │ │ countries: 12        │   ║        │
│  ║ │  ╰─────────────────╯   │     ║       │ │ population: 420M     │   ║        │
│  ║ │        FR              │     ║       │ │ GDP: $18.5T          │   ║        │
│  ║ └────────────────────────┘     ║       │ └──────────────────────┘   ║        │
│  ║ ┌────────────────────────┐     ║       └════════════════════════════┘        │
│  ║ │ region  Western Europe │     ║                                             │
│  ║ │ income  High           │     ║                                             │
│  ║ │ pop     67M            │     ║                                             │
│  ║ └────────────────────────┘     ║                                             │
│  └════════════════════════════════┘                                             │
│                                                                                 │
│  Optional: COBE-style 3D globe visualization for geographic context            │
│  (using react-globe.gl or similar for interactive geo cards)                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Locale/Geo Card Components

```typescript
// Large Flag Display
const LargeFlag: React.FC<{
  locale: string;
  size?: 'sm' | 'md' | 'lg';
}> = ({ locale, size = 'md' }) => {
  const countryCode = locale.split('-')[1];
  const dimensions = { sm: 24, md: 40, lg: 56 };

  return (
    <div
      className="rounded-lg overflow-hidden border border-white/10 shadow-lg"
      style={{ width: dimensions[size], height: dimensions[size] * 0.75 }}
    >
      <img
        src={`https://flagcdn.com/w80/${countryCode.toLowerCase()}.png`}
        alt={locale}
        className="w-full h-full object-cover"
      />
    </div>
  );
};

// Hofstede Dimension Gauge
const HofstedeGauge: React.FC<{
  dimension: 'PDI' | 'IDV' | 'MAS' | 'UAI' | 'LTO' | 'IVR';
  value: number; // 0-100
}> = ({ dimension, value }) => {
  const labels = {
    PDI: 'Power Distance',
    IDV: 'Individualism',
    MAS: 'Masculinity',
    UAI: 'Uncertainty Avoidance',
    LTO: 'Long-Term Orientation',
    IVR: 'Indulgence',
  };

  const filled = Math.round(value / 12.5);

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[9px] font-mono text-white/50 w-6">{dimension}</span>
      <div className="flex gap-0.5">
        {Array.from({ length: 8 }).map((_, i) => (
          <div
            key={i}
            className="w-1.5 h-3 rounded-sm"
            style={{
              backgroundColor: i < filled ? '#22c55e' : 'rgba(255,255,255,0.1)',
            }}
          />
        ))}
      </div>
      <span className="text-[9px] text-white/60">{value}</span>
    </div>
  );
};

// Passport Style Country Card
const PassportCard: React.FC<{
  name: string;
  code: string;
  capital: string;
  region: string;
}> = ({ name, code, capital, region }) => (
  <div className="border-4 border-double border-orange-500/40 rounded-lg p-3 bg-orange-950/30">
    <div className="text-center mb-2">
      <div className="text-xs font-mono text-orange-400/60 tracking-widest">{name.toUpperCase()}</div>
    </div>
    <div className="flex items-center gap-3">
      <LargeFlag locale={`xx-${code}`} size="lg" />
      <div className="text-[9px]">
        <div className="flex items-center gap-1">
          <span className="text-white/40">capital:</span>
          <span className="text-white/70">● {capital}</span>
        </div>
        <div className="text-white/40">{region}</div>
      </div>
    </div>
    <div className="text-center mt-2 text-lg font-mono text-orange-400/80">{code}</div>
  </div>
);

// Mini Flag Grid (for regions)
const MiniFlagGrid: React.FC<{
  countries: Array<{ code: string; locale: string }>;
  max?: number;
}> = ({ countries, max = 5 }) => (
  <div className="flex gap-1 flex-wrap">
    {countries.slice(0, max).map(({ code, locale }) => (
      <img
        key={code}
        src={`https://flagcdn.com/w20/${code.toLowerCase()}.png`}
        alt={locale}
        className="w-5 h-3 rounded-sm border border-white/10"
      />
    ))}
    {countries.length > max && (
      <span className="text-[9px] text-white/40">+{countries.length - max}</span>
    )}
  </div>
);
```

---

## Part 6: Premium Effects Library (Enhanced)

### Matrix Rain - Canvas Implementation

```typescript
interface MatrixRainProps {
  color: string;
  fontSize?: number;
  fadeOpacity?: number;
  speed?: number;
  active: boolean;
}

const MatrixRain: React.FC<MatrixRainProps> = ({
  color,
  fontSize = 14,
  fadeOpacity = 0.1,
  speed = 1,
  active,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!active) return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const columns = Math.floor(canvas.width / fontSize);
    const drops: number[] = Array(columns).fill(1);
    const chars = '01アイウエオカキクケコ'.split('');

    const draw = () => {
      // Fade effect
      ctx.fillStyle = `rgba(0, 0, 0, ${fadeOpacity})`;
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw characters
      ctx.fillStyle = color;
      ctx.font = `${fontSize}px monospace`;
      ctx.shadowBlur = 8;
      ctx.shadowColor = color;

      for (let i = 0; i < drops.length; i++) {
        const char = chars[Math.floor(Math.random() * chars.length)];
        ctx.fillText(char, i * fontSize, drops[i] * fontSize);

        if (drops[i] * fontSize > canvas.height && Math.random() > 0.975) {
          drops[i] = 0;
        }
        drops[i]++;
      }
    };

    const interval = setInterval(draw, 33 / speed);
    return () => clearInterval(interval);
  }, [active, color, fontSize, fadeOpacity, speed]);

  if (!active) return null;

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full rounded-xl pointer-events-none opacity-30"
      style={{ mixBlendMode: 'screen' }}
    />
  );
};
```

### Glassmorphic Blur Effect

```typescript
const GlassmorphicBlur: React.FC<{
  intensity?: 'light' | 'medium' | 'heavy';
  tint?: string;
}> = ({ intensity = 'medium', tint = 'rgba(0,0,0,0.5)' }) => {
  const blurValues = {
    light: 4,
    medium: 8,
    heavy: 16,
  };

  return (
    <>
      <svg className="absolute w-0 h-0">
        <defs>
          <filter id="glassmorphism">
            <feGaussianBlur in="SourceGraphic" stdDeviation={blurValues[intensity]} />
            <feColorMatrix
              type="matrix"
              values="1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 18 -7"
            />
          </filter>
        </defs>
      </svg>
      <div
        className="absolute inset-0 rounded-xl"
        style={{
          backdropFilter: `blur(${blurValues[intensity]}px)`,
          WebkitBackdropFilter: `blur(${blurValues[intensity]}px)`,
          backgroundColor: tint,
        }}
      />
    </>
  );
};
```

### Neon Border Glow

```typescript
const NeonBorderGlow: React.FC<{
  color: string;
  intensity: 'idle' | 'hover' | 'selected';
  borderRadius?: number;
}> = ({ color, intensity, borderRadius = 12 }) => {
  const glowConfig = {
    idle: { blur: 6, spread: 0, opacity: 0.3 },
    hover: { blur: 12, spread: 2, opacity: 0.5 },
    selected: { blur: 20, spread: 4, opacity: 0.7 },
  };

  const { blur, spread, opacity } = glowConfig[intensity];

  return (
    <motion.div
      className="absolute -inset-0.5 rounded-xl pointer-events-none"
      style={{ borderRadius: borderRadius + 2 }}
      animate={{
        boxShadow: `0 0 ${blur}px ${spread}px ${color}${Math.round(opacity * 255).toString(16)}`,
      }}
      transition={{ duration: 0.3 }}
    />
  );
};
```

### Glitch Effect (for error states)

```typescript
const GlitchEffect: React.FC<{
  active: boolean;
  color: string;
}> = ({ active, color }) => {
  if (!active) return null;

  return (
    <div className="absolute inset-0 pointer-events-none overflow-hidden rounded-xl">
      <motion.div
        className="absolute inset-0"
        style={{
          background: `linear-gradient(transparent 0%, ${color}10 50%, transparent 100%)`,
          height: '2px',
        }}
        animate={{
          y: ['-100%', '200%'],
        }}
        transition={{
          duration: 0.1,
          repeat: Infinity,
          repeatDelay: Math.random() * 2 + 1,
        }}
      />
      <style jsx>{`
        @keyframes glitch-clip {
          0% { clip-path: inset(40% 0 61% 0); }
          20% { clip-path: inset(92% 0 1% 0); }
          40% { clip-path: inset(43% 0 1% 0); }
          60% { clip-path: inset(25% 0 58% 0); }
          80% { clip-path: inset(54% 0 7% 0); }
          100% { clip-path: inset(58% 0 43% 0); }
        }
      `}</style>
    </div>
  );
};
```

---

## Part 7: Performance Tier System

### Tier Definitions

```typescript
type PerformanceTier = 'ULTRA' | 'HIGH' | 'MEDIUM' | 'LOW' | 'MINIMAL';

interface PerformanceConfig {
  tier: PerformanceTier;
  effects: {
    techCorners: boolean;
    scanlines: boolean;
    gridPattern: boolean;
    shimmer: boolean;
    matrixRain: boolean;
    outerGlow: boolean;
    particles: boolean;     // For ArcClass
    borderMorph: boolean;   // For Taxonomy
    glassmorphism: boolean; // backdrop-filter
    neonGlow: boolean;      // box-shadow animation
  };
  animation: {
    enabled: boolean;
    duration: 'fast' | 'normal' | 'slow' | 'none';
    stagger: boolean;       // Staggered entrance
    spring: boolean;        // Spring physics
  };
  rendering: {
    lazy: boolean;
    virtualized: boolean;
    simplified: boolean;
    suspense: boolean;      // React Suspense boundaries
  };
}

function getPerformanceConfig(nodeCount: number): PerformanceConfig {
  if (nodeCount <= 20) return ULTRA_CONFIG;
  if (nodeCount <= 50) return HIGH_CONFIG;
  if (nodeCount <= 100) return MEDIUM_CONFIG;
  if (nodeCount <= 200) return LOW_CONFIG;
  return MINIMAL_CONFIG;
}

const ULTRA_CONFIG: PerformanceConfig = {
  tier: 'ULTRA',
  effects: {
    techCorners: true, scanlines: true, gridPattern: true,
    shimmer: true, matrixRain: true, outerGlow: true,
    particles: true, borderMorph: true, glassmorphism: true, neonGlow: true,
  },
  animation: { enabled: true, duration: 'normal', stagger: true, spring: true },
  rendering: { lazy: false, virtualized: false, simplified: false, suspense: false },
};

const MINIMAL_CONFIG: PerformanceConfig = {
  tier: 'MINIMAL',
  effects: {
    techCorners: false, scanlines: false, gridPattern: false,
    shimmer: false, matrixRain: false, outerGlow: false,
    particles: false, borderMorph: false, glassmorphism: false, neonGlow: false,
  },
  animation: { enabled: false, duration: 'none', stagger: false, spring: false },
  rendering: { lazy: true, virtualized: true, simplified: true, suspense: true },
};
```

### Tier Specifications

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PERFORMANCE TIERS                                                              │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ULTRA (0-20 nodes)                                                             │
│  ├── All effects enabled (12/12)                                                │
│  ├── Full animations (300ms spring physics)                                     │
│  ├── Matrix rain on selection (canvas)                                          │
│  ├── Particles for ArcClass (SVG animateMotion)                                 │
│  ├── Glassmorphism + Neon glow                                                  │
│  └── No optimization needed                                                     │
│                                                                                 │
│  HIGH (20-50 nodes)                                                             │
│  ├── Most effects (10/12, no matrixRain, no particles)                          │
│  ├── Normal animations (200ms ease-out)                                         │
│  ├── Shimmer on hover                                                           │
│  ├── Neon glow on selected                                                      │
│  └── Staggered entrance animations                                              │
│                                                                                 │
│  MEDIUM (50-100 nodes)                                                          │
│  ├── Core effects only (6/12)                                                   │
│  ├── Reduced animations (150ms)                                                 │
│  ├── No shimmer, no particles                                                   │
│  ├── Start memoization                                                          │
│  └── Suspense boundaries                                                        │
│                                                                                 │
│  LOW (100-200 nodes)                                                            │
│  ├── Minimal effects (3/12: corners, grid, glow)                                │
│  ├── Minimal animations (100ms)                                                 │
│  ├── Heavy memoization                                                          │
│  ├── Virtualized list ready                                                     │
│  └── Lazy loading (10 at a time)                                                │
│                                                                                 │
│  MINIMAL (200+ nodes)                                                           │
│  ├── No effects (solid colors only)                                             │
│  ├── No animations                                                              │
│  ├── Fully virtualized rendering                                                │
│  ├── Aggressive lazy loading                                                    │
│  └── Simplified card layout (no badges, minimal text)                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Part 8: File Structure (Updated)

```
apps/studio/src/components/graph/nodes/
├── index.ts                          # Node registry
├── BaseNodeWrapper.tsx               # HOC with handles + dimming
│
├── card/
│   ├── index.ts                      # Card component exports
│   ├── CardShell.tsx                 # Base shell with context + perf tier
│   ├── CardHeader.tsx                # Shared header (icon, label, status)
│   ├── CardBody.tsx                  # Shared body (title, subtitle, children)
│   ├── CardFooter.tsx                # Shared footer (badges, metrics)
│   ├── CardBadge.tsx                 # Realm/Layer/Trait badges
│   ├── hooks/
│   │   ├── useCardColors.ts          # Color palette memoization
│   │   ├── useCardPerformance.ts     # Performance tier detection
│   │   └── useCardAnimations.ts      # Framer Motion variants
│   └── presets/
│       ├── index.ts
│       ├── TaxonomyCardContent.tsx   # Level 1: 4 variants (Realm/Layer/Trait/Arc)
│       ├── ClassCardContent.tsx      # Level 2: NodeClass "Blueprint"
│       ├── ArcClassCardContent.tsx   # Level 2: ArcClass "Connection Flow"
│       ├── FoundationCardContent.tsx # Level 3: Project, Brand, Design, Style
│       ├── StructureCardContent.tsx  # Level 3: Page, Block, Slot
│       ├── SemanticCardContent.tsx   # Level 3: Entity, Native, Persona, Channel
│       ├── InstructionCardContent.tsx# Level 3: BlockType, Instruction, Rules
│       ├── OutputCardContent.tsx     # Level 3: PageNative, BlockNative, Artifact
│       ├── KnowledgeCardContent.tsx  # Level 3: Containers (Set nodes)
│       ├── AtomChipContent.tsx       # Level 3: Term, Expression, SEO, GEO atoms
│       ├── LocaleCardContent.tsx     # Level 3: Locale, Culture, Style
│       └── GeoCardContent.tsx        # Level 3: Country, Region, Continent
│
├── effects/
│   ├── index.ts                      # Effects barrel export
│   ├── PremiumSchemaEffects.tsx      # Combined effects bundle
│   ├── TechCorners.tsx               # L-corners with glow (enhanced)
│   ├── Scanlines.tsx                 # Scanline drift
│   ├── GridPattern.tsx               # Blueprint grid
│   ├── HolographicShimmer.tsx        # Hover shimmer
│   ├── MatrixRain.tsx                # Canvas-based rain (enhanced)
│   ├── OuterGlow.tsx                 # Selection glow
│   ├── NeonBorderGlow.tsx            # NEW: Animated border gradient
│   ├── GlassmorphicBlur.tsx          # NEW: Backdrop filter + SVG
│   ├── FlowingParticles.tsx          # NEW: ArcClass particles (offset-path)
│   ├── OrbitalRings.tsx              # NEW: Realm concentric rings
│   ├── StackedPlanes.tsx             # NEW: Layer 3D parallax
│   ├── BorderMorph.tsx               # NEW: Trait style cycling
│   ├── RadiatingPulse.tsx            # NEW: ArcFamily outward flow
│   └── GlitchEffect.tsx              # NEW: Error state glitch
│
├── metrics/
│   ├── CoverageMeter.tsx             # Knowledge container coverage
│   ├── SEODifficultyGauge.tsx        # SEO keyword difficulty
│   ├── AIConfidenceGauge.tsx         # AI generation confidence
│   ├── KPIGauge.tsx                  # Project KPI meters
│   └── HofstedeGauge.tsx             # Culture dimension bars
│
├── specialized/
│   ├── JSONSchemaPreview.tsx         # BlockType JSON syntax highlight
│   ├── URLPathDisplay.tsx            # Page URL paths with locales
│   ├── SERPPreview.tsx               # SEO SERP card preview
│   ├── PersonaAvatar.tsx             # Audience persona avatar
│   ├── ChannelMatrix.tsx             # Channel capability grid
│   ├── AIPlatformStatus.tsx          # GEO AI platform status
│   ├── ColorSwatch.tsx               # Brand color swatches
│   ├── TypographyPreview.tsx         # Font preview
│   └── PassportCard.tsx              # Country passport style
│
├── TaxonomyNode.tsx                  # Level 1 wrapper
├── ClassNode.tsx                     # Level 2: NodeClass wrapper (existing)
├── ArcClassNode.tsx                  # Level 2: ArcClass wrapper
├── FoundationNode.tsx                # Level 3 wrappers...
├── StructureNode.tsx
├── SemanticNode.tsx
├── InstructionNode.tsx
├── OutputNode.tsx
├── KnowledgeNode.tsx
├── LocaleNode.tsx
└── GeoNode.tsx
```

---

## Part 9: Implementation Phases (Updated)

### Phase 1: Foundation & Infrastructure (Week 1)

- Extract shared components from ClassCardContent
- Create hooks for colors, performance, animations
- Implement performance tier system
- Create CardShell with render props

### Phase 2: Level 1 - Taxonomy Cards (Week 2)

- Create TaxonomyCardContent with 4 variants
- Implement OrbitalRings, StackedPlanes, BorderMorph, RadiatingPulse
- Register taxonomy nodes in node registry

### Phase 3: Level 2 - Schema Enhancement (Week 3)

- Create ArcClassCardContent with FlowingParticles
- Enhance ClassCardContent with NeonBorderGlow
- Add source→target visualization

### Phase 4: Level 3 - Data Layers (Weeks 4-5)

- Week 4: Foundation, Structure, Semantic layers
- Week 5: Instruction, Output, Knowledge, Locale/Geo layers

### Phase 5: Premium Effects & Polish (Week 6)

- Canvas-based MatrixRain optimization
- Glassmorphism + backdrop-filter support
- Performance profiling and optimization
- Accessibility audit (ARIA, reduced motion)
- Visual regression tests

---

## Appendix A: Component Quick Reference (Updated)

| Node Type | Card Component | Unique Effect | Width | Border |
|-----------|---------------|---------------|-------|--------|
| Realm | TaxonomyCardContent | OrbitalRings | 200px | solid |
| Layer | TaxonomyCardContent | StackedPlanes | 200px | solid |
| Trait | TaxonomyCardContent | BorderMorph | 200px | solid |
| ArcFamily | TaxonomyCardContent | RadiatingPulse | 200px | solid |
| NodeClass | ClassCardContent | Blueprint+NeonGlow | 240px | solid |
| ArcClass | ArcClassCardContent | FlowingParticles | 260px | solid |
| Project | FoundationCardContent | KPI gauges | 280px | solid |
| Brand | FoundationCardContent | Soul/Pitch/Voice | 280px | solid |
| BrandDesign | FoundationCardContent | ColorSwatches | 280px | solid |
| PromptStyle | FoundationCardContent | Platform badges | 280px | solid |
| Page | StructureCardContent | URL preview | 260px | solid |
| Block | StructureCardContent | Order indicator | 220px | solid |
| ContentSlot | StructureCardContent | Dashed interior | 220px | dashed |
| Entity | SemanticCardContent | Diamond tree | 260px | solid |
| EntityNative | SemanticCardContent | Locale flag | 240px | dashed |
| AudiencePersona | SemanticCardContent | Avatar+Pain/Goal | 260px | solid |
| ChannelSurface | SemanticCardContent | Channel matrix | 240px | solid |
| BlockType | InstructionCardContent | JSON preview | 260px | solid |
| BlockInstruction | InstructionCardContent | @ ref counter | 240px | solid |
| BlockRules | InstructionCardContent | Severity badges | 240px | solid |
| PromptArtifact | InstructionCardContent | Token count | 240px | dotted |
| PageNative | OutputCardContent | SERP preview | 260px | dotted |
| BlockNative | OutputCardContent | AI confidence | 220px | dotted |
| OutputArtifact | OutputCardContent | Deploy status | 240px | dotted |
| TermSet | KnowledgeCardContent | Coverage meter | 260px | double |
| Term | AtomChipContent | Hover expand | 120px | — |
| SEOKeyword | AtomChipContent | Difficulty gauge | 180px | — |
| GEOQuery | AtomChipContent | AI platforms | 200px | — |
| Locale | LocaleCardContent | Large flag | 200px | solid |
| Culture | LocaleCardContent | Hofstede gauges | 240px | double |
| Country | GeoCardContent | Passport style | 220px | double |
| Region | GeoCardContent | Mini flag grid | 200px | double |

---

## Appendix B: Color Palette Reference

```typescript
export const LAYER_COLORS = {
  config: '#64748b',      // slate-500
  locale: '#22c55e',      // green-500
  geography: '#f97316',   // orange-500
  knowledge: '#eab308',   // yellow-500
  foundation: '#8b5cf6',  // violet-500
  structure: '#3b82f6',   // blue-500
  semantic: '#06b6d4',    // cyan-500
  instruction: '#ec4899', // pink-500
  output: '#10b981',      // emerald-500
} as const;

export const REALM_COLORS = {
  shared: '#2aa198',      // solarized cyan
  org: '#6c71c4',         // solarized violet
} as const;

export const TRAIT_STYLES = {
  defined: { border: 'solid', width: 2 },
  authored: { border: 'dashed', width: 2 },
  imported: { border: 'double', width: 3 },
  generated: { border: 'dotted', width: 2 },
  retrieved: { border: 'dotted', width: 1 },
} as const;

export const ARC_FAMILY_COLORS = {
  ownership: '#3b82f6',   // blue-500
  localization: '#22c55e', // green-500
  semantic: '#06b6d4',    // cyan-500
  generation: '#ec4899',  // pink-500
  mining: '#eab308',      // yellow-500
} as const;
```

---

## Appendix C: Animation Timing Reference

```typescript
// Framer Motion spring presets
export const SPRING_CONFIGS = {
  snappy: { type: 'spring', stiffness: 500, damping: 30 },
  smooth: { type: 'spring', stiffness: 300, damping: 25 },
  bouncy: { type: 'spring', stiffness: 400, damping: 20, mass: 1.5 },
} as const;

// Duration presets by performance tier
export const DURATION_PRESETS = {
  fast: { duration: 0.1 },
  normal: { duration: 0.2 },
  slow: { duration: 0.3 },
  none: { duration: 0 },
} as const;

// Easing functions
export const EASINGS = {
  easeOut: [0.175, 0.885, 0.32, 1],
  easeInOut: [0.645, 0.045, 0.355, 1],
  bounce: [0.175, 0.885, 0.32, 2.2],
} as const;
```

---

*Document enhanced with 21st.dev component patterns, Framer Motion animations, and Context7 research on 2026-02-17*
*Research agents: 10 parallel brainstorm sessions completed*
