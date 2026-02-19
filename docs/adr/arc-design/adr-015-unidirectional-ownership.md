---
id: "015"
title: "Unidirectional Ownership Arcs"
version: "v10.9.0"
status: "active"
domain: "arc-design"
---

# ADR-015: Unidirectional Ownership Arcs

**Status**: Approved (v10.9.0)

**Decision**: Ownership family arcs are intentionally unidirectional. Only a subset of ownership arcs have explicit inverse relationships.

**Arcs with inverses**:
- `HAS_BLOCK` <-> `BLOCK_OF`
- `HAS_NATIVE` <-> `NATIVE_OF`
- `HAS_NATIVE` <-> `NATIVE_OF`
- `HAS_TYPE` <-> `TYPE_OF`

**Arcs without inverses** (intentional):
- `HAS_PAGE`, `HAS_ENTITY`, `HAS_TERMS`, `HAS_EXPRESSIONS`, etc.

**Rationale**:
- Ownership implies hierarchy: parent owns children
- Traversal is typically parent->child (downward)
- Inverse navigation uses explicit Cypher: `(child)<-[:HAS_*]-(parent)`
- Adding inverses for all 43 ownership arcs would double complexity without proportional benefit
- Content/Generated inverses exist because bidirectional traversal is common for those patterns

**When to add an inverse**:
- Frequent bidirectional traversal in LLM context loading
- Performance-critical paths that benefit from indexed reverse lookup
- NOT just for DX convenience
