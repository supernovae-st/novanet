<div align="center">

# 🪽 NovaNet Studio

**Interactive 2D/3D knowledge graph visualization with AI-powered queries**

[![Next.js](https://img.shields.io/badge/Next.js-16-000000?style=flat-square&logo=next.js&logoColor=white)](https://nextjs.org/)
[![React](https://img.shields.io/badge/React-19-61DAFB?style=flat-square&logo=react&logoColor=black)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind](https://img.shields.io/badge/Tailwind-3.4-06b6d4?style=flat-square&logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)

---

*Part of the [🪽 NovaNet Monorepo](../../README.md)*

</div>

---

## Overview

NovaNet Studio provides interactive visualization for the NovaNet knowledge graph:

- **Dual View Mode** — 2D (React Flow) + 3D (force-graph) visualization
- **AI-Powered Search** — Natural language to Cypher via Claude
- **Keyboard-First** — Full navigation with 40+ shortcuts
- **Filter Presets** — Quick views for ~19,000 nodes
- **DX-First Design** — Copy anything (JSON/TS/YAML), inspect everything

---

## Features

| Feature | Description |
|---------|-------------|
| **2D/3D Toggle** | Switch views with `V` key |
| **AI Chat** | Ask questions in natural language (`⌘J`) |
| **Keyboard Nav** | Command palette (`⌘K`), presets (`1-9`) |
| **Quick Views** | 10 built-in filter presets |
| **Copy Anything** | JSON/TypeScript/YAML export |
| **Locale Filter** | Browse by language/region |

---

## Quick Start

```bash
# From monorepo root
pnpm install

# Configure environment
cp apps/studio/.env.example apps/studio/.env.local
# Set NEO4J_PASSWORD and ANTHROPIC_API_KEY

# Start Neo4j + seed
pnpm infra:up && pnpm infra:seed

# Start development
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000)

---

## Keyboard Shortcuts

### Navigation

| Key | Action |
|-----|--------|
| `⌘K` | Command palette |
| `⌘J` | AI Chat |
| `V` | Toggle 2D/3D |
| `F` | Fit view |
| `?` | Show shortcuts |

### Quick Views (Presets)

| Key | View |
|-----|------|
| `1` | Project Overview |
| `2` | Full Graph |
| `3` | Core + Concepts |
| `4` | All Locales |
| `5` | Concepts |
| `6` | Current Locale |
| `7` | Locale + Expressions |
| `8` | Locale Knowledge |
| `9` | Expressions |
| `0` | Clear Filters |

### Layout

| Key | Layout |
|-----|--------|
| `⇧H` | Horizontal |
| `⇧V` | Vertical |
| `⇧D` | Dagre |
| `⇧R` | Radial |
| `⇧F` | Force |

---

## Structure

```
studio/
├── src/
│   ├── app/                # Next.js App Router
│   │   ├── api/chat/       # Claude AI endpoint
│   │   ├── api/graph/      # Neo4j graph endpoints
│   │   └── page.tsx        # Main visualization
│   ├── components/
│   │   ├── chat/           # AI chat interface
│   │   ├── graph/          # React Flow + force-graph
│   │   ├── sidebar/        # Filters, details panel
│   │   └── ui/             # Base components (Radix UI)
│   ├── config/             # Presets, shortcuts, node types
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Utilities (auth, rate-limit, neo4j)
│   ├── stores/             # Zustand state management
│   └── types/              # TypeScript types
└── .env.example            # Environment template
```

---

## Tech Stack

| Technology | Purpose |
|------------|---------|
| **Next.js 16** | App Router + Turbopack |
| **React 19** | UI framework |
| **TypeScript 5.9** | Type safety |
| **Tailwind CSS** | Styling |
| **Zustand 5** | State management (persist + immer) |
| **@xyflow/react** | 2D graph visualization |
| **react-force-graph-3d** | 3D graph visualization |
| **neo4j-driver** | Database client |
| **@anthropic-ai/sdk** | AI integration |

---

## Development

```bash
pnpm dev                              # Start dev server
pnpm build --filter=@novanet/studio   # Production build
pnpm lint --filter=@novanet/studio    # ESLint
pnpm type-check --filter=@novanet/studio  # TypeScript check
pnpm test --filter=@novanet/studio    # Run tests
```

---

## Related Packages

| Package | Description |
|---------|-------------|
| [@novanet/core](../../packages/core/) | Types, schemas, filters |
| [@novanet/db](../../packages/db/) | Neo4j Docker infrastructure |

---

<div align="center">

**[🪽 NovaNet](../../README.md)** · [SuperNovae Studio](https://github.com/supernovae-st)

</div>
