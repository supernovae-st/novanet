---
id: "026"
title: "Inverse Arc Policy"
version: "v0.12.1"
status: "active"
domain: "arc-design"
---

# ADR-026: Inverse Arc Policy

**Status**: Approved (v0.12.1)

**Problem**: NovaNet had inconsistent inverse arc coverage:
- 115 arcs total, but only ~5 had explicit inverses
- Some arcs declared `inverse: FOO` but `FOO` didn't exist (broken references)
- No clear policy on which arcs need inverses vs. which can remain unidirectional

**Decision**: Define a tiered inverse arc policy based on traversal patterns.

## Tier Definitions

| Tier | Requirement | Criteria |
|------|-------------|----------|
| **TIER 1** | Required | Core ownership arcs with frequent bidirectional traversal |
| **TIER 2** | Recommended | Knowledge/locale traversal arcs (high LLM context value) |
| **TIER 3** | Optional | Config/low-frequency arcs (unidirectional acceptable) |

## TIER 1: Required Inverses

These arcs MUST have explicit inverse definitions:

| Forward Arc | Inverse Arc | Rationale |
|-------------|-------------|-----------|
| `HAS_ENTITY` | `ENTITY_OF` | "Which pages use this entity?" |
| `HAS_PAGE` | `PAGE_OF` | "Which project owns this page?" |
| `HAS_PROJECT` | `PROJECT_OF` | "Which org owns this project?" |
| `HAS_BLOCK` | `BLOCK_OF` | "Which page contains this block?" (exists) |
| `HAS_NATIVE` | `NATIVE_OF` | "Which entity owns this content?" (exists) |
| `HAS_NATIVE` | `NATIVE_OF` | "Which page owns this output?" (exists) |
| `HAS_CHILD` | `CHILD_OF` | "What is this entity's parent?" (created v0.12.1) |
| `HAS_INSTRUCTION` | `INSTRUCTION_OF` | "Which page/block owns this instruction?" (created v0.12.1) |

## TIER 2: Recommended Inverses

These arcs SHOULD have inverses for LLM context loading:

| Forward Arc | Inverse Arc | Rationale |
|-------------|-------------|-----------|
| `HAS_TERMS` | `TERMS_OF` | Locale ↔ TermSet traversal |
| `HAS_EXPRESSIONS` | `EXPRESSIONS_OF` | Locale ↔ ExpressionSet traversal |
| `HAS_PATTERNS` | `PATTERNS_OF` | Locale ↔ PatternSet traversal |
| `HAS_CULTURE` | `CULTURE_OF` | Locale ↔ CultureSet traversal |
| `USES_ENTITY` | `USED_BY` | "Which pages reference this entity?" |
| `FOR_LOCALE` | `LOCALE_OF` | "Which content targets this locale?" |

## TIER 3: Optional (No Inverse Needed)

These arcs are acceptable without inverses:

- Configuration arcs: `BELONGS_TO_ORG`, `SUPPORTS_LOCALE`
- Type arcs: `OF_TYPE`, `HAS_STRUCTURE`
- Container arcs: `CONTAINS_*` (traversal is typically downward only)
- Semantic one-way: `ENABLES`, `REQUIRES` (use explicit inverse arcs)

## Naming Convention

| Pattern | Use Case | Example |
|---------|----------|---------|
| `HAS_*` | Ownership (parent→child) | `HAS_PAGE`, `HAS_ENTITY` |
| `*_OF` | Inverse ownership | `PAGE_OF`, `ENTITY_OF` |
| `CONTAINS_*` | Container→Atom (no inverse) | `CONTAINS_TERM` |
| `*_FOR` / `*_BY` | Direction indicator | `NATIVE_OF`, `USED_BY` |

## Implementation

**Arc YAML structure for inverses:**

```yaml
# Forward arc (has-entity.yaml)
arc:
  name: HAS_ENTITY
  inverse: ENTITY_OF
  # ...

# Inverse arc (entity-of.yaml)
arc:
  name: ENTITY_OF
  inverse_of: HAS_ENTITY  # Reference to forward arc
  # ...
```

**Validation rule**: If an arc declares `inverse: FOO`, then `FOO.yaml` MUST exist in the same family directory.

## Migration

1. **P0 (v0.12.1)**: Create missing inverses for broken references (CHILD_OF, INSTRUCTION_OF)
2. **P1 (v0.12.2)**: Create TIER 1 inverses (ENTITY_OF, PAGE_OF, PROJECT_OF)
3. **P2 (v0.13.0)**: Create TIER 2 inverses (knowledge atom traversal)

**Rationale**:

1. **LLM Context Loading**: Bidirectional traversal enables "spreading activation" patterns
2. **Query Efficiency**: Inverse arcs avoid expensive reverse pattern matching
3. **Semantic Clarity**: Inverse names document the relationship from both perspectives
4. **Maintainability**: Clear tier policy prevents arbitrary inverse proliferation

**Reference**: `docs/plans/2026-02-13-semantic-coherence-v0121-design.md`
