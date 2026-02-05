---
name: neo4j-development
description: Neo4j driver patterns, Cypher queries, and TypeScript integration. Use when working with graph database code.
user-invocable: false
---

# Neo4j Development Patterns for NovaNet

> Best practices for Neo4j 5.x with APOC, TypeScript driver, and graph visualization

## Connection Setup

### Driver Configuration

```typescript
// lib/neo4j.ts
import neo4j, { Driver, Session, QueryResult } from 'neo4j-driver';

const driver: Driver = neo4j.driver(
  process.env.NEO4J_URI || 'bolt://localhost:7687',
  neo4j.auth.basic(
    process.env.NEO4J_USERNAME || 'neo4j',
    process.env.NEO4J_PASSWORD || 'novanetpassword'
  ),
  {
    maxConnectionPoolSize: 50,
    connectionAcquisitionTimeout: 10000,
    maxTransactionRetryTime: 30000,
  }
);

export default driver;

// Health check
export async function verifyConnection(): Promise<boolean> {
  const session = driver.session();
  try {
    await session.run('RETURN 1');
    return true;
  } catch (error) {
    console.error('Neo4j connection failed:', error);
    return false;
  } finally {
    await session.close();
  }
}
```

### Session Management

```typescript
// GOOD - Session per operation
export async function getNodes(labels: string[]): Promise<Node[]> {
  const session = driver.session({ database: 'neo4j' });
  try {
    const result = await session.run(
      `MATCH (n) WHERE any(label IN labels(n) WHERE label IN $labels)
       RETURN n LIMIT 1000`,
      { labels }
    );
    return result.records.map((r) => r.get('n').properties);
  } finally {
    await session.close();  // ALWAYS close
  }
}

// BAD - Reusing session across requests
let globalSession = driver.session();  // DON'T DO THIS
```

---

## Cypher Best Practices

### Parameterized Queries (Security)

```typescript
// BAD - SQL/Cypher injection vulnerability
const query = `MATCH (n:${nodeType}) WHERE n.key = '${key}' RETURN n`;

// GOOD - Parameterized (safe)
const query = `MATCH (n) WHERE $label IN labels(n) AND n.key = $key RETURN n`;
const params = { label: nodeType, key };
await session.run(query, params);
```

### Result Limits

```typescript
// ALWAYS limit results - NovaNet has 19k+ nodes
const DEFAULT_LIMIT = 100;
const MAX_LIMIT = 1000;

export async function queryNodes(
  cypher: string,
  params: Record<string, unknown>,
  limit = DEFAULT_LIMIT
): Promise<QueryResult> {
  const safeLimit = Math.min(limit, MAX_LIMIT);

  // Add LIMIT if not present
  const limitedQuery = cypher.includes('LIMIT')
    ? cypher
    : `${cypher} LIMIT ${safeLimit}`;

  return session.run(limitedQuery, params);
}
```

### APOC Utilities

```cypher
-- Schema inspection (used by neo4j-architect agent)
CALL apoc.meta.schema() YIELD value RETURN value;

-- Batch operations
CALL apoc.periodic.iterate(
  'MATCH (n:Entity) RETURN n',
  'SET n.updated_at = datetime()',
  { batchSize: 500, parallel: false }
);

-- JSON export
CALL apoc.export.json.query(
  'MATCH (n:Entity)-[r]->(m) RETURN n, r, m',
  '/export/concepts.json',
  {}
);
```

---

## NovaNet Graph Patterns

### Spreading Activation Query

```cypher
-- Find semantically related concepts with weighted traversal
MATCH (c:Entity {key: $entityKey})
MATCH path = (c)-[r:SEMANTIC_LINK*1..2]->(related:Entity)
WHERE ALL(rel IN relationships(path) WHERE rel.temperature >= 0.3)
WITH related,
     reduce(activation = 1.0, rel IN relationships(path) |
       activation * rel.temperature
     ) AS activation
WHERE activation >= $threshold
RETURN related.key AS key,
       related.display_name AS name,
       activation
ORDER BY activation DESC
LIMIT 20
```

### Context Loading Query

```cypher
-- Load full context for content generation
MATCH (b:Block {key: $blockKey})
MATCH (b)-[:OF_TYPE]->(bt:BlockType)
MATCH (b)<-[:HAS_BLOCK]-(p:Page)

OPTIONAL MATCH (b)-[:USES_ENTITY]->(c:Entity)
OPTIONAL MATCH (c)-[:HAS_L10N]->(cl:EntityL10n)-[:FOR_LOCALE]->(l:Locale {key: $locale})

OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)

RETURN b, bt, p,
       collect(DISTINCT c) AS concepts,
       collect(DISTINCT cl) AS conceptL10ns,
       li, lv, lc
```

### Node Type Colors

```typescript
// Match Tailwind config node colors
export const NODE_COLORS: Record<string, string> = {
  Project: '#8b5cf6',      // violet-500
  Page: '#3b82f6',         // blue-500
  Block: '#06b6d4',        // cyan-500
  BlockType: '#14b8a6',    // teal-500
  Entity: '#f59e0b',      // amber-500
  EntityL10n: '#fbbf24',  // amber-400
  Locale: '#10b981',       // emerald-500
  LocaleIdentity: '#22c55e',
  LocaleVoice: '#22c55e',
  LocaleCulture: '#22c55e',
  LocaleMarket: '#22c55e',
  LocaleLexicon: '#22c55e',
  Expression: '#ec4899',   // pink-500
};

export function getNodeColor(labels: string[]): string {
  for (const label of labels) {
    if (NODE_COLORS[label]) return NODE_COLORS[label];
  }
  return '#71717a';  // zinc-500 fallback
}
```

