# Card Visual Coverage Design v0.13.1

**Date**: 2026-02-16
**Status**: In Progress
**Goal**: 100% visual coverage of 61 node types with modular, semantic CSS architecture

---

## Executive Summary

Current state: ~50% specialized card coverage, ~50% fallback to `StructuralCardContent`.
Target state: All 61 nodes have proper TaxonomyBadge + layer-appropriate styling.

---

## 1. Current Coverage Analysis

### Covered with Specialized Cards (~30 nodes)

| Layer | Cards | Nodes |
|-------|-------|-------|
| Foundation | BrandCardContent, BrandDesignCardContent, PromptStyleCardContent | 3/6 |
| Structure | PageCardContent, BlockCardContent, ContentSlotCardContent | 3/3 |
| Semantic | EntityCardContent, EntityNativeCardContent | 2/4 |
| Instruction | BlockInstructionCardContent, BlockTypeCardContent, BlockRulesCardContent, PromptArtifactCardContent | 4/4 |
| Output | PageNativeCardContent, BlockNativeCardContent, OutputArtifactCardContent | 3/3 |
| Knowledge | TermCardContent, ExpressionCardContent, SEOKeywordCardContent | 3/24 |

### Gaps (Using StructuralCardContent Fallback)

| Layer | Missing | Nodes |
|-------|---------|-------|
| Config | 2/3 | EntityCategory, SEOKeywordFormat |
| Locale | 5/6 | Culture, Style, Grammar, Formatting, Slugification |
| Geography | 7/7 | Continent, Region, GeoSubRegion, Country, GeoZone, GeoFeature, GeoPath |
| Knowledge | 21/24 | TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet, Pattern, CultureRef, Taboo, AudienceTrait, SEOKeywordMetrics, SEOCluster, GEOQuery, GEOAnswer, GEOMetrics, Adaptation |
| Foundation | 3/6 | Project, BrandPrinciples, ProjectNative |
| Semantic | 2/4 | AudiencePersona, ChannelSurface |

---

## 2. Architecture Design

### 2.1 Semantic Token System

Create CSS variables that map to taxonomy concepts:

```css
/* Layer tokens (fill color) */
--layer-config: theme('colors.slate.500');
--layer-locale: theme('colors.violet.500');
--layer-geography: theme('colors.emerald.500');
--layer-knowledge: theme('colors.violet.500');
--layer-foundation: theme('colors.blue.500');
--layer-structure: theme('colors.cyan.500');
--layer-semantic: theme('colors.orange.500');
--layer-instruction: theme('colors.yellow.500');
--layer-output: theme('colors.green.500');

/* Realm tokens (border color) */
--realm-shared: theme('colors.teal.400');
--realm-org: theme('colors.sky.400');

/* Trait tokens (animation + border style) */
--trait-defined-style: solid;
--trait-defined-animation: pulse;
--trait-authored-style: dashed;
--trait-authored-animation: breathe;
--trait-imported-style: dotted;
--trait-imported-animation: colorShift;
--trait-generated-style: double;
--trait-generated-animation: flowHorizontal;
--trait-retrieved-style: dotted;
--trait-retrieved-animation: rotate;
```

### 2.2 Card Component Hierarchy

```
CardShell (wrapper, context provider)
└── LayerCardWrapper (layer-specific gradient + realm border)
    └── TaxonomyBadge (visual encoding: layer+realm+trait+class)
        └── ClassCardContent (node-specific content)
            └── TraitGlow (trait-based animation)
```

### 2.3 CVA Variants Pattern (inspired by Magic MCP GradientCard)

```typescript
const layerCardVariants = cva(
  "relative overflow-hidden rounded-xl border-2 transition-all",
  {
    variants: {
      layer: {
        config: "bg-gradient-to-br from-slate-900/80 to-slate-800/60",
        locale: "bg-gradient-to-br from-violet-900/80 to-violet-800/60",
        geography: "bg-gradient-to-br from-emerald-900/80 to-emerald-800/60",
        knowledge: "bg-gradient-to-br from-violet-900/80 to-violet-800/60",
        foundation: "bg-gradient-to-br from-blue-900/80 to-blue-800/60",
        structure: "bg-gradient-to-br from-cyan-900/80 to-cyan-800/60",
        semantic: "bg-gradient-to-br from-orange-900/80 to-orange-800/60",
        instruction: "bg-gradient-to-br from-yellow-900/80 to-yellow-800/60",
        output: "bg-gradient-to-br from-green-900/80 to-green-800/60",
      },
      realm: {
        shared: "border-teal-500/60",
        org: "border-sky-500/60",
      },
      trait: {
        defined: "border-solid",
        authored: "border-dashed",
        imported: "border-dotted",
        generated: "[border-style:double]",
        retrieved: "border-dotted border-3",
      },
    },
  }
);
```

---

## 3. Implementation Phases

### Phase 1: CSS Foundation (P0) - 2h

