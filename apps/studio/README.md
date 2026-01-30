<div align="center">

# 🎨 NovaNet Studio

**Interactive 2D/3D knowledge graph visualization with React Flow and force-graph**

[![← HQ](https://img.shields.io/badge/←_HQ-64748b?style=flat-square)](https://github.com/supernovae-st/novanet-hq)
[![🏢 HQ](https://img.shields.io/badge/🏢_HQ-8b5cf6?style=flat-square)](https://github.com/supernovae-st/novanet-hq)
[![📦 Core →](https://img.shields.io/badge/📦_Core_→-6366f1?style=flat-square)](https://github.com/supernovae-st/novanet-core)

<br>

[![Next.js](https://img.shields.io/badge/Next.js-15-black?style=flat-square&logo=next.js&logoColor=white)](https://nextjs.org/)
[![React](https://img.shields.io/badge/React-19-61DAFB?style=flat-square&logo=react&logoColor=black)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.7-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind](https://img.shields.io/badge/Tailwind-3.x-06b6d4?style=flat-square&logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)

---

*Part of the [NovaNET Ecosystem](https://github.com/supernovae-st/novanet-hq)*

</div>

---

## 📋 Overview

NovaNet Studio provides interactive visualization for the NovaNet knowledge graph:

- **Dual View Mode** - 2D (React Flow) + 3D (force-graph) visualization
- **AI-Powered Search** - Natural language to Cypher via Claude
- **Keyboard-First** - Full navigation with shortcuts
- **Filter Presets** - Quick views for 19k+ nodes
- **DX-First Design** - Copy anything, inspect everything

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔀 **2D/3D Toggle** | Switch views with `V` key |
| 🤖 **AI Chat** | Ask questions in natural language (`⌘J`) |
| ⌨️ **Keyboard Nav** | Command palette (`⌘K`), presets (`1-9`) |
| 🎯 **Quick Views** | 10 built-in filter presets |
| 📋 **Copy Anything** | JSON/TypeScript/YAML export |
| 🌍 **Locale Filter** | Browse by language/region |

## 🚀 Quick Start

```bash
# Install dependencies
npm install

# Configure environment
cp .env.example .env.local
# Edit with Neo4j and Anthropic credentials

# Start development
npm run dev
```

Open [http://localhost:3000](http://localhost:3000)

## ⌨️ Keyboard Shortcuts

### Navigation
| Key | Action |
|-----|--------|
| `⌘K` | Command palette |
| `⌘J` | AI Chat |
| `V` | Toggle 2D/3D |
| `F` | Fit view |
| `?` | Show shortcuts |

### Quick Views
| Key | View |
|-----|------|
| `1` | Project Structure |
| `2` | Generation Chain |
| `3` | Locale Knowledge |
| `4` | Concept Network |
| `5` | Prompts & Rules |
| `6` | SEO & GEO |
| `7` | High Priority |
| `8` | Realtime Content |
| `0` | All Nodes |

### Layout
| Key | Layout |
|-----|--------|
| `⇧H` | Horizontal |
| `⇧V` | Vertical |
| `⇧D` | Dagre |
| `⇧R` | Radial |
| `⇧F` | Force |

## 📁 Structure

```
src/
├── app/                # Next.js App Router
│   ├── api/chat/       # Claude AI endpoint
│   └── page.tsx        # Main visualization
├── components/
│   ├── chat/           # AI chat interface
│   ├── graph/          # React Flow + force-graph
│   ├── sidebar/        # Filters, details panel
│   └── ui/             # Base components
├── config/             # Presets, shortcuts, node types
├── hooks/              # Custom React hooks
├── lib/                # Utilities
├── stores/             # Zustand state
└── types/              # TypeScript types
```

## 🛠️ Tech Stack

| Technology | Purpose |
|------------|---------|
| **Next.js 15** | App Router + Turbopack |
| **React 19** | UI framework |
| **TypeScript 5.7** | Type safety |
| **Tailwind CSS** | Styling |
| **Zustand 5** | State management |
| **@xyflow/react** | 2D graph |
| **react-force-graph-3d** | 3D graph |
| **neo4j-driver** | Database client |
| **@anthropic-ai/sdk** | AI integration |

## 🧪 Development

```bash
npm run dev          # Start dev server
npm run build        # Production build
npm run lint         # ESLint
npm run type-check   # TypeScript check
npm test             # Run tests
```

## 🔗 Related

- [novanet-hq](https://github.com/supernovae-st/novanet-hq) - Dev workspace
- [novanet-core](https://github.com/supernovae-st/novanet-core) - Core library
- [novanet-infra](https://github.com/supernovae-st/novanet-infra) - Docker

---

<div align="center">

**Navigate**

[![← HQ](https://img.shields.io/badge/←_HQ-64748b?style=flat-square)](https://github.com/supernovae-st/novanet-hq)
[![🏢 HQ](https://img.shields.io/badge/🏢_HQ-8b5cf6?style=flat-square)](https://github.com/supernovae-st/novanet-hq)
[![📦 Core →](https://img.shields.io/badge/📦_Core_→-6366f1?style=flat-square)](https://github.com/supernovae-st/novanet-core)

---

[🏢 HQ](https://github.com/supernovae-st/novanet-hq) · [🎨 Studio](./README.md) · [📦 Core](https://github.com/supernovae-st/novanet-core) · [🐳 Infra](https://github.com/supernovae-st/novanet-infra)

---

**Part of [SuperNovae Studio](https://github.com/supernovae-st)** · [NovaNET Public](https://github.com/supernovae-st/novanet)

</div>
