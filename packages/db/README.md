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

## Graph Schema (v9.0.0)

| Realm | Nodes | Description |
|-------|-------|-------------|
| **🌍 Global** | 15 | Locale + 14 LocaleKnowledge nodes |
| **📦 Project** | 21 | Project structure, content, generation |
| **🎯 Shared** | 8 | SEO/GEO targeting nodes |

Total: **44 node types**, **50 relationships**, **109 meta-nodes** (Realm/Layer/Kind/Trait/EdgeFamily/EdgeKind)

---

## Locale Knowledge Architecture

```
Locale {key: "fr-FR"}
    │
    ├──[:HAS_IDENTITY]──▶ LocaleIdentity
    ├──[:HAS_VOICE]─────▶ LocaleVoice
    ├──[:HAS_CULTURE]───▶ LocaleCulture ──▶ LocaleCultureReferences
    ├──[:HAS_MARKET]────▶ LocaleMarket
    └──[:HAS_LEXICON]───▶ LocaleLexicon ──[:HAS_EXPRESSION]──▶ Expression
```

---

## Useful Queries

### Count nodes by type

```cypher
MATCH (n)
RETURN labels(n)[0] AS label, count(*) AS count
ORDER BY count DESC;
```

### Load Locale with Knowledge

```cypher
MATCH (l:Locale {key: $locale})
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
RETURN l, li, lv, lc, lm, ll, collect(e) AS expressions
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
