---
name: infra-neo4j
description: Infrastructure commands and Neo4j administration. Use for database setup, seeding, backup, or troubleshooting.
user-invocable: false
---

# Infrastructure & Neo4j Development for NovaNet

> Development environment setup and troubleshooting

## Quick Reference

```bash
# Start Neo4j
npm run infra:up

# Stop Neo4j
npm run infra:down

# View logs
npm run infra:logs

# Seed database
cd core/neo4j && ./seed.sh

# Neo4j Browser
open http://localhost:7474
```

---

## Docker Compose Configuration

### NovaNet docker-compose.yml

```yaml
# infra/docker-compose.yml
name: novanet

services:
  neo4j:
    image: neo4j:5.26.0-community
    container_name: novanet-neo4j
    ports:
      - "7474:7474"   # Browser HTTP
      - "7687:7687"   # Bolt protocol
    environment:
      - NEO4J_AUTH=neo4j/novanetpassword
      - NEO4J_PLUGINS=["apoc"]
      - NEO4J_apoc_export_file_enabled=true
      - NEO4J_apoc_import_file_enabled=true
      - NEO4J_apoc_import_file_use__neo4j__config=true
      - NEO4J_dbms_security_procedures_unrestricted=apoc.*
      - NEO4J_dbms_security_procedures_allowlist=apoc.*
    volumes:
      - novanet_neo4j_data:/data
      - novanet_neo4j_logs:/logs
      - novanet_neo4j_plugins:/plugins
      - ../core/neo4j/seed:/import
    networks:
      - novanet-network
    healthcheck:
      test: ["CMD-SHELL", "wget --no-verbose --tries=1 --spider http://localhost:7474 || exit 1"]
      interval: 10s
      timeout: 10s
      retries: 10
      start_period: 30s

volumes:
  novanet_neo4j_data:
  novanet_neo4j_logs:
  novanet_neo4j_plugins:

networks:
  novanet-network:
    driver: bridge
```

---

## Environment Variables

### Root .env (create if missing)

```bash
# .env (root)
NEO4J_URI=bolt://localhost:7687
NEO4J_USERNAME=neo4j
NEO4J_PASSWORD=novanetpassword
```

### Studio .env.local

```bash
# studio/.env.local
NEO4J_URI=bolt://localhost:7687
NEO4J_USERNAME=neo4j
NEO4J_PASSWORD=novanetpassword

# Claude API (for AI chat)
ANTHROPIC_API_KEY=sk-ant-...
```

---

## Docker Commands

### Lifecycle

```bash
# Start services (detached)
docker compose -f infra/docker-compose.yml up -d

# Stop services
docker compose -f infra/docker-compose.yml down

# Stop and remove volumes (DESTRUCTIVE)
docker compose -f infra/docker-compose.yml down -v

# Restart
docker compose -f infra/docker-compose.yml restart

# View logs
docker compose -f infra/docker-compose.yml logs -f neo4j

# Follow logs (tail)
docker compose -f infra/docker-compose.yml logs -f --tail=100 neo4j
```

### Container Access

```bash
# Shell into container
docker exec -it novanet-neo4j bash

# Run cypher-shell
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword

# Copy files to container
docker cp ./data.csv novanet-neo4j:/import/

# Copy files from container
docker cp novanet-neo4j:/data/export.json ./
```

### Health & Status

```bash
# Check container status
docker ps -a | grep novanet

# Check container health
docker inspect novanet-neo4j --format='{{.State.Health.Status}}'

# Resource usage
docker stats novanet-neo4j

# Inspect container
docker inspect novanet-neo4j
```

---

## Neo4j Browser

### Access

- **URL**: http://localhost:7474
- **Connect URL**: bolt://localhost:7687
- **Username**: neo4j
- **Password**: novanetpassword

### Useful Browser Commands

```cypher
-- Show database schema
CALL db.schema.visualization()

-- Count all nodes by label
CALL db.labels() YIELD label
MATCH (n) WHERE label IN labels(n)
RETURN label, count(n) as count
ORDER BY count DESC

-- Count all relationships
CALL db.relationshipTypes() YIELD relationshipType
MATCH ()-[r]->()
WHERE type(r) = relationshipType
RETURN relationshipType, count(r) as count
ORDER BY count DESC

-- Check indexes
SHOW INDEXES

-- Check constraints
SHOW CONSTRAINTS

-- Clear entire database (DANGEROUS)
MATCH (n) DETACH DELETE n
```

---

## Database Seeding

### Seed Script

```bash
# From project root
cd core/neo4j && ./seed.sh

# Or manually
docker exec -i novanet-neo4j cypher-shell \
  -u neo4j \
  -p novanetpassword \
  < core/neo4j/seed/schema.cypher

docker exec -i novanet-neo4j cypher-shell \
  -u neo4j \
  -p novanetpassword \
  < core/neo4j/seed/data.cypher
```

### Import CSV

