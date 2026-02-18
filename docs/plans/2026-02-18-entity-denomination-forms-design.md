# Entity Denomination Forms Design

**Date**: 2026-02-18
**Status**: Design validated

## Problem

Entity/EntityNative need canonical, prescriptive ways to name the entity across
locales — for content generation, internal links, slug derivation, and cross-page
consistency. The LLM must follow these forms strictly with no invention.

## Two Distinct Systems

| System | Where | Decided by |
|--------|-------|-----------|
| `denomination_forms` | Entity / EntityNative | Editorial (human) |
| Slug URL | BlockNative:head-seo-meta | SEO pipeline (LLM + strategy) |

The SEO-derived slug is **written back** to EntityNative.denomination_forms[url]
after the pipeline runs, enabling cross-page URL consistency.

## Form Types

| type | EN example | fr-FR example | zh-CN example | When to use |
|------|-----------|--------------|--------------|-------------|
| text | "qr code" | "qr code" | "二维码" | Prose, body content |
| title | "QR Code" | "QR Code" | "二维码" | H1, H2, meta_title |
| abbrev | "qr" | "qr" | "QR" | After first mention, short text |
| mixed | — | — | "QR码" | native_script locales tech context |
| base | — | — | "QR Code" | International reference form |
| url | — | "créer-un-qr-code" | "qr-code-sakusei" | URLs, internal link href |

Notes:
- `url` on Entity (invariant): does NOT exist — EN URL lives on `Page.slug`
- `url` on EntityNative: optional, populated post-SEO pipeline (not set manually)
- `mixed` / `base`: only for locales using `native_script` slugification rule
- All form values must follow locale slugification rules (ADR-032)
- `url` values for native_script locales use romanized ASCII (not native characters)

## Schema Property

### On Entity (invariant EN)

```yaml
denomination_forms:
  type: array
  required: true
  description: |
    Official and prescriptive denomination forms for this entity.

    ABSOLUTE RULE: The LLM MUST use only these forms when referring to
    this entity — in generated content, internal link anchors, headings,
    and slug derivations. No invention, paraphrase, or unlisted variation
    is allowed.

    Usage by type:
      text   → prose and body content
      title  → H1, H2, meta_title
      abbrev → after first mention, short text
  items:
    type: object
    properties:
      type:  { type: string, enum: [text, title, abbrev] }
      value: { type: string }
```

### On EntityNative (per locale)

```yaml
denomination_forms:
  type: array
  required: true
  description: |
    Official and prescriptive denomination forms for this entity in this locale.

    ABSOLUTE RULE: The LLM MUST use only these forms when referring to
    this entity — in generated content, internal link anchors, headings,
    and slug derivations. No invention, paraphrase, or unlisted variation
    is allowed.

    Usage by type:
      text   → prose and body content
      title  → H1, H2, meta_title
      abbrev → after first mention, short text
      mixed  → native_script locales tech context (e.g. QR码 for zh-CN)
      base   → international reference form
      url    → populated post-SEO pipeline (do not set manually)

    priority: 1 = default, 2+ = variation within same paragraph
  items:
    type: object
    properties:
      type:     { type: string, enum: [text, title, abbrev, mixed, base, url] }
      value:    { type: string }
      priority: { type: integer, default: 1 }
```

## Concrete Examples

### Entity:qr-code (invariant EN)

```yaml
denomination_forms:
  - { type: text,   value: "qr code"  }
  - { type: title,  value: "QR Code"  }
  - { type: abbrev, value: "qr"       }
```

### EntityNative:qr-code@fr-FR

```yaml
denomination_forms:
  - { type: text,   value: "qr code",          priority: 1 }
  - { type: title,  value: "QR Code",           priority: 1 }
  - { type: abbrev, value: "qr",                priority: 1 }
  - { type: url,    value: "créer-un-qr-code",  priority: 1 }
```

### EntityNative:qr-code@zh-CN

```yaml
denomination_forms:
  - { type: text,   value: "二维码",            priority: 1 }
  - { type: mixed,  value: "QR码",              priority: 2 }
  - { type: abbrev, value: "QR",                priority: 1 }
  - { type: base,   value: "QR Code",           priority: 3 }
  - { type: url,    value: "qr-code-sakusei",   priority: 1 }
```

## Validation Rules

1. Entity MUST have: `text` + `title` + `abbrev` (3 minimum)
2. EntityNative MUST have: `text` + `title` + `abbrev` (3 minimum)
3. `url` is optional — populated by SEO pipeline, never set manually
4. `mixed` / `base` only allowed for `native_script` locales
5. `url` values follow locale slugification rules (ASCII romanized for native_script)
6. Entity.denomination_forms = fallback if EntityNative.denomination_forms incomplete

## LLM Query (via MCP NovaNet)

```cypher
MATCH (en:EntityNative {key: 'entity:qr-code@zh-CN'})
UNWIND en.denomination_forms AS form
RETURN form.type, form.value, form.priority
ORDER BY form.priority
```

## Impact on YAML Files

- `packages/core/models/node-classes/org/semantic/entity.yaml`
  → add `denomination_forms` to properties
- `packages/core/models/node-classes/org/semantic/entity-native.yaml`
  → add `denomination_forms` to properties

## Relation to Other ADRs

- **ADR-030**: `url` form populated post-SEO pipeline (DERIVED_SLUG_FROM → EntityNative)
- **ADR-032**: form values must follow locale slugification rules
- **ADR-029**: denomination_forms live on EntityNative (*Native pattern)
