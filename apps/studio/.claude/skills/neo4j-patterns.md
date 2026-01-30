---
name: neo4j-patterns
description: Neo4j driver connection, read/write queries, and API routes. Use when implementing database queries, graph data fetching, or error handling for Neo4j operations.
user-invocable: false
---

# Neo4j Driver Patterns

> NovaNet database layer with neo4j-driver

## Use When

- Setting up Neo4j connection
- Writing Cypher queries for graph data
- Creating API routes for graph fetching
- Implementing error handling for database operations
- Optimizing query performance

## Quick Reference

| Pattern | Usage |
|---------|-------|
| Read query | `session.executeRead(tx => tx.run(cypher, params))` |
| Write query | `session.executeWrite(tx => tx.run(cypher, params))` |
| Always close | `await session.close()` in finally block |
| Parameterize | Never interpolate user input |

---

## Connection Setup

```typescript
import neo4j, { Driver, Session } from 'neo4j-driver';

let driver: Driver | null = null;

export function getDriver(): Driver {
  if (!driver) {
    driver = neo4j.driver(
      process.env.NEO4J_URI!,
      neo4j.auth.basic(process.env.NEO4J_USER!, process.env.NEO4J_PASSWORD!)
    );
  }
  return driver;
}

export async function closeDriver(): Promise<void> {
  if (driver) {
    await driver.close();
    driver = null;
  }
}
```

## Read Queries (Most Common)

```typescript
export async function executeRead<T>(
  cypher: string,
  params: Record<string, unknown> = {}
): Promise<T[]> {
  const driver = getDriver();
  const session = driver.session({ database: 'neo4j' });

  try {
    const result = await session.executeRead(async (tx) => {
      const res = await tx.run(cypher, params);
      return res.records.map((record) => record.toObject() as T);
    });
    return result;
  } finally {
    await session.close();
  }
}

// Usage
const projects = await executeRead<{ p: Project }>(
  'MATCH (p:Project) RETURN p LIMIT $limit',
  { limit: 100 }
);
```

## Graph Queries for Visualization

```typescript
interface GraphQueryResult {
  nodes: Array<{
    id: string;
    labels: string[];
    properties: Record<string, unknown>;
  }>;
  edges: Array<{
    id: string;
    type: string;
    source: string;
    target: string;
    properties: Record<string, unknown>;
  }>;
}

export async function fetchGraphData(
  nodeTypes: string[],
  locale?: string,
  limit = 1000
): Promise<GraphQueryResult> {
  const typeFilter = nodeTypes.map((t) => `n:${t}`).join(' OR ');
  const localeFilter = locale ? 'AND (n.locale = $locale OR n.locale IS NULL)' : '';

  const cypher = `
    MATCH (n)
    WHERE (${typeFilter}) ${localeFilter}
    WITH n LIMIT $limit
    OPTIONAL MATCH (n)-[r]->(m)
    WHERE (${nodeTypes.map((t) => `m:${t}`).join(' OR ')})
    RETURN
      collect(DISTINCT {
        id: elementId(n),
        labels: labels(n),
        properties: properties(n)
      }) as nodes,
      collect(DISTINCT {
        id: elementId(r),
        type: type(r),
        source: elementId(startNode(r)),
        target: elementId(endNode(r)),
        properties: properties(r)
      }) as edges
  `;

  const result = await executeRead<GraphQueryResult>(cypher, {
    locale,
    limit,
  });

  return result[0] || { nodes: [], edges: [] };
}
```

## Write Queries (Admin Only)

```typescript
export async function executeWrite<T>(
  cypher: string,
  params: Record<string, unknown> = {}
): Promise<T[]> {
  const driver = getDriver();
  const session = driver.session({ database: 'neo4j' });

  try {
    const result = await session.executeWrite(async (tx) => {
      const res = await tx.run(cypher, params);
      return res.records.map((record) => record.toObject() as T);
    });
    return result;
  } finally {
    await session.close();
  }
}
```

## Common Cypher Patterns for NovaNet

```cypher
-- Get project overview
MATCH (p:Project)-[:HAS_LOCALE]->(l:Locale)
RETURN p.name, collect(l.code) as locales

-- Get translation chain
MATCH path = (s:Source)-[:HAS_UNIT]->(tu:TranslationUnit)
  -[:HAS_TRANSLATION]->(t)
WHERE t:AITranslation OR t:HumanTranslation
RETURN s, tu, t, relationships(path)

-- Find concepts used by expressions
MATCH (e:Expression)-[:USES_CONCEPT]->(c:Concept)
RETURN e.value, c.name, c.llm_context

-- Get pipeline with outputs
MATCH (p:Pipeline)-[:HAS_OUTPUT]->(o:PlatformOutput)
RETURN p.name, collect(o.format) as formats
```

## Error Handling

```typescript
import { Neo4jError } from 'neo4j-driver';

export async function safeQuery<T>(
  cypher: string,
  params: Record<string, unknown> = {}
): Promise<{ data: T[] | null; error: string | null }> {
  try {
    const data = await executeRead<T>(cypher, params);
    return { data, error: null };
  } catch (err) {
    if (err instanceof Neo4jError) {
      console.error('Neo4j Error:', err.code, err.message);
      return { data: null, error: `Database error: ${err.message}` };
    }
    throw err;
  }
}
```

## API Route Pattern

```typescript
// app/api/graph/route.ts
import { NextRequest, NextResponse } from 'next/server';
import { fetchGraphData } from '@/lib/neo4j';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const types = searchParams.get('types')?.split(',') || [];
  const locale = searchParams.get('locale') || undefined;
  const limit = parseInt(searchParams.get('limit') || '1000', 10);

  try {
    const data = await fetchGraphData(types, locale, limit);
    return NextResponse.json(data);
  } catch (error) {
    console.error('Graph fetch error:', error);
    return NextResponse.json(
      { error: 'Failed to fetch graph data' },
      { status: 500 }
    );
  }
}
```
