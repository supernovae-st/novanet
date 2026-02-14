# Locale Knowledge v10 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor 14 locale knowledge nodes into 27 granular nodes with domain-tagged retrieval and clear TUI visualization.

**Architecture:** Delete 14 existing node-classes, create 27 new node-classes organized by function (Technical/Style/Semantic), add 10 new arc-classes for knowledge relationships, update Rust parsers to support `knowledge_tier` property, and modify TUI to display tier groupings within the knowledge layer.

**Tech Stack:** YAML (node-classes, arc-classes), Rust (parsers, generators, TUI), Cypher (seeds), Neo4j (meta-graph)

**Design Document:** `docs/plans/2026-02-04-locale-knowledge-v10-design.md`

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | 1-3 | Create 4 Technical tier node-classes (Formatting, Slugification, Adaptation, Style) |
| 2 | 4-9 | Create 6 TermSet nodes (domain-tagged) |
| 3 | 10-12 | Create 3 ExpressionSet nodes (register-tagged) |
| 4 | 13-16 | Create 4 PatternSet nodes (usage-tagged) |
| 5 | 17-20 | Create 4 CultureSet nodes (type-tagged) |
| 6 | 21-23 | Create 3 TabooSet nodes (severity-tagged) |
| 7 | 24-26 | Create 3 AudienceSet nodes (segment-tagged) |
| 8 | 27-36 | Create 10 new arc-classes for knowledge relationships |
| 9 | 37-38 | Delete 14 old node-classes and their arcs |
| 10 | 39-41 | Update Rust parsers for `knowledge_tier` property |
| 11 | 42-44 | Update TUI to show tier groupings |
| 12 | 45-47 | Regenerate all artifacts and validate |

---

## Phase 1: Technical Tier (Fat Nodes)

### Task 1: Create Formatting node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/formatting.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/formatting.yaml
# Formatting — Technical formatting rules for a locale (dates, numbers, currency, phone, address, units)
#
# Tier: technical (always loaded together)
# Stability: High - changes rarely

node:
  name: Formatting
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: technical
  icon: "🔢"
  description: "Technical formatting rules: dates, numbers, currency, phone, address, units"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Formatting"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Date, number, currency formatting for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for numeric/date formatting. TRIGGERS: dates, prices, numbers. NOT: content style."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Date formatting
    date_format:
      type: string
      required: true
      description: "Primary date format"
      example: "DD/MM/YYYY"

    date_formats:
      type: json
      required: false
      description: "All date formats by context"
      example:
        short: "DD/MM"
        medium: "DD/MM/YYYY"
        long: "D MMMM YYYY"
        full: "dddd D MMMM YYYY"

    # Number formatting
    decimal_separator:
      type: string
      required: true
      description: "Decimal separator"
      example: ","

    thousands_separator:
      type: string
      required: true
      description: "Thousands separator"
      example: " "

    # Currency
    currency_code:
      type: string
      required: true
      description: "ISO 4217 currency code"
      example: "EUR"

    currency_symbol:
      type: string
      required: true
      description: "Currency symbol"
      example: "€"

    currency_position:
      type: string
      required: true
      enum: [before, after]
      description: "Currency symbol position"
      example: "after"

    currency_format:
      type: string
      required: false
      description: "Full currency format pattern"
      example: "{amount} €"

    # Phone
    phone_format:
      type: string
      required: false
      description: "Phone number format"
      example: "XX XX XX XX XX"

    country_code:
      type: string
      required: false
      description: "International dialing code"
      example: "+33"

    # Address
    address_format:
      type: json
      required: false
      description: "Address line order and format"
      example:
        order: ["street", "postal_city", "country"]
        postal_city: "{postal} {city}"

    # Units
    measurement_system:
      type: string
      required: true
      enum: [metric, imperial, mixed]
      description: "Default measurement system"
      example: "metric"

    temperature_unit:
      type: string
      required: false
      enum: [celsius, fahrenheit]
      description: "Temperature display unit"
      example: "celsius"

  relations:
    incoming:
      - type: HAS_FORMATTING
        from: Locale
        cardinality: "1:1"
        description: "Locale has exactly one formatting config"
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/formatting.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/formatting.yaml
git commit -m "feat(schema): add Formatting node-kind (knowledge tier: technical)

Part of Locale Knowledge v10 refactor.
Technical tier node for date/number/currency/phone/address/unit formatting.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2: Create Slugification node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/slugification.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/slugification.yaml
# Slugification — URL slug rules for a locale (transliteration, stop words, URL conventions)
#
# Tier: technical (always loaded together)
# Stability: High - changes rarely

node:
  name: Slugification
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: technical
  icon: "🔗"
  description: "URL slug rules: transliteration, stop words, URL conventions"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Slugification"

    description:
      type: string
      required: true
      description: "Short description"
      example: "URL slug generation rules for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for URL/slug generation. TRIGGERS: slug, URL, permalink. NOT: content text."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Transliteration
    transliteration_map:
      type: json
      required: true
      description: "Character replacement map for URL-safe slugs"
      example:
        é: "e"
        è: "e"
        ê: "e"
        ë: "e"
        à: "a"
        â: "a"
        ç: "c"
        ù: "u"
        û: "u"
        ü: "u"
        î: "i"
        ï: "i"
        ô: "o"
        œ: "oe"
        æ: "ae"

    # Stop words
    stop_words:
      type: json
      required: true
      description: "Words to remove from slugs"
      example: ["le", "la", "les", "un", "une", "des", "du", "de", "et", "ou", "à", "en", "pour", "par", "sur", "avec"]

    # URL rules
    separator:
      type: string
      required: true
      description: "Word separator in slugs"
      example: "-"

    max_length:
      type: int
      required: false
      description: "Maximum slug length"
      example: 80

    lowercase:
      type: bool
      required: true
      description: "Force lowercase"
      example: true

    preserve_numbers:
      type: bool
      required: false
      description: "Keep numbers in slugs"
      example: true

    # Special rules
    word_boundaries:
      type: json
      required: false
      description: "Characters that create word boundaries"
      example: ["'", "'", "-", " "]

    abbreviation_expansions:
      type: json
      required: false
      description: "Abbreviations to expand in slugs"
      example:
        "n°": "numero"
        "&": "et"

  relations:
    incoming:
      - type: HAS_SLUGIFICATION
        from: Locale
        cardinality: "1:1"
        description: "Locale has exactly one slugification config"
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/slugification.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/slugification.yaml
git commit -m "feat(schema): add Slugification node-kind (knowledge tier: technical)

