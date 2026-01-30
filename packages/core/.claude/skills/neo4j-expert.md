---
name: neo4j-expert
description: Use when writing Cypher queries, designing Neo4j schemas, optimizing graph performance, or troubleshooting Neo4j issues
---

# Neo4j Expert Skill

Master Neo4j graph database patterns for NovaNet's content generation system.

## When to Use

- Writing or reviewing Cypher queries
- Designing graph schemas and relationships
- Optimizing query performance
- Creating indexes and constraints
- Debugging slow queries

## Graph Modeling Principles

### Node Design

```cypher
-- Nodes represent entities with labels and properties
(:Concept {key: "tier-pro", llm_context: "Premium subscription..."})

-- Use labels for classification (max 4 per node for performance)
(:Block:Hero {key: "hero-pricing"})

-- Properties: primitives only (no nested objects)
-- Use relationships instead of arrays of references
```

### Relationship Design

```cypher
-- Relationships MUST have a type (verb-like, UPPER_SNAKE_CASE)
-[:USES_CONCEPT {purpose: "primary", temperature: 0.95}]->

-- Direction matters for traversal performance
(Block)-[:USES_CONCEPT]->(Concept)  -- Block references Concept
(Concept)-[:SEMANTIC_LINK]->(Concept)  -- Bidirectional semantically

-- Properties on relationships for edge metadata
-[:HAS_L10N {locale: "fr-FR", version: 2}]->  -- for Concept
-[:HAS_OUTPUT {locale: "fr-FR", version: 2}]->  -- for Page/Block
```

### NovaNet-Specific Patterns (v8.2.0)

```
35 Node Types across 3 Scopes:
  🌍 Global (15): Locale + 14 LocaleKnowledge nodes
  📦 Project (14): Foundation, Structure, Semantic, Instruction, Output
  🎯 Shared (6): SEO/GEO mining nodes

Entity → Content Pattern:
  (Page)-[:HAS_OUTPUT]->(PageL10n)-[:FOR_LOCALE]->(Locale)
  (Block)-[:HAS_OUTPUT]->(BlockL10n)-[:FOR_LOCALE]->(Locale)
  (Concept)-[:HAS_L10N]->(ConceptL10n)-[:FOR_LOCALE]->(Locale)

Hierarchy Pattern:
  (Project)-[:HAS_PAGE]->(Page)-[:HAS_BLOCK {position}]->(Block)-[:OF_TYPE]->(BlockType)

Locale Knowledge Pattern (14 nodes):
  (Locale)-[:HAS_IDENTITY]->(LocaleIdentity)
  (Locale)-[:HAS_VOICE]->(LocaleVoice)
  (Locale)-[:HAS_CULTURE]->(LocaleCulture)-[:HAS_CULTURE_REFS]->(LocaleCultureReferences)
  (LocaleCultureReferences)-[:HAS_REFERENCE]->(Reference)
  (LocaleCultureReferences)-[:HAS_METAPHOR]->(Metaphor)
  (LocaleCultureReferences)-[:HAS_CONSTRAINT]->(Constraint)
  (Locale)-[:HAS_MARKET]->(LocaleMarket)
  (Locale)-[:HAS_LEXICON]->(LocaleLexicon)-[:HAS_EXPRESSION]->(Expression)
  (Locale)-[:HAS_RULES_ADAPTATION]->(LocaleRulesAdaptation)
  (Locale)-[:HAS_RULES_FORMATTING]->(LocaleRulesFormatting)-[:HAS_PATTERN]->(Pattern)
  (Locale)-[:HAS_RULES_SLUG]->(LocaleRulesSlug)

SEO/GEO Pattern:
  (Concept)-[:TARGETS_SEO]->(SEOKeywordL10n)-[:FOR_LOCALE]->(Locale)
  (Concept)-[:TARGETS_GEO]->(GEOSeedL10n)-[:FOR_LOCALE]->(Locale)

Standard Properties (all nodes - v8.2.0):
  key, display_name, description, llm_context, created_at, updated_at
  (NOTE: icon, priority, freshness REMOVED in v8.2.0 - YAGNI)
```

## Query Optimization

### Always Use EXPLAIN/PROFILE

```cypher
-- Preview plan without execution
EXPLAIN
MATCH (b:Block {key: "hero-pricing"})-[:USES_CONCEPT]->(c:Concept)
RETURN c.key;

-- Execute with metrics (db hits, rows, time)
PROFILE
MATCH (b:Block {key: "hero-pricing"})-[:USES_CONCEPT]->(c:Concept)
RETURN c.key;
```

**Look for:**
- Green = efficient (Index Seek, Index Scan)
- Yellow/Red = costly (NodeByLabelScan, AllNodesScan)
- Low "DB Hits" = good index usage

### Index Strategy

```cypher
-- Create indexes for frequently queried properties
CREATE INDEX block_key IF NOT EXISTS FOR (b:Block) ON (b.key);
CREATE INDEX concept_key IF NOT EXISTS FOR (c:Concept) ON (c.key);
CREATE INDEX content_locale IF NOT EXISTS FOR (cl:ConceptL10n) ON (cl.locale);

-- Composite indexes for multi-property filters
CREATE INDEX block_page_position IF NOT EXISTS
FOR (b:Block) ON (b.page_key, b.position);

-- Uniqueness constraints (also creates index)
CREATE CONSTRAINT concept_unique_key IF NOT EXISTS
FOR (c:Concept) REQUIRE c.key IS UNIQUE;

-- Full-text for search
CREATE FULLTEXT INDEX concept_search IF NOT EXISTS
FOR (c:Concept) ON EACH [c.key, c.llm_context];
```

