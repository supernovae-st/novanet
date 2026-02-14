<div align="center">

# NovaNet DB

**Neo4j infrastructure for NovaNet knowledge graph**

[![Neo4j](https://img.shields.io/badge/Neo4j-5.26-018bff?style=flat-square&logo=neo4j&logoColor=white)](https://neo4j.com/)
[![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?style=flat-square&logo=docker&logoColor=white)](https://docs.docker.com/compose/)
[![APOC](https://img.shields.io/badge/APOC-5.26-10b981?style=flat-square)](https://neo4j.com/labs/apoc/)

---

*Part of the [NovaNet Monorepo](../../README.md)*

</div>

---

## Overview

This package provides Neo4j infrastructure for the NovaNet knowledge graph:

- **Docker Compose** - Neo4j 5.26 + APOC configuration
- **Seed Scripts** - Initial data (constraints + sample data)
- **Cypher Queries** - Reusable query templates

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
+-- docker-compose.yml    # Neo4j 5.26 + APOC
+-- seed.sh               # Seed runner script
+-- seed/                 # Cypher seed files
|   +-- 00-constraints.cypher
|   +-- *.cypher
+-- README.md             # This file
```

---

## Graph Schema (v0.12.4)

| Realm | Nodes | Description |
|-------|-------|-------------|
| **Shared** | 40 | Config, locale, geography, knowledge atoms (READ-ONLY) |
| **Org** | 21 | Organization structure, content, generation |

Total: **61 node types**, **156 arc types**, **~200 schema-nodes** (Realm/Layer/Class/Trait/ArcFamily/ArcClass)

---

## Locale Knowledge Architecture (v11.7)

```
Locale {key: "fr-FR"}
    |
    |   LOCALE LAYER
    +--[:HAS_VOICE]----------> LocaleVoice
    +--[:HAS_GRAMMAR]--------> LocaleGrammar
    +--[:HAS_FORMATS]--------> LocaleFormats
    |
    |   KNOWLEDGE LAYER
    +--[:HAS_TERMS]----------> TermSet
    +--[:HAS_EXPRESSIONS]----> ExpressionSet
    +--[:HAS_PATTERNS]-------> PatternSet
    +--[:HAS_CULTURE]--------> CultureSet
    +--[:HAS_TABOOS]---------> TabooSet
    +--[:HAS_AUDIENCE]-------> AudienceSet
```

---

## Useful Queries

### Count nodes by type

```cypher
MATCH (n)
RETURN labels(n)[0] AS label, count(*) AS count
ORDER BY count DESC;
```

### Load Locale with Knowledge (v11.7)

```cypher
MATCH (l:Locale {key: $locale})
// Locale layer
OPTIONAL MATCH (l)-[:HAS_VOICE]->(voice:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_GRAMMAR]->(grammar:LocaleGrammar)
OPTIONAL MATCH (l)-[:HAS_FORMATS]->(formats:LocaleFormats)
// Knowledge layer
OPTIONAL MATCH (l)-[:HAS_TERMS]->(terms:TermSet)
OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(expr:ExpressionSet)
OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(pat:PatternSet)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(cult:CultureSet)
OPTIONAL MATCH (l)-[:HAS_TABOOS]->(taboo:TabooSet)
OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(aud:AudienceSet)
RETURN l, voice, grammar, formats, terms, expr, pat, cult, taboo, aud
```

### Spreading Activation

```cypher
MATCH (e:Entity {key: "tier-pro"})-[r:SEMANTIC_LINK*1..2]->(e2:Entity)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH e2, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
MATCH (e2)-[:HAS_CONTENT]->(el:EntityContent)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN e2.key, el.title, activation
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

**[NovaNet](../../README.md)** - [SuperNovae Studio](https://github.com/supernovae-st)

</div>
