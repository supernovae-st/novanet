---
id: "016"
title: "Type-Constrained Container Arcs"
version: "v10.9.0"
status: "active"
domain: "arc-design"
---

# ADR-016: Type-Constrained Container Arcs

**Status**: Approved (v10.9.0)

**Decision**: Split the generic `CONTAINS` arc into 6 type-specific arcs for semantic correctness.

**Previous** (v10.7):
```yaml
# Generic CONTAINS allowed 6x6=36 invalid combinations
arc:
  name: CONTAINS
  source: [ExpressionSet, TermSet, CultureSet, TabooSet, PatternSet, AudienceSet]
  target: [Expression, Term, CultureRef, Taboo, Pattern, AudienceTrait]
```

**New** (v10.9.0):
```yaml
# 6 type-specific arcs, 1:1 mapping only
CONTAINS_TERM:           TermSet -> Term
CONTAINS_EXPRESSION:     ExpressionSet -> Expression
CONTAINS_PATTERN:        PatternSet -> Pattern
CONTAINS_CULTURE_REF:    CultureSet -> CultureRef
CONTAINS_TABOO:          TabooSet -> Taboo
CONTAINS_AUDIENCE_TRAIT: AudienceSet -> AudienceTrait
```

**Rationale**:
- Semantic correctness: An ExpressionSet cannot contain a Taboo
- Graph validation: Type constraints prevent invalid data
- Query clarity: Arc name reveals target type
- No runtime overhead: Same performance, better semantics

**Impact**:
- Total arc count increases by 5 (1 generic -> 6 specific)
- All existing CONTAINS arcs must be migrated with correct type suffix
- Queries must use specific arc type instead of generic CONTAINS
