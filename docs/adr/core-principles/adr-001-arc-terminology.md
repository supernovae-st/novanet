---
id: 1
title: "Arc Terminology"
version: "v9.5"
status: stable
domain: core-principles
---

# ADR-001: Arc Terminology

**Status**: Approved (v9.5)

**Decision**: Use "Arc" instead of "Edge" or "Relation" for directed links.

**Rationale**:
- Graph theory uses "arc" for directed edges
- Avoids confusion with React Flow's "Edge" (implementation detail)
- Avoids confusion with Neo4j's "Relationship" (database detail)
- Single consistent term across all platforms (YAML, Rust, TypeScript, UI)

**Exception**: React Flow components may use "Edge" internally since that's the library's API.
