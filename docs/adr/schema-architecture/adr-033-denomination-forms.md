---
id: ADR-033
title: "Denomination Forms"
version: v0.13.1
status: active
domain: schema-architecture
---

# ADR-033: Denomination Forms

**Status**: Approved (v0.13.1)

**Problem**: The LLM has no explicit contract on how to refer to entities in generated content:
1. No prescribed canonical forms — LLM can invent arbitrary phrasings
2. `text` / `title` / `abbrev` / URL forms mixed or missing
3. Native-script locales (ja-JP, zh-CN) need special handling (mixed, base forms)
4. URL form is derived post-SEO pipeline (ADR-030, ADR-032) but not stored back
5. No formal governance — LLM may paraphrase, improvise, or repeat parent-path terms

**Decision**: Add `denomination_forms` to Entity and EntityNative as the **prescriptive canonical source** for all entity references in generated content.

## denomination_forms Schema

```
[
  { type: "text",   value: "...",  priority: 1 },
  { type: "title",  value: "...",  priority: 1 },
  { type: "abbrev", value: "...",  priority: 1 },
  // native_script locales only:
  { type: "mixed",  value: "...",  priority: 2 },
  { type: "base",   value: "...",  priority: 2 },
  // post-SEO pipeline only (written back after derivation):
  { type: "url",    value: "...",  priority: 1 }
]
```

**Storage**: Neo4j array of maps — `{type, value}` or `{type, value, priority}`.

## Form Types

| Type | Where used | Set by | Example (es-MX) |
|------|-----------|--------|-----------------|
| `text` | Prose, body content | Human (at authoring) | `"código qr"` |
| `title` | H1, H2, meta_title | Human (at authoring) | `"Código QR"` |
| `abbrev` | After first mention, short text | Human (at authoring) | `"qr"` |
| `mixed` | **native_script only**: tech/brand hybrid | Human (at authoring) | `"QR码"` (zh-CN) |
| `base` | **native_script only**: international reference form | Human (at authoring) | `"QR Code"` (in ja-JP content) |
| `url` | URL-safe slug for entity in this locale | SEO pipeline (write-back, ADR-030) | `"crear-código-qr"` |

## Placement per Node

| Node | Section | Rationale |
|------|---------|-----------|
| `Entity` | `standard_properties` | Required for all Entity instances |
| `EntityNative` | `properties` | Per-locale authored forms (required, full type set) |

**Entity.denomination_forms** contains: `text`, `title`, `abbrev` only.
- Entity is **invariant** (not locale-specific) — no `url` form, no `mixed`, no `base`.
- `url` belongs to EntityNative (locale-specific slug, post-SEO pipeline).

**EntityNative.denomination_forms** contains: all types applicable to the locale's script family.

## ABSOLUTE RULE (LLM Contract)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  ABSOLUTE RULE                                                                  │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  The LLM MUST use ONLY denomination_forms values when referring to an entity.   │
│                                                                                 │
│  NO invention, NO paraphrase, NO unlisted variation is allowed.                 │
│                                                                                 │
│  Usage rules:                                                                   │
│    text   → prose, body paragraphs (default prose form)                         │
│    title  → H1, H2, meta_title headings                                         │
│    abbrev → after first mention, space-constrained contexts                     │
│    mixed  → native_script locales: tech/brand hybrid allowed                   │
│    base   → native_script locales: international reference form                 │
│    url    → used ONLY by slug derivation pipeline (ADR-030), not directly by LLM│
│                                                                                 │
│  priority: 1 = default form, 2+ = acceptable variation within same paragraph   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## url Form and SEO Pipeline (ADR-030 Connection)

The `url` form is **not authored manually**. It is written back to `EntityNative.denomination_forms` by the SEO pipeline after slug derivation:

```
SEO pipeline:
  1. Derives slug from SEOKeyword.slug_form (INPUT REFERENCE + 5 modes)
  2. Writes result to BlockNative:head-seo-meta.slug
  3. Writes back url form to EntityNative.denomination_forms
     → enables cross-page URL consistency

Query pattern:
  MATCH (en:EntityNative {key: $key})
  WITH [f IN en.denomination_forms WHERE f.type = 'url'] AS url_forms
  RETURN url_forms[0].value AS url_form
```

