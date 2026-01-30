---
name: dev-environment
description: Use when setting up development environment, troubleshooting Neo4j connection, or onboarding to the NovaNet project
---

# Dev Environment Skill

Setup and troubleshoot the NovaNet development environment.

## Quick Start

```bash
# 1. Start Neo4j (from monorepo root)
npm run infra:up

# 2. Wait for ready (~30s)
npm run infra:logs  # Wait for "Started."

# 3. Seed database
cd core/neo4j && ./seed.sh

# 4. Open Neo4j Browser
open http://localhost:7474
```

## MCP Configuration

NovaNet uses project-level MCP servers defined in `.mcp.json`:

```json
{
  "mcpServers": {
    "neo4j-cypher": {
      "command": "uvx",
      "args": ["mcp-neo4j-cypher"],
      "env": {
        "NEO4J_URI": "neo4j://localhost:7687",
        "NEO4J_USERNAME": "neo4j",
        "NEO4J_PASSWORD": "novanetpassword"
      }
    }
  }
}
```

### Enable Project MCP Servers

Add to `~/.claude/settings.json`:

```json
{
  "enableAllProjectMcpServers": true
}
```

Or whitelist specific servers:

```json
{
  "allowedMcpjsonServers": ["neo4j-cypher", "neo4j-memory"]
}
```

### Available MCP Tools

| Tool | Description |
|------|-------------|
| `mcp__neo4j__get_neo4j_schema` | Get graph schema (nodes, relationships, properties) |
| `mcp__neo4j__read_neo4j_cypher` | Execute read-only Cypher queries |
| `mcp__neo4j__write_neo4j_cypher` | Execute write Cypher queries |

### MCP Usage Examples

```
# Get schema
mcp__neo4j__get_neo4j_schema(sample_size=100)

# Read query
mcp__neo4j__read_neo4j_cypher(
  query="MATCH (c:Concept) RETURN c.key LIMIT 10"
)

# Write query with params
mcp__neo4j__write_neo4j_cypher(
  query="CREATE (c:Concept {key: $key})",
  params={"key": "new-concept"}
)
```

## Troubleshooting

### MCP Connection Failed

**Symptom**: `Neo4j Client Error: Unauthorized`

**Cause**: Wrong credentials or Neo4j not running

**Fix**:
```bash
# Check Neo4j is running
docker ps | grep neo4j

# Restart if needed (from monorepo root)
npm run infra:down && npm run infra:up

# Verify credentials
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword "RETURN 1"
```

### Docker Not Running

**Symptom**: `Cannot connect to Docker daemon`

**Fix**: Start Docker Desktop

### Port Conflict

**Symptom**: `Port 7474 or 7687 already in use`

**Fix**:
```bash
# Find what's using the port
lsof -i :7474
lsof -i :7687

# Kill or change ports in infra/docker-compose.yml
```

### Seed Failed

**Symptom**: `Cypher error during seed`

**Fix**:
```bash
# Reset database completely (from monorepo root)
npm run infra:down
docker volume rm novanet_neo4j_data
npm run infra:up
sleep 30  # Wait for Neo4j
cd core/neo4j && ./seed.sh
```

### Empty Database

**Symptom**: Queries return no results

**Fix**:
```bash
# Verify data exists
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (n) RETURN labels(n)[0] AS label, count(*) AS count ORDER BY count DESC"

# Re-run seed if empty
cd neo4j && ./seed.sh
```

## Environment Variables

Copy `.env.example` to `.env`:

```bash
cp .env.example .env
```

Required variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `NEO4J_URI` | `neo4j://localhost:7687` | Neo4j Bolt connection |
| `NEO4J_USERNAME` | `neo4j` | Database user |
| `NEO4J_PASSWORD` | `novanetpassword` | Database password |

## Development Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                    DEVELOPMENT LOOP                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. Edit models/*.yaml     → Define schema changes          │
│                                                             │
│  2. Update neo4j/seed/*.cypher → Write seed queries         │
│                                                             │
│  3. Reset & seed           → npm run infra:down -v && up    │
│                                                             │
│  4. Test with MCP          → Use mcp__neo4j__* tools        │
│                                                             │
│  5. Iterate                → Repeat until correct           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Useful Commands

```bash
# Neo4j status (from monorepo root)
docker compose -f infra/docker-compose.yml ps

# Neo4j logs
npm run infra:logs

# Interactive Cypher shell
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword

# Quick query
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (n) RETURN labels(n)[0], count(*) ORDER BY count(*) DESC"

# Export graph (APOC)
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "CALL apoc.export.json.all('/var/lib/neo4j/export/full.json', {})"
```

## Neo4j Browser Queries

Open http://localhost:7474 and try:

```cypher
// Overview
CALL db.schema.visualization()

// All node types
MATCH (n) RETURN labels(n)[0] AS label, count(*) AS count ORDER BY count DESC

// Sample concepts
MATCH (c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)
RETURN c.key, cl.locale, cl.title LIMIT 10

// Graph visualization
MATCH (n)-[r]->(m) RETURN n, r, m LIMIT 100
```

## Checklist: New Developer Setup

- [ ] Docker Desktop installed and running
- [ ] Ports 7474 and 7687 available
- [ ] `npm run infra:up` successful
- [ ] `./seed.sh` completed without errors
- [ ] Neo4j Browser accessible at http://localhost:7474
- [ ] MCP tools responding (test with `mcp__neo4j__get_neo4j_schema`)
- [ ] `.env` file created from `.env.example`
