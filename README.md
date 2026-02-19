<div align="center">

# 🧠 supernovae-agi

**AI-Powered Content Generation System**

NovaNet (brain) + Nika (body) — Generate culturally-native content across 200+ locales

[![Rust](https://img.shields.io/badge/Rust-1.84-DEA584?style=flat-square&logo=rust&logoColor=white)](https://rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://typescriptlang.org)
[![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com)
[![Tests](https://img.shields.io/badge/tests-1811_passing-success?style=flat-square)](https://github.com/supernovae-st/supernovae-agi)

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
| 61 node types, 182 arcs | Native generation per locale | Interactive 2D visualization | Claude API for natural language queries |
| Neo4j with APOC | Locale knowledge layer | React Flow + ELK.js layouts | Cypher generation from text |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  supernovae-agi ARCHITECTURE                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────┐         MCP Protocol        ┌─────────────────┐   │
│  │     NOVANET         │◄──────────────────────────►│      NIKA       │   │
│  │     (Brain)         │                             │     (Body)      │   │
│  │     v0.14.0         │                             │     v0.4.0      │   │
│  ├─────────────────────┤                             ├─────────────────┤   │
│  │ • Knowledge Graph   │    novanet_generate         │ • YAML Workflows│   │
│  │ • 61 NodeClasses    │    novanet_describe         │ • 5 Verbs       │   │
│  │ • 182 ArcClasses    │    novanet_traverse         │ • DAG Execution │   │
│  │ • Neo4j + Rust TUI  │◄────────────────────────────│ • rig-core LLM  │   │
│  │ • 1194 tests        │                             │ • 617 tests     │   │
│  └─────────────────────┘                             └─────────────────┘   │
│         │                                                     │            │
│         ▼                                                     ▼            │
│  novanet-dev/                                          nika-dev/           │
│  └── tools/novanet/                                    └── tools/nika/     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Projects

| Project | Version | Description | Tests |
|---------|---------|-------------|-------|
| **NovaNet** | v0.14.0 | Knowledge graph + MCP Server | 1194 |
| **Nika** | v0.4.0 | YAML workflow engine + MCP Client | 617 |

---

## Quick Start

**Prerequisites**

![Node.js](https://img.shields.io/badge/Node.js-≥20-339933?style=flat-square&logo=node.js&logoColor=white)
![pnpm](https://img.shields.io/badge/pnpm-≥9-F69220?style=flat-square&logo=pnpm&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Required-2496ED?style=flat-square&logo=docker&logoColor=white)

```bash
# 1. Clone the repository
git clone git@github.com:supernovae-st/supernovae-agi.git
cd supernovae-agi/novanet-dev

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

## Workspace Structure

```
supernovae-agi/
├── ROADMAP.md                 # Master roadmap (MVP tracking)
├── CHANGELOG.md               # Combined changelog
├── .claude/                   # Claude Code DX (skills, hooks, rules)
│
├── novanet-dev/               # NovaNet (Brain) — git submodule
│   ├── packages/
│   │   ├── core/              # @novanet/core — types, schemas, filters
│   │   └── db/                # @novanet/db — Neo4j infrastructure
│   ├── tools/novanet/         # Rust CLI + TUI (1194 tests)
│   └── apps/studio/           # Web visualization (Next.js)
│
└── nika-dev/                  # Nika (Body) — git submodule
    └── tools/nika/            # Rust CLI (617 tests)
        └── src/
            ├── ast/           # YAML → Rust structs
            ├── mcp/           # MCP client (rmcp v0.16)
            ├── runtime/       # Execution engine
            ├── provider/      # rig-core LLM providers
            └── tui/           # Terminal UI
```

---

## Packages

### NovaNet

| Package | Version | Description |
|---------|---------|-------------|
| **@novanet/core** | `0.14.0` | Types, Zod schemas, NovaNetFilter API, Cypher generators |
| **@novanet/db** | `0.14.0` | Docker Compose for Neo4j, Cypher seeds, migrations |
| **@novanet/studio** | `0.14.0` | Interactive graph visualization with AI chat |
| **tools/novanet** | `0.14.0` | Rust CLI + TUI for schema generation, validation, queries |
| **tools/novanet-mcp** | `0.4.0` | MCP Server (7 tools) |

### Nika

| Package | Version | Description |
|---------|---------|-------------|
| **tools/nika** | `0.4.0` | Rust CLI + TUI for YAML workflow execution |

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

### NovaNet CLI (tools/novanet)

```bash
cd novanet-dev/tools/novanet

cargo run -- schema generate        # Regenerate all artifacts
cargo run -- schema validate        # Validate YAML coherence
cargo run -- blueprint              # Schema-graph visualization
cargo run -- tui                    # Interactive TUI
```

### Nika CLI (tools/nika)

```bash
cd nika-dev/tools/nika

cargo run -- run workflow.yaml      # Execute workflow
cargo run -- validate workflow.yaml # Validate only
cargo run -- tui workflow.yaml      # Interactive TUI
cargo run -- trace list             # List traces
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

NovaNet models content as a knowledge graph with **2 Realms** and **10 Layers** (v0.13.0):

| Realm | Layers | Description |
|-------|--------|-------------|
| **Shared** | config, locale, geography, knowledge | Universal definitions + locale knowledge (READ-ONLY) — 40 nodes |
| **Org** | config, foundation, structure, semantic, instruction, output | Organization-specific content — 21 nodes |

**v11.5 changes:** Locale definition moved to shared/config. SEO/GEO nodes consolidated to shared/knowledge.

Each node type has a **Trait** (defined / authored / imported / generated / retrieved) and arcs are classified by **ArcFamily**.

> **v0.13.0 ADR-024: Data Origin traits** — Trait now answers "WHERE does data come from?" (defined = human-created once, authored = human-written per locale, imported = external data brought in, generated = LLM output, retrieved = external API snapshots)

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
│  Mode: Graph  │  61 nodes  │  182 arcs  │  Zoom: 100%  │  Locale: fr-FR │
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