```bash
# Copy CSV to import directory
cp data.csv core/neo4j/seed/

# Import via cypher-shell
docker exec -i novanet-neo4j cypher-shell -u neo4j -p novanetpassword << 'EOF'
LOAD CSV WITH HEADERS FROM 'file:///data.csv' AS row
CREATE (n:Node {key: row.key, name: row.name})
EOF
```

---

## APOC Procedures

### Common APOC Operations

```cypher
-- Schema inspection
CALL apoc.meta.schema() YIELD value RETURN value

-- Export to JSON
CALL apoc.export.json.all('/export/full.json', {})

-- Import from JSON
CALL apoc.import.json('/import/data.json')

-- Batch update
CALL apoc.periodic.iterate(
  'MATCH (n:Concept) RETURN n',
  'SET n.updated_at = datetime()',
  {batchSize: 500}
)

-- Create UUID
RETURN apoc.create.uuid() AS uuid

-- Merge nodes
CALL apoc.merge.node(['Concept'], {key: 'pricing'}, {display_name: 'Pricing'})
```

### Check APOC Installation

```cypher
-- List all procedures
CALL dbms.procedures() YIELD name WHERE name STARTS WITH 'apoc' RETURN name

-- APOC version
RETURN apoc.version()
```

---

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs novanet-neo4j

# Common issues:
# 1. Port already in use
lsof -i :7474
lsof -i :7687

# 2. Volume permissions
docker volume rm novanet_neo4j_data
docker compose -f infra/docker-compose.yml up -d

# 3. Memory issues
docker system prune -f
```

### Connection Refused

```bash
# 1. Check container is running
docker ps | grep neo4j

# 2. Check health
docker inspect novanet-neo4j --format='{{.State.Health.Status}}'

# 3. Wait for startup (30s)
sleep 30 && curl http://localhost:7474

# 4. Check network
docker network ls | grep novanet
```

### Slow Queries

```cypher
-- Check indexes
SHOW INDEXES

-- Create missing indexes
CREATE INDEX concept_key IF NOT EXISTS FOR (c:Concept) ON (c.key);
CREATE INDEX locale_key IF NOT EXISTS FOR (l:Locale) ON (l.key);

-- Profile query
PROFILE MATCH (c:Concept)-[:HAS_L10N]->(cl) RETURN count(*)
```

### Out of Memory

```bash
# Increase heap memory in docker-compose.yml
environment:
  - NEO4J_server_memory_heap_initial__size=512m
  - NEO4J_server_memory_heap_max__size=1G
  - NEO4J_server_memory_pagecache_size=512m
```

### Reset Database

```bash
# Stop, remove volume, restart
docker compose -f infra/docker-compose.yml down -v
docker compose -f infra/docker-compose.yml up -d

# Wait for startup
sleep 30

# Re-seed
cd core/neo4j && ./seed.sh
```

---

## Development Workflow

### Daily Workflow

```bash
# Start of day
npm run infra:up

# Verify connection
curl -s http://localhost:7474 > /dev/null && echo "Neo4j ready"

# Start dev server
npm run dev
```

### Before Commit

```bash
# Run tests (include integration tests)
npm test

# Type check
npm run type-check

# Lint
npm run lint
```

### Clean Restart

```bash
# Full reset (data loss!)
docker compose -f infra/docker-compose.yml down -v
docker compose -f infra/docker-compose.yml up -d
sleep 30
cd core/neo4j && ./seed.sh
```

---

## Backup & Restore

### Backup

```bash
# Stop writes first
docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "CALL dbms.setDatabaseAccess('neo4j', 'READ_ONLY')"

# Dump database
docker exec novanet-neo4j neo4j-admin database dump neo4j --to-path=/data/

# Copy dump
docker cp novanet-neo4j:/data/neo4j.dump ./backups/

# Restore writes
docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "CALL dbms.setDatabaseAccess('neo4j', 'READ_WRITE')"
```

### Restore

```bash
# Stop container
docker compose -f infra/docker-compose.yml down

# Remove data volume
docker volume rm novanet_neo4j_data

# Start fresh container
docker compose -f infra/docker-compose.yml up -d

# Copy dump and restore
docker cp ./backups/neo4j.dump novanet-neo4j:/data/
docker exec novanet-neo4j neo4j-admin database load neo4j --from-path=/data/
```

---

## Checklist

### Setup

- [ ] Docker Desktop running
- [ ] `docker compose up -d` successful
- [ ] Neo4j Browser accessible at http://localhost:7474
- [ ] Can connect with credentials neo4j/novanetpassword
- [ ] APOC procedures available

### Development

- [ ] `.env.local` configured in studio/
- [ ] `npm run dev` connects to Neo4j
- [ ] Graph visualization loads data
- [ ] AI chat can query database

### Before Deploy

- [ ] All tests pass
- [ ] Database seeded with latest schema
- [ ] No hardcoded credentials in code
