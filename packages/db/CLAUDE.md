# @novanet/db

Database infrastructure package for NovaNet.

## Overview

This package contains:
- **docker-compose.yml**: Neo4j container configuration
- **seed/**: Initial seed data (Cypher scripts)
- **migrations/**: Schema migrations
- **queries/**: Reusable Cypher queries

## Commands

```bash
# From monorepo root (recommended)
pnpm infra:up      # Start Neo4j
pnpm infra:down    # Stop Neo4j
pnpm infra:logs    # View logs
pnpm infra:seed    # Seed database
pnpm infra:reset   # Reset (down + up + seed)

# Or from this package
pnpm up
pnpm down
pnpm logs
pnpm seed
pnpm reset
```

## Neo4j Credentials

- **Browser**: http://localhost:7474
- **Bolt**: bolt://localhost:7687
- **User**: `neo4j`
- **Password**: `novanetpassword`

## Cypher Shell

```bash
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword
```
