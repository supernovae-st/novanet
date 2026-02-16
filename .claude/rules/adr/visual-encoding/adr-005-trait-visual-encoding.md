---
id: 5
title: "Trait-Based Visual Encoding"
version: "v9.0"
status: stable
domain: visual-encoding
---

# ADR-005: Trait-Based Visual Encoding

**Status**: Approved (v9.0, updated v11.2)

**Decision**: Node trait (invariant/localized/knowledge/generated/aggregated) is encoded via border style, not color.

| Trait | Border Style | CSS |
|-------|--------------|-----|
| invariant | solid | `border-2 border-solid` |
| localized | dashed | `border-2 border-dashed` |
| knowledge | double | `border-[3px] border-double` |
| generated | dotted | `border-2 border-dotted` |
| aggregated | dotted thin | `border border-dotted` |

> **v11.2 Changes**: `derived` -> split into `generated` + `aggregated`, `job` removed.

**Rationale**: Colorblind-safe. Layer already uses fill color.
