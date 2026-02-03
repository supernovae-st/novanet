# Architecture Overview

NovaNet is a **self-describing context graph** for native content generation.

## System Architecture

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {
  'primaryColor': '#6366f1',
  'secondaryColor': '#06b6d4',
  'tertiaryColor': '#10b981',
  'lineColor': '#64748b'
}}}%%
flowchart TB
    subgraph MONO["NovaNet Monorepo"]
        direction TB

        subgraph PACKAGES["packages/"]
            CORE["@novanet/core\nTypes · Schemas · Zod"]
            DB["@novanet/db\nDocker · Seeds · Migrations"]
        end

        subgraph APPS["apps/"]
            STUDIO["@novanet/studio\nNext.js 16 · React 19\nGraph Visualization"]
        end

        subgraph TOOLS["tools/"]
            RUST["novanet CLI\nRust · 13 commands\nTUI · 396 tests"]
        end
    end

    YAML[("YAML Models\n44 node definitions")]
    NEO4J[("Neo4j 5.26\n~19,000 nodes")]

    YAML --> RUST
    RUST -->|generates| CORE
    RUST -->|generates| DB
    CORE --> STUDIO
    DB --> NEO4J
    STUDIO --> NEO4J
    RUST --> NEO4J

    style CORE fill:#06b6d4,color:#fff
    style DB fill:#10b981,color:#fff
    style STUDIO fill:#8b5cf6,color:#fff
    style RUST fill:#DEA584,color:#fff
    style NEO4J fill:#018bff,color:#fff
    style YAML fill:#f59e0b,color:#fff
```

## Core Principle: Generation, NOT Translation

```
Traditional:  Source → Translate → Target        ❌
NovaNet:      Concept → Generate → L10n          ✅
```

Content is generated natively per locale from invariant semantic concepts, not translated from a source language.

## Package Responsibilities

| Package | Responsibility | Language |
|---------|----------------|----------|
| **@novanet/core** | Types, Zod schemas, filter API | TypeScript |
| **@novanet/db** | Neo4j Docker, seeds, migrations | Cypher |
| **@novanet/studio** | Web visualization, AI chat | TypeScript/React |
| **tools/novanet** | CLI, TUI, generators, queries | Rust |

## Data Flow

```mermaid
%%{init: {'theme': 'base'}}%%
sequenceDiagram
    participant Y as YAML Models
    participant R as Rust CLI
    participant N as Neo4j
    participant S as Studio

    Note over Y,R: Build Time
    Y->>R: novanet schema generate
    R->>R: Parse YAML
    R->>R: Generate TypeScript
    R->>R: Generate Cypher seeds

    Note over R,N: Seed Time
    R->>N: novanet db seed
    N->>N: Create constraints
    N->>N: Create nodes/relations

    Note over S,N: Runtime
    S->>N: Cypher queries
    N-->>S: Graph data
    S->>S: Visualize
```

## Source of Truth

**YAML is the single source of truth:**

```
packages/core/models/
├── _index.yaml              # Schema registry
├── organizing-principles.yaml  # Meta-graph definition
├── relations.yaml           # 50 relationship types
└── nodes/
    ├── global/              # 15 global nodes
    ├── project/             # 21 project nodes
    └── shared/              # 8 shared nodes
```

All other artifacts (TypeScript, Cypher, Mermaid) are generated from YAML.

## Key Technologies

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Graph DB** | Neo4j 5.26 + APOC | Knowledge storage |
| **Backend** | Rust (neo4rs, tokio) | CLI, generators, queries |
| **Frontend** | Next.js 16, React 19 | Web visualization |
| **State** | Zustand + Zod | Client state management |
| **Build** | Turborepo + pnpm | Monorepo orchestration |

## Boundary Rule (v9)

```
TypeScript = Types + Presentation
Rust       = Runtime + Generation
```

- **TypeScript**: Generates type artifacts, UI components
- **Rust**: Executes all runtime operations (queries, CRUD, validation)

## Related Documentation

- [Ontology v9](./ontology-v9.md) — Meta-graph structure
- [Meta-Graph](./meta-graph.md) — Classification system
- [Rust CLI](./rust-cli.md) — Command reference
