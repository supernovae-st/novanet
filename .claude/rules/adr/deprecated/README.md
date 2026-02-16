# Deprecated ADRs

Superseded or historical-only ADRs. Kept for reference and migration documentation.

## ADRs in this Domain

| ADR | Name | Status | Superseded By | Summary |
|-----|------|--------|---------------|---------|
| [011](adr-011-company-project.md) | Company Project Pattern | superseded | ADR-012 | Org realm = Organization only |
| [014](adr-014-l10n-to-content.md) | L10n to Content/Generated | superseded | ADR-029 | *L10n → *Content/*Generated |
| [018](adr-018-classification-refinement.md) | Classification System Refinement | historical | — | global→shared, tenant→org |
| [019](adr-019-layer-reorganization.md) | Layer Reorganization | historical | — | locale-knowledge split |
| [020](adr-020-schema-refinement.md) | Schema Refinement | historical | — | Locale to config, SEO/GEO consolidation |

## Status Definitions

| Status | Meaning |
|--------|---------|
| **superseded** | Replaced by a newer ADR. Read the replacement instead. |
| **historical** | Documents a migration that happened. Reference only. |

## When to Consult

- **Understanding old code**: These ADRs explain why older code might use deprecated patterns
- **Migration archaeology**: When you find `EntityL10n` or `global` realm in old data
- **Evolution understanding**: How NovaNet schema evolved from v10 to v0.13.0

## Migration Quick Reference

```
┌─────────────────────────────────────────────────────────────────┐
│  MAJOR RENAMES (for archaeology)                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  global → shared          (ADR-018)                             │
│  tenant → org             (ADR-018)                             │
│  EntityL10n → EntityNative (ADR-014 → ADR-029)                  │
│  PageL10n → PageNative    (ADR-014 → ADR-029)                   │
│  locale-knowledge → locale + geography + knowledge (ADR-019)    │
│  Organization + Tenant → OrgConfig (ADR-019)                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Key Insight

> "Read these if you find old patterns in code. The superseding ADR has the current truth."
