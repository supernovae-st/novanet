---
id: 6
title: "Realm Differentiates Scope"
version: "v9.0"
status: stable
domain: schema-architecture
---

# ADR-006: Realm Differentiates Scope

**Status**: Approved (v9.0, refined v10)

**Decision**: Same type name can exist in different realms with different scope.

```
Thing (shared)   -> Universal definition (Wikidata-linked)
Thing (project)  -> Brand-specific definition
```

**Rationale**: Realm is the WHERE axis. Type name is the WHAT axis. They're orthogonal.
