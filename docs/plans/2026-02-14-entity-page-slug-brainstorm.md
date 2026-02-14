# Entity / Page / Slug Architecture Design

**Date**: 2026-02-14
**Status**: Proposed (Brainstorm Session)
**Version**: v0.12.5
**Context**: Clarifying the relationship between Entity, Page, and URL slugs

## Problem Statement

Current architecture mixes concerns:
1. Entity has semantic identity (key)
2. EntityContent (→EntityNative) has slug, full_path, parent_slug, depth
3. Page has slug
4. Entity.HAS_CHILD comment says "URL path = parent.slug" but Entity has NO slug
5. Page.REPRESENTS Entity (1:1 mandatory per ADR-028)

**Which is source of truth for URLs?**

### The Confusing Case

```
URL: /qr-code-generator/instagram

Questions:
- Which Entity represents this URL?
- Where does "instagram" segment come from?
- Where does "qr-code-generator" segment come from?
- Why do both EntityNative and Page have slugs?
```

## Current Architecture (Confused)

```
EntityContent (entity-content.yaml:106-144)
├── slug: "créer-qr-code"              ← SLUG HERE (UTF-8)
├── full_path: "/fr/créer-qr-code"     ← FULL PATH HERE
├── parent_slug: null
└── depth: 0

Page (page.yaml:34-42)
├── slug: "qr-code-generator"          ← ALSO SLUG HERE
└── (no full_path, implicit via SUBTOPIC_OF)

Entity (entity.yaml:312)
└── HAS_CHILD comment: "URL path = parent.slug + / + child.slug"
    → But Entity has NO slug property!
```

## Decision: Clear Separation of Concerns

### Principle

```
Entity  = QUOI (semantic concept, invariant)
Page    = OÙ   (URL structure, navigation)

Entity.key     = Semantic identifier (english, invariant)
Page.slug      = URL segment (english, invariant)
PageNative.slug = Localized URL segment (per locale)
```

### Who Has What

| Node          | slug? | full_path? | Why |
|---------------|-------|------------|-----|
| Entity        | ❌     | ❌          | Semantic concept, not URL-related |
| EntityNative  | ❌     | ❌          | Content for concept, URL lives on Page |
| Page          | ✅ EN  | ❌          | URL segment (invariant, english) |
| PageNative    | ✅ L10n| ✅          | Localized URL segment + full path |

## Architecture Diagram

```
INVARIANT (defined)                  LOCALE-SPECIFIC (*Native)
────────────────────                 ─────────────────────────

Entity (semantic)                    EntityNative (authored)
├── key: "qr-code-instagram"         ├── key: entity:qr-code-instagram@fr-FR
├── display_name: "QR Code Instagram"├── display_name: "QR Code Instagram"
├── is_pillar: false                 ├── description: "..."
├── SUBTOPIC_OF → qr-code-generator  ├── benefits: [...]
└── (NO slug!)                       └── (NO slug!)
       │
       │ 1:1 REPRESENTS
       ▼
Page (structure)                     PageNative (generated)
├── key: "qr-code-instagram"         ├── key: page:qr-code-instagram@fr-FR
├── slug: "instagram"    ← JUST THE SEGMENT, not "qr-code-instagram"
├── SUBTOPIC_OF → page:qr-code-generator
│                                    ├── slug: "instagram" (brand unchanged)
└── (NO full_path!)                  └── full_path: "/fr/générateur-qr-code/instagram"
```

## Concrete Example: 4 Entities

### Entity: instagram

```yaml
Entity:
  key: "instagram"
  display_name: "Instagram"
  category: BRAND (external)
  # NO PAGE - external brand, not a page on our site

EntityNative (fr-FR):
  display_name: "Instagram"
  # Referenced by other entities via SEMANTIC_LINK
```

### Entity: qr-code-generator (PILLAR)

```yaml
Entity:
  key: "qr-code-generator"
  display_name: "QR Code Generator"
  is_pillar: true
  category: THING

Page:
  key: "qr-code-generator"
  slug: "qr-code-generator"  # EN invariant
  REPRESENTS → Entity:qr-code-generator

PageNative (fr-FR):
  slug: "générateur-qr-code"
  full_path: "/fr/générateur-qr-code"
```

