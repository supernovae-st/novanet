<div align="center">

# 📦 NovaNet Core

**Neo4j graph models, TypeScript types, and filter system for native multilingual content generation**

[![← Studio](https://img.shields.io/badge/←_Studio-64748b?style=flat-square)](https://github.com/supernovae-st/novanet-studio)
[![🏢 HQ](https://img.shields.io/badge/🏢_HQ-8b5cf6?style=flat-square)](https://github.com/supernovae-st/novanet-hq)
[![🐳 Infra →](https://img.shields.io/badge/🐳_Infra_→-6366f1?style=flat-square)](https://github.com/supernovae-st/novanet-infra)

<br>

[![TypeScript](https://img.shields.io/badge/TypeScript-5.7-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Neo4j](https://img.shields.io/badge/Neo4j-5.x-10b981?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com/)
[![Zod](https://img.shields.io/badge/Zod-3.x-8b5cf6?style=flat-square)](https://zod.dev/)

---

*Part of the [NovaNET Ecosystem](https://github.com/supernovae-st/novanet-hq)*

</div>

---

## 📋 Overview

NovaNet Core provides the foundational layer for the NovaNet knowledge graph system:

- **YAML Schema Definitions** - 35 node types across 7 categories
- **TypeScript Types** - Full type safety for Neo4j operations
- **NovaNetFilter** - Fluent API for composable graph queries
- **Zod Schemas** - Runtime validation for locale knowledge
- **Cypher Generator** - Convert filters to executable queries

## ⚠️ Philosophy

```
NOVANET = NATIVE GENERATION (NOT Translation)

Source → Translate → Target        ❌ WRONG
Concept → Generate Natively → L10n ✅ RIGHT
```

Content is **generated natively** per locale from invariant Concepts, not translated.

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Run tests
npm test

# Validate schemas
npm run validate
```

## 📁 Structure

```
core/
├── models/                    # YAML schema definitions
│   ├── _index.yaml            # Graph structure overview
│   ├── relations.yaml         # 46 Neo4j relationships
│   ├── nodes/                 # One file per node type
│   │   ├── project/           # 📦 Business (3 nodes)
│   │   ├── content/           # 💡 Semantic (5 nodes)
│   │   ├── locale/            # 🌍 Locale knowledge (15 nodes)
│   │   ├── generation/        # ⚡ Prompts & output (5 nodes)
│   │   ├── seo/               # 🔍 SEO (4 nodes)
│   │   ├── geo/               # 🤖 GEO (4 nodes)
│   │   └── analytics/         # 📊 Metrics (1 node)
│   └── views/                 # Predefined view definitions
├── src/
│   ├── filters/               # NovaNetFilter, CypherGenerator
│   ├── parsers/               # Markdown → Neo4j parsers
│   ├── schemas/               # Zod validation schemas
│   └── types/                 # TypeScript definitions
├── neo4j/                     # Seed scripts
│   └── seed/                  # Cypher files
└── scripts/                   # Build & validation scripts
```

## 🔧 Node Categories

| Category | Icon | Nodes | Purpose |
|----------|------|-------|---------|
| **project** | 📦 | 3 | Business definition |
| **content** | 💡 | 5 | Semantic structure |
| **locale** | 🌍 | 15 | Locale knowledge |
| **generation** | ⚡ | 5 | Prompts & output |
| **seo** | 🔍 | 4 | Search optimization |
| **geo** | 🤖 | 4 | Generative engine optimization |
| **analytics** | 📊 | 1 | Performance metrics |

## 💻 Usage

### NovaNetFilter

```typescript
import { NovaNetFilter } from 'novanet-core';

// Fluent API for graph queries
const filter = new NovaNetFilter()
  .withNodeTypes(['Concept', 'ConceptL10n'])
  .withRelations(['HAS_L10N'])
  .withLocale('fr-FR')
  .withPriority(['critical', 'high'])
  .withLimit(100);

// Generate Cypher
const cypher = filter.toCypher();
```

### Type Imports

```typescript
import type {
  Project,
  Concept,
  Locale,
  NodeType,
  NodeCategory
} from 'novanet-core';
```

## 🧪 Testing

```bash
npm test              # Run all tests
npm run test:watch    # Watch mode
npm run validate      # Schema + version + relations
```

## 🗄️ Neo4j Setup

```bash
# From monorepo root
npm run infra:up

# Seed database
cd neo4j && ./seed.sh

# Access
# Browser: http://localhost:7474
# Bolt: bolt://localhost:7687
# Credentials: neo4j / novanetpassword
```

## 📊 Schema Version

Current: **v7.2.4**

See `models/_index.yaml` for full changelog and graph structure.

## 🔗 Related

- [novanet-hq](https://github.com/supernovae-st/novanet-hq) - Dev workspace
- [novanet-studio](https://github.com/supernovae-st/novanet-studio) - Visualization
- [novanet-infra](https://github.com/supernovae-st/novanet-infra) - Docker

---

<div align="center">

**Navigate**

[![← Studio](https://img.shields.io/badge/←_Studio-64748b?style=flat-square)](https://github.com/supernovae-st/novanet-studio)
[![🏢 HQ](https://img.shields.io/badge/🏢_HQ-8b5cf6?style=flat-square)](https://github.com/supernovae-st/novanet-hq)
[![🐳 Infra →](https://img.shields.io/badge/🐳_Infra_→-6366f1?style=flat-square)](https://github.com/supernovae-st/novanet-infra)

---

[🏢 HQ](https://github.com/supernovae-st/novanet-hq) · [🎨 Studio](https://github.com/supernovae-st/novanet-studio) · [📦 Core](./README.md) · [🐳 Infra](https://github.com/supernovae-st/novanet-infra)

---

**Part of [SuperNovae Studio](https://github.com/supernovae-st)** · [NovaNET Public](https://github.com/supernovae-st/novanet)

</div>