Part of Locale Knowledge v10 refactor.
Technical tier node for URL slug transliteration, stop words, conventions.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3: Create Adaptation node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/adaptation.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/adaptation.yaml
# Adaptation — Content adaptation rules for a locale (length preferences, structure, SEO)
#
# Tier: technical (always loaded together)
# Stability: Medium - update quarterly

node:
  name: Adaptation
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: technical
  icon: "📐"
  description: "Content adaptation rules: length preferences, structure patterns, SEO guidelines"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Adaptation"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Content length, structure, SEO rules for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for content structure decisions. TRIGGERS: length, structure, SEO. NOT: tone/voice."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Length preferences
    length_multiplier:
      type: float
      required: true
      description: "Text expansion/contraction factor vs English"
      example: 1.15

    headline_max_chars:
      type: int
      required: false
      description: "Preferred max headline length"
      example: 70

    cta_max_chars:
      type: int
      required: false
      description: "Preferred max CTA length"
      example: 35

    # Structure
    paragraph_style:
      type: string
      required: false
      enum: [short, medium, long]
      description: "Preferred paragraph length"
      example: "medium"

    list_preference:
      type: string
      required: false
      enum: [bullets, numbers, prose]
      description: "Preferred list format"
      example: "bullets"

    heading_capitalization:
      type: string
      required: true
      enum: [sentence, title, uppercase]
      description: "Heading capitalization style"
      example: "sentence"

    # SEO
    title_structure:
      type: json
      required: false
      description: "SEO title patterns"
      example:
        home: "{brand} - {tagline}"
        product: "{product} | {brand}"
        article: "{title} - {brand}"

    meta_description_length:
      type: json
      required: false
      description: "Meta description length ranges"
      example:
        min: 120
        max: 160
        optimal: 145

    # Content patterns
    intro_style:
      type: string
      required: false
      description: "Preferred introduction style"
      example: "context-first"

    conclusion_style:
      type: string
      required: false
      description: "Preferred conclusion style"
      example: "action-oriented"

  relations:
    incoming:
      - type: HAS_ADAPTATION
        from: Locale
        cardinality: "1:1"
        description: "Locale has exactly one adaptation config"
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/adaptation.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/adaptation.yaml
git commit -m "feat(schema): add Adaptation node-kind (knowledge tier: technical)

Part of Locale Knowledge v10 refactor.
Technical tier node for content length, structure, SEO adaptation rules.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 4: Create Style node-kind (merge voice + identity)

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/style.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/style.yaml
# Style — Communication style for a locale (tone, formality, directness, warmth, humor, identity)
#
# Tier: style (loaded as cohesive unit)
# Stability: Medium - update quarterly
# Merges: locale-voice + locale-identity

node:
  name: Style
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: style
  icon: "🎭"
  description: "Communication style: tone, formality, directness, warmth, humor, identity traits"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Style"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Tone, formality, and communication identity for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for tone/voice decisions. TRIGGERS: formality, vous/tu, style. NOT: formatting."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # === FORMALITY (from locale-voice) ===
    formality_score:
      type: int
      required: true
      description: "Formality level 0-100"
      example: 72

    default_formality:
      type: string
      required: true
      enum: [formal, casual, mixed]
      description: "Default formality level"
      example: "formal"

    default_pronoun:
      type: string
      required: false
      description: "Default pronoun (vous, tu, Sie, du)"
      example: "vous"

    pronoun_rules:
      type: json
      required: false
      description: "Pronoun usage rules by context"
      example:
        formal: "vous"
        casual: "tu"
        switch_triggers: ["established relationship"]

    # === DIRECTNESS (from locale-voice) ===
    directness_score:
      type: int
      required: true
      description: "Directness level 0-100"
      example: 45

    directness_style:
      type: string
      required: true
      enum: [direct, indirect, balanced]
      description: "Overall directness style"
      example: "indirect"

    softening_patterns:
      type: json
      required: false
      description: "Patterns for softening direct statements"
      example:
        requests: "Pourriez-vous..."
        negatives: "Difficile"

    # === WARMTH (from locale-voice) ===
    warmth_score:
      type: int
      required: false
      description: "Overall warmth level 0-100"
      example: 55

    warmth_by_stage:
      type: json
      required: false
      description: "Warmth levels by relationship stage"
      example:
        initial: 45
        established: 60
        longterm: 70

    # === HUMOR (from locale-voice) ===
    humor_score:
      type: int
      required: false
      description: "Humor acceptance level 0-100"
      example: 40

    humor_types:
      type: json
      required: false
      description: "Acceptability of humor types"
      example:
        wordplay: "acceptable"
        irony: "subtle"
        slapstick: "avoid"

    # === SENTENCE STYLE (from locale-voice) ===
    avg_sentence_length:
      type: int
      required: false
      description: "Average sentence length in words"
      example: 18

    preferred_voice:
      type: string
      required: false
      enum: [active, passive, mixed]
      description: "Preferred grammatical voice"
      example: "mixed"

    rhythm_style:
      type: string
      required: false
      description: "Writing rhythm"
      example: "sophisticated"

    punctuation_rules:
      type: json
      required: false
      description: "Punctuation conventions"
      example:
        colon: "space_before"
        exclamation: "rare"
        quotes: "guillemets"

    # === HONORIFICS (from locale-voice) ===
    honorific_system:
      type: json
      required: false
      description: "Honorific system"
      example:
        levels: ["san", "sama"]
        usage:
          business: "san"

    # === IDENTITY (from locale-identity) ===
    self_references:
      type: json
      required: false
      description: "How to refer to brand"
      example:
        company: "nous"
        product: "notre solution"

    reader_references:
      type: json
      required: false
      description: "How to refer to reader"
      example:
        formal: "vous"
        familiar: "tu"

    authority_style:
      type: string
      required: false
      enum: [expert, peer, guide, helper]
      description: "Brand authority positioning"
      example: "expert"

    personality_traits:
      type: json
      required: false
      description: "Brand personality traits"
      example: ["professional", "trustworthy", "innovative"]

  relations:
    incoming:
      - type: HAS_STYLE
        from: Locale
        cardinality: "1:1"
        description: "Locale has exactly one style config"
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/style.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/style.yaml
git commit -m "feat(schema): add Style node-kind (merges voice + identity)

