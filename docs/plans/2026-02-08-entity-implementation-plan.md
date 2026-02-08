# QR Code AI — Entity Implementation Plan

**Date**: 2026-02-08
**Status**: Ready for Execution
**Depends On**: v10.9 Schema (✅ Complete)
**Target**: 279 entities + ~825 semantic arcs

---

## Executive Summary

This plan implements the Entity data layer for QR Code AI based on the design document
`2026-02-07-qrcode-ai-entities-design.md`. The schema (v10.9) is complete with typed
semantic arcs and GEO layer. Now we implement the actual data.

---

## Architecture Decision: Data File Structure

### Option A: Single YAML per Entity (Rejected)
```
packages/core/data/entities/qrcode-ai/
├── thing/
│   ├── qr-code.yaml
│   ├── custom-qr-code.yaml
│   └── ...
└── content-type/
    ├── url-qr-code.yaml
    └── ...
```
**Con**: 279 files = maintenance nightmare

### Option B: Phase-Based YAML Files (Selected) ✅
```
packages/core/data/entities/qrcode-ai/
├── phase-01-core-products.yaml      # 39 entities
├── phase-02-content-types.yaml      # 58 entities
├── phase-03-barcode-types.yaml      # 18 entities
├── phase-04-features-tools.yaml     # 25 entities
├── phase-05-mediums.yaml            # 20 entities
├── phase-06-actions.yaml            # 15 entities
├── phase-07-industries.yaml         # 25 entities
├── phase-08-brands.yaml             # 25 entities
├── phase-09-integrations.yaml       # 12 entities
├── phase-10-concepts.yaml           # 8 entities
├── phase-11-social-subcategories.yaml # 4 entities
├── phase-12-use-cases.yaml          # 12 entities
├── phase-13-guides.yaml             # 10 entities
└── phase-14-comparisons.yaml        # 8 entities
```
**Pro**: Logical grouping, easy to review, matches design phases

---

## YAML Data Format

```yaml
# phase-01-core-products.yaml
project: qrcode-ai
phase: 1
name: Core Products
description: Pillars, categories, styles, frames

entities:
  # ═══════════════════════════════════════════════════════════════
  # PILLARS (4)
  # ═══════════════════════════════════════════════════════════════
  - key: qr-code
    type: THING
    is_pillar: true
    display_name: QR Code
    description: 2D matrix barcode for encoding data
    entity_summary: |
      A QR Code (Quick Response Code) is a two-dimensional barcode that stores
      information in a machine-readable format. QR codes can encode URLs, text,
      contact information, WiFi credentials, and more.

  - key: smart-link
    type: THING
    is_pillar: true
    display_name: Smart Link
    description: Intelligent shortened URL with routing rules
    entity_summary: |
      A Smart Link is an intelligent URL shortener that routes users based on
      device, location, time, or other contextual factors. Used for A/B testing,
      geo-targeting, and analytics.

arcs:
  # ═══════════════════════════════════════════════════════════════
  # PILLAR RELATIONSHIPS
  # ═══════════════════════════════════════════════════════════════
  - from: qr-code
    to: qr-code-style
    type: INCLUDES
    strength: 0.70

  - from: qr-code
    to: qr-code-content
    type: INCLUDES
    strength: 0.70

  - from: qr-code
    to: barcode
    type: SIMILAR_TO
    strength: 0.40

  - from: smart-link
    to: short-link
    type: TYPE_OF
    strength: 0.90

  - from: dynamic-qr-code
    to: short-link
    type: REQUIRES
    strength: 0.95
```

---

## Execution Phases

### Phase 0: Setup (Prerequisite)
```
[ ] Create directory structure
[ ] Create Project node for QR Code AI in Neo4j
[ ] Implement YAML → Cypher generator in Rust
```

### Phase 1: Core Products — 39 entities
```
Pillars (4):        qr-code, smart-link, barcode, landing-page
Infrastructure (1): short-link
Categories (5):     qr-code-style, qr-code-content, qr-code-frame,
                    barcode-format, landing-page-type
Styles (4):         custom-qr-code, qr-code-image, qr-code-art, qr-code-photo
Design Specs (9):   with-logo, with-text, color, shapes, transparent-bg,
                    background, background-color, background-gradient, background-image
Concepts (4):       dynamic-qr-code, static-qr-code, light-mode, dark-mode
Frames (6):         business-card, email-signature, flyer, poster,
                    table-tent, packaging-label
Landing Types (6):  link-in-bio, menu-restaurant, forms, announcement,
                    event-rsvp, booking-appointment

Arcs: ~85 semantic connections
Dependencies: None (root phase)
```

