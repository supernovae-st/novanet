# Native Pattern Design

**Date**: 2026-02-14
**Status**: Approved (Brainstorm Session)
**Version**: v0.12.5

## Summary

Rename all locale-specific nodes to use `*Native` suffix, unifying the naming pattern while using traits to distinguish authorship (authored vs generated).

## Problem

1. `EntityContent` doesn't convey "locale-specific"
2. `PageGenerated` implies it's different from `EntityContent`, but both are "native" (not translated)
3. Inconsistent suffixes: `*Content` vs `*Generated`
4. NovaNet philosophy: content is GENERATED NATIVELY, not translated from a source

## Decision

### Naming Convention

All locale-specific nodes use `*Native` suffix:

| Old Name | New Name | Trait | Who Creates |
|----------|----------|-------|-------------|
| `EntityContent` | `EntityNative` | authored | Human writes natively |
| `ProjectContent` | `ProjectNative` | authored | Human writes natively |
| `PageGenerated` | `PageNative` | generated | LLM generates natively |
| `BlockGenerated` | `BlockNative` | generated | LLM generates natively |

### Trait Semantics (ADR-024)

The trait answers "WHO creates the data?":

- `authored` = Human writes per locale (editorial content)
- `generated` = Our LLM produces per locale (output)

Both are "native" - generated natively for the locale, NOT translated.

### Arc Unification

Merge `HAS_CONTENT` and `HAS_GENERATED` into single `HAS_NATIVE`:

| Old Arc | New Arc | Properties |
|---------|---------|------------|
| `HAS_CONTENT` | `HAS_NATIVE` | `{locale: "fr-FR"}` |
| `HAS_GENERATED` | `HAS_NATIVE` | `{locale: "fr-FR"}` |
| `CONTENT_OF` | `NATIVE_OF` | — |
| `GENERATED_FOR` | `NATIVE_OF` | — |

### Key Pattern

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
4. **Simplification**: Single arc type `HAS_NATIVE` instead of `HAS_CONTENT` + `HAS_GENERATED`

## Migration

### Files to Update

**YAML Node Definitions:**
- `packages/core/models/node-kinds/org/semantic/entity-content.yaml` → `entity-native.yaml`
- `packages/core/models/node-kinds/org/foundation/project-content.yaml` → `project-native.yaml`
- `packages/core/models/node-kinds/org/output/page-generated.yaml` → `page-native.yaml`
- `packages/core/models/node-kinds/org/output/block-generated.yaml` → `block-native.yaml`

**YAML Arc Definitions:**
- `packages/core/models/arc-kinds/ownership/has-content.yaml` → `has-native.yaml`
- `packages/core/models/arc-kinds/ownership/has-generated.yaml` → DELETE (merged)
- `packages/core/models/arc-kinds/ownership/content-of.yaml` → `native-of.yaml`

**Documentation:**
- `.claude/rules/novanet-terminology.md`
- `.claude/rules/novanet-decisions.md` (add ADR-029)

### Neo4j Migration

```cypher
// Rename node labels
MATCH (n:EntityContent) SET n:EntityNative REMOVE n:EntityContent;
MATCH (n:ProjectContent) SET n:ProjectNative REMOVE n:ProjectContent;
MATCH (n:PageGenerated) SET n:PageNative REMOVE n:PageGenerated;
MATCH (n:BlockGenerated) SET n:BlockNative REMOVE n:BlockGenerated;

// Rename relationships (requires recreation)
// HAS_CONTENT → HAS_NATIVE
// HAS_GENERATED → HAS_NATIVE
// CONTENT_OF → NATIVE_OF
// GENERATED_FOR → NATIVE_OF
```

## Related ADRs

- **ADR-014**: L10n to Content/Generated (superseded by this)
- **ADR-024**: Trait as Data Origin (unchanged, referenced)
- **ADR-029**: Native Pattern (NEW - this document)
