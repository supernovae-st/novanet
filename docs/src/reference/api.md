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

// v0.12.0 ADR-024: Trait = Data Origin
type NodeTrait =
  | 'defined'     // Human-created once (was invariant)
  | 'authored'    // Human-written per locale (was localized)
  | 'imported'    // External data brought in (was knowledge)
  | 'generated'
  | 'retrieved';  // Fetched from external APIs (was aggregated)
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
  classes?: string[];      // v0.12.0: was kinds
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

// v0.12.0 ADR-024: Trait = Data Origin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeTrait {
    Defined,    // Human-created once (was Invariant)
    Authored,   // Human-written per locale (was Localized)
    Imported,   // External data brought in (was Knowledge)
    Generated,
    Retrieved,  // Fetched from external APIs (was Aggregated)
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

### Schema Labels

All schema nodes carry the `:Schema` label (v0.12.0 ADR-023: was `:Meta`):

| Label | Description |
|-------|-------------|
| `:Schema:Realm` | Realm definition |
| `:Schema:Layer` | Layer definition |
| `:Schema:Trait` | Trait definition |
| `:Schema:Class` | Node type definition (was `:Meta:Kind`) |
| `:Schema:ArcFamily` | Arc family definition |
| `:Schema:ArcClass` | Arc type definition (was `:Meta:ArcKind`) |

### Constraints

```cypher
// Key uniqueness (v0.12.0: Meta→Schema, Kind→Class, ArcKind→ArcClass)
CREATE CONSTRAINT class_key IF NOT EXISTS
FOR (n:Schema:Class) REQUIRE n.name IS UNIQUE;

CREATE CONSTRAINT arc_class_key IF NOT EXISTS
FOR (n:Schema:ArcClass) REQUIRE n.name IS UNIQUE;

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

### Query Classes by Layer

```cypher
MATCH (l:Schema:Layer {key: $layer})-[:HAS_CLASS]->(c:Schema:Class)
RETURN c.name, c.trait
ORDER BY c.name
```

### Query Arcs by Family

```cypher
MATCH (f:Schema:ArcFamily {key: $family})-[:HAS_ARC_CLASS]->(a:Schema:ArcClass)
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
WHERE any(label IN labels(n) WHERE label IN $classes)  // v0.12.0: was $kinds
  AND n.realm IN $realms
  AND n.layer IN $layers
RETURN n
```

## Generated Files

### TypeScript

| File | Content |
|------|---------|
| `node-classes.generated.ts` | Class Zod schemas |
| `arc-classes.generated.ts` | ArcClass definitions |
| `taxonomy.generated.ts` | Enums for realm/layer/trait |
| `visual-encoding.generated.ts` | Colors, icons |
| `filters.generated.ts` | FacetFilter types |

### Cypher

| File | Content |
|------|---------|
| `00.0-constraints.cypher` | Uniqueness constraints |
| `00.5-taxonomy.cypher` | Schema graph nodes |
| `01-classes.cypher` | Class definitions (v0.12.0: was 01-kinds.cypher) |
| `02-arcs.cypher` | ArcClass definitions |

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
GET /api/graph/schema             # Schema graph nodes (v0.12.0: was /api/graph/meta)
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