Part of Locale Knowledge v10 refactor.
Style tier node combining formality, directness, warmth, humor, identity.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 2: TermSet Nodes (6 domain-tagged nodes)

### Task 5: Create TermSet node-kind template

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/term-set.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/term-set.yaml
# TermSet — Domain-specific vocabulary for a locale
#
# Tier: semantic (contextual retrieval by domain)
# Domains: pricing, features, technical, marketing, support, general
# Stability: Medium - update monthly

node:
  name: TermSet
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: semantic
  icon: "📖"
  description: "Domain-specific terminology and vocabulary"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Pricing Terms"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Pricing-specific vocabulary for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for domain-specific terms. TRIGGERS: pricing words. NOT: general vocabulary."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Domain identification
    domain:
      type: string
      required: true
      enum: [pricing, features, technical, marketing, support, general]
      description: "Vocabulary domain"
      example: "pricing"

    # Terms collection
    terms:
      type: json
      required: true
      description: "Term mappings with context"
      example:
        price: "prix"
        pricing: "tarification"
        subscription: "abonnement"
        plan: "formule"
        billing: "facturation"
        invoice: "facture"
        discount: "remise"
        free_trial: "essai gratuit"

    # Preferred usage
    preferred_terms:
      type: json
      required: false
      description: "Preferred term choices when multiple options exist"
      example:
        "price point": "niveau de prix"
        "monthly fee": "mensualité"

    # Terms to avoid
    avoid_terms:
      type: json
      required: false
      description: "Terms to avoid with alternatives"
      example:
        "cheap": "abordable"
        "expensive": "premium"

    # Context-specific variants
    variants_by_context:
      type: json
      required: false
      description: "Term variants by context"
      example:
        formal:
          buy: "acquérir"
        casual:
          buy: "acheter"

  relations:
    incoming:
      - type: HAS_TERMS
        from: Locale
        cardinality: "1:N"
        description: "Locale has multiple term sets (one per domain)"
        properties:
          domain: string
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/term-set.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/term-set.yaml
git commit -m "feat(schema): add TermSet node-kind (6 domains: pricing/features/technical/marketing/support/general)

Part of Locale Knowledge v10 refactor.
Semantic tier nodes for domain-specific vocabulary retrieval.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 3: ExpressionSet Nodes (3 register-tagged nodes)

### Task 6: Create ExpressionSet node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/expression-set.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/expression-set.yaml
# ExpressionSet — Register-specific expressions and idioms for a locale
#
# Tier: semantic (contextual retrieval by register)
# Registers: formal, neutral, casual
# Stability: Medium - update quarterly

node:
  name: ExpressionSet
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: semantic
  icon: "💬"
  description: "Register-specific expressions, idioms, and phrases"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Formal Expressions"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Formal register expressions for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for register-appropriate phrasing. TRIGGERS: formal context. NOT: casual copy."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Register identification
    register:
      type: string
      required: true
      enum: [formal, neutral, casual]
      description: "Expression register"
      example: "formal"

    # Expressions collection
    expressions:
      type: json
      required: true
      description: "Expression mappings with usage"
      example:
        greeting: "Nous avons le plaisir de vous présenter"
        thanks: "Nous vous remercions de votre confiance"
        invitation: "Nous vous invitons à découvrir"
        confirmation: "Nous confirmons la bonne réception"

    # Idioms safe to use
    idioms:
      type: json
      required: false
      description: "Cultural idioms appropriate for register"
      example:
        certainty: "sans l'ombre d'un doute"
        quality: "la crème de la crème"
        precision: "au millimètre près"

    # Transition phrases
    transitions:
      type: json
      required: false
      description: "Transition phrases for flow"
      example:
        addition: ["de plus", "par ailleurs", "en outre"]
        contrast: ["cependant", "néanmoins", "toutefois"]
        conclusion: ["en définitive", "pour conclure"]

    # Emphasis phrases
    emphasis:
      type: json
      required: false
      description: "Phrases for emphasis"
      example:
        strong: "Il est essentiel de souligner que"
        moderate: "Il convient de noter que"
        subtle: "On remarquera que"

  relations:
    incoming:
      - type: HAS_EXPRESSIONS
        from: Locale
        cardinality: "1:N"
        description: "Locale has multiple expression sets (one per register)"
        properties:
          register: string
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/expression-set.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/expression-set.yaml
git commit -m "feat(schema): add ExpressionSet node-kind (3 registers: formal/neutral/casual)

Part of Locale Knowledge v10 refactor.
Semantic tier nodes for register-appropriate expressions retrieval.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 4: PatternSet Nodes (4 usage-tagged nodes)

### Task 7: Create PatternSet node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/pattern-set.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/pattern-set.yaml
# PatternSet — Usage-specific content patterns for a locale
#
# Tier: semantic (contextual retrieval by usage)
# Usages: cta, headlines, body, social
# Stability: Medium - update quarterly

