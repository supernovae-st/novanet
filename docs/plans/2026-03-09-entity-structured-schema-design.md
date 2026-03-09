# Entity/EntityNative Structured Schema Design

**Date**: 2026-03-09
**Status**: Draft - Pending Validation
**Author**: Thibaut + Claude

## Overview

This document defines a new structured approach for Entity and EntityNative schemas:

1. **Rename**: `description` → `content` (object)
2. **Restructure**: `llm_context` from string to object
3. **Inheritance**: BASE schema + NodeClass-specific extensions

## Philosophy

### Core Principles

1. **Project-Centered, Not Encyclopedia**
   - Entity content is about QR Code AI, not universal Wikipedia definitions
   - Focus on OUR features, OUR context, OUR platform

2. **Anti-Hallucination**
   - Only include verified information
   - No unverified specs, dates, or numbers
   - Structure > prose (LLM parses objects better than free text)

3. **Graph = Source of Truth for Relations**
   - Relationships between entities = graph arcs
   - Categorization = graph structure (EntityCategory nodes)
   - NO duplication of relations inside entity data

4. **BASE + EXTENSIONS Model**
   - All nodes share a BASE schema for `content` and `llm_context`
   - Each NodeClass can EXTEND with additional fields
   - Ensures consistency while allowing flexibility

## Schema Changes Summary

| Field | Before | After |
|-------|--------|-------|
| `description` | `string` | RENAMED to `content: object` |
| `llm_context` | `string` (ADR-027 pattern) | `object` with structured fields |

## standard_properties (All Nodes)

```yaml
standard_properties:
  key: string              # unchanged
  display_name: string     # unchanged
  content: object          # NEW (was description: string)
  llm_context: object      # CHANGED (was string)
  created_at: datetime     # unchanged
  updated_at: datetime     # unchanged
```

## content Schema (BASE + EXTENSIONS)

### BASE (All Nodes)

Every node MUST have these fields in `content`:

```yaml
content:
  definition: string    # required - what it is
  context: string       # required - role/usage in the system
```

### ENTITY extends BASE

```yaml
content:
  # ─── BASE (inherited) ───
  definition: string        # required
  context: string           # required

  # ─── ENTITY EXTENSIONS (by category) ───
  features: string[]        # pillar/type - platform features related
  technical: string[]       # optional - verified technical specs
  steps: string[]           # action - workflow steps
  inputs: string[]          # action - required inputs
  outputs: string[]         # action - what it produces
  capabilities: string[]    # feature - specific capabilities
```

### ENTITY_NATIVE extends ENTITY

```yaml
content:
  # ─── BASE (inherited, localized) ───
  definition: string        # localized
  context: string           # localized

  # ─── ENTITY EXTENSIONS (localized) ───
  features: string[]        # localized

  # ─── ENTITY_NATIVE EXTENSION ───
  cultural_context: string  # NEW - locale-specific cultural notes
```

### PAGE extends BASE

```yaml
content:
  # ─── BASE (inherited) ───
  definition: string        # required
  context: string           # required

  # ─── PAGE EXTENSIONS ───
  purpose: string           # page goal/intent
  structure: string[]       # sections/blocks overview
```

### LOCALE (BASE only)

Simple nodes use BASE without extensions:

```yaml
content:
  definition: string    # "French language as spoken in France"
  context: string       # "For content targeting French audience in France"
```

## llm_context Schema (BASE + EXTENSIONS)

### BASE (All Nodes)

```yaml
llm_context:
  use: string           # required - when to use this concept
  triggers: string[]    # required - keywords that trigger this
```

### ENTITY extends BASE

```yaml
llm_context:
  # ─── BASE (inherited) ───
  use: string           # required
  triggers: string[]    # required

  # ─── ENTITY EXTENSION ───
  not_for: string[]     # optional - explicit exclusions
```

## Entity Categories

Each Entity has a `category` that determines which `content` extensions are used:

| Category | Extensions Used | Examples |
|----------|-----------------|----------|
| `pillar` | features, technical | qr-code, smart-link |
| `type` | features, technical, data_format | qr-code-wifi, qr-code-vcard |
| `action` | steps, inputs, outputs | create-qr-code, scan-qr-code |
| `feature` | capabilities | analytics, customization |

## denomination_forms

Unchanged from current schema.

| Type | Entity | EntityNative | Description |
|------|--------|--------------|-------------|
| text | Yes | Yes | Prose, body content |
| title | Yes | Yes | H1, H2, meta_title |
| abbrev | Yes | Yes | Short form after first mention |
| url | No | Yes | URL-safe slug (post-SEO pipeline) |
| mixed | No | Yes | Native script hybrid (e.g., "QR码") |
| base | No | Yes | International reference in native script locales |

## Complete Example: qr-code (Pillar)

### Entity (Invariant, EN)

```yaml
entity:
  # ─── IDENTIFIERS ───
  key: entity:qr-code
  display_name: QR Code
  category: pillar
  is_pillar: true
  audience_segment: null  # universal

  # ─── CONTENT (object, BASE + pillar extensions) ───
  content:
    # BASE
    definition: >
      Two-dimensional barcode encoding data in a scannable visual pattern.
      Readable by smartphone cameras without specialized apps.
      Stores various data types including URLs, text, and structured information.
    context: >
      Pillar entity of QR Code AI. All platform features connect to this
      core concept. Foundation for creation, design, tracking, and analysis.

    # PILLAR EXTENSIONS
    features:
      - "AI Art: generate artistic QR codes that remain scannable"
      - "Dynamic QR: URLs updateable after printing, no reprint needed"
      - "Analytics: scan tracking, geographic location, device stats"
      - "Customization: colors, logos, frames, shapes, eye patterns"
      - "Export: PNG, SVG, PDF, EPS for print and digital use"
      - "Bulk Generation: create multiple QR codes from data files"
      - "API Access: programmatic QR code generation and management"
    technical:
      - "Data types: URLs, plain text, WiFi credentials, vCard, email, SMS"
      - "Error correction: allows partial damage recovery while scannable"
      - "Static vs Dynamic: static encodes data directly, dynamic uses redirect"

  # ─── LLM_CONTEXT (object, BASE + entity extension) ───
  llm_context:
    # BASE
    use: >
      When discussing 2D barcodes, mobile scanning, or scannable codes
      on QR Code AI. Core concept for all content.
    triggers:
      - "qr"
      - "qr code"
      - "qrcode"
      - "scan"
      - "scannable"
      - "2d barcode"
      - "matrix code"

    # ENTITY EXTENSION
    not_for:
      - "1D barcodes / linear barcodes (use Barcode entity if exists)"
      - "NFC tags or contactless chips (use NFC entity if exists)"
      - "RFID technology (different technology)"

  # ─── DENOMINATION_FORMS ───
  denomination_forms:
    - type: text
      value: "qr code"
    - type: title
      value: "QR Code"
    - type: abbrev
      value: "QR"

  # ─── SEO/SCHEMA ───
  schema_org_type: Product
```

### EntityNative (fr-FR)