### Phase 2: Content Types — 58 entities
```
URL-based (12):     url, website, pdf, app-store, google-play, youtube,
                    tiktok, twitter, facebook, linkedin, instagram, pinterest
Contact (6):        vcard, email, phone, sms, whatsapp, telegram
WiFi/Network (3):   wifi, location, calendar
Payment (8):        payment, paypal, venmo, cashapp, bitcoin, ethereum,
                    alipay, wechat-pay
Business (10):      menu, coupon, feedback, review, survey, appointment,
                    event, ticket, membership, loyalty
Media (8):          audio, video, image, gallery, podcast, spotify,
                    apple-music, soundcloud
Social (11):        See Phase 11 for breakdown

Arcs: ~180 semantic connections
Dependencies: Phase 1 (categories)
```

### Phase 3: Barcode Types — 18 entities
```
1D Linear (10):     ean-13, ean-8, upc-a, upc-e, code-128, code-39,
                    code-93, itf-14, codabar, msi
2D Matrix (8):      data-matrix, pdf417, aztec, maxicode, han-xin,
                    dotcode, gs1-datamatrix, gs1-qr

Arcs: ~45 semantic connections
Dependencies: Phase 1 (barcode pillar)
```

### Phase 4: Features & Tools — 25 entities
```
Features (15):      analytics, tracking, a-b-testing, geo-targeting,
                    device-detection, scheduling, password-protection,
                    expiration, bulk-generation, api-access,
                    white-label, custom-domain, retargeting,
                    utm-parameters, link-cloaking
Tools (10):         qr-generator, qr-scanner, link-shortener,
                    landing-page-builder, analytics-dashboard,
                    bulk-creator, api, sdk, wordpress-plugin,
                    shopify-app

Arcs: ~75 semantic connections
Dependencies: Phases 1-3
```

### Phase 5: Mediums — 20 entities
```
Print (8):          business-cards, flyers, posters, brochures,
                    packaging, labels, magazines, newspapers
Digital (6):        social-media, email, websites, mobile-apps,
                    digital-signage, presentations
Physical (6):       storefronts, vehicles, trade-shows,
                    product-tags, receipts, table-tents

Arcs: ~60 semantic connections
Dependencies: Phase 1 (frames)
```

### Phase 6: Actions — 15 entities
```
create, generate, scan, customize, design, track,
share, download, print, embed, redirect, shorten,
analyze, export, import

Arcs: ~45 semantic connections
Dependencies: Phases 1-5
```

### Phase 7: Industries — 25 entities
```
restaurants, retail, healthcare, education, real-estate,
hospitality, events, marketing, manufacturing, logistics,
automotive, fitness, beauty, legal, finance, insurance,
nonprofits, government, construction, agriculture,
entertainment, media, technology, travel, sports

Arcs: ~75 semantic connections
Dependencies: Phases 1-6
```

### Phase 8: Brands — 25 entities
```
Tech Giants:    google, apple, microsoft, amazon, meta
Social:         instagram, tiktok, twitter, linkedin, youtube,
                facebook, pinterest, snapchat
Payment:        paypal, venmo, cashapp, stripe, square
Music:          spotify, apple-music, soundcloud
Messaging:      whatsapp, telegram, wechat

Arcs: ~50 semantic connections
Dependencies: Phase 2 (content types reference brands)
```

### Phase 9: Integrations — 12 entities
```
Automation:     zapier, make, n8n
CRM:            hubspot, salesforce, zoho
Spreadsheets:   google-sheets, airtable, notion
E-commerce:     shopify, woocommerce, bigcommerce

Arcs: ~36 semantic connections
Dependencies: Phase 4 (features)
```

### Phase 10: Concepts — 8 entities
```
dynamic-qr-code, static-qr-code, quiet-zone, error-correction,
version-number, encoding-mode, mask-pattern, format-information

Arcs: ~24 semantic connections
Dependencies: Phase 1
```

### Phase 11: Social Subcategories — 4 entities
```
qr-code-social, qr-code-music, qr-code-video, qr-code-messaging

Arcs: ~20 semantic connections
Dependencies: Phase 2
```

### Phase 12: Use Cases — 12 entities
```
marketing-campaigns, contactless-menu, product-authentication,
inventory-tracking, event-check-in, loyalty-programs,
virtual-tours, digital-business-card, wifi-sharing,
payment-collection, feedback-collection, document-sharing

Arcs: ~48 semantic connections
Dependencies: Phases 1-7
```