---

## TypeScript Type Safety

### Driver Result Mapping

```typescript
import { Record as Neo4jRecord, Node as Neo4jNode, Integer } from 'neo4j-driver';

// Convert Neo4j types to JS
function toNumber(value: Integer | number): number {
  return Integer.isInteger(value) ? value.toNumber() : value;
}

function toDate(value: unknown): Date | null {
  if (!value) return null;
  if (value instanceof Date) return value;
  // Neo4j DateTime
  if (typeof value === 'object' && 'toStandardDate' in value) {
    return (value as { toStandardDate: () => Date }).toStandardDate();
  }
  return new Date(String(value));
}

// Map Neo4j node to app type
function mapNode(neo4jNode: Neo4jNode): AppNode {
  const props = neo4jNode.properties;
  return {
    id: toNumber(neo4jNode.identity),
    labels: neo4jNode.labels,
    key: String(props.key),
    displayName: String(props.display_name || ''),
    priority: props.priority as Priority,
    freshness: props.freshness as Freshness,
    createdAt: toDate(props.created_at),
    updatedAt: toDate(props.updated_at),
  };
}
```

### Generic Query Helper

```typescript
type QueryMapper<T> = (record: Neo4jRecord) => T;

async function query<T>(
  cypher: string,
  params: Record<string, unknown>,
  mapper: QueryMapper<T>
): Promise<T[]> {
  const session = driver.session();
  try {
    const result = await session.run(cypher, params);
    return result.records.map(mapper);
  } finally {
    await session.close();
  }
}

// Usage
const concepts = await query(
  'MATCH (c:Entity) RETURN c LIMIT 100',
  {},
  (record) => mapNode(record.get('c'))
);
```

---

## Performance Optimization

### Index Usage

```cypher
-- Create indexes for common queries
CREATE INDEX entity_key IF NOT EXISTS FOR (c:Entity) ON (c.key);
CREATE INDEX locale_key IF NOT EXISTS FOR (l:Locale) ON (l.key);
CREATE INDEX node_priority IF NOT EXISTS FOR (n) ON (n.priority);

-- Check index usage
EXPLAIN MATCH (c:Entity {key: 'pricing'}) RETURN c;
```

### Query Profiling

```cypher
-- Profile query performance
PROFILE MATCH (p:Project)-[:HAS_PAGE]->(page:Page)
        -[:HAS_BLOCK]->(b:Block)
        -[:USES_ENTITY]->(c:Entity)
RETURN count(*)
```

### Batch Processing

```typescript
// Process nodes in batches to avoid memory issues
async function processAllNodes(
  processor: (node: AppNode) => Promise<void>
): Promise<void> {
  const BATCH_SIZE = 500;
  let offset = 0;
  let hasMore = true;

  while (hasMore) {
    const nodes = await query(
      `MATCH (n) RETURN n
       ORDER BY n.key
       SKIP $offset LIMIT $limit`,
      { offset, limit: BATCH_SIZE },
      (r) => mapNode(r.get('n'))
    );

    await Promise.all(nodes.map(processor));

    hasMore = nodes.length === BATCH_SIZE;
    offset += BATCH_SIZE;
  }
}
```

---

## React Integration

### Server Component Data Fetching

```typescript
// app/graph/page.tsx (Server Component)
export default async function GraphPage() {
  const [nodes, edges] = await Promise.all([
    fetchNodes(['Project', 'Page', 'Entity']),
    fetchEdges(['HAS_PAGE', 'USES_ENTITY']),
  ]);

  return <GraphClient initialNodes={nodes} initialEdges={edges} />;
}
```

### Client-Side Refetch

```typescript
// hooks/useGraphData.ts
export function useGraphData(filters: FilterState) {
  const [data, setData] = useState<GraphData | null>(null);
  const [loading, setLoading] = useState(false);

  const refetch = useCallback(async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/graph', {
        method: 'POST',
        body: JSON.stringify(filters),
      });
      const data = await response.json();
      setData(data);
    } finally {
      setLoading(false);
    }
  }, [filters]);

  useEffect(() => {
    refetch();
  }, [refetch]);

  return { data, loading, refetch };
}
```

---

## Troubleshooting

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `ServiceUnavailable` | Neo4j not running | `npm run infra:up` |
| `AuthorizationExpired` | Session timeout | Close and recreate session |
| `ClientError: SyntaxError` | Invalid Cypher | Check query syntax |
| `TransientError` | Deadlock | Retry with backoff |

### Connection Health Check

```typescript
// api/health/route.ts
export async function GET() {
  const healthy = await verifyConnection();
  return Response.json(
    { neo4j: healthy ? 'ok' : 'error' },
    { status: healthy ? 200 : 503 }
  );
}
```

---

## Checklist

- [ ] Always use parameterized queries (prevent injection)
- [ ] Always close sessions (prevent connection leaks)
- [ ] Always limit results (prevent memory issues)
- [ ] Use transactions for write operations
- [ ] Map Neo4j types to JS types (Integer, DateTime)
- [ ] Index frequently queried properties
- [ ] Profile slow queries
