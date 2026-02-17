# Card Enhancement Plan - MagicUI Patterns

**Date**: 2026-02-17
**Status**: In Progress

## Objective

Enhance NovaNet Studio cards with premium visual effects inspired by MagicUI patterns while maintaining performance awareness via PerformanceContext.

## MagicUI Patterns to Incorporate

### 1. GlassCard 3D Transforms
```tsx
// From MagicUI research
[perspective:1000px]
[transform-style:preserve-3d]
group-hover:[transform:rotate3d(1,1,0,15deg)]
```

### 2. Glassmorphism
```tsx
backdrop-blur-sm
bg-white/10
border border-white/20
```

### 3. Electric Glow Borders
```tsx
// Animated gradient borders
background: linear-gradient(90deg, color1, color2, color1)
background-size: 200% 100%
animation: shimmer 2s infinite
```

### 4. Layered Glow Effects
```tsx
// Multiple blur layers for depth
<div className="absolute inset-0 blur-xl opacity-30" />
<div className="absolute inset-0 blur-md opacity-50" />
```

## Implementation Plan

### Phase 1: Shared Components (Priority: High)

| Component | Purpose | Performance Gate |
|-----------|---------|------------------|
| GlowEffect | Layered glow with blur | MEDIUM+ |
| GlassMorphism | Backdrop blur overlay | HIGH+ |
| ElectricBorder | Animated gradient border | ULTRA only |
| Perspective3D | 3D hover transforms | HIGH+ |

### Phase 2: CardShell Enhancement

Add optional props to CardShell:
- `enableGlow?: boolean` - Add glow effect
- `enableGlass?: boolean` - Add glassmorphism
- `enable3D?: boolean` - Add 3D perspective transforms
- `enableElectric?: boolean` - Add electric border

### Phase 3: Layer-Specific Enhancements

| Layer | Primary Effect | Secondary Effect |
|-------|---------------|------------------|
| Foundation | GlassMorphism | 3D transforms |
| Structure | Glow (cyan) | Electric border |
| Semantic | Glow (blue) | Glass overlay |
| Instruction | Electric (orange) | 3D transforms |
| Output | Glow (green) | Shimmer |
| Knowledge | Electric (purple) | Glass overlay |

## Performance Considerations

Effects are gated by PerformanceContext tier:

```
ULTRA   (0-50)   : All effects enabled
HIGH    (51-200) : Glow + Glass + 3D
MEDIUM  (201-500): Glow only
LOW     (501-2000): No effects
MINIMAL (2000+)  : Static rendering
```

## Files to Create/Modify

1. **Create**: `card/effects/GlowEffect.tsx`
2. **Create**: `card/effects/GlassMorphism.tsx`
3. **Create**: `card/effects/ElectricBorder.tsx`
4. **Create**: `card/effects/Perspective3D.tsx`
5. **Create**: `card/effects/index.ts`
6. **Modify**: `card/CardShell.tsx` - Add effect props
7. **Modify**: `card/animationPresets.ts` - Add shimmer/electric variants

## Success Criteria

- [ ] Effects respect PerformanceContext tiers
- [ ] No visual regressions in existing cards
- [ ] Type check passes
- [ ] All tests pass
- [ ] Cards look "premium" at ULTRA/HIGH tiers