node:
  name: PatternSet
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: semantic
  icon: "📝"
  description: "Usage-specific content patterns and templates"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) CTA Patterns"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Call-to-action patterns for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for CTA writing. TRIGGERS: button, action, convert. NOT: headlines."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Usage identification
    usage:
      type: string
      required: true
      enum: [cta, headlines, body, social]
      description: "Pattern usage category"
      example: "cta"

    # Pattern templates
    patterns:
      type: json
      required: true
      description: "Content patterns with placeholders"
      example:
        primary_action: "Découvrir {product}"
        secondary_action: "En savoir plus"
        urgency: "Profitez-en maintenant"
        benefit: "Bénéficiez de {benefit}"

    # Structure templates
    structures:
      type: json
      required: false
      description: "Structural patterns"
      example:
        short: "{verb} {object}"
        medium: "{verb} {object} {benefit}"
        long: "{context}, {verb} {object} et {benefit}"

    # Effective verbs
    power_verbs:
      type: json
      required: false
      description: "High-conversion verbs for this usage"
      example: ["découvrir", "profiter", "bénéficier", "accéder", "obtenir"]

    # Patterns to avoid
    avoid_patterns:
      type: json
      required: false
      description: "Ineffective or culturally inappropriate patterns"
      example:
        aggressive: ["Achetez maintenant!", "Ne ratez pas!"]
        overused: ["Cliquez ici"]

  relations:
    incoming:
      - type: HAS_PATTERNS
        from: Locale
        cardinality: "1:N"
        description: "Locale has multiple pattern sets (one per usage)"
        properties:
          usage: string
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/pattern-set.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/pattern-set.yaml
git commit -m "feat(schema): add PatternSet node-kind (4 usages: cta/headlines/body/social)

Part of Locale Knowledge v10 refactor.
Semantic tier nodes for usage-specific content patterns retrieval.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 5: CultureSet Nodes (4 type-tagged nodes)

### Task 8: Create CultureSet node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/culture-set.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/culture-set.yaml
# CultureSet — Type-specific cultural knowledge for a locale
#
# Tier: semantic (contextual retrieval by type)
# Types: values, references, celebrities, calendar
# Stability: Low - update monthly for references/celebrities/calendar

node:
  name: CultureSet
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: semantic
  icon: "🌍"
  description: "Type-specific cultural knowledge and references"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Cultural Values"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Core cultural values for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for cultural alignment. TRIGGERS: values, trust. NOT: calendar dates."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Type identification
    culture_type:
      type: string
      required: true
      enum: [values, references, celebrities, calendar]
      description: "Cultural knowledge type"
      example: "values"

    # For type: values
    core_values:
      type: json
      required: false
      description: "Core cultural values (when type=values)"
      example:
        hierarchy: "respected"
        individualism: "moderate"
        uncertainty_avoidance: "high"
        long_term_orientation: "moderate"

    trust_factors:
      type: json
      required: false
      description: "What builds trust in this culture"
      example:
        credentials: "very important"
        testimonials: "important"
        certifications: "valued"
        guarantees: "expected"

    decision_factors:
      type: json
      required: false
      description: "Purchase decision factors"
      example:
        quality: "primary"
        price: "secondary"
        brand: "important"
        origin: "valued"

    # For type: references
    landmarks:
      type: json
      required: false
      description: "Safe-to-reference landmarks (when type=references)"
      example: ["Tour Eiffel", "Mont-Blanc", "Côte d'Azur"]

    events:
      type: json
      required: false
      description: "Cultural events safe to reference"
      example: ["Tour de France", "Roland-Garros", "Festival de Cannes"]

    shared_knowledge:
      type: json
      required: false
      description: "Universally known cultural references"
      example:
        literature: ["Le Petit Prince", "Les Misérables"]
        film: ["Amélie", "Intouchables"]
        cuisine: ["croissant", "baguette", "fromage"]

    # For type: celebrities
    safe_celebrities:
      type: json
      required: false
      description: "Non-controversial public figures (when type=celebrities)"
      example:
        sports: ["Zinedine Zidane", "Kylian Mbappé"]
        business: ["Bernard Arnault"]
        culture: ["Thomas Pesquet"]

    celebrity_guidelines:
      type: json
      required: false
      description: "Guidelines for celebrity references"
      example:
        avoid_political: true
        require_recent_relevance: true
        check_controversy_status: true

    # For type: calendar
    holidays:
      type: json
      required: false
      description: "Public holidays (when type=calendar)"
      example:
        - name: "Jour de l'An"
          date: "01-01"
          marketing_relevance: "high"
        - name: "Fête du Travail"
          date: "05-01"
          marketing_relevance: "low"

    vacation_periods:
      type: json
      required: false
      description: "Major vacation periods"
      example:
        summer:
          peak: "July-August"
          note: "Business slowdown"
        winter:
          peak: "December 20 - January 5"

    business_calendar:
      type: json
      required: false
      description: "Business year patterns"
      example:
        fiscal_year_end: "December"
        budget_season: "October-November"
        slow_periods: ["August", "December 24-31"]

  relations:
    incoming:
      - type: HAS_CULTURE
        from: Locale
        cardinality: "1:N"
        description: "Locale has multiple culture sets (one per type)"
        properties:
          type: string
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/culture-set.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/culture-set.yaml
git commit -m "feat(schema): add CultureSet node-kind (4 types: values/references/celebrities/calendar)

Part of Locale Knowledge v10 refactor.
Semantic tier nodes for type-specific cultural knowledge retrieval.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 6: TabooSet Nodes (3 severity-tagged nodes)

### Task 9: Create TabooSet node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/taboo-set.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/taboo-set.yaml
# TabooSet — Severity-specific taboos and constraints for a locale
#
# Tier: semantic (contextual retrieval by severity)
# Severities: avoid, careful, legal
# Stability: High - critical safety knowledge

