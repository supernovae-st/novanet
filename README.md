<div align="center">

# 🪽 NovaNet

**Native content generation engine powered by Neo4j knowledge graphs**

Generate culturally-native content across 200+ locales — not translation, but true localization from semantic concepts.

[![CI](https://img.shields.io/github/actions/workflow/status/supernovae-st/novanet/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/supernovae-st/novanet/actions)
[![Tests](https://img.shields.io/badge/tests-1249_passing-success?style=flat-square)](https://github.com/supernovae-st/novanet)
[![Rust](https://img.shields.io/badge/Rust-1.84-DEA584?style=flat-square&logo=rust&logoColor=white)](https://rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://typescriptlang.org)
[![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com)
[![Turborepo](https://img.shields.io/badge/Turborepo-2.8-EF4444?style=flat-square&logo=turborepo&logoColor=white)](https://turbo.build)
[![pnpm](https://img.shields.io/badge/pnpm-9-F69220?style=flat-square&logo=pnpm&logoColor=white)](https://pnpm.io)
[![Node](https://img.shields.io/badge/Node-≥20-339933?style=flat-square&logo=node.js&logoColor=white)](https://nodejs.org)

</div>

---

> **Generation, NOT Translation**
>
> ```
> Source → Translate → Target                           ❌ Traditional
> Entity (defined) → Generate natively → EntityNative ✅ NovaNet
> ```
>
> Content is generated natively per locale from defined semantic entities, not translated from a source language.
> Each locale gets culturally-native content, preserving local nuances that translation would lose.

---

## Features

|  |  |  |  |
|:---:|:---:|:---:|:---:|
| **Knowledge Graph** | **200+ Locales** | **Graph Studio** | **AI-Powered** |
| 60 node types, 149 arcs | Native generation per locale | Interactive 2D visualization | Claude API for natural language queries |
| Neo4j with APOC | Locale knowledge layer | React Flow + ELK.js layouts | Cypher generation from text |

---

## Architecture

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {
  'primaryColor': '#6366f1',
  'primaryTextColor': '#fff',
  'primaryBorderColor': '#4f46e5',
  'lineColor': '#94a3b8',
  'secondaryColor': '#06b6d4',
  'tertiaryColor': '#f8fafc'
}}}%%
flowchart TB
    subgraph MONO["NovaNet Monorepo"]
        direction TB
        CORE["@novanet/core v0.17.2\nTypes · Schemas · Filters"]
        DB["@novanet/db v0.17.2\nDocker · Seeds · Migrations"]
        STUDIO["@novanet/studio v0.17.2\nNext.js 16 · React 19"]
        RUST["novanet CLI v0.17.2\nRust · 16 commands · TUI"]
    end

    CORE --> STUDIO
    RUST -.->|reads YAML| CORE
    RUST -.->|generates| DB

    NEO4J[("Neo4j 5.26\n~19,000 nodes")]
    DB -.-> NEO4J
    STUDIO --> NEO4J
    RUST --> NEO4J

    style CORE fill:#06b6d4,stroke:#0891b2,color:#fff
    style DB fill:#10b981,stroke:#059669,color:#fff
    style STUDIO fill:#8b5cf6,stroke:#7c3aed,color:#fff
    style RUST fill:#DEA584,stroke:#B7410E,color:#fff
    style NEO4J fill:#018bff,stroke:#0284c7,color:#fff
```

---

## Quick Start

**Prerequisites**

![Node.js](https://img.shields.io/badge/Node.js-≥20-339933?style=flat-square&logo=node.js&logoColor=white)
![pnpm](https://img.shields.io/badge/pnpm-≥9-F69220?style=flat-square&logo=pnpm&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Required-2496ED?style=flat-square&logo=docker&logoColor=white)

```bash
# 1. Clone the repository
git clone git@github.com:supernovae-st/supernovae-agi.git
cd supernovae-agi/novanet

# 2. Install dependencies
pnpm install

# 3. Start Neo4j
pnpm infra:up

# 4. Seed the database
pnpm infra:seed

# 5. Start development server
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) — Neo4j Browser at [http://localhost:7474](http://localhost:7474)

---

## Monorepo Structure

```
novanet/
├── turbo.json                 # Turborepo pipeline config
├── pnpm-workspace.yaml        # Workspace definitions
├── packages/
│   ├── core/                  # @novanet/core — types, schemas, filters
│   │   ├── models/            # YAML schema definitions (source of truth)
│   │   │   ├── taxonomy.yaml  # 2 realms (shared 4 + org 6), 10 layers
│   │   │   ├── node-classes/    # node definitions by realm/layer
│   │   │   └── arc-classes/     # arc definitions by family
│   │   └── src/               # TypeScript implementation
│   └── db/                    # @novanet/db — Neo4j infrastructure
│       ├── docker-compose.yml # Neo4j 5.26 + APOC
│       ├── seed/              # Cypher seed scripts
│       └── seed.sh            # Seed runner
├── tools/
│   └── novanet/               # Rust CLI + TUI binary
│       ├── src/               # Rust source (16 commands, 8 generators)
│       └── Cargo.toml         # 1255 tests, zero clippy warnings
└── apps/
    └── studio/                # @novanet/studio — web visualization
        ├── src/app/           # Next.js App Router
        ├── src/components/    # React components
        ├── src/stores/        # Zustand state management
        └── src/lib/           # Utilities
```

---

## Packages

| Package | Version | Description |
|---------|---------|-------------|
| **@novanet/core** | `0.17.2` | Types, Zod schemas, NovaNetFilter API, Cypher generators |
| **@novanet/db** | `0.17.2` | Docker Compose for Neo4j, Cypher seeds, migrations |
| **@novanet/studio** | `0.17.2` | Interactive graph visualization with AI chat |
| **tools/novanet** | `0.17.2` | Rust CLI + TUI for schema generation, validation, queries |
| **tools/novanet-mcp** | `0.17.2` | MCP Server for AI agent integration (14 tools) |

---

## Commands

### Development

| Command | Description |
|---------|-------------|
| `pnpm dev` | Start Studio dev server |
| `pnpm build` | Build all packages |
| `pnpm lint` | Lint all packages |
| `pnpm type-check` | TypeScript type checking |
| `pnpm test` | Run all tests |

### Infrastructure

| Command | Description |
|---------|-------------|
| `pnpm infra:up` | Start Neo4j container |
| `pnpm infra:down` | Stop Neo4j container |
| `pnpm infra:seed` | Seed database with initial data |
| `pnpm infra:reset` | Reset database (down + up + seed) |

### Rust CLI (tools/novanet)

```bash
# Schema operations (YAML → Cypher/TS/Mermaid)
cargo run -- schema generate        # Regenerate all artifacts
cargo run -- schema validate        # Validate YAML coherence

# Navigation commands (v0.12.5)
cargo run -- blueprint              # Schema-graph visualization
cargo run -- data                   # Data nodes only
cargo run -- overlay                # Data + Schema combined
cargo run -- query --realm=org      # Faceted query

# CRUD operations
cargo run -- node create --class=Page --key=my-page
cargo run -- search --query="page" --class=Page

# Interactive TUI
cargo run -- tui                    # Galaxy-themed terminal UI
```

### Turborepo Filters

```bash
pnpm build --filter=@novanet/core        # Build only core
pnpm test --filter=@novanet/studio       # Test only studio
pnpm build --filter=...[HEAD^1]          # Build changed packages
```

---

## Neo4j Access

```
Browser:  http://localhost:7474
Bolt:     bolt://localhost:7687
User:     neo4j
Password: (see NEO4J_PASSWORD env var)
```

---

## Graph Schema

NovaNet models content as a knowledge graph with **2 Realms** and **10 Layers** (v0.17.2):

| Realm | Layers | Description |
|-------|--------|-------------|
| **Shared** | config, locale, geography, knowledge | Universal definitions + locale knowledge (READ-ONLY) — 36 nodes |
| **Org** | config, foundation, structure, semantic, instruction, output | Organization-specific content — 21 nodes |

**v11.5 changes:** Locale definition moved to shared/config. SEO/GEO nodes consolidated to shared/knowledge.

Arcs are classified by **ArcFamily** (ownership, localization, semantic, generation, mining, schema).

> **v0.19.0**: Traits removed from schema (ADR-024 deprecated). Provenance is now tracked per-instance on nodes that need it.

See [`packages/core/models/taxonomy.yaml`](packages/core/models/taxonomy.yaml) for complete schema.

---

## Studio Features

**NovaNet Studio** is a web-based graph visualization tool:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NovaNet Studio                                          ⌘K  ⌘J  ?  N  F   │
├─────────────────────────────────────────────────────────────────────────────┤
│ ┌─ Filters ──────┐  ┌─ Graph View ─────────────────────┐  ┌─ Details ────┐ │
│ │ Realm          │  │                                  │  │ Page         │ │
│ │ ☑ Shared       │  │      [Locale]──┐                 │  │ key: home    │ │
│ │ ☑ Org          │  │          │     ▼                 │  │ realm: org   │ │
│ │                │  │   [Project]──[Page]──[Block]     │  │ layer: struc │ │
│ │                │  │          │     │                 │  │              │ │
│ │ Layer          │  │          ▼     ▼                 │  │ Relations:   │ │
│ │ ☑ Foundation   │  │   [Entity]──[EntityNative]       │  │ → 3 blocks   │ │
│ │ ☑ Structure    │  │          │                       │  │ → 1 project  │ │
│ │ ☑ Semantic     │  │          ▼                       │  │              │ │
│ │ ...            │  │   [BlockNative]                  │  │ [Copy JSON]  │ │
│ └────────────────┘  └──────────────────────────────────┘  └──────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│  Mode: Graph  │  60 nodes  │  149 arcs  │  Zoom: 100%  │  Locale: fr-FR │
└─────────────────────────────────────────────────────────────────────────────┘
```
*Interactive 2D graph visualization with AI-powered queries (⌘J)*

- **2 Navigation Modes** — Graph (unified tree), Nexus (hub) — `[1]` `[2]` keys
- **9 Filter Presets** — Quick views via `1-8, 0` keys
- **AI Chat** — Natural language to Cypher with `⌘J`
- **40+ Keyboard Shortcuts** — Full keyboard navigation
- **DX-First** — Every property is copyable (JSON/TS/YAML)

---

## Tech Stack

**Runtime**

![React](https://img.shields.io/badge/React-19-61DAFB?style=flat-square&logo=react&logoColor=white)
![Next.js](https://img.shields.io/badge/Next.js-16-000000?style=flat-square&logo=next.js&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)
![Tailwind](https://img.shields.io/badge/Tailwind-3.4-06B6D4?style=flat-square&logo=tailwindcss&logoColor=white)

**Data**

![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)
![Zustand](https://img.shields.io/badge/Zustand-5-764ABC?style=flat-square)
![Zod](https://img.shields.io/badge/Zod-3.24-3E67B1?style=flat-square)

**Tools**

![Turborepo](https://img.shields.io/badge/Turborepo-2.8-EF4444?style=flat-square&logo=turborepo&logoColor=white)
![pnpm](https://img.shields.io/badge/pnpm-9-F69220?style=flat-square&logo=pnpm&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?style=flat-square&logo=docker&logoColor=white)

---

## Environment Variables

Copy `.env.example` to `.env.local` in `apps/studio/`:

```bash
# Required
NEO4J_PASSWORD=novanetpassword

# Optional (for AI features)
ANTHROPIC_API_KEY=sk-ant-xxx
```

---

## Target Application

NovaNet powers **[QR Code AI](https://qrcode-ai.com)** — generating native content across 200+ locales for the world's most advanced QR code platform.

---

## License

This project is **proprietary software** owned by [SuperNovae Studio](https://github.com/supernovae-st). All rights reserved.

---

<div align="center">

**[SuperNovae Studio](https://github.com/supernovae-st)**

</div>
