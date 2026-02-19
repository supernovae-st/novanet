---
id: 8
title: "Invariant Structure, Localized Content"
version: "v9.0"
status: stable
domain: ux-architecture
---

# ADR-008: Invariant Structure, Localized Content

**Status**: Approved (v9.0)

**Decision**: Relationships are defined at the invariant level. Content is resolved at generation time.

```
STRUCTURE = invariant (defined 1x)
CONTENT = localized (resolved 200x)
```

**Rationale**: Structure changes rarely. Content changes per locale.
