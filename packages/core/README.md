<div align="center">

# 🪽 NovaNet Core

**Neo4j graph models, TypeScript types, and filter system for native multilingual content generation**

[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com/)
[![Zod](https://img.shields.io/badge/Zod-3.24-8b5cf6?style=flat-square)](https://zod.dev/)
[![Version](https://img.shields.io/badge/v9.0.0-06b6d4?style=flat-square)]()

---

*Part of the [🪽 NovaNet Monorepo](../../README.md)*

</div>

---

## Overview

NovaNet Core provides the foundational layer for the NovaNet knowledge graph system:

- **YAML Schema Definitions** — 35 node types across 3 Realms and 9 Layers
- **TypeScript Types** — Full type safety for Neo4j operations
- **NovaNetFilter** — Fluent API for composable graph queries
- **Zod Schemas** — Runtime validation for locale knowledge
- **Cypher Generator** — Convert filters to executable queries

---

> **Generation, NOT Translation**
>
> ```
> Source → Translate → Target        ❌ Traditional
> Concept → Generate Natively → L10n ✅ NovaNet
> ```
>
> Content is **generated natively** per locale from invariant Concepts, not translated.

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
├── models/                    # YAML schema definitions (source of truth)
│   ├── _index.yaml            # Graph structure overview (35 nodes, 50 relations)
│   ├── relations.yaml         # All Neo4j relationships (with family field)
│   ├── organizing-principles.yaml  # v9: Realm/Layer/Trait/EdgeFamily
│   ├── nodes/                 # One file per node type
│   │   ├── global/            # Realm: global (15 nodes)
│   │   ├── project/           # Realm: project (14 nodes)
│   │   └── shared/            # Realm: shared (6 nodes)
│   └── views/                 # Predefined view definitions
├── src/
│   ├── types/                 # TypeScript definitions
│   ├── schemas/               # Zod validation schemas
│   └── filters/               # NovaNetFilter, CypherGenerator
└── scripts/                   # Build & validation scripts
```

---

## Node Types (v9.0.0)

| Realm | Nodes | Layers |
|-------|-------|--------|
| **Global** | 15 | Configuration (1), Locale Knowledge (14) |
| **Project** | 14 | Foundation (3), Structure (4), Semantic (2), Instructions (3), Output (2) |
| **Shared** | 6 | SEO Intelligence (3), GEO Intelligence (3) |

---

## Usage

### NovaNetFilter

```typescript
import { NovaNetFilter, CypherGenerator } from '@novanet/core';

const filter = NovaNetFilter.create()
  .fromPage('page-pricing')
  .includeBlocks()
  .includeConcepts({ spreading: true })
  .forLocale('fr-FR')
  .maxDepth(2);

const { query, params } = CypherGenerator.generate(filter);
```

### Type Imports

```typescript
import type {
  Project,
  Concept,
  ConceptL10n,
  Locale,
  NodeType,
  RelationType,
} from '@novanet/core';

// Namespaced imports (v8.0.0+)
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

**[🪽 NovaNet](../../README.md)** · [SuperNovae Studio](https://github.com/supernovae-st)

</div>