node:
  name: TabooSet
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: semantic
  icon: "⚠️"
  description: "Severity-specific taboos, constraints, and compliance rules"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) Avoid Topics"

    description:
      type: string
      required: true
      description: "Short description"
      example: "Topics to never mention for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: always loaded for safety. TRIGGERS: content generation. NOT: optional."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Severity identification
    severity:
      type: string
      required: true
      enum: [avoid, careful, legal]
      description: "Taboo severity level"
      example: "avoid"

    # For severity: avoid
    forbidden_topics:
      type: json
      required: false
      description: "Topics to never mention (when severity=avoid)"
      example:
        - topic: "political parties"
          reason: "divisive"
        - topic: "religious beliefs"
          reason: "sensitive"
        - topic: "historical controversies"
          reason: "contentious"

    forbidden_comparisons:
      type: json
      required: false
      description: "Comparisons to avoid"
      example:
        - "comparing to competitors by name"
        - "war/conflict metaphors"
        - "religious analogies"

    forbidden_imagery:
      type: json
      required: false
      description: "Imagery to avoid"
      example:
        - "national flags in commercial context"
        - "religious symbols"
        - "political symbols"

    # For severity: careful
    sensitive_topics:
      type: json
      required: false
      description: "Topics requiring careful handling (when severity=careful)"
      example:
        - topic: "environmental claims"
          guideline: "must be substantiated"
        - topic: "health benefits"
          guideline: "avoid medical claims"
        - topic: "price comparisons"
          guideline: "must be verifiable"

    cultural_sensitivities:
      type: json
      required: false
      description: "Cultural sensitivities to respect"
      example:
        - sensitivity: "regional identities"
          guideline: "acknowledge diversity"
        - sensitivity: "language variants"
          guideline: "use metropolitan French"

    # For severity: legal
    legal_requirements:
      type: json
      required: false
      description: "Legal compliance requirements (when severity=legal)"
      example:
        - requirement: "GDPR privacy notice"
          applies_to: "data collection"
        - requirement: "Cookie consent"
          applies_to: "all pages"
        - requirement: "Price display"
          applies_to: "must include TTC"

    advertising_rules:
      type: json
      required: false
      description: "Advertising regulation compliance"
      example:
        comparative_advertising: "strictly regulated"
        superlatives: "must be provable"
        testimonials: "must be genuine"

    industry_specific:
      type: json
      required: false
      description: "Industry-specific regulations"
      example:
        financial: "AMF disclaimers required"
        health: "ANSM guidelines"
        alcohol: "mandatory warnings"

  relations:
    incoming:
      - type: HAS_TABOOS
        from: Locale
        cardinality: "1:N"
        description: "Locale has multiple taboo sets (one per severity)"
        properties:
          severity: string
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/taboo-set.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/taboo-set.yaml
git commit -m "feat(schema): add TabooSet node-kind (3 severities: avoid/careful/legal)

Part of Locale Knowledge v10 refactor.
Semantic tier nodes for severity-based constraint retrieval.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 7: AudienceSet Nodes (3 segment-tagged nodes)

### Task 10: Create AudienceSet node-kind

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/audience-set.yaml`

**Step 1: Write the node-kind YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/audience-set.yaml
# AudienceSet — Segment-specific audience knowledge for a locale
#
# Tier: semantic (contextual retrieval by segment)
# Segments: b2b, b2c, general
# Stability: Medium - update quarterly

node:
  name: AudienceSet
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: semantic
  icon: "👥"
  description: "Segment-specific audience behavior and preferences"

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "French (France) B2B Audience"

    description:
      type: string
      required: true
      description: "Short description"
      example: "B2B audience behavior for fr-FR"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: for B2B content. TRIGGERS: business, enterprise. NOT: consumer content."

    created_at:
      type: datetime
      required: true
      description: "Creation timestamp"

    updated_at:
      type: datetime
      required: true
      description: "Last update timestamp"

  properties:
    # Segment identification
    segment:
      type: string
      required: true
      enum: [b2b, b2c, general]
      description: "Audience segment"
      example: "b2b"

    # For segment: b2b
    decision_makers:
      type: json
      required: false
      description: "Decision maker characteristics (when segment=b2b)"
      example:
        typical_roles: ["DSI", "Directeur Marketing", "DG"]
        decision_process: "committee-based"
        decision_timeline: "3-6 months"

    purchasing_behavior:
      type: json
      required: false
      description: "B2B purchasing patterns"
      example:
        procurement_process: "formal RFP common"
        budget_cycles: "annual, Q4 planning"
        vendor_requirements: ["references", "certifications", "support SLA"]

    trust_signals:
      type: json
      required: false
      description: "What builds trust for this segment"
      example:
        b2b:
          - "client logos"
          - "case studies"
          - "industry certifications"
          - "data security compliance"

    # For segment: b2c
    consumer_behavior:
      type: json
      required: false
      description: "Consumer behavior patterns (when segment=b2c)"
      example:
        impulse_purchase_threshold: "50€"
        research_behavior: "comparison sites, reviews"
        preferred_channels: ["mobile", "social"]

    payment_preferences:
      type: json
      required: false
      description: "Preferred payment methods"
      example:
        online: ["carte bancaire", "PayPal"]
        installments: "3x sans frais popular"

    # For segment: general
    demographics:
      type: json
      required: false
      description: "Market demographics (when segment=general)"
      example:
        population: 67000000
        internet_penetration: 0.92
        mobile_penetration: 0.95

    digital_behavior:
      type: json
      required: false
      description: "General digital behavior"
      example:
        social_platforms: ["Facebook", "Instagram", "LinkedIn"]
        search_engine: "Google (94%)"
        ecommerce_adoption: "high"

    media_consumption:
      type: json
      required: false
      description: "Media consumption patterns"
      example:
        video: "YouTube, streaming dominant"
        news: "online news, podcasts growing"
        social: "Instagram, TikTok (younger)"

  relations:
    incoming:
      - type: HAS_AUDIENCE
        from: Locale
        cardinality: "1:N"
        description: "Locale has multiple audience sets (one per segment)"
        properties:
          segment: string
```

**Step 2: Verify YAML syntax**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && python3 -c "import yaml; yaml.safe_load(open('packages/core/models/node-classes/global/knowledge/audience-set.yaml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/audience-set.yaml
git commit -m "feat(schema): add AudienceSet node-kind (3 segments: b2b/b2c/general)

Part of Locale Knowledge v10 refactor.
Semantic tier nodes for segment-specific audience knowledge retrieval.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 8: New Arc-Kinds (10 knowledge relationship arcs)

### Task 11: Create knowledge arc-classes

