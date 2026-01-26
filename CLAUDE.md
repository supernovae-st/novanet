# NovaNet

Knowledge graph localization orchestrator for native multilingual content generation.

---

## Overview

NovaNet uses Neo4j to orchestrate **native content generation** (NOT translation) across 200+ locales. An orchestrator dispatches content tasks to specialized sub-agents, each generating content natively in the target locale.

**Target Application**: QR Code AI (https://qrcode-ai.com)

```
CRITICAL: Generation, NOT Translation

Source -> Translate -> Target        <-- WRONG
Concept (invariant) -> Generate natively -> ConceptL10n (local)  <-- RIGHT
```

---

## Monorepo Structure

```
novanet/
├── core/                 # Core library (TypeScript)
│   ├── models/           # YAML schema definitions (35 node types)
│   ├── src/              # NovaNetFilter, parsers, schemas
│   ├── neo4j/            # Seed scripts
│   └── CLAUDE.md         # Core-specific context
│
├── studio/               # Visualization app (Next.js 15)
│   ├── src/              # React Flow, force-graph, Zustand
│   └── CLAUDE.md         # Studio-specific context
│
└── infra/                # Infrastructure (Docker, Neo4j)
```

---

## Commands

```bash
# Development
npm run dev              # Start studio dev server
npm run build            # Build studio
npm run lint             # Lint all workspaces
npm run type-check       # Type check all workspaces
npm run test             # Test all workspaces

# Infrastructure (Neo4j)
npm run infra:up         # Start Neo4j
npm run infra:down       # Stop Neo4j
npm run infra:logs       # View logs

# Database seed
cd core/neo4j && ./seed.sh
```

---

## Neo4j

- **Browser**: http://localhost:7474
- **Bolt**: bolt://localhost:7687
- **Credentials**: `neo4j` / `novanetpassword`

---

## Conventions

| Aspect | Convention |
|--------|------------|
| **Naming** | `novanet` (packages), `NovaNet` (classes/types) |
| **Formatting** | 2 spaces, 100 chars, single quotes, semicolons |
| **Components** | PascalCase |
| **Functions** | camelCase |
| **Constants** | UPPER_SNAKE_CASE |
| **Commits** | Conventional Commits |

---

## Workspace Details

| Workspace | Focus | See |
|-----------|-------|-----|
| **core** | Neo4j, schemas, filters, parsers | `core/CLAUDE.md` |
| **studio** | Visualization, React, Zustand | `studio/CLAUDE.md` |

---

## Quick Start

```bash
# 1. Start services
npm run infra:up
cd core/neo4j && ./seed.sh

# 2. Start development
npm run dev              # http://localhost:3000
```