```yaml
entity_native:
  # ─── IDENTIFIERS ───
  key: entity:qr-code@fr-FR
  entity_key: qr-code
  locale: fr-FR
  display_name: QR Code

  # ─── CONTENT (object, localized + cultural_context) ───
  content:
    # BASE (localized)
    definition: >
      Code-barres bidimensionnel encodant des données dans un motif visuel
      scannable. Lisible par les smartphones sans application spécialisée.
    context: >
      Entité pilier de QR Code AI. Toutes les fonctionnalités de la
      plateforme se connectent à ce concept central.

    # PILLAR EXTENSIONS (localized)
    features:
      - "Art IA: générer des QR codes artistiques qui restent scannables"
      - "QR Dynamique: URLs modifiables après impression"
      - "Analytique: suivi des scans, localisation, statistiques appareils"
      - "Personnalisation: couleurs, logos, cadres, formes"
      - "Export: PNG, SVG, PDF, EPS"

    # ENTITY_NATIVE EXTENSION
    cultural_context: >
      Les QR codes sont très répandus en France depuis le COVID-19
      (menus restaurants, pass sanitaire). On utilise "scanner" ou "flasher"
      pour l'action de lecture. Le terme "QR code" est utilisé tel quel.

  # ─── LLM_CONTEXT (object, localized) ───
  llm_context:
    # BASE (localized)
    use: >
      Quand on parle de codes-barres 2D, scan mobile, ou codes scannables
      sur QR Code AI. Concept central de tout le contenu.
    triggers:
      - "qr"
      - "qr code"
      - "code qr"
      - "scanner"
      - "flasher"
      - "scannable"

    # ENTITY EXTENSION (localized)
    not_for:
      - "Codes-barres 1D (utiliser entité Code-barres si existe)"
      - "Tags NFC (utiliser entité NFC si existe)"

  # ─── DENOMINATION_FORMS ───
  denomination_forms:
    - type: text
      value: "qr code"
      priority: 1
    - type: title
      value: "QR Code"
      priority: 1
    - type: abbrev
      value: "QR"
      priority: 1
    - type: url
      value: "qr-code"
      priority: 1

  # ─── METADATA ───
  version: 1
  status: draft
```

## Graph Relationships

Relations are NOT stored in entity data. They are graph arcs:

```cypher
// Type hierarchy
(entity:qr-code)-[:HAS_TYPE]->(entity:qr-code-wifi)
(entity:qr-code)-[:HAS_TYPE]->(entity:qr-code-vcard)

// Actions
(entity:qr-code)-[:HAS_ACTION]->(entity:create-qr-code)
(entity:qr-code)-[:HAS_ACTION]->(entity:scan-qr-code)

// Variants
(entity:qr-code)-[:HAS_VARIANT]->(entity:ai-qr-code)

// Features
(entity:qr-code)-[:HAS_FEATURE]->(entity:analytics)

// Categories
(entity:qr-code)-[:IN_CATEGORY]->(category:product)

// Native content
(entity:qr-code)-[:HAS_NATIVE]->(entity-native:qr-code@fr-FR)
(entity-native:qr-code@fr-FR)-[:FOR_LOCALE]->(locale:fr-FR)
```

## Migration Plan

1. **Phase 1: Validate with Data Files** (current)
   - Create sample Entity/EntityNative YAML files
   - Test with real content (qr-code, qr-code@fr-FR)
   - Validate structure works for different categories

2. **Phase 2: Update NovaNet Schema**
   - Modify `entity.yaml` and `entity-native.yaml` in packages/core/models
   - Rename `description` → `content`
   - Change type from `string` to `object`
   - Update TypeScript types
   - Run schema generation

3. **Phase 3: Update Database**
   - Create migration Cypher scripts
   - Transform existing `description` string → `content.definition`
   - Parse existing `llm_context` string → object fields
   - Validate with novanet_audit

4. **Phase 4: Update MCP Tools**
   - Update novanet_generate to use new structure
   - Update novanet_check validation for object types
   - Update documentation

## Validation Checklist

Before proceeding to NovaNet schema changes:

- [ ] qr-code.yaml Entity file created and validated
- [ ] qr-code@fr-FR.yaml EntityNative file created and validated
- [ ] At least one TYPE entity tested (e.g., qr-code-wifi)
- [ ] At least one ACTION entity tested (e.g., create-qr-code)
- [ ] Structure reviewed for LLM clarity
- [ ] Anti-hallucination rules followed (no unverified specs)

---

**Files Created**:
- `novanet/docs/plans/2026-03-09-entity-structured-schema-design.md` (this file)
- `/Users/thibaut/Projects/traduction_ai/ath-know-qrcai/_docs/sitemap-structure/entity/qr-code.yaml`
- `/Users/thibaut/Projects/traduction_ai/ath-know-qrcai/_docs/sitemap-structure/entity/entity-native/qr-code@fr-FR.yaml`
