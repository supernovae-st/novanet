# Page-Entity Architecture & @ Reference System

**Status**: Approved → ADR-028
**Version**: v0.12.4
**Date**: 2026-02-13
**ADR**: [ADR-028](./.claude/rules/novanet-decisions.md#adr-028-page-entity-architecture)

**v0.12.4 Changes** (Gap Analysis & Corrections):
- **Country node CREATED**: Added `country.yaml` (shared/geography) with 249 ISO 3166-1 countries
- **Country seed generated**: `29-countries.cypher` links countries to GeoRegion/Continent via IN_REGION/IN_CONTINENT
- **HAS_BLOCK property renamed**: `position` → `order` for semantic clarity
- **Page arc naming**: `USES_ENTITY` (N:N) kept for multiple refs, `REPRESENTS` (1:1) for main Entity
- **PageStructure/PageInstruction**: Confirmed as CALCULATED (not stored), existing YAMLs to be DELETED

**v0.12.3 Changes** (Research-backed refinements):
- **Refined visual_prompt schema** based on Midjourney, DALL-E 3, Sora, Stable Diffusion best practices
- Added **weighted_modifiers** with Stable Diffusion weight syntax `(term:1.4)`
- Added **structured negative_prompts** (content, technical, style, cultural)
- Added **cinematography** section for Sora video generation (camera movements, shot types)
- Added **platform_hints** for cross-platform compatibility (Midjourney, DALL-E, SD, Sora)
- Added **prompt compilation pipeline** showing how visual_prompt converts to platform-specific formats

**v0.12.2 Changes**:
- Brand Architecture (Atlas Pattern): Brand (Soul + Pitch + Voice) + BrandDesign + BrandPrinciples
- PromptStyle system for AI image/video generation
- Geographic cultural styles on Continent/GeoRegion/GeoSubRegion
- Semantic links to AudiencePersona (no BrandAudience needed)
- **Geographic Visual Prompts**: `visual_prompt` property for AI-specific generation presets (image, video, illustration, product_3d)
- Merge algorithm: Brand.PromptStyle + Geographic visual_prompt hierarchy

---

## Overview

This document defines the relationship between Pages and Entities in NovaNet, and introduces a comprehensive @ reference system for content generation.

**Key Principles**:
1. Page ↔ Entity = 1:1 obligatoire
2. Slug = Entity.key (source of truth unique)
3. Separation of concerns: BlockType (contraintes) vs BlockInstruction (créativité)
4. @ refs: `@type:key` (inject) vs `[@page:X]` (link)
5. Order on arc: `[:HAS_BLOCK {order}]` (source of truth unique)

---

## Gap Analysis (v0.12.4)

Analysis comparing this plan vs current codebase state.

### Findings & Resolutions

| # | Gap | Current State | Plan State | Resolution | Status |
|---|-----|---------------|------------|------------|--------|
| 1 | **REPRESENTS arc missing** | Page has USES_ENTITY (N:N) | Page needs REPRESENTS (1:1) | ADD REPRESENTS to page.yaml | **DONE** |
| 2 | **Country node missing** | GeoSubRegion → Locale directly | Plan requires Country between | CREATED country.yaml + 29-countries.cypher (249 countries) | **DONE** |
| 3 | **HAS_BLOCK property** | Uses `position` | Plan says `order` | RENAME position → order | **DONE** |
| 4 | **PageStructure YAML** | File exists in org/instruction | Plan: CALCULATED (not stored) | DELETE page-structure.yaml | **DONE** |
| 5 | **PageInstruction YAML** | File exists in org/instruction | Plan: CALCULATED (not stored) | DELETE page-instruction.yaml | **DONE** |
| 6 | **BrandIdentity → Brand** | BrandIdentity exists | Plan: Brand with Atlas Pattern | RENAME + restructure | PENDING (Phase 2) |

### User Decisions

- **Q1**: PageStructure/PageInstruction files → **DELETE** (calculated at runtime)
- **Q2**: Semantic arc naming → **REPRESENTS** (1:1 main) + **USES_ENTITY** (N:N block refs)
- **Q3**: Country node → **CREATE** with all 249 countries
- **Q4**: HAS_BLOCK property → **RENAME** `position` to `order`

### Geographic Hierarchy (After Country Addition)

```
Continent (6) ──[:IN_CONTINENT]──▶ GeoRegion (22) ──[:IN_REGION]──▶ GeoSubRegion (~15)
                                                                            │
                                                                            ▼
                                       Locale (200+) ◀──[:IN_COUNTRY]── Country (249)
```

---

## Core Decisions

### Decision 1: Page ↔ Entity = 1:1 Obligatoire

Every Page MUST have exactly one Entity principale via `[:REPRESENTS]`.

```
Page (structure, defined) ──[:REPRESENTS]──▶ Entity (semantic, defined)
```

**Rationale**:
- 100% des pages ont une Entity (même /contact, /legal)
- La distinction est le TYPE d'Entity (via EntityCategory), pas "avec/sans"
- Le slug est partagé: `Page.key = Entity.key`

**EntityCategory examples**:

| Category | Pages |
|----------|-------|
| product | /qr-code-generator |
| feature | /api-documentation |
| pricing | /pricing |
| legal | /terms-of-service |
| support | /contact |
| index | /blog |

### Decision 2: Slug = Entity Key (Source of Truth)

```
Entity.key = "qr-code-generator"
     ↓
Page.key = derived from Entity.key
     ↓
URL = /{Entity.key} = /qr-code-generator
```

- Entity est la source of truth pour le slug
- Page hérite du slug via [:REPRESENTS]
- Pas de duplication, cohérence garantie

### Decision 3: SEO Keywords sur Entity

```
Entity ──[:HAS_KEYWORD]──▶ SEOKeyword (primary + secondary)
       ──[:HAS_CONTENT]──▶ EntityContent@locale
```

- Entity porte les SEOKeywords (pas Page)
- Page hérite des keywords via son Entity principale
- Requête: `Page→Entity→SEOKeyword`

### Decision 4: Liens entre Pages via @ References

Les liens entre Pages sont EXPLICITES, déterminés par les `[@page:X]` refs dans BlockInstruction.

**Pas d'héritage automatique** depuis Entity[:RELATES_TO]Entity.

### Decision 5: Ordre sur l'Arc (Source of Truth Unique)

```
Page ──[:HAS_BLOCK {order: 1}]──▶ Block (hero)
     ──[:HAS_BLOCK {order: 2}]──▶ Block (features)
     ──[:HAS_BLOCK {order: 3}]──▶ Block (cta)
```

- L'ordre est UNIQUEMENT sur l'arc `[:HAS_BLOCK {order}]`
- PageStructure = CALCULÉ (pas un node stocké)
- PageInstruction = CALCULÉ (pas un node stocké)

### Decision 6: Block.key = Composite (Page:BlockType:Index)

```
Block.key = "{page_key}:{block_type}:{index}"
```

Exemples:
- `homepage:hero:1` (premier hero sur homepage)
- `pricing:hero:1` (hero sur pricing, différent de homepage)
- `homepage:testimonials:1` (premier testimonials)
- `homepage:testimonials:2` (deuxième testimonials, si répété)

**Rationale**:
- Unique globalement (pas de collision entre pages)
- Parseable (extract page, type, index)
- Permet plusieurs blocks du même type sur une page

### Decision 7: Brand ↔ Project Relationship

```
Project ──[:HAS_BRAND]──▶ Brand (1:1)
```

- Un Project a exactement un Brand
- Brand définit voice, tone, humor, formality, values
- Tous les BlockTypes du Project héritent du Brand

---

## Architecture en Couches

### Principe: Séparation des Concerns

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  COUCHES DE DÉFINITION                                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Brand (1 par projet)                                                           │
│  │ Définit: voice, tone, humor, formality, values                              │
│  │ Scope: défauts GLOBAUX                                                      │
│  │                                                                              │
│  └──▶ BlockType (~30 templates)                                                │
│       │ Définit: schema JSON, behaviors, lengths, formats, seo rules           │
│       │ Scope: contraintes TECHNIQUES réutilisables                            │
│       │                                                                         │
│       └──▶ BlockInstruction (~200, spécifique par Block)                       │
│            Définit: @ refs, quoi dire, pourquoi                                │
│            Scope: instructions CRÉATIVES par Page                              │
│                                                                                 │
│  Héritage: Brand → BlockType → BlockInstruction                                │
│  (chaque niveau peut override)                                                  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Brand (replaces BrandIdentity)

Brand suit le pattern **Atlas** (Soul/Principles/Agents) pour une identité complète.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  BRAND ARCHITECTURE (Atlas Pattern)                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Brand (org/foundation, defined)                                                │
│  │ Soul: who we are, why we exist                                              │
│  │ Pitch: what, for whom, how (positioning)                                    │
│  │                                                                              │
│  ├──[:HAS_DESIGN]──────────▶ BrandDesign (1:1)                                 │
│  │   Visual direction + design tokens                                          │
│  │                                                                              │
│  ├──[:HAS_PRINCIPLES]──────▶ BrandPrinciples (1:1)                             │
│  │   Decision heuristics for agents                                            │
│  │                                                                              │
│  ├──[:HAS_PROMPT_STYLE]────▶ PromptStyle* (1:N)                                │
│  │   AI generation presets (image, video, illustration)                        │
│  │                                                                              │
│  └──[:TARGETS_PERSONA]─────▶ AudiencePersona* (semantic link)                  │
│      Target audiences via existing org/semantic node                           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### Brand (Soul + Pitch)

Définit l'identité fondamentale et le positionnement du projet.

```yaml
brand:
  # ═══════════════════════════════════════════════════════════════════════════
  # SOUL (Who we are)
  # ═══════════════════════════════════════════════════════════════════════════
  purpose: "Democratize QR code creation for everyone"     # Raison d'être
  mission: "Make professional QR codes accessible"         # Comment on y arrive
  vision: "A world where every business has smart QR"      # Où on va

  # ═══════════════════════════════════════════════════════════════════════════
  # PITCH (Positioning)
  # ═══════════════════════════════════════════════════════════════════════════
  what: "AI-powered QR code generator"                     # Product description
  for_whom: "Marketers and small businesses"               # Target audience
  how: "Using AI to create beautiful, scannable codes"     # Value proposition
  elevator_pitch: "QR Code AI creates stunning QR codes in seconds using AI"
  differentiators:                                         # Vs competitors
    - "AI-generated designs"
    - "No design skills needed"
    - "Analytics included"

  # ═══════════════════════════════════════════════════════════════════════════
  # VOICE (How we communicate)
  # ═══════════════════════════════════════════════════════════════════════════
  voice: expert           # Comment on parle (expert, friendly, authoritative)
  tone: professional      # Registre par défaut (professional, casual, playful)
  humor: subtle           # Niveau d'humour (none, subtle, bold)
  formality: formal       # Tutoiement/vouvoiement
  values:                 # Valeurs de marque
    - innovation
    - trust
    - simplicity
```

#### BrandDesign (Visual Direction)

Définit la direction artistique pour génération d'images/vidéos.

```yaml
brand_design:
  # ═══════════════════════════════════════════════════════════════════════════
  # DESIGN PHILOSOPHY
  # ═══════════════════════════════════════════════════════════════════════════
  philosophy: "Clean, modern, approachable tech"
  influences: ["Apple", "Stripe", "Linear"]

  # ═══════════════════════════════════════════════════════════════════════════
  # VISUAL DIRECTION (for AI generation)
  # ═══════════════════════════════════════════════════════════════════════════
  style_keywords: ["modern", "minimal", "tech-forward", "friendly"]
  style_mood: "Professional yet approachable, innovative but not intimidating"

  # Image generation defaults
  image_style: "flat illustration, soft gradients, geometric shapes"
  image_do: ["Use brand colors", "Abstract tech elements", "Clean compositions"]
  image_dont: ["Stock photos", "Cluttered layouts", "Photorealistic people"]

  # ═══════════════════════════════════════════════════════════════════════════
  # DESIGN TOKENS (3-tier hierarchy)
  # ═══════════════════════════════════════════════════════════════════════════
  tokens:
    # Tier 1: Primitives (raw values)
    primitives:
      colors:
        indigo_500: "#6366F1"
        pink_500: "#EC4899"
        emerald_500: "#10B981"
        slate_900: "#0F172A"
        white: "#FFFFFF"
      spacing: [4, 8, 12, 16, 24, 32, 48, 64]
      radii: [4, 8, 12, 16, 24]

    # Tier 2: Semantic (purpose-driven)
    semantic:
      colors:
        primary: "{primitives.colors.indigo_500}"
        secondary: "{primitives.colors.pink_500}"
        accent: "{primitives.colors.emerald_500}"
        background: "{primitives.colors.white}"
        text: "{primitives.colors.slate_900}"
      moods:
        trust: ["#2563EB", "#3B82F6", "#60A5FA"]      # Blues
        energy: ["#EF4444", "#F97316", "#FBBF24"]     # Warm
        calm: ["#10B981", "#14B8A6", "#06B6D4"]       # Cool greens

    # Tier 3: Component (specific usage)
    component:
      button:
        primary_bg: "{semantic.colors.primary}"
        primary_text: "{primitives.colors.white}"
        radius: "{primitives.radii[3]}"
      card:
        bg: "{primitives.colors.white}"
        border: "1px solid {semantic.colors.primary}20"
        shadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)"

  # Typography
  typography:
    font_primary: "Inter"
    font_secondary: "Space Grotesk"
    font_mono: "JetBrains Mono"
    scale: [12, 14, 16, 18, 20, 24, 30, 36, 48, 60]

  # UI patterns
  ui:
    border_radius: "12px"
    shadow_style: "subtle"      # subtle | pronounced | none
    animation_style: "smooth"   # smooth | snappy | playful
```

#### BrandPrinciples (Decision Heuristics)

Règles pour la prise de décision des agents LLM (pattern Atlas "Agents").

```yaml
brand_principles:
  # ═══════════════════════════════════════════════════════════════════════════
  # HEURISTICS (for LLM decision-making)
  # ═══════════════════════════════════════════════════════════════════════════
  heuristics:
    - trigger: "choosing between features"
      rule: "Prioritize simplicity over power"
      rationale: "Our users want quick results, not complexity"

    - trigger: "writing error messages"
      rule: "Be helpful, not technical"
      rationale: "Users shouldn't need to understand the system"

    - trigger: "pricing communication"
      rule: "Lead with value, not cost"
      rationale: "We're not the cheapest, we're the best"

    - trigger: "competitor mentions"
      rule: "Acknowledge, don't attack"
      rationale: "Confidence, not insecurity"

  # ═══════════════════════════════════════════════════════════════════════════
  # DO/DON'T RULES
  # ═══════════════════════════════════════════════════════════════════════════
  do:
    - "Use concrete examples over abstract explanations"
    - "Show results, not process"
    - "Celebrate user success"
    - "Keep CTAs action-oriented"

  dont:
    - "Use jargon without explanation"
    - "Make promises we can't keep"
    - "Blame the user for errors"
    - "Use fear-based marketing"
```

#### PromptStyle (AI Generation Presets)

Presets pour génération d'images/vidéos avec granularité fine.

```yaml
prompt_style:
  key: "prompt-style-hero-illustration"
  name: "Hero Illustration"
  media_type: image         # image | video | animation

  # ═══════════════════════════════════════════════════════════════════════════
  # CORE COMPONENTS (granular control)
  # ═══════════════════════════════════════════════════════════════════════════

  # Style (overall aesthetic)
  style:
    base: "flat vector illustration"
    modifiers: ["isometric", "geometric", "minimal"]
    negative: ["photorealistic", "3D render", "sketch"]

  # Subject (what's depicted)
  subject:
    primary: "abstract QR code visualization"
    secondary: ["floating UI elements", "tech patterns"]
    avoid: ["human faces", "hands", "text"]

  # Environment (setting/background)
  environment:
    type: "abstract"        # studio | outdoor | indoor | abstract
    elements: ["gradient background", "subtle grid pattern"]
    depth: "shallow"        # shallow | medium | deep

  # Lighting
  lighting:
    type: "soft"           # soft | hard | dramatic | natural
    direction: "top-front"
    color_temp: "neutral"   # warm | neutral | cool

  # Color palette (references design tokens)
  color_palette:
    mode: "brand"          # brand | mood | custom
    primary: "@design.tokens.semantic.colors.primary"
    accent: "@design.tokens.semantic.colors.accent"
    mood: "@design.tokens.semantic.moods.trust"

  # Composition
  composition:
    aspect_ratio: "16:9"
    focal_point: "center"
    negative_space: "generous"
    rule: "rule_of_thirds"

  # Mood/Atmosphere
  mood:
    primary: "professional"
    secondary: "innovative"
    energy: "medium"        # low | medium | high

  # Quality/Technical
  quality:
    resolution: "high"
    detail: "medium"
    noise: "none"
```

---

## Geographic Cultural Styles

Les nodes géographiques portent des styles culturels pour la génération locale.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  GEOGRAPHIC STYLE INHERITANCE                                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Continent ─────▶ GeoRegion ─────▶ GeoSubRegion ─────▶ Locale                  │
│  │                │                │                    │                       │
│  │ cultural_style │ cultural_style │ cultural_style    │ (inherits)            │
│  │                │                │                    │                       │
│  └────────────────┴────────────────┴────────────────────┘                       │
│                                                                                 │
│  At generation time:                                                            │
│  1. Start with Brand.PromptStyle (project defaults)                            │
│  2. Override with geographic cultural_style (if specified)                     │
│  3. Most specific wins: Locale > SubRegion > Region > Continent                │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Cultural Style Properties (on Geography Nodes)

```yaml
# Added to Continent, GeoRegion, GeoSubRegion nodes
cultural_style:
  # Color preferences
  color_preferences:
    primary_colors: ["red", "gold"]        # Culturally significant colors
    avoid_colors: ["white"]                # Colors with negative connotations
    color_meanings:
      red: "prosperity, luck"
      white: "mourning"

  # Visual style
  visual_style:
    aesthetic: "ornate"                    # minimal | ornate | balanced
    patterns: ["geometric", "floral"]      # Cultural pattern preferences
    imagery: ["nature", "family"]          # Preferred imagery themes

  # Typography hints
  typography:
    style: "traditional"                   # modern | traditional | mixed
    script_direction: "ltr"                # ltr | rtl | vertical
    formality: "high"                      # low | medium | high

  # Cultural codes
  cultural_codes:
    formality_level: "high"
    humor_style: "subtle"
    directness: "indirect"
    hierarchy_importance: "high"
```

### Geographic Visual Prompts (AI Generation Presets)

Presets visuels **spécifiques à la génération AI** pour chaque niveau géographique.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  GEOGRAPHIC VISUAL PROMPT INHERITANCE                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Continent ──▶ GeoRegion ──▶ GeoSubRegion ──▶ Country ──▶ Locale               │
│      │             │              │              │           │                  │
│      │  visual_    │  visual_     │  visual_     │  visual_  │ (inherits)      │
│      │  prompt     │  prompt      │  prompt      │  prompt   │                  │
│      │             │              │              │           │                  │
│      │  (broad)    │ (regional)   │ (specific)   │ (local)   │                  │
│                                                                                 │
│  Merge strategy at generation:                                                  │
│  1. Start with Brand.PromptStyle (project defaults)                            │
│  2. Layer geographic visual_prompt (most specific wins)                        │
│  3. Locale > Country > SubRegion > Region > Continent                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### Visual Prompt Schema (JSON property on Geographic nodes)

**Research-backed schema** integrating Midjourney, DALL-E 3, Sora, and Stable Diffusion best practices.

```yaml
# visual_prompt property on Continent, GeoRegion, GeoSubRegion, Country
visual_prompt:
  # ═══════════════════════════════════════════════════════════════════════════
  # IMAGE GENERATION (Midjourney + DALL-E 3 + Stable Diffusion)
  # ═══════════════════════════════════════════════════════════════════════════
  image:
    # ─────────────────────────────────────────────────────────────────────────
    # SUBJECT (Midjourney: "what is depicted")
    # ─────────────────────────────────────────────────────────────────────────
    subject:
      primary: "abstract QR code visualization"
      secondary: ["floating UI elements", "tech patterns"]
      action: "emerging from light"                    # Midjourney: action verb
      human_representation: "stylized"                 # realistic | stylized | abstract | avoid
      nature_elements: ["cherry blossom", "bamboo", "water", "mountains"]
      symbols: ["torii", "koi", "crane", "wave"]
      architectural_style: "traditional Japanese"
      avoid: ["human faces", "hands", "text"]          # Cultural sensitivity

    # ─────────────────────────────────────────────────────────────────────────
    # ENVIRONMENT (Midjourney: "setting/background")
    # ─────────────────────────────────────────────────────────────────────────
    environment:
      type: "abstract"                                 # studio | outdoor | indoor | abstract
      setting: "minimal zen space"
      elements: ["gradient background", "subtle grid pattern"]
      depth: "layered"                                 # shallow | medium | deep | layered
      atmosphere: "ethereal"

    # ─────────────────────────────────────────────────────────────────────────
    # STYLE (Midjourney + Stable Diffusion weights)
    # ─────────────────────────────────────────────────────────────────────────
    style:
      base: "flat vector illustration"
      modifiers:                                       # Appended to prompt
        - "traditional Japanese aesthetic"
        - "ukiyo-e influence"
        - "wabi-sabi imperfection"
      weighted_modifiers:                              # Stable Diffusion weight syntax
        - { term: "minimalist", weight: 1.3 }
        - { term: "zen aesthetic", weight: 1.2 }
        - { term: "geometric precision", weight: 0.8 }

    # ─────────────────────────────────────────────────────────────────────────
    # NEGATIVE PROMPTS (Stable Diffusion: exclusions)
    # ─────────────────────────────────────────────────────────────────────────
    negative_prompts:
      content: ["western corporate style", "stock photo", "clip art"]
      technical: ["blurry", "low resolution", "watermark", "signature"]
      style: ["photorealistic", "3D render", "sketch", "cartoon"]
      cultural: ["culturally insensitive", "stereotypical"]

    # ─────────────────────────────────────────────────────────────────────────
    # COLOR PALETTE (semantic + cultural)
    # ─────────────────────────────────────────────────────────────────────────
    color_palette:
      primary: ["#C41E3A", "#FFD700", "#1C1C1C"]       # Red, Gold, Black
      secondary: ["#F5F5DC", "#8B4513"]                # Beige, Brown
      accent: ["#FF69B4", "#98FB98"]                   # Sakura, Green
      avoid: ["#FFFFFF alone", "#0000FF bright"]
      seasonal:                                        # Cultural seasonal colors
        spring: ["#FFB7C5", "#98FB98"]                 # Sakura, Fresh green
        summer: ["#4169E1", "#228B22"]                 # Hydrangea, Deep green
        autumn: ["#FF6B35", "#8B0000"]                 # Momiji, Deep red
        winter: ["#FFFFFF", "#1C1C1C"]                 # Snow, Black

    # ─────────────────────────────────────────────────────────────────────────
    # COMPOSITION (Midjourney: visual structure)
    # ─────────────────────────────────────────────────────────────────────────
    composition:
      balance: "asymmetric"                            # symmetric | asymmetric | dynamic
      negative_space: "abundant"                       # minimal | moderate | abundant (ma 間)
      focal_point: "off-center"                        # center | off-center | distributed
      rule: "rule_of_thirds"                           # rule_of_thirds | golden_ratio | centered
      depth: "layered"                                 # flat | layered | deep
      framing: "wide"                                  # tight | medium | wide | panoramic

    # ─────────────────────────────────────────────────────────────────────────
    # LIGHTING (Midjourney: mood lighting)
    # ─────────────────────────────────────────────────────────────────────────
    lighting:
      type: "soft diffused"                            # soft | hard | dramatic | natural
      direction: "ambient"                             # top | side | back | ambient | rim
      color_temp: "warm"                               # warm | neutral | cool
      mood: "contemplative"
      time_of_day: "golden hour"                       # dawn | morning | noon | golden | dusk | night
      shadows: "soft"                                  # none | soft | hard | dramatic

    # ─────────────────────────────────────────────────────────────────────────
    # TEXTURE (detail level)
    # ─────────────────────────────────────────────────────────────────────────
    texture:
      style: "textured"                                # smooth | textured | rough
      detail_level: "intricate"                        # minimal | moderate | intricate
      organic_elements: true
      materials: ["paper", "silk", "wood grain"]

    # ─────────────────────────────────────────────────────────────────────────
    # MOOD/ATMOSPHERE (Midjourney: emotional tone)
    # ─────────────────────────────────────────────────────────────────────────
    mood:
      primary: "serene"                                # energetic | calm | mysterious | serene
      secondary: "contemplative"
      energy: "low"                                    # low | medium | high
      emotion: "peaceful"

    # ─────────────────────────────────────────────────────────────────────────
    # QUALITY (DALL-E 3 API parameters)
    # ─────────────────────────────────────────────────────────────────────────
    quality:
      dalle_style: "natural"                           # natural | vivid (DALL-E 3)
      dalle_quality: "hd"                              # standard | hd (DALL-E 3)
      resolution: "high"
      detail: "medium"
      noise: "none"

    # ─────────────────────────────────────────────────────────────────────────
    # PLATFORM PARAMETERS (Midjourney specific)
    # ─────────────────────────────────────────────────────────────────────────
    parameters:
      aspect_ratio: "16:9"                             # --ar (Midjourney)
      stylize: 100                                     # --s 0-1000 (Midjourney)
      chaos: 0                                         # --chaos 0-100 (Midjourney)
      weird: 0                                         # --weird 0-3000 (Midjourney)
      cfg_scale: 7.5                                   # CFG scale (Stable Diffusion)
      steps: 30                                        # Inference steps (Stable Diffusion)

  # ═══════════════════════════════════════════════════════════════════════════
  # VIDEO GENERATION (Sora + Gen-3 Alpha)
  # ═══════════════════════════════════════════════════════════════════════════
  video:
    # ─────────────────────────────────────────────────────────────────────────
    # SCENE DESCRIPTION (Sora: "what happens")
    # ─────────────────────────────────────────────────────────────────────────
    scene:
      description: "Gentle cherry blossoms falling in a zen garden"
      action: "slowly drifting"                        # Main action verb
      subjects: ["cherry blossoms", "stone lantern", "koi pond"]
      setting: "traditional Japanese garden"
      time_period: "spring afternoon"

    # ─────────────────────────────────────────────────────────────────────────
    # CINEMATOGRAPHY (Sora: camera work)
    # ─────────────────────────────────────────────────────────────────────────
    cinematography:
      camera_movement: "slow dolly"                    # static | pan | tilt | dolly | crane | handheld | drone
      camera_angle: "eye level"                        # low | eye | high | bird | worm
      shot_type: "wide establishing"                   # close-up | medium | wide | extreme wide
      movement_speed: "slow"                           # very slow | slow | medium | fast
      direction: "left to right"                       # left-right | right-left | forward | backward

    # ─────────────────────────────────────────────────────────────────────────
    # MOTION (pace and rhythm)
    # ─────────────────────────────────────────────────────────────────────────
    motion:
      pace: "contemplative"                            # slow | moderate | fast | dynamic
      rhythm: "flowing"                                # rhythmic | flowing | staccato | random
      transitions: "dissolve"                          # cut | dissolve | fade | wipe | morph
      transition_duration: "slow"

    # ─────────────────────────────────────────────────────────────────────────
    # TEMPORAL (duration and timing)
    # ─────────────────────────────────────────────────────────────────────────
    temporal:
      duration_preference: "longer"                    # short (5s) | medium (10s) | longer (15s+)
      loop_friendly: true                              # For social media loops
      beat_sync: false
      time_manipulation: "normal"                      # normal | slow-mo | timelapse | reverse

    # ─────────────────────────────────────────────────────────────────────────
    # VISUAL STYLE (consistent with image)
    # ─────────────────────────────────────────────────────────────────────────
    visual_style:
      aesthetic: "cinematic"                           # cinematic | documentary | artistic | anime
      color_grading: "warm muted"                      # Color grading preset
      film_grain: "subtle"                             # none | subtle | moderate | heavy
      depth_of_field: "shallow"                        # deep | medium | shallow

    # ─────────────────────────────────────────────────────────────────────────
    # AUDIO HINTS (for AI video generators)
    # ─────────────────────────────────────────────────────────────────────────
    audio_style:
      music_genre: "ambient traditional"
      instruments: ["koto", "shakuhachi", "taiko"]
      tempo: "slow"                                    # BPM range: 60-80
      mood: "serene"
      sound_effects: ["water", "wind chimes", "birds"]
      voice_over: false

  # ═══════════════════════════════════════════════════════════════════════════
  # ILLUSTRATION GENERATION
  # ═══════════════════════════════════════════════════════════════════════════
  illustration:
    style_base: "traditional ink painting"
    techniques:
      - "sumi-e brush strokes"
      - "gold leaf accents"
      - "woodblock print influence"
    weighted_techniques:                               # Stable Diffusion weights
      - { term: "ukiyo-e style", weight: 1.4 }
      - { term: "washi paper texture", weight: 1.2 }
    line_work: "expressive"                            # clean | expressive | sketchy | none
    fill_style: "watercolor wash"                      # flat | gradient | watercolor | textured
    stroke_weight: "varied"                            # thin | medium | thick | varied
    color_mode: "limited palette"                      # full | limited | monochrome | duotone

  # ═══════════════════════════════════════════════════════════════════════════
  # 3D/PRODUCT VISUALIZATION
  # ═══════════════════════════════════════════════════════════════════════════
  product_3d:
    environment: "minimal zen garden"
    setting_style: "studio"                            # studio | lifestyle | abstract
    materials: ["wood", "paper", "ceramic", "lacquer"]
    surface_finish: "matte"                            # glossy | matte | satin | textured
    lighting_setup: "soft studio"                      # soft studio | dramatic | natural | HDRI
    background: "neutral warm"
    props: ["bamboo", "stone", "moss"]
    camera_angle: "three-quarter"                      # front | three-quarter | top-down | isometric
    render_style: "photorealistic"                     # photorealistic | stylized | minimal

  # ═══════════════════════════════════════════════════════════════════════════
  # CROSS-PLATFORM HINTS (multi-model compatibility)
  # ═══════════════════════════════════════════════════════════════════════════
  platform_hints:
    midjourney:
      version: "v7"                                    # Latest Midjourney
      style_raw: false                                 # --style raw
      niji: false                                      # --niji for anime
      stylize: 100                                     # --s 0-1000
      chaos: 0                                         # --chaos 0-100
    dalle:
      model: "dall-e-3"
      size: "1792x1024"                                # landscape
      style: "natural"                                 # natural | vivid
      quality: "hd"                                    # standard | hd
    stable_diffusion:
      model: "SDXL"
      sampler: "DPM++ 2M Karras"
      clip_skip: 2
      cfg_scale: 7.5
      steps: 30
    gemini:                                            # Nano Banana (Gemini 2.5/3)
      model: "gemini-2.5-flash-image"                  # or gemini-3-pro-image-preview
      resolution: "1024x1024"                          # Up to 4096x4096 for Pro
      # Prompt formula: [Action] + [Subject] + [Pose] + [Setting] + [Style] + [Technical]
      # Example: "Create an ultra-HD portrait with cinematic glow, shot on 35mm film"
    sora:
      duration: "10s"
      resolution: "1080p"
    ideogram:                                          # Best for typography
      model: "ideogram-v2"
      style: "realistic"                               # realistic | design | anime
    seedream:                                          # Short, precise prompts
      model: "seedream-4.0"
      # Keep prompts short and precise for best results

  # ═══════════════════════════════════════════════════════════════════════════
  # PROMPT TEMPLATES (platform-specific compilation)
  # ═══════════════════════════════════════════════════════════════════════════
  prompt_templates:
    # Gemini/Nano Banana formula: [Action] + [Subject] + [Pose] + [Setting] + [Style] + [Technical]
    gemini_formula:
      action: "Create"                                 # Create | Transform | Edit | Replace
      subject: "{subject.primary}"
      pose: "{subject.action}"
      setting: "{environment.setting}"
      style: "{style.modifiers[]}"
      technical:
        lighting: "{lighting.type}"
        camera: "DSLR 50mm"
        resolution: "8K"
        mood: "{mood.primary}"
        depth_of_field: "shallow"

    # Stable Diffusion: Positive + Negative + Weights
    sd_formula:
      positive_template: "{subject}, {environment}, {style.modifiers[]}, {weighted_modifiers as (term:weight)}"
      negative_template: "{negative_prompts.content[]}, {negative_prompts.technical[]}"

    # Midjourney: Short, high-signal phrases + parameters
    mj_formula:
      prompt_template: "{subject.primary}, {subject.action}, {environment.setting}, {style.modifiers[]}, {lighting.type}, {mood.primary}"
      suffix: "--ar {aspect_ratio} --s {stylize} --chaos {chaos}"
```

#### Examples by Geographic Level

```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# CONTINENT: Asia (AS) — Broad strokes
# ═══════════════════════════════════════════════════════════════════════════════
continent_asia:
  visual_prompt:
    image:
      style:
        modifiers: ["Asian aesthetic", "harmonious balance"]
        weighted_modifiers:
          - { term: "harmony", weight: 1.2 }
      color_palette:
        primary: ["#C41E3A", "#FFD700"]                 # Red, Gold
        avoid: ["aggressive contrasts", "neon"]
      composition:
        balance: "balanced"
        negative_space: "moderate"
      lighting:
        color_temp: "warm"
      quality:
        dalle_style: "natural"

# ═══════════════════════════════════════════════════════════════════════════════
# REGION: Eastern Asia (030) — More specific
# ═══════════════════════════════════════════════════════════════════════════════
region_eastern_asia:
  visual_prompt:
    image:
      style:
        modifiers: ["East Asian art influence", "calligraphic elements"]
        weighted_modifiers:
          - { term: "ink painting influence", weight: 1.3 }
          - { term: "brush stroke texture", weight: 1.1 }
      subject:
        nature_elements: ["bamboo", "plum blossom", "pine"]
        symbols: ["dragon", "phoenix", "clouds"]
      negative_prompts:
        cultural: ["western corporate", "generic stock"]
    illustration:
      techniques: ["ink wash", "brush painting"]
      weighted_techniques:
        - { term: "sumi-e", weight: 1.4 }

# ═══════════════════════════════════════════════════════════════════════════════
# SUBREGION: Japan (GeoSubRegion) — Very specific
# ═══════════════════════════════════════════════════════════════════════════════
subregion_japan:
  visual_prompt:
    image:
      subject:
        nature_elements: ["cherry blossom", "bamboo", "koi", "crane", "wave"]
        architectural_style: "Japanese traditional"
        human_representation: "stylized"
        symbols: ["torii", "ensō", "mon (family crest)"]
        avoid: ["Chinese dragon style", "Korean hanbok"]

      style:
        modifiers:
          - "Japanese aesthetic"
          - "wabi-sabi imperfection"
          - "ma (negative space) 間"
          - "subtle asymmetry"
        weighted_modifiers:
          - { term: "wabi-sabi", weight: 1.4 }
          - { term: "zen minimalism", weight: 1.3 }
          - { term: "ukiyo-e influence", weight: 1.2 }

      negative_prompts:
        content: ["Chinese dragon style", "Korean hanbok"]
        style: ["bright bold colors", "cluttered composition"]
        cultural: ["stereotypical geisha", "ninja cliché"]

      color_palette:
        primary: ["#C41E3A", "#FFD700"]                 # Vermillion, Gold
        secondary: ["#F5F5DC", "#4A5568"]              # Natural, Slate
        seasonal:
          spring: ["#FFB7C5", "#98FB98"]               # Sakura, Fresh green
          summer: ["#4169E1", "#228B22"]               # Hydrangea, Deep green
          autumn: ["#FF6B35", "#8B0000"]               # Momiji, Deep red
          winter: ["#FFFFFF", "#1C1C1C"]               # Snow, Black

      composition:
        balance: "asymmetric"
        negative_space: "abundant"
        focal_point: "off-center"
        rule: "rule_of_thirds"

      lighting:
        type: "soft diffused"
        time_of_day: "golden hour"
        mood: "contemplative"

      mood:
        primary: "serene"
        energy: "low"

      parameters:
        stylize: 150                                    # Higher stylization for artistic feel
        chaos: 0                                        # Low randomness

    video:
      scene:
        setting: "traditional Japanese environment"
        action: "slowly drifting"
      cinematography:
        camera_movement: "slow dolly"
        shot_type: "wide establishing"
      motion:
        pace: "contemplative"
        rhythm: "flowing"
      audio_style:
        instruments: ["koto", "shakuhachi", "shamisen"]
        tempo: "slow"

    illustration:
      style_base: "ukiyo-e woodblock"
      techniques: ["sumi-e", "gold leaf", "washi texture"]
      weighted_techniques:
        - { term: "woodblock print", weight: 1.5 }
        - { term: "gold leaf accent", weight: 1.2 }

    platform_hints:
      midjourney:
        stylize: 200
      stable_diffusion:
        cfg_scale: 8

# ═══════════════════════════════════════════════════════════════════════════════
# COUNTRY: Japan (JP) — Modern/Contemporary layer
# ═══════════════════════════════════════════════════════════════════════════════
country_japan:
  visual_prompt:
    image:
      style:
        modifiers:
          - "contemporary Japanese design"
          - "kawaii elements when appropriate"
          - "minimalist tech aesthetic"
        weighted_modifiers:
          - { term: "Japanese minimalism", weight: 1.3 }
          - { term: "contemporary Tokyo style", weight: 1.1 }
      subject:
        modern_elements: ["neon signs", "convenience store", "train station"]
        pop_culture: ["anime influence", "manga style available"]
        architectural_style: "modern Japanese"
      quality:
        dalle_style: "vivid"                            # More vibrant for modern contexts

# ═══════════════════════════════════════════════════════════════════════════════
# LOCALE: ja-JP — Locale-specific refinements
# ═══════════════════════════════════════════════════════════════════════════════
locale_ja_jp:
  visual_prompt:
    # Inherits from Country Japan
    # Locale-specific overrides for Japanese language contexts
    image:
      subject:
        text_elements:
          script: "japanese"                            # Use Japanese typography
          style: "vertical when appropriate"
    platform_hints:
      midjourney:
        niji: true                                      # Enable Niji for anime-style when needed
```

#### Prompt Compilation Process

When generating AI assets, the system compiles the `visual_prompt` hierarchy into platform-specific prompts:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PROMPT COMPILATION PIPELINE                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. COLLECT VISUAL PROMPTS                                                      │
│     Brand.PromptStyle + Geographic hierarchy (Continent → Locale)              │
│                                                                                 │
│  2. MERGE (deep merge, specific wins)                                          │
│     Arrays: APPEND  │  Objects: DEEP MERGE  │  Scalars: OVERRIDE               │
│                                                                                 │
│  3. COMPILE TO PLATFORM FORMAT                                                  │
│                                                                                 │
│     ┌─────────────────────────────────────────────────────────────────────┐    │
│     │  MIDJOURNEY                                                         │    │
│     │  {subject.primary}, {subject.action}, {environment.setting},       │    │
│     │  {style.modifiers[]}, {lighting.type}, {mood.primary}              │    │
│     │  --ar {parameters.aspect_ratio} --s {parameters.stylize}           │    │
│     │  --chaos {parameters.chaos}                                        │    │
│     │                                                                     │    │
│     │  Example output:                                                    │    │
│     │  "abstract QR code visualization, emerging from light,             │    │
│     │   minimal zen space, traditional Japanese aesthetic, ukiyo-e       │    │
│     │   influence, wabi-sabi imperfection, soft diffused lighting,       │    │
│     │   serene mood --ar 16:9 --s 150 --chaos 0"                         │    │
│     └─────────────────────────────────────────────────────────────────────┘    │
│                                                                                 │
│     ┌─────────────────────────────────────────────────────────────────────┐    │
│     │  STABLE DIFFUSION                                                   │    │
│     │  Positive: {subject}, {environment}, {style.modifiers[]},          │    │
│     │            {weighted_modifiers as (term:weight)}                   │    │
│     │  Negative: {negative_prompts.content[]}, {negative_prompts.style[]}│    │
│     │  Parameters: cfg_scale, steps, sampler                             │    │
│     │                                                                     │    │
│     │  Example output:                                                    │    │
│     │  Positive: "abstract QR code, minimal zen space,                   │    │
│     │            (wabi-sabi:1.4), (zen minimalism:1.3), soft lighting"   │    │
│     │  Negative: "western corporate, stock photo, blurry, watermark"     │    │
│     │  CFG: 8, Steps: 30                                                 │    │
│     └─────────────────────────────────────────────────────────────────────┘    │
│                                                                                 │
│     ┌─────────────────────────────────────────────────────────────────────┐    │
│     │  DALL-E 3                                                          │    │
│     │  prompt: "{subject}, {environment}, {style}, {mood}"               │    │
│     │  style: "{quality.dalle_style}"                                    │    │
│     │  quality: "{quality.dalle_quality}"                                │    │
│     │  size: "{platform_hints.dalle.size}"                               │    │
│     │                                                                     │    │
│     │  Example API call:                                                  │    │
│     │  {                                                                  │    │
│     │    "prompt": "Abstract QR code visualization in a minimal zen     │    │
│     │              space, traditional Japanese aesthetic with wabi-sabi │    │
│     │              imperfection, soft diffused lighting, serene mood",  │    │
│     │    "style": "natural",                                            │    │
│     │    "quality": "hd",                                               │    │
│     │    "size": "1792x1024"                                            │    │
│     │  }                                                                  │    │
│     └─────────────────────────────────────────────────────────────────────┘    │
│                                                                                 │
│     ┌─────────────────────────────────────────────────────────────────────┐    │
│     │  SORA (Video)                                                       │    │
│     │  "{scene.description}, {cinematography.camera_movement},           │    │
│     │   {cinematography.shot_type}, {motion.pace} pace,                  │    │
│     │   {visual_style.aesthetic} aesthetic"                              │    │
│     │                                                                     │    │
│     │  Example output:                                                    │    │
│     │  "Gentle cherry blossoms falling in a zen garden, slow dolly      │    │
│     │   shot, wide establishing view, contemplative pace, cinematic     │    │
│     │   aesthetic with warm muted color grading, subtle film grain"     │    │
│     └─────────────────────────────────────────────────────────────────────┘    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### @ Reference for Visual Prompts

```markdown
# In BlockInstruction — using visual prompts

## For hero image generation:
Generate image using @prompt:hero-illustration
Apply regional style from @geo:JP.visual_prompt

## Specific property access:
Use colors from @geo:030.visual_prompt.image.color_palette.primary
Apply @geo:AS.visual_prompt.image.composition

## Merged generation (recommended):
Generate hero with:
- Base: @prompt:hero-illustration
- Cultural: @geo:JP.visual_prompt
- Seasonal: @geo:JP.visual_prompt.image.color_palette.seasonal.spring
```

#### Merge Algorithm at Generation

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  VISUAL PROMPT MERGE ALGORITHM                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Input: target_locale = "ja-JP"                                                 │
│                                                                                 │
│  1. COLLECT HIERARCHY                                                           │
│     Locale(ja-JP) → Country(JP) → SubRegion(Japan) → Region(030) → Continent(AS)│
│                                                                                 │
│  2. START WITH BRAND DEFAULTS                                                   │
│     merged = Brand.PromptStyle("hero-illustration")                            │
│                                                                                 │
│  3. LAYER GEOGRAPHIC (broad to specific)                                        │
│     merged = merge(merged, Continent(AS).visual_prompt)                        │
│     merged = merge(merged, Region(030).visual_prompt)                          │
│     merged = merge(merged, SubRegion(Japan).visual_prompt)                     │
│     merged = merge(merged, Country(JP).visual_prompt)                          │
│     merged = merge(merged, Locale(ja-JP).visual_prompt)  // if exists          │
│                                                                                 │
│  4. MERGE STRATEGY                                                              │
│     - Arrays: APPEND (style_modifiers, negative_prompts, nature_elements)      │
│     - Objects: DEEP MERGE (color_palette, composition)                         │
│     - Scalars: OVERRIDE (balance, pace, focal_point)                           │
│     - Nulls: SKIP                                                               │
│                                                                                 │
│  5. OUTPUT                                                                      │
│     Final AI prompt with all merged visual parameters                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Example: East Asia Cultural Style

```yaml
# GeoSubRegion: Eastern Asia (EA)
cultural_style:
  color_preferences:
    primary_colors: ["red", "gold", "black"]
    avoid_colors: ["white alone", "4 items"]  # 白 = death, 四 = death
    color_meanings:
      red: "luck, prosperity, celebration"
      gold: "wealth, imperial"
      white: "mourning, death"

  visual_style:
    aesthetic: "balanced"
    patterns: ["waves", "clouds", "bamboo", "cherry_blossom"]
    imagery: ["nature harmony", "seasons", "calligraphy"]

  typography:
    style: "mixed"
    formality: "contextual"

  cultural_codes:
    formality_level: "high"
    humor_style: "subtle_wordplay"
    directness: "indirect"
    group_harmony: "prioritized"
```

### @ Reference for Cultural Styles

```markdown
# In BlockInstruction
Generate hero image using @prompt:hero-illustration
Apply cultural style for @geo:EA (Eastern Asia)

# Resolution at generation time:
# 1. Load Brand.PromptStyle("hero-illustration")
# 2. Load GeoSubRegion("EA").cultural_style
# 3. Merge: cultural_style overrides PromptStyle where specified
# 4. Generate image with merged parameters
```

---

## Semantic Links: Brand → Existing Nodes

Plutôt que créer de nouveaux nodes, on utilise des liens sémantiques vers les nodes existants.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SEMANTIC CONNECTIONS                                                           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Brand ──[:TARGETS_PERSONA]──▶ AudiencePersona (org/semantic)                  │
│         │ Uses existing AudiencePersona node                                   │
│         │ No BrandAudience needed                                              │
│         │                                                                       │
│         └──[:FOR_MARKET]──▶ Market (shared/locale)                             │
│            Target markets for the brand                                        │
│                                                                                 │
│  PromptStyle ──[:INSPIRED_BY_REGION]──▶ GeoRegion (shared/geography)           │
│              │ Cultural inspiration source                                     │
│              │                                                                  │
│              └──[:FOR_LOCALE]──▶ Locale (shared/config)                        │
│                 Locale-specific prompt variations                              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Complete Node & Arc Architecture

### All Nodes (New + Modified)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  COMPLETE NODE ARCHITECTURE (ADR-028 v0.12.3)                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─── NEW NODES (4) ───────────────────────────────────────────────────────┐   │
│  │                                                                          │   │
│  │  Brand (org/foundation, defined)                                         │   │
│  │  ├── soul: { purpose, mission, vision }                                 │   │
│  │  ├── pitch: { what, for_whom, how, elevator_pitch, differentiators }   │   │
│  │  └── voice: { voice, tone, humor, formality, values }                   │   │
│  │                                                                          │   │
│  │  BrandDesign (org/foundation, defined)                                   │   │
│  │  ├── philosophy, influences, style_keywords, style_mood                 │   │
│  │  ├── tokens: { primitives, semantic, component }                        │   │
│  │  ├── typography: { font_primary, font_secondary, scale }                │   │
│  │  └── ui: { border_radius, shadow_style, animation_style }               │   │
│  │                                                                          │   │
│  │  BrandPrinciples (org/foundation, defined)                               │   │
│  │  ├── heuristics[]: { trigger, rule, rationale }                         │   │
│  │  └── do[], dont[]                                                        │   │
│  │                                                                          │   │
│  │  PromptStyle (org/foundation, defined)                                   │   │
│  │  ├── key, name, media_type (image|video|animation)                      │   │
│  │  ├── style: { base, modifiers[], negative[] }                           │   │
│  │  ├── subject: { primary, secondary[], avoid[] }                         │   │
│  │  ├── environment: { type, elements[], depth }                           │   │
│  │  ├── lighting: { type, direction, color_temp }                          │   │
│  │  ├── color_palette: { mode, primary, accent, mood }                     │   │
│  │  ├── composition: { aspect_ratio, focal_point, negative_space }         │   │
│  │  ├── mood: { primary, secondary, energy }                               │   │
│  │  └── quality: { resolution, detail, noise }                             │   │
│  │                                                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌─── MODIFIED NODES (5) ──────────────────────────────────────────────────┐   │
│  │                                                                          │   │
│  │  Continent (shared/geography) — ADD visual_prompt, cultural_style       │   │
│  │  GeoRegion (shared/geography) — ADD visual_prompt, cultural_style       │   │
│  │  GeoSubRegion (shared/geography) — ADD visual_prompt, cultural_style    │   │
│  │  Country (shared/geography) — ADD visual_prompt, cultural_style         │   │
│  │  BrandIdentity → RENAME to Brand, restructure properties                │   │
│  │                                                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌─── EXISTING NODES (referenced) ─────────────────────────────────────────┐   │
│  │                                                                          │   │
│  │  Project (org/foundation)      — [:HAS_BRAND] → Brand                   │   │
│  │  AudiencePersona (org/semantic) — Brand [:TARGETS_PERSONA] →            │   │
│  │  Locale (shared/config)         — PromptStyle [:FOR_LOCALE] →           │   │
│  │  Page (org/structure)           — [:REPRESENTS] → Entity                │   │
│  │  Entity (org/semantic)          — [:HAS_KEYWORD] → SEOKeyword           │   │
│  │  Block (org/structure)          — [:REFERENCES] → Entity                │   │
│  │  BlockInstruction (org/instruction) — [:MENTIONS] → *                   │   │
│  │                                                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### All Arcs (New)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NEW ARCS (ADR-028 v0.12.3)                                                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─── OWNERSHIP FAMILY ────────────────────────────────────────────────────┐   │
│  │                                                                          │   │
│  │  HAS_BRAND         Project → Brand         1:1    intra_realm           │   │
│  │  HAS_DESIGN        Brand → BrandDesign     1:1    intra_realm           │   │
│  │  HAS_PRINCIPLES    Brand → BrandPrinciples 1:1    intra_realm           │   │
│  │  HAS_PROMPT_STYLE  Brand → PromptStyle     1:N    intra_realm           │   │
│  │  HAS_PAGE          Project → Page          1:N    intra_realm           │   │
│  │  HAS_ENTITY        Project → Entity        1:N    intra_realm           │   │
│  │  HAS_KEYWORD       Entity → SEOKeyword     1:N    cross_realm           │   │
│  │                                                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌─── SEMANTIC FAMILY ─────────────────────────────────────────────────────┐   │
│  │                                                                          │   │
│  │  REPRESENTS        Page → Entity           1:1    intra_realm           │   │
│  │  LINKS_TO          Page → Page             N:M    intra_realm           │   │
│  │                    { via_blocks[], strength }                            │   │
│  │  REFERENCES        Block → Entity          N:M    intra_realm           │   │
│  │                    { purpose: inject|link, count }                       │   │
│  │  MENTIONS          BlockInstruction → *    N:M    intra_realm           │   │
│  │                    { position, ref_type, purpose }                       │   │
│  │  TARGETS_PERSONA   Brand → AudiencePersona N:M    intra_realm           │   │
│  │                    { priority }                                          │   │
│  │  FOR_MARKET        Brand → Market          N:M    cross_realm           │   │
│  │  INSPIRED_BY_REGION PromptStyle → GeoRegion N:M   cross_realm           │   │
│  │                                                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  ┌─── LOCALIZATION FAMILY ─────────────────────────────────────────────────┐   │
│  │                                                                          │   │
│  │  FOR_LOCALE        PromptStyle → Locale    N:M    cross_realm           │   │
│  │                                                                          │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Entity-Relationship Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ENTITY-RELATIONSHIP DIAGRAM                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌──────────┐                                                                   │
│  │ Project  │──[:HAS_BRAND]──────────────────────▶┌─────────┐                  │
│  │(1 per org)│                                    │  Brand  │                  │
│  └────┬─────┘                                     └────┬────┘                  │
│       │                                                │                        │
│       │[:HAS_PAGE]                      ┌──────────────┼──────────────┐        │
│       │[:HAS_ENTITY]                    │              │              │        │
│       ▼                            [:HAS_DESIGN] [:HAS_PRINCIPLES] [:HAS_PROMPT_STYLE]
│  ┌─────────┐                            │              │              │        │
│  │  Page   │◀──[:REPRESENTS]──┐         ▼              ▼              ▼        │
│  └────┬────┘                  │    ┌─────────┐   ┌───────────┐  ┌───────────┐  │
│       │                       │    │ Design  │   │Principles │  │PromptStyle│  │
│       │[:HAS_BLOCK {order}]   │    └─────────┘   └───────────┘  └─────┬─────┘  │
│       │[:LINKS_TO]            │                                       │        │
│       ▼                       │                                       │        │
│  ┌─────────┐                  │    [:INSPIRED_BY_REGION]──────────────┘        │
│  │  Block  │──[:REFERENCES]───┼────────────────────────────────────────▶       │
│  └────┬────┘                  │                                    ┌─────────┐ │
│       │                       │                                    │GeoRegion│ │
│       │[:HAS_INSTRUCTION]     │                                    └─────────┘ │
│       ▼                       │                                         │      │
│  ┌───────────────┐            │         visual_prompt                   │      │
│  │BlockInstruction│           │         cultural_style                  │      │
│  └───────┬───────┘            │                ▲                        │      │
│          │                    │                │                        │      │
│          │[:MENTIONS]         │         ┌──────┴──────┐                │      │
│          ▼                    │         │  Continent  │────────────────┘      │
│     ┌─────────┐               │         │  GeoRegion  │      [:IN_CONTINENT]   │
│     │ Entity  │◀──────────────┘         │GeoSubRegion │      [:IN_REGION]      │
│     └────┬────┘                         │   Country   │      [:IN_SUBREGION]   │
│          │                              └─────────────┘                        │
│          │[:HAS_CONTENT]                                                       │
│          │[:HAS_KEYWORD]                                                       │
│          │[:BELONGS_TO]                                                        │
│          ▼                                                                     │
│     ┌─────────────┐  ┌──────────┐  ┌───────────────┐                          │
│     │EntityContent│  │SEOKeyword│  │EntityCategory │                          │
│     └─────────────┘  └──────────┘  └───────────────┘                          │
│                                                                                 │
│  [:TARGETS_PERSONA]                                                            │
│  Brand ────────────────────────────────────────────▶ AudiencePersona           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Property Summary Table

| Node | New Properties | Type | Description |
|------|----------------|------|-------------|
| **Continent** | `visual_prompt` | JSON | AI generation presets (image, video, illustration, product_3d) |
| **Continent** | `cultural_style` | JSON | Color preferences, visual style, typography, cultural codes |
| **GeoRegion** | `visual_prompt` | JSON | Regional AI generation presets |
| **GeoRegion** | `cultural_style` | JSON | Regional cultural preferences |
| **GeoSubRegion** | `visual_prompt` | JSON | Sub-regional AI generation presets |
| **GeoSubRegion** | `cultural_style` | JSON | Sub-regional cultural preferences |
| **Country** | `visual_prompt` | JSON | Country-specific AI generation presets |
| **Country** | `cultural_style` | JSON | Country cultural preferences |

### Arc Summary Table

| Arc | Source | Target | Cardinality | Family | Scope | Properties |
|-----|--------|--------|-------------|--------|-------|------------|
| HAS_BRAND | Project | Brand | 1:1 | ownership | intra | — |
| HAS_DESIGN | Brand | BrandDesign | 1:1 | ownership | intra | — |
| HAS_PRINCIPLES | Brand | BrandPrinciples | 1:1 | ownership | intra | — |
| HAS_PROMPT_STYLE | Brand | PromptStyle | 1:N | ownership | intra | — |
| TARGETS_PERSONA | Brand | AudiencePersona | N:M | semantic | intra | priority |
| FOR_MARKET | Brand | Market | N:M | semantic | cross | — |
| INSPIRED_BY_REGION | PromptStyle | GeoRegion | N:M | semantic | cross | — |
| FOR_LOCALE | PromptStyle | Locale | N:M | localization | cross | — |
| REPRESENTS | Page | Entity | 1:1 | semantic | intra | — |
| LINKS_TO | Page | Page | N:M | semantic | intra | via_blocks[], strength |
| REFERENCES | Block | Entity | N:M | semantic | intra | purpose, count |
| MENTIONS | BlockInstruction | * | N:M | semantic | intra | position, ref_type, purpose |
| HAS_PAGE | Project | Page | 1:N | ownership | intra | — |
| HAS_ENTITY | Project | Entity | 1:N | ownership | intra | — |
| HAS_KEYWORD | Entity | SEOKeyword | 1:N | ownership | cross | rank |

---

### BlockType (~30 templates)

Définit le schema JSON + contraintes techniques. Réutilisable sur plusieurs Pages.

```yaml
block_type:
  name: hero
  description: "Full-width hero section with title, subtitle, CTA"

  # Peut override Brand
  tone_override: null     # inherit from Brand

  schema:
    title:
      type: string
      behavior: translate
      length: { min: 30, max: 70 }
      seo: primary_required

    subtitle:
      type: string
      behavior: translate
      length: { max: 150 }

    cta_text:
      type: string
      behavior: translate
      tone_override: urgent   # Override pour CTA
      length: { max: 25 }

    cta_url:
      type: string
      behavior: fixed         # Jamais traduit

    background_image:
      type: string
      behavior: fixed
```

### BlockInstruction (spécifique par Block)

Instructions créatives en markdown + @ refs. **PAS de directives techniques** (elles sont dans BlockType).

```markdown
# Hero Block (homepage)

title:
  Focus on @entity:qr-generator speed advantage.
  Differentiate from @competitor:qr-monkey.

subtitle:
  Expand value prop for @audience:marketers.
  Mention @entity:api integration.

cta_text:
  Drive signup action. Link to [@page:signup].

cta_url: /signup
background_image: /images/hero-home.webp
```

---

## Field Behaviors

Définit COMMENT un champ est traité lors de la génération.

| Behavior | Description | Exemple |
|----------|-------------|---------|
| `translate` | LLM génère nativement pour la locale | title, description |
| `fixed` | Copié tel quel (pas de traitement) | urls, image paths, ids |
| `derive` | LLM dérive/paraphrase depuis source | meta_description from title |
| `copy` | Copie depuis un autre Block | shared footer |
| `computed` | Calculé (pas LLM) | reading_time from body.length |
| `conditional` | Behavior dépend du contexte | legal_text: fixed if US |

### Exemples de Configuration

```yaml
# translate (défaut pour contenu)
title:
  behavior: translate
  length: { max: 70 }

# fixed (valeurs techniques)
cta_url:
  behavior: fixed

# derive (dérivé d'un autre champ)
meta_description:
  behavior: derive
  derive_from: title
  length: { max: 160 }

# copy (réutilisation)
footer:
  behavior: copy
  copy_from: block:shared-footer.content

# computed (calculé)
reading_time:
  behavior: computed
  compute: "Math.ceil(body.split(' ').length / 200)"

# conditional (contextuel)
legal_disclaimer:
  behavior: conditional
  conditions:
    - if: "locale.region == 'US'"
      then: fixed
    - else: translate
```

---

## @ Reference System

### Principe Fondamental

```
@type:key              → INJECTION (contexte LLM, pas de lien HTML)
[@type:key]            → LIEN (crée un <a href>)
[@type:key|anchor]     → LIEN + ANCHOR personnalisé
```

- Sans crochets `@` = injection de contexte
- Avec crochets `[@]` = création de lien

### Catalogue Complet

#### Injection Sémantique (contexte LLM)

| Syntaxe | Effet | Exemple |
|---------|-------|---------|
| `@entity:X` | Inject EntityContent(X@locale) | `@entity:tier-pro` |
| `@entity:X.field` | Inject champ spécifique | `@entity:tier-pro.tagline` |
| `@project` | Inject ProjectContent | Contexte projet global |
| `@brand` | Inject Brand (soul, pitch, voice) | `@brand.elevator_pitch` |
| `@brand.design` | Inject BrandDesign | `@brand.design.style_mood` |
| `@brand.principles` | Inject BrandPrinciples | `@brand.principles.heuristics` |
| `@audience:X` | Inject AudiencePersona | `@audience:developers` |
| `@block:X` | Inject BlockGenerated/Instruction | `@block:shared-footer` |

#### Injection Design (génération d'images/vidéos)

| Syntaxe | Effet | Exemple |
|---------|-------|---------|
| `@prompt:X` | Inject PromptStyle preset | `@prompt:hero-illustration` |
| `@design.tokens.X` | Inject design token | `@design.tokens.semantic.colors.primary` |
| `@design.moods.X` | Inject mood palette | `@design.moods.trust` |
| `@geo:X` | Inject cultural_style from geography | `@geo:EA` (Eastern Asia) |
| `@geo:X.cultural_style` | Inject specific cultural property | `@geo:EU.cultural_style.color_preferences` |
| `@geo:X.visual_prompt` | Inject AI visual prompt preset | `@geo:JP.visual_prompt` |
| `@geo:X.visual_prompt.image` | Inject image generation preset | `@geo:JP.visual_prompt.image` |
| `@geo:X.visual_prompt.video` | Inject video generation preset | `@geo:JP.visual_prompt.video` |
| `@geo:X.visual_prompt.illustration` | Inject illustration preset | `@geo:JP.visual_prompt.illustration` |
| `@geo:X.visual_prompt.product_3d` | Inject 3D product preset | `@geo:JP.visual_prompt.product_3d` |

#### Injection Locale (vocabulaire)

| Syntaxe | Effet | Exemple |
|---------|-------|---------|
| `@term:X` | Inject Term(X@locale) | `@term:subscription` |
| `@expr:X` | Inject Expression(X@locale) | `@expr:call-to-action` |
| `@pattern:X` | Inject Pattern(X@locale) | `@pattern:value-prop` |
| `@culture:X` | Inject CultureRef(X@locale) | `@culture:formality` |
| `@taboo:X` | Inject Taboo(X@locale) | `@taboo:anglicisms` |

#### Injection SEO

| Syntaxe | Effet | Exemple |
|---------|-------|---------|
| `@seo:X` | Inject SEOKeyword | `@seo:qr-generator` |
| `@competitor:X` | Inject concurrent (différenciation) | `@competitor:qr-monkey` |

#### Liens (crée `<a href>`)

| Syntaxe | Résultat HTML | Notes |
|---------|---------------|-------|
| `[@page:X]` | `<a href="/X">{page.title}</a>` | Anchor = titre page |
| `[@page:X\|@entity:Y]` | `<a href="/X">{entity.name}</a>` | Anchor = nom entity |
| `[@page:X\|@term:Y]` | `<a href="/X">{term.value}</a>` | Anchor = terme localisé |
| `[@page:X\|"text"]` | `<a href="/X">text</a>` | Anchor = littéral |
| `[@page:X#section]` | `<a href="/X#section">...</a>` | Deep link |
| `[@external:X]` | `<a href="{url}">...</a>` | URL centralisée |

---

## Modèle de Liens (3 Niveaux)

### Niveau 1: MENTIONS (granulaire)

```
BlockInstruction ──[:MENTIONS]──▶ Entity|Page|Term|...
                   { position: N,
                     ref_type: "entity|page|term|...",
                     purpose: "inject|link" }
```

Parsing des @ refs avec position exacte dans le texte.

### Niveau 2: REFERENCES (par block)

```
Block ──[:REFERENCES]──▶ Entity
        { purpose: "inject|link",
          count: N }
```

Agrégé depuis les [:MENTIONS] de son BlockInstruction.

### Niveau 3: LINKS_TO (par page)

```
Page ──[:LINKS_TO]──▶ Page
       { via_blocks: ["hero", "pricing"],
         strength: N }
```

Agrégé depuis les [:REFERENCES purpose:"link"] de ses Blocks.

---

## Architecture Finale

### Nodes Stockés

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  NODES STOCKÉS                                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Entity (semantic, defined)                                                     │
│  │ key: "qr-generator"          ← SOURCE OF TRUTH for slug                     │
│  │                                                                              │
│  ├──[:BELONGS_TO]────────▶ EntityCategory (product|feature|legal|...)          │
│  ├──[:HAS_KEYWORD]───────▶ SEOKeyword* (primary + secondary)                   │
│  ├──[:RELATES_TO]────────▶ Entity* (relations sémantiques)                     │
│  └──[:HAS_CONTENT]───────▶ EntityContent@locale                                │
│                                                                                 │
│  Page (structure, defined)                                                      │
│  │ key: derived from Entity.key                                                │
│  │                                                                              │
│  ├──[:REPRESENTS]──────────────▶ Entity (1:1 OBLIGATOIRE)                      │
│  ├──[:HAS_BLOCK {order: N}]────▶ Block* ← ORDER IS HERE (source of truth)      │
│  ├──[:LINKS_TO]────────────────▶ Page* (SEO maillage, from @ refs)             │
│  └──[:HAS_GENERATED]───────────▶ PageGenerated@locale                          │
│                                                                                 │
│  BlockType (instruction, defined) — ~30 templates                              │
│  │ Schema JSON + behaviors + constraints                                       │
│  │                                                                              │
│  └── Réutilisé par N Blocks                                                    │
│                                                                                 │
│  Block (structure, defined)                                                     │
│  │                                                                              │
│  ├──[:OF_TYPE]─────────────────▶ BlockType                                     │
│  ├──[:HAS_INSTRUCTION]─────────▶ BlockInstruction                              │
│  ├──[:REFERENCES]──────────────▶ Entity* { purpose: inject|link }              │
│  └──[:HAS_GENERATED]───────────▶ BlockGenerated@locale                         │
│                                                                                 │
│  BlockInstruction (instruction, defined)                                        │
│  │ Markdown + @ refs                                                           │
│  │                                                                              │
│  └──[:MENTIONS]────────────────▶ Entity|Page|Term|... { position, purpose }    │
│                                                                                 │
│  Project (foundation, defined)                                                 │
│  │                                                                              │
│  ├──[:HAS_BRAND]───────────────▶ Brand (1:1)                                   │
│  ├──[:HAS_PAGE]────────────────▶ Page*                                         │
│  └──[:HAS_ENTITY]──────────────▶ Entity*                                       │
│                                                                                 │
│  Brand (foundation, defined) — 1 par Project                                   │
│  │ soul (purpose, mission, vision)                                             │
│  │ pitch (what, for_whom, how, elevator_pitch)                                 │
│  │ voice (voice, tone, humor, formality, values)                               │
│  │                                                                              │
│  ├──[:HAS_DESIGN]──────────▶ BrandDesign                                       │
│  │   │ design_philosophy, style_keywords, style_mood                           │
│  │   │ tokens (primitives, semantic, component)                                │
│  │   └── typography, ui patterns                                               │
│  │                                                                              │
│  ├──[:HAS_PRINCIPLES]──────▶ BrandPrinciples                                   │
│  │   │ heuristics (trigger, rule, rationale)                                   │
│  │   └── do/dont rules                                                         │
│  │                                                                              │
│  ├──[:HAS_PROMPT_STYLE]────▶ PromptStyle* (N presets)                          │
│  │   │ style, subject, environment, lighting                                   │
│  │   │ color_palette, composition, mood, quality                               │
│  │   │                                                                          │
│  │   ├──[:INSPIRED_BY_REGION]▶ GeoRegion (cultural inspiration)                │
│  │   └──[:FOR_LOCALE]────────▶ Locale (locale-specific)                        │
│  │                                                                              │
│  └──[:TARGETS_PERSONA]─────▶ AudiencePersona* (semantic link)                  │
│                                                                                 │
│  Geographic Nodes (shared/geography)                                            │
│  │ Continent > GeoRegion > GeoSubRegion > Locale                               │
│  │ Each carries: cultural_style (colors, visual, typography, cultural_codes)   │
│  │                                                                              │
│  └── At generation: Brand.PromptStyle + Geo.cultural_style merged              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Concepts Calculés (pas stockés)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CONCEPTS CALCULÉS (au moment de la génération)                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  PageStructure (calculé)                                                        │
│  └── Query: MATCH (p:Page)-[r:HAS_BLOCK]->(b:Block)-[:OF_TYPE]->(bt:BlockType) │
│             RETURN bt ORDER BY r.order                                         │
│  └── Result: [BlockType schemas in order]                                      │
│                                                                                 │
│  PageInstruction (calculé)                                                      │
│  └── Query: MATCH (p:Page)-[r:HAS_BLOCK]->(b:Block)-[:HAS_INSTRUCTION]->(bi)   │
│             RETURN bi.content ORDER BY r.order                                 │
│  └── Result: Concatenated BlockInstructions                                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Exemple Complet

### Configuration

**Brand**:
```yaml
voice: expert
tone: professional
formality: formal
```

**BlockType: hero**:
```yaml
schema:
  title: { behavior: translate, length: { max: 70 }, seo: primary_required }
  subtitle: { behavior: translate, length: { max: 150 } }
  cta_text: { behavior: translate, tone_override: urgent, length: { max: 25 } }
  cta_url: { behavior: fixed }
```

### BlockInstruction (homepage/hero)

```markdown
title:
  Focus on @entity:qr-generator speed advantage.
  Differentiate from @competitor:qr-monkey.

subtitle:
  Expand value prop for @audience:marketers.
  Mention @entity:api integration.

cta_text:
  Drive signup action. Link to [@page:signup].

cta_url: /signup
```

### Parsing @ Refs

| Ref | Type | Purpose |
|-----|------|---------|
| `@entity:qr-generator` | entity | inject |
| `@competitor:qr-monkey` | competitor | inject |
| `@audience:marketers` | audience | inject |
| `@entity:api` | entity | inject |
| `[@page:signup]` | page | link |

### Génération (fr-FR)

**Input LLM**:
- BlockType.schema (contraintes)
- BlockInstruction (créativité)
- Brand (voice: expert, tone: professional)
- Resolved @ refs (EntityContent, etc.)
- Locale context (fr-FR)

**Output BlockGenerated**:
```json
{
  "title": "Générez vos QR codes en un éclair",
  "subtitle": "Notre solution premium pour les marketeurs exigeants...",
  "cta_text": "Essayer gratuitement",
  "cta_url": "/signup"
}
```

### Liens Créés

```
Page(homepage) ──[:LINKS_TO]──▶ Page(signup) { via: ["hero"], strength: 1 }
```

---

## Generation Pipeline

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PIPELINE DE GÉNÉRATION                                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. LOAD PAGE STRUCTURE                                                         │
│     Query: Page -[:HAS_BLOCK {order}]-> Block -[:OF_TYPE]-> BlockType          │
│     Result: Ordered list of Blocks with their schemas                          │
│                                                                                 │
│  2. LOAD INSTRUCTIONS                                                           │
│     Query: Block -[:HAS_INSTRUCTION]-> BlockInstruction                        │
│     Result: Instructions per Block                                             │
│                                                                                 │
│  3. PARSE @ REFS                                                                │
│     Extract: @entity:X, @term:Y, [@page:Z], etc.                               │
│     Classify: inject vs link                                                   │
│                                                                                 │
│  4. RESOLVE @ REFS                                                              │
│     @entity:X  → EntityContent(X@locale)                                       │
│     @term:X    → Term(X) from locale TermSet                                   │
│     @seo:X     → SEOKeyword(X)                                                 │
│     @brand     → BrandIdentity                                                 │
│                                                                                 │
│  5. BUILD LLM CONTEXT                                                           │
│     Combine: BlockType.schema + BlockInstruction + resolved refs + Brand       │
│                                                                                 │
│  6. GENERATE (per Block, per Locale)                                           │
│     Input: LLM context                                                         │
│     Output: BlockGenerated@locale                                              │
│                                                                                 │
│  7. ASSEMBLE PAGE                                                               │
│     Combine: BlockGenerated[] in order → PageGenerated@locale                  │
│                                                                                 │
│  8. CREATE LINKS                                                                │
│     Extract: [@page:X] refs with purpose: link                                 │
│     Create: Page -[:LINKS_TO]-> Page arcs                                      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Migration

### Nodes Supprimés (vs ADR-025)

| Node | Raison |
|------|--------|
| `PageStructure` | Calculé, pas stocké |
| `PageInstruction` | Calculé, pas stocké |

### Arcs Supprimés (vs ADR-025)

| Arc | Raison |
|-----|--------|
| `[:HAS_STRUCTURE]` (Page→) | PageStructure n'est plus un node |
| `[:HAS_INSTRUCTION]` (Page→) | PageInstruction n'est plus un node |

### Nouveaux Arcs

| Arc | Source | Target | Family | Properties |
|-----|--------|--------|--------|------------|
| `REPRESENTS` | Page | Entity | semantic | — |
| `LINKS_TO` | Page | Page | semantic | via_blocks[], strength |
| `REFERENCES` | Block | Entity | semantic | purpose, count |
| `MENTIONS` | BlockInstruction | * | semantic | position, ref_type, purpose |
| `HAS_BLOCK` | Page | Block | ownership | **order** (integer) |
| `HAS_BRAND` | Project | Brand | ownership | — |
| `HAS_KEYWORD` | Entity | SEOKeyword | ownership | rank (primary/secondary) |
| `HAS_PAGE` | Project | Page | ownership | — |
| `HAS_ENTITY` | Project | Entity | ownership | — |
| `HAS_DESIGN` | Brand | BrandDesign | ownership | — |
| `HAS_PRINCIPLES` | Brand | BrandPrinciples | ownership | — |
| `HAS_PROMPT_STYLE` | Brand | PromptStyle | ownership | — |
| `TARGETS_PERSONA` | Brand | AudiencePersona | semantic | priority |
| `FOR_MARKET` | Brand | Market | semantic | — |
| `INSPIRED_BY_REGION` | PromptStyle | GeoRegion | semantic | — |
| `FOR_LOCALE` | PromptStyle | Locale | localization | — |

### Validation Rules

1. Toute Page DOIT avoir exactement un `[:REPRESENTS]` vers Entity
2. Page.key DOIT égaler Entity.key
3. `[:HAS_BLOCK].order` doit être unique par Page (no duplicates)
4. Les `[:LINKS_TO]` sont calculés depuis les @ refs avec `purpose: link`
5. Les @ refs invalides génèrent une erreur de validation

---

## Resolved Questions

### Q1: Page ↔ Entity 1:1 Strict

**Decision**: 1:1 strict. Chaque Page a sa propre Entity, même les sous-pages.

```
/pricing              → Entity "pricing"
/pricing/enterprise   → Entity "pricing-enterprise" (NOT child of pricing)
```

- Pas de hiérarchie Page parent/child
- Le slug est le Entity.key, composite si nécessaire
- URL structure = Entity.key avec `/` remplacé par `-` ou vice versa

### Q2: BlockGenerated = Node par Block×Locale

**Decision**: Un node BlockGenerated par combinaison Block × Locale.

```
Block(hero) ──[:HAS_GENERATED]──▶ BlockGenerated (key: "homepage:hero@fr-FR")
            ──[:HAS_GENERATED]──▶ BlockGenerated (key: "homepage:hero@en-US")
```

- Scale: 200 blocks × 200 locales = 40,000 nodes (Neo4j handles it)
- Queryable: MATCH (bg:BlockGenerated {locale: "fr-FR"})
- Deletable: Régénération = delete old + create new

### Q3: BlockType Versioning = Migration Explicite (V1)

**Decision**: Pas de versioning automatique pour V1.

- Si BlockType.schema change → migration explicite des Blocks
- Régénération nécessaire des BlockGenerated affectés
- Future: peut ajouter BlockType.version + migration tooling

---

## Open Questions (Deferred)

1. **Validation temps réel**: Parser @ refs pendant l'édition? (DX)
2. **Refactoring cascade**: Si Entity.key change, cascade sur Page.key? (consistency)
3. **Preview mode**: Générer preview sans persister BlockGenerated? (UX)
4. **Shared blocks**: Comment gérer footer/header réutilisés? (behavior: copy suffit?)
5. **Locale override**: Sections conditionnelles `[IF locale:X]` dans BlockInstruction?

---

## Summary

| Concept | Stocké? | Source of Truth | Key Format |
|---------|---------|-----------------|------------|
| Entity.key | Yes | Slug unique | `qr-generator` |
| Page.key | Yes | = Entity.key | `qr-generator` |
| Block.key | Yes | Composite | `{page}:{type}:{index}` |
| BlockGenerated.key | Yes | Composite | `{block_key}@{locale}` |
| Block order | Yes | `[:HAS_BLOCK {order}]` | — |
| PageStructure | **No** | Calculated | — |
| PageInstruction | **No** | Calculated | — |
| Constraints | Yes | BlockType.schema | — |
| Creativity | Yes | BlockInstruction | — |
| Links | Yes | `[:LINKS_TO]` | from `[@page:X]` refs |
| Brand | Yes | `[:HAS_BRAND]` | 1 per Project |
| BrandDesign | Yes | `[:HAS_DESIGN]` | 1 per Brand |
| BrandPrinciples | Yes | `[:HAS_PRINCIPLES]` | 1 per Brand |
| PromptStyle | Yes | `[:HAS_PROMPT_STYLE]` | N per Brand |
| Cultural Style | Yes | Geography.cultural_style | Inherited geo hierarchy |

---

---

## Implementation Plan

### Phase 0: Brand Architecture (P0)

#### New Node YAML Files

| File | Node | Layer | Trait |
|------|------|-------|-------|
| `org/foundation/brand.yaml` | Brand | foundation | defined |
| `org/foundation/brand-design.yaml` | BrandDesign | foundation | defined |
| `org/foundation/brand-principles.yaml` | BrandPrinciples | foundation | defined |
| `org/foundation/prompt-style.yaml` | PromptStyle | foundation | defined |

#### Modified Geography YAML Files

Add `cultural_style` and `visual_prompt` properties to:
- `shared/geography/continent.yaml`
- `shared/geography/geo-region.yaml`
- `shared/geography/geo-sub-region.yaml`
- `shared/geography/country.yaml` (if exists, or create)

#### Brand YAML Arc Files

| File | Arc | Source | Target |
|------|-----|--------|--------|
| `ownership/has-design.yaml` | HAS_DESIGN | Brand | BrandDesign |
| `ownership/has-principles.yaml` | HAS_PRINCIPLES | Brand | BrandPrinciples |
| `ownership/has-prompt-style.yaml` | HAS_PROMPT_STYLE | Brand | PromptStyle |
| `semantic/targets-persona.yaml` | TARGETS_PERSONA | Brand | AudiencePersona |
| `semantic/for-market.yaml` | FOR_MARKET | Brand | Market |
| `semantic/inspired-by-region.yaml` | INSPIRED_BY_REGION | PromptStyle | GeoRegion |
| `localization/for-locale.yaml` | FOR_LOCALE | PromptStyle | Locale |

#### Migration: BrandIdentity → Brand

```cypher
// 1. Rename BrandIdentity to Brand
MATCH (bi:BrandIdentity)
SET bi:Brand
REMOVE bi:BrandIdentity;

// 2. Update HAS_BRAND_IDENTITY to HAS_BRAND
MATCH (p:Project)-[r:HAS_BRAND_IDENTITY]->(b:Brand)
CREATE (p)-[:HAS_BRAND]->(b)
DELETE r;
```

### Phase 1: YAML Arc Definitions (P0)

Create new arc YAML files in `packages/core/models/arc-kinds/`:

#### Semantic Family (`semantic/`)

| File | Arc | Source | Target | Priority |
|------|-----|--------|--------|----------|
| `represents.yaml` | REPRESENTS | Page | Entity | P0 |
| `links-to.yaml` | LINKS_TO | Page | Page | P0 |
| `references.yaml` | REFERENCES | Block | Entity | P1 |
| `mentions.yaml` | MENTIONS | BlockInstruction | * | P2 |

#### Ownership Family (`ownership/`)

| File | Arc | Source | Target | Priority |
|------|-----|--------|--------|----------|
| `has-brand.yaml` | HAS_BRAND | Project | Brand | P0 |
| `has-keyword.yaml` | HAS_KEYWORD | Entity | SEOKeyword | P1 |
| `has-page.yaml` | HAS_PAGE | Project | Page | P0 |
| `has-entity.yaml` | HAS_ENTITY | Project | Entity | P0 |

### Phase 2: Node YAML Modifications (P0)

#### Modify `page.yaml`

```yaml
# Add REPRESENTS arc reference
arcs:
  outgoing:
    - REPRESENTS  # 1:1 mandatory to Entity
    - HAS_BLOCK   # with {order} property
    - LINKS_TO    # calculated from @ refs
    - HAS_GENERATED
```

#### Modify `entity.yaml`

```yaml
# Add incoming REPRESENTS
arcs:
  incoming:
    - REPRESENTS  # from Page (1:1)
  outgoing:
    - HAS_KEYWORD  # to SEOKeyword
    - HAS_CONTENT  # to EntityContent
```

#### Modify `block.yaml`

```yaml
# Add composite key pattern
properties:
  key:
    type: string
    format: "{page_key}:{block_type}:{index}"
    description: "Composite key ensuring global uniqueness"
```

### Phase 3: Remove PageStructure/PageInstruction Nodes (P1)

#### Files to DELETE

```
packages/core/models/node-kinds/org/instruction/page-structure.yaml
packages/core/models/node-kinds/org/instruction/page-instruction.yaml
```

#### Arcs to DELETE

```
packages/core/models/arc-kinds/ownership/has-structure.yaml  (Page→PageStructure)
packages/core/models/arc-kinds/ownership/has-instruction.yaml  (Page→PageInstruction)
```

> **Note**: BlockInstruction remains (Block→BlockInstruction via HAS_INSTRUCTION)

### Phase 4: Generator Updates (P1)

#### `tools/novanet/src/generators/`

| Generator | Change |
|-----------|--------|
| `arc_schema.rs` | Add new arcs, remove deleted arcs |
| `node_kind.rs` | Remove PageStructure, PageInstruction |
| `cypher_seed.rs` | Update seed queries |
| `validation.rs` | Add REPRESENTS 1:1 validation |

#### New Validation Rules

```rust
// In validation.rs
fn validate_page_represents(&self) -> Result<()> {
    // Every Page MUST have exactly one [:REPRESENTS] to Entity
    // Page.key MUST equal Entity.key
}

fn validate_block_order(&self) -> Result<()> {
    // [:HAS_BLOCK].order must be unique per Page
}

fn validate_at_refs(&self, instruction: &str) -> Result<Vec<AtRef>> {
    // Parse @ refs, validate targets exist
}
```

### Phase 5: @ Reference Parser (P2)

Create new module `tools/novanet/src/refs/`:

```rust
// at_ref_parser.rs
pub enum AtRefType {
    Entity,
    Page,
    Term,
    Expr,
    Brand,
    Audience,
    Block,
    Seo,
    Competitor,
}

pub enum AtRefPurpose {
    Inject,  // @type:key
    Link,    // [@type:key]
}

pub struct AtRef {
    ref_type: AtRefType,
    key: String,
    field: Option<String>,  // @entity:X.field
    anchor: Option<String>, // [@page:X|anchor]
    purpose: AtRefPurpose,
    position: usize,
}

pub fn parse_at_refs(content: &str) -> Vec<AtRef>;
```

### Phase 6: Neo4j Migration (P0)

#### Migration Script `01-page-entity-migration.cypher`

```cypher
// 1. Create REPRESENTS arcs for existing Page-Entity pairs
MATCH (p:Page), (e:Entity)
WHERE p.key = e.key
MERGE (p)-[:REPRESENTS]->(e);

// 2. Validate all Pages have REPRESENTS
MATCH (p:Page)
WHERE NOT (p)-[:REPRESENTS]->(:Entity)
RETURN p.key AS orphan_page;

// 3. Remove PageStructure nodes (if any exist)
MATCH (ps:PageStructure)
DETACH DELETE ps;

// 4. Remove PageInstruction nodes (if any exist)
MATCH (pi:PageInstruction)
WHERE pi:PageInstruction AND NOT pi:BlockInstruction
DETACH DELETE pi;

// 5. Ensure [:HAS_BLOCK] has order property
MATCH (p:Page)-[r:HAS_BLOCK]->(b:Block)
WHERE r.order IS NULL
SET r.order = 0;
```

### Phase 7: Schema Regeneration (P1)

```bash
# 1. Regenerate all artifacts
cargo run -- schema generate

# 2. Validate YAML coherence
cargo run -- schema validate --strict

# 3. Run Neo4j migration
cargo run -- db migrate

# 4. Seed updated schema
cargo run -- db seed
```

### Verification Checklist

- [ ] All new arc YAML files created
- [ ] PageStructure/PageInstruction YAML deleted
- [ ] HAS_STRUCTURE/HAS_INSTRUCTION arcs deleted
- [ ] `cargo run -- schema validate` passes
- [ ] `cargo test` passes (update test fixtures)
- [ ] Neo4j migration script tested on dev DB
- [ ] REPRESENTS 1:1 validation working
- [ ] @ ref parser basic tests passing

### Timeline

| Phase | Description | Effort |
|-------|-------------|--------|
| P0a | Brand Architecture YAML (Brand, BrandDesign, BrandPrinciples, PromptStyle) | 2-3h |
| P0b | Geographic cultural_style properties | 1-2h |
| P0c | Brand Arc YAML (HAS_DESIGN, HAS_PRINCIPLES, etc.) | 1-2h |
| P1 | Page-Entity Arc YAML + Node mods + Migration | 2-3h |
| P2 | Node deletion + Generators | 2-3h |
| P3 | @ Reference Parser (including @prompt, @geo) | 4-6h |
| Total | | 12-19h |

---

## Incohérences Résolues

### 1. Brand ↔ Project Relationship

**Résolution**: Arc `HAS_BRAND` ajouté (Project → Brand, 1:1)

### 2. Block.key Uniqueness

**Résolution**: Format composite `{page_key}:{block_type}:{index}`

### 3. BlockType Layer

**Décision**: BlockType reste dans `instruction` layer
- BlockType définit le schema JSON (instruction pour le LLM)
- Cohérent avec BlockInstruction dans le même layer

### 4. HAS_KEYWORD Arc

**Résolution**: Arc ajouté (Entity → SEOKeyword) avec propriété `rank`

### 5. REPRESENTS Arc

**Résolution**: Nouvel arc sémantique (Page → Entity, 1:1 obligatoire)

---

## Améliorations Différées (V2)

### 1. Entity Semantic Relations Typing

**Proposition**: `[:RELATES_TO {type: "prerequisite"|"alternative"|"complement"}]`
- Différé car nécessite une taxonomie de types de relations
- Pour V1: `[:RELATES_TO]` sans typage

### 2. SEO Keyword Primary/Secondary

**Proposition**: Propriété `rank` sur `[:HAS_KEYWORD]`
- `rank: "primary"` (1 seul par Entity)
- `rank: "secondary"` (N par Entity)

### 3. @ Ref Validation

**Proposition**: Parser avec modes error/warning
- `@entity:INVALID` → Error (bloque génération)
- `@entity:draft-entity` → Warning (entité non publiée)

### 4. Block Reuse (Shared Blocks)

**Proposition différée**: `behavior: copy` suffisant pour V1
- V2: SharedBlock avec `[:USES_SHARED_BLOCK]`

### 5. Locale Override in BlockInstruction

**Proposition différée**: `[IF locale:fr-FR]` sections conditionnelles
- V2: Syntaxe de sections conditionnelles dans BlockInstruction

---

## References

- ADR-025: Instruction Layer Renaming
- ADR-026: Inverse Arc Policy
- ADR-028: Page-Entity Architecture (this design)
- `packages/core/models/node-kinds/org/semantic/entity.yaml`
- `packages/core/models/node-kinds/org/structure/page.yaml`
- `packages/core/models/node-kinds/org/instruction/block-type.yaml`
