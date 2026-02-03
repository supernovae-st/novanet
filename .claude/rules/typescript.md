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

## NovaNet v9 Terminology
- `Realm` not Scope
- `Layer` not Subcategory/NodeCategory
- `Kind` not NodeTypeMeta
- `NavigationMode` not DataMode
- `Trait` for locale behavior classification

## Imports
```typescript
// Correct
import type { NovaNetNode } from '@novanet/core';
import { KIND_META } from '@novanet/core';

// Wrong
import { NovaNetNode } from '@novanet/core'; // Use 'import type' for types
```

## Testing
- Jest with `@testing-library/react` for components
- 80% coverage target
- Mock Neo4j with `jest.mock`