### Entity: qr-code-instagram (SUBTOPIC)

```yaml
Entity:
  key: "qr-code-instagram"
  display_name: "QR Code Instagram"
  SUBTOPIC_OF → qr-code-generator
  SEMANTIC_LINK {used_for} → instagram

Page:
  key: "qr-code-instagram"
  slug: "instagram"           # JUST the segment!
  SUBTOPIC_OF → page:qr-code-generator

PageNative (fr-FR):
  slug: "instagram"           # Brand name stays unchanged
  full_path: "/fr/générateur-qr-code/instagram"
```

**Note**: Page.slug = "instagram", NOT "qr-code-instagram"
This avoids: `/qr-code-generator/qr-code-instagram` ❌
We get: `/qr-code-generator/instagram` ✅

### Entity: template-instagram (DIFFERENT PILLAR)

```yaml
Entity:
  key: "template-instagram"
  display_name: "Instagram Templates"
  SUBTOPIC_OF → templates  (different pillar)
  SEMANTIC_LINK {used_for} → instagram

Page:
  key: "template-instagram"
  slug: "instagram"           # Same segment, different parent!
  SUBTOPIC_OF → page:templates

PageNative (fr-FR):
  slug: "instagram"
  full_path: "/fr/modeles/instagram"  # Different path!
```

## Key Design Decisions

### D1: Entity.key ≠ Page.slug

```
Entity.key:  "qr-code-instagram"  (full semantic identity)
Page.slug:   "instagram"          (just the URL segment)
```

The Entity key carries full semantic meaning.
The Page slug is just the URL segment, derived from context.

### D2: Brands Stay Invariant

"instagram" doesn't translate - it's a brand name.
Same slug across all locales.

### D3: Hierarchy Separation

```
SEMANTIC (Entity)              URL (Page)
─────────────────              ──────────
Entity:qr-code                 Page:qr-code
    │ SUBTOPIC_OF                  │ SUBTOPIC_OF
    ▼                              ▼
Entity:qr-code-instagram       Page:qr-code-instagram

SAME STRUCTURE but DIFFERENT PURPOSE:
- Entity hierarchy = topic/cluster (for content strategy)
- Page hierarchy = URL/navigation (for routing)
```

### D4: Page.key = Entity.key

ADR-028 requires: Page.key = Entity.key (1:1 mandatory)

So we have:
- Entity.key = "qr-code-instagram"
- Page.key = "qr-code-instagram"
- Page.slug = "instagram" (derived, just the segment)

### D5: full_path on PageNative, NOT EntityNative

```yaml
PageNative:
  slug: "instagram"
  full_path: "/fr/générateur-qr-code/instagram"
  # Calculated: locale + parent.full_path + "/" + this.slug

EntityNative:
  # NO slug, NO full_path
  # Just content: display_name, description, benefits
```

## Migration Required

### Remove from EntityContent (→EntityNative)

```yaml
# DELETE these properties:
slug: ...
full_path: ...
parent_slug: ...
depth: ...
slug_history: ...
```

### Add to PageGenerated (→PageNative)

```yaml
# ADD these properties:
slug:
  type: string
  required: true
  description: "Localized URL segment"

full_path:
  type: string
  required: true
  indexed: true
  description: "Full localized URL path"
```

### Fix Entity.yaml

Remove or correct line 312 comment:
```yaml
# OLD (wrong):
# URL path = parent.slug + "/" + child.slug

# NEW (correct):
# HAS_CHILD is for SEMANTIC hierarchy (pillar/cluster)
# URL hierarchy is on Page via SUBTOPIC_OF
```

## Rationale

1. **Single Source of Truth**: Page owns URL, Entity owns semantics
2. **No Duplication**: slug/full_path only on Page/PageNative
3. **Flexibility**: Page.slug can differ from Entity.key
4. **Localization**: PageNative has localized slug
5. **Brands Protected**: Instagram stays "instagram" everywhere

## Related ADRs

- **ADR-028**: Page-Entity Architecture (1:1 mandatory)
- **ADR-029**: Native Pattern (*Native suffix)
- **ADR-030**: Slug Ownership (NEW - this document)
