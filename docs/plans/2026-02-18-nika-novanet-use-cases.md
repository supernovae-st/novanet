# Nika + NovaNet Integration — Use Cases Design

**Date:** 2026-02-18
**Status:** Draft
**Target:** QR Code AI (https://qrcode-ai.com)
**Systems:** Nika v0.2 (invoke: verb) + NovaNet v0.13.1 (MCP)

---

## Overview

This document defines 5 concrete use cases showing Nika orchestrating NovaNet via MCP to produce
real content for QR Code AI. Each use case maps to a distinct value proposition:

| # | Use Case | Primary Value |
|---|----------|---------------|
| 1 | Multi-Locale Page Generation | Scale: one entity, 200 locales |
| 2 | SEO Content Sprint | Traffic: entity cluster → SEO pages |
| 3 | Entity Knowledge Retrieval | Context: semantic graph traversal |
| 4 | Block Generation with Locale Context | Quality: knowledge atoms in prompts |
| 5 | Semantic Content Planning | Planning: coverage gap discovery |

**Integration contract:**

```
Nika workflow
  └── invoke: novanet_generate    # Pull entity context + generate content
  └── invoke: novanet_traverse    # Walk semantic graph
  └── invoke: novanet_describe    # Get entity description + locale voice
       └── MCP Protocol
            └── NovaNet MCP Server
                 └── Neo4j (61 nodes, 182 arcs)
```

---

## Use Case 1: Multi-Locale Page Generation

### Name

`generate-page-multilingual`

### User Story

As a content engineer at QR Code AI, I want to generate a complete product page for the
"QR Code Art" entity across 5 priority locales simultaneously, so that we can launch localized
pages in one workflow run instead of 5 manual operations.

### Context

- **Entity:** `qr-code-art` (AI-generated artistic QR code)
- **Page:** `qr-code-art` landing page
- **Target locales:** `fr-FR`, `es-ES`, `de-DE`, `ja-JP`, `pt-BR`
- **NovaNet state:** Entity defined, EntityNative exists for each locale, Page + Blocks defined

### Workflow YAML

```yaml
# examples/generate-page-multilingual.nika.yaml
#
# DAG: locales → [gen_fr, gen_es, gen_de, gen_ja, gen_pt] → report
#
# PHASE 1: Input
# ┌─────────────┬──────────────┐
# │  entity_key │   locales    │
# └──────┬──────┴──────┬───────┘
#        │             │
#        ▼             ▼ (fan-out x5)
# PHASE 2: Parallel generation
# ┌──────┬──────┬──────┬──────┬───────┐
# │fr_FR │es_ES │de_DE │ja_JP │pt_BR  │
# └──────┴──────┴──────┴──────┴───────┘
#        │ (fan-in)
#        ▼
# PHASE 3: Report
# ┌───────────────┐
# │ generation_rpt│
# └───────────────┘
#
schema: 'nika/workflow@0.1'

provider: claude
model: claude-opus-4-5-20251101

mcp:
  novanet:
    transport: stdio
    command: node
    args: ['novanet-dev/tools/novanet-mcp/dist/index.js']

tasks:
  # ─────────────────────────────────────────────────────────────────
  # PHASE 1: Input
  # ─────────────────────────────────────────────────────────────────

  - id: entity_key
    exec:
      command: "echo 'qr-code-art'"

  # ─────────────────────────────────────────────────────────────────
  # PHASE 2: Parallel locale generation (fan-out)
  # ─────────────────────────────────────────────────────────────────

  - id: gen_fr
    use:
      entity: entity_key
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: '{{use.entity}}'
        locale: 'fr-FR'
        page_key: 'qr-code-art'
        include_blocks: true
    output:
      format: json

  - id: gen_es
    use:
      entity: entity_key
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: '{{use.entity}}'
        locale: 'es-ES'
        page_key: 'qr-code-art'
        include_blocks: true
    output:
      format: json

  - id: gen_de
    use:
      entity: entity_key
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: '{{use.entity}}'
        locale: 'de-DE'
        page_key: 'qr-code-art'
        include_blocks: true
    output:
      format: json

  - id: gen_ja
    use:
      entity: entity_key
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: '{{use.entity}}'
        locale: 'ja-JP'
        page_key: 'qr-code-art'
        include_blocks: true
    output:
      format: json

  - id: gen_pt
    use:
      entity: entity_key
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: '{{use.entity}}'
        locale: 'pt-BR'
        page_key: 'qr-code-art'
        include_blocks: true
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # PHASE 3: Generation report (fan-in)
  # ─────────────────────────────────────────────────────────────────

  - id: generation_report
    use:
      fr: gen_fr
      es: gen_es
      de: gen_de
      ja: gen_ja
      pt: gen_pt
      entity: entity_key
    infer:
      prompt: |
        Generate a generation summary report for entity: {{use.entity}}

        RESULTS:
        - fr-FR: {{use.fr.status}} — {{use.fr.blocks_generated}} blocks
        - es-ES: {{use.es.status}} — {{use.es.blocks_generated}} blocks
        - de-DE: {{use.de.status}} — {{use.de.blocks_generated}} blocks
        - ja-JP: {{use.ja.status}} — {{use.ja.blocks_generated}} blocks
        - pt-BR: {{use.pt.status}} — {{use.pt.blocks_generated}} blocks

        Summarize: total blocks generated, any failures, time estimate saved vs manual.
        Output as a production-ready deployment note.

flows:
  - source: entity_key
    target: [gen_fr, gen_es, gen_de, gen_ja, gen_pt]

  - source: [gen_fr, gen_es, gen_de, gen_ja, gen_pt]
    target: generation_report
```

### NovaNet Tools Used

| Tool | Called By | Purpose |
|------|-----------|---------|
| `novanet_generate` | `gen_fr`, `gen_es`, `gen_de`, `gen_ja`, `gen_pt` | Generates PageNative + BlockNative nodes for each locale |

### Expected Output

5 PageNative nodes written to Neo4j, each with all associated BlockNative nodes populated.
A plain-text deployment summary from the final `infer:` task, e.g.:

```
Generated: qr-code-art @ 5 locales
- 5 pages x 8 blocks = 40 BlockNative nodes created
- 0 failures
- Estimated time saved: ~4h vs manual authoring
Ready for CMS export.
```

---

## Use Case 2: SEO Content Sprint

### Name

`seo-content-sprint`

### User Story

As an SEO manager at QR Code AI, I want to automatically generate SEO-optimized landing pages for
all entities in the "restaurant" industry cluster across 3 locales, so that we can capture
long-tail search traffic without manually writing 30+ pages.

### Context

- **Entity cluster:** Restaurant-related entities (`menu-restaurant`, `qr-code-table-tent`,
  `qr-code-reviews`, `qr-code-contactless-payment`, `digital-menu`)
- **Target locales:** `fr-FR`, `en-US`, `es-MX`
- **SEO goal:** Target `qr code menu restaurant` keyword cluster

### Workflow YAML

```yaml
# examples/seo-content-sprint.nika.yaml
#
# DAG:
# PHASE 1: Discover restaurant entities via graph traversal
# PHASE 2: Fan-out SEO generation per entity x locale
# PHASE 3: Aggregate SEO report
#
schema: 'nika/workflow@0.1'

provider: claude
model: claude-opus-4-5-20251101

mcp:
  novanet:
    transport: stdio
    command: node
    args: ['novanet-dev/tools/novanet-mcp/dist/index.js']

tasks:
  # ─────────────────────────────────────────────────────────────────
  # PHASE 1: Entity discovery via semantic graph
  # ─────────────────────────────────────────────────────────────────

  - id: discover_entities
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'entity:restaurants'
        arc: 'APPLIES_TO'
        direction: 'incoming'
        depth: 1
        project_key: 'qrcode-ai'
    output:
      format: json

  - id: seo_strategy
    use:
      entities: discover_entities
    infer:
      prompt: |
        You are an SEO strategist for QR Code AI (https://qrcode-ai.com).

        Available entities in the restaurant cluster:
        {{use.entities}}

        Define a priority order for SEO page generation.
        For each entity, specify: entity_key, primary_keyword, page_intent (informational/commercial).
        Output as JSON array: [{entity_key, primary_keyword, page_intent}]
      model: claude-opus-4-5-20251101
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # PHASE 2: Parallel SEO generation (entity x locale)
  # ─────────────────────────────────────────────────────────────────

  # French market — highest restaurant QR adoption
  - id: seo_fr_menu
    use:
      strategy: seo_strategy
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: 'menu-restaurant'
        locale: 'fr-FR'
        page_key: 'menu-restaurant'
        seo_mode: true
        target_keyword: 'qr code menu restaurant'
    output:
      format: json

  - id: seo_fr_reviews
    use:
      strategy: seo_strategy
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: 'qr-code-reviews'
        locale: 'fr-FR'
        page_key: 'qr-code-reviews-restaurant'
        seo_mode: true
        target_keyword: 'avis google qr code restaurant'
    output:
      format: json

  # English US market
  - id: seo_en_menu
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: 'menu-restaurant'
        locale: 'en-US'
        page_key: 'menu-restaurant'
        seo_mode: true
        target_keyword: 'qr code restaurant menu'
    output:
      format: json

  - id: seo_en_payment
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: 'qr-code-contactless-payment'
        locale: 'en-US'
        page_key: 'qr-contactless-payment-restaurant'
        seo_mode: true
        target_keyword: 'contactless payment qr restaurant'
    output:
      format: json

  # Spanish Mexico market
  - id: seo_mx_menu
    invoke:
      server: novanet
      tool: novanet_generate
      params:
        entity_key: 'menu-restaurant'
        locale: 'es-MX'
        page_key: 'menu-restaurant'
        seo_mode: true
        target_keyword: 'menú digital qr restaurante'
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # PHASE 3: SEO report
  # ─────────────────────────────────────────────────────────────────

  - id: seo_report
    use:
      fr_menu: seo_fr_menu
      fr_reviews: seo_fr_reviews
      en_menu: seo_en_menu
      en_payment: seo_en_payment
      mx_menu: seo_mx_menu
    infer:
      prompt: |
        Write an SEO sprint completion report for QR Code AI.

        Pages generated:
        - fr-FR / menu-restaurant: {{use.fr_menu.status}}
        - fr-FR / qr-code-reviews: {{use.fr_reviews.status}}
        - en-US / menu-restaurant: {{use.en_menu.status}}
        - en-US / contactless-payment: {{use.en_payment.status}}
        - es-MX / menu-restaurant: {{use.mx_menu.status}}

        Include: estimated organic traffic potential, indexing priority order,
        recommended internal linking structure. Keep it actionable, max 200 words.

flows:
  - source: discover_entities
    target: seo_strategy

  - source: seo_strategy
    target: [seo_fr_menu, seo_fr_reviews, seo_en_menu, seo_en_payment, seo_mx_menu]

  - source: [seo_fr_menu, seo_fr_reviews, seo_en_menu, seo_en_payment, seo_mx_menu]
    target: seo_report
```

### NovaNet Tools Used

| Tool | Called By | Purpose |
|------|-----------|---------|
| `novanet_traverse` | `discover_entities` | Walks `APPLIES_TO` arc from `restaurants` entity inward to find all applicable entities |
| `novanet_generate` | `seo_fr_menu`, `seo_fr_reviews`, `seo_en_menu`, `seo_en_payment`, `seo_mx_menu` | Generates SEO-tuned PageNative + BlockNative per locale |

### Expected Output

5 SEO pages generated in Neo4j (PageNative + BlockNative nodes per locale). A strategic
deployment report including estimated traffic potential and internal linking recommendations.
Replaces 5 hours of manual copy writing per page.

---

## Use Case 3: Entity Knowledge Retrieval

### Name

`entity-knowledge-retrieval`

### User Story

As a product manager building a QR code configurator for the hospitality industry, I want to
retrieve the full semantic knowledge around the "Dynamic QR Code" entity — including related
entities, required dependencies, and applicable industries — so that I can build an accurate
feature matrix without manually querying the knowledge graph.

### Context

- **Entity:** `dynamic-qr-code`
- **Goal:** Understand semantic neighborhood: what it requires, what it enables, where it applies
- **Output:** Structured knowledge map for product spec

### Workflow YAML

```yaml
# examples/entity-knowledge-retrieval.nika.yaml
#
# DAG:
# entity_key → [describe, requires, enables, industries] → knowledge_map
#
schema: 'nika/workflow@0.1'

provider: claude
model: claude-opus-4-5-20251101

mcp:
  novanet:
    transport: stdio
    command: node
    args: ['novanet-dev/tools/novanet-mcp/dist/index.js']

tasks:
  # ─────────────────────────────────────────────────────────────────
  # Input
  # ─────────────────────────────────────────────────────────────────

  - id: entity_key
    exec:
      command: "echo 'dynamic-qr-code'"

  # ─────────────────────────────────────────────────────────────────
  # Parallel graph traversals (fan-out)
  # ─────────────────────────────────────────────────────────────────

  - id: describe
    use:
      key: entity_key
    invoke:
      server: novanet
      tool: novanet_describe
      params:
        entity_key: '{{use.key}}'
        include_llm_context: true
        include_entity_summary: true
    output:
      format: json

  - id: requires
    use:
      key: entity_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'entity:{{use.key}}'
        arc: 'REQUIRES'
        direction: 'outgoing'
        depth: 2
        project_key: 'qrcode-ai'
    output:
      format: json

  - id: enables
    use:
      key: entity_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'entity:{{use.key}}'
        arc: 'ENABLES'
        direction: 'outgoing'
        depth: 1
        project_key: 'qrcode-ai'
    output:
      format: json

  - id: similar_to
    use:
      key: entity_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'entity:{{use.key}}'
        arc: 'SIMILAR_TO'
        direction: 'outgoing'
        depth: 1
        project_key: 'qrcode-ai'
    output:
      format: json

  - id: industries
    use:
      key: entity_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'entity:{{use.key}}'
        arc: 'APPLIES_TO'
        direction: 'outgoing'
        depth: 1
        project_key: 'qrcode-ai'
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # Knowledge map synthesis (fan-in)
  # ─────────────────────────────────────────────────────────────────

  - id: knowledge_map
    use:
      desc: describe
      deps: requires
      unlocks: enables
      alts: similar_to
      sectors: industries
    infer:
      prompt: |
        Build a structured product knowledge map for: {{use.desc.display_name}}

        DESCRIPTION:
        {{use.desc.entity_summary}}

        REQUIRES (dependencies):
        {{use.deps}}

        ENABLES (what it unlocks):
        {{use.unlocks}}

        ALTERNATIVES:
        {{use.alts}}

        APPLICABLE INDUSTRIES:
        {{use.sectors}}

        Produce a structured knowledge map with:
        1. One-line product definition
        2. Dependency chain (what the user needs)
        3. Feature matrix (what users can do with it)
        4. Competitive positioning vs alternatives
        5. Top 3 target industries with use case examples

        Format: clean markdown, suitable for a product spec document.

flows:
  - source: entity_key
    target: [describe, requires, enables, similar_to, industries]

  - source: [describe, requires, enables, similar_to, industries]
    target: knowledge_map
```

### NovaNet Tools Used

| Tool | Called By | Purpose |
|------|-----------|---------|
| `novanet_describe` | `describe` | Returns entity definition, llm_context, entity_summary, display_name |
| `novanet_traverse` | `requires` | Walks `REQUIRES` arcs (depth 2) — finds `short-link` dependency |
| `novanet_traverse` | `enables` | Walks `ENABLES` arcs — finds tracking, retargeting capabilities |
| `novanet_traverse` | `similar_to` | Finds `static-qr-code` as alternative |
| `novanet_traverse` | `industries` | Finds retail, restaurant, e-commerce sectors |

### Expected Output

A structured markdown product knowledge map, e.g.:

```markdown
# Dynamic QR Code — Product Knowledge Map

**Definition:** An editable QR code that encodes a short link, allowing
destination changes after printing with full scan analytics.

## Dependencies
- Short Link (required, strength: 0.95)
- Active subscription (required)

## Feature Matrix
- Destination editing post-print
- Scan tracking (timestamp, location, device)
- Retargeting pixel support

## vs. Static QR Code
- Dynamic: editable, trackable, subscription-based
- Static: fixed, untracked, free forever

## Target Industries
1. Retail — product packaging with changeable offers
2. Restaurants — menu QR that updates daily specials
3. Events — ticket QR with rescheduling capability
```

---

## Use Case 4: Block Generation with Locale Context

### Name

`block-generation-locale-aware`

### User Story

As a content generator building the French landing page for "QR Code Art", I want to generate
a hero block that uses authentic French expressions, respects the fr-FR locale voice (informal,
enthusiastic), and avoids cultural taboos, so that the output feels natively French rather than
translated from English.

### Context

- **Entity:** `qr-code-art`
- **Locale:** `fr-FR`
- **Block type:** `hero` (title, subtitle, CTA)
- **Knowledge atoms needed:** locale Terms, Expressions, Voice config, Taboos

### Workflow YAML

```yaml
# examples/block-generation-locale-aware.nika.yaml
#
# DAG:
# [entity, locale] → [entity_ctx, locale_voice, expressions, taboos] → generate_hero → validate
#
schema: 'nika/workflow@0.1'

provider: claude
model: claude-opus-4-5-20251101

mcp:
  novanet:
    transport: stdio
    command: node
    args: ['novanet-dev/tools/novanet-mcp/dist/index.js']

tasks:
  # ─────────────────────────────────────────────────────────────────
  # Inputs
  # ─────────────────────────────────────────────────────────────────

  - id: entity_key
    exec:
      command: "echo 'qr-code-art'"

  - id: locale_key
    exec:
      command: "echo 'fr-FR'"

  - id: block_type
    exec:
      command: "echo 'hero'"

  # ─────────────────────────────────────────────────────────────────
  # Parallel context loading (fan-out)
  # ─────────────────────────────────────────────────────────────────

  - id: entity_context
    use:
      entity: entity_key
      locale: locale_key
    invoke:
      server: novanet
      tool: novanet_describe
      params:
        entity_key: '{{use.entity}}'
        locale: '{{use.locale}}'
        include_entity_native: true
        include_llm_context: true
    output:
      format: json

  - id: locale_voice
    use:
      locale: locale_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:{{use.locale}}'
        arc: 'HAS_VOICE'
        direction: 'outgoing'
        depth: 1
    output:
      format: json

  - id: locale_expressions
    use:
      locale: locale_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:{{use.locale}}'
        arc: 'HAS_EXPRESSIONS'
        direction: 'outgoing'
        depth: 2
        semantic_field_filter: ['marketing', 'design', 'creativity', 'urgency']
        limit: 15
    output:
      format: json

  - id: locale_taboos
    use:
      locale: locale_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:{{use.locale}}'
        arc: 'HAS_TABOOS'
        direction: 'outgoing'
        depth: 2
    output:
      format: json

  - id: block_instruction
    use:
      entity: entity_key
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'entity:{{use.entity}}'
        arc: 'HAS_PAGE'
        direction: 'outgoing'
        depth: 3
        filter_block_type: 'hero'
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # Context assembly (fan-in)
  # ─────────────────────────────────────────────────────────────────

  - id: generate_hero
    use:
      entity_ctx: entity_context
      voice: locale_voice
      expressions: locale_expressions
      taboos: locale_taboos
      instruction: block_instruction
      locale: locale_key
    infer:
      prompt: |
        Generate a HERO block for QR Code AI in native {{use.locale}}.

        ENTITY CONTEXT:
        - Name: {{use.entity_ctx.display_name}}
        - Summary: {{use.entity_ctx.entity_summary}}
        - LLM Context: {{use.entity_ctx.llm_context}}
        - Native title (reference): {{use.entity_ctx.entity_native.title}}

        LOCALE VOICE ({{use.locale}}):
        {{use.voice}}

        AUTHENTIC EXPRESSIONS TO USE (pick 1-2):
        {{use.expressions}}

        TABOOS TO AVOID:
        {{use.taboos}}

        BLOCK INSTRUCTION:
        {{use.instruction}}

        Generate:
        - title: [GENERATE] — compelling, native {{use.locale}}, uses locale expressions
        - subtitle: [GENERATE] — 1-2 sentences explaining the product value
        - cta_primary: [GENERATE] — action-oriented, 2-4 words max
        - cta_secondary: [GENERATE] — softer alternative CTA

        RULES:
        - Generate natively in {{use.locale}}, do NOT translate from English
        - Brand name "QR Code AI" is [FIXED] — never translate
        - URL qrcode-ai.com is [FIXED]
        - Respect voice formality level exactly
        - Zero taboos from the list above

        Output as JSON: {title, subtitle, cta_primary, cta_secondary}
      model: claude-opus-4-5-20251101
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # Quality validation
  # ─────────────────────────────────────────────────────────────────

  - id: validate_block
    use:
      block: generate_hero
      voice: locale_voice
      taboos: locale_taboos
      locale: locale_key
    infer:
      prompt: |
        Quality check this {{use.locale}} hero block:

        GENERATED CONTENT:
        {{use.block}}

        VOICE REQUIREMENTS:
        {{use.voice}}

        TABOOS:
        {{use.taboos}}

        Check:
        1. Is the title compelling and native {{use.locale}}? (not a translation)
        2. Does it respect voice formality?
        3. Are any taboos violated?
        4. Is "QR Code AI" brand name preserved exactly?
        5. CTA length: is it 2-4 words max?

        Output JSON: {passed: bool, score: 0-10, issues: [string], approved_content: {...}}

    output:
      format: json

flows:
  - source: [entity_key, locale_key]
    target: [entity_context, locale_voice, locale_expressions, locale_taboos]

  - source: entity_key
    target: block_instruction

  - source: [entity_context, locale_voice, locale_expressions, locale_taboos, block_instruction]
    target: generate_hero

  - source: [generate_hero, locale_voice, locale_taboos]
    target: validate_block
```

### NovaNet Tools Used

| Tool | Called By | Purpose |
|------|-----------|---------|
| `novanet_describe` | `entity_context` | Entity definition + EntityNative title in fr-FR |
| `novanet_traverse` | `locale_voice` | `LocaleVoice` node: formality, enthusiasm, humor level |
| `novanet_traverse` | `locale_expressions` | 15 marketing expressions native to fr-FR |
| `novanet_traverse` | `locale_taboos` | Cultural taboos to avoid in French marketing copy |
| `novanet_traverse` | `block_instruction` | `BlockInstruction` YAML for hero block rules |

### Expected Output

A validated fr-FR hero block ready for CMS import:

```json
{
  "passed": true,
  "score": 9,
  "issues": [],
  "approved_content": {
    "title": "Transformez votre QR Code en véritable oeuvre d'art",
    "subtitle": "QR Code AI génère des codes QR artistiques uniques grâce à l'IA — scannables, originaux, inoubliables.",
    "cta_primary": "Créer mon QR art",
    "cta_secondary": "Voir les exemples"
  }
}
```

---

## Use Case 5: Semantic Content Planning

### Name

`semantic-content-planning`

### User Story

As a content strategist at QR Code AI, I want to discover which entity combinations have gaps
in locale coverage — entities that exist in the knowledge graph but have no PageNative generated
for high-priority locales — so that I can prioritize the content roadmap based on real data
rather than intuition.

### Context

- **Goal:** Surface coverage gaps across entity x locale matrix
- **Priority locales:** `fr-FR`, `en-US`, `es-ES`, `de-DE`, `pt-BR`, `ja-JP`
- **Scope:** All pillar entities (`is_pillar: true`) in qrcode-ai project

### Workflow YAML

```yaml
# examples/semantic-content-planning.nika.yaml
#
# DAG:
# PHASE 1: Discover pillars + check coverage per locale
# PHASE 2: Analyze gaps → prioritize → content plan
#
schema: 'nika/workflow@0.1'

provider: claude
model: claude-opus-4-5-20251101

mcp:
  novanet:
    transport: stdio
    command: node
    args: ['novanet-dev/tools/novanet-mcp/dist/index.js']

tasks:
  # ─────────────────────────────────────────────────────────────────
  # PHASE 1: Discovery
  # ─────────────────────────────────────────────────────────────────

  - id: discover_pillars
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'project:qrcode-ai'
        arc: 'HAS_ENTITY'
        direction: 'outgoing'
        filter_properties:
          is_pillar: true
        depth: 1
    output:
      format: json

  # Check coverage for each priority locale (parallel fan-out)

  - id: coverage_fr
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:fr-FR'
        arc: 'FOR_LOCALE'
        direction: 'incoming'
        filter_node_class: 'PageNative'
        project_key: 'qrcode-ai'
        depth: 1
    output:
      format: json

  - id: coverage_en
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:en-US'
        arc: 'FOR_LOCALE'
        direction: 'incoming'
        filter_node_class: 'PageNative'
        project_key: 'qrcode-ai'
        depth: 1
    output:
      format: json

  - id: coverage_es
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:es-ES'
        arc: 'FOR_LOCALE'
        direction: 'incoming'
        filter_node_class: 'PageNative'
        project_key: 'qrcode-ai'
        depth: 1
    output:
      format: json

  - id: coverage_de
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:de-DE'
        arc: 'FOR_LOCALE'
        direction: 'incoming'
        filter_node_class: 'PageNative'
        project_key: 'qrcode-ai'
        depth: 1
    output:
      format: json

  - id: coverage_pt
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:pt-BR'
        arc: 'FOR_LOCALE'
        direction: 'incoming'
        filter_node_class: 'PageNative'
        project_key: 'qrcode-ai'
        depth: 1
    output:
      format: json

  - id: coverage_ja
    invoke:
      server: novanet
      tool: novanet_traverse
      params:
        start: 'locale:ja-JP'
        arc: 'FOR_LOCALE'
        direction: 'incoming'
        filter_node_class: 'PageNative'
        project_key: 'qrcode-ai'
        depth: 1
    output:
      format: json

  # ─────────────────────────────────────────────────────────────────
  # PHASE 2: Gap analysis (fan-in)
  # ─────────────────────────────────────────────────────────────────

  - id: gap_analysis
    use:
      pillars: discover_pillars
      fr: coverage_fr
      en: coverage_en
      es: coverage_es
      de: coverage_de
      pt: coverage_pt
      ja: coverage_ja
    infer:
      prompt: |
        Analyze content coverage gaps for QR Code AI.

        PILLAR ENTITIES (must have pages):
        {{use.pillars}}

        EXISTING PAGES PER LOCALE:
        - fr-FR: {{use.fr}}
        - en-US: {{use.en}}
        - es-ES: {{use.es}}
        - de-DE: {{use.de}}
        - pt-BR: {{use.pt}}
        - ja-JP: {{use.ja}}

        TASK:
        1. Build a coverage matrix (entity x locale) — mark each cell: EXISTS / MISSING
        2. Identify the top 5 highest-priority gaps (consider: locale traffic weight, entity pillar status)
        3. Estimate generation effort per gap (blocks x locales)
        4. Output a prioritized content sprint plan

        Format: markdown table for coverage matrix + numbered priority list.
      model: claude-opus-4-5-20251101
    output:
      format: json

  - id: sprint_plan
    use:
      gaps: gap_analysis
    infer:
      prompt: |
        Convert this gap analysis into an actionable Nika workflow plan.

        GAP ANALYSIS:
        {{use.gaps}}

        For the top 5 priority gaps, generate:
        1. A Nika workflow YAML snippet (invoke: novanet_generate) for each gap
        2. Estimated runtime per gap (assume 30s per block)
        3. Total estimated content generation time

        Output a ready-to-execute sprint plan with copy-paste YAML snippets.

flows:
  - source: discover_pillars
    target: [coverage_fr, coverage_en, coverage_es, coverage_de, coverage_pt, coverage_ja]

  - source: [discover_pillars, coverage_fr, coverage_en, coverage_es, coverage_de, coverage_pt, coverage_ja]
    target: gap_analysis

  - source: gap_analysis
    target: sprint_plan
```

### NovaNet Tools Used

| Tool | Called By | Purpose |
|------|-----------|---------|
| `novanet_traverse` | `discover_pillars` | Finds all `is_pillar: true` entities via `HAS_ENTITY` arc from project |
| `novanet_traverse` | `coverage_fr`, `coverage_en`, `coverage_es`, `coverage_de`, `coverage_pt`, `coverage_ja` | Traverses `FOR_LOCALE` incoming arcs to count existing PageNative nodes per locale |

### Expected Output

A coverage matrix and prioritized content plan, e.g.:

```markdown
## Coverage Matrix — Pillar Entities x Priority Locales

| Entity         | fr-FR  | en-US  | es-ES  | de-DE  | pt-BR  | ja-JP  |
|----------------|--------|--------|--------|--------|--------|--------|
| qr-code        | EXISTS | EXISTS | MISSING| EXISTS | MISSING| MISSING|
| smart-link     | EXISTS | EXISTS | MISSING| MISSING| MISSING| MISSING|
| barcode        | MISSING| EXISTS | MISSING| MISSING| MISSING| MISSING|
| landing-page   | EXISTS | EXISTS | MISSING| MISSING| MISSING| MISSING|

## Top 5 Priority Gaps

1. qr-code / es-ES — Pillar entity, 500M+ ES speakers, estimated traffic: HIGH
2. qr-code / ja-JP — Pillar entity, #3 QR market globally
3. smart-link / de-DE — High commercial intent market
4. barcode / fr-FR — Pillar entity, no FR coverage
5. landing-page / es-ES — Commercial pillar, untapped ES market

## Estimated Sprint

Total gaps: 14 pages x ~8 blocks = 112 BlockNative nodes
Estimated generation time: ~56 minutes (parallel, 5 workers)
Recommended: run as 2 Nika workflows x 7 pages each
```

---

## Cross-Cutting Patterns

### Pattern 1: Context-First Loading

Every use case loads context via NovaNet before generating content:

```
invoke: novanet_describe  →  entity definition + LLM context
invoke: novanet_traverse  →  semantic neighborhood (arcs, depth)
  └── infer: ...          →  generate with loaded context
```

This maps directly to NovaNet's knowledge atom architecture:
load only relevant Terms/Expressions, not full 20K JSON blobs.

### Pattern 2: Fan-Out for Parallelism

Multi-locale generation always uses parallel fan-out in Nika's DAG:

```
entity_key → [gen_fr, gen_es, gen_de, gen_ja, gen_pt] → report
```

Nika's DAG executor runs independent tasks concurrently — 5 locales
in the time of 1 when the MCP server can handle parallel requests.

### Pattern 3: Validate After Generate

Use cases 4 and 5 add a validation step after generation:

```
generate_hero → validate_block
gap_analysis  → sprint_plan
```

This implements a "Generate + Critique" loop without a full `agent:` verb,
keeping the workflow simple for MVP 1 while still checking quality.

### Pattern 4: Semantic Arc Walking

All traversals use semantic arc names, never raw Cypher in Nika:

```yaml
# RIGHT (Nika workflow)
invoke: novanet_traverse
params:
  arc: 'APPLIES_TO'
  direction: 'incoming'

# WRONG — Architecture Rule 4 violation
exec:
  command: "cypher MATCH (e)-[:APPLIES_TO]->(i:Industry) RETURN e"
```

---

## Implementation Notes

### MCP Config Block

All workflows require the `mcp:` config block pointing to the NovaNet MCP server.
Once MVP 1 (invoke: verb) is implemented in Nika, this block will be parsed
from the workflow YAML and used to establish the MCP connection.

```yaml
mcp:
  novanet:
    transport: stdio
    command: node
    args: ['novanet-dev/tools/novanet-mcp/dist/index.js']
```

### Workflow Maturity by MVP

| Use Case | Requires MVP | Key Feature |
|----------|-------------|-------------|
| UC1: Multi-Locale | MVP 1 | `invoke:` verb |
| UC2: SEO Sprint | MVP 1 | `invoke:` + multi-step |
| UC3: Knowledge Retrieval | MVP 1 | `invoke:` + parallel |
| UC4: Locale-Aware Block | MVP 1 + MVP 2 | `invoke:` + validation loop |
| UC5: Content Planning | MVP 1 + MVP 6 | `invoke:` + `for_each:` (parallel) |

### File Locations

```
nika-dev/examples/
├── generate-page-multilingual.nika.yaml    # UC1
├── seo-content-sprint.nika.yaml            # UC2
├── entity-knowledge-retrieval.nika.yaml    # UC3
├── block-generation-locale-aware.nika.yaml # UC4
└── semantic-content-planning.nika.yaml     # UC5
```

These files should be created as part of MVP 6 deliverables
("examples/ with 5+ documented workflows").

---

## Summary Table

| Use Case | Nika Verbs | NovaNet Tools | Locales | Parallelism | Est. Runtime |
|----------|-----------|--------------|---------|-------------|-------------|
| UC1: Multi-Locale Page | invoke, infer | generate x5 | 5 | High | ~2 min |
| UC2: SEO Sprint | invoke, infer | traverse, generate x5 | 3 | High | ~3 min |
| UC3: Knowledge Retrieval | invoke, infer | describe, traverse x4 | 1 | High | ~30 sec |
| UC4: Locale-Aware Block | exec, invoke, infer | describe, traverse x4 | 1 | Medium | ~1 min |
| UC5: Content Planning | invoke, infer | traverse x7 | 6 | High | ~1 min |
