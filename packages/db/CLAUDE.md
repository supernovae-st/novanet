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
# Start Neo4j
npm run up

# Stop Neo4j
npm run down

# View logs
npm run logs

# Seed database
npm run seed

# Reset (wipe + reseed)
npm run reset
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
