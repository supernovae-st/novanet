<div align="center">

# NovaNET

**Knowledge graph localization orchestrator for native multilingual content generation.**

Turn your content into a living knowledge graph. Generate native-quality translations at scale.

[![Building](https://img.shields.io/badge/Status-Building-f59e0b?style=flat-square)](https://github.com/supernovae-st/novanet)
[![Neo4j](https://img.shields.io/badge/Neo4j-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://typescriptlang.org)

<br>

**Part of [SuperNovae Studio](https://github.com/supernovae-st)**

[Why NovaNET?](#why-novanet) · [Features](#features) · [Architecture](#architecture) · [Ecosystem](#ecosystem)

</div>

---

## Why NovaNET?

Traditional translation pipelines lose context. They translate words, not meaning.

NovaNET builds a **knowledge graph** of your content — understanding relationships, terminology, and domain concepts — then generates native-quality content that preserves meaning across languages.

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart LR
    accTitle: Traditional vs NovaNET Translation
    accDescr: Shows how NovaNET preserves context through knowledge graphs

    classDef old fill:#64748b,stroke:#475569,stroke-width:2px,color:#ffffff
    classDef process fill:#6366f1,stroke:#4f46e5,stroke-width:2px,color:#ffffff
    classDef success fill:#10b981,stroke:#059669,stroke-width:2px,color:#ffffff

    A[Source Content]:::old --> B[Knowledge Graph]:::process
    B --> C[Context Preserved]:::process
    C --> D[Native Output]:::success
```

| Problem | NovaNET Solution |
|---------|------------------|
| Lost context in translation | Knowledge graph preserves relationships |
| Inconsistent terminology | Centralized concept mapping |
| Manual locale management | Automated multi-locale generation |
| Generic AI translations | Domain-aware native content |

---

## Features

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart TB
    accTitle: NovaNET Features
    accDescr: Core capabilities of the NovaNET platform

    classDef feature fill:#6366f1,stroke:#4f46e5,stroke-width:2px,color:#ffffff
    classDef benefit fill:#10b981,stroke:#059669,stroke-width:2px,color:#ffffff

    subgraph GRAPH[" Knowledge Graph "]
        G1[Neo4j Backend]:::feature
        G2[Concept Mapping]:::feature
        G3[Relationship Tracking]:::feature
    end

    subgraph GEN[" Content Generation "]
        C1[Native Quality]:::feature
        C2[200+ Locales]:::feature
        C3[SEO Optimized]:::feature
    end

    subgraph VIS[" Visualization "]
        V1[2D/3D Graph View]:::feature
        V2[Interactive Explorer]:::feature
        V3[Filter System]:::feature
    end

    GRAPH --> OUTPUT[Multilingual Content at Scale]:::benefit
    GEN --> OUTPUT
    VIS --> OUTPUT

    style GRAPH fill:#dbeafe,stroke:#3b82f6,stroke-width:2px,color:#1e3a8a
    style GEN fill:#d1fae5,stroke:#10b981,stroke-width:2px,color:#064e3b
    style VIS fill:#e0e7ff,stroke:#6366f1,stroke-width:2px,color:#312e81
```

| Feature | Description |
|---------|-------------|
| **Knowledge Graph** | Neo4j-powered content relationships |
| **Native Generation** | Context-aware multilingual content |
| **Visual Explorer** | Interactive 2D/3D graph visualization |
| **Filter System** | TypeScript-based query filters |
| **200+ Locales** | Full international coverage |

---

## Architecture

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart TB
    accTitle: NovaNET Architecture
    accDescr: System components and data flow

    classDef core fill:#6366f1,stroke:#4f46e5,stroke-width:2px,color:#ffffff
    classDef studio fill:#10b981,stroke:#059669,stroke-width:2px,color:#ffffff
    classDef infra fill:#06b6d4,stroke:#0891b2,stroke-width:2px,color:#ffffff
    classDef data fill:#8b5cf6,stroke:#7c3aed,stroke-width:2px,color:#ffffff

    subgraph REPOS[" Ecosystem "]
        CORE[novanet-core<br/>Models & Types]:::core
        STUDIO[novanet-studio<br/>Visualization]:::studio
        INFRA[novanet-infra<br/>Docker Configs]:::infra
    end

    NEO4J[(Neo4j)]:::data

    CORE --> NEO4J
    STUDIO --> CORE
    INFRA --> NEO4J

    style REPOS fill:#f8fafc,stroke:#64748b,stroke-width:2px,color:#334155
```

---

## Ecosystem

| Repository | Description | Status |
|------------|-------------|--------|
| [**novanet**](https://github.com/supernovae-st/novanet) | Public showcase (this repo) | Public |
| [**novanet-core**](https://github.com/supernovae-st/novanet-core) | Neo4j models & TypeScript types | Private |
| [**novanet-studio**](https://github.com/supernovae-st/novanet-studio) | Interactive graph visualization | Private |
| [**novanet-infra**](https://github.com/supernovae-st/novanet-infra) | Docker configurations | Private |
| [**novanet-hq**](https://github.com/supernovae-st/novanet-hq) | Development workspace | Private |

---

## Links

| Resource | Description |
|----------|-------------|
| [Vision](./VISION.md) | Where we're going |
| [Ecosystem](./ECOSYSTEM.md) | All repositories |

---

<div align="center">

**Built by [Thibaut MÉLEN](https://github.com/ThibautMelen) & [Nicolas CELLA](https://github.com/NicolasCELLA) at [SuperNovae Studio](https://supernovae.studio)**

[![SuperNovae Studio](https://img.shields.io/badge/SuperNovae-Studio-8b5cf6?style=flat-square&logo=github)](https://github.com/supernovae-st)
[![Hub](https://img.shields.io/badge/Hub-64748b?style=flat-square)](https://github.com/supernovae-st/hub)

</div>
