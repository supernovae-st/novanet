# NovaNet — Specification & Architecture

**Version**: v0.13.1 | **Target**: [QR Code AI](https://qrcode-ai.com) | **Status**: Pre-production (0.x)

---

## Executive Summary

NovaNet is a **native content generation engine** powered by Neo4j knowledge graphs. It generates culturally-authentic content across 200+ locales — not through translation, but through native generation from semantic concepts.

```
Source -> Translate -> Target                           WRONG (traditional)
Entity (defined) -> Generate natively -> EntityNative   RIGHT (NovaNet)
```

---

## Problem Statement

| Traditional Approach | Cost |
|---------------------|------|
| Translate to 200 locales | 200x translation cost |
| Translation loses cultural nuance | Idioms, humor, formality lost |
| Maintain translation databases | Synchronization nightmare |
| SEO keywords differ by country | Manual keyword research per locale |

**NovaNet Solution**: Define semantic entities once, generate native content 200x with cultural context.

---

## Core Philosophy (ADR-007)

> **Generation, NOT Translation**

Content is generated natively per locale from defined semantic entities. Each locale gets culturally-native content, preserving local nuances that translation would lose.

---

## Architecture Overview

### Two-Realm Model (ADR-012)

```
+===============================================================================+
|  NOVANET SCHEMA v0.13.1                                                       |
+===============================================================================+
|                                                                               |
|  SHARED REALM (40 nodes) — Universal knowledge, READ-ONLY                     |
|  ├── config/      3 nodes: Locale, EntityCategory, SEOKeywordFormat           |
|  ├── locale/      6 nodes: Culture, Style, Formatting, Slugification...       |
|  ├── geography/   7 nodes: Continent, GeoRegion, Country...                   |
|  └── knowledge/  24 nodes: Terms, Expressions, SEO, GEO, taxonomies           |
|                                                                               |
|  ORG REALM (21 nodes) — Organization-specific content                         |
|  ├── config/      1 node:  OrgConfig                                          |
|  ├── foundation/  6 nodes: Project, Brand, BrandDesign, PromptStyle...        |
|  ├── structure/   3 nodes: Page, Block, ContentSlot                           |
|  ├── semantic/    4 nodes: Entity, EntityNative, AudiencePersona...           |
|  ├── instruction/ 4 nodes: BlockType, BlockRules, BlockInstruction...         |
|  └── output/      3 nodes: PageNative, BlockNative, OutputArtifact            |
|                                                                               |
|  TOTAL: 61 nodes, 182 arcs, 10 layers, 6 arc families, 5 traits               |
+===============================================================================+
```

### Traits — Data Origin (ADR-024)

| Trait | Who Creates | Examples |
|-------|-------------|----------|
| `defined` | Human, ONCE | Page, Block, Entity, Locale |
| `authored` | Human, PER locale | EntityNative, ProjectNative |
| `imported` | External data | Term, SEOKeyword, GEOQuery |
| `generated` | Our LLM | PageNative, BlockNative |
| `retrieved` | External APIs | GEOAnswer, SEOKeywordMetrics |

### Arc Families (182 arcs)

| Family | Count | Purpose | Examples |
|--------|-------|---------|----------|
| `ownership` | 54 | Structural containment | HAS_PAGE, HAS_BLOCK, CONTAINS_* |
| `localization` | 16 | Locale linking | HAS_NATIVE, FOR_LOCALE |
| `semantic` | 44 | Meaning connections | SEMANTIC_LINK, USES_ENTITY |
| `generation` | 11 | AI output pipeline | GENERATED, ASSEMBLES |
| `mining` | 1 | Analytics | HAS_METRICS |
| `schema` | 56 | Meta-graph | HAS_CLASS, BELONGS_TO_LAYER |

---

## Content Generation Pipeline

### Flow

```
1. DEFINITION (human, once)
   Entity:qr-code-generator
      └── key, description, category, semantic_links

2. AUTHORING (human, per locale)
   EntityNative:qr-code-generator@fr-FR
      └── title: "Générateur de QR Code"
      └── tagline: "Créez des QR codes personnalisés"
      └── denomination_forms: [text, title, abbrev, url]

3. INSTRUCTION (human)
   BlockInstruction (markdown with @ refs)
      └── "Generate hero section using @entity:qr-code-generator"
      └── "Include CTA linking to [@page:pricing]"

4. GENERATION (LLM)
   BlockNative:homepage:hero:1@fr-FR
      └── title: "Le meilleur générateur de QR Code gratuit"
      └── body: "Créez des QR codes personnalisés..."

5. OUTPUT
   PageNative:homepage@fr-FR
      └── Assemblage de tous les BlockNative
      └── slug: "generateur-qr-code" (from SEOKeyword)
```

### @ Reference System (ADR-028)

**Injection** (LLM context, no HTML):
```
@entity:X           → Inject EntityNative(X@locale)
@entity:X.field     → Inject specific field
@brand              → Inject Brand (soul, pitch, voice)
@term:X             → Inject Term(X@locale)
@seo:X              → Inject SEOKeyword
```

**Links** (creates HTML):
```
[@page:X]           → <a href="/X">{page.title}</a>
[@page:X|"text"]    → <a href="/X">text</a>
```

---

## Key Architectural Decisions

### *Native Pattern (ADR-029)

All locale-specific nodes use `*Native` suffix. Traits distinguish authorship.

| Node | Trait | Who Creates |
|------|-------|-------------|
| EntityNative | authored | Human writes natively |
| ProjectNative | authored | Human writes natively |
| PageNative | generated | LLM generates natively |
| BlockNative | generated | LLM generates natively |

### Slug Ownership (ADR-030)

```
Entity = QUOI (semantic concept, invariant)
Page   = OÙ   (URL structure, navigation)

Entity.key  = "qr-code-instagram"  (semantic identity)
Page.slug   = "instagram"          (URL segment only)

Result: /qr-code-generator/instagram  (no repetition)
```

### Denomination Forms (ADR-033)

Prescriptive canonical forms for LLM entity references:

```yaml
EntityNative:qr-code@es-MX:
  denomination_forms:
    - { type: text,   value: "código qr" }       # prose
    - { type: title,  value: "Código QR" }       # headings
    - { type: abbrev, value: "qr" }              # short form
    - { type: url,    value: "crear-codigo-qr" } # post-SEO pipeline
```

**Rule**: LLM MUST use ONLY denomination_forms values. No invention, no paraphrase.

### Page-Entity Architecture (ADR-028)

```
Page ──[:REPRESENTS]──> Entity (1:1 OBLIGATOIRE)

Every Page MUST have exactly one Entity:
- /qr-code-generator → Entity:qr-code-generator
- /pricing → Entity:pricing
- /contact → Entity:contact (utility pages too)
```

---

## Knowledge Atoms

Granular knowledge nodes for selective LLM context loading.

```
Locale:fr-FR
    │
    ├──[:HAS_TERMS]──> TermSet (pricing)
    │                      └──[:CONTAINS_TERM]──> Term (50 terms)
    │
    ├──[:HAS_EXPRESSIONS]──> ExpressionSet (call-to-action)
    │                            └──[:CONTAINS_EXPRESSION]──> Expression
    │
    └──[:HAS_CULTURE]──> CultureSet
                             └──[:CONTAINS_CULTURE_REF]──> CultureRef

Benefit: Load 50 relevant Terms, not 20K JSON blob
Query: "Terms used by this Block" via [:USES_TERM] arcs
```

### Container Types (8)

TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet, SEOKeywordSet, GEOQuerySet

### Atom Types (6)

Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait

---

## SEO/GEO Architecture

### Pillar/Cluster Strategy (ADR-031)

Three distinct hierarchies:

```
1. Entity.SUBTOPIC_OF  = SEMANTIC hierarchy (topic clusters)
2. Page.SUBTOPIC_OF    = URL hierarchy (routing, navigation)
3. Page.SEO_CLUSTER_OF = SEO hierarchy (pillar/cluster strategy)
```

### URL Slugification (ADR-032)

**No-Repetition Rule**:
```
BAD:  /créer-qr-code/qr-code-pour-instagram  (repetition!)
GOOD: /créer-qr-code/instagram               (differentiator only)
```

**Locale-specific rules**:

| Rule | Locales | Example |
|------|---------|---------|
| latin_preserve | fr-FR, es-MX | código-qr (accents kept) |
| latin_strip | en-US, en-GB | qr-code (diacritics removed) |
| latin_transform | de-DE | fuer (ü→ue) |
| native_script | ja-JP, zh-CN | romanized for URL |

---

## Tech Stack

### Monorepo Structure

```
novanet-hq/
├── packages/
│   ├── core/                  @novanet/core
│   │   └── models/            YAML source of truth
│   │       ├── taxonomy.yaml
│   │       ├── node-classes/{realm}/{layer}/*.yaml
│   │       └── arc-classes/{family}/*.yaml
│   └── db/                    @novanet/db
│       └── seed/*.cypher      Generated from YAML
├── apps/
│   └── studio/                @novanet/studio (Next.js 16)
└── tools/
    └── novanet/               Rust CLI + TUI (1139 tests)
```

### Dependencies

| Component | Technology |
|-----------|------------|
| Graph DB | Neo4j 5.26 + APOC |
| Frontend | Next.js 16, React 19, Tailwind |
| CLI/TUI | Rust (neo4rs, ratatui, clap) |
| Schema | YAML → TypeScript/Cypher/Mermaid |
| AI | Claude API (generation + chat) |
| Build | Turborepo, pnpm |

### Rust CLI Commands

```bash
# Schema operations
cargo run -- schema generate        # YAML → artifacts
cargo run -- schema validate        # Coherence check

# Navigation
cargo run -- blueprint              # Schema-graph ASCII
cargo run -- tui                    # Interactive TUI

# CRUD
cargo run -- node create --class=Page --key=my-page
cargo run -- search --query="page" --class=Page

# Database
cargo run -- db seed                # Execute seed files
```

---

## Visual Encoding (ADR-005, ADR-013)

### Color Channels

| Channel | Encodes | Example |
|---------|---------|---------|
| Fill color | Layer | semantic=orange, output=green |
| Border color | Realm | shared=teal, org=sky |
| Border style | Trait | solid=defined, dashed=authored |
| Arc stroke | Family | ownership=solid, semantic=dashed |

### Dual Icons

```yaml
icons:
  realms:
    shared: { web: "globe", terminal: "◉" }
    org: { web: "building", terminal: "◆" }
```

---

## Key Statistics

| Metric | Value |
|--------|-------|
| Node Types | 61 (40 shared + 21 org) |
| Arc Types | 182 (6 families) |
| Realms | 2 (shared, org) |
| Layers | 10 (4 shared + 6 org) |
| Traits | 5 (defined, authored, imported, generated, retrieved) |
| Rust Tests | 1139 passing |
| Target Locales | 200+ |

---

## ADR Index

| ADR | Title | Status |
|-----|-------|--------|
| 001 | Arc Terminology | stable |
| 003 | YAML-First Architecture | stable |
| 007 | Generation, Not Translation | core |
| 012 | 2-Realm Architecture | stable |
| 021 | Query-First Architecture | active |
| 024 | Trait = Data Origin | active |
| 026 | Inverse Arc Policy | active |
| 028 | Page-Entity Architecture | active |
| 029 | *Native Pattern | active |
| 030 | Slug Ownership | active |
| 033 | Denomination Forms | active |

Full ADRs: `.claude/rules/adr/` in novanet-hq repository.

---

## Why This Architecture?

1. **Scalability**: Define 1x, generate 200x — no linear cost
2. **Cultural Quality**: Native content, not translation — preserves nuances
3. **SEO/GEO Optimized**: Keywords and culture per locale in graph
4. **LLM-Ready**: Knowledge atoms for selective context loading
5. **Single Source of Truth**: YAML generates everything
6. **Auditability**: Every arc traced, every generation from instructions

---

## Getting Started

```bash
# Clone
git clone git@github.com:supernovae-st/novanet-hq.git
cd novanet-hq

# Install
pnpm install

# Start Neo4j + seed
pnpm infra:up && pnpm infra:seed

# Development
pnpm dev          # Studio at http://localhost:3000

# Rust CLI
cd tools/novanet
cargo run -- tui  # Interactive exploration
```

---

*NovaNet v0.13.1 — SuperNovae Studio*
