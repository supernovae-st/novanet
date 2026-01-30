# NovaNet HQ

Development workspace with Git submodules for NovaNet ecosystem.

---

## Overview

NovaNet uses Neo4j to orchestrate **native content generation** (NOT translation) across 200+ locales.

**Target Application**: QR Code AI (https://qrcode-ai.com)

```
CRITICAL: Generation, NOT Translation

Source -> Translate -> Target        <-- WRONG
Concept (invariant) -> Generate natively -> ConceptL10n (local)  <-- RIGHT
```

---

## Submodule Architecture

```
novanet-hq/                          # This repo (orchestrator)
├── packages/
│   ├── core/                        # Submodule → supernovae-st/novanet-core
│   ├── db/                          # Submodule → supernovae-st/novanet-db
│   └── cli/                         # Submodule → supernovae-st/novanet-cli
├── apps/
│   └── studio/                      # Submodule → supernovae-st/novanet-studio
└── docs/                            # Local docs & plans
```

Each package/app is a **separate Git repo** linked as a submodule.

---

## Submodule Commands

```bash
# Clone with submodules
git clone --recurse-submodules git@github.com:supernovae-st/novanet-hq.git

# Initialize submodules (if cloned without --recurse-submodules)
npm run submodules:init

# Update submodules to latest
npm run submodules:update

# Work in a submodule
cd packages/core
git checkout main
git pull
# make changes, commit, push
```

---

## Workspace Commands

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
cd packages/db && ./seed.sh
```

---

## Repos

| Package | Repo | Description |
|---------|------|-------------|
| @novanet/core | [novanet-core](https://github.com/supernovae-st/novanet-core) | Types, schemas, filters |
| @novanet/db | [novanet-db](https://github.com/supernovae-st/novanet-db) | Neo4j infrastructure |
| @novanet/cli | [novanet-cli](https://github.com/supernovae-st/novanet-cli) | Dev tools |
| @novanet/studio | [novanet-studio](https://github.com/supernovae-st/novanet-studio) | Web visualization |

---

## Neo4j

- **Browser**: http://localhost:7474
- **Bolt**: bolt://localhost:7687
- **Credentials**: `neo4j` / `novanetpassword`

---

## Quick Start

```bash
# 1. Clone with submodules
git clone --recurse-submodules git@github.com:supernovae-st/novanet-hq.git
cd novanet-hq

# 2. Install dependencies
npm install

# 3. Start Neo4j + seed
npm run infra:up
cd packages/db && ./seed.sh

# 4. Start development
npm run dev    # → http://localhost:3000
```