**Files:**
- Create: `packages/core/models/arc-classes/ownership/has-formatting.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-slugification.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-adaptation.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-style.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-terms.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-expressions.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-patterns.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-culture.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-taboos.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-audience.yaml`

**Step 1: Create all arc-kind YAMLs**

```yaml
# packages/core/models/arc-classes/ownership/has-formatting.yaml
arc:
  name: HAS_FORMATTING
  family: ownership
  scope: intra_realm
  source: Locale
  target: Formatting
  cardinality: one_to_one
  llm_context: Locale has exactly one formatting configuration.
  cypher_pattern: (Locale)-[:HAS_FORMATTING]->(Formatting)
```

```yaml
# packages/core/models/arc-classes/ownership/has-slugification.yaml
arc:
  name: HAS_SLUGIFICATION
  family: ownership
  scope: intra_realm
  source: Locale
  target: Slugification
  cardinality: one_to_one
  llm_context: Locale has exactly one slugification configuration.
  cypher_pattern: (Locale)-[:HAS_SLUGIFICATION]->(Slugification)
```

```yaml
# packages/core/models/arc-classes/ownership/has-adaptation.yaml
arc:
  name: HAS_ADAPTATION
  family: ownership
  scope: intra_realm
  source: Locale
  target: Adaptation
  cardinality: one_to_one
  llm_context: Locale has exactly one adaptation configuration.
  cypher_pattern: (Locale)-[:HAS_ADAPTATION]->(Adaptation)
```

```yaml
# packages/core/models/arc-classes/ownership/has-style.yaml
arc:
  name: HAS_STYLE
  family: ownership
  scope: intra_realm
  source: Locale
  target: Style
  cardinality: one_to_one
  llm_context: Locale has exactly one style configuration.
  cypher_pattern: (Locale)-[:HAS_STYLE]->(Style)
```

```yaml
# packages/core/models/arc-classes/ownership/has-terms.yaml
arc:
  name: HAS_TERMS
  family: ownership
  scope: intra_realm
  source: Locale
  target: TermSet
  cardinality: one_to_many
  llm_context: Locale has multiple term sets, one per domain.
  cypher_pattern: (Locale)-[:HAS_TERMS {domain: $domain}]->(TermSet)
  properties:
    domain:
      type: string
      required: true
      description: "Term domain (pricing, features, technical, marketing, support, general)"
```

```yaml
# packages/core/models/arc-classes/ownership/has-expressions.yaml
arc:
  name: HAS_EXPRESSIONS
  family: ownership
  scope: intra_realm
  source: Locale
  target: ExpressionSet
  cardinality: one_to_many
  llm_context: Locale has multiple expression sets, one per register.
  cypher_pattern: (Locale)-[:HAS_EXPRESSIONS {register: $register}]->(ExpressionSet)
  properties:
    register:
      type: string
      required: true
      description: "Expression register (formal, neutral, casual)"
```

```yaml
# packages/core/models/arc-classes/ownership/has-patterns.yaml
arc:
  name: HAS_PATTERNS
  family: ownership
  scope: intra_realm
  source: Locale
  target: PatternSet
  cardinality: one_to_many
  llm_context: Locale has multiple pattern sets, one per usage.
  cypher_pattern: (Locale)-[:HAS_PATTERNS {usage: $usage}]->(PatternSet)
  properties:
    usage:
      type: string
      required: true
      description: "Pattern usage (cta, headlines, body, social)"
```

```yaml
# packages/core/models/arc-classes/ownership/has-culture.yaml
arc:
  name: HAS_CULTURE
  family: ownership
  scope: intra_realm
  source: Locale
  target: CultureSet
  cardinality: one_to_many
  llm_context: Locale has multiple culture sets, one per type.
  cypher_pattern: (Locale)-[:HAS_CULTURE {type: $type}]->(CultureSet)
  properties:
    type:
      type: string
      required: true
      description: "Culture type (values, references, celebrities, calendar)"
```

```yaml
# packages/core/models/arc-classes/ownership/has-taboos.yaml
arc:
  name: HAS_TABOOS
  family: ownership
  scope: intra_realm
  source: Locale
  target: TabooSet
  cardinality: one_to_many
  llm_context: Locale has multiple taboo sets, one per severity.
  cypher_pattern: (Locale)-[:HAS_TABOOS {severity: $severity}]->(TabooSet)
  properties:
    severity:
      type: string
      required: true
      description: "Taboo severity (avoid, careful, legal)"
```

```yaml
# packages/core/models/arc-classes/ownership/has-audience.yaml
arc:
  name: HAS_AUDIENCE
  family: ownership
  scope: intra_realm
  source: Locale
  target: AudienceSet
  cardinality: one_to_many
  llm_context: Locale has multiple audience sets, one per segment.
  cypher_pattern: (Locale)-[:HAS_AUDIENCE {segment: $segment}]->(AudienceSet)
  properties:
    segment:
      type: string
      required: true
      description: "Audience segment (b2b, b2c, general)"
```

**Step 2: Verify all arc-kind YAMLs**

Run: `for f in has-formatting has-slugification has-adaptation has-style has-terms has-expressions has-patterns has-culture has-taboos has-audience; do python3 -c "import yaml; yaml.safe_load(open('packages/core/models/arc-classes/ownership/${f}.yaml'))"; done`
Expected: No output (all valid)

**Step 3: Commit**

