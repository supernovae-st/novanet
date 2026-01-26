<div align="center">

# 🏢 NovaNet HQ

**Development workspace for the NovaNet ecosystem**

[![Private](https://img.shields.io/badge/repo-private-red?style=for-the-badge)](https://github.com/supernovae-ai/novanet-hq)

</div>

---

## 📋 Overview

Central development workspace that orchestrates the NovaNet monorepo with npm workspaces, Claude Code configurations, and shared tooling.

## 🗂️ Ecosystem

| Repo | Description | Status |
|------|-------------|--------|
| [novanet](https://github.com/supernovae-ai/novanet) | 🏠 Public showcase | Public |
| **novanet-hq** | 🏢 Dev workspace (you are here) | Private |
| [novanet-core](https://github.com/supernovae-ai/novanet-core) | 📦 Neo4j models & TypeScript | Private |
| [novanet-studio](https://github.com/supernovae-ai/novanet-studio) | 🎨 Graph visualization | Private |
| [novanet-infra](https://github.com/supernovae-ai/novanet-infra) | 🐳 Docker configs | Private |

## 🚀 Quick Start

```bash
# Clone the workspace
git clone git@github.com:supernovae-ai/novanet-hq.git
cd novanet-hq

# Clone sub-repositories
git clone git@github.com:supernovae-ai/novanet-core.git core
git clone git@github.com:supernovae-ai/novanet-studio.git studio
git clone git@github.com:supernovae-ai/novanet-infra.git infra

# Install dependencies
npm install

# Start infrastructure
npm run infra:up

# Seed database
cd core/neo4j && ./seed.sh && cd ../..

# Start development
npm run dev
```

## 📁 Structure

```
novanet-hq/
├── .claude/              # Claude Code configuration
│   ├── agents/           # Custom agents
│   ├── project-info.json # Workspace metadata
│   └── settings.json     # Claude settings
├── .mcp.json             # MCP server configs (Neo4j)
├── CLAUDE.md             # AI assistant instructions
├── package.json          # npm workspaces orchestration
├── docs/                 # Planning documents
│   └── plans/
├── core/                 # → novanet-core (git clone)
├── studio/               # → novanet-studio (git clone)
└── infra/                # → novanet-infra (git clone)
```

## 🛠️ Commands

```bash
# Development
npm run dev              # Start studio dev server
npm run build            # Build studio
npm run lint             # Lint all workspaces
npm run type-check       # Type check all workspaces
npm run test             # Test all workspaces

# Infrastructure
npm run infra:up         # Start Neo4j
npm run infra:down       # Stop Neo4j
npm run infra:logs       # View logs
```

## 🔧 Configuration

### Claude Code

The `.claude/` directory contains AI assistant configurations:

- **agents/** - Custom Claude agents for code review and Neo4j architecture
- **project-info.json** - Workspace and skill definitions
- **settings.json** - Claude Code settings

### MCP Servers

`.mcp.json` configures Model Context Protocol servers:

```json
{
  "mcpServers": {
    "neo4j": {
      "command": "uvx",
      "args": ["mcp-neo4j-cypher"]
    }
  }
}
```

## 📄 License

Private - SuperNovae Studio
