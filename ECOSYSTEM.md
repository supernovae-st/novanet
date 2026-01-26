# Ecosystem

> All repositories that make up the NovaNET universe.

---

## Architecture Overview

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'lineColor': '#64748b'}}}%%
flowchart TB
    accTitle: NovaNET Ecosystem Relationships
    accDescr: Diagram showing the relationship between core repositories, infrastructure components, and future open source contributions in the NovaNET ecosystem

    classDef core fill:#6366f1,stroke:#4f46e5,stroke-width:2px,color:#ffffff
    classDef infra fill:#06b6d4,stroke:#0891b2,stroke-width:2px,color:#ffffff
    classDef future fill:#64748b,stroke:#475569,stroke-width:2px,color:#ffffff

    subgraph Core["Core Infrastructure"]
        A[novanet-core]:::core
        B[Connection Layer]:::core
        C[Traffic Manager]:::core
    end

    subgraph Infra["Infrastructure"]
        D[Load Balancers]:::infra
        E[Service Mesh]:::infra
        F[Monitoring]:::infra
    end

    subgraph Future["Future Open Source"]
        G[Developer Tools]:::future
        H[Client Libraries]:::future
    end

    A --> B
    A --> C
    B --> D
    C --> E
    D --> F
    E --> F
    A -.-> G
    A -.-> H
```

---

## Core

| Repository | Description | Status |
|------------|-------------|--------|
| *Coming soon* | · | · |

---

## Open Source

| Repository | Description | Status |
|------------|-------------|--------|
| *Coming soon* | · | · |