### Phase 13: Guides — 10 entities
```
how-to-create-qr-code, how-to-scan-qr-code, qr-code-best-practices,
qr-code-size-guide, qr-code-placement-guide, qr-code-design-tips,
dynamic-vs-static-guide, qr-code-tracking-guide,
qr-code-printing-guide, qr-code-marketing-guide

Arcs: ~40 semantic connections
Dependencies: Phases 1-6
```

### Phase 14: Comparisons — 8 entities
```
qr-code-vs-barcode, dynamic-vs-static, custom-vs-standard,
qr-code-vs-nfc, free-vs-paid, url-shortener-vs-qr,
qr-code-vs-ar, print-vs-digital

Arcs: ~32 semantic connections
Dependencies: Phases 1-5
```

---

## Generator Implementation

### New Rust Command
```bash
cargo run -- entity seed --project=qrcode-ai --phase=1
cargo run -- entity seed --project=qrcode-ai --all
cargo run -- entity validate --project=qrcode-ai
```

### Generator Logic
```
1. Read phase YAML file
2. Validate entity types against Entity.type enum (13 values)
3. Validate arc types against ArcKind registry (14 semantic types)
4. Generate Cypher:
   - MERGE Entity nodes with [:HAS_ENTITY] from Project
   - MERGE semantic arcs between entities
5. Execute against Neo4j
```

### Output Files
```
packages/db/seed/entities/
├── qrcode-ai-phase-01.cypher
├── qrcode-ai-phase-02.cypher
├── ...
└── qrcode-ai-phase-14.cypher
```

---

## Verification Checklist

### Per Phase
- [ ] YAML syntax valid
- [ ] All entity types in Entity.type enum
- [ ] All arc types in ArcKind registry
- [ ] No duplicate entity keys
- [ ] All arc targets exist
- [ ] Cypher generates without errors
- [ ] Neo4j import successful
- [ ] TUI displays entities correctly

### Final
- [ ] 279 Entity nodes in Neo4j
- [ ] ~825 semantic arcs in Neo4j
- [ ] All pillars connected
- [ ] Cross-phase arcs valid
- [ ] Graph traversal works (Cypher queries)

---

## Execution Timeline

```
┌─────────────────────────────────────────────────────────────────┐
│  EXECUTION ORDER                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  BATCH 1: Foundation (no dependencies)                          │
│  ├── Phase 0: Setup + Generator                                  │
│  ├── Phase 1: Core Products (39)                                 │
│  └── Phase 10: Concepts (8)                                      │
│                                                                  │
│  BATCH 2: Content Layer                                          │
│  ├── Phase 2: Content Types (58)                                 │
│  ├── Phase 3: Barcode Types (18)                                 │
│  └── Phase 11: Social Subcategories (4)                          │
│                                                                  │
│  BATCH 3: Feature Layer                                          │
│  ├── Phase 4: Features & Tools (25)                              │
│  └── Phase 9: Integrations (12)                                  │
│                                                                  │
│  BATCH 4: Context Layer                                          │
│  ├── Phase 5: Mediums (20)                                       │
│  ├── Phase 6: Actions (15)                                       │
│  └── Phase 7: Industries (25)                                    │
│                                                                  │
│  BATCH 5: External Layer                                         │
│  └── Phase 8: Brands (25)                                        │
│                                                                  │
│  BATCH 6: Knowledge Layer                                        │
│  ├── Phase 12: Use Cases (12)                                    │
│  ├── Phase 13: Guides (10)                                       │
│  └── Phase 14: Comparisons (8)                                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Arc target doesn't exist | Validate all targets before generation |
| Duplicate entity keys | Unique constraint in generator |
| Wrong entity type | Validate against Entity.type enum |
| Wrong arc type | Validate against ArcKind registry |
| Circular dependencies | DAG validation in generator |
| Performance (825 arcs) | Batch inserts, UNWIND in Cypher |

---

## Success Criteria

1. **279 Entity nodes** created in Neo4j under Project "qrcode-ai"
2. **~825 semantic arcs** connecting entities with typed relationships
3. **Zero validation errors** in generator
4. **TUI displays** all entities in correct hierarchy
5. **Graph queries** return expected traversal results

---

## Next Steps

1. [ ] Review and approve this plan
2. [ ] Create directory structure
3. [ ] Implement entity generator in Rust
4. [ ] Create Phase 1 YAML (copy from design doc)
5. [ ] Test generator with Phase 1
6. [ ] Execute all phases in batch order
7. [ ] Verify in Neo4j and TUI