```bash
git add packages/core/models/arc-classes/ownership/has-*.yaml
git commit -m "feat(schema): add 10 knowledge arc-classes for Locale Knowledge v10

HAS_FORMATTING, HAS_SLUGIFICATION, HAS_ADAPTATION, HAS_STYLE (1:1)
HAS_TERMS, HAS_EXPRESSIONS, HAS_PATTERNS, HAS_CULTURE, HAS_TABOOS, HAS_AUDIENCE (1:N with properties)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 9: Delete Old Node-Kinds

### Task 12: Delete 14 old locale knowledge node-classes

**Files:**
- Delete: `packages/core/models/node-classes/global/knowledge/locale-voice.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-identity.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-lexicon.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-culture.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-culture-references.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-market.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-rules-formatting.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-rules-adaptation.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/locale-rules-slug.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/constraint.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/expression.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/metaphor.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/pattern.yaml`
- Delete: `packages/core/models/node-classes/global/knowledge/reference.yaml`

**Step 1: Delete old node-kind files**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
rm packages/core/models/node-classes/global/knowledge/locale-voice.yaml
rm packages/core/models/node-classes/global/knowledge/locale-identity.yaml
rm packages/core/models/node-classes/global/knowledge/locale-lexicon.yaml
rm packages/core/models/node-classes/global/knowledge/locale-culture.yaml
rm packages/core/models/node-classes/global/knowledge/locale-culture-references.yaml
rm packages/core/models/node-classes/global/knowledge/locale-market.yaml
rm packages/core/models/node-classes/global/knowledge/locale-rules-formatting.yaml
rm packages/core/models/node-classes/global/knowledge/locale-rules-adaptation.yaml
rm packages/core/models/node-classes/global/knowledge/locale-rules-slug.yaml
rm packages/core/models/node-classes/global/knowledge/constraint.yaml
rm packages/core/models/node-classes/global/knowledge/expression.yaml
rm packages/core/models/node-classes/global/knowledge/metaphor.yaml
rm packages/core/models/node-classes/global/knowledge/pattern.yaml
rm packages/core/models/node-classes/global/knowledge/reference.yaml
```

**Step 2: Verify deletion**

Run: `ls packages/core/models/node-classes/global/knowledge/`
Expected: Only new files (formatting.yaml, slugification.yaml, adaptation.yaml, style.yaml, term-set.yaml, expression-set.yaml, pattern-set.yaml, culture-set.yaml, taboo-set.yaml, audience-set.yaml)

**Step 3: Commit**

```bash
git add -A packages/core/models/node-classes/global/knowledge/
git commit -m "refactor(schema): delete 14 old locale knowledge node-classes

Replaced by 10 new granular node-classes in Locale Knowledge v10:
- locale-voice + locale-identity → Style
- locale-rules-* → Formatting, Slugification, Adaptation
- locale-lexicon, expression, metaphor → TermSet, ExpressionSet
- locale-culture, locale-culture-references, locale-market → CultureSet, AudienceSet
- constraint, pattern, reference → TabooSet, PatternSet

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 13: Delete old arc-classes referencing deleted nodes

**Files:**
- Delete: `packages/core/models/arc-classes/ownership/has-voice.yaml` (if exists)
- Delete: `packages/core/models/arc-classes/ownership/has-identity.yaml` (if exists)
- Delete: `packages/core/models/arc-classes/ownership/has-lexicon.yaml` (if exists)
- Update: `packages/core/models/relations.yaml` (remove old relations)

**Step 1: Check for and delete old arc-kind files**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
# Check which files exist
ls packages/core/models/arc-classes/ownership/ | grep -E "(voice|identity|lexicon|culture|market|rules)"
# Delete any that exist
rm -f packages/core/models/arc-classes/ownership/has-voice.yaml
rm -f packages/core/models/arc-classes/ownership/has-identity.yaml
rm -f packages/core/models/arc-classes/ownership/has-lexicon.yaml
rm -f packages/core/models/arc-classes/ownership/has-culture-refs.yaml
rm -f packages/core/models/arc-classes/ownership/has-market.yaml
```

**Step 2: Update relations.yaml to remove old relations**

Read `packages/core/models/relations.yaml` and remove any relations referencing deleted node types.

**Step 3: Commit**

