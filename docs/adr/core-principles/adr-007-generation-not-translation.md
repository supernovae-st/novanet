---
id: 7
title: "Generation, Not Translation"
version: "core"
status: stable
domain: core-principles
---

# ADR-007: Generation, Not Translation

**Status**: Approved (core principle)

**Decision**: Content is generated natively per locale, NOT translated from a source.

```
WRONG:  Source -> Translate -> Target
RIGHT:  Entity (invariant) -> Generate natively -> EntityNative (local)
```

> **Note**: v10.9 renamed `EntityL10n` to `EntityNative`. See ADR-014.

**Rationale**: Translation loses cultural nuance. Native generation preserves it.
