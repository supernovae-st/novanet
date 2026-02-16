---
id: ADR-029
title: "*Native Pattern"
version: v0.12.5
status: active
domain: schema-architecture
---

# ADR-029: *Native Pattern

**Status**: Approved (v0.12.5)

**Problem**: Inconsistent naming for locale-specific nodes:
1. `EntityNative` doesn't convey "locale-specific"
2. `PageNative` implies it's different from `EntityNative`, but both are "native" (not translated)
3. Inconsistent suffixes: `*Content` vs `*Generated`
4. NovaNet philosophy: content is GENERATED NATIVELY, not translated from a source

**Decision**: Rename all locale-specific nodes to use `*Native` suffix. Traits distinguish authorship (authored vs generated).

## Node Renames

| Old Name | New Name | Trait | Who Creates |
|----------|----------|-------|-------------|
| `EntityNative` | `EntityNative` | authored | Human writes natively |
| `ProjectNative` | `ProjectNative` | authored | Human writes natively |
| `PageNative` | `PageNative` | generated | LLM generates natively |
| `BlockNative` | `BlockNative` | generated | LLM generates natively |

## Arc Unification

Merge `HAS_NATIVE` and `HAS_NATIVE` into single `HAS_NATIVE`:

| Old Arc | New Arc | Properties |
|---------|---------|------------|
| `HAS_NATIVE` | `HAS_NATIVE` | `{locale: "fr-FR"}` |
| `HAS_NATIVE` | `HAS_NATIVE` | `{locale: "fr-FR"}` |
| `NATIVE_OF` | `NATIVE_OF` | — |
| `NATIVE_OF` | `NATIVE_OF` | — |

## Key Pattern

Composite key unchanged:

```
{type}:{invariant_key}@{locale}

EntityNative.key  = "entity:qr-code@fr-FR"
ProjectNative.key = "project:qrcode-ai@fr-FR"
PageNative.key    = "page:homepage@fr-FR"
BlockNative.key   = "block:homepage:hero:1@fr-FR"
```

## Architecture

```
INVARIANT (defined)              LOCALE-SPECIFIC (*Native)
────────────────────             ─────────────────────────

Entity  ──[:HAS_NATIVE {locale}]──▶ EntityNative  (authored)
                                         │
                                         ├──[:FOR_LOCALE]──▶ Locale
                                         └──[:TARGETS]──▶ SEOKeyword

Project ──[:HAS_NATIVE {locale}]──▶ ProjectNative (authored)

Page    ──[:HAS_NATIVE {locale}]──▶ PageNative    (generated)

Block   ──[:HAS_NATIVE {locale}]──▶ BlockNative   (generated)
```

## Rationale

1. **Consistency**: All locale-specific nodes use same suffix pattern
2. **NovaNet Philosophy**: "Native" emphasizes content is generated natively, not translated
3. **Clarity**: Node name = "locale-specific content", Trait = "who creates it"
4. **Simplification**: Single arc type `HAS_NATIVE` instead of `HAS_NATIVE` + `HAS_NATIVE`

**Reference**: `docs/plans/2026-02-14-native-pattern-design.md`
