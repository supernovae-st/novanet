# Outils MCP Disponibles

## Source de verite: ~/.spn/mcp.yaml

Nika charge les MCP depuis `~/.spn/mcp.yaml` (global) + `.spn/mcp.yaml` (projet).

Les secrets sont references avec `${VAR_NAME}` et le shell les expand.

## Configuration actuelle (nika/tools/nika/config/mcp-servers.yaml)

### Perplexity
```yaml
perplexity:
  command: npx
  args:
    - -y
    - "@perplexity-ai/mcp-server"
  env:
    PERPLEXITY_API_KEY: "${PERPLEXITY_API_KEY}"
```

**Tools**: `perplexity_search_web`, `perplexity_ask`, `perplexity_research`, `perplexity_reason`

### Ahrefs
```yaml
ahrefs:
  type: http
  url: https://api.ahrefs.com/mcp/mcp
```

**Tools**: keyword research, backlink analysis, site audit

**Note**: A configurer dans `~/.spn/mcp.yaml` avec la cle API.

### Firecrawl
```yaml
firecrawl:
  command: npx
  args:
    - -y
    - "firecrawl-mcp"
  env:
    FIRECRAWL_API_KEY: "${FIRECRAWL_API_KEY}"
```

**Tools**: `firecrawl_scrape`, `firecrawl_map`, `firecrawl_search`, `firecrawl_crawl`, `firecrawl_extract`

### NovaNet
```yaml
novanet:
  command: cargo
  args:
    - run
    - --manifest-path
    - ../../../novanet/tools/novanet-mcp/Cargo.toml
  env:
    NOVANET_MCP_NEO4J_URI: bolt://localhost:7687
    NOVANET_MCP_NEO4J_USER: neo4j
    NOVANET_MCP_NEO4J_PASSWORD: novanetpassword
```

**Tools** (11 outils):
- `novanet_query` - Execute Cypher (lecture)
- `novanet_describe` - Bootstrap agent
- `novanet_search` - Fulltext + property search
- `novanet_traverse` - Graph traversal
- `novanet_assemble` - Context assembly
- `novanet_atoms` - Knowledge atoms
- `novanet_generate` - RLM-on-KG context
- `novanet_introspect` - Schema introspection
- `novanet_batch` - Bulk operations
- `novanet_cache_stats` - Cache statistics
- `novanet_cache_invalidate` - Cache invalidation

## A configurer

### Ahrefs dans ~/.spn/mcp.yaml

```yaml
# Ajouter dans ~/.spn/mcp.yaml
servers:
  ahrefs:
    type: http
    url: https://api.ahrefs.com/mcp/mcp
    headers:
      Authorization: "Bearer ${AHREFS_API_KEY}"
    enabled: true
```

### Variables d'environnement necessaires

```bash
# Dans ~/.zshrc ou ~/.spn/secrets.env
export PERPLEXITY_API_KEY="pplx-..."
export AHREFS_API_KEY="..."
export FIRECRAWL_API_KEY="fc-..."
```

## Question ouverte: Ecriture dans NovaNet

Le MCP NovaNet actuel ne permet que la LECTURE (`novanet_query` est read-only).

**Options**:
1. Ajouter un tool `novanet_write` pour ecrire dans le graphe
2. Utiliser `novanet_batch` avec des operations d'ecriture
3. Appeler Neo4j directement depuis Nika (bypass MCP)

**A discuter avec Thibaut** dans une session separee.