**url form MUST follow locale slugification rule (ADR-032)**:

| Rule | Locales | Example |
|------|---------|---------|
| `latin_preserve` | fr-FR, es-MX, es-ES, pt-BR | `"crear-código-qr"` (ó retained) |
| `latin_strip` | en-US, en-GB | `"qr-code-generator"` (diacritics removed) |
| `latin_transform` | de-DE, de-AT | `"qr-code-erstellen"` (ü→ue etc.) |
| `transliterate` | ru-RU, uk-UA, el-GR | `"kod"` (Cyrillic→Latin) |
| `native_script` | ja-JP, ko-KR, zh-CN, ar-SA | ALWAYS romanized ASCII for `url` type |

## native_script Locales: Special Forms

For locales with `native_script` slugification rule (ja-JP, ko-KR, zh-CN, ar-SA):

```
EntityNative:qr-code@ja-JP:
  denomination_forms:
    - { type: text,   value: "QRコード",      priority: 1 }  ← native script prose
    - { type: title,  value: "QRコード作成",   priority: 1 }  ← native heading
    - { type: abbrev, value: "QR",           priority: 1 }  ← short form
    - { type: base,   value: "QR Code",      priority: 2 }  ← international reference
    - { type: url,    value: "qr-code-sakusei", priority: 1 } ← ALWAYS romanized ASCII

`base` = the international form (useful for non-native readers or when branding matters)
`url` = romanized ASCII (ADR-032: native_script → always ASCII for URLs)
```

## Validation Rule (Schema Rule 14)

The Rust `schema_rules` module enforces Rule 14 `DENOMINATION_FORMS_REQUIRED`:

```
Applies to: Entity, EntityNative
Checks:
  - denomination_forms field MUST be present in standard_properties (Entity)
    or properties (EntityNative)
  - At least text, title, abbrev types required
  - Warning if url type present on Entity (url belongs to EntityNative only)
```

Run: `cargo run -- schema validate --strict`

## Concrete Examples

### Entity:qr-code (invariant, no url)

```yaml
denomination_forms:
  - { type: text,   value: "qr code"  }
  - { type: title,  value: "QR Code"  }
  - { type: abbrev, value: "qr"       }
```

### EntityNative:qr-code@es-MX (latin_preserve)

```yaml
denomination_forms:
  - { type: text,   value: "código qr",      priority: 1 }
  - { type: title,  value: "Código QR",       priority: 1 }
  - { type: abbrev, value: "qr",              priority: 1 }
  - { type: url,    value: "crear-código-qr", priority: 1 }  ← post-SEO write-back
```

### EntityNative:qr-code@en-US (latin_strip)

```yaml
denomination_forms:
  - { type: text,   value: "qr code",          priority: 1 }
  - { type: title,  value: "QR Code",           priority: 1 }
  - { type: abbrev, value: "qr",                priority: 1 }
  - { type: url,    value: "qr-code-generator", priority: 1 }  ← post-SEO write-back
```

## Rationale

1. **LLM Governance**: Prevents invention and paraphrase — LLM has exact forms to use
2. **Terminological Consistency**: Same entity referred to identically across all blocks
3. **URL Consistency**: url form written back once, reused across all pages for this entity+locale
4. **Native-Script Support**: `mixed` and `base` forms handle CJK/Arabic locale needs
5. **Decoupled Pipeline**: url form is separate from prose forms — SEO pipeline owns it

## Reference Files

- `packages/core/models/node-classes/org/semantic/entity.yaml` — denomination_forms in standard_properties
- `packages/core/models/node-classes/org/semantic/entity-native.yaml` — denomination_forms in properties
- `packages/db/seed/54-denomination-forms.cypher` — seed for qr-code entity (5 locales)
- `tools/novanet/src/parsers/schema_rules.rs` — Rule 14 DENOMINATION_FORMS_REQUIRED
- ADR-030 (Slug Ownership) — url form write-back
- ADR-032 (URL Slugification) — locale slugification rules
