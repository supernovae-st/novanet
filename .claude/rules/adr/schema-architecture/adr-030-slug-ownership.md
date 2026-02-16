---
id: ADR-030
title: "Slug Ownership"
version: v0.12.5
status: active
domain: schema-architecture
---

# ADR-030: Slug Ownership

**Status**: Approved (v0.12.5)

**Problem**: Current architecture mixes concerns:
1. Entity has semantic identity (key)
2. EntityNative has slug, full_path, parent_slug, depth
3. Page has slug
4. Entity.HAS_CHILD comment says "URL path = parent.slug" but Entity has NO slug
5. Page.REPRESENTS Entity (1:1 mandatory per ADR-028)

Which is source of truth for URLs?

**Decision**: Clear separation of concerns — Entity owns semantics, Page owns URLs.

## Principle

```
Entity  = QUOI (semantic concept, invariant)
Page    = OU   (URL structure, navigation)

Entity.key     = Semantic identifier (english, invariant)
Page.slug      = URL segment (english, invariant)
PageNative.slug = Localized URL segment (per locale)
```

## Who Has What

| Node | slug? | full_path? | Why |
|------|-------|------------|-----|
| Entity | No | No | Semantic concept, not URL-related |
| EntityNative | No | No | Content for concept, URL lives on Page |
| Page | Yes EN | No | URL segment (invariant, english) |
| PageNative | Yes L10n | Yes | Localized URL segment + full path |

## Key Design Decision: Entity.key != Page.slug

```
Entity.key:  "qr-code-instagram"  (full semantic identity)
Page.slug:   "instagram"          (just the URL segment)
```

This avoids: `/qr-code-generator/qr-code-instagram` (wrong)
We get: `/qr-code-generator/instagram` (correct)

## Concrete Example - 4 Entities

**Entity: instagram (BRAND)**
- No Page — external brand, not a page on our site
- Referenced via SEMANTIC_LINK from other entities

**Entity: qr-code-generator (PILLAR)**
```
Page.slug: "qr-code-generator"
PageNative(fr).slug: "generateur-qr-code"
PageNative(fr).full_path: "/fr/generateur-qr-code"
```

**Entity: qr-code-instagram (SUBTOPIC of qr-code-generator)**
```
Page.slug: "instagram"              # NOT "qr-code-instagram"
Page.SUBTOPIC_OF: page:qr-code-generator
PageNative(fr).slug: "instagram"    # Brand unchanged
PageNative(fr).full_path: "/fr/generateur-qr-code/instagram"
```

**Entity: template-instagram (SUBTOPIC of templates)**
```
Page.slug: "instagram"              # Same segment, different parent!
Page.SUBTOPIC_OF: page:templates
PageNative(fr).full_path: "/fr/modeles/instagram"
```

## Hierarchy Separation

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

## Migration Required

**Remove from EntityNative**:
- `slug`, `full_path`, `parent_slug`, `depth`, `slug_history`

**Add to PageNative**:
- `slug` (required, localized URL segment)
- `full_path` (required, indexed, full localized path)

**Fix Entity.yaml**: Remove misleading HAS_CHILD comment about URL paths.

## Rationale

1. **Single Source of Truth**: Page owns URL, Entity owns semantics
2. **No Duplication**: slug/full_path only on Page/PageNative
3. **Flexibility**: Page.slug can differ from Entity.key
4. **Localization**: PageNative has localized slug
5. **Brands Protected**: "instagram" stays "instagram" everywhere

**Reference**: `docs/plans/2026-02-14-entity-page-slug-brainstorm.md`
