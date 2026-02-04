<div align="center">

# 🪽 NovaNet DB

**Neo4j infrastructure for NovaNet knowledge graph**

[![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com/)
[![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?style=flat-square&logo=docker&logoColor=white)](https://docs.docker.com/compose/)
[![APOC](https://img.shields.io/badge/APOC-5.26-10b981?style=flat-square)](https://neo4j.com/labs/apoc/)

---

*Part of the [🪽 NovaNet Monorepo](../../README.md)*

</div>

---

## Overview

This package provides Neo4j infrastructure for the NovaNet knowledge graph:

- **Docker Compose** — Neo4j 5.26 + APOC configuration
- **Seed Scripts** — Initial data (constraints + sample data)
- **Cypher Queries** — Reusable query templates

---

## Quick Start

```bash
# From monorepo root
pnpm infra:up      # Start Neo4j container
pnpm infra:seed    # Run seed scripts
pnpm infra:logs    # View container logs
pnpm infra:down    # Stop container
pnpm infra:reset   # Reset (down + up + seed)
```

---

## Neo4j Access

| Key | Value |
|-----|-------|
| Browser | http://localhost:7474 |
| Bolt | bolt://localhost:7687 |
| User | `neo4j` |
| Password | `novanetpassword` |

```bash
# Cypher shell
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword
```

---

## Structure

```
db/
├── docker-compose.yml    # Neo4j 5.26 + APOC
├── seed.sh               # Seed runner script
├── seed/                 # Cypher seed files
│   ├── 00-constraints.cypher
│   └── *.cypher
└── README.md             # This file
```

---

## Graph Schema (v9.9.0)

| Realm | Nodes | Description |
|-------|-------|-------------|
| **🌍 Global** | 11 | Locale + 10 tiered knowledge nodes |
| **📦 Project** | 23 | Project structure, content, generation |
| **🎯 Shared** | 8 | SEO/GEO targeting nodes |

Total: **42 node types**, **73 arc types**, **126 meta-nodes** (Realm/Layer/Kind/Trait/ArcFamily/ArcKind)

---

## Locale Knowledge Architecture (v9.9 Tiered Model)

```
Locale {key: "fr-FR"}
    │
    │   TECHNICAL TIER
    ├──[:HAS_FORMATTING]───▶ Formatting
    ├──[:HAS_SLUGIFICATION]─▶ Slugification
    ├──[:HAS_ADAPTATION]────▶ Adaptation
    │
    │   STYLE TIER
    ├──[:HAS_STYLE]─────────▶ Style
    │
    │   SEMANTIC TIER
    ├──[:HAS_TERMS]─────────▶ TermSet
    ├──[:HAS_EXPRESSIONS]───▶ ExpressionSet
    ├──[:HAS_PATTERNS]──────▶ PatternSet
    ├──[:HAS_CULTURE]───────▶ CultureSet
    ├──[:HAS_TABOOS]────────▶ TabooSet
    └──[:HAS_AUDIENCE]──────▶ AudienceSet
```

---

## Useful Queries

### Count nodes by type

```cypher
MATCH (n)
RETURN labels(n)[0] AS label, count(*) AS count
ORDER BY count DESC;
```

### Load Locale with Knowledge (v9.9 Tiered Model)

```cypher
MATCH (l:Locale {key: $locale})
// Technical tier
OPTIONAL MATCH (l)-[:HAS_FORMATTING]->(fmt:Formatting)
OPTIONAL MATCH (l)-[:HAS_SLUGIFICATION]->(slug:Slugification)
OPTIONAL MATCH (l)-[:HAS_ADAPTATION]->(adapt:Adaptation)
// Style tier
OPTIONAL MATCH (l)-[:HAS_STYLE]->(style:Style)
// Semantic tier
OPTIONAL MATCH (l)-[:HAS_TERMS]->(terms:TermSet)
OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(expr:ExpressionSet)
OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(pat:PatternSet)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(cult:CultureSet)
OPTIONAL MATCH (l)-[:HAS_TABOOS]->(taboo:TabooSet)
OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(aud:AudienceSet)
RETURN l, fmt, slug, adapt, style, terms, expr, pat, cult, taboo, aud
```

### Spreading Activation

```cypher
MATCH (c:Concept {key: "tier-pro"})-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH c2, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
MATCH (c2)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN c2.key, cl.title, activation
ORDER BY activation DESC
```

---

## Related Packages

| Package | Description |
|---------|-------------|
| [@novanet/core](../core/) | Types, schemas, filters |
| [@novanet/studio](../../apps/studio/) | Graph visualization |

---

<div align="center">

**[🪽 NovaNet](../../README.md)** · [SuperNovae Studio](https://github.com/supernovae-st)

</div>