### Query Patterns

```cypher
-- GOOD: Index in MATCH pattern
MATCH (c:Concept {key: "tier-pro"})

-- BAD: Index after WHERE (may not use index)
MATCH (c:Concept) WHERE c.key = "tier-pro"

-- GOOD: Limit early with WHERE
MATCH (b:Block)-[:USES_CONCEPT]->(c:Concept)
WHERE b.key = "hero-pricing"
RETURN c

-- BAD: Filter after collecting all
MATCH (b:Block)-[:USES_CONCEPT]->(c:Concept)
WITH b, collect(c) AS concepts
WHERE b.key = "hero-pricing"
RETURN concepts
```

## Essential Cypher Patterns

### Variable-Length Paths (Spreading)

```cypher
-- Find concepts within 2 hops with temperature cutoff
MATCH path = (c:Concept {key: "tier-pro"})-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH c2,
     reduce(activation = 1.0, rel IN r | activation * rel.temperature) AS activation,
     [rel IN r | rel.type] AS path_types
WHERE activation >= 0.3
RETURN c2.key, activation, path_types
ORDER BY activation DESC;
```

### Aggregation with Context (v7.0.0)

```cypher
-- Load block with all related content (v7.0.0 - Locale Knowledge via graph)
MATCH (b:Block {key: $blockKey})
OPTIONAL MATCH (b)-[:USES_CONCEPT]->(c:Concept)
OPTIONAL MATCH (c)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})
OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
RETURN b.instructions,
       collect(DISTINCT {key: c.key, title: cl.title, definition: cl.definition}) AS concepts,
       v.formality_score AS formality,
       collect(DISTINCT e.text) AS expressions,
       bt.rules;
```

### UNWIND for Batch Operations

```cypher
-- Create multiple nodes from list
UNWIND $concepts AS concept
CREATE (c:Concept {key: concept.key, llm_context: concept.hints})
RETURN count(c);

-- Create relationships from mapping
UNWIND $mappings AS m
MATCH (b:Block {key: m.block_key})
MATCH (c:Concept {key: m.concept_key})
CREATE (b)-[:USES_CONCEPT {purpose: m.purpose, temperature: m.temp}]->(c);
```

### MERGE for Idempotent Operations

```cypher
-- Create if not exists, update if exists
MERGE (c:Concept {key: $key})
ON CREATE SET c.created_at = datetime(), c.llm_context = $hints
ON MATCH SET c.updated_at = datetime(), c.llm_context = $hints
RETURN c;
```

## Performance Checklist

1. **Check indexes exist** for all WHERE/MATCH properties
2. **Use parameters** ($param) instead of string interpolation
3. **Limit results** early with WHERE, not late with LIMIT
4. **Avoid Cartesian products** - always connect patterns
5. **Use OPTIONAL MATCH** for nullable relationships
6. **Profile queries** before production deployment

## Common Anti-Patterns

```cypher
-- ANTI: Collecting then filtering (memory explosion)
MATCH (n) WITH collect(n) AS all WHERE size(all) > 100...

-- ANTI: Unbounded variable-length paths
MATCH (a)-[*]->(b)  -- Can explode on large graphs

-- ANTI: Multiple unconnected patterns (Cartesian product)
MATCH (a:A), (b:B)  -- Returns A × B rows

-- ANTI: Regex on unindexed properties
WHERE n.name =~ '.*pattern.*'  -- Full scan
```

## Debugging Commands

```cypher
-- Show all indexes
SHOW INDEXES;

-- Show all constraints
SHOW CONSTRAINTS;

-- Database statistics
CALL db.stats.retrieve('GRAPH COUNTS');

-- Clear query cache (after schema changes)
CALL db.clearQueryCaches();
```

## MCP Integration

NovaNet uses Neo4j MCP servers for direct database access from Claude Code.

### Available MCP Tools

| Tool | Purpose |
|------|---------|
| `mcp__neo4j__get_neo4j_schema` | Inspect graph schema |
| `mcp__neo4j__read_neo4j_cypher` | Execute read queries |
| `mcp__neo4j__write_neo4j_cypher` | Execute write queries |

### MCP Query Examples

```python
# Get schema overview
mcp__neo4j__get_neo4j_schema(sample_size=100)

# Read query with parameters
mcp__neo4j__read_neo4j_cypher(
    query="""
        MATCH (c:Concept {key: $key})-[:HAS_L10N]->(cl:ConceptL10n {locale: $locale})
        RETURN c.key, cl.title, cl.definition
    """,
    params={"key": "tier-pro", "locale": "fr-FR"}
)

# Write query
mcp__neo4j__write_neo4j_cypher(
    query="""
        MERGE (c:Concept {key: $key})
        ON CREATE SET c.created_at = datetime()
        ON MATCH SET c.updated_at = datetime()
        RETURN c
    """,
    params={"key": "new-concept"}
)
```

### When to Use MCP vs Cypher Shell

| Scenario | Use |
|----------|-----|
| Quick exploration | MCP (`mcp__neo4j__read_neo4j_cypher`) |
| Schema changes | MCP (`mcp__neo4j__write_neo4j_cypher`) |
| Bulk data import | Cypher Shell (`./seed.sh`) |
| Interactive debugging | Neo4j Browser (http://localhost:7474) |
| Complex transactions | Cypher Shell with APOC |

### MCP Troubleshooting

```bash
# MCP not connecting? Check:
# 1. Neo4j running
docker ps | grep neo4j

# 2. Credentials correct in .mcp.json
cat .mcp.json

# 3. MCP servers enabled in Claude settings
# Add to ~/.claude/settings.json:
# "enableAllProjectMcpServers": true
```
