---
id: "011"
title: "Company Project Pattern"
version: "v10.5"
status: "superseded"
superseded_by: "012"
domain: "schema-architecture"
---

# ADR-011: Company Project Pattern (Superseded)

**Status**: Superseded by ADR-012 (v10.6)

**Decision**: Organization realm contains only the Organization node. Entity/EntityNative live in PROJECT realm only.

```
Organization -[:HAS_COMPANY_PROJECT]-> Project (company project)
                                         |-- Entity nodes here
             -[:HAS_PROJECT]-----------> Project (product projects)
```

**Rationale**:
- An organization has a "company project" that holds org-wide Entity nodes
- Entity/EntityNative in organization was redundant (same nodes existed in project)
- Simplifies the schema: 43 nodes instead of 45, 9 layers instead of 10
- Organization realm becomes a pure multi-tenant isolation boundary

> **Note**: v10.9 renamed `EntityL10n` to `EntityNative`. See ADR-014.
