# API Reference

NovaNet internal APIs and type definitions.

## TypeScript Types

### Node Types

```typescript
// Generated from YAML
type NodeRealm = 'shared' | 'org';

type NodeLayer =
  | 'config'
  | 'locale'
  | 'geography'
  | 'knowledge'
  | 'foundation'
  | 'structure'
  | 'semantic'
  | 'instruction'
  | 'output';

type NodeTrait =
  | 'invariant'
  | 'localized'
  | 'knowledge'
  | 'generated'
  | 'aggregated';
```

### Arc Types

```typescript
type ArcScope = 'intra_realm' | 'cross_realm';

type ArcFamily =
  | 'ownership'
  | 'localization'
  | 'semantic'
  | 'generation'
  | 'mining';

type ArcCardinality =
  | 'zero_to_one'
  | 'one_to_one'
  | 'one_to_many'
  | 'many_to_many';
```

### Filter Types

```typescript
interface FacetFilter {
  realms?: NodeRealm[];
  layers?: NodeLayer[];
  traits?: NodeTrait[];
  kinds?: string[];
  arcFamilies?: ArcFamily[];
  arcScopes?: ArcScope[];
}
```

## Rust Types

### Core Types

```rust
// From taxonomy.yaml
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRealm {
    Shared,
    Org,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeLayer {
    Config,
    Locale,
    Geography,
    Knowledge,
    Foundation,
    Structure,
    Semantic,
    Instruction,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeTrait {
    Invariant,
    Localized,
    Knowledge,
    Generated,
    Aggregated,
}
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum NovaNetError {
    #[error("Neo4j error: {0}")]
    Neo4j(#[from] neo4rs::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NovaNetError>;
```

## Neo4j Schema

### Meta Labels

All meta-nodes carry the `:Meta` label:

| Label | Description |
|-------|-------------|
| `:Meta:Realm` | Realm definition |
| `:Meta:Layer` | Layer definition |
| `:Meta:Trait` | Trait definition |
| `:Meta:Kind` | Node type definition |
| `:Meta:ArcFamily` | Arc family definition |
| `:Meta:ArcKind` | Arc type definition |

### Constraints

```cypher
// Key uniqueness
CREATE CONSTRAINT kind_key IF NOT EXISTS
FOR (n:Meta:Kind) REQUIRE n.name IS UNIQUE;

CREATE CONSTRAINT arc_key IF NOT EXISTS
FOR (n:Meta:ArcKind) REQUIRE n.name IS UNIQUE;

// Data node keys
CREATE CONSTRAINT page_key IF NOT EXISTS
FOR (n:Page) REQUIRE n.key IS UNIQUE;
```

### Indexes

```cypher
// Full-text search
CREATE FULLTEXT INDEX entity_fulltext IF NOT EXISTS
FOR (n:Entity) ON EACH [n.key, n.display_name, n.description];

// Property indexes
CREATE INDEX page_realm IF NOT EXISTS
FOR (n:Page) ON (n.realm);
```

## Cypher Patterns

### Query Kinds by Layer

```cypher
MATCH (l:Meta:Layer {key: $layer})-[:HAS_KIND]->(k:Meta:Kind)
RETURN k.name, k.trait
ORDER BY k.name
```

### Query Arcs by Family

```cypher
MATCH (f:Meta:ArcFamily {key: $family})-[:HAS_ARC_KIND]->(a:Meta:ArcKind)
RETURN a.name, a.source, a.target
ORDER BY a.name
```

### Full-Text Search

```cypher
CALL db.index.fulltext.queryNodes("entity_fulltext", $query)
YIELD node, score
RETURN node.key, node.display_name, score
ORDER BY score DESC
LIMIT $limit
```

### Faceted Query

```cypher
MATCH (n)
WHERE any(label IN labels(n) WHERE label IN $kinds)
  AND n.realm IN $realms
  AND n.layer IN $layers
RETURN n
```

## Generated Files

### TypeScript

| File | Content |
|------|---------|
| `node-kinds.generated.ts` | Kind Zod schemas |
| `arc-kinds.generated.ts` | ArcKind definitions |
| `taxonomy.generated.ts` | Enums for realm/layer/trait |
| `visual-encoding.generated.ts` | Colors, icons |
| `filters.generated.ts` | FacetFilter types |

### Cypher

| File | Content |
|------|---------|
| `00.0-constraints.cypher` | Uniqueness constraints |
| `00.5-taxonomy.cypher` | Meta-graph nodes |
| `01-kinds.cypher` | Kind definitions |
| `02-arcs.cypher` | ArcKind definitions |

### Rust

| File | Content |
|------|---------|
| `icons.rs` | TUI icon constants |

## Studio API Routes

### Views API

```
GET /api/views                    # List all views
GET /api/views/:id                # Get view definition
GET /api/views/:id/query          # Execute view query
POST /api/views/:id/query         # Execute with parameters
```

### Graph API

```
GET /api/graph/meta               # Meta-graph nodes
GET /api/graph/data               # Data nodes
POST /api/graph/query             # Execute Cypher
```

### Filter API

```
POST /api/filter/build            # Build Cypher from filter
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `NOVANET_ROOT` | (auto-detect) | Monorepo root path |
| `NEO4J_URI` | `bolt://localhost:7687` | Neo4j connection |
| `NEO4J_USER` | `neo4j` | Neo4j username |
| `NEO4J_PASSWORD` | `novanetpassword` | Neo4j password |
