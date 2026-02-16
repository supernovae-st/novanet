---
id: 3
title: "YAML-First Architecture"
version: "v9.0"
status: stable
domain: core-principles
---

# ADR-003: YAML-First Architecture

**Status**: Approved (v9.0)

**Decision**: YAML files are the single source of truth. All code is generated.

```
taxonomy.yaml           -> Colors, display names, facet definitions
node-classes/**/*.yaml    -> NodeKind definitions
arc-classes/**/*.yaml     -> ArcKind definitions
         |
    Rust Generator
         |
TypeScript + Cypher + Mermaid + Rust structs
```

**Rationale**:
- Single source of truth prevents drift
- Non-developers can edit YAML
- Generators enforce consistency
- CI validates sync
