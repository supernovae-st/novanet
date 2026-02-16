---
paths:
  - "packages/**/*.ts"
  - "packages/**/*.tsx"
  - "apps/**/*.ts"
  - "apps/**/*.tsx"
---

# TypeScript Development Rules

## Code Style
- 2 spaces indentation
- Single quotes for strings
- Semicolons required
- 100 char line limit
- `PascalCase` for components/types, `camelCase` for functions/variables

## Type Safety
- No `any` - use `unknown` with type guards
- Prefer `interface` over `type` for object shapes
- Use Zod schemas for runtime validation
- Export types from `@novanet/core`

## React Patterns (Studio)
- Functional components only
- Zustand for state management with `useShallow`
- `useCallback` for handlers passed to children
- Avoid inline objects in JSX props

## NovaNet v0.13.0 Terminology
- `Realm` not Scope (shared, org)
- `Layer` not Subcategory (10 layers: 4 shared + 6 org)
- `Class` not Kind (NodeClass, ArcClass)
- `Trait` = data origin (defined, authored, imported, generated, retrieved)
- `*Native` suffix for locale-specific nodes (EntityNative, PageNative, BlockNative)
- `HAS_NATIVE` arc (replaces HAS_CONTENT/HAS_GENERATED)

## Imports
```typescript
// Correct
import type { NovaNetNode } from '@novanet/core';
import { CLASS_TAXONOMY } from '@novanet/core';

// Wrong
import { NovaNetNode } from '@novanet/core'; // Use 'import type' for types
```

## Testing
- Jest with `@testing-library/react` for components
- 80% coverage target
- Mock Neo4j with `jest.mock`