- [ ] Create `design/tokens/layerTokens.ts` with layer colors and gradients
- [ ] Create `design/tokens/traitTokens.ts` with trait animations and borders
- [ ] Create `card/variants/layerCardVariants.ts` using CVA
- [ ] Update `tailwind.config.ts` with CSS variables

### Phase 2: Layer Card Wrapper (P1) - 3h

- [ ] Create `LayerCardWrapper.tsx` component
- [ ] Integrate with existing CardShell
- [ ] Add TaxonomyBadge to all cards missing it:
  - LocaleCardContent
  - StructuralCardContent
  - ClassCardContent
  - ProjectCardContent
  - RealmOrbitalCardContent
  - AttractorCardContent

### Phase 3: Missing Node Coverage (P2) - 6h

**Geography Layer (7 nodes):**
- [ ] GeographyHelpers.tsx (shared components: LocationBadge, CoordinateDisplay, RegionIndicator)
- [ ] ContinentCardContent.tsx
- [ ] RegionCardContent.tsx
- [ ] CountryCardContent.tsx
- [ ] GeoZoneCardContent.tsx (or use StructuralCardContent with TaxonomyBadge)

**Knowledge Layer Sets (6 containers):**
- [ ] KnowledgeSetHelpers.tsx (shared: DomainBadge, AtomCount, RegisterBadge)
- [ ] TermSetCardContent.tsx
- [ ] ExpressionSetCardContent.tsx
- [ ] PatternSetCardContent.tsx
- [ ] CultureSetCardContent.tsx
- [ ] TabooSetCardContent.tsx
- [ ] AudienceSetCardContent.tsx

**Knowledge Layer Atoms (existing Term, Expression - add missing):**
- [ ] PatternCardContent.tsx
- [ ] CultureRefCardContent.tsx
- [ ] TabooCardContent.tsx
- [ ] AudienceTraitCardContent.tsx

**Locale Settings (5 nodes):**
- [ ] LocaleSettingsHelpers.tsx (shared: RuleBadge, ExampleDisplay)
- [ ] CultureCardContent.tsx
- [ ] StyleCardContent.tsx
- [ ] GrammarCardContent.tsx
- [ ] FormattingCardContent.tsx
- [ ] SlugificationCardContent.tsx

**Config Layer (2 nodes):**
- [ ] EntityCategoryCardContent.tsx
- [ ] SEOKeywordFormatCardContent.tsx

### Phase 4: Visual Encoding Update (P3) - 2h

- [ ] Update visual-encoding.yaml with new trait animations
- [ ] Sync taxonomyColors.ts with design tokens
- [ ] Generate TypeScript types from YAML
- [ ] Add Mermaid documentation

---

## 4. File Structure

```
apps/studio/src/
├── design/
│   └── tokens/
│       ├── layerTokens.ts      # Layer colors, gradients
│       ├── traitTokens.ts      # Trait animations, borders
│       └── index.ts            # Re-exports
├── components/graph/nodes/card/
│   ├── variants/
│   │   └── layerCardVariants.ts  # CVA variants
│   ├── LayerCardWrapper.tsx      # Layer-specific wrapper
│   ├── presets/
│   │   ├── geography/
│   │   │   ├── GeographyHelpers.tsx
│   │   │   ├── ContinentCardContent.tsx
│   │   │   ├── RegionCardContent.tsx
│   │   │   └── CountryCardContent.tsx
│   │   ├── locale/
│   │   │   ├── LocaleSettingsHelpers.tsx
│   │   │   ├── CultureCardContent.tsx
│   │   │   ├── StyleCardContent.tsx
│   │   │   └── GrammarCardContent.tsx
│   │   └── knowledge/
│   │       ├── KnowledgeSetHelpers.tsx
│   │       ├── TermSetCardContent.tsx
│   │       └── ...
```

---

## 5. Design Decisions

### 5.1 Why CVA (class-variance-authority)?

- Type-safe variant definitions
- Composable with Tailwind
- Used successfully in Magic MCP GradientCard component
- Clean separation of concerns

### 5.2 Why Layer Gradients?

- Visual hierarchy: Layer = primary identification (ADR-005)
- Gradient provides depth and premium feel
- Dark mode compatible (from-900/80 to-800/60)
- Matches existing card aesthetics

### 5.3 Why Trait Animations?

- TraitGlow already implemented with 5 animation modes
- Semantic meaning: animation reflects data origin
- Accessible: supplements color/border with motion
- Performance-aware: respects performanceConfig

---

## 6. Success Criteria

- [ ] All 61 node types render with correct TaxonomyBadge
- [ ] Layer color visible as gradient background
- [ ] Realm color visible as border
- [ ] Trait style visible as border pattern
- [ ] Trait animation active on hover/selection
- [ ] No hardcoded colors in card components
- [ ] All colors reference design tokens
- [ ] Performance: animations respect tier system

---

## 7. References

- ADR-005: Trait-Based Visual Encoding
- ADR-024: Trait = Data Origin
- visual-encoding.yaml: Source of truth
- taxonomyColors.ts: Current implementation
- Magic MCP GradientCard: Inspiration
