<div align="center">

# NovaNet Core

**Neo4j graph models, TypeScript types, and filter system for native multilingual content generation**

[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com/)
[![Zod](https://img.shields.io/badge/Zod-3.24-8b5cf6?style=flat-square)](https://zod.dev/)
[![Version](https://img.shields.io/badge/v0.19.0-06b6d4?style=flat-square)]()

---

*Part of the [NovaNet Monorepo](../../README.md)*

</div>

---

## Overview

NovaNet Core provides the foundational layer for the NovaNet knowledge graph system:

- **YAML Schema Definitions** - Node types across 2 Realms and 10 Layers
- **TypeScript Types** - Full type safety for Neo4j operations
- **NovaNetFilter** - Fluent API for composable graph queries
- **Zod Schemas** - Runtime validation for locale knowledge
- **Cypher Generator** - Convert filters to executable queries

---

> **Generation, NOT Translation**
>
> ```
> Source -> Translate -> Target        Wrong (traditional)
> Entity -> Generate Natively -> Content  Right (NovaNet)
> ```
>
> Content is **generated natively** per locale from invariant Entities, not translated.

---

## Quick Start

```bash
# From monorepo root
pnpm install
pnpm build --filter=@novanet/core

# Run tests
pnpm test --filter=@novanet/core

# Validate schemas
pnpm --filter=@novanet/core validate
```

---

## Structure

```
core/
+-- models/                    # YAML schema definitions (source of truth)
|   +-- _index.yaml            # Graph structure overview
|   +-- taxonomy.yaml          # v0.19.0: 2 Realms (shared/org), 10 Layers
|   +-- relations.yaml         # Legacy format (parser compatibility)
|   +-- node-classes/            # One file per node type (59 classes)
|   |   +-- shared/            # Realm: shared (config, locale, geography, knowledge) - 36 nodes
|   |   +-- org/               # Realm: org (6 layers) - 23 nodes
|   +-- arc-classes/             # One file per arc type (159 arcs)
|   +-- views/                 # Predefined view definitions
+-- src/
|   +-- types/                 # TypeScript definitions
|   +-- schemas/               # Zod validation schemas
|   +-- filters/               # NovaNetFilter, CypherGenerator
+-- scripts/                   # Build & validation scripts
```

---

## Node Types (v0.19.0)

| Realm | Layers | Description |
|-------|--------|-------------|
| **Shared** | config, locale, geography, knowledge | Universal locale knowledge (READ-ONLY) - 36 nodes |
| **Org** | config, foundation, structure, semantic, instruction, output | Business-specific content - 23 nodes |

---

## Usage

### NovaNetFilter

```typescript
import { NovaNetFilter, CypherGenerator } from '@novanet/core';

const filter = NovaNetFilter.create()
  .fromPage('page-pricing')
  .includeBlocks()
  .includeEntities({ spreading: true })
  .forLocale('fr-FR')
  .maxDepth(2);

const { query, params } = CypherGenerator.generate(filter);
```

### Type Imports

```typescript
import type {
  Project,
  Entity,
  EntityNative,   // v0.13.0: ADR-029 *Native pattern
  Locale,
  NodeType,
  RelationType,
} from '@novanet/core';

// Namespaced imports
import { config, schemas } from '@novanet/core';
```

---

## Testing

```bash
pnpm --filter=@novanet/core test           # Run all tests
pnpm --filter=@novanet/core test:watch     # Watch mode
pnpm --filter=@novanet/core validate       # Schema validation
```

---

## Related Packages

| Package | Description |
|---------|-------------|
| [@novanet/db](../db/) | Neo4j Docker infrastructure |
| [@novanet/studio](../../apps/studio/) | Graph visualization |

---

<div align="center">

**[NovaNet](../../README.md)** - [SuperNovae Studio](https://github.com/supernovae-st)

</div>