```bash
git add -A packages/core/models/arc-classes/
git add packages/core/models/relations.yaml
git commit -m "refactor(schema): remove old arc-classes for deleted locale knowledge nodes

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 10: Update Rust Parsers

### Task 14: Add knowledge_tier to NodeDef

**Files:**
- Modify: `tools/novanet/src/parsers/yaml_node.rs`
- Test: `tools/novanet/src/parsers/yaml_node.rs` (inline tests)

**Step 1: Write the failing test**

Add to `tools/novanet/src/parsers/yaml_node.rs`:

```rust
#[test]
fn parse_knowledge_tier() {
    let yaml = r#"
node:
  name: Formatting
  realm: global
  layer: knowledge
  trait: knowledge
  knowledge_tier: technical
  description: "Test"
"#;
    let def: NodeDefWrapper = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(def.node.knowledge_tier, Some("technical".to_string()));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet parse_knowledge_tier`
Expected: FAIL with "no field `knowledge_tier`"

**Step 3: Add knowledge_tier field to NodeDef**

In `tools/novanet/src/parsers/yaml_node.rs`, add to `NodeDef` struct:

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct NodeDef {
    pub name: String,
    pub realm: String,
    pub layer: String,
    #[serde(rename = "trait")]
    pub node_trait: LocaleBehavior,
    /// Knowledge tier for semantic nodes: technical, style, semantic
    #[serde(default)]
    pub knowledge_tier: Option<String>,
    // ... rest of fields
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p novanet parse_knowledge_tier`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/parsers/yaml_node.rs
git commit -m "feat(parser): add knowledge_tier field to NodeDef

Supports technical/style/semantic tiers for Locale Knowledge v10.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 15: Update kind generator to emit knowledge_tier

**Files:**
- Modify: `tools/novanet/src/generators/kind.rs`
- Update: Snapshot test

**Step 1: Write the failing test**

Add assertion to existing test or create new test:

```rust
#[test]
fn kind_cypher_includes_knowledge_tier() {
    let mut node = make_node("Formatting", "global", "knowledge", LocaleBehavior::Knowledge);
    node.def.knowledge_tier = Some("technical".to_string());
    let cypher = generate_kind_cypher(&[node]).unwrap();
    assert!(cypher.contains("knowledge_tier: 'technical'"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet kind_cypher_includes_knowledge_tier`
Expected: FAIL

**Step 3: Update generate_kind_cypher to include knowledge_tier**

In `tools/novanet/src/generators/kind.rs`, modify the Kind node MERGE statement to include:

```rust
// Add knowledge_tier if present
if let Some(ref tier) = node.def.knowledge_tier {
    writeln!(out, "    knowledge_tier: {},", cypher_str(tier)).unwrap();
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p novanet kind_cypher_includes_knowledge_tier`
Expected: PASS

**Step 5: Update snapshot**

Run: `cargo insta test --accept`

**Step 6: Commit**

```bash
git add tools/novanet/src/generators/kind.rs
git add tools/novanet/src/generators/snapshots/
git commit -m "feat(generator): emit knowledge_tier property in Kind Cypher

Supports TUI grouping of knowledge nodes by tier.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 11: Update TUI

### Task 16: Add knowledge_tier to KindInfo

**Files:**
- Modify: `tools/novanet/src/tui/data.rs`

**Step 1: Add knowledge_tier field to KindInfo struct**

```rust
pub struct KindInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
    pub trait_name: String,
    pub instance_count: i64,
    pub arcs: Vec<ArcInfo>,
    pub yaml_path: String,
    pub properties: Vec<String>,
    pub required_properties: Vec<String>,
    pub schema_hint: String,
    pub context_budget: String,
    /// Knowledge tier for grouping (technical, style, semantic)
    pub knowledge_tier: Option<String>,
}
```

**Step 2: Update Cypher query to fetch knowledge_tier**

In `TaxonomyTree::load()`, add to RETURN clause:

```cypher
coalesce(k.knowledge_tier, null) AS knowledge_tier
```

**Step 3: Update row parsing to extract knowledge_tier**

```rust
let knowledge_tier: Option<String> = row.get("knowledge_tier").ok();
```

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/data.rs
git commit -m "feat(tui): add knowledge_tier to KindInfo for tier grouping

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 17: Update TUI tree rendering to show tier groups

**Files:**
- Modify: `tools/novanet/src/tui/ui.rs`

**Step 1: Group kinds by knowledge_tier when rendering knowledge layer**

In the tree rendering code, when rendering kinds under the "knowledge" layer, group them by tier:

```rust
// In render_kinds_for_layer or equivalent function
if layer.key == "knowledge" {
    // Group by tier: technical, style, semantic, (none)
    let mut by_tier: BTreeMap<String, Vec<&KindInfo>> = BTreeMap::new();
    for kind in &layer.kinds {
        let tier = kind.knowledge_tier.as_deref().unwrap_or("other");
        by_tier.entry(tier.to_string()).or_default().push(kind);
    }

    // Render in order: technical, style, semantic, other
    for tier_key in ["technical", "style", "semantic", "other"] {
        if let Some(kinds) = by_tier.get(tier_key) {
            // Render tier header
            let tier_display = match tier_key {
                "technical" => "🔧 Technical",
                "style" => "🎭 Style",
                "semantic" => "📚 Semantic",
                _ => "📦 Other",
            };
            // Render tier as collapsible group
            // ... render kinds under this tier
        }
    }
}
```

**Step 2: Add tier collapse state**

Add tier collapse keys like `"tier:knowledge:technical"` to collapsed set.

**Step 3: Test TUI manually**

Run: `cargo run -- tui`
Expected: Knowledge layer shows grouped kinds under Technical, Style, Semantic headers

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/ui.rs
git commit -m "feat(tui): render knowledge layer with tier groupings

Shows Technical (Formatting, Slugification, Adaptation),
Style (Style), and Semantic (TermSet, ExpressionSet, etc.)
as collapsible sub-groups.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 12: Regenerate and Validate

### Task 18: Regenerate all schema artifacts

**Step 1: Run schema generate**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq/tools/novanet && cargo run -- schema generate`
Expected: All 10 artifacts regenerated successfully

**Step 2: Run schema validate**

Run: `cargo run -- schema validate --strict`
Expected: No errors or warnings

**Step 3: Commit generated artifacts**

```bash
git add packages/db/seed/
git add packages/core/src/
git commit -m "chore(generated): regenerate schema artifacts for Locale Knowledge v10

10 new node-classes, 10 new arc-classes, 14 deleted node-classes.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 19: Run all tests

**Step 1: Run Rust tests**

Run: `cargo test`
Expected: All tests pass

**Step 2: Run TypeScript tests**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && pnpm test`
Expected: All tests pass

**Step 3: Run linting**

Run: `cargo clippy -- -D warnings && pnpm lint`
Expected: No warnings

---

### Task 20: Seed Neo4j and verify TUI

**Step 1: Seed database**

Run: `cd /Users/thibaut/supernovae-st/novanet-hq && pnpm infra:reset`
Expected: Database reset and seeded successfully

**Step 2: Launch TUI and verify tree**

Run: `cargo run -- tui`
Expected:
- Knowledge layer shows 10 node-classes (not 14)
- Kinds are grouped by tier (Technical, Style, Semantic)
- No errors in console

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat(schema): complete Locale Knowledge v10 refactor

- 14 flat nodes → 10 granular nodes with tier groupings
- Technical tier: Formatting, Slugification, Adaptation
- Style tier: Style (merged from voice + identity)
- Semantic tier: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
- 10 new arc-classes for contextual retrieval
- TUI shows tier groupings for better taxonomy understanding

Closes design: docs/plans/2026-02-04-locale-knowledge-v10-design.md

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Verification Checklist

After implementation, verify:

- [ ] 14 old node-classes deleted from `global/knowledge/`
- [ ] 10 new node-classes created with `knowledge_tier` property
- [ ] 10 new arc-classes created in `ownership/`
- [ ] `relations.yaml` updated (old relations removed)
- [ ] Rust parser supports `knowledge_tier` field
- [ ] Kind generator emits `knowledge_tier` to Cypher
- [ ] TUI groups knowledge kinds by tier
- [ ] All tests pass (`cargo test`, `pnpm test`)
- [ ] No clippy warnings
- [ ] Schema validates (`cargo run -- schema validate --strict`)
- [ ] Neo4j seeds successfully
- [ ] TUI renders correctly with 10 knowledge nodes in 3 tiers
