---
id: 2
title: "Symmetric Taxonomy"
version: "v9.5"
status: stable
domain: node-classification
---

# ADR-002: Symmetric Taxonomy

**Status**: Approved (v9.5)

**Decision**: Use prefixed types for global uniqueness, short properties for local context.

```typescript
// Types with prefix (globally unique)
type NodeRealm = 'shared' | 'org';  // v11.2: renamed from global/tenant
type ArcFamily = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';

// Properties without prefix (context is clear)
interface NodeKind {
  realm: NodeRealm;    // Not "nodeRealm"
  layer: NodeLayer;    // Not "nodeLayer"
}

interface ArcKind {
  family: ArcFamily;   // Not "arcFamily"
  scope: ArcScope;     // Not "arcScope"
}
```

**Rationale**: Avoids stuttering (`arcKind.arcFamily`) while maintaining type safety.
